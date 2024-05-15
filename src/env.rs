use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Env {
    pub port: Option<u16>,
    pub health_check_port: Option<u16>,
    pub threads: Option<usize>,
    pub server_root: Option<PathBuf>,
}

impl Env {
    pub fn new() -> Result<Self, envy::Error> {
        envy::from_env()
    }
}
