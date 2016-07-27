


#![allow(dead_code)]

extern crate rustc_serialize;
#[macro_use]
extern crate log;

extern crate hyper;

pub use agent::Agent;
pub use client::Client;
pub use catalog::Catalog;
pub use health::Health;
pub use keystore::Keystore;
// pub use session::Session;
pub use structs::{AgentMember, Check, Service, ServiceHealth, RegisterService, TtlHealthCheck};

mod agent;
mod catalog;
mod client;
mod consul_error;
mod health;
mod keystore;
mod session;
mod structs;
