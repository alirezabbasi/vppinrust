# vppinrust Ruleset

This file is the top-level project ruleset for `/home/alireza/Projects/vppinrust`.

## 0) Purpose
- Project goal: rewrite the full VPP codebase in Rust.
- `vppinc/` is the original VPP C source reference.
- `echel-core/` is the planning/execution engine workspace.

## 1) Session Start Protocol (Mandatory)
At the beginning of every session, do the following in order:
1. Read this file: `ruleset.md`.
2. Read `README.md` at repository root.
3. Read `echel-core/ruleset.md` and `echel-core/docs/ruleset.md`.
4. Run `make session-bootstrap` in `echel-core/`.
5. Run `make wrw` in `echel-core/`.
6. Confirm current focus task from `echel-core/docs/development/02-execution/KANBAN.md`.

## 2) Source of Truth
- Execution tasks/status are tracked in `echel-core/docs/development/02-execution/KANBAN.md`.
- Task artifacts are in `echel-core/wiki/tasks/`.
- Rewrite governance/parity contract is:
  - `echel-core/docs/development/00-governance/VPP_RUST_PARITY_CONTRACT.md`

## 3) Working Rules
- Do not treat chat-only reasoning as final; persist durable decisions in project artifacts.
- Any major rewrite decision requires a documented record in `echel-core/wiki/`.
- Before closing substantial work, run required quality gates in `echel-core`.

## 4) Completion Gate (for substantial changes)
- Run: `make wiki-health` in `echel-core/`.
- If it fails, either fix failures or document why they are deferred.

## 5) If Unsure
- Default to safety and traceability:
  - avoid destructive actions,
  - keep migration steps reversible,
  - record assumptions and risks in `echel-core` memory docs.
