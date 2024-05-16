use static_server::{env::Env, http};

fn main() {
    essentials::install();
    let env = Env::new().unwrap();
    http::build_from_env(&env).run_forever();
}
