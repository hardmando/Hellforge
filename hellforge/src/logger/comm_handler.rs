use reqwest::blocking::Client;
use serde::Serialize;

#[derive(Serialize)]
pub struct SyncEvent {
    pub timestamp: String,
    pub event_kind: String,
    pub path: String,
}

pub fn send_event(event: &SyncEvent) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client
        .post("http://localhost:8080/event")
        .json(&event)
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
