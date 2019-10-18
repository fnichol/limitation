// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod rate_limiter;

pub use rate_limiter::RateLimiter;

// re-export Limitation types
pub use limitation::{Builder, Error, Limiter, Status};
