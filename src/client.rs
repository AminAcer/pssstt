use zmq;

pub struct Client {
    context: zmq::Context,
    socket: zmq::Socket,
}

impl Client {
    pub fn new(server_address: &str) -> Self {
        let context = zmq::Context::new();
        let socket = context.socket(zmq::REQ).expect("Failed to create socket");
        socket.connect(server_address).expect("Failed to connect to server");

        Self { context, socket }
    }

    pub fn send_message(&self, message: &str) -> String {
        self.socket.send(message, 0).expect("Failed to send message");
        let reply = self.socket.recv_string(0).expect("Failed to receive reply");
        reply.unwrap()
    }
}
