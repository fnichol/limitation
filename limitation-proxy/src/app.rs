// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Rate-limiting reverse proxy Actix service.

use actix_web::{client::Client, http::header::HeaderName, middleware, web, App, HttpServer};
use limitation_actix_middleware::{Limiter, RateLimiter};
use std::error;
use std::net::SocketAddr;
use std::time::Duration;
use typed_builder::TypedBuilder;
use url::Url;

mod handlers;

/// Error type for the application.
pub type Error = Box<dyn error::Error>;

/// Service configuration.
#[derive(Clone, Debug, TypedBuilder)]
pub struct Config {
    pub(crate) bind_addr: SocketAddr,
    pub(crate) redis_url: Url,
    pub(crate) proxy_to: Url,
    pub(crate) header: HeaderName,
    pub(crate) rate_limit: usize,
    pub(crate) rate_period: Duration,
}

/// Build and run the service given a configuration.
pub fn run(config: Config) -> Result<(), Error> {
    let sys = actix_rt::System::new(env!("CARGO_PKG_NAME"));
    start_server(config)?;
    Ok(sys.run()?)
}

fn start_server(config: Config) -> Result<(), Error> {
    let addr = config.bind_addr;
    let proxy_to = web::Data::new(config.proxy_to);
    let limiter = web::Data::new(
        Limiter::build(config.redis_url.as_str())
            .limit(config.rate_limit)
            .period(config.rate_period)
            .finish()?,
    );
    let header = web::Data::new(config.header);

    HttpServer::new(move || {
        App::new()
            .register_data(limiter.clone())
            .register_data(proxy_to.clone())
            .register_data(header.clone())
            .data(Client::new())
            .wrap(RateLimiter)
            .wrap(middleware::Logger::default())
            .default_service(web::route().to_async(handlers::forward))
    })
    .bind(addr)?
    .start();

    Ok(())
}
