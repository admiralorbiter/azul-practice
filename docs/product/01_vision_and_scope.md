# Vision & Scope (MVP)

## Vision
Create a polished practice tool that repeatedly asks: **“What is the best move in this Azul position?”**

The tool should feel like deliberate practice:
- fast setup
- clear legal interactions
- trustworthy grading
- quick, meaningful feedback

## Target user
- Primary: **the builder** (a motivated Azul player practicing decision quality)
- Secondary (future): competitive players and learners

## MVP promise
- Generate a valid scenario (2-player).
- The user makes one drafting move.
- The system computes a best move (Tier 2 rollout) and grades the user’s move.
- Feedback is short and explainable.

## MVP goals
- Very smooth practice loop (repeatable, low friction).
- High-quality, frequently used board UI.
- Robust core engine with logging/debug tooling.

## Non-goals (MVP)
- Multiplayer (including WebSockets-based play)
- Accounts, profiles, cloud sync
- Social features, leaderboards
- Perfect optimal-play AI (MVP uses rollout-based “best move” under a defined budget)

## Success criteria
- You can run 20+ scenarios back-to-back without friction.
- Illegal moves are prevented or clearly explained.
- “Best move” grading is consistent and feels reasonable.
- The UI is stable, responsive, and enjoyable.
