// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use actix_web::http::header::HeaderName;
use std::error;
use std::net::SocketAddr;
use std::time::Duration;
use url::Url;

#[derive(Clone, Debug)]
pub struct Config {
    pub(crate) bind_addr: SocketAddr,
    pub(crate) redis_url: Url,
    pub(crate) proxy_to: Url,
    pub(crate) header: HeaderName,
    pub(crate) rate_limit: usize,
    pub(crate) rate_period: Duration,
}

impl Config {
    pub fn build<'a, 'b, 'c, 'd>() -> Builder<'a, 'b, 'c, 'd> {
        Builder {
            bind_addr: "127.0.0.1:8080",
            redis_url: "redis://127.0.0.1/",
            proxy_to: "127.0.0.1:8000",
            header: "authorization",
            rate_limit: 5000,
            rate_period: Duration::from_secs(60 * 60),
        }
    }
}

pub struct Builder<'a, 'b, 'c, 'd> {
    bind_addr: &'a str,
    redis_url: &'b str,
    proxy_to: &'c str,
    header: &'d str,
    rate_limit: usize,
    rate_period: Duration,
}

impl<'a, 'b, 'c, 'd> Builder<'a, 'b, 'c, 'd> {
    pub fn bind_addr(&'a mut self, bind_addr: &'a str) -> &'a mut Self {
        self.bind_addr = bind_addr;
        self
    }

    pub fn redis_url(&'b mut self, redis_url: &'b str) -> &'b mut Self {
        self.redis_url = redis_url;
        self
    }

    pub fn proxy_to(&'c mut self, proxy_to: &'c str) -> &'c mut Self {
        self.proxy_to = proxy_to;
        self
    }

    pub fn rate_limit(&mut self, rate_limit: usize) -> &mut Self {
        self.rate_limit = rate_limit;
        self
    }

    pub fn rate_period(&mut self, rate_period: Duration) -> &mut Self {
        self.rate_period = rate_period;
        self
    }

    pub fn finish(&self) -> Result<Config, Box<dyn error::Error>> {
        let proxy_to_sock = self.proxy_to.parse::<SocketAddr>()?;

        Ok(Config {
            bind_addr: self.bind_addr.parse()?,
            redis_url: Url::parse(self.redis_url)?,
            proxy_to: Url::parse(&format!("http://{}", proxy_to_sock))?,
            header: HeaderName::from_lowercase(self.header.to_lowercase().as_bytes())?,
            rate_limit: self.rate_limit,
            rate_period: self.rate_period,
        })
    }
}
