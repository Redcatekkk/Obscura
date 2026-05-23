# obscura.deck

Native Tauri 2 desktop port of the NULLBYTE control-deck mockup. The app is being built as a local simulation deck: the interface will look and behave like the reference UI, while every Discord-like event comes from a local Simulation Engine ("simverse").

## Status

Milestone M0 is closed. The workspace scaffold, desktop shell, CI skeleton, smoke tests, and documentation foundation exist; the M1 visual port, simulation engine, module behavior, and release packaging work are still future milestone work.

## Stack

- Desktop shell: Tauri 2 and Rust 1.78+
- Frontend: React 19, Vite 5, TypeScript 5.5 strict
- Package management: pnpm workspace plus Cargo workspace
- Testing: Cargo tests, Vitest, scaffold E2E smoke checks
- CI: GitHub Actions matrix for `macos-14`, `ubuntu-22.04`, and `windows-latest`

## Install

Install Rust, Node.js 22, and pnpm 9.15.4. Then install frontend dependencies:

```sh
pnpm install
```

On Linux, Tauri also needs WebKit and app indicator system packages. The CI workflow documents the Ubuntu 22.04 package list in `.github/workflows/ci.yml`.

## Development

Run the Vite frontend shell:

```sh
pnpm dev
```

Run the Tauri desktop shell:

```sh
pnpm tauri dev
```

The current M0 app opens an intentionally empty desktop window. The full NULLBYTE UI is planned for M1.

## Build

Build the frontend:

```sh
pnpm build
```

Run a Tauri build smoke check without bundling installers:

```sh
pnpm ci:tauri
```

Full signed installers are a later release milestone.

## Test

Run frontend type checks and tests:

```sh
pnpm typecheck
pnpm test
pnpm test:e2e
```

Run Rust workspace checks:

```sh
cargo fmt --check
cargo test --workspace --all-features
```

Run the local CI script groups:

```sh
pnpm ci:frontend
pnpm ci:rust
pnpm ci:tauri
```

## CI

The GitHub Actions workflow runs on macOS, Linux, and Windows. It installs pinned pnpm and Rust versions, restores pnpm cache through `actions/setup-node`, runs Rust formatting and tests, runs frontend typecheck/test/E2E/build checks, and performs a Tauri no-bundle smoke build.

## Security Model

obscura.deck is simulation-only by default. It must not automate real Discord user accounts, scrape Discord, or accept Discord user tokens. Discord self-bots and user-token automation are outside the project scope.

All planned modules operate against the local simverse event stream. A future optional `bot_adapter` feature may support official Discord bot-token integration only. That feature must stay disabled by default, must reject values that look like user tokens, and must never provide user-account automation.

Do not add user tokens to source files, fixtures, tests, environment examples, docs, screenshots, or support instructions. Use local simulated data for development and testing.

## Repository Layout

```text
apps/desktop/          Tauri 2 + React desktop app
apps/desktop/src/      React frontend shell
apps/desktop/src-tauri Rust desktop crate and Tauri config
docs/orchestration/    Three-agent orchestration records
```
