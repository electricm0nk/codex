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

**Re-cut 2026-08-10 (`decisions.md §37`).** Requirements below are re-scoped from four/per-book
framing to the 11-epic kind-lane structure in `epic-breakdown.md`. `beastiary<N>` below ranges over
all seven in-scope books, not the retired `{2,3,4,5}` set, and TR-29-001's file-touch partition is
per-book-within-a-lane (a lane cycle-batch still writes to exactly one book's tree at a time).

**RE-SCOPED CORPUS-WIDE, 2026-08-10 (`decisions.md §38`).** `beastiary<N>`/`<book>` below now
ranges over all 37 in-scope books (`../corpus-work-channels.md §10.2`), not the seven named above.
TR-29-001's per-book-within-a-lane partition is unaffected — it already generalizes to any book
count. Epic numbers referenced below shifted per `decisions.md §38`/`epic-breakdown.md`: Epic 4 is
now the corpus-wide proven-path lane; Monster/Monster-Ability moved to Epic 5, Race-Trait to Epic 6,
Companion to Epic 7.

## Objective

Per lane cycle-batch, ingest one canonical record for one kind (monster+monster_ability chassis,
race_trait, companion, or a residual proven-path kind) from one book into
`src/rules_core/rules_tables/<book>/`, satisfy the reach gate, pass the Epic 3 provenance gate, and
observe the file-touch partition.

## Normative language

- **MUST** — required; a cycle fails if missing.
- **SHOULD** — required unless a cycle receipt cites a substitute.
- **MUST NOT** — forbidden; a cycle fails if violated.

## TR-29-001 — Per-cycle file-touch partition

Cycle writes are bounded to:

- `src/rules_core/rules_tables/<book>/` (one book per cycle, within the lane's assigned kind — e.g.
  Epic 4's Bonus Bestiary pilot cycle writes only Bonus Bestiary's monster/monster_ability records).
- `data/corpus/<book>/` (Shape B cache for the active book).
- new bins under `src/bin/` and new tests under `tests/` named per the identifier-discipline doctrine (no `sd29_` prefix).
- `docs/release/SD-29-.../` (the bundle's own docs — published; landed source removed by move-not-copy).

Cycle writes MUST NOT touch:

- `src/rules_core/pilot_compute.rs`.
- `src/rules_core/rules_tables/<other_book>/`.
- another lane's kind within the same book (e.g. an Epic 4 cycle on Bestiary 2 MUST NOT write Epic 5's `race_trait` records for the same book).
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

## TR-29-008 — Provenance gate: PI-screening wired into every lane's extraction step

**Retired shape (pre-2026-08-10):** a per-book shape-resolution gate for Bestiary 5. Superseded —
Bestiary 5 has no `monster` epic to gate; its `monster_ability`/`race_trait`/`companion` units are
each the affected lane's ordinary per-book cycle-batch, per `decisions.md §37`.

Per `decisions.md §37.3` and Epic 3 (`epic-breakdown.md`): before any lane (Epic 4, 5, 6, or 7)
lands its first content commit for a book, `pi_screening::classify_field` (or the 55-term
blacklist sweep it implements) MUST run against that lane's own newly-generated content, and the
sweep's output MUST be recorded in the cycle receipt. `docs/governance/license-matrix.md` (commit
`314a7ad9`) found zero PI-screening anywhere in `rules_tables/*.rs` — the pipeline every SD-29
lane writes into — and three real, unredacted leaks in other bundles' tables of the same pipeline.
A hit is a hard stop for that record, per `loop-instruction.md` "Stop vs. press on."

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
repo-resident home is `docs/release/SD-29-corpus-wide-catch-up-lanes/`.

## TR-29-012 — Local-file work-queue dispatch

The cycle reads `kanban.md` at top to identify the next ready card
(replacing Hermes-board card dispatch per `decisions.md §14a`). Cycle
receipts append to `progress.md`. Per-cycle file-touch partition is
enforced by the supervisor reading one card at a time.

## Produced artifacts

- 14-file canonical chassis at `docs/release/SD-29-corpus-wide-catch-up-lanes/` (after the move-not-copy publish lands).
- `src/rules_core/rules_tables/{beastiary2,beastiary3,beastiary4,beastiary5,beastiary6,bonus_bestiary,monster_codex}/` — all seven books' canonical records, populated lane by lane (Epic 4's monster+monster_ability chassis, Epic 5's race-trait, Epic 6's companion, Epic 7's residual kinds).
- `data/corpus/{bestiary_2,bestiary_3,bestiary_4,bestiary_5,bestiary_6,bonus_bestiary,monster_codex}/` — Shape B cache per book.
- Per-cycle artifacts under `artifacts/` — trap-reports (one per book, corpus-wide pre-flight), per-lane PI-screening sweep outputs, progress receipts.
- `release-notes.md` populated at closure, rolled up by lane.

## Success definition

The bundle closes when:

1. All four content lanes (Epic 4 monster+monster_ability chassis, Epic 5 race-trait, Epic 6 companion, Epic 7 residual) have reached the gate for every book carrying units of that lane's kind, with Epic 3's provenance gate cleared per book per lane.
2. Epic 8 (DM Toolkit extension) lands (in scope) or surfaces as a Class 1/3 retrofit.
3. Epic 11 (Closure Epilogue) has opened and merged the tranche promotion PR.
4. The workspace tree has been removed on the publish commit.
5. The canonical 14-file chassis lives at `docs/release/SD-29-corpus-wide-catch-up-lanes/`.
