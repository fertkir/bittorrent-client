use std::net::{IpAddr, Ipv4Addr};

use serde_derive::Deserialize;
use serde_with::Bytes;
use serde_with::serde_as;

use crate::torrent::TorrentFile;

const PORT: i32 = 9876; // todo generate

#[derive(Debug)]
pub struct TrackerResponse {
    pub interval: u32,
    pub peers: Vec<Peer>,
}

#[derive(Debug)]
pub struct Peer {
    pub ip: IpAddr,
    pub port: u16,
}

#[serde_as]
#[derive(Deserialize)]
struct TrackerResp {
    interval: u32,
    #[serde_as(as = "Bytes")]
    peers: Vec<u8>,
}

pub fn query(torrent: &TorrentFile) -> TrackerResponse {
    let resp = query_tracker(torrent);
    TrackerResponse {
        interval: resp.interval,
        peers: parse_peers(&resp.peers),
    }
}

fn query_tracker(torrent: &TorrentFile) -> TrackerResp {
    let url = torrent.announce.to_string() + "?uploaded=0&downloaded=0&compact=1\
        &info_hash=" + &urlencoding::encode_binary(&torrent.info_hash)
        + "&port=" + &PORT.to_string()
        + "&peer_id=jfa68h4w7i8eghei8rdf" // todo generate
        + "&left=" + &torrent.length.to_string();
    println!("{}", &url);
    let result = reqwest::blocking::get(&url).unwrap().bytes().unwrap();
    bt_bencode::from_slice(&result.as_ref()).unwrap()
}

fn parse_peers(peers: &[u8]) -> Vec<Peer> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < peers.len() {
        result.push(Peer {
            ip: IpAddr::V4(Ipv4Addr::new(peers[i], peers[i + 1], peers[i + 2], peers[i + 3])),
            port: ((peers[i + 4] as u16) << 8) | peers[i + 5] as u16,
        });
        i = i + 6
    }
    result
}