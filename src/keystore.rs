use hyper::client::{Client, Response};
use super::consul_error::ConsulError;
use hyper::Url;
use rustc_serialize::json;
use std::collections::HashMap;
use std::io::Read;
use hyper::status::StatusCode;

// [
// {
// "CreateIndex": 100,
// "ModifyIndex": 200,
// "LockIndex": 200,
// "Key": "zip",
// "Flags": 0,
// "Value": "dGVzdA==",
// "Session": "adf4238a-882b-9ddc-4a9d-5b6758e4159e"
// }
// ]

#[derive(RustcDecodable, RustcEncodable)]
#[allow(non_snake_case)]
pub struct KeystoreEntry {
    CreateIndex: u32,
    ModifyIndex: u32,
    Key: String,
    Flags: u32,
    Value: String,
    Session: String,
}

pub struct Keystore<'a> {
    endpoint: String,
    client: &'a Client,
}


impl<'a> Keystore<'a> {
    pub fn new(client: &'a Client, address: &str) -> Keystore<'a> {
        Keystore {
            endpoint: format!("{}/v1/kv", address),
            client: client,
        }
    }

    pub fn get(&self, key: &str) -> Result<Vec<KeystoreEntry>, ConsulError> {
        Url::parse(&format!("{}/{}", self.endpoint, key))
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

    pub fn set(&self, key: &str, value: &str) -> Result<bool, ConsulError> {
        Url::parse(&format!("{}/{}", self.endpoint, key))
            .map_err(|_| ConsulError::BadURL)
            .and_then(|url| {
                self.client
                    .put(url)
                    .body(value)
                    .send()
                    .map_err(|_| ConsulError::HTTPFailure)
            })
            .and_then(|mut response: Response| {
                let mut body: String = String::new();
                match response.read_to_string(&mut body) {
                    Ok(_) => {
                        if response.status == StatusCode::Ok {
                            if body.to_lowercase().contains("true") {
                                Ok(true)
                            } else {
                                Ok(false)
                            }
                        } else {
                            Err(ConsulError::RemoteFailure)
                        }
                    }
                    Err(_) => Err(ConsulError::HTTPFailure),
                }
            })

    }

    pub fn delete(&self, key: &str) -> Result<(), ConsulError> {
        Url::parse(&format!("{}/{}", self.endpoint, key))
            .map_err(|_| ConsulError::BadURL)
            .and_then(|url| self.client.delete(url).send().map_err(|_| ConsulError::HTTPFailure))
            .and_then(|response: Response| {
                if response.status == StatusCode::Ok {
                    Ok(())
                } else {
                    Err(ConsulError::RemoteFailure)
                }
            })
    }
}
