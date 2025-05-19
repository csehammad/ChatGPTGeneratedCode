# Machine-to-Machine Identity Provider

This repository hosts a Rust-based Identity Provider (IdP) tailored for services and other non-human clients. It exposes a small HTTP API that issues JSON Web Tokens (JWTs) to registered accounts.

The following documents provide more detail:

- [docs/architecture.md](docs/architecture.md) describes the system components and data flow.
- [docs/roadmap.md](docs/roadmap.md) outlines development milestones.
- [docs/tasks.md](docs/tasks.md) lists the tasks required for each phase.

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
