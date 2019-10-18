// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use futures::Future;
use redis::Client;
use std::error;
use std::fmt;
use std::time::Duration;

const DEFAULT_LIMIT: usize = 5000;
const DEFAULT_PERIOD_SECS: u64 = 60 * 60;

#[derive(Clone, Debug)]
pub struct Limiter {
    client: Client,
    limit: usize,
    period: Duration,
}

impl Limiter {
    pub fn build(redis_url: &str) -> Builder {
        Builder {
            redis_url,
            limit: DEFAULT_LIMIT,
            period: Duration::from_secs(DEFAULT_PERIOD_SECS),
        }
    }

    pub fn count<K: Into<String>>(&self, key: K) -> impl Future<Item = usize, Error = Error> {
        let limit = self.limit;

        self.track(key).and_then(move |count| {
            if count > limit {
                Err(Error::LimitExceeded(count))
            } else {
                Ok(count)
            }
        })
    }

    fn track<K: Into<String>>(&self, key: K) -> impl Future<Item = usize, Error = Error> {
        let key = key.into();
        let exipres = self.period.as_secs();

        self.client
            .get_async_connection()
            .from_err()
            .and_then(move |con| {
                let mut pipe = redis::pipe();
                pipe.atomic()
                    .cmd("SET")
                    .arg(&key)
                    .arg(0)
                    .arg("EX")
                    .arg(exipres)
                    .arg("NX")
                    .ignore()
                    .cmd("INCR")
                    .arg(&key);

                pipe.query_async(con)
                    .from_err()
                    .and_then(|(_, (count,)): (_, (usize,))| Ok(count))
            })
    }
}

pub struct Builder<'a> {
    redis_url: &'a str,
    limit: usize,
    period: Duration,
}

impl Builder<'_> {
    pub fn limit<'a>(&'a mut self, limit: usize) -> &'a mut Self {
        self.limit = limit;
        self
    }

    pub fn period<'a>(&'a mut self, period: Duration) -> &'a mut Self {
        self.period = period;
        self
    }

    pub fn finish(&self) -> Result<Limiter, Error> {
        Ok(Limiter {
            client: Client::open(self.redis_url)?,
            limit: self.limit,
            period: self.period,
        })
    }
}

#[derive(Debug)]
pub enum Error {
    Client(redis::RedisError),
    LimitExceeded(usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Client(ref err) => write!(f, "client error ({})", err),
            Error::LimitExceeded(ref count) => write!(f, "rate limit exceeded count={}", count),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Client(ref err) => err.source(),
            Error::LimitExceeded(_) => None,
        }
    }
}

impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        Error::Client(err)
    }
}
