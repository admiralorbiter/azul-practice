# Azul Best-Move Practice Tool (MVP) — Documentation Pack

**Project Status:** 6 of 8 sprints complete (75%) - [View Details](SPRINT_STATUS.md)  
**Last Updated:** January 19, 2026  
**MVP Status:** ✅ **PRODUCTION-READY**

This folder contains the v0 draft documents for the **Azul Best-Move Practice Tool**, designed for a **Rust core compiled to WebAssembly (WASM)** with a web UI.

## 📊 Current Status

**✅ Completed:**
- Sprint 00: Foundation & WASM Pipeline
- Sprint 01: Core Engine (Draft Phase) - 4 sub-sprints
- Sprint 02: UI v0 (Board Rendering & Click Interaction)
- Sprint 03: End-of-Round Scoring & Refill - 3 sub-sprints
- Sprint 04: Scenario Generation
- Sprint 05: Best Move Evaluation - 3 sub-sprints
- Sprint 06: Drag/Drop & UI Polish ⬅ **JUST COMPLETED**

**📋 Optional:** Sprint 07 - Advanced Features (3/4-player, enhanced policies)

**🎉 MVP Complete:** The core practice loop is fully functional and polished, ready for user testing!

See [SPRINT_STATUS.md](SPRINT_STATUS.md) for detailed progress tracking.

## MVP in one sentence
Generate a valid Azul scenario (2-player), let the user make a move on a polished board UI, then **grade the move** against a computed **best move** (Tier 2 rollout) and provide short, explainable feedback.

## Document map

- **Status & Planning**
  - `SPRINT_STATUS.md` - ⭐ **Current sprint progress and completion tracking**
  - `sprints/` - Individual sprint plans and completion reports

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

- **Sprints**
  - `sprints/Sprint_00_Foundation_WASM_Pipeline.md` - ✅ Complete
  - `sprints/Sprint_01_Core_Engine_v0_State_Actions_Legality.md` - ✅ Complete (4 sub-sprints)
  - `sprints/Sprint_02_UI_v0_Board_Render_Click_Interactions.md` - ✅ Complete
  - `sprints/Sprint_03_End_of_Round_Scoring_Refill.md` - ✅ Complete (3 sub-sprints)
  - `sprints/Sprint_04_Scenario_Generation_Phases_Filters.md` - ✅ Complete
  - `sprints/Sprint_05_Best_Move_Evaluator_Tier2_Think_Longer.md` - ✅ Complete (3 sub-sprints)
  - `sprints/Sprint_06_Feedback_Explanations_DragDrop_Polish.md` - ✅ Complete
  - `sprints/Sprint_07_Optional_Content_Calibration.md` - 📋 Optional

## Assumptions baked into this pack
- **2-player Azul** scenarios.
- Scenarios come from **early**, **mid**, and **late** (late drafting) round positions.
- “Best move” is computed using **Tier 2 rollouts**, with a default fast budget and an optional **“Think longer”** mode.

---

If you want to iterate: edit these markdown files directly, then regenerate the zip.
