use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub content: String,
}

pub fn serialize_request(request: &Request) -> String {
    serde_json::to_string(request).expect("Failed to serialize request")
}

pub fn deserialize_request(data: &str) -> Request {
    serde_json::from_str(data).expect("Failed to deserialize request")
}

pub fn serialize_response(response: &Response) -> String {
    serde_json::to_string(response).expect("Failed to serialize response")
}

pub fn deserialize_response(data: &str) -> Response {
    serde_json::from_str(data).expect("Failed to deserialize response")
}
