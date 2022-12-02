use std::fs::File;
use std::io::Read;
use serde_derive::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::Bytes;
use sha1::{Sha1, Digest};

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

#[derive(Debug)]
struct FlatTorrent {
    announce: String,
    info_hash: [u8; 20],
    // todo should use Vec<[u8; 20]> below:
    piece_hashes: Vec<Vec<u8>>,
    piece_length: u32,
    length: u32,
    name: String,
}

fn flatten(torrent: Torrent) -> FlatTorrent {
    FlatTorrent {
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

fn query_tracker(flat_torrent: &FlatTorrent) -> String {
    let url = flat_torrent.announce.to_string() + "?uploaded=0&downloaded=0&compact=1\
        &info_hash=" + &urlencoding::encode_binary(&flat_torrent.info_hash)
        + "&port=" + &PORT.to_string()
        + "&peer_id=jfa68h4w7i8eghei8rdf"
        + "&left=" + &flat_torrent.length.to_string();
    println!("{}", &url);
    reqwest::blocking::get(&url).unwrap().text().unwrap()
}

const PORT: i32 = 9876;

fn main() {
    let filepath = "/home/fertkir/Downloads/debian-11.5.0-amd64-netinst.iso.torrent";
    let torrent = parse_torrent(filepath);
    let flat_torrent = flatten(torrent);
    println!("{:?}", flat_torrent);
    let response = query_tracker(&flat_torrent);
    println!("{}", response)
}