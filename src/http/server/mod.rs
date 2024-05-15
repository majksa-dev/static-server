use std::{
    net::{SocketAddr, TcpListener},
    sync::Arc,
};

use essentials::error;

use crate::{env::Env, utils};

use self::{builder::Builder, handle::handle_connection};

mod builder;
mod handle;
mod request;

pub fn create() -> Builder {
    Builder::default()
}

pub fn build_from_env(env: &Env) -> Builder {
    create().load_env(env)
}

fn run_forever(context: Builder) {
    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], context.port)))
        .expect("Port is already in use");
    let pool =
        utils::thread::ThreadPool::new(context.threads).expect("Could not create thread pool");
    let ctx = Arc::new(context);
    let health_check_port = ctx.health_check_port;
    std::thread::spawn(move || {
        crate::http::health_check::run_forever(health_check_port);
    });
    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                error!("HTTP server failed to accept connection: {e}");
                continue;
            }
        };
        let ctx = ctx.clone();
        let task = pool.execute(move || {
            handle_connection(stream, ctx);
        });
        if task.is_err() {
            error!("HTTP handle channel closed!");
        }
    }
}
