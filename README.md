# ChatGPTGeneratedCode

This repository contains a prototype Rust Identity Provider (IdP) optimized for machine-to-machine authentication. The project demonstrates a minimal HTTP server that issues JWTs to registered service accounts.

## Building

Ensure you have Rust and Cargo installed. Then build the project:

```bash
cargo build --release
```

## Running the Server

Create at least one service account and start the server:

```bash
# create a new account
cargo run --release -- create-account --accounts accounts.json

# run the server
cargo run --release -- server --accounts accounts.json --addr 127.0.0.1:3000
```

The server exposes a `POST /token` endpoint that accepts JSON credentials:

```json
{ "client_id": "<id>", "client_secret": "<secret>" }
```

If the credentials match, the server returns a JWT.

