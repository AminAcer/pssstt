use chrono::Utc;
use uuid::Uuid;
use zmq;

use crate::common::messages::{Header, Message, ServiceID};

pub struct Client {
    service_id: ServiceID,
    socket: zmq::Socket,
}

impl Client {
    pub fn new(svc_id: ServiceID, server_address: &str) -> Self {
        let service_id = svc_id;
        let context = zmq::Context::new();
        let socket = context.socket(zmq::REQ).expect("Failed to create socket");
        socket
            .connect(server_address)
            .expect("Failed to connect to server");

        Self { service_id, socket }
    }

    pub fn send_message(&self, content: String) -> String {
        let header = Header {
            service: self.service_id.clone(),
            message_id: Uuid::new_v4(),
            timestamp: Utc::now(),
        };
        let msg = Message { header, content };
        self.socket
            .send(&serde_json::to_string(&msg).unwrap(), 0)
            .expect("Failed to send message");
        let reply = self.socket.recv_string(0).expect("Failed to receive reply");
        reply.unwrap()
    }
}
