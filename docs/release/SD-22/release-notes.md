---
title: SD-22 — Content-Source Ingest (APG + ACG + Bestiary 1) + DM Toolkit — Release Notes
release_version: 0.5.96
canonical_branch: tranche/5
date: 2026-07-20
companion_to: ./progress.md, ./epic-breakdown.md, ./decisions.md, ./closure-readiness-report.md
---

# SD-22 — Release Notes

Tranche-5 release: content-source ingest for two Pathfinder 1e source books
(Advanced Player's Guide, Advanced Class Guide) plus Bestiary 1 monster data,
a new DM toolkit (encounter difficulty + party challenge rating), a
code-side identifier-cleanup pass, and the closure infrastructure that ships
this release.

## New content

**Advanced Player's Guide (APG) — 6 classes (Epic 3, criteria 6-9).**
`src/rules_core/rules_tables/apg/` ships BAB/save chassis for all six real
APG base classes — Alchemist, Cavalier, Inquisitor, Oracle, Summoner, Witch
— each transcribed directly from the real `apg_classes.lst` /
`apg_abilities_class.lst` PCGen records, with cross-book resolution tests
confirming `RuleSetId::Apg` queries resolve correctly and fall through to
CRB where appropriate. Bootstrap spell (`apg/spell_list.rs`) and equipment
(`apg/equipment_tables.rs`) sample tables ship alongside the class chassis.
(Gunslinger and Magus were correctly excluded — they are Ultimate
Combat / Ultimate Magic content, not APG, per the real corpus roster.)

**Advanced Class Guide (ACG) — 10 classes (Epic 4, criteria 10-13).**
`src/rules_core/rules_tables/acg/` ships the full real ACG roster — Arcanist,
Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Slayer,
Swashbuckler, Warpriest — each with the same real-`.lst`-sourced BAB/save
chassis and `RuleSetId::Acg` cross-book resolution tests. Bootstrap spell
and equipment tables ship in `acg/spell_list.rs` / `acg/equipment_tables.rs`.
("Alchemist" was correctly dropped from the ACG roster — no ACG-side
Alchemist record exists in the real corpus — and `Slayer` was added, which
does have a real record but was missing from the original planning roster.)

**Bestiary 1 — 8 subsets, 41 monsters (Epic 5, criteria 14-17).**
`src/rules_core/rules_tables/beastiary1/` ships 8 monster-block subsets
(`monster_subset_01.rs` through `monster_subset_08.rs`), covering CR 1
through CR 3 alphabetically within each CR band — 41 monsters total,
meeting the acceptance floor of 8-12 subsets. A new bare-tab-delimited
monster-stat-block parser (`src/pcgen_import/lst_parser/monster_stat_block.rs`)
was added to read the real `b1_races.lst` records, which use an unprefixed
row shape the existing `RACE:`/`ABILITY:` parser didn't cover.
`RuleSetId::Bestiary1` cross-book resolution tests hold for all eight
subsets.

## DM toolkit

Epic 6 (criteria 18-21) ships a new DM-facing toolkit grounded in the PF1
Core Rulebook's "Gamemastering" chapter:

- **`src/rules_core/encounters.rs`** — `Encounter::new`, `CharacterSnapshot`,
  `MonsterRef`, `Difficulty`, and `EncounterResult`, implementing the
  Table: Encounter Design / Table: CR Equivalencies / Table: Experience
  Point Awards rules (criterion 18).
- **`src/rules_core/party_cr.rs`** — `party_challenge_rating`, implementing
  the "Designing Encounters → Step 1 — Determine APL" rule (criterion 19).
- **Deterministic tests** (`tests/sd22_dm_toolkit_deterministic.rs`) — five
  acceptance-level tests covering both modules against the canonical Paizo
  examples (criterion 20).
- **Happy-path integration test**
  (`tests/sd22_dm_toolkit_happy_path_integration.rs`) — consumes real
  ingested Bestiary 1 monster blocks (Ghoul, Darkmantle) through
  `Encounter::new`, confirming the toolkit works end-to-end against Epic 5's
  real ingested content, not synthetic fixtures (criterion 21).

## Maintenance

Epic 1 (criteria 1-2) ran a source-code identifier audit
(`grep -rE "sd22_|SD22_|Sd22|SD-22-[A-Z][0-9]"` across `apps/desktop/`,
`apps/desktop/src-tauri/`, `src/rules_core/`) before any Epic 3 content
landed, confirming zero identifier-discipline leaks (Tauri command names,
TypeScript symbols, `data-testid` attributes, or embedded kanban/audit
tokens) carrying SD-22-specific scratch naming. The audit was vacuous — no
renames were required — and was re-verified live at Epic 9's closure-
readiness eval with the same clean result.

## Versioning

Epic 8 (criteria 27-29) established the three-position
`<major>.<tranche-base>.<build>` build-version scheme on this branch:
`major` increments only on first publish to `main`; `tranche-base` tracks
the active tranche (`5` for this release); `build` is a monotonic counter
across all branches. This cycle set the concrete per-build value to
`0.5.95` across `apps/desktop/package.json`,
`apps/desktop/src-tauri/tauri.conf.json`, and
`apps/desktop/src-tauri/Cargo.toml`; updated the `Codex ${buildVersion}`
build-label format's test fixtures to match; and committed the four-step
closure-process checklist at `docs/SD-22/release-closure-checklist.md`.

**Correction (post-closure):** Epic 7's criterion-26 cycle initially bumped
the tranche-base position (`0.5.95` → `0.6.0`), on the assumption that
closure always advances the tranche digit. That was wrong for this bundle:
`tranche/5` is still the active branch — the tranche digit only advances
when a new `tranche/N` branch is cut for the next bundle, not automatically
at a bundle's own closure. The bump was reverted; only the build position
incremented: `0.5.95` → `0.5.96`.
