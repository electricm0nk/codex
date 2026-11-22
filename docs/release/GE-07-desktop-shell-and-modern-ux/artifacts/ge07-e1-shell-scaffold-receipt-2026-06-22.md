---
title: GE07-E1 Shell Scaffold Receipt
artifact_type: spike-receipt
stc_id: STC-CODEX-GE-07
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE07-E1 — Shell scaffold and runtime boundary spike
workflow_route: planning
readiness: planning-ready
status: active
review_date: 2026-06-22
owner: Todd Hintzmann
scope: program
code_authority: false
related_artifacts:
  - ./ge07-e1-runtime-boundary-adr-input-2026-06-22.md
  - ../../GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md
  - ../../../research/codex-reference-architecture-2026-06-17.md
---

# GE07-E1 Shell Scaffold Receipt

## Verdict
The smallest honest GE-07 move is now grounded.

It is not a product shell. It is a non-production scaffold posture over the existing headless Codex core.

This receipt proves four things:
1. the GE-06 viability gate decision now exists, so the old planning blocker is stale
2. the live Codex repo still has no desktop-shell scaffold checked in
3. the current runtime does have the minimum general toolchain needed to evaluate a future shell spike (`cargo`, `rustc`, `node`, `npm`)
4. the first truthful GE-07 implementation target must therefore be scaffold-only plus boundary proof, not product-truth claims

## Grounded observations from the live repo
### Repo identity
Commands run in `/home/ubuntu/workspace/repos/codex` on 2026-06-22:
- `git rev-parse --abbrev-ref HEAD`
- `git rev-parse HEAD`
- `git rev-parse origin/develop`
- `git status --short --branch`

Observed results:
- checked-out branch: `ge06-e3-f2-classifier-impl`
- checked-out commit: `cc45f2c84b0c6bd3b3a7886f9f3068ece8b58e48`
- `origin/develop`: `b2f21544b8def8759d98b30d8f2a6cfb8ad94df1`
- repo state remained on a GE-06 branch; this is not GE-07 execution authority by itself

### Repo layout
Commands run:
- `find . -maxdepth 2 \( -type f -o -type d \) | sed 's#^./##' | sort | head -n 120`
- repository file search for `package.json`, `tauri.conf.json`, `vite.config.*`, `src-tauri`, and other desktop-shell markers

Observed layout truth:
- present at repo root: `Cargo.toml`, `src/`, `tests/`, `.github/`, repo conduct surfaces
- absent from the repo: `apps/desktop/`, `src-tauri/`, `package.json`, `tauri.conf.json`, and any existing frontend/UI subtree
- current implementation surface is still a Rust library/test repo, not a mixed Rust + desktop-shell workspace

### Toolchain truth in this runtime
Commands run:
- `cargo --version`
- `rustc --version`
- `node --version`
- `npm --version`

Observed versions:
- `cargo 1.96.0 (30a34c682 2026-05-25)`
- `rustc 1.96.0 (ac68faa20 2026-05-25)`
- `node v22.22.3`
- `npm 10.9.8`

This does not prove Tauri packaging/signing readiness. It does prove that the runtime is no longer blocked at the crude level of “missing Rust or Node entirely.”

### Core verification truth
Command run:
- `cargo test --quiet`

Observed result:
- the current Rust suite passed in the live repo before any GE-07 changes

That means GE07-E1 should preserve the current headless-core proof surface and layer a shell scaffold around it, not rewrite the core to make the shell possible.

## Smallest honest scaffold shape
The first non-production scaffold should stay additive and obvious. The smallest grounded directory posture is:

```text
apps/
  desktop/
    package.json
    src/
      main.tsx
      App.tsx
    src-tauri/
      Cargo.toml
      tauri.conf.json
      src/
        main.rs
```

Why this shape is the right first move:
- it leaves the existing root Rust crate and tests untouched as the authoritative headless core
- it makes the desktop shell visibly additive rather than silently entangling UI and domain code
- it matches the reference-architecture posture closely enough to avoid gratuitous novelty
- it is narrow enough to produce a startup/scaffold receipt without counterfeiting product capability

## What this receipt authorizes
This receipt authorizes documentary truth only:
- GE07-E1 may be described as a grounded non-production scaffold candidate
- future GE-07 readiness work may use the path classes above as the default starting hypothesis
- future GE-07 handoffs may assume the runtime has Rust and Node toolchains available unless a later check disproves it

## What this receipt does not authorize
This receipt does not authorize:
- a broad “build the UI” lane
- product-visible truth claims
- React finality as an irrevocable decision
- final Tauri transport/API design
- packaging/signing claims for Linux, Windows, or macOS
- repo writes without a later stage-specific readiness closure and execution handoff

## Completion rule
This artifact is complete because it records the exact live repo/toolchain posture, names the smallest additive shell scaffold shape, and preserves the rule that GE07-E1 is still a non-production spike rather than a product claim.
