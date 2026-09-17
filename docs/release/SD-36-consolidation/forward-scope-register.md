---
canonical: true
owner: operator
bundle_id: SD-36
date: 2026-09-15
---

# SD-36 Forward Scope Register

Planned work deferred from SD-36. Each row is a **capability deferral** (not a blocker) — scoped work that was out of scope for this bundle but should land in a successor.

---

## Deferrals recorded during SD-36

| ID | Work | Reason | Target bundle | Notes |
|---|---|---|---|---|
| FS-1 | Semantic dedups in pilot_compute (fable review PC5-3/PC6-1/PC2-3) | Design work, requires adversarial review. Deferred for correctness focus bundle. | SD-36.5 or later (correctness phase) | Logged in SD-36 retrospective; blocks some compute optimizations but does not block release. |
| FS-2 | SD-34 fable-review P1 correctness rows (C2.5) | Separate correctness review bundle. 8 open P1s from fable review (doc link: `docs/release/SD-34-corpus-sheet-completion/references/README.md`). | Correctness bundle (post-SD-36) | Independent scope; should land before Starfinder bundle. |
| FS-3 | rules_tables → data package (181,797 lines) | Starfinder data-driven architecture requires rules as JSON/YAML, not Rust. Burn-down tied to `.lst` citation de-duplication (D6). | Starfinder data-package bundle | 8,592 `.lst` citations (provenance strings in rules_tables) burned down when Rust data becomes external package. Work inventory already frozen; this unblocks that transition. |
| FS-4 | pf1e_dashboard_producer.py extraction (if incomplete during B7) | Extract doneness.py library (~150 lines) from the producer if the extraction is clean; else defer whole producer deletion. | Immediate post-SD-36 fix cycle or next bundle | Recorded in Epic B7 deferral decision if not complete. |
| FS-5 | CI oracle fetch for converter test (optional per D1 Phase 0b) | Sparse clone via `scripts/fetch-pcgen-oracle.sh` in CI test job so gate proves something in CI. Currently: gate skips without checkout. | Next CI infrastructure bundle | Orthogonal to SD-36 scope; improves test coverage but not required for ship. |
| FS-6 | GATE-03: widen `scripts/pcgen_residue_gate.py`'s `TOKEN_SYNTAX_PATTERNS` beyond its current 8 heads to the full PCGen token-head vocabulary `table.rs`'s `row_for_head` already enumerates | Explicit P3 in the SD-35 code review (`scratchpad/review/converter.md` #11); confirmed a real gap (`SPROP:`/`CHOOSE:`/`AUTO:`/`QUALIFY:`/`COST:`/`SPELLKNOWN:`/`NATURALATTACKS:`/`CSKILL:` uncovered) but no current live-code exploit (every uncovered-head hit today sits in a `#[cfg(test)]` fixture that also carries an already-covered pattern). | Next SD-N code-quality/gates pass | No change made this cycle per the SD-36 Epic E brief's own instruction ("GATE-03 (P3): record as a forward-scope row; no change"). |
| FS-7 | engine-P1-3 full fix: thread the source book through `HeldSeed`/`ChosenCharacterState` so `SheetRulePackage::find` can resolve a (kind,slug) collision by the character's own held book, not an alphabetical tie-break | `ChosenCharacterState`'s selection fields (`selected_feats` etc.) are bare compound-string slugs with zero book component, corpus-wide, across all of `pilot_compute` (88k+ lines) and its fixtures -- a schema migration (saved-character version bump, default `core_rulebook`), not a bounded fix. 32 real (kind,slug) collisions with differing converted values are reviewed and pinned as a regression gate in the interim (`tests/sheet_rule_book_collision_census.rs`). | Correctness bundle (post-SD-36), or its own scoped migration cycle | NEEDS HUMAN RULING logged in `codex-morning-log-2026-09-16.md`; retro deferral not filed (a scope decision, not a mistake) -- see commit `670b8546fd`. |
| FS-8 | ~~PC4-1: Warpriest Blessing chooser "recognized" contradiction~~ -- **RESOLVED, fix cycle 2**: `blessing_recognized` now also checks whether either generic pass actually pushed a real explanation for the player's own selection (`blessing_generically_grounded`, bracketing `explanations.len()` around both `push_generic_pool_group_selection_{magnitude,description_magnitude}` calls), not just Destruction/Strength by name. A Rune/Earth/etc. Blessing choice that resolves generically is now `recognized` (the non-blocking `unmodeled` note), never `unsupported`; a selection that resolves to NOTHING (an invented slug, a formula the interpreter refuses) correctly stays `unsupported` -- the exact per-choice success detection this row's own "correct fix" description called for. | Was: the review itself marked this COMPLEX/unresolved; the fix needed per-player-choice success detection through the generic pass's member-level resolution rather than a group-membership check alone. | N/A -- fixed | RED-confirmed test `warpriest_with_a_generically_grounded_blessing_is_recognized_not_unsupported` (`pilot_compute/mod.rs`); retro deferral `1789643677178-sd36-epic-e-30e143` superseded by the fix, `codex-morning-log-2026-09-16.md`'s NEEDS HUMAN RULING bullet answered by the code change rather than a ruling. |
| FS-9 | GATE-02 full roster widening: stratified cross-book PCGen oracle-parity roster + re-run (beyond the 29-character CRB-only roster) | Requires a real PCGen `BatchExporter` invocation per new character (new `.pcg` templates per target book's own class/race/feat/equipment naming) -- infrastructure work, not a doc/code-only change. **Fix cycle 2 update**: widened 29 -> 31 (`EXTRA_BOOK_CLASSES` in `scripts/oracle_harness/sheet_parity.py`: Alchemist L1 and Witch L1, both Advanced Player's Guide, each carrying `CAMPAIGN:Core Rulebook` + `CAMPAIGN:Advanced Player's Guide` per that book's own `PRECAMPAIGN:1,INCLUDES=Core Rulebook`), genuinely re-run end to end (roster -> PCGen `BatchExporter` export -> `sheet_rule_parity` -> compare). Result: lines compared 159->167, chassis compared 382->408, the same 8 pre-existing disagreements reproduce byte-for-byte (none new, none resolved -- the 2 new members introduce zero new disagreements). This is a real start, not the full ask: 31 books ship a `class` or `race_trait` directory today (`ls -d data/sheet_rules/*/{class,race_trait} 2>/dev/null`) and this cycle covers 2 (Core Rulebook, Advanced Player's Guide); equipment/feats exercising CONV-01..04 on a cross-book member were not added. Artifacts: `docs/release/SD-36-consolidation/artifacts/gate-02-oracle-parity-widening/{roster,exports,ours.json,sheet-parity.json}`. | Next SD-N oracle-harness infrastructure pass (remaining ~29 books) | Retro deferral `1789645756930-sd36-epic-e-aa019a` (still applies to the remaining scope; the 2-book start is not itself a repeat of the original deferral). |

---

## No deferrals for SD-36 blockers

Every acceptance criterion in `epic-breakdown.md` is scoped to this bundle. No work falls into the "open blocker" category (see `docs/governance/blocker-closure-doctrine.md`). The bundle is 100% scoped or explicitly deferred.

---

