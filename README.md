# vppinrust

`vppinrust` is the workspace for rewriting VPP in Rust.

## Structure

- `crates/`: Rust rewrite crates (core/runtime/control-plane and future modules)
- `scripts/`: workspace automation and CI helpers
- `vppinc/`: original VPP source tree (C codebase)
- `echel-core/`: planning and execution engine artifacts

## Goal

Rewrite the full VPP system in Rust while preserving functional behavior, performance expectations, and operational reliability.

## Baseline Checks

Run from project root:

```bash
./scripts/ci/rust-ci.sh
```
