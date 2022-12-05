use rand::distributions::{Alphanumeric, DistString};

use crate::handshake::handshake;
use crate::torrent::TorrentFile;

mod torrent;
mod tracker;
mod handshake;


fn main() {
    let filepath = "/home/fertkir/Downloads/debian-11.5.0-amd64-netinst.iso.torrent";
    let torrent = TorrentFile::new(filepath);
    let peer_id = generate_peer_id();
    let tracker_response = tracker::query(&torrent, &peer_id);
    println!("{:?}", tracker_response);

    for peer in tracker_response.peers {
        let stream = match handshake(&peer, torrent.info_hash, &peer_id) {
            Err(reason) => {
                println!("Handshake with {} failed: {}", peer, reason);
                continue;
            }
            Ok(stream) => stream
        };
        println!("Handshake with {} succeeded", peer)
    }
}


fn generate_peer_id() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 20)
}