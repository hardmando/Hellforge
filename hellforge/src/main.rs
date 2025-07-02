use clap::{Parser, Subcommand};
mod config;
use config::{load_config, save_config};
pub mod logger;
pub mod sync_state;
mod watcher;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    watcher::start_watching(&watched_path, true)
}
