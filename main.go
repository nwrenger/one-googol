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
}

func PathExists(path string) bool {
	_, err := os.Stat(path)
	if err == nil {
		return true
	} else {
		return false
	}
}

func cli_args() Args {
	if len(os.Args) < 2 {
		log.Fatalf("Usage: %s <host(ip:port)> [-view <path>] [-db <path>]\n", module)
	}

	hostArg := os.Args[1]

	flagSet := flag.NewFlagSet(module, flag.ExitOnError)
	view := flagSet.String("view", "view", "Path to the view folder")
	db := flagSet.String("db", "db.json", "Path to the database file")

	flagSet.Parse(os.Args[2:])

	if !PathExists(*view) {
		log.Fatalf("The path '%s' is invalid!", *view)
	}

	if !PathExists(*db) {
		log.Fatalf("The path '%s' is invalid!", *db)
	}

	return Args{
		host: hostArg,
		view: *view,
		db:   *db,
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

	// count loading
	db.GlobalCount.LoadCountFromFile(args.db)

	// count saving
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

	// ws
	router.HandleFunc("/ws", ws.WsHandler)
	ws.StartBroadcast()

	// api
	router.HandleFunc("/count", api.GetCount).Methods("GET")
	router.HandleFunc("/count/increment", api.IncrementCount).Methods("POST")
	router.HandleFunc("/count/decrement", api.DecrementCount).Methods("POST")

	// file server
	router.PathPrefix("/").Handler(http.FileServer(BetterFS{
		fs: http.Dir(args.view),
	}))

	// start server
	log.Printf("Server started on '%s' with frontend at '%s' and Database at '%s'\n", args.host, args.view, args.db)
	if err := http.ListenAndServe(args.host, router); err != nil {
		log.Fatalln("Error starting server:", err)
	}
}
