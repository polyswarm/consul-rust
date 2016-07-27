extern crate hyper;
extern crate consul;


use hyper::server::{Server, Request, Response};
use consul::Client;
use hyper::method::Method;
use hyper::uri::RequestUri::AbsolutePath;
use hyper::status::StatusCode;
fn handler(req: Request, mut res: Response) {
    if let AbsolutePath(path) = req.uri{
        println!("We have a request at {:?}", path);
        match path.as_ref() {
            "/v1/agent/services" =>{
                let resp = b"
{
    \"redis\": {
        \"ID\": \"redis\",
        \"Service\": \"redis\",
        \"Tags\": null,
        \"Address\": \"\",
        \"Port\": 8000
    }
}";
                if req.method == Method::Get {
                    res.send(resp).unwrap();
                }
            }
            "/v1/agent/members" =>{
                let resp = b"
[
    {
        \"Name\": \"foobar\",
        \"Addr\": \"10.1.10.12\",
        \"Port\": 8301,
        \"Tags\": {
            \"bootstrap\": \"1\",
            \"dc\": \"dc1\",
            \"port\": \"8300\",
            \"role\": \"consul\"
        },
        \"Status\": 1,
        \"ProtocolMin\": 1,
        \"ProtocolMax\": 2,
        \"ProtocolCur\": 2,
        \"DelegateMin\": 1,
        \"DelegateMax\": 3,
        \"DelegateCur\": 3
    }
]";
                if req.method == Method::Get {
                    res.send(resp).unwrap();
                }
            }
            "/v1/agent/check/pass/redis" =>{
                if req.method != Method::Get {
                    *res.status_mut() = StatusCode::BadRequest;
                }
            }
            "/v1/agent/check/pass/service?note=stuff" =>{
                if req.method != Method::Get {
                    *res.status_mut() = StatusCode::BadRequest;
                }
            }
            "/v1/catalog/services" => {
                let resp = b"{
  \"consul\": [],
  \"redis\": [],
  \"postgresql\": [
    \"master\",
    \"slave\"
  ]
}";
                if req.method == Method::Get {
                    res.send(resp).unwrap();
                }
            }
            "/v1/health/services/redis" => {
                let resp = b"[
  {
    \"Node\": {
      \"Node\": \"foobar\",
      \"Address\": \"10.1.10.12\",
      \"TaggedAddresses\": {
        \"wan\": \"10.1.10.12\"
      }
    },
    \"Service\": {
      \"ID\": \"redis\",
      \"Service\": \"redis\",
      \"Tags\": null,
      \"Address\": \"10.1.10.12\",
      \"Port\": 8000
    },
    \"Checks\": [
      {
        \"Node\": \"foobar\",
        \"CheckID\": \"service:redis\",
        \"Name\": \"Service 'redis' check\",
        \"Status\": \"passing\",
        \"Notes\": \"\",
        \"Output\": \"\",
        \"ServiceID\": \"redis\",
        \"ServiceName\": \"redis\"
      },
      {
        \"Node\": \"foobar\",
        \"CheckID\": \"serfHealth\",
        \"Name\": \"Serf Health Status\",
        \"Status\": \"passing\",
        \"Notes\": \"\",
        \"Output\": \"\",
        \"ServiceID\": \"\",
        \"ServiceName\": \"\"
      }
    ]
  }
]";
                if req.method == Method::Get {
                    res.send(resp).unwrap();
                }

            }
            _ =>{
                *res.status_mut() = StatusCode::NotFound;
                res.send(b"garbage response").unwrap();
            }
        }
    }else{
        *res.status_mut() = StatusCode::InternalServerError;
    }
}

#[test]
fn test_agent_sevices(){
    let mut server = Server::http("localhost:8500").unwrap().handle(handler).unwrap();
    let client = Client::new("localhost:8500");
    let services = client.agent.services().unwrap();
    assert_eq!(services.len(), 1);
    let redis = services.get("redis").unwrap();
    assert_eq!(redis.ID, "redis");
    let _ = server.close();
}

#[test]
fn test_agent_members(){
    let mut server = Server::http("localhost:8501").unwrap().handle(handler).unwrap();
    let client = Client::new("localhost:8501");
    let mut members = client.agent.members().unwrap();
    assert_eq!(members.len(), 1);
    let first = members.pop().unwrap();
    assert_eq!(first.Port, 8301);
    assert_eq!(first.Addr, "10.1.10.12");
    let _ = server.close();
}
//TODO: Write this test.
/*#[test]
fn test_register_ttl_check(){
    let mut server = Server::http("localhost:8502").unwrap().handle(handler).unwrap();
    let client = Client::new("localhost:8502");
    let body = TtlHealthCheck {
        ServiceID: ,
        ID: ,
        Name: ,
        Notes: ,
        TTL: ,
    }

    client.agent.register_ttl_check()
}*/

#[test]
fn test_check_pass(){
    let mut server = Server::http("localhost:8503").unwrap().handle(handler).unwrap();
    let client = Client::new("localhost:8503");
    client.agent.check_pass("redis", None).unwrap();
    client.agent.check_pass("service", Some("stuff")).unwrap();
    let _ = server.close();
}

//TODO: test agent::get_self


#[test]
fn test_catalog_services(){
    let mut server = Server::http("localhost:8504").unwrap().handle(handler).unwrap();
    let client = Client::new("localhost:8504");
    let services = client.catalog.services();
    let reply = services.unwrap();
    assert_eq!(reply.get("redis").unwrap().len(), 0);
    assert_eq!(reply.get("postgresql").unwrap().len(), 2);
    let _ = server.close();
}

#[test]
fn test_health_service(){
    let mut server = Server::http("localhost:8505").unwrap().handle(handler).unwrap();
    let client = Client::new("localhost:8505");
    let reply = client.health.service("redis", None,None,None,None).unwrap();
    assert_eq!(reply.len(), 1);
    assert_eq!(reply[0].Node.Node, "foobar");
    let _ = server.close();
}
