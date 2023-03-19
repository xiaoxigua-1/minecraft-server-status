mod get_server_status;

use std::env;
use std::net::SocketAddr;
use std::str;
use std::sync::Arc;

use tokio::net::UdpSocket;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listen = env::var("LISTEN").unwrap();
    let mut buf = [0;1024];
    let udp_socket = UdpSocket::bind(listen).await?;
    let udp_scoket_arc = Arc::new(udp_socket);    
    let udp_scoket_clone = udp_scoket_arc.clone(); 
    let (tx, mut rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1_000);

    tokio::spawn(async move {
        while let Some((buf, addr)) = rx.recv().await {
            match str::from_utf8(&buf).unwrap() {
                _ => {}
            }
        } 
    });

    loop {
        let (_, addr) = udp_scoket_arc.recv_from(&mut buf).await?;
        
        tx.send((buf.to_vec(), addr)).await.expect("");
    }
}
