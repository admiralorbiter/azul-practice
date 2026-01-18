# Build & Tooling Plan (Rust → WASM)

## Toolchain
- Rust stable
- wasm-bindgen
- wasm-pack (or equivalent)
- Vite + React (web UI)

## Local development workflow
- `pnpm dev`: run UI
- `pnpm wasm:dev`: rebuild wasm (watch)
- Dev mode features:
  - Rust logs forwarded to browser console
  - Seed display/control
  - Copy scenario JSON

## Artifacts
- WASM bundle for web
- Optional: scenario pack outputs
