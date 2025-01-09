# One Googol

This is a collaborative project where you, the participants, work together to reach a number with 100 zeros, known as one Googol.

## Architecture

- Backend: Located in the [server](server/) directory, built with Go.
- Frontend: Located under the [view](view/) directory, built with Svelte.

### Backend

The Go-based backend manages the application's core logic, API endpoints, and real-time communication. Below are the primary routes and their purposes:

- **HTTP API Routes:**
  - `GET /count`: Returns a text response with the current count.

  - `POST /count/increment`: Increases the counter by the defined step size and returns the updated value.

  - `POST /count/decrement`: Decreases the counter by the defined step size and returns the updated value.

- WebSocket Route:
  - `/ws`: Enables real-time broadcasting of counter updates to all connected clients, ensuring that every participant sees the latest count instantly.

It also saves the current count on save in the file provided by the `--db` arg.

### Frontend

The Svelte-based frontend provides an interactive user interface for participants to view and manipulate the counter and a guide on [How to Play](https://one-googol.nwrenger.dev/faq).

## Development

1. Build Frontend:
```sh
cd view
bun run build
```

2. Run Server:
```sh
cd server
# build tls cert and key
./data/cert/gen.sh
go run . localhost:8080 --db data/db.txt --view ../view/build --cert data/cert/cert.pem --key data/cert/key.pem
```

## Building

1. Build Frontend:
```sh
cd view
bun run build
```

2. Build Backend:
```sh
cd server
# For your platform
go build -o ../build/native
# For arm64 linux
GOOS=linux GOARCH=arm64 go build -o ../build/arm64
```

3. Package:
```sh
cp -r view/build build/view
```

## Usage

```sh
./one-googol localhost:8080 --view ./public --db ./data/counter.txt --cert ./cert/cert.pem --key ./cert/key.pem
```
_Starts the server on localhost at port 8080, serves the frontend from ./public, uses ./data/counter.txt for data persistence, and the ./cert directory for tls configuration._

## Todo

- Add increase timer vote system (base, exponent, etc. like kind)
