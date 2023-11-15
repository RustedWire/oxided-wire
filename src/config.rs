use std::net::{SocketAddr, IpAddr};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub local: Addr,
    pub remote: Addr,
}

#[derive(Deserialize)]
pub struct Addr {
    pub ip: String,
    pub port: u16,
}

impl From<String> for Config {
    fn from(toml: String) -> Self {
        match  toml::from_str(&toml) {
            Ok(config) => config,
            Err(e) => panic!("Error parsing string into Config: {}", e.to_string()),
        }
    }
}

impl Into<SocketAddr> for Addr {
    fn into(self) -> SocketAddr {
        let ip: IpAddr = match self.ip.parse() {
            Ok(ip) => ip,
            Err(e) => panic!("Error parsing ip: {}", e.to_string()),
        };
    
        SocketAddr::new(ip, self.port)
    }
}