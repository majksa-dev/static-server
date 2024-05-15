use static_server::{env::Env, http::server};

fn main() {
    essentials::install();
    let env = Env::new().unwrap();
    server::build_from_env(&env).run_forever();
}
