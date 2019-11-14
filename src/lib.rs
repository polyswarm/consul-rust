#![allow(non_snake_case)]
#![allow(unused_doc_comments)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

pub mod agent;
pub mod catalog;
pub mod connect_ca;
pub mod errors;
pub mod health;
pub mod kv;
pub mod session;

mod request;

use std::time::Duration;

use reqwest::header::HeaderValue;
use reqwest::Client as HttpClient;
use reqwest::ClientBuilder;

use errors::{ErrorKind, Result, ResultExt};

#[derive(Clone, Debug)]
pub struct Client {
    config: Config,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Client { config: config }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub address: String,
    pub datacenter: Option<String>,
    pub http_client: HttpClient,
    pub wait_time: Option<Duration>,
    pub token: Option<HeaderValue>,
}

impl Config {
    pub fn new(address: String, token: Option<String>) -> Result<Config> {
        let token: Option<HeaderValue> = if let Some(t) = token {
            HeaderValue::from_str(&t)
                .map_err(|_| ErrorKind::ConsulError)?
                .into()
        } else {
            None
        };
        ClientBuilder::new()
            .build()
            .chain_err(|| "Failed to build reqwest client")
            .map(|client| Config {
                address,
                datacenter: None,
                http_client: client,
                wait_time: None,
                token,
            })
    }
}

#[derive(Clone, Debug, Default)]
pub struct QueryOptions {
    pub datacenter: Option<String>,
    pub wait_index: Option<u64>,
    pub wait_time: Option<Duration>,
}

#[derive(Clone, Debug)]
pub struct QueryMeta {
    pub last_index: Option<u64>,
    pub request_time: Duration,
}

#[derive(Clone, Debug, Default)]
pub struct WriteOptions {
    pub datacenter: Option<String>,
}

#[derive(Clone, Debug)]
pub struct WriteMeta {
    pub request_time: Duration,
}
