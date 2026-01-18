# ADR-0001 — Key MVP Decisions

**Status:** Accepted  
**Date:** 2026-01-18

## Context
We are building an Azul practice tool that can scale over time, but the MVP must remain laser-focused on a single loop: present a scenario, let the user choose a move, then grade it against a computed best move.

## Decisions
1. **Architecture:** Implement the authoritative game engine in **Rust**, compiled to **WebAssembly**, consumed by a web UI.
2. **MVP Feature Focus:** Only the best-move practice loop (no multiplayer, no accounts, no full product suite).
3. **Ruleset:** Assume **2-player Azul**.
4. **Best-move algorithm:** Use **Tier 2 rollout-based evaluation** with a configurable time budget.
5. **Scenario timing:** Generate scenarios from **early**, **mid**, and **late** stages of drafting.
6. **UX:** Polished board UI with drag/drop; aligned loosely with Board Game Arena aesthetics.
7. **Observability:** Logging + debugging hooks are in-scope for MVP (robust core), even if features remain minimal.
8. **Performance UX:** Provide a **“Think longer”** control (default fast; optional longer budgets).

## Consequences
- We must define stable, versioned **State/Action** schemas and WASM boundary APIs.
- Rollouts require deterministic RNG and profiling to keep “fast mode” responsive.
- Scenario generation must be legality-preserving and quality-filtered.
