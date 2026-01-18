# Azul Practice Tool

A web-based practice tool for the Azul board game, featuring a Rust game engine compiled to WebAssembly.

## Project Structure

- `rust/` - Rust workspace containing the game engine
- `web/` - React/Vite web UI
- `docs/` - Project documentation and specifications
- `scripts/` - Build and development scripts

## Prerequisites

- Rust stable (install from [rustup.rs](https://rustup.rs/))
- Node.js 18+ and npm or pnpm
- `wasm-pack` (install via `cargo install wasm-pack`)

## Quick Start

1. **Install dependencies:**
   ```bash
   cd web
   npm install
   # or: pnpm install
   ```

2. **Build WASM:**
   ```bash
   # From project root
   scripts/build-wasm.sh  # or scripts/build-wasm.ps1 on Windows
   ```

3. **Start development server:**
   ```bash
   cd web
   npm run dev
   # or: pnpm dev
   ```

## Development Workflow

All commands below work with both `npm` and `pnpm` (replace `npm` with `pnpm` if you prefer):

- `npm run dev` - Start Vite dev server (from `web/` directory)
- `npm run wasm:build` - Rebuild WASM module (from `web/` directory)
- `npm run build` - Build UI for production
- `npm run lint` - Run linters

## Architecture

The engine is written in Rust and compiled to WebAssembly. The web UI communicates with the WASM module via JSON-serialized messages. See `docs/engineering/05_architecture_and_wasm_boundary.md` for details.
