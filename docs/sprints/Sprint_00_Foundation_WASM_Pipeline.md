# Sprint 0 — Project Skeleton + WASM Pipeline

**Goal:** Build/run the app end-to-end with a “hello engine” WASM call, and set up the foundation for a robust core (logging, CI, deterministic seeds).

## Outcomes
- A working Rust→WASM build that the web UI can import and call
- A clean repo layout with docs, engine, UI, and shared schema decisions
- Dev-mode logging and basic diagnostics

## Scope
### 1) Repository structure
- `rust/` (Rust workspace for engine)
- `web/` (React/Vite UI)
- `docs/` (specs + ADRs)
- `scripts/` (build helpers)

### 2) Toolchain
- Rust stable + `wasm-bindgen` (or chosen binding approach)
- WASM build scripts (dev + release)
- Vite integration (WASM loaded properly, no manual hacks)

### 3) Observability (dev mode)
- Rust logs forwarded to browser console (dev-only)
- Engine/version endpoint exposed to UI
- Seed plumbing: allow passing a seed into engine functions for reproducibility

### 4) CI (minimum viable)
- `cargo fmt`, `cargo clippy`, `cargo test`
- `pnpm lint` (or npm/yarn equivalent), `pnpm build`
- A single end-to-end “smoke test” (even if manual for now)

## Deliverables
- `get_version()` export callable from UI
- CI pipeline running on each commit
- Documented build commands in `web/README.md` or root README

## Acceptance Criteria
- A developer can clone repo and run:
  - `pnpm install && pnpm dev` (or equivalent)
  - Engine builds and UI loads without errors
- UI successfully calls a WASM export and displays output
- Dev mode shows engine version and logs in console

## Demo (end-of-sprint)
- Open app → see engine version displayed
- Click “Ping Engine” → UI calls WASM → shows response + logs appear

## Risks / Notes
- WASM bundling issues can burn time; keep the first export minimal
- Decide early on error passing: structured JSON errors vs thrown exceptions

## Sprint Backlog (Suggested Tasks)
- [ ] Create repo layout and workspace configs
- [ ] Add `wasm-bindgen` setup + build scripts
- [ ] Add minimal WASM export(s): version + ping
- [ ] Add UI page that loads WASM and calls exports
- [ ] Add CI checks for Rust and web
- [ ] Add dev logging bridge
