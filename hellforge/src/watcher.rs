use crate::logger::comm_handler::{SyncEvent, poll_for_update, send_event};
use crate::logger::log_event;
use crate::sync_state::{self, SyncState};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

pub fn start_watching(path: &str, log_enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
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
            Ok(Ok(event)) => {
                handle_event(event, &mut sync_state, path, log_enabled);
            }
            Ok(Err(e)) => eprintln!("Watch Error: {:?}", e),
            Err(_) => continue,
        }
    }
}

fn handle_event(
    event: Event,
    sync_state: &mut SyncState,
    watch_path: &str,
    log_enabled: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    for path in event.paths {
        let path_str = path.to_string_lossy().to_string();

        // Skip `.meta` files
        if path_str.ends_with(".meta") {
            continue;
        }

        // Skip unchanged files
        if !sync_state.should_upload(&path_str)? {
            println!("🟢 Skipped unchanged: {}", path_str);
            continue;
        }

        // Create sync event and send
        let sync_event = SyncEvent::new(&path_str, watch_path.to_string());
        send_event(&sync_event)?;
        sync_state.save()?;
    }

    Ok(())
}
