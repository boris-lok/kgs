use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response {
    id: u128,
}

impl From<u128> for Response {
    fn from(e: u128) -> Self {
        Self { id: e }
    }
}
