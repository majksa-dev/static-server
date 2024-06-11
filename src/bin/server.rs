use static_server::{env::Env, http};

#[tokio::main]
async fn main() {
    essentials::install();
    let env = Env::new().unwrap();
    http::build_from_env(&env).build().run().await;
}
