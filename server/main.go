package main

import (
	"flag"
	"log"
	"net/http"
	"os"
	"os/signal"
	"path"
	"path/filepath"
	"strings"
	"syscall"

	"github.com/gorilla/mux"
	"github.com/nwrenger/one-googol/api"
	"github.com/nwrenger/one-googol/db"
	"github.com/nwrenger/one-googol/ws"
)

const module string = "one-googol"

type Args struct {
	host string
	view string
	db   string
	cert string
	key  string
}

func PathExists(path string) bool {
	_, err := os.Stat(path)
	return err == nil
}

func cli_args() Args {
	if len(os.Args) < 2 {
		log.Fatalf("Usage: %s <host(ip:port)> [-view <path>] [-db <path>] [-cert <path>] [-key <path>]\n", module)
	}

	hostArg := os.Args[1]

	flagSet := flag.NewFlagSet(module, flag.ExitOnError)
	view := flagSet.String("view", "view", "Path to the view folder")
	db := flagSet.String("db", "db.txt", "Path to the database file")
	cert := flagSet.String("cert", "/etc/letsencrypt/live/one-googol.nwrenger.dev/fullchain.pem", "Path to the SSL certificate")
	key := flagSet.String("key", "/etc/letsencrypt/live/one-googol.nwrenger.dev/privkey.pem", "Path to the SSL private key")

	flagSet.Parse(os.Args[2:])

	if !PathExists(*view) {
		log.Fatalf("The path for view content '%s' is invalid!", *view)
	}

	dbDir := filepath.Dir(*db)
	if !PathExists(dbDir) {
		log.Fatalf("The directory for the Database '%s' does not exist!", dbDir)
	}

	if !PathExists(*cert) {
		log.Fatalf("The SSL certificate path '%s' does not exist!", *cert)
	}

	if !PathExists(*key) {
		log.Fatalf("The SSL key path '%s' does not exist!", *key)
	}

	return Args{
		host: hostArg,
		view: *view,
		db:   *db,
		cert: *cert,
		key:  *key,
	}
}

type BetterFS struct {
	fs http.FileSystem
}

func (cfs BetterFS) Open(name string) (http.File, error) {
	cleanName := path.Clean("/" + name)

	if strings.HasSuffix(cleanName, "/") {
		return cfs.fs.Open(cleanName)
	}

	if filepath.Ext(cleanName) == "" {
		htmlName := cleanName + ".html"
		if file, err := cfs.fs.Open(htmlName); err == nil {
			return file, nil
		}
	}

	return cfs.fs.Open(cleanName)
}

func main() {
	args := cli_args()
	router := mux.NewRouter()

	// Load count from file
	db.GlobalCount.LoadCountFromFile(args.db)

	// Setup graceful shutdown and count saving
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, os.Interrupt, syscall.SIGTERM)
	go func() {
		<-sigChan
		log.Println("Gracefully shutting down...")

		if err := db.GlobalCount.SaveCountToFile(args.db); err != nil {
			log.Fatalf("Error saving count to file: %v\n", err)
		} else {
			log.Printf("Count saved successfully to %s\n", args.db)
		}
		os.Exit(0)
	}()

	// Setup WebSocket routes
	router.HandleFunc("/ws", ws.WsHandler)
	ws.StartBroadcast()

	// Setup API routes
	router.HandleFunc("/count", api.GetCount).Methods("GET")
	router.HandleFunc("/count/increment", api.IncrementCount).Methods("POST")
	router.HandleFunc("/count/decrement", api.DecrementCount).Methods("POST")

	// Setup File Server
	router.PathPrefix("/").Handler(http.FileServer(BetterFS{
		fs: http.Dir(args.view),
	}))

	// Start server
	log.Printf("Server started on '%s' with frontend at '%s' and Database at '%s'\n", args.host, args.view, args.db)
	if err := http.ListenAndServeTLS(args.host, args.cert, args.key, router); err != nil {
		log.Fatalln("Error starting server:", err)
	}
}
