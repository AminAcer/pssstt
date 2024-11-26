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

#[cfg(test)]
mod tests {
    use crate::common::messages::Response;
    use crate::server::Server;

    use super::*;
    use std::thread;

    #[test]
    fn test_send_message() {
        // Start a server in a separate thread
        thread::spawn(|| {
            let server_id = ServiceID::new("test_service".to_owned(), "test_desc".to_owned());
            let server = Server::new(server_id, "tcp://0.0.0.0:5555");
            let closure = |msg: String| {
                let des_msg = serde_json::from_str::<Message>(&msg).unwrap();
                let response = format!("Message \"{}\" Received", des_msg.content);
                return response;
            };
            Server::start(server, closure);
        });

        let client = Client::new(ServiceID::default(), "tcp://localhost:5555");
        let res = client.send_message("TestMessage".to_owned());
        let de_res = serde_json::from_str::<Response>(&res);
        assert!(de_res.is_ok());

        let result = de_res.unwrap();
        assert_eq!(&result.header.service.name, "test_service");
        assert_eq!(&result.header.service.description, "test_desc");
        assert_eq!(
            &result.header.service.uuid,
            &Uuid::new_v5(&Uuid::NAMESPACE_DNS, "test_service".as_bytes())
        );
        assert!(&result.content.contains("TestMessage"));
    }
}
