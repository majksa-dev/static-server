use std::path::PathBuf;

use crate::env::Env;

pub struct Builder {
    pub threads: usize,
    pub port: u16,
    pub health_check_port: u16,
    pub server_root: PathBuf,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            threads: 4,
            port: 80,
            health_check_port: 9000,
            server_root: PathBuf::from("./static"),
        }
    }
}

impl Builder {
    pub fn load_env(mut self, env: &Env) -> Self {
        if let Some(threads) = env.threads {
            self.threads = threads;
        }
        if let Some(port) = env.port {
            self.port = port;
        }
        if let Some(health_check_port) = env.health_check_port {
            self.health_check_port = health_check_port;
        }
        if let Some(server_root) = &env.server_root {
            self.server_root = server_root.clone();
        }
        self
    }

    pub fn threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn health_check_port(mut self, health_check_port: u16) -> Self {
        self.health_check_port = health_check_port;
        self
    }

    pub fn server_root(mut self, server_root: PathBuf) -> Self {
        self.server_root = server_root;
        self
    }

    pub fn run_forever(self) {
        super::run_forever(self);
    }
}
