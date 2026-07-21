# CLAUDE.md

This file is the lightweight activation surface for Claude Code and similar coding harnesses in this repository.

The primary durable conduct file is `./AGENTS.md`.

## Required read order

1. Read this file.
2. Read `./AGENTS.md`.
3. Read the execution handoff or explicitly provided implementation brief.
4. Read only the repo files and supporting docs explicitly required by that brief.

## Activation rules

- Do not start code-writing from a raw grand epic or source STC alone.
- Do not proceed without an explicit bounded execution brief.
- Keep context lean: load the smallest authoritative surface that answers the current question.
- Stay inside the granted write scope.
- Do not claim completion without concrete verification.
- If scope, authority, or required reads are unclear, stop and surface the missing truth.
- Code paths that ship must actually do what they claim to do. No stubs, no fixture-only data in production paths, no empty event handlers on user-facing affordances. Full doctrine at `./governance/no-stub-mvp-doctrine.md`; companion skill `wired-integration-discipline`.

## Practical default

Use `AGENTS.md` for behavior, the execution handoff for scope, and the repo itself for implementation truth.
