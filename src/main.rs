// main.rs

use anyhow::Result;
use std::env;

mod encoder;
mod decoder;
mod peer;
mod torrent;
mod tracker;
mod utils;

// Usage: your_program.sh decode "<encoded_value>"
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decoder::decode_bencoded_value(encoded_value)?;
        println!("{}", decoded_value.to_string());
    } else if command == "info" {
        let torrent_file_name = &args[2];
        let torrent = decoder::decode_torrent_file(torrent_file_name)?;
        utils::print_torrent(&torrent)?;
    } else if command == "peers" {
        let torrent_file_name = &args[2];
        let torrent = decoder::decode_torrent_file(torrent_file_name)?;
        let raw_response = tracker::track_request(&torrent)?.bytes()?;
        let response = decoder::decode_tracker_response(&raw_response)?;
        utils::print_peers(&response);
    } else if command == "handshae" {
        unimplemented!()
    } else {

        println!("unknown command: {}", args[1]);
    }

    Ok(())
}
