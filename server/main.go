package main

import (
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
)

type SyncEvent struct {
	Timestamp string `json:"timestamp"`
	EventKind string `json:"event_kind"`
	Path      string `json:"path"`
}

func handleEvent(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(10 << 20)

	eventJson := r.FormValue("event")
	var event SyncEvent

	if err := json.Unmarshal([]byte(eventJson), &event); err != nil {
		http.Error(w, "Invalid event JSON", http.StatusBadRequest)
		return
	}

	file, handler, err := r.FormFile("file")
	if err != nil {
		http.Error(w, "File Not Found", http.StatusBadRequest)
		return
	}
	defer file.Close()
	dstPath := "uploads/" + event.Path
	var str string = handler.Filename
	dst, err := os.Create(dstPath)
	if err != nil {
		http.Error(w, "Could not save file", http.StatusInternalServerError)
		return
	}
	defer dst.Close()

	io.Copy(dst, file)

	log.Printf("Received Event for %s and saved at %s, file: %s", event.Path, dst.Name(), str)
	w.WriteHeader(http.StatusOK)
}

func main() {
	http.HandleFunc("/event", handleEvent)
	fmt.Println("Listening on :8080...")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
