// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chrono::SubsecRound;
use futures::Future;
use redis::Client;
use std::convert::TryInto;
use std::error;
use std::fmt;
use std::ops::Add;
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

    pub fn count<K: Into<String>>(&self, key: K) -> impl Future<Item = Status, Error = Error> {
        let limit = self.limit;

        self.track(key).and_then(move |(count, reset_epoch_utc)| {
            let status = build_status(count, limit, reset_epoch_utc);

            if count > limit {
                Err(Error::LimitExceeded(status))
            } else {
                Ok(status)
            }
        })
    }

    fn track<K: Into<String>>(&self, key: K) -> impl Future<Item = (usize, usize), Error = Error> {
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
                    .arg(&key)
                    .cmd("TTL")
                    .arg(&key);

                pipe.query_async(con)
                    .from_err()
                    .and_then(|(_, (count, ttl)): (_, (usize, u64))| {
                        Ok((count, epoch_utc_plus(Duration::from_secs(ttl))?))
                    })
            })
    }
}

#[derive(Clone, Debug)]
pub struct Status {
    limit: usize,
    remaining: usize,
    reset_epoch_utc: usize,
}

impl Status {
    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn remaining(&self) -> usize {
        self.remaining
    }

    pub fn reset_epoch_utc(&self) -> usize {
        self.reset_epoch_utc
    }
}

pub struct Builder<'a> {
    redis_url: &'a str,
    limit: usize,
    period: Duration,
}

impl Builder<'_> {
    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = limit;
        self
    }

    pub fn period(&mut self, period: Duration) -> &mut Self {
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
    LimitExceeded(Status),
    Time(time::OutOfRangeError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Client(ref err) => write!(f, "client error ({})", err),
            Error::LimitExceeded(ref status) => write!(f, "rate limit exceeded ({:?})", status),
            Error::Time(ref err) => write!(f, "time conversion error ({})", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Client(ref err) => err.source(),
            Error::LimitExceeded(_) => None,
            Error::Time(ref err) => err.source(),
        }
    }
}

impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        Error::Client(err)
    }
}

impl From<time::OutOfRangeError> for Error {
    fn from(err: time::OutOfRangeError) -> Self {
        Error::Time(err)
    }
}

fn build_status(count: usize, limit: usize, reset_epoch_utc: usize) -> Status {
    let remaining = if count >= limit { 0 } else { limit - count };

    Status {
        limit,
        remaining,
        reset_epoch_utc,
    }
}

fn epoch_utc_plus(duration: Duration) -> Result<usize, time::OutOfRangeError> {
    Ok(chrono::Utc::now()
        .add(chrono::Duration::from_std(duration)?)
        .round_subsecs(0)
        .timestamp()
        .try_into()
        .unwrap_or(0))
}
