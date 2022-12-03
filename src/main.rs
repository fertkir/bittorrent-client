use serde_derive::Deserialize;
use serde_with::Bytes;
use serde_with::serde_as;

use crate::torrent::TorrentFile;

mod torrent;

#[serde_as]
#[derive(Deserialize, Debug)]
struct TrackerResponse {
    interval: u32,
    #[serde_as(as = "Bytes")]
    peers: Vec<u8>,
}

fn query_tracker(flat_torrent: &TorrentFile) -> TrackerResponse {
    let url = flat_torrent.announce.to_string() + "?uploaded=0&downloaded=0&compact=1\
        &info_hash=" + &urlencoding::encode_binary(&flat_torrent.info_hash)
        + "&port=" + &PORT.to_string()
        + "&peer_id=jfa68h4w7i8eghei8rdf" // todo regenerate peer id
        + "&left=" + &flat_torrent.length.to_string();
    println!("{}", &url);
    let result = reqwest::blocking::get(&url).unwrap().bytes().unwrap();
    bt_bencode::from_slice(&result.as_ref()).unwrap()
}

const PORT: i32 = 9876;

fn main() {
    let filepath = "/home/fertkir/Downloads/debian-11.5.0-amd64-netinst.iso.torrent";
    let torrent = torrent::parse(filepath);
    let response = query_tracker(&torrent);
    println!("{:?}", response)
}