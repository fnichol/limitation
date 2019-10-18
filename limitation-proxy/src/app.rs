// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use actix_web::{middleware, web, App, HttpServer};
use std::error;

mod config;

pub use config::Config;

pub type Error = Box<dyn error::Error>;

pub fn run(config: Config) -> Result<(), Error> {
    let sys = actix_rt::System::new(env!("CARGO_PKG_NAME"));
    start_server(config)?;
    Ok(sys.run()?)
}

fn start_server(config: Config) -> Result<(), Error> {
    let addr = config.bind_addr;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .default_service(web::route().to_async(handlers::forward))
    })
    .bind(addr)?
    .start();

    Ok(())
}

mod handlers {
    use actix_web::{Error, HttpRequest, HttpResponse};
    use futures::{future, Future};

    pub fn forward(_req: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error> {
        future::ok(
            HttpResponse::Ok()
                .content_type("text/plain")
                .body("Hello, world!\n"),
        )
    }
}
