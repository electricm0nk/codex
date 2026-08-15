---
canonical: true
owner: god-emporer
status: planning-ready (operator directive 2026-08-11)
date: 2026-08-11
canonical_branch: tranche/11
kanban_board: retired — local-file dispatch via ./kanban.md
companion_to: ./scope-draft.md
build_version_target: 0.11.<build>
---

# SD-33 — PCGen Character Import (`.pcg`)

**Renamed from SD-31, operator ruling 2026-08-14** ("ok, let's split phase 3 and phase 4 into their
own SD's. SD-31 and SD-32. Take the existing SD-31 and rename it to SD-33"). The `SD-31`/`SD31`
identifiers throughout this package's own files were renamed in place (`git mv`, history preserved);
`SD-31` now names the package split out of `SD-30-class-feature-archetype-bundle`
(`docs/release/SD-31-corpus-closure-grind/`) — not this package. `SD-32` was also split out of SD-30
at the same time (`SD-32-engine-capability-builds`) but was absorbed back into SD-31 and deleted on
2026-08-15 (`SD-31-corpus-closure-grind/decisions.md §2`); that number currently names no package. Historical citations to "SD-31" in other packages' closed decision/progress records
(e.g. `SD-29-corpus-wide-catch-up-lanes/`) predate this rename and are left as-is per this program's
standing convention (original text stays visible, a correction pointer added where the reference is
still a live pointer rather than a closed narrative record).

## Purpose

Codex can read PCGen's **data** (`.lst`/`.pcc`, via `src/pcgen_import/`). It cannot read
PCGen's **characters**. A user with an existing PF1e character built in PCGen has no path
into this application except retyping it.

SD-33 builds that path: open a PCGen `.pcg` character file, resolve its references against
Codex's ingested corpus, recompute the character through the real rules engine, and either
save it or refuse with named diagnostics.

## Why this bundle exists now

It is the one substantial feature lane that is **write-disjoint from both bundles currently
in flight**:

- **SD-29** (`tranche/9`, corpus-wide kind lanes) writes `src/rules_core/rules_tables/<book>/`,
  `data/corpus/<book>/`, `src/bin/`, `tests/` (`TR-29-001`).
- **SD-30** (`tranche/10`, `class_feature` corpus-wide) writes the same, plus
  `archetype_resolver.rs` and `pilot_compute.rs` (`TR-30-001`).

Neither partition contains `src/pcgen_character/`, `apps/desktop/src/`, or the character-hub
IPC surface. `TR-31-001` states SD-33's partition and the two shared files it must handle by
protocol rather than by edit.

## This is not a greenfield build

The bundle's starting state is four existing assets, verified present at `tranche/9` HEAD
`3570d735` on 2026-08-11. Cycles cite them; they do not rebuild them.

| Asset | Location | What it gives SD-33 |
|---|---|---|
| Tokenized-line parser idiom | `src/pcgen_import/lst_parser/` | `.pcg` is the same `KEY:value` line format as `.lst` |
| Two real `.pcg` fixtures | `docs/release/GE-05-.../artifacts/*.pcg` | Human Fighter L1, Human Wizard L1, sha256-pinned by `tests/ge05_vendored_pcg_fixtures.rs` |
| Safe import landing pad | `character_hub.rs:3759` `import_character_from_json` | Mints a fresh id, recomputes via the real engine, returns `Blocked` rather than persisting an unproven build |
| A real oracle | `src/oracle_validation/pcgen_runner.rs` | Drives headless PCGen against a `.pcg` and normalizes to typed dimensions |

The fourth is what makes this bundle verifiable rather than merely testable — see
`acceptance-and-verification.md §2`.

## The hard problem, named up front

A `.pcg` may reference content Codex has not ingested — a feat from an unregistered book, an
archetype SD-30 has not reached, an item from a book outside the corpus. **The importer must
never silently drop a reference.** Every unresolved token is a named, user-visible diagnostic
carried in a fidelity report, and a character that loses mechanical content is refused, not
quietly degraded.

This is `docs/governance/no-stub-mvp-doctrine.md` applied to an import path: a character that
claims to be your PCGen character must actually be your PCGen character.

## Source STC contents

- `scope-draft.md` — bundle shape and the in/out boundary.
- `decisions.md` — operative decisions `§1` onward.
- `epic-breakdown.md` — dependency-ordered epics.
- `technical-requirements.md` — write partition + normative requirements.
- `technical-design.md` — the three-layer architecture and the reference-resolution model.
- `acceptance-and-verification.md` — the dual-oracle definition of done.
- `kanban.md` / `progress.md` — local-file dispatch queue and per-cycle receipts.
- `loop-instruction.md` — per-cycle procedure.
- `forward-scope-register.md` — successor work this bundle deliberately defers.
- `risks-and-open-questions.md` — live risks, including the one shared-file collision.
- `release-notes.md` — closure notes.

## Cross-references

- `../corpus-work-channels.md` — channel ownership.
- `../../governance/no-stub-mvp-doctrine.md` — the doctrine the fidelity report enforces.
- `../../governance/identifier-discipline.md` — no `sd31_*` identifiers in surface code.
- `../SD-29-corpus-wide-catch-up-lanes/technical-requirements.md` `TR-29-001` — the partition
  this bundle stays clear of.
- `../SD-30-class-feature-archetype-bundle/technical-requirements.md` `TR-30-001` — likewise.
