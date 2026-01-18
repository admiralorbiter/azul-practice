# Azul Best-Move Practice Tool (MVP) — Documentation Pack

This folder contains the v0 draft documents for the **Azul Best-Move Practice Tool**, designed for a **Rust core compiled to WebAssembly (WASM)** with a web UI.

## MVP in one sentence
Generate a valid Azul scenario (2-player), let the user make a move on a polished board UI, then **grade the move** against a computed **best move** (Tier 2 rollout) and provide short, explainable feedback.

## Document map
- **Product**
  - `product/01_vision_and_scope.md`
  - `product/02_mvp_requirements.md`
- **Specs**
  - `specs/03_rules_and_edge_cases.md`
  - `specs/04_state_action_model_and_serialization.md`
  - `specs/08_best_move_evaluation_and_feedback.md`
  - `specs/09_scenario_generation.md`
- **UX**
  - `ux/06_ux_flows_and_screen_map.md`
  - `ux/07_ui_spec.md`
- **Engineering**
  - `engineering/05_architecture_and_wasm_boundary.md`
  - `engineering/10_build_and_tooling_plan.md`
- **Testing**
  - `testing/11_testing_strategy.md`
- **ADRs**
  - `adr/ADR-0001_key_mvp_decisions.md`

## Assumptions baked into this pack
- **2-player Azul** scenarios.
- Scenarios come from **early**, **mid**, and **late** (late drafting) round positions.
- “Best move” is computed using **Tier 2 rollouts**, with a default fast budget and an optional **“Think longer”** mode.

---

If you want to iterate: edit these markdown files directly, then regenerate the zip.
