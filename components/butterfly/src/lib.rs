// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Butterfly is the [SWIM](https://www.cs.cornell.edu/~asdas/research/dsn02-swim.pdf)
//! implementation for Habitat, along with a ZeroMQ based gossip protocol.
//!
//! It implements SWIM+Susp+Inf. It uses Newscast-style "heat" tracking to share membership rumors,
//! while trying to keep UDP packet sizes below 512 bytes. It has the following changes:
//!
//! 1. It uses a single membership rumor with internal logic for applying the rumors state, rather
//!    than sending differential event messages.
//! 1. If an "Alive" membership rumor is received with a higher incarnation, it takes precedent
//!    over "Confirmed" membership rumors.
//! 1. Members can be marked "persistent", which means that they will always be taken through the
//!    Probe cycle, regardless of their status. This allows networks to heal from partitions.
//!
//! The SWIM implementation has three working threads:
//!
//! 1. An inbound thread, handling receipt of SWIM messages.
//! 1. An outbound thread, which handles the Ping->PingReq cycle and protocol timing.
//! 1. An expire thread, which handles timing out suspected members.
//!
//! The Gossip implementation has two working threads:
//!
//! 1. A 'push' thread, which fans out to 5 members every second (or longer, if it takes longer
//!    than 1 second to send all the messages to all the members in the fan-out; no more frequently
//!    than one second).
//! 1. A 'pull' thread, which takes messages from any push source and applies them locally.
//!
//! Start exploring the code base by following the thread of execution in the `server` module.

extern crate habitat_core;
extern crate habitat_net;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate rand;
extern crate rustc_serialize;
extern crate time;
extern crate uuid;
extern crate zmq;

#[macro_use]
pub mod trace;
pub mod client;
pub mod error;
pub mod member;
pub mod message;
pub mod rumor;
pub mod server;

pub use server::Server;
