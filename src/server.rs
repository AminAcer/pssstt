use zmq;

pub struct Server {
    context: zmq::Context,
    socket: zmq::Socket,
}

impl Server {
    pub fn new(bind_address: &str) -> Self {
        let context = zmq::Context::new();
        let socket = context.socket(zmq::REP).expect("Failed to create socket");
        socket.bind(bind_address).expect("Failed to bind socket");

        Self { context, socket }
    }

    pub fn start<F>(&self, handler: F)
    where
        F: Fn(String) -> String,
    {
        loop {
            let request = self.socket.recv_string(0).expect("Failed to receive message");
            if let Ok(request) = request {
                let response = handler(request);
                self.socket.send(&response, 0).expect("Failed to send response");
            }
        }
    }
}
