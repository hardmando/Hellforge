use reqwest::Error;
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

pub fn poll_for_update() -> Result<(), Box<dyn std::error::Error>> {
    if check_update()? {
        receive_event()?;
    }
    Ok(())
}

pub fn receive_event() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    println!("Pulling archive...");
    let res = client.get("http://localhost:8080/pull").send()?;

    if !res.status().is_success() {
        return Err(format!("Server error: {}", res.status()).into());
    }

    let bytes = res.bytes()?;
    let reader = std::io::Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(reader)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = format!("./watched/{}", file.name());

        if let Some(parent) = std::path::Path::new(&out_path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut out_file = std::fs::File::create(&out_path)?;
        std::io::copy(&mut file, &mut out_file)?;
        println!("Extracted {}", out_path);
    }

    println!("Sync complete.");
    Ok(())
}

pub fn check_update() -> Result<bool, reqwest::Error> {
    let client = Client::new();

    println!("Checking for updates...");
    let res = client.get("http://localhost:8080/fetch").send()?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let body = res.text()?.trim().to_string();
            println!("Update available: {}", body);
            Ok(body == "true")
        }
        reqwest::StatusCode::NO_CONTENT => {
            println!("🟢 No updates.");
            Ok(false)
        }
        other => {
            eprintln!("❌ Server error: {}", other);
            Ok(false)
        }
    }
}
