# Hearth

Hearth is a local-first, P2P-first desktop messenger. It is deliberately being built in vertical, verifiable phases: a control plane can assist discovery and offline ciphertext delivery, but it never becomes the chat-history database.

## Current status

**Phase 0 — foundation, complete.** The repository contains a runnable Tauri desktop shell, first-launch identity setup backed by local SQLite, a versioned ciphertext-only protocol envelope, and migration-backed persistence. Phases 1–6 are **not complete**: tray/OS integration, device key storage, coordination, P2P transport, authenticated encrypted DMs, and offline mailbox delivery have not yet been implemented.

The desktop UI includes an explicit **Explore test mode** button on onboarding. It opens a disposable sample workspace so the home, messages, people, groups, and settings pages can be reviewed without creating an identity or contacting a server. Sample messages remain in the frontend process only and the UI labels unavailable product capabilities rather than presenting them as functional.

## Quick start

### Prerequisites

- Rust stable and platform build prerequisites for [Tauri 2](https://v2.tauri.app/start/prerequisites/)
- Node.js 22+ and npm

```bash
cd apps/desktop
npm install
npm run tauri dev
```

For non-desktop frontend work, run `npm run dev`. Run all Rust tests from the repository root with `cargo test --workspace`.

## Repository map

- `apps/desktop` — Tauri 2 shell and TypeScript frontend.
- `crates/protocol` — versioned application envelopes; payloads are ciphertext only.
- `crates/database` — SQLite migrations and repositories.
- `crates/app-core` — product state transitions and validation.
- `docs/` — architecture decisions, development phases, and contributor guide.
- `plan.md` — source architecture and complete roadmap.

## Guardrails

- Local SQLite is the authoritative client store.
- The server must never store message or attachment plaintext.
- No UI control may claim a network feature works before its implementation exists.
- Cryptographic protocols are selected from reviewed implementations; this repository does not invent cryptography.

Read [`docs/architecture.md`](docs/architecture.md), [`docs/phases.md`](docs/phases.md), and [`docs/contributing.md`](docs/contributing.md) before extending the project.
