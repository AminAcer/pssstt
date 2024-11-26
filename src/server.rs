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

    pub fn start<F>(server: Server, handler: F)
    where
        F: Fn(String) -> String + Send + 'static,
    {
        std::thread::spawn(move || loop {
            let request = server
                .socket
                .recv_string(0)
                .expect("Failed to receive message");
            if let Ok(request) = request {
                let content = handler(request);
                let header = Header {
                    service: server.service_id.clone(),
                    message_id: Uuid::new_v4(),
                    timestamp: Utc::now(),
                };
                let res = Response { header, content };
                server
                    .socket
                    .send(&serde_json::to_string(&res).unwrap(), 0)
                    .expect("Failed to send response");
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;
    use crate::common::messages::{Message, Response};
    use crate::server::Server;

    use super::*;
    use std::thread;

    #[test]
    fn test_recv_message() {
        // Start a server in a separate thread
        thread::spawn(|| {
            let server = Server::new(ServiceID::default(), "tcp://0.0.0.0:5551");
            let closure = |msg: String| {
                let des_msg = serde_json::from_str::<Message>(&msg);
                assert!(des_msg.is_ok());

                let result = des_msg.unwrap();
                assert_eq!(result.header.service.name, "test_client");
                assert_eq!(result.header.service.description, "test_desc");
                assert_eq!(
                    &result.header.service.uuid,
                    &Uuid::new_v5(&Uuid::NAMESPACE_DNS, "test_client".as_bytes())
                );
                assert_eq!(result.content, "TestMessage");
                return String::from("Received");
            };
            Server::start(server, closure);
        });

        let client_id = ServiceID::new("test_client".to_owned(), "test_desc".to_owned());
        let client = Client::new(client_id, "tcp://localhost:5551");
        let res = client.send_message("TestMessage".to_owned());
        assert!(serde_json::from_str::<Response>(&res).is_ok());
    }
}
