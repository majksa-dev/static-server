use std::{
    net::{SocketAddr, TcpListener},
    sync::Arc,
};

use essentials::info;

use crate::{env::Env, utils};

use self::builder::Builder;

mod builder;
pub mod health_check;
pub mod server;

pub fn create() -> Builder {
    Builder::default()
}

pub fn build_from_env(env: &Env) -> Builder {
    create().load_env(env)
}

fn run_forever(context: Builder) {
    let listener = TcpListener::bind(SocketAddr::from((context.host, context.port)))
        .expect("Port is already in use");
    let pool =
        utils::thread::ThreadPool::new(context.threads).expect("Could not create thread pool");
    let ctx = Arc::new(context);
    let ctx2 = ctx.clone();
    info!("HTTP server running at port {}", ctx.port);
    std::thread::spawn(move || {
        crate::http::health_check::run_forever(ctx);
    });
    server::run_forever(ctx2, listener, pool);
}
