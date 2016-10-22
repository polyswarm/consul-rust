#![allow(non_snake_case)]

use rustc_serialize::json;
use hyper::client::{Client, Response};
use hyper::Url;
use std::io::Read;
use super::ServiceHealth;
use super::Check;
use super::consul_error::ConsulError;
use std::sync::Arc;

#[derive(Clone)]
pub struct Health {
    endpoint: String,
    client: Arc<Client>,
}

impl Health {
    pub fn new(client: Arc<Client>, address: &str) -> Health {
        Health {
            endpoint: format!("http://{}/v1/health", address),
            client: client,
        }
    }

    /// https://www.consul.io/docs/agent/http/health.html#health_service
    pub fn service(&self,
                   name: &str,
                   dc: Option<&str>,
                   near: Option<&str>,
                   tag: Option<&str>,
                   passing: Option<bool>)
                   -> Result<Vec<ServiceHealth>, ConsulError> {
        Url::parse(&format!("{}/service/{}", self.endpoint, name))
            .map_err(|_| ConsulError::BadURL)
            .and_then(|url| {
                let mut query = String::from("?");
                let mut use_query = false;
                if let Some(dc) = dc {
                    use_query = true;
                    query.push_str("dc=");
                    query.push_str(dc);
                }
                if let Some(near) = near {
                    if use_query {
                        query.push('&');
                    }
                    use_query = true;
                    query.push_str("near=");
                    query.push_str(near);
                }
                if let Some(tag) = tag {
                    if use_query {
                        query.push('&');
                    }
                    use_query = true;
                    query.push_str("tag=");
                    query.push_str(tag);
                }
                if let Some(passing) = passing {
                    if use_query {
                        query.push('&');
                    }
                    use_query = true;
                    query.push_str(&format!("passing={}", passing));
                }
                // TODO: This doesn't seem correct.  I must test it and clean it up
                let f = if use_query {
                    format!("{}{}", url, query)
                } else {
                    format!("{}", url)
                };
                self.client.get(&f).send().map_err(|_| ConsulError::HTTPFailure)
            })
            .and_then(|mut response: Response| {
                let mut body: String = String::new();
                match response.read_to_string(&mut body) {
                    Ok(_) => Ok(body),
                    Err(_) => Err(ConsulError::HTTPFailure),
                }
            })
            .and_then(|body| json::decode(&body).map_err(|_| ConsulError::BadJSON))
    }

    /// https://www.consul.io/docs/agent/http/health.html#health_checks
    pub fn checks(&self,
                  name: &str,
                  dc: Option<&str>,
                  near: Option<&str>,
                  passing: Option<bool>)
                  -> Result<Vec<Check>, ConsulError> {
        Url::parse(&format!("{}/checks/{}", self.endpoint, name))
            .map_err(|_| ConsulError::BadURL)
            .and_then(|mut url| {
                let mut query_pairs = url.query_pairs_mut();
                if let Some(dc) = dc {
                    query_pairs.append_pair("dc", dc);
                }
                if let Some(near) = near {
                    query_pairs.append_pair("near", near);
                }
                if let Some(passing) = passing {
                    query_pairs.append_pair("passing", &format!("{}", passing));
                }
                // TODO: This doesn't seem correct.  I must test it and clean it up
                let f = query_pairs.finish().to_owned();
                self.client.get(f).send().map_err(|_| ConsulError::HTTPFailure)
            })
            .and_then(|mut response: Response| {
                let mut body: String = String::new();
                match response.read_to_string(&mut body) {
                    Ok(_) => Ok(body),
                    Err(_) => Err(ConsulError::HTTPFailure),
                }
            })
            .and_then(|body| json::decode(&body).map_err(|_| ConsulError::BadJSON))
    }
}
