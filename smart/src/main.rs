use std::env;

use smart::{config, daemon};
use anyhow::Result;
use clap::{Parser, Subcommand};
use tokio::fs;
use validator::Validate;
#[derive(Parser)]
#[clap(
    version = utils::version::get_version(), 
    about = "smart money bot",
)]
#[clap(propagate_version = true)]
struct Cli {
    #[arg(short, long, default_value = "app.toml")]
    config: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Daemon, // daemon command
    Web, // web服务
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let config_path = cli.config;
    let c: config::Config = fs::read_to_string(&config_path).await?.parse()?;
    c.validate()?;

    env::set_var("SMART_CONFIG", config_path);

    utils::log::init_tracing();

    match cli.command {
        Some(Commands::Daemon) => {
            daemon::daemon().await;
        }
        Some(Commands::Web) => {
            smart::web::start_server().await?;
        }
        None => {
            println!("Please specify a subcommand");
        }
    }

    Ok(())
}