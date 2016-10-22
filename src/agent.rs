#![allow(non_snake_case)]

use std::sync::Arc;
use std::collections::HashMap;
use std::io::Read;
use rustc_serialize::json;
use hyper::client::{Client, Response};
use hyper::status::StatusCode;
use hyper::Url;
use super::{Service, TtlHealthCheck};
use super::consul_error::ConsulError;
use super::structs::AgentMember;

/// Agent can be used to query the Agent endpoints
#[derive(Clone)]
pub struct Agent {
    endpoint: String,
    client: Arc<Client>,
}

impl Agent {
    pub fn new(client: Arc<Client>, address: &str) -> Agent {
        Agent {
            endpoint: format!("http://{}/v1/agent", address),
            client: client,
        }
    }

    // TODO: Implement https://www.consul.io/docs/agent/http/agent.html#agent_checks

    /// https://www.consul.io/docs/agent/http/agent.html#agent_services
    pub fn services(&self) -> Result<HashMap<String, Service>, ConsulError> {
        Url::parse(&format!("{}/services", self.endpoint))
            .map_err(|_| ConsulError::BadURL)
            .and_then(|url| self.client.get(url).send().map_err(|_| ConsulError::HTTPFailure))
            .and_then(|mut response: Response| {
                let mut body: String = String::new();
                match response.read_to_string(&mut body) {
                    Ok(_) => Ok(body),
                    Err(_) => Err(ConsulError::HTTPFailure),
                }
            })
            .and_then(|body| json::decode(&body).map_err(|_| ConsulError::BadJSON))
    }

    /// https://www.consul.io/docs/agent/http/agent.html#agent_members
    pub fn members(&self) -> Result<Vec<AgentMember>, ConsulError> {
        Url::parse(&format!("{}/members", self.endpoint))
            .map_err(|_| ConsulError::BadURL)
            .and_then(|url| self.client.get(url).send().map_err(|_| ConsulError::HTTPFailure))
            .and_then(|mut response: Response| {
                let mut body: String = String::new();
                match response.read_to_string(&mut body) {
                    Ok(_) => Ok(body),
                    Err(_) => Err(ConsulError::HTTPFailure),
                }
            })
            .and_then(|body| json::decode(&body).map_err(|_| ConsulError::BadJSON))
    }

    /// https://www.consul.io/docs/agent/http/agent.html#agent_check_register
    pub fn register_ttl_check(&self, health_check: &TtlHealthCheck) -> Result<(), ConsulError> {
        if let Ok(json) = json::encode(&health_check) {
            Url::parse(&format!("{}/check/register", self.endpoint))
                .map_err(|_| ConsulError::BadURL)
                .and_then(|url| {
                    self.client.post(url).body(&json).send().map_err(|_| ConsulError::HTTPFailure)
                })
                .and_then(|response: Response| {
                    match response.status {
                        StatusCode::Ok => Ok(()),
                        _ => Err(ConsulError::RemoteFailure),
                    }
                })
        } else {
            Err(ConsulError::BadJSON)
        }
    }

    /// https://www.consul.io/docs/agent/http/agent.html#agent_check_pass
    pub fn check_pass(&self, check_id: &str, note: Option<&str>) -> Result<(), ConsulError> {
        match note {
                Some(n) => {
                    Url::parse(&format!("{}/check/pass/{}?note={}", self.endpoint, check_id, n))
                }
                None => Url::parse(&format!("{}/check/pass/{}", self.endpoint, check_id)),
            }
            .map_err(|_| ConsulError::BadURL)
            .and_then(|url| self.client.get(url).send().map_err(|_| ConsulError::HTTPFailure))
            .and_then(|response: Response| {
                match response.status {
                    StatusCode::Ok => Ok(()),
                    _ => Err(ConsulError::RemoteFailure),
                }
            })
    }


    // TODO: Structure the json returned
    /// https://www.consul.io/docs/agent/http/agent.html#agent_self
    pub fn get_self(&self) -> Result<String, ConsulError> {
        Url::parse(&format!("{}/self", self.endpoint))
            .map_err(|_| ConsulError::BadURL)
            .and_then(|url| self.client.get(url).send().map_err(|_| ConsulError::HTTPFailure))
            .and_then(|mut response: Response| {
                let mut body: String = String::new();
                match response.read_to_string(&mut body) {
                    Ok(_) => Ok(body),
                    Err(_) => Err(ConsulError::HTTPFailure),
                }
            })
    }
}
