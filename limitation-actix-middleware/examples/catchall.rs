// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use actix_web::{http::header::HeaderName, middleware, web, App, HttpResponse, HttpServer};
use futures::{future, Future};
use limitation_actix_middleware::{Limiter, RateLimiter};
use std::env;
use std::error;
use std::net::ToSocketAddrs;
use std::time::Duration;

const LOGGER: &Logger = &Logger;

type Error = Box<dyn error::Error>;

fn main() -> Result<(), Error> {
    // Initialize a simple logger
    log::set_logger(LOGGER).expect("error setting logger");
    log::set_max_level(log::LevelFilter::Info);

    // Fetch the host and port on which to run the HTTP service
    let addr = env::var("HOST").unwrap_or_else(|_| "127.0.0.1:8000".to_string());
    // Fetch a Redis URL from an environment variable or fallback to a suitable default
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    // Fetch a header name from an environment variable or fallback to a suitable default
    let header = env::var("HEADER")
        .unwrap_or_else(|_| "authorization".to_string())
        .to_lowercase();

    // Print some help instructions for the example
    emit_preamble(&addr, &header);
    // Configure and start the HTTP service
    start_server(addr, &redis_url, &header)?;

    Ok(())
}

fn start_server<A: ToSocketAddrs>(addr: A, redis_url: &str, header: &str) -> Result<(), Error> {
    let limiter = web::Data::new(
        Limiter::build(&redis_url)
            .limit(5)
            .period(Duration::from_secs(10))
            .finish()?,
    );
    let token_header = web::Data::new(HeaderName::from_lowercase(header.as_bytes())?);

    HttpServer::new(move || {
        App::new()
            .register_data(token_header.clone())
            .register_data(limiter.clone())
            .wrap(RateLimiter)
            .wrap(middleware::Logger::default())
            .default_service(web::route().to_async(hello))
    })
    .bind(addr)?
    .run()?;

    Ok(())
}

fn hello() -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    future::ok(
        HttpResponse::Ok()
            .content_type("text/plain")
            .body("Hello, world!\n"),
    )
}

fn emit_preamble(addr: &str, header: &str) {
    println!(
        "\

# Catch All Example

This service responds to all HTTP verbs on all paths identically with a plain
text hello message.

Once the HTTP service is running, provide the '{header}' header to
trigger the rate-limiting middleware:

     curl -v -H \"{header}: token MYTOKEN\" http://{addr}

If the '{}' header is missing, the service will return an
HTTP/403 Forbidden response:

     curl -v http://{addr}

---
",
        addr = addr,
        header = header,
    );
}

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        println!("{:<5} {}", record.level(), record.args());
    }

    fn flush(&self) {
        // `eprintln!` flushes on every call
    }
}
