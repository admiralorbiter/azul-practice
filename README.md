# Azul Practice Tool

A web-based practice tool for the Azul board game, featuring AI-powered move evaluation and feedback. Built with a Rust game engine compiled to WebAssembly.

**Status:** ✅ MVP Complete (5 of 8 sprints finished)  
**Latest:** Sprint 05 - Best Move Evaluator with Monte Carlo evaluation and feedback system

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

## Features

### Core Practice Loop ✅
- **Scenario Generation:** Realistic game states (Early/Mid/Late game, Start/Mid/End of round)
- **Interactive Board:** Click-to-select interface with legal move highlighting
- **Move Evaluation:** Monte Carlo rollout-based evaluation with adjustable time budgets
- **Grading System:** EXCELLENT/GOOD/OKAY/MISS ratings based on EV comparison
- **Feedback Generation:** 1-3 explanatory bullets highlighting key differences
- **Complete Flow:** Generate → Play → Evaluate → Feedback → Repeat

### Technical Features
- **Rust Game Engine:** Full Azul rules implementation with tile conservation
- **WebAssembly:** High-performance engine compiled to WASM
- **Monte Carlo Evaluation:** Rollout-based move analysis with policy bots
- **Deterministic Simulation:** Seeded RNG for reproducible results
- **Comprehensive Testing:** 154 tests passing (unit + integration + doc tests)

## Current Capabilities

✅ **Working Now:**
- Load realistic practice scenarios
- Interactive board visualization
- Legal move validation and highlighting
- End-of-round resolution with scoring
- Factory refill and round transitions
- Monte Carlo move evaluation
- AI-powered move grading
- Explanatory feedback generation
- Adjustable thinking time (Fast/Medium/Deep)
- Complete practice loop

📋 **Planned:**
- Drag-and-drop move input (Sprint 06)
- UI animations and polish (Sprint 06)
- Multi-player support 3/4 players (Sprint 07)

## Architecture

The engine is written in Rust and compiled to WebAssembly. The web UI communicates with the WASM module via JSON-serialized messages. Key components:

- **Engine Core:** State model, action application, legality checks
- **Rules System:** Scoring, end-of-round resolution, factory refill
- **AI System:** Policy bots (Random/Greedy), rollout simulation, move evaluation
- **UI Layer:** React components with WASM integration

See `docs/engineering/05_architecture_and_wasm_boundary.md` for details.

## Documentation

- **Sprint Status:** [docs/SPRINT_STATUS.md](docs/SPRINT_STATUS.md)
- **Sprint Plans:** [docs/sprints/](docs/sprints/)
- **Completion Reports:** Sprint 02, 03, 04, 05 completed
- **Engineering Docs:** [docs/engineering/](docs/engineering/)
- **Product Specs:** [docs/specs/](docs/specs/)

## Project Status

**Completed Sprints:**
- ✅ Sprint 00: Foundation & WASM Pipeline
- ✅ Sprint 01: Core Engine (4 sub-sprints)
- ✅ Sprint 02: UI v0 (Board Rendering & Interaction)
- ✅ Sprint 03: End-of-Round Scoring & Refill (3 sub-sprints)
- ✅ Sprint 04: Scenario Generation
- ✅ Sprint 05: Best Move Evaluator (3 sub-sprints)

**Progress:** 62.5% complete (5 of 8 major sprints)

See [docs/SPRINT_STATUS.md](docs/SPRINT_STATUS.md) for detailed progress tracking.
