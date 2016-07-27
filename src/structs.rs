#![allow(non_snake_case)]

use std::collections::HashMap;

/// AgentMember represents a cluster member known to the agent
#[derive(RustcDecodable, RustcEncodable)]
pub struct AgentMember {
    pub Name: String,
    pub Addr: String,
    pub Port: u16,
    pub Tags: HashMap<String, String>,
    pub Status: usize,
    pub ProtocolMin: u8,
    pub ProtocolMax: u8,
    pub ProtocolCur: u8,
    pub DelegateMin: u8,
    pub DelegateMax: u8,
    pub DelegateCur: u8,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Check {
    pub Node: String,
    pub CheckID: String,
    pub Name: String,
    pub Status: String,
    pub Notes: String,
    pub Output: String,
    pub ServiceID: String,
    pub ServiceName: String,
}


/// Node represents a node
#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Node {
    pub Node: String,
    pub Address: String,
    pub TaggedAddresses: Option<TaggedAddress>,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct TaggedAddress {
    pub wan: String,
}

/// Service represents a service
// TODO:  Test this.  I think tags should be hasmap of strings
#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Service {
    pub ID: String,
    pub Service: String,
    pub Tags: Option<Vec<String>>,
    pub Address: String,
    pub Port: u16,
}

/// ServiceHealth is used for the health service
#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct ServiceHealth {
    pub Node: Node,
    pub Service: Service,
    pub Checks: Vec<Check>,
}

/// Service represents a service
#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct RegisterService {
    pub ID: String,
    pub Name: String,
    pub Tags: Vec<String>,
    pub Port: u16,
    pub Address: String,
}


// TODO: This doesn't provide near what the documentation allows
#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct TtlHealthCheck {
    pub ServiceID: String,
    pub ID: Option<String>,
    pub Name: String,
    pub Notes: String,
    pub TTL: String,
}
