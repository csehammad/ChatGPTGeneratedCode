# Architecture Overview

This document describes the core components and data flow of the proposed Rust-based Identity Provider (IdP) optimized for non-human identities.

## Components

1. **HTTP API Layer**
   - Built with Axum (or Actix Web) running on Tokio async runtime for performance.
   - Exposes RESTful endpoints for token issuance, validation, and management of service accounts.
   - Handles JSON/Web token requests and responses.

2. **Authentication & Authorization Module**
   - Implements OAuth2 client credentials flow and custom service account login.
   - Generates signed JWTs for agents with configurable lifetimes.
   - Supports role-based permissions stored in the identity database.

3. **Identity Store**
   - Backed by PostgreSQL for reliability; SQLite may be used during development.
   - Stores service account credentials, API keys, and metadata about agent roles.
   - Provides a query layer for fast lookups with optional caching.

4. **Token Service**
   - Issues, validates, and revokes JWTs.
   - Utilizes rotating signing keys and secure random generation.
   - Caches verified keys to reduce latency during validation.

5. **Observability & Monitoring**
   - Metrics exported via Prometheus and structured logs for auditing.
   - Tracing spans around key operations to identify latency issues.

6. **CLI & SDK**
   - Command-line tools for managing identities and tokens.
   - Reference SDKs (Rust, Python, etc.) for integrating agents.

## Data Flow
1. An agent authenticates using a registered client ID and secret.
2. The HTTP API validates the credentials against the Identity Store.
3. Upon success, the Token Service generates a JWT with roles/permissions embedded.
4. The agent receives the token and uses it to access protected resources.
5. Auditing and metrics are recorded throughout the process.

## Security Considerations
- All secrets are stored encrypted at rest.
- TLS is mandatory for all network communication.
- Rate limiting and anomaly detection guard against abuse.

## Performance Strategies
- Use asynchronous I/O throughout to minimize blocking.
- Employ connection pooling for database interactions.
- Cache identity lookups and signing keys to reduce response times.

