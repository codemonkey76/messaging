use clap::Parser;
use config::{Config, File};
use serde::Deserialize;
use std::path::PathBuf;
use tokio;

use clicksend::{clicksend::ClickSendApi, AppError, AppResult, ClickSendClient};

#[derive(Parser, Debug)]
#[command(name = "Message Sender")]
#[command(author = "Shane Poppleton")]
#[command(version = "1.0")]
#[command(about = "Send SMS using ClickSend", long_about = None)]
struct Cli {
    #[arg(short, long)]
    sender: String,

    #[arg(short, long)]
    recipient: String,

    #[arg(short, long)]
    message: String,
}

#[derive(Debug, Deserialize)]
struct ClickSendConfig {
    api_key: String,
    username: String,
    base_url: String,
    version: String,
}

impl ClickSendConfig {
    fn from_file(path: &PathBuf) -> Result<Self, config::ConfigError> {
        let config = Config::builder()
            .add_source(File::from(path.as_path()))
            .build()?;

        config.try_deserialize::<ClickSendConfig>()
    }
}

#[tokio::main]
async fn main() -> AppResult<()> {
    // 1. Parse command-line arguments
    let args = Cli::parse();

    // 2. Load the config.toml from ~/.config/messaging
    let config_path = dirs::config_dir()
        .unwrap()
        .join("messaging")
        .join("config.toml");

    if !config_path.exists() {
        eprintln!("Error: config.toml not found at {:?}", config_path);
        std::process::exit(1);
    }

    let config = ClickSendConfig::from_file(&config_path).expect("Failed to load config file");

    // 3. Initialize the clicksend client
    let client = ClickSendClient::new(
        &config.api_key,
        &config.username,
        &config.base_url,
        &config.version,
    )?;

    client
        .send_single_sms(&args.recipient, &args.sender, &args.message)
        .await?;

    Ok(())
}
