// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements. See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership. The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License. You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. See the License for the
// specific language governing permissions and limitations
// under the License.

//! Rust runtime library for the Apache Thrift RPC system.
//!
//! This crate implements the components required to build a working
//! Thrift server and client. It is divided into the following modules:
//!
//! 1. errors
//! 2. configuration
//! 3. protocol
//! 4. transport
//! 5. server
//! 6. autogen
//!
//! The modules are layered as shown in the diagram below. The `autogen'd`
//! layer is generated by the Thrift compiler's Rust plugin. It uses the
//! types and functions defined in this crate to serialize and deserialize
//! messages and implement RPC. Users interact with these types and services
//! by writing their own code that uses the auto-generated clients and
//! servers.
//!
//! ```text
//! +-----------+
//! | user app  |
//! +-----------+
//! | autogen'd | (uses errors, autogen)
//! +-----------+
//! |  protocol |
//! +-----------+
//! | transport |
//! +-----------+
//! ```
//!
//! # Tutorial
//!
//! For an example of how to setup a simple client and server using this crate
//! see the [tutorial].
//!
//! [tutorial]: https://github.com/apache/thrift/tree/master/tutorial/rs

#![crate_type = "lib"]
#![doc(test(attr(allow(unused_variables, dead_code), deny(warnings))))]
#![deny(bare_trait_objects)]

// NOTE: this macro has to be defined before any modules. See:
// https://danielkeep.github.io/quick-intro-to-macros.html#some-more-gotchas

/// Assert that an expression returning a `Result` is a success. If it is,
/// return the value contained in the result, i.e. `expr.unwrap()`.
#[cfg(test)]
macro_rules! assert_success {
    ($e: expr) => {{
        let res = $e;
        assert!(res.is_ok());
        res.unwrap()
    }};
}

pub mod protocol;

#[cfg(feature = "server")]
pub mod server;
pub mod transport;

mod errors;
pub use crate::errors::*;

mod autogen;
pub use crate::autogen::*;

mod configuration;
pub use crate::configuration::*;

/// Result type returned by all runtime library functions.
///
/// As is convention this is a typedef of `std::result::Result`
/// with `E` defined as the `thrift::Error` type.
pub type Result<T> = std::result::Result<T, self::Error>;

// Re-export ordered-float, since it is used by the generator
// FIXME: check the guidance around type reexports
pub use ordered_float::OrderedFloat;
