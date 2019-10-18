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

const HEADER_LIMIT: &str = "x-ratelimit-limit";
const HEADER_REMAINING: &str = "x-ratelimit-remaining";
const HEADER_RESET: &str = "x-ratelimit-reset";

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
            Err(_) => Either::B(Either::B(service.borrow_mut().call(req))),
        }))
    }
}

fn key(req: &ServiceRequest) -> Option<String> {
    let token_header = req.app_data::<HeaderName>().expect(
        "web::Data<HeaderName> should be set in app data for RateLimiter middleware token header",
    );

    req.headers()
        .get(token_header.get_ref())
        .and_then(|s| s.to_str().ok())
        .map(|s| s.to_string())
}

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
