extern crate d5_futures;
use std::net::SocketAddr;

use tokio::net::{TcpListener, TcpStream};

async fn process_socket(socket: TcpStream) {
    // ...
}

use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket).await;
    }
}
