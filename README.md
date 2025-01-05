# One Googol

This is a collaborative project where you, the participants, work together to reach a number with 100 zeros, known as one Googol.

## Architecture

- Backend: Located in the [root](/) directory, built with Go.
- Frontend: Located under the [view](view/) directory, built with Svelte.

### Backend

The Go-based backend manages the application's core logic, API endpoints, and real-time communication. Below are the primary routes and their purposes:

- **HTTP API Routes:**
  - `GET /count`: Returns a JSON response with the current count.

  - `POST /count/increment`: Increases the counter by the defined step size and returns the updated value.

  - `POST /count/decrement`: Decreases the counter by the defined step size and returns the updated value.

- WebSocket Route:
  - `/ws`: Enables real-time broadcasting of counter updates to all connected clients, ensuring that every participant sees the latest count instantly.

It also saves the current count on save in the file provided by the `--db` arg.

### Frontend

The Svelte-based frontend provides an interactive user interface for participants to view and manipulate the counter and a guide on #[How to Play](https://one-googol.nwrenger.dev/faq).

## Building

1. Build Frontend:
```sh
cd view
bun run build
```

2. Build Backend:
```sh
cd ../
# For your platform
go build -o build/native
# For arm64 linux
GOOS=linux GOARCH=arm64 go build -o build/arm64
```

3. Package:
```sh
cp -r view/build build/view
touch build/db.json
```

## Usage

```sh
./one-googol localhost:8080 --view ./public --db ./data/counter.db
```
_Starts the server on localhost at port 8080, serves the frontend from ./public, and uses ./data/counter.db for data persistence._
