// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use actix_web::http::header::HeaderName;
use limitation_proxy::app::Config;
use std::net::SocketAddr;
use std::num::ParseIntError;
use std::time::Duration;
use structopt::{clap, StructOpt};
use url::Url;

/// Default bind address
const DEFAULT_CLI_BIND: &str = "0.0.0.0:8080";
/// Default header
const DEFAULT_CLI_HEADER: &str = "authorization";
/// Default rate limit
const DEFAULT_CLI_LIMIT: &str = "5000";
/// Default rate limit period in seconds
const DEFAULT_CLI_PERIOD_SECS: &str = "3600";
/// Default backend proxy URL
const DEFAULT_CLI_PROXY: &str = "http://127.0.0.1:8000";
/// Default Redis URL
const DEFAULT_CLI_REDIS: &str = "redis://127.0.0.1/";

/// The "author" string for help messages.
const AUTHOR: &str = concat!(env!("CARGO_PKG_AUTHORS"), "\n\n");

/// Builds an [`Args`] from command line arguments with environment variables and defaults.
///
/// Note that any CLI parsing failures will result in a printed error message and the program will
/// quit.
///
/// [`Args`]: struct.Args.html
pub(crate) fn from_args() -> Args {
    Args::from_args()
}

/// A reverse proxy service with configurable rate limiting
///
/// The `limitation-proxy` service is an HTTP reverse proxy which sits in front of another HTTP
/// service and will perform rate limiting on all requests that pass through it. The rate limiting
/// is a variant of a fixed window rate limiting strategy and Redis is used for its persistence.
///
/// Project home page: https://github.com/fnichol/limitation
#[derive(Debug, StructOpt)]
#[structopt(
    global_settings(&[clap::AppSettings::UnifiedHelpMessage]),
    max_term_width = 80,
    author = AUTHOR,
)]
pub(crate) struct Args {
    /// Bind address for the service
    #[structopt(
        short = "b",
        long = "bind",
        env = "BIND_ADDR",
        hide_env_values = true,
        rename_all = "screaming_snake_case",
        default_value = DEFAULT_CLI_BIND
    )]
    pub(crate) bind: SocketAddr,

    /// Header to be used as the key for rate-limiting
    #[structopt(
        short = "H",
        long = "header",
        rename_all = "screaming_snake_case",
        default_value = DEFAULT_CLI_HEADER
    )]
    pub(crate) header: HeaderName,

    /// Maximum number of requests per key in the period
    #[structopt(
        short = "l",
        long = "limit",
        rename_all = "screaming_snake_case",
        default_value = DEFAULT_CLI_LIMIT
    )]
    pub(crate) limit: usize,

    /// Duration of period window in seconds
    #[structopt(
        short = "P",
        long = "period",
        rename_all = "screaming_snake_case",
        parse(try_from_str = parse_period),
        default_value = DEFAULT_CLI_PERIOD_SECS
    )]
    pub(crate) period: Duration,

    /// Backend proxy URL target
    #[structopt(
        short = "p",
        long = "proxy",
        env = "PROXY_URL",
        hide_env_values = true,
        rename_all = "screaming_snake_case",
        default_value = DEFAULT_CLI_PROXY
    )]
    pub(crate) proxy: Url,

    /// Redis URL for persistence
    #[structopt(
        short = "r",
        long = "redis",
        env = "REDIS_URL",
        hide_env_values = true,
        rename_all = "screaming_snake_case",
        default_value = DEFAULT_CLI_REDIS
    )]
    pub(crate) redis: Url,
}

/// Custom parser that takes a number of seconds as a `str` and returns a `Duration`.
///
/// # Errors
///
/// Returns an `Err` if the `str` does not parse as a positive integer.
fn parse_period(src: &str) -> Result<Duration, ParseIntError> {
    Ok(Duration::from_secs(u64::from_str_radix(src, 16)?))
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        Config::builder()
            .bind_addr(args.bind)
            .redis_url(args.redis)
            .proxy_to(args.proxy)
            .header(args.header)
            .rate_limit(args.limit)
            .rate_period(args.period)
            .build()
    }
}

pub(crate) mod util {
    use std::env;

    /// Configure and initialize the logger.
    pub(crate) fn init_logger() {
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
