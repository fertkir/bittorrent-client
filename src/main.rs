mod torrent;
mod tracker;


fn main() {
    let filepath = "/home/fertkir/Downloads/debian-11.5.0-amd64-netinst.iso.torrent";
    let torrent = torrent::parse(filepath);
    let response = tracker::query(&torrent);
    println!("{:?}", response);
}