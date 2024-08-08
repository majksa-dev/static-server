use std::{net::IpAddr, path::PathBuf};

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Env {
    pub port: Option<u16>,
    pub health_check_port: Option<u16>,
    pub host: Option<IpAddr>,
    pub server_root: Option<PathBuf>,
}

impl Env {
    pub fn new() -> Result<Self, envy::Error> {
        envy::from_env()
    }
}
