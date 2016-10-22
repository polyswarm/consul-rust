
use std::sync::Arc;
use std::collections::HashMap;
use std::io::Read;
use hyper::client::{Client, Response};
use rustc_serialize::json;

use hyper::Url;
use super::consul_error::ConsulError;

// Catalog can be used to query the Catalog endpoints
#[derive(Clone)]
pub struct Catalog {
    endpoint: String,
    client: Arc<Client>,
}

#[derive(RustcDecodable, RustcEncodable)]
#[allow(non_snake_case)]
pub struct ServiceNode {
    Node: String,
    Address: String,
    ServiceID: String,
    ServiceName: String,
    ServiceTags: Vec<String>,
    ServiceAddress: String,
    ServicePort: u16,
}

impl Catalog {
    pub fn new(client: Arc<Client>, address: &str) -> Catalog {
        Catalog {
            endpoint: format!("http://{}/v1/catalog", address),
            client: client,
        }
    }

    // TODO: add dc flag
    /// https://www.consul.io/docs/agent/http/catalog.html#catalog_services
    pub fn services(&self) -> Result<HashMap<String, Vec<String>>, ConsulError> {
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

    // TODO: add dc, tag and near flags
    /// https://www.consul.io/docs/agent/http/catalog.html#catalog_service
    pub fn service(&self, service: &str) -> Result<Vec<ServiceNode>, ConsulError> {
        Url::parse(&format!("{}/services/{}", self.endpoint, service))
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
}
