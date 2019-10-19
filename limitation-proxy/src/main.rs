// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use limitation_proxy::app::{self, Error};
use log::{debug, error};
use std::process;

mod cli;

fn main() {
    cli::util::init_logger();

    if let Err(err) = try_main() {
        error!("{}", err);
        process::exit(1);
    }
}

fn try_main() -> Result<(), Error> {
    let args = cli::from_args();
    debug!("parsed cli arguments; args={:?}", args);

    app::run(args.into())
}
