// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#![crate_name = "libraft"]
#![crate_type="lib"]
#![doc(html_logo_url = "https://raw.githubusercontent.com/Hoverbear/raft/master/raft.png")]
#![doc(html_root_url = "https://hoverbear.github.io/raft/raft/")]

//! This is the Raft Distributed Consensus Protocol implemented for Rust.
//! [Raft](http://raftconsensus.github.io/) is described as:
//!
//! > Raft is a consensus algorithm that is designed to be easy to understand. It's equivalent to
//! > Paxos in fault-tolerance and performance. The difference is that it's decomposed into
//! > relatively independent subproblems, and it cleanly addresses all major pieces needed for
//! > practical systems.
//!
//! This implementation utilizes [Cap'n Proto](https://kentonv.github.io/capnproto/) for its RPC,
//! [`mio`](https://github.com/carllerche/mio) for it's async event loop.
//!
//! If this package fails to build for you it is possibly because you do not have the
//! [`capnp`](https://capnproto.org/capnp-tool.html) utility installed. You should be able to find
//! appropriate packages for most popular distributions.
//!
//! # Consuming this library
//!
//! Consuming this library works in a few parts:
//!
//! 1. Implement or consume a Persistent Log and a State Machine such that they will hook into
//!    your application desirably.
//! 2. Create a `Server` with those implementations. It will independently fire up and join the
//!    cluster.
//! 3. Interact with the cluster by issuing `.propose()` and `.query()` calls via the `Client`
//! 4. React to calls to `.propose()` and `.query()` from the implemented `StateMachine`
//!
//! ## Persistent Log
//!
//! A `Log` represents the **replicated, persistent log** of your application. It has a
//! strong ordering such that `A → B → C` and should **only** act to store information. Entries
//! placed into the log should not be acted on in any way by the consuming application as they
//! have not been committed to the cluster.
//!
//! Some ideas for a Persistent Log implementation:
//!
//!   * A PostgreSQL / SQLite instance.
//!   * A plain old file.
//!   * A vector in memory *(Note: Log compaction is still pending, so be aware of running out!)*
//!
//! > It is our belief that in many cases the implementation of `Log` will be generic to
//! > application purposes. You are encouraged to submit your own implementations to us!
//!
//! ## State Machine
//!
//! The `StateMachine` represents the **stateful representation** of your application. Events
//! are applied to the `StateMachine` in the correct ordering at the time of commit. This is where
//! your application **should** act on information.
//!
//! In the `StateMachine` there are both mutable (`.apply()`) and immutable (`.query()`) calls.
//! There is a considerable performance difference, as `.query()` calls do not pass through the
//! durable `Log` while `.apply()` events do.
//!
//! Some ideas for a State Machine implementation:
//!
//!   * A Hashmap or key-value store (Example provided)
//!   * A single register (Example provided)
//!   * Basically anything from `std::collections`
//!
//! ## Client Requests
//!
//! Client requests are the **only** way to interact with the Raft cluster. Calls to `.propose()`
//! and `.query()` are automatically routed to the relevant `Leader` node and behave as blocking
//! calls.
//!
//! This means `.propose()` won't return until the entry is durably replicated into the log of at
//! least the majority of the cluster and has been commited. `.query()` will perform better if
//! you wish to only read data and not have it pass through the persisted log.
//!

#![allow(unused_extern_crates)]
#![cfg_attr(test, feature(test))]
extern crate bufstream;
extern crate capnp;
extern crate capnp_nonblock;
extern crate mio;
extern crate rand;
extern crate uuid;
extern crate libproto;
extern crate protobuf;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
#[macro_use]
extern crate scoped_log;
#[macro_use]
extern crate wrapped_enum;
#[cfg(test)]
extern crate env_logger;

/// Prepares the environment testing. Should be called as the first line of every test with the
/// name of the test as the only argument.
///
/// TODO: Make this an annotation like #[rust_test] instead of a macro.
#[cfg(test)]
macro_rules! setup_test {
    ($test_name:expr) => (
        let _ = env_logger::init();
        push_log_scope!($test_name);
    );
}

pub mod state_machine;
pub mod persistent_log;
pub mod messages_capnp;
mod backoff;
mod client;
mod connection;
mod messages;
mod consensus;
mod server;
mod state;
mod cmd;

pub use client::Client;
pub use cmd::{Command, encode, decode};
pub use persistent_log::Log;
pub use server::NotifyMessage;
pub use server::Server;
pub use state_machine::StateMachine;

use std::{io, net, ops, fmt};

use uuid::Uuid;

/// A simple convienence type.
pub type Result<T> = std::result::Result<T, Error>;

