mod comm_handler;
use chrono::Local;
use comm_handler::{SyncEvent, send_event};
use notify::Event;
use std::fs::File;
use std::io::Write;

pub fn log_event(event: Event, log: &mut File) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let mut sync_event: SyncEvent;

    println!("\n[{}]", timestamp);

    for path in event.paths {
        let kind = format!("{:?}", event.kind);
        writeln!(log, "[{}] [{}] - {}", timestamp, kind, path.display());
        println!("[{}] - {}", kind, path.display());

        sync_event = SyncEvent {
            timestamp: timestamp.to_string(),
            event_kind: kind,
            path: path.into_os_string().into_string().unwrap(),
        };

        match send_event(&sync_event) {
            Ok(()) => println!("Succsesfully sent!"),
            Err(e) => eprintln!("Error! {}", e),
        };
    }
}
