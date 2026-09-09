# Delivery phases

Each phase requires implementation, automated tests, failure behavior, documentation, and diagnostics. A mock UI is never acceptance evidence.

| Phase | Outcome | Exit signal |
| --- | --- | --- |
| 0 | Workspace, desktop shell, protocol, SQLite | `cargo test --workspace`; desktop opens and identity persists |
| 1 | Desktop integration | tray, notifications, window controls, shortcut behave on target OSes |
| 2 | Identity | device keys are generated and held in secure local storage |
| 3 | Coordination | username/device registration and ephemeral signaling work without history storage |
| 4 | Transport | two installs connect via authenticated Iroh, direct then relay fallback |
| 5 | Encrypted DMs | local-first message lifecycle is durable and idempotent |
| 6 | Offline delivery | recipient retrieves and acknowledges encrypted mailbox bundles |
| 7 | Friends | discover/request/accept/block are backed by the control plane |
| 8 | Groups | MLS membership epochs and reconnect synchronization work for three clients |
| 9 | Files | chunking, integrity, cancellation, resume, encrypted fallback |
| 10 | Calls | stable 1:1 media only after messaging is proven |
| 11–16 | Presence through release | ephemeral UX, security audit, network matrix, packaging, RC |

## Verified roadmap status

| Phase | Status | Evidence / remaining work |
| --- | --- | --- |
| 0 | Complete | Workspace, migrations, profile setup, protocol validation, and automated unit tests are present. |
| 1 | Not started | No tray, notification, global-shortcut, always-on-top, or packaging implementation exists. |
| 2 | Partial | A local profile exists, but device keys, secure OS key storage, and username registration do not. |
| 3 | Not started | There is no Cloudflare Worker, D1 schema, Durable Object, R2 mailbox, or signaling service. |
| 4 | Not started | There is no Iroh endpoint, peer authentication, relay fallback, reconnect worker, or transport diagnostics. |
| 5 | Not started | The schema reserves ciphertext and pending outbound work, but no reviewed session protocol, message worker, acknowledgement, or retry implementation exists. |
| 6 | Not started | There is no encrypted mailbox implementation or remote object-store integration. |

The test-mode UI is a review aid, not acceptance evidence for any phase: it neither persists sample chats nor opens a network connection.

## Phase 0 delivered

- Cargo workspace with explicit layering.
- Tauri 2 shell and responsive onboarding UI.
- Local SQLite database opened with WAL and foreign keys.
- First profile creation validates normalized usernames.
- Protocol envelope contains ciphertext only and validates its version.
- Unit tests cover persistence, profile validation, and protocol validation.

## Next slice

Phase 1 begins with platform capability checks and a tray/window lifecycle design. Phase 2 must define the reviewed key-storage and identity crates before any “encrypted” UI copy ships.
