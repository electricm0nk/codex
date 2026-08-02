---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-08-01)
date: 2026-08-01
canonical_branch: tranche/8 (operator directive 2026-08-01)
build_version_target: 0.8.<build>
companion_to: ./scope-draft.md, ./decisions.md, ./epic-breakdown.md
---

# SD-28 Technical Requirements

## Objective

Per-cycle, ingest one canonical record from one Ultimate book into
`src/rules_core/rules_tables/<book>/`, satisfy the reach gate, and observe
the file-touch partition.

## Normative language

- **MUST** — required; a cycle fails if missing.
- **SHOULD** — required unless a cycle receipt cites a substitute.
- **MUST NOT** — forbidden; a cycle fails if violated.

## TR-28-001 — Per-cycle file-touch partition

Cycle writes are bounded to:

- `src/rules_core/rules_tables/ultimate_<paizo-book>/` (one book per cycle).
- `src/rules_core/rules_tables/ultimate_psionics/` (Dreamscarred Press tier).
- `data/corpus/<book>/` (Shape B cache for the active book).
- `src/bin/sd28_*` (new) and `tests/sd28_*` (new) if the cycle requires them.
- `docs/release/SD-28-.../` (the bundle's own docs — published; landed source removed by move-not-copy).

Cycle writes MUST NOT touch:

- `src/rules_core/pilot_compute.rs`.
- `src/rules_core/rules_tables/<other_book>/`.
- `docs/release/v0.6/`.
- `src/oracle_validation/`.
- `src/pcgen_import/corpus_traps.rs` (read-only).

## TR-28-002 — Reach-gate dependency

A cycle is not done until the active record's IPC builder executes from
`apps/desktop/src-tauri/src/reach_gate.rs`. Per `decisions.md §18`, the
reach gate is the definition of done, not a side check.

The cycle MUST capture the gate's exit code in the cycle receipt.

## TR-28-003 — Pre-cycle trap-report (mandatory)

Before writing any ingest code for a book, the cycle MUST run:

```sh
cargo run --locked --bin v06_corpus_trap_report -- <book_dir>
```

and record the output in `artifacts/<book>-trap-report.md`. Skipping this
is a cycle-defect. The trap-report runs once per book, not once per cycle.

## TR-28-004 — Definition-of-done audit (5-grep dual-gate)

Every cycle MUST pass the wired-integration 4-grep audit
(`docs/governance/no-stub-mvp-doctrine.md` §"Per-cycle audit"). For
license-stripping cycles, a 5th audit grep applies (PI-blacklist). SD-28
operates on already-license-clean Shape B caches (per SD-27), so the 5th
audit is a courtesy check, not a gate.

## TR-28-005 — Build version

The bundle's first concrete build value is `0.8.<build>`, where `<build>` is
the current build counter. Per `decisions.md §15`. The closure PR
(Epic 10) reads the post-cycle counter.

## TR-28-006 — Identifier discipline

Per `decisions.md §6`:

- PascalCase for functions, methods, constants, properties, Tauri commands.
- lowercase camelCase for variables.
- Forbidden patterns: `sd28_*`, `SD28_*`, `Sd28*`, `sd28-*`, `t_<hex>`, `SD-28-Ex...`, `AV-PAY-N`.
- The identifier-discipline audit script in `scripts/` MUST exit 0 before commit.

## TR-28-007 — Cross-book conflict rule

When two SD-28 books (or SD-28 and a closed/adjacent SD-N) conflict on a
record, **the newer book is doctrine and the older book is errata** per
`decisions.md §16`. The class-grant overlap rule (`decisions.md §5`) is
the only exception; SD-30 owns canonical class definitions, SD-28 references.

## TR-28-008 — Dreamscarred Press third-party tier license gate

Before any Epic 9 cycle fires, the bundle MUST produce
`artifacts/dreamscarred-license-precheck.md` capturing the trap-report output
against `dreamscarred_press/ultimate_psionics/` and a license-conformance
finding per record. Records not matching open-content tier drop from
per-cycle scope.

## TR-28-009 — Per-entity counts are generated

Cycle receipts MUST cite `cargo run --locked --bin v06_work_inventory`
output for any figure they publish. Hand-maintained per-entity counts
are forbidden.

## TR-28-010 — Rules-as-data, no real-time engines

Per `decisions.md §18`:

- Numerical spell effects (e.g., 1d6/level for a caster level of 6) MUST be
  posted as calculated values in the spell description (e.g., `6d6`).
- The runtime MUST NOT call a die-rolling engine for these effects; the
  player rolls physical dice.
- Real-time engines (RNG, opponent state, turn sequencing) are out of scope.
- Rules-data engines (e.g., a feat's branching condition evaluator) are
  in scope only when strictly necessary to satisfy TR-28-002.

## TR-28-011 — Move-not-copy publish

Source-of-record `programs/codex/requirements/SD-28-.../` MUST be removed
on the publish commit per `decisions.md §22`. The canonical
repo-resident home is `docs/release/SD-28-ultimate-book-content-ingestion/`.

## TR-28-012 — Local-file work-queue dispatch

The cycle reads `kanban.md` at top to identify the next ready card
(replacing Hermes-board card dispatch per `decisions.md §15a`). Cycle
receipts append to `progress.md`. Per-cycle file-touch partition is
enforced by the supervisor reading one card at a time.

## Produced artifacts

- 12-file canonical chassis at `docs/release/SD-28-ultimate-book-content-ingestion/` (after the move-not-copy publish lands).
- `src/rules_core/rules_tables/ultimate_<paizo-book>/` plus `.../dreamscarred_press/ultimate_psionics/` — seven books' canonical records.
- `data/corpus/<book>/` — Shape B cache per book.
- Per-cycle artifacts under `artifacts/` — trap-reports, dreamscarred license precheck, progress receipts.
- `release-notes.md` populated at closure.

## Success definition

The bundle closes when:

1. All seven books' per-class/per-monster-block/per-equipment-entry cycles have reached the gate.
2. The Dreamscarred Press tier license precheck has been recorded.
3. Epic 10 (Closure Epilogue) has opened and merged the tranche promotion PR.
4. The workspace tree has been removed on the publish commit.
5. The canonical 12-file chassis lives at `docs/release/SD-28-ultimate-book-content-ingestion/`.
