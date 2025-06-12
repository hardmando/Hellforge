mod logger;
use clap::{Parser, Subcommand};
use logger::log_event;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::fmt::Error;
use std::path::{self, Path};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::{fs::OpenOptions, ptr::null};
mod config;
use config::{load_config, save_config};

#[derive(Parser)]
#[command(name = "hellforge", about = "File sync service.", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Watch {
        #[arg(short, long, default_value = "./watched")]
        path: String,
    },

    Mode {
        #[command(subcommand)]
        command: ModeSubcommands,
    },

    Pull {},
    Push {},
}

#[derive(Subcommand)]
enum ModeSubcommands {
    Set {
        #[arg(value_parser = ["instant", "auto", "manual"])]
        mode: String,
    },
    Get,
}

fn main() -> notify::Result<()> {
    let cli = Cli::parse();
    let mut watched_path = String::new();
    let mut config = load_config().unwrap_or_default();

    match cli.command {
        Commands::Watch { path } => {
            println!(
                "\n╔════════════════════╗
                      \n║ Watching directory : {}
                      \n╚════════════════════╝
                ",
                path
            );
            watched_path = path;
            match config.mode.as_str() {
                "instant" => {
                    println!(
                        "\n╔═══════════════╗
                              \n║ Mode: Instant ║ 
                              \n╚═══════════════╝"
                    );
                }
                "auto" => {
                    println!(
                        "\n╔═══════════════╗
                              \n║ Mode: Auto    ║
                              \n║ Interval: {}  ║
                              \n╚═══════════════╝",
                        config.interval_in_seconds
                    );
                    // implement timed upload logic
                }
                "manual" => {
                    println!(
                        "\n╔══════════════╗
                              \n║ Mode: Manual ║ 
                              \n╚══════════════╝"
                    );
                    // just log the changes, don't upload
                }
                _ => {
                    eprintln!(
                        "\n╔═══════════════╗
                               \n║ Mode Unknown! : {}  
                               \n╚═══════════════╝",
                        config.mode
                    );
                }
            }
        }

        Commands::Pull {} => {
            println!("⬇️ Pulling updates...");
            // fetch files from server
        }
        Commands::Push {} => {
            println!("📤 Uploading unsynced changes...");
            // push all changes to server
        }
        Commands::Mode { command } => match command {
            ModeSubcommands::Set { mode } => {
                println!("✅ Mode set to: {}", mode);
                config.mode = mode;
                save_config(&config).expect("Failed to save config");
            }
            ModeSubcommands::Get => {
                println!("🔧 Current mode: {}", config.mode);
            }
        },
    }

    assert_ne!(watched_path.is_empty(), true);

    let path = path::Path::new(&watched_path);
    let log_path = Path::new("./src/log/watch_log.txt");
    let mut log = OpenOptions::new().append(true).open(&log_path);

    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(&path, RecursiveMode::Recursive)?;

    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(Ok(event)) => match log {
                Ok(ref mut file) => {
                    log_event(event, file, path);
                }
                Err(ref e) => {
                    eprintln!("Error opening watch_log!: {}", e);
                }
            },
            Ok(Err(e)) => eprintln!("Watch Error: {:?}", e),
            Err(_) => continue,
        }
    }
}
