use crate::mpmc::{self, Receiver, Sender, TryRecvError};

pub struct HttpServer {
    // TODO: implement
}

impl HttpServer {
    #[must_use]
    pub fn new(port: u16, num_workers: u32) -> Self {
        unimplemented!("http server")
    }
}
