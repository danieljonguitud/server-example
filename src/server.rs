use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self {
            address
        }
    }

    pub fn run(self) {
        print!("Listing on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {

        }
    }
}