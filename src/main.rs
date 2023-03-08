use std::net::{UdpSocket, TcpStream};
use std::sync::Arc;
use std::{str, env};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listen = env::var("LISTEN").unwrap();
    let socket = Arc::new(UdpSocket::bind(listen)?);
    let mut buf = [0u8; 1024];
    
    loop {
        let (len, _) = socket.recv_from(&mut buf).expect("");
        let content = str::from_utf8(&buf[..len - 1]).unwrap();
        let content = String::from(content);
        let udp_socket = socket.clone();
        tokio::spawn(async move {
            let send_buf = if let Ok(_) = TcpStream::connect(content) {
                "Up"
            } else {
                "Down"
            }.as_bytes();
            udp_socket.send(send_buf).expect("");
        });
    }
}
