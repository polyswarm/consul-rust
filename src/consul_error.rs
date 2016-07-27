#[derive(Debug)]
pub enum ConsulError {
    BadURL,
    HTTPFailure,
    BadJSON,
    RemoteFailure,
}
