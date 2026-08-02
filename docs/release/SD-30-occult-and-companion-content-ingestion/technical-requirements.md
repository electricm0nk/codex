---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-08-01)
date: 2026-08-01
canonical_branch: tranche/10 (operator directive 2026-08-01)
build_version_target: 0.10.<build>
companion_to: ./scope-draft.md, ./decisions.md, ./epic-breakdown.md
---

# SD-30 Technical Requirements

## Objective

Per-cycle, ingest one canonical record from one of the sixteen books
into `src/rules_core/rules_tables/<book>/`, satisfy the reach gate, and
observe the file-touch partition.

## Normative language

- **MUST** — required; a cycle fails if missing.
- **SHOULD** — required unless a cycle receipt cites a substitute.
- **MUST NOT** — forbidden; a cycle fails if violated.

## TR-30-001 — Per-cycle file-touch partition

Cycle writes are bounded to:

- `src/rules_core/rules_tables/<book>/` (one book per cycle; `<book>` ∈ sixteen in-scope corpus dirs).
- `data/corpus/<book>/` (Shape B cache for the active book).
- `src/bin/sd30_*` (new) and `tests/sd30_*` (new) if the cycle requires them.
- `docs/release/SD-30-.../` (the bundle's own docs — published; landed source removed by move-not-copy).

Cycle writes MUST NOT touch:

- `src/rules_core/pilot_compute.rs`.
- `src/rules_core/rules_tables/<other_book>/`.
- `docs/release/v0.6/`.
- `src/oracle_validation/`.
- `src/pcgen_import/corpus_traps.rs` (read-only).

## TR-30-002 — Reach-gate dependency (PRIME RULE per `decisions.md §18`)

A cycle is not done until the active record's IPC builder executes from
`apps/desktop/src-tauri/src/reach_gate.rs`. This is the prime rule —
`decisions.md §18` calls it out as load-bearing.

The cycle MUST capture the gate's exit code in the cycle receipt. A
gate that returns exit code 0 with zero matched tests is a hard failure
(a gate running zero tests asserts nothing).

## TR-30-003 — Pre-cycle trap-report (mandatory)

Before writing any ingest code for a book, the cycle MUST run:

```sh
cargo run --locked --bin v06_corpus_trap_report -- <book_dir>
```

and record the output in `artifacts/<book>-trap-report.md`. Skipping this
is a cycle-defect. The trap-report runs once per book, not once per cycle.

## TR-30-004 — Definition-of-done audit (4-grep dual-gate)

Every cycle MUST pass the wired-integration 4-grep audit
(`docs/governance/no-stub-mvp-doctrine.md` §"Per-cycle audit").

## TR-30-005 — Build version

The bundle's first concrete build value is `0.10.<build>`, where
`<build>` is the current build counter. Per `decisions.md §15`. The
closure PR reads the post-cycle counter.

## TR-30-006 — Identifier discipline

Per `decisions.md §6`:

- PascalCase for functions, methods, constants, properties, Tauri commands.
- lowercase camelCase for variables.
- Forbidden patterns: `sd30_*`, `SD30_*`, `Sd30*`, `sd30-*`, `t_<hex>`, `SD-30-Ex...`, `AV-PAY-N`.
- The identifier-discipline audit script in `scripts/` MUST exit 0 before commit.

## TR-30-007 — Cross-book conflict rule

When two SD-30 books (or SD-30 and a closed/adjacent SD-N) conflict on
a record, **the newer book is doctrine and the older book is errata**
per `decisions.md §16`. The class-grant overlap rule (canonical class
definition lives in the bundle that owns the book's primary class
definition) is the only exception.

**Cross-bundle precedence.** Per the 2026-08-01 "recently published takes
precident" rule, when SD-30's records conflict with records SD-28 /
SD-29 already published, SD-28 / SD-29's records are doctrine because
they were published more recently. SD-30 references the canonical id
only.

## TR-30-008 — Per-entity counts are generated

Cycle receipts MUST cite `cargo run --locked --bin v06_work_inventory`
output for any figure they publish. Hand-maintained per-entity counts
are forbidden.

## TR-30-009 — Rules-as-data, no real-time engines (PRIME RULE)

Per `decisions.md §18`:

- Numerical effects (e.g., a Mythic tier's `+5` damage bonus posted as `15` total damage at the documented CL) MUST be posted as calculated values where appropriate.
- The runtime MUST NOT call a die-rolling function for these effects.
- Real-time engines are out of scope.
- Rules-data engines are in scope only when strictly necessary to satisfy TR-30-002.

## TR-30-010 — Move-not-copy publish

Source-of-record `programs/codex/requirements/SD-30-.../` MUST be
removed on the publish commit per the move-not-copy doctrine
(`forward-scope-register.md` Class 0 anchor; `AT-30-011`). The canonical
repo-resident home is `docs/release/SD-30-occult-and-companion-content-ingestion/`.
**SATISFIED 2026-08-01:** the publish landed; the workspace directory is
gone and this package is repo-resident. Closure re-verifies (Closure-F2).

## TR-30-011 — Local-file work-queue dispatch

The cycle reads `kanban.md` at top to identify the next ready card
(replacing Hermes-board card dispatch per `decisions.md §14a`). Cycle
receipts append to `progress.md`. Per-cycle file-touch partition is
enforced by the supervisor reading one card at a time.

## TR-30-012 — Cycle-0 trap-report gating

Before any per-book cycle fires, Epic 2's pre-flight runs the
trap-report + work-inventory against all sixteen books in scope. Each
book's inventory surfaces the per-book shape (kinds, files_not_enumerated,
trap_hits); per-book cycles dispatch per the shape finding. Books
without inventory `monster` units (Bestiary 5 precedent from SD-29)
adapt to per-trait / per-race cycles instead of per-monster-block.

## Produced artifacts

- 13+ file canonical chassis at `docs/release/SD-30-occult-and-companion-content-ingestion/` (after the move-not-copy publish lands).
- `src/rules_core/rules_tables/<book>/` for sixteen in-scope books — canonical records.
- `data/corpus/<book>/` — Shape B cache per book.
- Per-cycle artifacts under `artifacts/` — trap-reports, cycle-0 inventory findings, progress receipts.
- `release-notes.md` populated at closure.

## Success definition

The bundle closes when:

1. All sixteen in-scope books' cycles have reached the gate.
2. Closure has opened and merged the tranche promotion PR.
3. The workspace tree has been removed on the publish commit.
4. The canonical 13+ file chassis lives at `docs/release/SD-30-occult-and-companion-content-ingestion/`.
