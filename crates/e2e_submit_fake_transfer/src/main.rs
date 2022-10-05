use clap::Parser;
use config::{Config, Environment, File};
use std::time::Duration;
use tokio::fs;
use tokio::time::timeout;

mod cli;
mod configuration;
mod test;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cli = cli::Cli::parse();

    let mut cfg = Config::builder();
    if let Some(config_path) = cli.config.as_deref() {
        tracing::info!(path = %config_path.display(), "Loading config from file");
        cfg = cfg.add_source(File::from(config_path));
    }

    let cfg = cfg
        .add_source(Environment::with_prefix("ANOMA"))
        .build()
        .unwrap();
    tracing::debug!("Loaded configuration: {:#?}", cfg);
    let cfg: configuration::Config = match cfg.clone().try_deserialize() {
        Ok(cfg) => cfg,
        Err(error) => {
            eprintln!("Could't load configuration: {}", error);
            std::process::exit(1);
        }
    };
    tracing::info!("Parsed configuration: {:#?}", cfg);
    let exit_code = dispatch(cli.command.unwrap_or_default(), cfg).await;
    std::process::exit(exit_code);
}

async fn dispatch(command: cli::Commands, cfg: configuration::Config) -> i32 {
    match command {
        cli::Commands::Test => match test::test(cfg).await {
            Ok(succeeded) => {
                if succeeded {
                    println!("Test succeeded");
                    0
                } else {
                    println!("Test failed");
                    2
                }
            }
            Err(error) => {
                println!("### Test errored: {:#?} ###", error);
                1
            }
        },
        cli::Commands::Clean => {
            // TODO: by default, prompt user with full path to .anoma dir that will be deleted before actually deleting it
            let path = ".anoma/";
            let remove_dir_all_cmd = fs::remove_dir_all(path);
            match timeout(Duration::from_secs(10), remove_dir_all_cmd).await {
                Ok(result) => match result {
                    Ok(()) => {
                        println!("Deleted directory {path}");
                        0
                    }
                    Err(error) => {
                        println!("Error deleting directory {path}: {:#?}", error);
                        1
                    }
                },
                Err(timeout) => {
                    println!("Timed out trying to delete {path}: {:?}", timeout);
                    1
                }
            }
        }
    }
}
