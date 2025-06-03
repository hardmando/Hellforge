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
}

pub fn send_event(event: &SyncEvent) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let path = Path::new(&event.path);
    let file = File::open(path)?;

    let form = multipart::Form::new()
        .text("event", serde_json::to_string(event)?)
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
