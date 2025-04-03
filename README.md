# One Googol

This is a collaborative project where you, the participants, work together to reach a number with 100 zeros, known as one Googol.

## Architecture

- Backend: Located in the [root](/) directory, built with Rust.
- Frontend: Located under the [view](view/) directory, built with Svelte.

### Backend

The Rust-based backend manages the application's core logic, static file serving and real-time communication. These is established through a Websocket at `/ws`.

It also saves the current count on quit in the file provided by the `--counter` arg.

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
./data/cert/gen.sh
cargo run -- localhost:8080 -c data/counter.json -v view/build --cert data/cert/cert.pem --key data/cert/key.pem
```

## Building

1. Build Frontend:

```sh
cd view
bun run build
```

2. Build Backend:

```sh
cd ..
# For your platform
bun build -r
# For arm64 linux (using cross)
cross build -r --target aarch64-unknown-linux-gnu
```

## Usage

```sh
./one-googol localhost:8080 -v ./public -d ./data/counter.txt --cert ./cert/cert.pem --key ./cert/key.pem
```

_Starts the server on localhost at port 8080, serves the frontend from ./public, uses ./data/counter.txt for data persistence, and the ./cert directory for tls configuration._

## Todos

- [x] Add increase timer vote system (base, exponent, etc. like kind or smth)
  - [x] Reaching Count Values/Levels (10th length) -> Upgrades:
    - [x] Vote on what gets upgraded (5 min time)
    - [x] Exponent +1 (addition modifier)
    - [x] Base/User +1 (multiply modifier)
- [x] Add Clicking yourself +Clicking Multiplier (exponent + base)
