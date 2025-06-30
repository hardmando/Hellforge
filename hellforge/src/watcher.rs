use crate::logger::comm_handler::poll_for_update;
use crate::logger::log_event;
use crate::sync_state::SyncState;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

pub fn start_watching(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = channel();

    let log_path = Path::new("./src/log/watch_log.txt");
    let mut log = OpenOptions::new().append(true).open(&log_path);

    let mut sync_state = SyncState::load();

    fs::create_dir_all(path)?;
    let file_path = Path::new(path);
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(file_path, RecursiveMode::Recursive)?;

    thread::spawn(|| {
        loop {
            if let Err(e) = poll_for_update() {
                eprintln!("Error during poll: {}", e);
            }

            thread::sleep(std::time::Duration::from_secs(5));
        }
    });

    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(Ok(event)) => match log {
                Ok(ref mut file) => {
                    log_event(event, file, file_path);
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
