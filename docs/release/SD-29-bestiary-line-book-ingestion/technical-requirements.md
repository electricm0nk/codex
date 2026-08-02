---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-08-01)
date: 2026-08-01
canonical_branch: tranche/9 (operator directive 2026-08-01)
build_version_target: 0.9.<build>
companion_to: ./scope-draft.md, ./decisions.md, ./epic-breakdown.md
---

# SD-29 Technical Requirements

## Objective

Per-cycle, ingest one canonical record from one bestiary into
`src/rules_core/rules_tables/beastiary<N>/`, satisfy the reach gate, and
observe the file-touch partition.

## Normative language

- **MUST** — required; a cycle fails if missing.
- **SHOULD** — required unless a cycle receipt cites a substitute.
- **MUST NOT** — forbidden; a cycle fails if violated.

## TR-29-001 — Per-cycle file-touch partition

Cycle writes are bounded to:

- `src/rules_core/rules_tables/beastiary<N>/` (one bestiary per cycle).
- `data/corpus/beastiary<N>/` (Shape B cache for the active bestiary).
- new bins under `src/bin/` and new tests under `tests/` named per the identifier-discipline doctrine (no `sd29_` prefix).
- `docs/release/SD-29-.../` (the bundle's own docs — published; landed source removed by move-not-copy).

Cycle writes MUST NOT touch:

- `src/rules_core/pilot_compute.rs`.
- `src/rules_core/rules_tables/<other_bestiary>/`.
- `docs/release/v0.6/`.
- `src/oracle_validation/`.
- `src/pcgen_import/corpus_traps.rs` (read-only).

## TR-29-002 — Reach-gate dependency

A cycle is not done until the active record's IPC builder executes from
`apps/desktop/src-tauri/src/reach_gate.rs`. Per `decisions.md §19`, the
reach gate is the definition of done, not a side check.

The cycle MUST capture the gate's exit code in the cycle receipt.

## TR-29-003 — Pre-cycle trap-report (mandatory)

Before writing any ingest code for a bestiary, the cycle MUST run:

```sh
cargo run --locked --bin v06_corpus_trap_report -- <book_dir>
```

and record the output in `artifacts/<book>-trap-report.md`. Skipping this
is a cycle-defect. The trap-report runs once per book, not once per cycle.

## TR-29-004 — Definition-of-done audit (4-grep dual-gate)

Every cycle MUST pass the wired-integration 4-grep audit
(`docs/governance/no-stub-mvp-doctrine.md` §"Per-cycle audit").

## TR-29-005 — Build version

The bundle's first concrete build value is `0.9.<build>`, where `<build>` is
the current build counter. Per `decisions.md §14`. The closure PR
(Epic 8) reads the post-cycle counter.

## TR-29-006 — Identifier discipline

Per `decisions.md §6`:

- PascalCase for functions, methods, constants, properties, Tauri commands.
- lowercase camelCase for variables.
- Forbidden patterns: `sd29_*`, `SD29_*`, `Sd29*`, `sd29-*`, `t_<hex>`, `SD-29-Ex...`, `AV-PAY-N`.
- The identifier-discipline audit script in `scripts/` MUST exit 0 before commit.

## TR-29-007 — Cross-book conflict rule

When a monster reprint or erratum conflicts with another book, **the newer
book is doctrine and the older book is errata** per `decisions.md §16`.
The class-grant overlap rule (canonical class definition lives in the
bundle that owns the book's primary class definition) is the only
exception.

## TR-29-008 — Bestiary 5 shape-resolution gate

Bestiary 5 is gated on cycle-0 trap-report + work-inventory output. If the
inventory surfaces zero `monster` units, Epic 6's cycle runs the
per-race / per-feat / per-companion-mod cycles against Bestiary 5's
`b5_*` LST files instead of the monster-block cycle shape applied to
Bestiary 2-4.

## TR-29-009 — Per-entity counts are generated

Cycle receipts MUST cite `cargo run --locked --bin v06_work_inventory`
output for any figure they publish. Hand-maintained per-entity counts
are forbidden.

## TR-29-010 — Rules-as-data, no real-time engines

Per `decisions.md §19`:

- Numerical monster abilities (e.g., a monster's damage die dropping `2d6` for a confirmed CR) MUST be posted as calculated values where appropriate.
- The runtime MUST NOT call a die-rolling function for these effects.
- Real-time engines are out of scope.
- Rules-data engines are in scope only when strictly necessary to satisfy TR-29-002.

## TR-29-011 — Move-not-copy publish

Source-of-record `programs/codex/requirements/SD-29-.../` MUST be removed
on the publish commit per `decisions.md §13`. The canonical
repo-resident home is `docs/release/SD-29-bestiary-line-book-ingestion/`.

## TR-29-012 — Local-file work-queue dispatch

The cycle reads `kanban.md` at top to identify the next ready card
(replacing Hermes-board card dispatch per `decisions.md §14a`). Cycle
receipts append to `progress.md`. Per-cycle file-touch partition is
enforced by the supervisor reading one card at a time.

## Produced artifacts

- 15-file canonical chassis at `docs/release/SD-29-bestiary-line-book-ingestion/` (after the move-not-copy publish lands).
- `src/rules_core/rules_tables/beastiary{2,3,4,5}/` — four bestiaries' canonical records.
- `data/corpus/beastiary{2,3,4,5}/` — Shape B cache per book.
- Per-cycle artifacts under `artifacts/` — trap-reports, cycle-0 inventory findings, progress receipts.
- `release-notes.md` populated at closure.

## Success definition

The bundle closes when:

1. All four bestiaries' per-monster-block (or per-race / per-feat / per-companion-mod for B5) cycles have reached the gate.
2. Epic 7 (DM Toolkit extension) lands (in scope) or surfaces as a Class 1/3 retrofit.
3. Epic 8 (Closure Epilogue) has opened and merged the tranche promotion PR.
4. The workspace tree has been removed on the publish commit.
5. The canonical 15-file chassis lives at `docs/release/SD-29-bestiary-line-book-ingestion/`.
