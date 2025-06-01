package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
)

type SyncEvent struct {
	Timestamp string `json:"timestamp"`
	EventKind string `json:"event_kind"`
	Path      string `json:"path"`
}

func handleEvent(w http.ResponseWriter, r *http.Request) {
	var event SyncEvent
	err := json.NewDecoder(r.Body).Decode(&event)
	if err != nil {
		http.Error(w, "Invalid JSON", http.StatusBadRequest)
		return

	}
	log.Printf("Received Event: %+v\n", event)
	w.WriteHeader(http.StatusOK)
}

func main() {
	http.HandleFunc("/event", handleEvent)
	fmt.Println("Listening on :8080...")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
