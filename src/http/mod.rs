use crate::env::Env;

use self::builder::Builder;

mod builder;
pub mod server;

pub fn create() -> Builder {
    Builder::default()
}

pub fn build_from_env(env: &Env) -> Builder {
    create().load_env(env)
}
