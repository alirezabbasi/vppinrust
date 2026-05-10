# vppinrust

`vppinrust` is the workspace for rewriting VPP in Rust.

## Structure

- `crates/`: Rust rewrite crates (core/runtime/control-plane and future modules)
- `scripts/`: workspace automation and CI helpers
- `vppinc/`: original VPP source tree (C codebase)
- `echel-core/`: planning and execution engine artifacts

## Goal

Rewrite the full VPP system in Rust while preserving functional behavior, performance expectations, and operational reliability.

## Why Rust for VPP

- Better memory safety by default, reducing classes of crashes and vulnerabilities.
- Stronger concurrency guarantees for multi-core packet-processing paths.
- Improved long-term maintainability through clearer ownership and type-driven APIs.
- Modern tooling for testing, linting, and documentation with reproducible CI workflows.
- Easier onboarding for new contributors with safer abstractions around low-level code.

## Baseline Checks

Run from project root:

```bash
./scripts/ci/rust-ci.sh
```
