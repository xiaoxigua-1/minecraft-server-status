use std::{net::TcpStream, io::{Write, Read}, time::{SystemTime, UNIX_EPOCH}, usize};
use std::str;

use tokio::io::AsyncWriteExt;

pub async fn get_server_status(addr: &str, port: u16) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", addr, port))?;

    let handshake_buf = create_handshake_message(addr, port).await?;

    packet(&mut stream, 0x00, handshake_buf)?;

    status(&mut stream)?;
    Ok(())
}

async fn create_handshake_message(addr: &str, port: u16) -> std::io::Result<Vec<u8>> {
    let mut buf: Vec<u8> = vec![];
    write_var_int(&mut buf, 760);
    write_string(&mut buf, addr.to_string());
    buf.write_u16(port).await?;
    write_var_int(&mut buf, 1);

    Ok(buf) 
}

fn write_string(buf: &mut Vec<u8>, s: String) {
    write_var_int(buf, s.len());
    buf.append(&mut s.into_bytes());
}

fn write_var_int(buf: &mut Vec<u8>, number: usize) {
    let mut number = number;
    loop {
        if (number & 0xFFFFFF80) == 0 {
            buf.push(number as u8);
            break;
        }

        buf.push((number & 0x7F | 0x80) as u8);
        number >>= 7;
    }
}

fn status(stream: &mut TcpStream) -> std::io::Result<()> {
    packet(stream, 0x00, vec![])?;
    let reponse_len = read_var_int(stream)?;
    let reponse_state = read_var_int(stream)?;

    let mut reponse = vec![0; (reponse_len - 2) as usize];
    stream.read(&mut reponse)?;
    println!("{:?}", str::from_utf8(&reponse[2..]));

    Ok(())
}

fn read_var_int(stream: &mut TcpStream) -> std::io::Result<i32> {
    let mut value: i32 = 0;
    let mut pos = 0;
    let mut current_byte = [0; 1];

    loop {
        stream.read(&mut current_byte)?;
        value |= ((current_byte[0] & 0x7F) as i32) << pos;

        if (current_byte[0] & 0x80) == 0 {
            break;
        }

        pos += 7;
    }
    
    Ok(value) 
}

fn packet(stream: &mut TcpStream, packet_id: u8, data_buf: Vec<u8>) -> std::io::Result<()> {
    let mut buf: Vec<u8> = vec![];
    buf.push((data_buf.len() + 1) as u8);
    buf.push(packet_id);
    buf.append(&mut data_buf.clone());

    stream.write(&buf)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::get_server_status::get_server_status;

    #[tokio::test]
    async fn get_server_status_test() {
        get_server_status("mc.xigua.tw", 25577).await.unwrap();
    }
}

