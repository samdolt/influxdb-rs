//! InfluxDB is a popular time series database created by InfluxData.
//!
//! This crate is a wrapper around the http api of InfluxDb
//!
//! # InfluxDB version
//!
//! This library support `InfluxDB` v.1.2.x
//!
//! # SSL/TLS
//!
//! For now, using https to connect with `InfluxDB` is untested
//!
//! # Write using the Line Protocol
//!
//! ## With LinesBuilder
//!
//! `LinesBuilder` is a type safe abstraction over the Line Protocol. Whenever possible, it is
//! recommended to build line with `LinesBuilder`.
//!
//! ```rust,no_run
//! extern crate influxdb;
//!
//! use influxdb::LinesBuilder;
//!
//! fn main() {
//!
//!     // Write a line to mydb, without authentication
//!     let con = influxdb::Connection::connect("http://localhost/mydb").expect("Can't connect");
//!     con.write(&LinesBuilder::new("temperature").add_field("value", 25.0).build() ).unwrap();
//!
//!     // Write multiple lines to mydb, with authentication
//!     let con = influxdb::Connection::connect("http://user:password@localhost/mydb").expect("Can't connect");
//!     let lines = LinesBuilder::new("temperature")
//!                                 .add_tag("sensor_id", "2345A")
//!                                 .add_tag("room", "N204")
//!                                 .add_field("value", 25.0)
//!                                 .add_field("unit", "C")
//!                             .add_line("pression")
//!                                 .add_tag("sensor_id", "2345A")
//!                                 .add_tag("room", "N204")
//!                                 .add_field("value", 1.0)
//!                                 .add_field("unit", "ATM")
//!                             .build();
//!     con.write(&lines).expect("Can't write");
//!
//! }
//! ```


#[macro_use] extern crate hyper;

extern crate reqwest;
extern crate url;
extern crate semver;
#[macro_use] extern crate slog;
#[macro_use] extern crate error_chain;

mod params;
mod connection;
mod lines;
mod values;
mod errors;

pub use connection::Connection;
pub use params::ConnectParams;
pub use params::Credential;
pub use lines::LinesBuilder;
pub use lines::Lines;
pub use values::Value;

pub use errors::{Error, ErrorKind, ResultExt, Result};

mod headers;

