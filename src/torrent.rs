// torrent.rs
use serde::{Serialize, Deserialize};

// Decode torrent file
#[derive(Serialize, Deserialize, Debug)]
pub struct TorrentFile {
    pub announce: String,
    pub info: TorrentInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TorrentInfo {
    pub length: usize,
    pub name: String,
    #[serde(rename="piece length")]
    pub piece_length: usize,
    #[serde(with="serde_bytes")]
    pub pieces: Vec<u8>,
}