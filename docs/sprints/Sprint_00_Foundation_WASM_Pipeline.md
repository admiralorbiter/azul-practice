# Sprint 0 — Project Skeleton + WASM Pipeline

**Goal:** Build/run the app end-to-end with a "hello engine" WASM call, and set up the foundation for a robust core (logging, deterministic seeds).

## Outcomes
- ✅ A working Rust→WASM build that the web UI can import and call
- ✅ A clean repo layout with docs, engine, UI, and shared schema decisions
- ✅ Dev-mode logging and basic diagnostics

## Scope
### 1) Repository structure ✅
- ✅ `rust/` (Rust workspace for engine)
- ✅ `web/` (React/Vite UI)
- ✅ `docs/` (specs + ADRs)
- ✅ `scripts/` (build helpers)

### 2) Toolchain ✅
- ✅ Rust stable + `wasm-bindgen`
- ✅ WASM build scripts (dev + release) - bash and PowerShell versions
- ✅ Vite integration (WASM loaded properly, no manual hacks)

### 3) Observability (dev mode) ✅
- ✅ Rust logs forwarded to browser console (dev-only) - using `wasm-logger`
- ✅ Engine/version endpoint exposed to UI - `get_version()` function
- ⏭️ Seed plumbing: allow passing a seed into engine functions for reproducibility (deferred to later sprints when needed)


## Deliverables
- ✅ `get_version()` export callable from UI
- ✅ `ping()` export for testing WASM communication
- ✅ Documented build commands in root `README.md`

## Acceptance Criteria
- ✅ A developer can clone repo and run:
  - ✅ `npm install && npm run dev` (or `pnpm install && pnpm dev`)
  - ✅ Engine builds and UI loads without errors (after running WASM build first)
- ✅ UI successfully calls a WASM export and displays output
- ✅ Dev mode shows engine version and logs in console

**Implementation notes:**
- Build process: `wasm-pack` must be run first (via `scripts/build-wasm.ps1` on Windows or `scripts/build-wasm.sh` on Unix)
- WASM output goes to `web/src/wasm/pkg/` directory
- Both npm and pnpm are supported for package management

## Demo (end-of-sprint) ✅
- ✅ Open app → see engine version displayed
- ✅ Click "Ping Engine" → UI calls WASM → shows response + logs appear

**Demo status:** ✅ Complete - All demo requirements met. EngineStatus component displays version info and ping functionality.

## Risks / Notes
- ✅ WASM bundling: Resolved - using `wasm-pack` with `--target web` works cleanly
- ✅ Error passing: Using JSON strings for now (structured errors deferred to Sprint 01+)
- ⚠️ Build order: Users must build WASM before running dev server (documented in README)
- ✅ Cross-platform: Both bash and PowerShell build scripts provided

## Sprint Backlog (Suggested Tasks)
- [x] Create repo layout and workspace configs
- [x] Add `wasm-bindgen` setup + build scripts
- [x] Add minimal WASM export(s): version + ping
- [x] Add UI page that loads WASM and calls exports
- [x] Add dev logging bridge

## Implementation Summary

**Files created:**
- Repository structure: `rust/`, `web/`, `scripts/`
- Rust: `rust/Cargo.toml`, `rust/engine/Cargo.toml`, `rust/engine/src/lib.rs`, `rust/engine/src/version.rs`
- Web UI: Vite + React + TypeScript setup with `EngineStatus` component
- Build scripts: `scripts/build-wasm.sh`, `scripts/build-wasm.ps1`
- Configuration: Root `README.md`, `.gitignore`

**Key technical decisions:**
- Using `wasm-pack` with `--target web` for WASM compilation
- JSON string responses for WASM API (v0 contract)
- `wasm-logger` for dev-mode console logging
- Both npm and pnpm supported

**Known limitations:**
- Seed plumbing deferred (not needed until scenario generation in Sprint 04)
