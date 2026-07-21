# Release Notes: SD-22 Content-Source Ingest (APG + ACG + Bestiary 1) + DM Toolkit

## Summary

Tranche 5 lands content-source ingest for two Pathfinder 1e source books (Advanced Player's Guide, Advanced Class Guide) plus Bestiary 1 monster data, a new DM toolkit (encounter difficulty + party challenge rating), a code-side identifier-cleanup pass, and the closure-readiness infrastructure that gates this release. All content is transcribed directly from the real public PCGen `.lst` corpus, not fabricated — every class chassis and monster stat block cites its exact source file and line.

## User-Visible Changes

- Add Advanced Player's Guide (APG) support: 6 real base classes (Alchemist, Cavalier, Inquisitor, Oracle, Summoner, Witch) with BAB/save chassis, plus bootstrap spell and equipment tables, resolvable via `RuleSetId::Apg`.
- Add Advanced Class Guide (ACG) support: 10 real base classes (Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest) with BAB/save chassis, plus bootstrap spell and equipment tables, resolvable via `RuleSetId::Acg`.
- Add Bestiary 1 monster data: 8 monster-block subsets (41 monsters) spanning CR 1 through CR 3, resolvable via `RuleSetId::Bestiary1`.
- Add a DM toolkit: `Encounter::new` (encounter-difficulty rating) and `party_challenge_rating` (party CR), both grounded in the PF1 Core Rulebook's "Gamemastering" chapter and consuming real ingested monster data end-to-end.
- This release is engine/data-layer content — no desktop-app UI surfaces this content directly yet; it establishes the data these classes/monsters/toolkit functions will be wired into in a future tranche.

## Defects Fixed

- The original planning roster for APG incorrectly listed Gunslinger and Magus as APG classes; corrected to the real 6-class roster (they are Ultimate Combat / Ultimate Magic content) before any chassis work was fabricated against them.
- The original planning roster for ACG incorrectly listed "Alchemist (ACG-side)," which has no real record in the corpus; corrected to the real 10-class roster, adding Slayer (which does have a real record but was missing from the original plan).
- Bestiary 1's planning-doc illustrative monster samples were wrong for every subset attempted (duplicate names across subsets, names with no real standalone stat block, wrong CR band); each subset's roster was independently re-verified against the real corpus before landing.
- The build-version scheme's closure-epilogue step initially bumped the tranche-base version digit (`0.5.95` → `0.6.0`) on the mistaken assumption that a bundle's own closure always advances the tranche digit. Corrected: the tranche digit only advances when a new `tranche/N` branch is cut for the next bundle; this bundle's closure only incremented the build position (`0.5.95` → `0.5.96`).
- Two PF1 encounter-difficulty/party-CR canonical test-fixture values in the bundle's own planning docs were independently found and corrected against the real Core Rulebook rules (a "Hard" encounter case was actually "Deadly"; a "~3.5" party-CR case was actually exactly "3.0" — APL always rounds to a whole number).

## Operational Notes

- Corpus source: real public PCGen `.lst` data (`advanced_players_guide/`, `advanced_class_guide/`, `bestiary/`), not synthetic or hand-invented content. Every landed class/monster cites its exact source file and line in code doc comments and in the per-cycle artifacts under `docs/release/SD-22/artifacts/`.
- A new parser (`src/pcgen_import/lst_parser/monster_stat_block.rs`) was added to read Bestiary 1's bare tab-delimited monster records, which the pre-existing `RACE:`/`ABILITY:`-prefixed parser didn't cover.
- Bestiary 1 landed 8 of a stated 8-12 target subset range (41 monsters) — the acceptance floor, not the ceiling; more subsets can land in a follow-on tranche without any structural change.
- Build version scheme: `<major>.<tranche-base>.<build>`, currently `0.5.96` on `tranche/5`.
- Full per-cycle audit trail (RED/GREEN test evidence, kanban card IDs, source citations) lives in `docs/release/SD-22/receipts.md` and `docs/release/SD-22/artifacts/`.

## Verification Evidence

- `cargo test --locked`: 428 `test result: ok` blocks, 0 failures, across every suite (sibling-preservation held for every prior SD's tests throughout the loop).
- `cargo clippy --locked --tests -- -D warnings`: clean.
- `npm test` (apps/desktop): 48/48 test files passed.
- Closure-readiness eval (Epic 9): full artifact-evidence survey across all 30 prior acceptance criteria, 4 mechanical shortfalls self-healed, 2 judgment calls deferred to `docs/release/SD-22/risks-and-open-questions.md`. Report: `docs/release/SD-22/closure-readiness-report.md`.
- Closure PR: [electricm0nk/codex#325](https://github.com/electricm0nk/codex/pull/325), `tranche/5 → develop`, merged as commit `f5e2b62`.
- Kanban board `codex-tranche-5`: every cycle minted a card recording its own RED/GREEN evidence and corpus citation.

## Known Issues

- Bestiary 1 has 8 of the stated 8-12 target subsets (41 of an eventual larger monster roster) — a deliberate floor, not a defect, deferred as an open judgment call rather than pushed to the ceiling this tranche.
- Epic 1's identifier-audit grep pattern flags the bundle's own approved `tests/sd22_*.rs` test-file names as doc-comment citations in later cycles' code comments — zero real identifier-discipline leaks, but the criterion's exception clause doesn't formally cover this shape (deferred judgment call, not a defect).
- None of this tranche's content (APG/ACG classes, Bestiary 1 monsters, DM toolkit) is wired into any desktop-app UI surface yet — it is available at the `src/rules_core/` engine layer only.

## Update Eligibility

- No changes to the update/install mechanism in this tranche — eligibility follows the same Linux AppImage mechanism established in SD-16 (`docs/release/SD-16/release-notes.md`).
- This release introduces no update-critical UI or install-path changes; existing eligible installs Check/Install through the same governed channel-index flow.
