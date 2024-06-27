use std::{net::IpAddr, path::PathBuf};

use gateway::{builder, Result, Server};

use crate::env::Env;

use super::{router::AnyRouterBuilder, server::FileServerBuilder};

pub struct Builder {
    pub port: u16,
    pub host: IpAddr,
    pub health_check_port: u16,
    pub server_root: PathBuf,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            port: 80,
            host: IpAddr::from([127, 0, 0, 1]),
            health_check_port: 9000,
            server_root: PathBuf::from("./static"),
        }
    }
}

impl Builder {
    pub fn load_env(mut self, env: &Env) -> Self {
        if let Some(port) = env.port {
            self.port = port;
        }
        if let Some(host) = env.host {
            self.host = host;
        }
        if let Some(health_check_port) = env.health_check_port {
            self.health_check_port = health_check_port;
        }
        if let Some(server_root) = &env.server_root {
            self.server_root.clone_from(server_root);
        }
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn host(mut self, host: IpAddr) -> Self {
        self.host = host;
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

    pub async fn build(self) -> Result<Server> {
        builder(FileServerBuilder::new(self.server_root), |_| {
            Some(String::new())
        })
        .with_app_port(self.port)
        .with_health_check_port(self.health_check_port)
        .with_host(self.host)
        .register_peer(String::new(), AnyRouterBuilder::new())
        .build()
        .await
    }
}
