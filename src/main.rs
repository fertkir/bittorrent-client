use std::io::{Read, Result, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

use rand::distributions::{Alphanumeric, DistString};

mod torrent;
mod tracker;

#[derive(Debug)]
struct Handshake {
    p_str: String,
    info_hash: [u8; 20],
    peer_id: Vec<u8>,
}

fn main() {
    let filepath = "/home/fertkir/Downloads/debian-11.5.0-amd64-netinst.iso.torrent";
    let torrent = torrent::parse(filepath);
    let peer_id = generate_peer_id();
    let tracker_response = tracker::query(&torrent, &peer_id);
    println!("{:?}", tracker_response);
    let handshake = Handshake {
        p_str: "BitTorrent protocol".to_string(),
        info_hash: torrent.info_hash,
        peer_id: peer_id.into_bytes(),
    };
    let serialized_handshake = serialize(&handshake);

    println!("{:?}", serialized_handshake);

    for peer in tracker_response.peers {
        match send_handshake(&peer, &serialized_handshake) {
            Ok(response) => {
                println!("{:?}", response)
            }
            Err(_) => {}
        }
    }
}

fn send_handshake(peer: &SocketAddr, handshake: &[u8]) -> Result<[u8; 68]> {
    let timeout = Duration::new(3, 0);
    let mut stream = TcpStream::connect_timeout(peer, timeout)?;
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;
    stream.write_all(handshake)?;
    let mut buf = [0; 68];
    stream.read_exact(&mut buf)?;
    Ok(buf)
}

fn serialize(handshake: &Handshake) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    result.push(19);
    result.extend(handshake.p_str.as_bytes());
    result.extend([0; 8]);
    result.extend(handshake.info_hash);
    result.extend(handshake.peer_id.as_slice());
    result
}

fn deserialize(handshake: &[u8; 68]) -> Handshake {
    Handshake {
        p_str: String::from("handshake[1..21]"),
        info_hash: [0; 20], // handshake[21..42].try_into().unwrap()
        peer_id: handshake[42..69].to_vec(),
    }
}

fn generate_peer_id() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 20)
}