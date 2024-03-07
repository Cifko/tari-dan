use std::{net::UdpSocket, path::PathBuf};

use lazy_static::lazy_static;

pub struct Config {
    pub minotari_path: PathBuf,
    pub data_folder: PathBuf,
    pub local_ip: String,
}

fn env_or_default(env: &str, default: &str) -> String {
    std::env::var(env).unwrap_or(default.to_string())
}

fn get_local_ip() -> String {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind socket");
    socket.connect("10.255.255.255:1").expect("Couldn't connect to address");
    socket
        .local_addr()
        .expect("Couldn't get local address")
        .ip()
        .to_string()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            minotari_path: env_or_default("TARI_BINS_FOLDER", "bins").into(),
            data_folder: env_or_default("DATA_FOLDER", "Data").into(),
            local_ip: get_local_ip(),
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::default();
}
