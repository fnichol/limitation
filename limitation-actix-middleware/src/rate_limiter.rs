// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header::HeaderName,
    Error, HttpResponse,
};
use futures::{
    future::{self, Either, FutureResult},
    Future, Poll,
};
use limitation::{Error as LError, Limiter, Status};
use std::cell::RefCell;
use std::rc::Rc;

/// The `limit` HTTP header name
const HEADER_LIMIT: &str = "x-ratelimit-limit";
/// The `remaining` HTTP header name
const HEADER_REMAINING: &str = "x-ratelimit-remaining";
/// The `reset` HTTP header name
const HEADER_RESET: &str = "x-ratelimit-reset";

/// `Middleware` for rate limiting requests using a fixed window counter keyed on a `HeaaderName`.
///
/// # Example
///
/// A basic example:
///
/// ```no_run
/// use actix_web::{http::header::HeaderName, web, App, HttpResponse};
/// use limitation_actix_middleware::{Limiter, RateLimiter};
///
/// let header = web::Data::new(HeaderName::from_static("authorization"));
/// let limiter = web::Data::new(Limiter::build("redis://127.0.0.1/").finish()?);
///
/// let app = App::new()
///     .register_data(header.clone())
///     .register_data(limiter.clone())
///     .wrap(RateLimiter)
///     .service(
///         web::resource("/test")
///             .route(web::get().to(|| HttpResponse::Ok()))
///             .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
///     );
/// # Ok::<(), limitation_actix_middleware::Error>(())
/// ```
pub struct RateLimiter;

impl<S, B> Transform<S> for RateLimiter
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimiterMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(RateLimiterMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct RateLimiterMiddleware<S> {
    // TODO: The service is reference counted so that it can be cloned into a Future for later
    // execution.  This should be avoidable if/when the library is upgraded to to use
    // *async/await*.
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for RateLimiterMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        // A mis-configuration of the Actix App will result in a **runtime** failure, so the expect
        // method description is important context for the developer.
        let limiter = req
            .app_data::<Limiter>()
            .expect("web::Data<Limiter> should be set in app data for RateLimiter middleware");
        let service = self.service.clone();
        let key = match key(&req) {
            Some(key) => key,
            None => {
                return Box::new(future::ok(
                    req.into_response(HttpResponse::Forbidden().finish().into_body()),
                ))
            }
        };

        Box::new(limiter.count(key).then(move |result| match result {
            Ok(status) => Either::A(service.borrow_mut().call(req).map(move |mut res| {
                add_rate_limit_headers(&mut res, &status);
                res
            })),
            Err(LError::LimitExceeded(status)) => Either::B(Either::A(
                future::ok(req.into_response(HttpResponse::Forbidden().finish().into_body())).map(
                    move |mut res| {
                        add_rate_limit_headers(&mut res, &status);
                        res
                    },
                ),
            )),
            // TODO: In the case of backend errors this middleware currently continues down the
            // chain. This may or may not be the desired behavior in the long run and is worthy of
            // revisiting.
            Err(_) => Either::B(Either::B(service.borrow_mut().call(req))),
        }))
    }
}

/// Determines a key on which to rate limit the request.
///
/// If the expected header is not present, then `None` will be returned.
fn key(req: &ServiceRequest) -> Option<String> {
    // A mis-configuration of the Actix App will result in a **runtime** failure, so the expect
    // method description is important context for the developer.
    let token_header = req.app_data::<HeaderName>().expect(
        "web::Data<HeaderName> should be set in app data for RateLimiter middleware token header",
    );

    // TODO: Currently the header value is used verbatim which likely contains a sensitive key.
    // This value could be hashed before being transmitted to the persistence backend.
    req.headers()
        .get(token_header.get_ref())
        .and_then(|s| s.to_str().ok())
        .map(|s| s.to_string())
}

/// Adds rate-limiting HTTP headers to the outgoing response.
fn add_rate_limit_headers<B>(res: &mut ServiceResponse<B>, status: &Status) {
    res.headers_mut()
        .insert(HeaderName::from_static(HEADER_LIMIT), status.limit().into());
    res.headers_mut().insert(
        HeaderName::from_static(HEADER_REMAINING),
        status.remaining().into(),
    );
    res.headers_mut().insert(
        HeaderName::from_static(HEADER_RESET),
        status.reset_epoch_utc().into(),
    );
}
