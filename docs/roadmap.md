# Roadmap for Rust-based IdP

## Overview
This document outlines the development plan for a Rust-based Identity Provider (IdP) designed for low latency and strong security. The primary use case focuses on non-human identities such as agents, services, and automated clients.

## Goals
- **Latency:** Optimize request handling and token issuance to maintain sub-millisecond response times where possible.
- **Security:** Use modern cryptographic primitives and minimize attack surface.
- **Scalability:** Support thousands of concurrent identities and token operations.
- **Automation:** Provide first-class APIs for non-human entities, including service accounts and agent authentication.

## Phased Plan

### Phase 1: Research & Requirements
1. Gather detailed requirements for non-human identities.
2. Evaluate existing Rust crates for OAuth2, JWT, and database layers.
3. Determine preferred async runtime (Tokio or async-std).

### Phase 2: Initial Prototype (MVP)
1. Set up a basic HTTP server using **Axum** or **Actix Web**.
2. Implement minimal token issuance using JWT for service accounts.
3. Integrate a secure storage layer (e.g., PostgreSQL or SQLite for development).
4. Create CLI utilities for managing service accounts.

### Phase 3: Core Security & Persistence
1. Harden JWT generation with rotating signing keys.
2. Implement database migrations and migrations tooling.
3. Introduce role-based access control for agents.
4. Add audit logging for token operations.

### Phase 4: Performance & Observability
1. Add in-memory caching for frequent identity lookups.
2. Benchmark response times and profile hot paths.
3. Expose metrics (Prometheus) and tracing spans.

### Phase 5: Feature Expansion
1. Support OAuth2 client credentials flow for machine-to-machine auth.
2. Provide token revocation and rotation APIs.
3. Build SDK examples for common agent platforms.

### Phase 6: Production Readiness
1. Containerize the service using Docker.
2. Provide CI/CD scripts for building and deploying the IdP.
3. Write extensive integration tests and security fuzz tests.