wrapped_enum!{
    #[doc = "The generic `raft::Error` is composed of one of the errors that can originate from the"]
    #[doc = "various libraries consumed by the library."]
    #[doc = "With the exception of the `Raft` variant these are generated from `try!()` macros invoking"]
    #[doc = "on `io::Error` or `capnp::Error` by using"]
    #[doc = "[`FromError`](https://doc.rust-lang.org/std/error/#the-fromerror-trait)."]
    #[derive(Debug)]
    pub enum Error {
/// An error originating from the [Cap'n Proto](https://github.com/dwrensha/capnproto-rust) library.
        CapnProto(capnp::Error),
/// A specific error produced when a bad Cap'n proto message is discovered.
        SchemaError(capnp::NotInSchema),
/// Errors originating from `std::io`.
        Io(io::Error),
/// Raft specific errors.
        Raft(RaftError),
/// Errors related to parsing addresses.
        AddrParse(net::AddrParseError),
    }
}

/// A Raft Error represents a Raft specific error that consuming code is expected to handle
/// gracefully.
#[derive(Debug)]
pub enum RaftError {
    /// The server ran out of slots in the slab for new connections
    ConnectionLimitReached,
    /// A client reported an invalid client id
    InvalidClientId,
    /// A consensus module reported back a leader not in the cluster.
    ClusterViolation,
    /// A remote connection attempted to use an unknown connection type in the connection preamble
    UnknownConnectionType,
    /// An invalid peer in in the peer set. Returned Server::new().
    InvalidPeerSet,
    /// Registering a connection failed
    ConnectionRegisterFailed,
    /// Failed to find a leader in the cluster. Try again later.
    LeaderSearchExhausted,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::CapnProto(ref error) => fmt::Display::fmt(error, f),
            Error::SchemaError(ref error) => fmt::Display::fmt(error, f),
            Error::Io(ref error) => fmt::Display::fmt(error, f),
            Error::Raft(ref error) => fmt::Debug::fmt(error, f),
            Error::AddrParse(ref error) => fmt::Debug::fmt(error, f),
        }
    }
}

/// The term of a log entry.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Term(u64);
impl Term {
    pub fn as_u64(self) -> u64 {
        self.0
    }
}
impl From<u64> for Term {
    fn from(val: u64) -> Term {
        Term(val)
    }
}
impl Into<u64> for Term {
    fn into(self) -> u64 {
        self.0
    }
}
impl ops::Add<u64> for Term {
    type Output = Term;
    fn add(self, rhs: u64) -> Term {
        Term(self.0.checked_add(rhs).expect("overflow while incrementing Term"))
    }
}
impl ops::Sub<u64> for Term {
    type Output = Term;
    fn sub(self, rhs: u64) -> Term {
        Term(self.0.checked_sub(rhs).expect("underflow while decrementing Term"))
    }
}
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

/// The index of a log entry.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogIndex(u64);
impl LogIndex {
    pub fn as_u64(self) -> u64 {
        self.0
    }
}
impl From<u64> for LogIndex {
    fn from(val: u64) -> LogIndex {
        LogIndex(val)
    }
}
impl Into<u64> for LogIndex {
    fn into(self) -> u64 {
        self.0
    }
}
impl ops::Add<u64> for LogIndex {
    type Output = LogIndex;
    fn add(self, rhs: u64) -> LogIndex {
        LogIndex(self.0.checked_add(rhs).expect("overflow while incrementing LogIndex"))
    }
}
impl ops::Sub<u64> for LogIndex {
    type Output = LogIndex;
    fn sub(self, rhs: u64) -> LogIndex {
        LogIndex(self.0.checked_sub(rhs).expect("underflow while decrementing LogIndex"))
    }
}
/// Find the offset between two log indices.
impl ops::Sub for LogIndex {
    type Output = u64;
    fn sub(self, rhs: LogIndex) -> u64 {
        self.0.checked_sub(rhs.0).expect("underflow while subtracting LogIndex")
    }
}
impl fmt::Display for LogIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

/// The ID of a Raft server. Must be unique among the participants in a
/// consensus group.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct ServerId(u64);
impl ServerId {
    fn as_u64(self) -> u64 {
        self.0
    }
}
impl From<u64> for ServerId {
    fn from(val: u64) -> ServerId {
        ServerId(val)
    }
}
impl Into<u64> for ServerId {
    fn into(self) -> u64 {
        self.0
    }
}
impl fmt::Debug for ServerId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ServerId({})", self.0)
    }
}
impl fmt::Display for ServerId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

/// The ID of a Raft client.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct ClientId(Uuid);
impl ClientId {
    fn new() -> ClientId {
        ClientId(Uuid::new_v4())
    }
    fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
    fn from_bytes(bytes: &[u8]) -> Result<ClientId> {
        match Uuid::from_bytes(bytes) {
            Some(uuid) => Ok(ClientId(uuid)),
            None => Err(Error::Raft(RaftError::InvalidClientId)),
        }
    }
}
impl fmt::Debug for ClientId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClientId({})", self.0)
    }
}
impl fmt::Display for ClientId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
