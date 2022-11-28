use std::fs::File;
use std::io::Read;
use std::iter::{Flatten, Map};
use http::Request;
use serde_derive::Deserialize;
use serde_with::serde_as;
use serde_with::Bytes;
use url::Url;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
struct Torrent {
    announce: String,
    info: Info,
}

#[serde_as]
#[derive(Deserialize, Debug)]
struct Info {
    #[serde_as(as = "Bytes")]
    pieces: Vec<u8>,
    #[serde(rename = "piece length")]
    piece_length: u32,
    length: u32,
    name: String,
}

#[derive(Debug)]
struct FlatTorrent {
    announce: String,
    info_hash: [u8; 20],
    piece_hashes: Vec<[u8; 20]>,
    piece_length: u32,
    length: u32,
    name: String
}

fn flatten(torrent: Torrent) -> FlatTorrent {
    FlatTorrent {
        announce: torrent.announce,
        info_hash: [],
        piece_hashes: ,
        piece_length: torrent.info.piece_length,
        length: torrent.info.length,
        name: torrent.info.name,
    }
}

fn parse_torrent(filepath: &str) -> Torrent {
    let mut file = match File::open(filepath) {
        Ok(file) => file,
        Err(reason) => panic!("couldn't open {}: {}", filepath, reason)
    };
    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => println!("read {}", filepath),
        Err(reason) => panic!("couldn't read {}: {}", filepath, reason)
    };
    match bt_bencode::from_slice(&buffer) {
        Ok(value) => value,
        Err(reason) => panic!("couldn't parse .torrent: {}", reason)
    }
}

fn main() {
    let filepath = "/home/fertkir/Downloads/debian-11.5.0-amd64-netinst.iso.torrent";
    let torrent = parse_torrent(filepath);
    println!("{:?}", torrent);
    let flat_torrent = flatten(torrent);
    println!("{:?}", flat_torrent);
    let url = Url::parse_with_params(&torrent.announce, &[
        ("info_hash", flat_torrent.info_hash),
        ("peer_id", Uuid::new_v4().to_string()),
        ("port", ),
        ("uploaded", "0"),
        ("downloaded", "0"),
        ("compact", "1"),
        ("left", torrent.info.length.to_string())
    ])?;

    let request = Request::builder()
        .uri(url.as_str());
    let response = send(request.body(()));
}