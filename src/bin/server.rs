use essentials::error;
use gateway::Result;
use static_server::{env::Env, http};

async fn run() -> Result<()> {
    let env = Env::new()?;
    http::build_from_env(&env).build().await?.run().await;
    Ok(())
}

#[tokio::main]
async fn main() {
    essentials::install();
    if let Err(err) = run().await {
        error!(error = %err, "Failed to start server:");
        eprintln!("Something went wrong: {}", err);
    }
}
