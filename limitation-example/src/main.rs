// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use futures::Future;
use limitation::Limiter;
use std::env;
use std::error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn error::Error>> {
    // Fetch a key name from the first argument to the program, otherwise fail with a usage message
    let key = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("usage: {} <KEY>", env!("CARGO_PKG_NAME")));

    // Fetch a Redis URL from an environment variable or fallback to a suitable default
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());

    // Build a `Limiter` and turn down the rate from its default
    let limiter = Limiter::build(&redis_url)
        .limit(5)
        .period(Duration::from_secs(10))
        .finish()?;

    // Run a single count operation and print to `stdout` if we're under the limit and to `stdout`
    // if we've exceeded the limit
    tokio::run(
        limiter
            .count(key)
            .map_err(|err| eprintln!("err: {}", err))
            .and_then(|count| {
                println!("ok: {:?}", count);
                Ok(())
            }),
    );

    Ok(())
}
