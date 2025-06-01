# Hellforge 📁  – Real-Time File Synchronization Tool

A modular, cross-platform file synchronization tool built with:

- 🦀 **Rust** – File system watcher and event emitter
- 🐹 **Go** – HTTP server to receive and process events
---

## ✨ Features

- 🔍 Watches directories for changes (create, modify, delete, rename)
- 📝 Logs events with timestamps
- 📡 Sends events to a remote Go server via HTTP
- 🛠️ Easy to extend with authentication, databases, or a web dashboard

---

## 🗂 Project Structure
Hellforge/
<br>├── hellforge/ # Rust CLI that watches file changes
<br>└── server/ # Go server to receive and store events
