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
#[cfg(feature = "native-tls")]
extern crate native_tls;
extern crate rmpv;
extern crate tokio_core;
extern crate tokio_io;
#[cfg(feature = "tokio-tls")]
extern crate tokio_tls;

mod errors;
mod codec;
mod message;
mod net;
mod endpoint;

pub use endpoint::{Ack, Client, Response, Service, ServiceBuilder};
pub use net::{serve, serve_on_listener, ClientOnlyConnector, Connection, Connector};

pub use rmpv::{Integer, Utf8String, Value};
