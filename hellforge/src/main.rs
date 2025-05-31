use chrono::Local;
use clap::Parser;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
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

    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(&path, RecursiveMode::Recursive)?;

    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(Ok(event)) => log_event(event),
            Ok(Err(e)) => eprintln!("Watch Error: {:?}", e),
            Err(_) => continue,
        }
    }
}

fn log_event(event: Event) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    println!("\n[{}]", timestamp);

    for path in event.paths {
        let kind = format!("{:?}", event.kind);
        println!("[{}] - {}", kind, path.display());
    }
}
