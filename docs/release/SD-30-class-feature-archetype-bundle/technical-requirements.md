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

**Re-scoped 2026-08-10 (`decisions.md §33-38`).** "The sixteen books" below is retired language;
SD-30's scope is now `class_feature` corpus-wide, 23 books. TR-30-001, TR-30-007, TR-30-012, and the
Produced Artifacts / Success Definition sections are updated inline. Where "sixteen" appears
unedited elsewhere in this file, it is stale — the 23-book, dependency-gated shape in
`epic-breakdown.md` governs.

## Objective

Per-cycle, either (a) hand-verify one class's archetype-slot wireable fraction (Epic 4), (b) wire one
class's measured mechanisms (Epic 5), or (c) ingest one canonical `class_feature` record for a class
Epic 4/5 have cleared (Epic 6) into `src/rules_core/rules_tables/<book>/`, satisfy the reach gate, and
observe the file-touch partition. A class's Epic 6 cycle MUST NOT fire before that class's Epic 4
measurement receipt exists (`decisions.md §37`).

## Normative language

- **MUST** — required; a cycle fails if missing.
- **SHOULD** — required unless a cycle receipt cites a substitute.
- **MUST NOT** — forbidden; a cycle fails if violated.

## TR-30-001 — Per-cycle file-touch partition

Cycle writes are bounded to:

- `src/rules_core/rules_tables/<book>/` (one book per Epic 6 cycle; `<book>` ∈ the 23 `class_feature`-bearing corpus dirs, `decisions.md §33`).
- `src/rules_core/archetype_resolver.rs` and `src/rules_core/pilot_compute.rs` (Epic 5 mechanism cycles only — the one exception to the historical "never touch pilot_compute.rs" rule below, scoped to the specific class's supersession/chooser mechanisms being wired).
- `data/corpus/<book>/` (Shape B cache for the active book).
- `src/bin/sd30_*` (new) and `tests/sd30_*` (new) if the cycle requires them.
- `docs/release/SD-30-.../` (the bundle's own docs — published; landed source removed by move-not-copy).

Cycle writes MUST NOT touch:

- `src/rules_core/pilot_compute.rs` — **except** an Epic 5 cycle wiring a specific class's measured
  mechanisms, scoped to that class's own supersession/chooser branch only.
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
repo-resident home is `docs/release/SD-30-class-feature-archetype-bundle/`.
**SATISFIED 2026-08-01:** the publish landed; the workspace directory is
gone and this package is repo-resident (renamed 2026-08-10 to
`docs/release/SD-30-class-feature-archetype-bundle/`; the rename does not
re-run the move-not-copy publish). Closure re-verifies (Closure-F2).

## TR-30-011 — Local-file work-queue dispatch

The cycle reads `kanban.md` at top to identify the next ready card
(replacing Hermes-board card dispatch per `decisions.md §14a`). Cycle
receipts append to `progress.md`. Per-cycle file-touch partition is
enforced by the supervisor reading one card at a time.

## TR-30-013 — Per-class measurement gate (NEW, 2026-08-10, `decisions.md §37`)

Epic 6 (chassis sweep) and Epic 5 (mechanism) MUST NOT claim a class-scoped cycle unless Epic 4
(per-class measurement) has produced that class's `wired-able / named` figure by direct evidence, no
automated proxy, cited in the claiming cycle's receipt. This is a per-class gate, not a bundle-wide
gate — Epic 4 does not need to reach 100% of all classes before any Epic 5/6 cycle starts.

## TR-30-012 — Cycle-0 trap-report gating

Before any Epic 6 chassis-sweep cycle fires for a given book, Epic 2's pre-flight runs the
trap-report + work-inventory against that book (re-derived corpus-wide for `class_feature` across the
23 in-scope books, not the old sixteen — `decisions.md §33`). Each book's inventory surfaces the
per-book shape (kinds, files_not_enumerated, trap_hits); cycles dispatch per the shape finding and per
TR-30-013's per-class gate.

## Produced artifacts

- Canonical file chassis at `docs/release/SD-30-class-feature-archetype-bundle/` (after the
  move-not-copy publish; renamed 2026-08-10 from `SD-30-occult-and-companion-content-ingestion`).
- `src/rules_core/rules_tables/<book>/` `class_feature` records for the 23 in-scope books.
- `archetype_resolver.rs`/`pilot_compute.rs` supersession + chooser-interaction wiring.
- Per-class measurement receipts, one per class (Epic 4), never blended.
- Per-cycle artifacts under `artifacts/` — trap-reports, cycle-0 inventory findings, progress receipts.
- `release-notes.md` populated at closure.

## Success definition

The bundle closes when:

1. Epic 4 has measured every `class_feature`-bearing class or named a successor for the remainder.
2. Epic 5 has landed the supersession shape for measured classes and resolved-or-deferred the
   chooser-interaction shape for Oracle/Arcanist/Sorcerer.
3. Epic 6's chassis-sweep cycles have reached the gate for every class Epic 4/5 cleared.
4. Closure has opened and merged the tranche promotion PR.
5. The workspace tree has been removed on the publish commit (already satisfied, TR-30-010).
6. The canonical file chassis lives at `docs/release/SD-30-class-feature-archetype-bundle/`.
