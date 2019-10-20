// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! An Actix web middleware for rate limiting requests using a fixed window counter keyed on a
//! header.
//!
//! # Usage
//!
//! Add `limitation-actix-middleware` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! limitation-actix-middleware = "0.1.1"
//! ```
//!
//! ## Quick Example
//!
//! The [`RateLimiter`] middleware is the primary type which is intended to be inserted in an Actix
//! web app's middleware chain. The middleware requires 2 `Data` types to be present:
//!
//! 1. A `HeaderName` which is the header to use as the rate limiter key
//! 2. A [`Limiter`] which performs the rate limiting and manages persistence
//!
//! ```no_run
//! use actix_web::{http::header::HeaderName, web, App, HttpResponse};
//! use limitation_actix_middleware::{Limiter, RateLimiter};
//!
//! // Choose a header to use for rate limit tracking
//! let header = web::Data::new(HeaderName::from_static("authorization"));
//! // Build a `Limiter` which will be used by the middleware
//! let limiter = web::Data::new(Limiter::build("redis://127.0.0.1/").finish()?);
//!
//! let app = App::new()
//!     // Register the header as application data
//!     .register_data(header.clone())
//!     // Register the Limiter as application data
//!     .register_data(limiter.clone())
//!     // Insert the RateLimter middleware
//!     .wrap(RateLimiter)
//!     .service(
//!         web::resource("/test")
//!             .route(web::get().to(|| HttpResponse::Ok()))
//!             .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
//!     );
//! # Ok::<(), limitation_actix_middleware::Error>(())
//! ```
//!
//! [`Limiter`]: struct.Limiter.html
//! [`RateLimiter`]: struct.RateLimiter.html
//!
//! # Examples
//!
//! This crate ships with an example program called [catchall] which can be run from the sources
//! with:
//!
//! ```console
//! $ cargo run --example catchall
//! ```
//!
//! [catchall]:
//! https://github.com/fnichol/limitation/blob/master/limitation-actix-middleware/examples/catchall.rs

#![doc(html_root_url = "https://docs.rs/limitation-actix-middleware/0.1.1")]
#![deny(missing_docs)]

mod rate_limiter;

pub use rate_limiter::RateLimiter;

// re-export Limitation types
pub use limitation::{Builder, Error, Limiter, Status};
