use std::{net::TcpListener, sync::Arc};

use essentials::error;

use crate::utils::thread::ThreadPool;

use self::handle::handle_connection;

use super::builder::Builder;

mod handle;
mod request;

pub(super) fn run_forever(ctx: Arc<Builder>, listener: TcpListener, pool: ThreadPool) {
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
