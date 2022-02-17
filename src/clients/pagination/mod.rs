//! Utilities for pagination requests. If the configured client is asynchronous,
//! it'll be  based on `futures::stream::Stream`, if it's synchronous it'll just
//! use `std::iter::Iterator`.
//!
//! All implementations export:
//!
//! * A `Paginator` struct which wraps the iterable of items
//! * A `paginate` function, which returns a `Paginator` based on a request that
//!   may be repeated in order to return a continuous sequence of `Page`s
//!
//! Note that `Paginator` should actually be a trait so that a dynamic
//! allocation can be avoided when returning it with `-> impl Iterator<T>`, as
//! opposed to `-> Box<dyn Iterator<T>>`. But since the Spotify clients are
//! trait-based, they can't return anonymous types, and the former option is
//! impossible for now. This is the same small overhead introduced by the
//! `async_trait` crate and that will hopefully be fixed in the future.
//!
//! Both `Paginator` and `paginate` have a lifetime of `'a`. This is because the
//! pagination may borrow the client itself in order to make requests, and said
//! lifetime helps ensure the `Paginator` struct won't outlive the client.

#[cfg(feature = "__sync")]
mod iter;
#[cfg(feature = "__async")]
mod stream;

#[cfg(feature = "__sync")]
pub use iter::{paginate, Paginator};
#[cfg(feature = "__async")]
pub use stream::{paginate, write_paginate, Paginator};
