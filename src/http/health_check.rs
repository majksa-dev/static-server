use std::{
    io::Write,
    net::{SocketAddr, TcpListener},
    sync::Arc,
};

use essentials::info;

use super::builder::Builder;

const HEALTH_CHECK_RESPONSE: &[u8; 38] = b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";

pub fn run_forever(context: Arc<Builder>) {
    let listener = TcpListener::bind(SocketAddr::from((context.host, context.health_check_port)))
        .expect("Health Port is already in use");
    info!(
        "Health check server running at port {}",
        context.health_check_port
    );
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        stream.write_all(HEALTH_CHECK_RESPONSE).unwrap();
    }
}
