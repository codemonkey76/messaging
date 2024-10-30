use std::{collections::HashSet, env, sync::Arc};

use clap::Parser;
use queue::publisher::RabbitMQ;
use rand::{distributions::Alphanumeric, Rng};
use tokio::net::TcpListener;
mod routes;

#[derive(Parser, Debug)]
#[command(name = "API Server")]
#[command(about = "Runs the API server or generates API keys", long_about = None)]
struct Cli {
    /// Generate API Keys
    #[arg(short, long)]
    gen_keys: bool,
}

fn generate_api_key() -> String {
    let key_length = 32;
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(key_length)
        .map(char::from)
        .collect()
}

#[derive(Clone)]
pub struct AppState {
    pub valid_api_keys: HashSet<String>,
    pub rabbitmq: RabbitMQ,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    if args.gen_keys {
        println!("Generating 5 new API keys:");
        for i in 1..=5 {
            let api_key = generate_api_key();
            println!("API_KEY_{}=\"{}\"", i, api_key);
        }
        return;
    }

    dotenv::dotenv().ok();
    let mut valid_api_keys = HashSet::new();

    for (key, value) in env::vars() {
        if key.starts_with("API_KEY_") {
            valid_api_keys.insert(value);
        }
    }

    tracing_subscriber::fmt::init();

    let rabbitmq = match RabbitMQ::new("amqp://127.0.0.1:5672/%2f").await {
        Ok(connection) => connection,
        Err(err) => {
            eprintln!("Failed to initialize RabbitMQ: {:?}", err);
            return;
        }
    };
    let app_state = AppState {
        valid_api_keys,
        rabbitmq,
    };

    let app = routes::app(app_state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
