//! This crate provides facilities to use the `MessagePack` remote procedure call system
//! (`MessagePack-RPC`) in Rust.
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", deny(clippy))]
#![cfg_attr(feature = "clippy", allow(missing_docs_in_private_items))]
#![cfg_attr(feature = "clippy", allow(type_complexity))]

extern crate bytes;
extern crate futures;
#[macro_use]
extern crate log;
extern crate rmpv;
extern crate tokio_core;
extern crate tokio_io;

mod errors;
mod codec;
pub mod message;
mod endpoint;

pub use endpoint::{serve, Ack, Client, Endpoint, IntoStaticFuture, Response, Service,
                   ServiceWithClient};
pub use rmpv::{Integer, Utf8String, Value};
