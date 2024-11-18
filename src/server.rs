use chrono::Utc;
use uuid::Uuid;
use zmq;

use crate::common::messages::{Header, Response, ServiceID};

pub struct Server {
    service_id: ServiceID,
    socket: zmq::Socket,
}

impl Server {
    pub fn new(svc_id: ServiceID, bind_address: &str) -> Self {
        let service_id = svc_id;
        let context = zmq::Context::new();
        let socket = context.socket(zmq::REP).expect("Failed to create socket");
        socket.bind(bind_address).expect("Failed to bind socket");

        Self { service_id, socket }
    }

    pub fn start<F>(&self, handler: F)
    where
        F: Fn(String) -> String,
    {
        loop {
            let request = self
                .socket
                .recv_string(0)
                .expect("Failed to receive message");
            if let Ok(request) = request {
                let content = handler(request);
                let header = Header {
                    service: self.service_id.clone(),
                    message_id: Uuid::new_v4(),
                    timestamp: Utc::now(),
                };
                let res = Response { header, content };
                self.socket
                    .send(&serde_json::to_string(&res).unwrap(), 0)
                    .expect("Failed to send response");
            }
        }
    }
}
