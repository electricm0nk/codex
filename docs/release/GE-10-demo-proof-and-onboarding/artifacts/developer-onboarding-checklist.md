# Codex Developer Onboarding Checklist

## Purpose

Use this checklist when a new developer joins the project and needs the shortest truthful path from zero context to useful participation.

## Read first

1. `repos/codex/README.md`
2. `repos/codex/AGENTS.md`
3. `programs/codex/requirements/GE-10-demo-proof-and-onboarding/artifacts/current-project-state-summary.md`
4. `programs/codex/requirements/GE-10-demo-proof-and-onboarding/artifacts/vanilla-machine-demo-runbook.md`

## Run first

### Core proof surface

```bash
cd /home/ubuntu/workspace/repos/codex
. "$HOME/.cargo/env"
cargo test
```

### Desktop proof surface

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop
npm ci
npm run typecheck
npm run build
npm run tauri:check
npx tauri build --debug
```

## Understand these boundaries

- `repos/codex` is the implementation repo.
- `programs/codex` is the program/workspace control plane.
- Spec domains and source STCs define requirements and governance; they are not automatic code authority.
- The current desktop shell is real but bounded.
- PCGen is the heritage oracle substrate, not the architecture to clone.

## Do not assume

- that green tests mean broad feature completeness
- that a buildable desktop binary means product readiness
- that Linux verification automatically proves Windows or macOS paths
- that the repo README replaces the source STCs when planning new work
- that a planning-ready STC is an implementation handoff

## Minimum shared language

Use these phrases accurately:
- **Spec Domain -> Epic -> Story**
- **source STC** = authoritative requirements construct
- **execution handoff** = narrower code-authorizing brief
- **developer proof harness** = current truthful maturity posture

## Before taking implementation work

Confirm you have:
- the exact objective
- the exact target repo/workdir
- the exact allowed write scope
- the exact required reads
- the exact verification commands

If any of those are missing, stop and ask for the bounded handoff instead of guessing.
