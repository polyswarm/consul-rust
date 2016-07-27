
use super::Agent;
use super::Catalog;
use super::Health;
use hyper::client::Client as HttpClient;
use std::rc::Rc;

pub struct Client {
    pub agent: Agent,
    pub catalog: Catalog,
    pub health: Health,
}
// TODO: change to Rc

impl Client {
    pub fn new(address: &str) -> Client {
        let http_client = Rc::new(HttpClient::new());
        Client {
            agent: Agent::new(http_client.clone(), address),
            catalog: Catalog::new(http_client.clone(), address),
            health: Health::new(http_client.clone(), address),
        }
    }
}
