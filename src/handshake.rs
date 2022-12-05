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

impl Handshake {

    fn new(info_hash: [u8; 20], peer_id: &str) -> Handshake {
        Handshake {
            p_str: String::from("BitTorrent protocol"),
            info_hash,
            peer_id: peer_id.as_bytes().to_vec(),
        }
    }

    fn from(bytes: &[u8; 68]) -> Handshake {
        Handshake {
            p_str: String::from(std::str::from_utf8(&bytes[1..20]).unwrap()),
            info_hash: bytes[28..48].try_into().unwrap(),
            peer_id: bytes[48..68].to_vec(),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.push(19);
        result.extend(self.p_str.as_bytes());
        result.extend([0; 8]);
        result.extend(self.info_hash);
        result.extend(self.peer_id.as_slice());
        // println!("{:?}", result);
        result
    }
}

pub fn perform_handshake(peer: &SocketAddr, info_hash: [u8; 20], peer_id: &str) -> Result<TcpStream> {
    let timeout = Duration::new(3, 0);
    let mut stream = TcpStream::connect_timeout(peer, timeout)?;
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;
    stream.write_all(&Handshake::new(info_hash, peer_id).to_bytes())?;
    let mut response_buf = [0; 68];
    stream.read_exact(&mut response_buf)?;
    if Handshake::from(&response_buf).info_hash == info_hash {
        Ok(stream)
    } else {
        Err(Error::new(ErrorKind::Other, "info_hash doesn't match"))
    }
}
