use reqwest::blocking::Client;
use reqwest::blocking::multipart;
use serde::Serialize;
use std::fs::File;
use std::path::Path;

#[derive(Serialize)]
pub struct SyncEvent {
    pub timestamp: String,
    pub event_kind: String,
    pub path: String,
    pub watched_path: String,
}

pub fn send_event(event: &SyncEvent) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let path = Path::new(&event.path);
    let file = File::open(path)?;

    let full = Path::new(&event.path).canonicalize().ok().unwrap();
    let base = Path::new(&event.watched_path).canonicalize().ok().unwrap();

    let rel_path = full
        .strip_prefix(base)
        .map_err(|e| format!("Could not strip prefix: {}", e))?;

    //    let meta_path = serde_json::to_string(&format!("/{}", rel_path.display()))?;
    let meta_path = rel_path
        .to_path_buf()
        .into_os_string()
        .into_string()
        .unwrap();
    println!("{}", meta_path);

    let form = multipart::Form::new()
        .text("event", serde_json::to_string(event)?)
        .text("metaPath", meta_path)
        .file("file", &event.path)?;

    let res = client
        .post("http://localhost:8080/event")
        .multipart(form)
        .send()?;

    if res.status().is_success() {
        println!("Event successfully sent!");
    } else {
        eprintln!(
            "Failed to send event! Server responded with: {} ",
            res.status()
        );
    }
    Ok(())
}
