use std::io::{Error, Read, Result, Write};
use std::io::ErrorKind;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

#[derive(Debug)]
struct Handshake {
    p_str: String,
    info_hash: [u8; 20],
    peer_id: Vec<u8>,
}

pub fn handshake(peer: &SocketAddr, info_hash: [u8; 20], peer_id: &str) -> Result<TcpStream> {
    let handshake = Handshake {
        p_str: String::from("BitTorrent protocol"),
        info_hash,
        peer_id: peer_id.as_bytes().to_vec(),
    };
    let timeout = Duration::new(3, 0);
    let mut stream = TcpStream::connect_timeout(peer, timeout)?;
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;
    stream.write_all(&serialize(&handshake))?;
    let mut response_buf = [0; 68];
    stream.read_exact(&mut response_buf)?;
    if deserialize(&response_buf).info_hash == info_hash {
        Ok(stream)
    } else {
        Err(Error::new(ErrorKind::Other, "info_hash doesn't match"))
    }
}

fn serialize(handshake: &Handshake) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    result.push(19);
    result.extend(handshake.p_str.as_bytes());
    result.extend([0; 8]);
    result.extend(handshake.info_hash);
    result.extend(handshake.peer_id.as_slice());
    // println!("{:?}", result);
    result
}

fn deserialize(handshake: &[u8; 68]) -> Handshake {
    Handshake {
        p_str: String::from(std::str::from_utf8(&handshake[1..20]).unwrap()),
        info_hash: handshake[28..48].try_into().unwrap(),
        peer_id: handshake[48..68].to_vec(),
    }
}