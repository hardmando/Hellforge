mod logger;
use clap::Parser;
use logger::log_event;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::OpenOptions;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "./watched")]
    path: String,
}

fn main() -> notify::Result<()> {
    let args = Args::parse();

    std::fs::create_dir_all(&args.path).expect("Failed to create directory!");
    println!("Watching folder {}", &args.path);

    let path = Path::new(&args.path);
    let log_path = Path::new("./src/log/watch_log.txt");
    let mut log = OpenOptions::new().append(true).open(&log_path);

    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(&path, RecursiveMode::Recursive)?;

    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(Ok(event)) => match log {
                Ok(ref mut file) => {
                    log_event(event, file);
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
