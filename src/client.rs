use std::sync::Arc;
use hyper::client::Client as HttpClient;
use super::Agent;
use super::Catalog;
use super::Health;


#[derive(Clone)]
pub struct Client {
    pub agent: Agent,
    pub catalog: Catalog,
    pub health: Health,
}


impl Client {
    pub fn new(address: &str) -> Client {
        let http_client = Arc::new(HttpClient::new());
        Client {
            agent: Agent::new(http_client.clone(), address),
            catalog: Catalog::new(http_client.clone(), address),
            health: Health::new(http_client.clone(), address),
        }
    }
}
