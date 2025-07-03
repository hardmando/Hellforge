package main

import (
	"archive/zip"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

type SyncEvent struct {
	Timestamp string `json:"timestamp"`
	EventKind string `json:"event_kind"`
	Path      string `json:"path"`
	MetaPath  string `json:"metaPath"`
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
	dstPath := "uploads/" + handler.Filename
	metaPath := "uploads/" + handler.Filename + ".meta"
	dst, err := os.Create(dstPath)
	if err != nil {
		http.Error(w, "Could not save file", http.StatusInternalServerError)
		return
	}
	defer dst.Close()

	io.Copy(dst, file)

	meta_path := "/" + r.FormValue("metaPath")
	os.WriteFile(metaPath, []byte(meta_path), 0666)

	fmt.Printf("Received Event for %s and saved at %s", event.Path, dst.Name())
	w.WriteHeader(http.StatusOK)
}

func handlePull(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/zip")
	w.Header().Set("Content-Disposition", "attachment; filename=sync.zip")

	zipWriter := zip.NewWriter(w)
	defer zipWriter.Close()

	filepath.Walk("./uploads", func(path string, info os.FileInfo, err error) error {
		if err != nil || info.IsDir() || !strings.HasSuffix(path, ".meta") {
			return nil
		}

		metaContent, err := os.ReadFile(path)
		if err != nil {
			return nil
		}
		relativePath := strings.TrimSpace(string(metaContent))

		dataFilePath := strings.TrimSuffix(path, ".meta")
		dataFile, err := os.Open(dataFilePath)
		if err != nil {
			return nil
		}
		defer dataFile.Close()

		zipEntry, err := zipWriter.Create(relativePath)
		if err != nil {
			return nil
		}

		io.Copy(zipEntry, dataFile)
		os.Remove(dataFilePath)
		os.Remove(path)
		return err
	})
}

func handleFetch(w http.ResponseWriter, r *http.Request) {
	files, err := os.ReadDir("./uploads")
	if err != nil {
		http.Error(w, "Internal Server Error", http.StatusInternalServerError)
		return
	}

	for _, file := range files {
		if strings.HasSuffix(file.Name(), ".meta") {
			w.WriteHeader(http.StatusOK)
			w.Write([]byte("true"))
			return
		}
	}

	w.WriteHeader(http.StatusNoContent)
}

func main() {
	http.HandleFunc("/event", handleEvent)
	http.HandleFunc("/fetch", handleFetch)
	http.HandleFunc("/pull", handlePull)
	fmt.Println("Listening on http://localhost:8443...")
	log.Fatal(http.ListenAndServeTLS(":8443", "cert.pem", "key.pem", nil))
}
