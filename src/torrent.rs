use std::fs::File;
use std::io::Read;

use serde_derive::{Deserialize, Serialize};
use serde_with::Bytes;
use serde_with::serde_as;
use sha1::{Digest, Sha1};

#[derive(Debug)]
pub struct TorrentFile {
    pub announce: String,
    pub info_hash: [u8; 20],
    // todo should use Vec<[u8; 20]> below:
    pub piece_hashes: Vec<Vec<u8>>,
    pub piece_length: u32,
    pub length: u32,
    pub name: String,
}

#[derive(Deserialize, Debug)]
struct Torrent {
    announce: String,
    info: Info,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct Info {
    #[serde_as(as = "Bytes")]
    pieces: Vec<u8>,
    #[serde(rename = "piece length")]
    piece_length: u32,
    length: u32,
    name: String,
}

pub fn parse(filepath: &str) -> TorrentFile {
    flatten(parse_torrent(filepath))
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

fn flatten(torrent: Torrent) -> TorrentFile {
    TorrentFile {
        announce: torrent.announce,
        info_hash: to_sha1(&torrent.info),
        piece_hashes: torrent.info.pieces.chunks(20).map(|s| s.into()).collect(),
        piece_length: torrent.info.piece_length,
        length: torrent.info.length,
        name: torrent.info.name,
    }
}

fn to_sha1(torrent_info: &Info) -> [u8; 20] {
    let mut hasher = Sha1::new();
    hasher.update(bt_bencode::to_vec(torrent_info).unwrap());
    <[u8; 20]>::from(hasher.finalize())
}