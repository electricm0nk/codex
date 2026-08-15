---
canonical: true
owner: god-emporer
status: planning-ready (operator directive 2026-08-11)
date: 2026-08-11
canonical_branch: tranche/11
build_version_target: 0.11.<build>
companion_to: ./technical-design.md, ./epic-breakdown.md
---

# SD-33 Technical Requirements

## Objective

Per cycle, advance one slice of the `.pcg` import path — tokenizer, record model, reference
resolver, mapper, IPC command, or player surface — with the slice's own executable proof, and
observe the file-touch partition.

## Normative language

- **MUST** — required; a cycle fails if missing.
- **SHOULD** — required unless a cycle receipt cites a substitute.
- **MUST NOT** — forbidden; a cycle fails if violated.

## TR-31-001 — Per-cycle file-touch partition

This is the requirement that makes SD-33 safe to run beside SD-29 and SD-30. It is derived by
subtracting `TR-29-001` and `TR-30-001`'s partitions from the repo.

Cycle writes are bounded to:

- `src/pcgen_character/**` — **new module, created by this bundle**. The whole parser, record
  model, resolver, and mapper live here.
- `apps/desktop/src/**` — the player surface (import affordance, mapping-review screen,
  fidelity report). No other bundle writes this tree.
- `apps/desktop/src-tauri/src/character_hub.rs` and `.../src/main.rs` — **registration only**
  (see `TR-31-002`).
- `tests/pcgen_character_*.rs` — new integration tests, named per identifier discipline.
- `docs/release/SD-33-pcgen-character-import/**` — this bundle's own docs and artifacts.

Cycle writes MUST NOT touch:

- `src/rules_core/rules_tables/**` — SD-29 and SD-30's shared surface.
- `src/rules_core/pilot_compute.rs` — SD-30's declared exception surface.
- `src/rules_core/archetype_resolver.rs` — SD-30's.
- `data/corpus/**` — SD-29's Shape-B cache.
- `src/pcgen_import/**` — read-only, **except** the single `mod.rs` registration line
  `TR-31-002` permits. `corpus_traps.rs` is read-only without exception.
- `src/oracle_validation/**` — read-only. SD-33 *calls* `pcgen_runner`; it does not modify it.
- `apps/desktop/src-tauri/src/reach_gate.rs` — both in-flight bundles depend on it.
- `docs/work-inventory.json` and `docs/release/v0.6/`.

## TR-31-002 — The two shared files, handled by protocol

Two files cannot be avoided, because a new module must be registered and a new IPC command
must be exposed. Both are **append-only, single-site** edits:

1. `src/lib.rs` (or `src/pcgen_import/mod.rs`, whichever declares top-level modules) — one
   `pub mod pcgen_character;` line.
2. `apps/desktop/src-tauri/src/main.rs` — one entry appended to the `invoke_handler` list.

A cycle touching either file MUST:

- Change **only** the registration line/entry — no reformatting, no reordering, no unrelated edits.
- Run `git status --porcelain` immediately before and after, per the shared-checkout discipline.
- Record the exact diff hunk in its `progress.md` receipt.

Appending at a list's end keeps the conflict surface to a single line, which resolves
mechanically if a sibling bundle ever appends there too.

## TR-31-003 — The shared test-count baseline is NOT SD-33's to move

The root suite's expected-test-count constant is a live collision surface: SD-30 already moved
it (`5930 → 5933`, commit `3a4a4169`) and SD-29 adds tests continuously. Three bundles editing
one integer is a guaranteed conflict on every merge.

Therefore:

- SD-33 cycles MUST NOT edit the shared root test-count baseline.
- Each cycle MUST record its own test-count delta in its `progress.md` receipt.
- The tranche-merge cycle reconciles the accumulated delta **once**, at merge time, from those
  receipts.

## TR-31-004 — No silent reference loss (the doctrine requirement)

For every token in a `.pcg` that names mechanical content, the importer MUST produce exactly
one of:

- **Resolved** — bound to a corpus record, with that record's key recorded.
- **Unresolved** — named in the fidelity report, with the raw token text and the reason
  (unknown book, unregistered record, unsupported token kind).

Dropping a token silently — or mapping it to a nearest-match guess — is a cycle defect.
Cosmetic/bio fields (`EYECOLOR`, `PHOBIAS`, …) are exempt and MAY be carried verbatim or
ignored, but the exemption list MUST be explicit in code, not implicit in omission.

## TR-31-005 — Refuse rather than degrade

An import whose fidelity report contains any **mechanically significant** unresolved token
MUST return `Blocked` with those diagnostics and persist nothing, mirroring
`create_character`'s existing "never persist an unproven build" invariant
(`character_hub.rs:3744` onward).

The user may then re-run with an explicit acknowledgement (see `TR-31-006`). What is forbidden
is persisting a silently lossy character.

## TR-31-006 — Lossy import is an explicit, informed choice

A second entry point MAY accept an import with named losses, but only when the request carries
an explicit acknowledgement flag **and** the UI has displayed the full fidelity report first.
The acknowledgement MUST NOT default to true anywhere in the stack.

## TR-31-007 — Fixture integrity

Cycles MUST use the vendored `.pcg` fixtures rather than fabricating character files. Any new
fixture this bundle adds MUST be committed under this bundle's `artifacts/` directory with a
pinned sha256 and a provenance note, following the pattern
`tests/ge05_vendored_pcg_fixtures.rs` established.

Hand-authoring a `.pcg` to make a test pass is a cycle defect: the format's truth is what
PCGen writes, not what we think it writes.

## TR-31-008 — Definition-of-done audit

Every cycle MUST pass the wired-integration 4-grep audit
(`docs/governance/no-stub-mvp-doctrine.md` §"Per-cycle audit"). An import affordance that
renders but invokes nothing is the exact failure that doctrine exists to catch.

## TR-31-009 — Gate

Every cycle MUST run and record:

```sh
cargo test --locked pcgen_character
cd apps/desktop && npm test && npx tsc --noEmit
```

A gate returning exit 0 with zero matched tests is a hard failure.

## TR-31-010 — Build version

The bundle's first concrete build value is `0.11.<build>`, following `tranche/11`.

## Out of scope

- **Export to `.pcg`.** Import only. Round-tripping back out is forward scope.
- **Non-PF1e game modes.** `GAMEMODE:Pathfinder_RPG` only; other modes are refused with a
  named diagnostic, not partially parsed.
- **PCGen party/campaign files** (`.pcp`). Single characters only.
- **Widening the corpus** to resolve a fixture's references. If a `.pcg` names content Codex
  has not ingested, that is an SD-29/SD-30 concern and a forward-scope entry — never an
  in-cycle ingest.
