use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceID {
    pub name: String,
    pub description: String,
    pub uuid: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    pub service: ServiceID,
    pub message_id: Uuid,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub header: Header,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub header: Header,
    pub content: String,
}
