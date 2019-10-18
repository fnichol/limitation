// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use limitation_proxy::app::{self, Error};
use log::error;
use std::process;

fn main() {
    util::init_logger();

    if let Err(err) = try_main() {
        error!("{}", err);
        process::exit(1);
    }
}

fn try_main() -> Result<(), Error> {
    let config = app::Config::build().finish()?;

    app::run(config)
}

mod util {
    use std::env;

    pub fn init_logger() {
        if env::var("RUST_LOG").is_err() {
            env::set_var(
                "RUST_LOG",
                concat!(
                    "actix_server=info,actix_web=info,",
                    env!("CARGO_PKG_NAME"),
                    "=info"
                ),
            );
        }
        env_logger::init();
    }
}
