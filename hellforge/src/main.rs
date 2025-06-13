mod logger;
use clap::{Parser, Subcommand};
use logger::log_event;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::OpenOptions;
use std::path::{self, Path};
use std::sync::mpsc::channel;
use std::time::Duration;
mod config;
use config::{load_config, save_config};

#[derive(Parser)]
#[command(name = "hellforge", about = "File sync service.", version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
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
        #[arg(value_parser = ["instant", "auto", "manual"], default_value = "auto")]
        mode: String,
    },
    Get,
}

fn main() -> notify::Result<()> {
    let cli = Cli::parse();
    let mut watched_path = String::new();
    let mut config = load_config().unwrap_or_default();

    match cli.command.unwrap_or(Commands::Watch {
        path: "./watched".to_string(),
    }) {
        Commands::Watch { path } => {
            println!(
                "
╔════════════════════╗
║ Watching directory : {}
╚════════════════════╝
                ",
                path
            );
            watched_path = path;
            match config.mode.as_str() {
                "instant" => {
                    println!(
                        "
╔═══════════════╗
║ Mode: Instant ║
╚═══════════════╝
                       "
                    );
                }
                "auto" => {
                    println!(
                        "
╔═══════════════╗
║ Mode: Auto    ║
║ Interval: {}  ║
╚═══════════════╝
                        ",
                        config.interval_in_seconds
                    );
                    // implement timed upload logic
                }
                "manual" => {
                    println!(
                        "
╔══════════════╗
║ Mode: Manual ║
╚══════════════╝
                        "
                    );
                    // just log the changes, don't upload
                }
                _ => {
                    eprintln!(
                        "
╔═══════════════╗
║ Mode Unknown! : {}
╚═══════════════╝
                        ",
                        config.mode
                    );
                }
            }
        }

        Commands::Pull {} => {
            println!(
                "
╔═════════════════╗
║ Pulling Updates ║
╚═════════════════╝
            "
            );
            // fetch files from server
        }
        Commands::Push {} => {
            println!(
                "
╔════════════════════╗
║ Uploading files... ║
╚════════════════════╝
            "
            );
            // push all changes to server
        }
        Commands::Mode { command } => match command {
            ModeSubcommands::Set { mode } => {
                println!(
                    "
╔══════════╗
║ Mode Set : {}
╚══════════╝
                ",
                    mode
                );
                config.mode = mode;
                save_config(&config).expect("Failed to save config");
                return Ok(());
            }
            ModeSubcommands::Get => {
                println!(
                    "
╔══════════════╗
║ Current Mode : {}
╚══════════════╝
                ",
                    config.mode
                );
                return Ok(());
            }
        },
    }

    //assert_ne!(watched_path.is_empty(), true);

    let path = path::Path::new(&watched_path);
    let log_path = Path::new("./src/log/watch_log.txt");
    let mut log = OpenOptions::new().append(true).open(&log_path);

    println!(
        "
 ╔══════════════════════════════════════════════════════════════════════════════════════════════════════╗
 ║ █████    █████             ███████ ████████    ███████                                               ║
 ║  ████    ████               ██████   █████    ████████                                               ║
 ║  ████    ████               ████     ████    ████    █                                               ║
 ║  ████    ████               ████     ████    ████                                                    ║
 ║  ████    ████  ██████████   ████     ████    ████████  ███████      █████████  █████████ ██████████  ║
 ║  ████████████   █████████   ████     ████    ████████  ████████████  ████████ ██████████  █████████  ║
 ║  ████████████   ████  ███   ████     ████    ████      ████ ███████  ████████ ███   ███   ████  ████ ║
 ║  ████    ████   ████  ███   ████     ████    ████      ████    ████  ████ ███ ███   ███   ████  ████ ║
 ║  ████    ████   ██████████  ████     ████    ████      ████    ████  ████ ███ █████████   ██████████ ║
 ║  ████    ████   ████        ████     ████    ████      ████    ████  ████   █ █████████   ████       ║
 ║  ████    ████   ████    ██  ████     ████    ████      ████    ████  ████           ███   ████    ██ ║
 ║ █████    █████  █████████  ██████   ██████  ██████     ████████████ ██████          ███   ██████████ ║
 ║  ████    ██      █████      ███████  ██████  █████         ███████  ██████    ███   ███    ██████    ║
 ║    █                                           █               █              █████████       █      ║
 ║                                                                              █████████               ║
 ╚══════════════════════════════════════════════════════════════════════════════════════════════════════╝
    
    Welcome to Hellforge!
"
    );

    println!(
        "
╔════════════════════╗
║ Watching directory : {}
╠════════════════════╣
║ Mode:                {}
╚════════════════════╝

",
        watched_path, config.mode
    );

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
