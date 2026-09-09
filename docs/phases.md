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

## Phase 0 delivered

- Cargo workspace with explicit layering.
- Tauri 2 shell and responsive onboarding UI.
- Local SQLite database opened with WAL and foreign keys.
- First profile creation validates normalized usernames.
- Protocol envelope contains ciphertext only and validates its version.
- Unit tests cover persistence, profile validation, and protocol validation.

## Next slice

Phase 1 begins with platform capability checks and a tray/window lifecycle design. Phase 2 must define the reviewed key-storage and identity crates before any “encrypted” UI copy ships.
