use crate::config;
use crate::logger::comm_handler::{SyncEvent, poll_for_update, send_event};
use crate::sync_state::SyncState;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::fs::OpenOptions;
use std::io::{Write, stdin, stdout};
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

pub fn start_watching(
    config: &config::Config,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let server_ip = config.server_ip.clone();
    match config.mode.as_str() {
        "instant" => {
            println!("Running in INSTANT mode");

            thread::spawn(move || {
                loop {
                    if let Err(e) = poll_for_update(&server_ip) {
                        eprintln!("Error during poll: {}", e);
                    }

                    thread::sleep(std::time::Duration::from_secs(5));
                }
            });
        }
        "auto" => {
            println!(
                "Running in AUTO mode (every {} seconds)",
                config.interval_in_secs
            );
            let interval_in_secs = config.interval_in_secs;
            thread::spawn(move || {
                loop {
                    if let Err(e) = poll_for_update(&server_ip) {
                        eprintln!("Error during poll: {}", e);
                    }

                    thread::sleep(std::time::Duration::from_secs(interval_in_secs));
                }
            });
        }
        "manual" => {
            println!("Running in MANUAL mode");

            loop {
                println!(">");
                let mut input = String::new();
                let _ = stdout().flush();
                stdin().read_line(&mut input).expect("Unknown command");
                if input.trim_end() == "fetch" {
                    if let Err(e) = poll_for_update(&server_ip) {
                        eprintln!("Error during poll: {}", e);
                    }
                }
            }
        }
        _ => {
            eprintln!("Unknown mode: {}", config.mode)
        }
    }

    let (tx, rx) = channel();

    let log_path = Path::new("./src/log/watch_log.txt");
    //    let mut log = OpenOptions::new().append(true).open(&log_path);

    let mut sync_state = SyncState::load();

    fs::create_dir_all(path)?;
    let file_path = Path::new(path);
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(file_path, RecursiveMode::Recursive)?;

    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(Ok(event)) => {
                handle_event(event, &mut sync_state, path, true, config);
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
    config: &config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    for path in event.paths {
        let path_str = path.to_string_lossy().to_string();
        let server_ip = &config.server_ip;
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
        send_event(&sync_event, server_ip)?;
        sync_state.save()?;
    }

    Ok(())
}
