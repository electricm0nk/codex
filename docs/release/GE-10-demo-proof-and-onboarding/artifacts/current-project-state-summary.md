# Codex Current Project State Summary

_As verified on 2026-06-28 from the live repo and build/test surfaces._

## One-sentence status

Codex is currently a **developer proof harness plus a buildable desktop workbench surface**, not a finished end-user product.

## What is verified and real today

### 1. Rust core proof harness
Verified live:
- root `cargo test` succeeds from `repos/codex`

What this proves:
- the crate builds and passes the current bounded test suite
- GE-03, GE-05, GE-06, and GE-08 proof surfaces are real code, not planning fiction

### 2. GE-03 import foothold
Verified code surface:
- `repos/codex/src/pcgen_import/pcc.rs`

What this proves:
- Codex has a real PCC entry-file parser foothold with provenance-aware parsing behavior

What it does **not** prove:
- full PCGen import coverage

### 3. GE-06 bounded pilot proof
Verified proof surface:
- `cargo test ge06_`
- fixture at `repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`

What this proves:
- a bounded deterministic PF1 Human Fighter pilot can be computed and explained through the current rules-core surface

What it does **not** prove:
- broad Pathfinder support
- broad parity
- finished product UX

### 4. GE-08 bounded homebrew/workbench proof
Verified proof surface:
- `cargo test ge08_`
- package fixture at `repos/codex/tests/fixtures/ge08/guard-stance-package/`

What this proves:
- bounded homebrew package validation, preview, and desktop-workbench integration are real

What it does **not** prove:
- general homebrew ecosystem readiness
- broad rules-studio maturity

### 5. Desktop frontend and Tauri build path
Verified live:
- `npm run typecheck`
- `npm run build`
- `npm run tauri:check`
- `npx tauri build --debug`

What this proves:
- the frontend builds
- the Tauri Rust boundary compiles
- the current desktop binary can be produced on the verified Linux path

What it does **not** prove:
- that the app is a complete end-user product
- that all target platforms are equally verified

## What is currently bounded or unfinished

- the desktop UI is a bounded GE-08 workbench/demo surface, not a general character builder
- onboarding had to be repaired by GE-10 because legibility was lagging behind implementation reality
- GUI launch requires a graphical desktop session; headless shells can build but not launch the GTK app
- cross-platform proof has not been refreshed in this pass outside Linux
- public release, installer, packaging, and broad parity claims remain out of bounds

## Practical interpretation

A truthful way to present Codex today is:

```text
a real proof-oriented Rust/Tauri codebase with working bounded engine, preview, and desktop-shell surfaces, but still well short of finished product scope
```

Any stronger claim needs new evidence.
