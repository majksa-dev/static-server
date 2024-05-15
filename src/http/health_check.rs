use std::{
    io::Write,
    net::{SocketAddr, TcpListener},
};

const HEALTH_CHECK_RESPONSE: &[u8; 38] = b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";

pub fn run_forever(port: u16) {
    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], port)))
        .expect("Health Port is already in use");
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        stream.write_all(HEALTH_CHECK_RESPONSE).unwrap();
    }
}
