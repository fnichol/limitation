// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use actix_web::{client::Client, middleware, web, App, HttpServer};
use std::error;

mod config;
mod handlers;

pub use config::Config;

pub type Error = Box<dyn error::Error>;

pub fn run(config: Config) -> Result<(), Error> {
    let sys = actix_rt::System::new(env!("CARGO_PKG_NAME"));
    start_server(config)?;
    Ok(sys.run()?)
}

fn start_server(config: Config) -> Result<(), Error> {
    let addr = config.bind_addr;
    let proxy_to = web::Data::new(config.proxy_to);

    HttpServer::new(move || {
        App::new()
            .register_data(proxy_to.clone())
            .data(Client::new())
            .wrap(middleware::Logger::default())
            .default_service(web::route().to_async(handlers::forward))
    })
    .bind(addr)?
    .start();

    Ok(())
}
