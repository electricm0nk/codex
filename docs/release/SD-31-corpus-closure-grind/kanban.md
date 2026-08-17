---
canonical: true
owner: god-emporer
status: planning-ready (SD-32 absorbed, epics re-sequenced, operator ruling 2026-08-15)
date: 2026-08-15
canonical_branch: tranche/11
---

# SD-31 — Local-file Work Queue

Same local-file dispatch convention as `SD-30-class-feature-archetype-bundle/kanban.md` (Hermes board
retired, operator directive 2026-08-01).

**Origin.** Cards `epic-3-measurement` through `epic-8-cloud-fanout` were moved from SD-30's
`epic-4-measurement`, `epic-5-mechanism`, `epic-6-chassis-sweep`, `epic-10-ingest-lanes`,
`epic-11-book-onboarding` and `epic-14-cloud-fanout` rows (operator ruling 2026-08-14, `SD-30
decisions.md §51`). Cards `epic-1-race-chassis` and `epic-2-verdict-paths` were moved from
`SD-32-engine-capability-builds/kanban.md`, which was absorbed and deleted by operator ruling
2026-08-15 (`decisions.md §2`). `epic-0-reachability-audit` and `epic-9-closure` are new in that same
ruling. No card was `IN-FLIGHT` at either move — verified against both source boards immediately
before each.

**Claim-priority order is the table order, top-down**, and it is not the old order: capability
(Epics 1-2) now precedes the lanes that consume it, because 8,524 units — 22.1 % of the board — cannot
reach `done` without it (`decisions.md §2`).

## Status legend

Identical to `SD-30-class-feature-archetype-bundle/kanban.md`'s legend — `READY`, `READY (gated on
...)`, `IN-FLIGHT`, `BLOCKED`, `COMPLETE`. See that file for the full definitions; not reproduced here
to avoid drift between two copies of the same legend text.

## Cards

| ID | Status | Epic | Cycle-type | Claimed-by | Claimed-at | Cycle-id |
|----|--------|------|-----------|------------|------------|----------|
| `epic-0-reachability-audit` | COMPLETE | **Order 1 — Reachability Audit (standing gate)** | `scripts/reachability_audit.py` built, self-tested (fabricated dead-end proven caught, 11 cases green), wired into `verify.sh` as `reachability-audit`/`reachability-audit-selftest`; baseline run committed at `artifacts/SD31-E0-F1-001-baseline.{md,json,txt}` — reachable ceiling 94.53 %, every dead-end/known-gap owned by Epic 1 or Epic 2, no SER proposal needed. **Standing gate per `decisions.md §4`: re-invoked at every epic closure, not a one-shot card** — this row records the initial build+baseline cycle only. **Re-invoked 2026-08-15 at the `SD31-W1-INTEGRATE-001` integrated tip (`sd31/e2-groundtruth` merged onto `tranche/11`): reachable ceiling unchanged at 94.53 % (36412/38521), same 9 `ambiguous|*` dead-end cells, all still owned by Epic 2, `AUDIT_EXIT=0` — `artifacts/SD31-W1-INTEGRATE-001-audit.json`. Also this cycle: fixed the audit-docstring over-claim CONFIRMED by adversarial review (narrowed to the wiring_class axis; status-axis gap logged `OPEN-ISSUES.md` row 6, non-blocking) — acceptance genuinely still met, COMPLETE stands.** **Re-invoked again 2026-08-15 at the `SD31-W3-INTEGRATE-001` tip (5 worktree branches merged: race, seam, equipment, spell/monster_ability, ambiguous): reachable ceiling **98.95% (38117/38521)**, +0.01pp vs wave 2, same 9 `ambiguous|*` dead-end cells, all still Epic-2-owned, `AUDIT_EXIT=0` — `artifacts/SD31-W3-INTEGRATE-001-audit.json`. COMPLETE stands.** **Re-invoked again 2026-08-16 at the `SD31-W4-INTEGRATE-001` tip (6 branches merged: class_feature already on `tranche/11`, PI repair, sweep-attrib/race, monster-widen, spell-reach, equipment_gap): reachable ceiling **98.95% (38117/38521)**, unchanged from wave 3, same 9 `ambiguous|*` dead-end cells, all still Epic-2-owned, `AUDIT_EXIT=0` — `artifacts/SD31-W4-INTEGRATE-001-audit.json`. COMPLETE stands.** **Re-invoked again 2026-08-16 at the `SD31-W5-INTEGRATE-001` tip (5 wave-5 branches merged: book-attrib, class wiring, spell lists, monster_ability, equipment residual, plus this cycle's own PI-placeholder/gathlain/engine_book_for/clippy fixes): reachable ceiling **98.95% (38117/38521)**, unchanged from wave 4, same 9 `ambiguous|*` dead-end cells, all still Epic-2-owned, `AUDIT_EXIT=0` — `artifacts/SD31-W5-INTEGRATE-001-audit.json`. Board headline at this tip (producer's own `doneness_verdict`): 38,521 units, done 7,340 (19.05%) — down from wave 4's 7,603 (19.74%), net of SD31-D7-PROSE-001's anti-gaming description-completeness fix (-1,060 demotions corpus-wide, only partly offset by +146 race_trait / +102 monster_ability / this wave's other real gains) landing for the first time at a fully-merged tip. Trap report unchanged at 1,191 wiring-class-mismatch (row 65's baseline exactly). COMPLETE stands.** **Re-invoked again 2026-08-16 at the `SD31-W6-INTEGRATE-001` tip (5 wave-6 branches merged: class wiring, attribution, spell+monster, equipment repair, companion+feat+monster_ability, plus this integration cycle's own confirmed-finding fixes — companion PI screening, the static-stamp book/source_book join, the monster_ability unresolved-description-argument refusal, the equipment %CHOICE leak refusal, and a self-caught %% false-positive correction in that same check): reachable ceiling **98.95% (38117/38521)**, unchanged from wave 5, same 9 `ambiguous|*` dead-end cells, all still Epic-2-owned, `AUDIT_EXIT=0` — `artifacts/SD31-W6-INTEGRATE-001-audit.json`. Board headline at this tip: 38,521 units, done **9,488 (24.63%)** — up from wave 5's 7,340 (19.05%); of the +2,148, **257 units are recovered from wave 5's demotion** and **2,688 are genuinely new** real paths, **0 regressed off the pre-demotion baseline**. Trap report unchanged at 1,191 wiring-class-mismatch. COMPLETE stands.** | sd31-e0-audit / sd31-w1-integrate / sd31-w3-integrate / sd31-w4-integrate / sd31-w5-integrate / sd31-w6-integrate | 2026-08-16 | `SD31-E0-F1-001` / `SD31-W1-INTEGRATE-001` / `SD31-W3-INTEGRATE-001` / `SD31-W4-INTEGRATE-001` / `SD31-W5-INTEGRATE-001` / `SD31-W6-INTEGRATE-001` |
| `epic-1-race-chassis` | READY (first batch landed, more races remain) | Race Chassis, 100 % mandate | chassis design → per-race (or batch) build with DoD-8 on-screen verification → ceiling release to `epic-6` per race batch. **SD31-E1-F1-001 landed 2026-08-15: `IN_SCOPE_RACES` widened 18 → 24 (`ingest_races.rs`/`ingest_race_traits.rs`), "Bestiary 2 batch" = Fetchling, Grippli, Ifrit, Oread, Sylph, Undine (Dhampir excluded — `core_essentials/races/dhampir/` carries a heritage/subrace shape this batch's mechanism does not model; deferred to a follow-on batch, not stubbed). DoD-8 on-screen verification done against the real running app (screenshots `artifacts/SD31-E1-F1-001/dod8-*.png`): a Fetchling character sheet shows real `+2 DEX, -2 WIS, +2 CHA`, `Medium` size, `Normal Speed 30 ft.`, applied to the CALCULATED ability scores. Corrected the inherited chassis-blind figure "~2,894" to the re-derived **2,689** (`evidence=="race_trait_race_not_modelled"`); this batch moves it to **2,576** (-113), `race_trait` `done` 266 → 316 (+50, doneness-ladder measurement only, nothing committed to `docs/work-inventory.json`). Skinwalker (Bestiary 5, 84 chassis-blind rows, the single largest remaining gap) shares Dhampir's heritage/subrace shape and is next in line for a batch that extends the ingest mechanism to model it. Full receipt: `progress.md` "SD31-E1-F1-001".** **Root-cause correction 2026-08-15 (`SD31-W2-INTEGRATE-001`, Finding 8):** the accepted-shortfall register (`reach_gate.rs` `UNREACHED_RECORD_FINDINGS`) had the wrong root cause for the 3 unreached ISR `Mostly Human ~ Ifrit/Sylph/Undine ~ Languages` records — PCGen ships a symmetric granter for all four races via `Geneiekin ~ Mostly Human.MOD` rows, so the gap is project-side, not an upstream absence. Corrected; numeric pin unchanged (the 3 records genuinely do not reach today). New Epic 1 follow-on named at `OPEN-ISSUES.md` row 18. **SD31-E6-F4-001 landed 2026-08-15** (`sd31-race-lane`, own worktree): Skinwalker (Bestiary 5) added to `IN_SCOPE_RACES` (18→24→25) — chassis + 9 standard-tier trait rows only, same rigor as prior batches (`RACE_SIZES` gained a real `Medium` entry from Skinwalker's own `~ Size` row's `TEMPLATE:SIZE_M`). Wired through every consumer surface: `race_catalog.rs` (`RACE_CORPUS_BOOKS`/`RACE_CATALOG_BOOKS`/`BOOK_B5`/`book_code`), `corpus_ingest_diagnostic.rs` (`diagnostic_book_id("B5")`), `reach_gate.rs` (`("bestiary_5","races")`/`("bestiary_5","race_traits")`). Measured delta (local, uncommitted per wave rule): `race` `done` unchanged at **0/103** — genuinely blocked, not by missing chassis, but by a `corpus_literal_sweep` book-attribution bug traced one unit deep and logged `OPEN-ISSUES.md` row 22 (out of this card's file territory to fix); `race_trait` `done` **478→484 (+6)**. **Skinwalker's heritage-shaped alternates (65 of its 86 chassis-blind rows) deliberately NOT ingested this batch** — `ingest_race_traits.rs`'s `subrace_grants()` mechanism (built for Aasimar/Tiefling) cannot be reused as-is because Skinwalker has no `_abilities_globalvar_subrace.lst` file; a genuinely new mechanism, not a config widening, deferred rather than stubbed. Also investigated and reverted an `advanced_players_guide` `race_trait` BookSource addition after confirming (via `race_resolver`'s own test suite) that 49 of its 50 rows duplicate already-ingested `advanced_race_guide` content — a pre-existing, already-correctly-handled measurement artifact (`OPEN-ISSUES.md` row 23). Full receipt: `progress.md` "SD31-E6-F4-001". **Correction, `SD31-W3-INTEGRATE-001` (integration, 2026-08-15): `bestiary_5/LICENSE.json`'s note falsely claimed the Skinwalker records were screened by the declared-PI reader; `ingest_races.rs` never calls it (only `ingest_race_traits.rs`, a different binary, does). Corrected in place — no PI leaked today, but the deferred Skinwalker heritage batch genuinely carries `DESCISPI:YES` rows and would hit this same gap (`OPEN-ISSUES.md` row 39, blocking that batch until wired).** **`SD31-PI-REPAIR-001` landed 2026-08-16** (own worktree, branch `sd31/pi-fix`, pushed, not yet merged): row 39 FIXED — `ingest_races.rs`'s chassis and trait writers now both call `pi_screening::declared_product_identity`, dropping a `NAMEISPI:YES` row (cascading to its traits) and redacting a `DESCISPI:YES` description via the real `pi_screening::classify_optional_field_declared` call, closing the gap that would have hit the deferred Skinwalker heritage batch. Re-ingested for real (25 races/241 traits, 0 drops/redactions today — this batch's own rows carry no declarations), `bestiary_5/LICENSE.json`'s claim corrected to a structured, machine-checked one. Full receipt: `progress.md` "SD31-PI-REPAIR-001".** **`SD31-E6-F4-007` landed 2026-08-17 (merged `SD31-W13-INTEGRATE-001`): Changeling + Samsaran chassis (18 new race_trait records + 2 race records), closing `arg_races.lst`'s full 37-row playable-race roster -- no unseamed race remains in-corpus without a chassis attempt. `race_trait` `done` +7 (board-verified: `changeling_{claws,size,type}` + `samsaran_{languages,samsaran_magic,size,type}`, all under `bestiary_4`). Adversarial review REFUTED the central suspicion (that this re-credited wave 12's 251-unit demotion under a new name) with hard mechanical evidence: `race_ids_with_a_magnitude_consumer()` and its 13-race pin are byte-unchanged; all 18 new records carry the wiring_class the pre-existing inventory already assigned before ingest (0/18 mismatches) -- the +7 is forced by an untouched classifier, not chosen. `race` still stalled at 7/103 (unchanged, same five-wave-plus blocker, `OPEN-ISSUES.md` row 207).** | sd31-e1-chassis / sd31-race-lane / sd31-w3-integrate / sd31-pi-fix / sd31-racetrait6 / sd31-w13-integrate | 2026-08-15, 2026-08-17 | `SD31-E1-F1-001` / `SD31-E6-F4-001` / `SD31-W3-INTEGRATE-001` / `SD31-PI-REPAIR-001` / `SD31-E6-F4-007` / `SD31-W13-INTEGRATE-001` |
| `epic-2-verdict-paths` | READY | Verdict-Path Capability, 100 % mandate | hand-labelled ground-truth sample (gate) → classifier build/accept or close-at-F1 → **`ambiguous` dead-end closed or registered**. **SD31-E2-F1 landed 2026-08-15 (`sd31/e2-groundtruth`, merged onto `tranche/11` this cycle): 150-unit hand-labelled sample committed, `artifacts/SD31-E2-F1-ground-truth-sample-v1.json` + methodology note. Card is NOT closable at F1 as-is — adversarial review CONFIRMED 105 of the 150 labels (including all 40 of the `display_grounded_target` population AT-31-010 binds) carry no record-specific token evidence; the 95.5 %/71.3 % headline agreement figures are WITHDRAWN and Decision 1(e) item 4's "close at F1" path is explicitly barred until re-labelled (`OPEN-ISSUES.md` row 3, `BLOCKER`, still open — not this cycle's scope). F1's evidenced Findings A/B/C stood and are now LANDED as production fixes. **SD31-E2-F2-001-wiringfix landed 2026-08-15** (`d07d41b5c`/`e219fed2f`/`b1139db41`, `tranche/11`): Finding A (`no_corpus_line` single-level path-join bug) and Findings B/C (`BONUS:STAT` selector + `CR:`/`DR:` slash false positives) fixed in `wiring_class.rs`, 18 new tests, movement reported both directions, validated against the F1 sample's 45 genuinely-evidenced (non-boilerplate) units: 40/45 agree, 5 documented out-of-scope disagreements (`OPEN-ISSUES.md` row 9). Guarded regen (local, uncommitted per wave rule) measured: `ambiguous` population **2,109 → 409** (−80.6 %), `no_corpus_line` **1,707 → 0** (fully eliminated), reachable ceiling **94.53 % → 98.94 %** (`+1,700` units, `progress.md` `SD31-E2-F2-001-wiringfix` receipt). Card stays READY: F1's row-3 blocker (105 unlabelled units) is still open, and F3 (`ambiguous` dead-end closed to `done` or registered to the Structural Exclusion Register — 409 units remain) has not been attempted.** **SD31-E2-F1-002-relabel landed 2026-08-15** (`worktree-wf_49e8e5da-ca5-2`, merged onto `tranche/11` this integration cycle): `OPEN-ISSUES.md` row 3's `BLOCKER` is RESOLVED — all 105 canned units re-labelled from the real corpus record (103 confirm the engine's verdict with genuine evidence, 2 disagree — new Findings D/E); 0 canned strings remain (`scripts/ground_truth_evidence_guard.py` proves it). Sample widened 150→185 units (row 5: 45/31-thin cells → 48/29-thin cells — improved, not fully closed). Row 4 resolved for future draws only (`scripts/sample_ground_truth_units.py` committed, seeded, reproducible); the original 150-unit v1 draw stays permanently non-reproducible. New guard `scripts/ground_truth_evidence_guard.py` (+ self-test) catches this exact defect shape in future; found a smaller residual gap in the untouched 45 (`OPEN-ISSUES.md` row 14, not fixed — barred from re-opening those 45) and is therefore not yet wired into `verify.sh`'s default stages (row 15). Findings D/E/F (new, `OPEN-ISSUES.md` row 16) extend A/B/C. **Adversarial review Finding 9 (`SD31-W2-INTEGRATE-001`) CONFIRMED the sample's `engine_wiring_class` columns were captured before the wiringfix landed and are therefore stale against the merged tip — re-derivation against the merged tip is this integration cycle's own follow-on work, tracked below.** **`SD31-W2-INTEGRATE-001` landed 2026-08-15**: fixed the D3 wiringfix's own 55-unit over-shoot (Finding 1 — `DR:`/`BONUS:STAT` rows whose magnitude is a NAMED VARIABLE, not a literal, were wrongly falling to `static:literal_magnitudes_only`; two narrow TDD'd fixes in `has_arith_scoped`/`has_scalar_or_arith_for_token`, 6 new tests). Re-derived `engine_wiring_class` for all 185 sample units against the merged, D4-fixed tip (Finding 9's remedy): **167/185 agree** (was an unmeasured/stale 40/45 before), all 18 disagreements attributed to already-documented out-of-scope gaps (`OPEN-ISSUES.md` rows 9/16, the test's own base-row-only scope limit, or a labeller judgement call). Corpus-wide guarded regen at the merged tip (committed, per the wave rule's one sanctioned regen): `ambiguous` population **409** (unchanged — D4 moves units between `derived`/`static`, not into/out of `ambiguous`), reachable ceiling **98.94%** (unchanged from the wiringfix's own measurement), board `done` **5,837 → 6,076** (+239, driven mostly by Epic 1's race-chassis reachability landing this wave, not by Epic 2). Card stays READY: F3 (`ambiguous` dead-end closed to `done` or registered to the Structural Exclusion Register — 409 units still remain, unchanged) has still not been attempted; F2 (classifier build) still not dispatched. **`SD31-E2-F3-001` landed 2026-08-15** (own worktree, branch `worktree-wf_e4e73f9a-9af-6`, pushed, not yet merged): re-derived the 409-unit `ambiguous` bucket as 100% `prose_scaling_phrase` (297) / `prose_ability_scaling` (112); the F1 sample's own 17 hand-labelled units of these two reasons agree 17/17 (`ambiguous` confirmed genuinely correct, not a bug — GE-01's own documented 5th-class content shape). Fixed 3 of the 6 named already-evidenced gaps: `SPELLS:` field scanning (Finding D), case-insensitive `classlevel(...)`, and `+`-then-`(` arithmetic (row 9(a)/(c)) — `ambiguous` **409 → 404** (all 5 confirmed off-ambiguous, zero regressions the other way), reachable ceiling **98.94% → 98.95%** (+5 units), ground-truth agreement (base-row-only integration test) **167/185 → 170/185**. Board `done` moved the UNFLATTERING way, **6076 → 6069 (-7)**, both directions reported: 9 lost `done` (a real over-claim removed — e.g. a feat with SEVEN CHA-scaling spell-like grants each behind a real guard, previously falsely `display`+`text-complete`=`done`), 2 gained `done` (a real under-claim corrected). **Investigated and REJECTED Finding E (`PLUS:` fields)** after measuring its true corpus-wide blast radius (264 units, not the 2 named examples — an unresolved "guard scoped to record eligibility vs. the magnitude" design question, `OPEN-ISSUES.md` row 35, renumbered from this cycle's own row 22 at integration); Finding F (`ASPECT:`) not attempted for the same reason. AT-31-010 (widened `display`+`grounded` acceptance, re-derived population **1,363**, not the cited ~1,243) run against its own 40-unit ground-truth oversample: 39/40 confirm correct, the 1 exception was Finding D and is now fixed — bullet satisfied. Proposed (not signed) a Structural Exclusion Register entry for the remaining 404-unit population (`OPEN-ISSUES.md` row 36, renumbered from this cycle's own row 23 at integration, `RULING-NEEDED`), with this cycle's own recommendation that exclusion is probably the wrong remedy (a new `ambiguous` done-bar in `doneness_verdict`, a file this card does not own, is the likelier real fix). Card stays READY pending integration merge, the operator's SER/done-bar ruling, and the still-open `bare_var_judgement_call` design question (row 9(b), 3 units, deliberately left unresolved).  **`SD31-W3-INTEGRATE-001` landed 2026-08-15**: fixed the D6 `SPELLS:` scan's own CONFIRMED slash-in-spell-name false positive (`Open/Close`, `Blindness/Deafness` misread `derived`) via a new `has_arith_no_slash` helper, TDD'd, 0 regressions (1815/1815 lib tests green) — `OPEN-ISSUES.md` row 45. Corrected the cycle's own unreproducing "17 units, 17/17" figure to the true **16/16** (`OPEN-ISSUES.md` row 5). Merged onto `tranche/11`; corpus-wide guarded regen at the fixed tip: `ambiguous` population **404** (unchanged — the D7 fix removes a false positive, it does not move the `ambiguous` boundary), reachable ceiling **98.95%**. Card stays READY: F3 (`ambiguous` dead-end closed to `done` or registered — 404 units remain) still open, SER proposal (`OPEN-ISSUES.md` row 36) still awaiting operator sign-off.** | sd31-e2-groundtruth / sd31-w1-integrate / sd31-e2-wiringfix / sd31-e2-relabel / sd31-w2-integrate / worktree-wf_e4e73f9a-9af-6 / sd31-w3-integrate | 2026-08-15 | `SD31-E2-F1-001` / `SD31-E2-F2-001-wiringfix` / `SD31-E2-F1-002-relabel` / `SD31-W2-INTEGRATE-001` / `SD31-E2-F3-001` / `SD31-W3-INTEGRATE-001` |
| `epic-3-measurement` | READY (per-class; F4 gated on `epic-2`) | Per-Class Archetype Measurement | class inventory + per-class hand-verification + chooser-primitive design + `unknown`-bucket characterization (F4). **SD31-E3-F1-001 landed 2026-08-15** (primary checkout, direct to `tranche/11`): F1 — direct enumeration of every `kind:class` `Base.PC`/`Base.Psionic.PC` record across the 23 in-scope books (`docs/work-inventory.json`'s own class-identity join, not a book-name proxy), cross-referenced against SD-28 `decisions.md §64`'s 28-class list, names a **24-class remainder**: the 6 Occult Adventures classes (Occultist, Spiritualist, Medium, Mesmerist, Kineticist, Psychic), Slayer (ACG — has real base-chassis wiring already, just never entered the archetype-slot measurement), Antipaladin/Ninja/Samurai (0 archetype-table content in the 23-book corpus — trivially cleared), Gunslinger/Vigilante/Magus/Shifter, and the 10 `ultimate_psionics` base classes (Aegis, Cryptic, Dread, Marksman, Psion, Psychic Warrior, Soulknife, Tactician, Vitalist, Wilder). Mythic Adventures is confirmed absent from the 23-book `class_feature` roster (no path-tier features to measure). F2 — all 24 measured by direct `pilot_compute.rs` grep evidence, no proxy, never blended: **23 of 24 show 0 wired-able** (the entire OA class family plus every non-OA newcomer except Slayer has zero base-chassis presence in the engine today); **Slayer measures 4/7** (collapsed from 10 raw archetype slots) — a genuinely already-measurable class Decision 64's original 25-class pass missed. F3 — chooser-interaction primitive DESIGNED (not built; Design B: reuse `archetype_claims_slot` verbatim for tier-availability, add one new `chooser_option_selected` primitive for per-option grounding, rejecting a unified-abstraction alternative — full tradeoffs in `artifacts/SD31-E3-F3-001-chooser-primitive-design.md`), then Oracle/Arcanist/Sorcerer re-measured by the same no-proxy standard: **Oracle 5/10 mysteries** (6 tier-1 revelations — corrects Decision 64's stale "5 revelations" figure, retro correction emitted), **Arcanist 1/46 exploits**, **Sorcerer 2/10 bloodlines** (book-scoped floor; ≥31 known corpus-wide across 5 of 23 books checked). All 24 supersession-shape classes are CLEARED-FOR-EPIC-4 (a produced figure, even 0/N, is a clearance — Epic 4 gates on measurement existing, not on a favorable ratio). Full per-class table with every evidence command: `artifacts/SD31-E3-F1-001-clearance-table.json`. Card stays READY, not COMPLETE: F4 remains hard-gated on `epic-2-verdict-paths` COMPLETE (still not the case this wave) and is explicitly out of this cycle's protocol. | sd31-e3-measure | 2026-08-15 | `SD31-E3-F1-001` |
| `epic-4-mechanism` | READY (per-class, gated on `epic-3` clearing the target class) | Archetype Mechanism | supersession-shape wiring per cleared class; chooser-shape wiring once `epic-3`-F3 lands | **`SD31-E4-F1-001` landed 2026-08-16 (merged `SD31-W5-INTEGRATE-001`): real Slayer archetype-supersession if-let/else wiring for 3 ACG archetypes (Bounty Hunter, Deliverer, Stygian Slayer), 15 records transcribed verbatim, all reach `build_pilot_headless_receipt`. Shortfall named honestly by the cycle itself: the wiring lands on `wiring_class=display`+`status=grounded`, which `doneness_verdict` caps at `held`, not `done` (rows 78-80/`SD31-E4-F1-001` name the two structural gaps — an id-naming mismatch blocking most other Slayer features, and `grounded` being the wrong terminal status word for `display`'s done-bar even with a real render path); the supersession branch itself has no player-facing archetype-selection surface yet (`reach_gate.rs` `OPEN_FINDINGS`, still the real remedy). Adversarial review CLEAN on gaming/PI; one overclaim fixed by the integration cycle (OPEN-ISSUES row 88, row 70's "DoD-8-proven" text corrected — no screenshot exists, and no `done` credit rests on the gap since the unit ships `held`).** **`SD31-E4-F1-002` landed 2026-08-16 (merged `SD31-W6-INTEGRATE-001`): real Gunslinger base chassis + supersession wiring (Grit, Nimble, Gun Training, Gunslinger Initiative; Pistolero + Mysterious Stranger archetypes), 11 new/updated tests, all reach `build_pilot_headless_receipt`. Found and reported a THIRD `class_feature` structural blocker (`OPEN-ISSUES.md` rows 96/97, renumbered from the branch's own 94/95): `v06_work_inventory.rs`'s `modelled_class_books()` hardcodes only CRB/APG/ACG, so Gunslinger's records never reach the id-suffix check row 78 already found broken — the registry gap short-circuits before that. DoD-8 confirmed genuinely blocked one level earlier than the archetype-picker gap: no `CLASS_OPTIONS` entry exists for Gunslinger at all, so no Gunslinger character can be created through the app's own form. Adversarial review: NOT GAMED, PI CLEAN. Board `done` unchanged by this card alone (0/0, predicted and confirmed) — the wiring lands on the same blocked cells row 78/96/Decision-7 name.** **`SD31-W9-INTEGRATE-001` landed 2026-08-17** (merged all 5 wave-9 worktree branches onto `tranche/11`, primary checkout): merged the `pilot_compute.rs` -> per-class-module split (`class_slayer.rs`/`class_ultimate_combat.rs`, `sd31/e4-classsplit-wire5`) plus its owed universal-size-bonus record-level provability wiring -- proven a pure code move at the byte level (one 15-line plumbing insertion, two contiguous pure-relocation deletions, 67 single-line path-depth bumps, zero body edits), confirmed zero board-verdict movement attributable to the move itself before merging anything else on top of it. Found and fixed a pre-existing `decisions.md §10` AMENDMENT gap in the class_feature explanation-id matcher (present since wave 8, not introduced by any wave-9 lane): 16 archetype/Unchained-variant units and `Bloodrager ~ Raging` were credited off a base class's or a NEGATION explanation; `class_feature` `done` corrected **82 -> 73** as the honest consequence (`OPEN-ISSUES.md` row 164). Card stays READY -- the 16 demoted units are owed real per-variant wiring in `pilot_compute.rs` under §8, and lane 1 now has more than one class's worth of module space to work in per wave without conflicting on one 3.5 MB file.** | sd31/e4-classwire / sd31-w5-integrate / sd31/e4-classwire2 / sd31-w6-integrate / sd31/e4-classsplit-wire5 / sd31-w9-integrate | 2026-08-16, 2026-08-17 | `SD31-E4-F1-001` / `SD31-W5-INTEGRATE-001` / `SD31-E4-F1-002` / `SD31-W6-INTEGRATE-001` / `SD31-E4-F1-005` / `SD31-W9-INTEGRATE-001` / `SD31-E4-F2-002` (wave 11, 2026-08-17) |
| `epic-5-chassis-sweep` | READY (per-class, gated on `epic-3` + `epic-4` for the target class; F3 additionally gated on `epic-2` and `epic-3`-F4) | Per-Class Chassis Sweep | per-class `class_feature` ingest across the 23 in-scope books, reach-gate claim per record; **F4 (added 2026-08-15) — the 36 `deferred-with-reason` units, each with a real path or a proposed register entry**. **`SD31-E5-F1-001` landed 2026-08-16: 12,431 `class_feature` corpus records written across 21 books (PI-screened both contracts); traced ONE unit end to end and found a corpus dump alone cannot manufacture `grounded` for this kind — `class_feature`'s classifier reads only the engine's own compute sweep, never `data/corpus/**/*.json`. `done` +14 (the `(static\|derived, grounded)` literal-verified population). Full receipt: `progress.md` `SD31-E5-F1-001`.** **`SD31-E5-F1-002` landed 2026-08-16 (this row's own card): answered the package's central question — quantified all 11,404 `not-started` `class_feature` units by exact cause (option-pool 4,520 / unmodelled-class 2,152 / owner-matched-no-id 3,803 / no-compiled-rule-set 929) and characterized the 3,917 `unmeasurable` population, artifact `artifacts/SD31-E5-F1-002-class-feature-not-started-breakdown.md`. Took the in-territory share: `modelled_class_books()` now registers `UcClassId` (Gunslinger/Ninja/Samurai, closing `OPEN-ISSUES.md` rows 96/118) and a `decisions.md §10`-guarded known-magnitude-suffix fallback closes row 78's naming mismatch for its safe 31-unit subset. Guarded regen: **class_feature `done` 82→105 (+23), `held` 35→56 (+21)**, board `done` 10,759→10,782 (27.9302%→27.9899%), zero regressions anywhere, 45/45 units individually diffed. DoD-8: real Dwarf Slayer 1 character, Actions tab, Track/Studied Target/Sneak Attack/Trap Sense/Trapfinding/Weapon-and-Armor-Proficiency all rendering live (`artifacts/SD31-E5-F1-002/class-feature-slayer-track-actions-tab.png`). Sized the single largest remaining lever precisely: 7,965 of 8,437 option-pool-shaped units (94.4%) touch no chooser primitive at all, spanning 1,847 distinct pool names — Epic 4-F2's job, not this card's. Full receipt: `progress.md` `SD31-E5-F1-002`.** **`SD31-E5-F1-004` (wave 11, 2026-08-17) — `CLASS_FEATURE_POOLS` pool-name matching (row 168) and `slug()` apostrophe handling (row 181) both landed for real, board-verified: +38 units guarded-regen-measured, then adversarial review CONFIRMED 20 of them (Shaman Wandering Spirit / Secondary Shaman Wandering Spirit) were credited on a different record's grounding path (`OPEN-ISSUES.md` row 186/195) — fixed in the same wave with a third same-class-different-slot matcher guard plus a permanent regression test, net **+18** genuine `class_feature` credits survive (10 primary Shaman Spirit, 3 Witch Hex, 2 Sorcerer Bloodline, 3 apostrophe joins). See `progress.md`'s `SD31-E5-F1-004` and `SD31-W11-INTEGRATE-001` receipts.** **`SD31-E6-F11-003` landed 2026-08-17 (merged `SD31-W13-INTEGRATE-001`): built the missing `derived_evaluator_fixture_check` seam for `kind=class_feature`'s per-level scaling `BONUS:VAR` formulas -- the mandate's own named highest-leverage gap ('wave 12 wired a real pool... it still could not reach done... no evaluator seam exists for that formula shape'), now closed for 8 units (a 9th, `ranger_favored_terrain`, was withdrawn by the integration cycle -- fabricated-done, its grounded evidence and its fixtured token described two DIFFERENT quantities on the same record). `class_feature` `done` +8, board-verified: `rage_power_superstition`, `rogue_trap_sense`, `paladin_channel_positive_energy`, `slayer_trapfinding`, `slayer_sneak_attack`, `slayer_stalker`, `bloodrager_damage_reduction`, `ninja_no_trace`. Corpus-wide the same shape matches **265 records** (`OPEN-ISSUES.md` row 225 corrects the card's own 23-record census), most still uncovered -- real follow-on for the next class_feature cycle. Separately, `SD31-E4-F2-004` (Unchained Barbarian's own Rage Power chooser + a PU-wide roster-id false-grounding audit) demoted 15 previously-`done` class_feature units whose only grounding evidence was the generic PU roster id, not a real per-feature magnitude -- correct, though the integration cycle renamed the evidence string (it was factually false for 3 of the 24 units, which DO have real dedicated magnitude functions the matcher's suffix table just doesn't cover yet). Net class_feature movement this wave: +8 -15 = -7 (plus +7 race_trait from epic-1, net board delta 0 -- see progress.md SD31-W13-INTEGRATE-001 for the full reconciliation ledger).** | sd31-pool-match / sd31-w11-integrate / sd31-fixture-seam / sd31-cf-pools / sd31-w13-integrate | 2026-08-17 | `SD31-E5-F1-001` / `SD31-E5-F1-002` / `SD31-E5-F1-004` / `SD31-W11-INTEGRATE-001` / `SD31-E6-F11-003` / `SD31-E4-F2-004` / `SD31-W13-INTEGRATE-001` |
| `epic-6-ingest-lanes` | READY for F1/F2/F5/F6/F7/F8/F9/F10/F11; **F3 and F4 gated on `epic-1` per race batch** | Corpus-Wide Ingest Lanes, folded from SD-29 | per-kind ingest/instrument: F1 `monster` (fixture-coverage lane, rewritten 2026-08-15), F2 `spell`, F3 `race`, F4 `race_trait`, **F5 `equipment`, F6 `equipment_modifier`, F7 `companion`, F8 `feat` (routes SD-30 E0-F3's 217-unit probe-fixture residue), F9 `monster_ability`, F10 `class` (all added 2026-08-15, blocker B2), F11 held static/derived residual (added 2026-08-15, blocker B4)** — each runs the raw-vs-workable split + pre-cycle classifier screen before claiming a book. **SD31-E6-F11-001 landed 2026-08-15** (`sd31-e6-heldcells`, merged onto `tranche/11` this integration cycle): exhaustive held-cell map built (`artifacts/SD31-E6-F11-001-held-cell-map.md`) covering all 6,916 held units — 2,481 `static`-held (0 overlap with `corpus_literal_sweep`'s verified set, three structurally distinct no-path buckets, `OPEN-ISSUES.md` row 11) and 2,792 `derived`-held, 97.4% of which (2,719) sit under a `kind` `derived_evaluator_fixture_check` has no evaluator seam for at all (`OPEN-ISSUES.md` row 12). **0 new fixture entries landed this cycle** — the card's own "growing fixture coverage" premise is exhausted against the currently-ingested corpus; the real levers are a new book ingest (`ultimate_equipment`, unblocks 60 units) or a `monster` evaluator seam (unblocks 1,229 units, multi-cycle). Card stays READY, not COMPLETE — F11 does not close on one fixture batch (this cycle's own zero-fixture result is the honest evidence for that). Headline figures corrected 2026-08-15 (`SD31-W2-INTEGRATE-001`, Findings 6/7): derived-held 2,777→2,792, ambiguous-held 309→400.** **SD31-E6-F4-001 (F3/F4, 2026-08-15):** Skinwalker (Bestiary 5) batch landed under the newly-opened per-race gate (see "The two gates that exist because of the merge" above) — `race_trait` `done` +6, `race` `done` unchanged at 0/103 (root cause traced and logged, `OPEN-ISSUES.md` row 22, not a chassis gap). Card stays READY for F3/F4: Skinwalker's heritage rows and every other unmodelled race remain gated on further Epic 1 chassis work. **SD31-E6-F11-002 landed 2026-08-15** (`sd31-e6-seam`, this worktree): re-derived the `derived\|grounded\|monster` headline fresh on this checkout (the classifier's own D3/D4 fix — `SD31-E2-F2-001-wiringfix`, "1,265 derived→static" — has moved most of `monster` since the F11-001 map was written) and found it **already down to 280**, not 1,229 (955 of the old 1,229 are now `static`-held instead, a separate F11 lever; `retro.py correction` emitted). Built the missing `derived_evaluator_fixture_check` evaluator seam for `kind=monster`: `spell_like_ability_caster_level()` (`src/rules_core/derived_evaluator_fixture_check.rs`) reads PF1's Spell-Like-Abilities universal monster rule (caster level = Hit Dice) off the `MONSTERCLASS:<type>:<HD>` token's trailing integer every monster row already carries, resolved through the real `monster_chassis::MONSTER_BOOKS` registry — not a new parallel table. `run_bar_check` now merges an equipment report and this new monster report (both public, `tests/fixtures/rules_core/derived-evaluator-fixtures.json` gained a sibling `monster_entries` array, the 94-entry `entries` array untouched). Landed a real first batch of **7 hand-derived monster fixtures** (`BONUS:VAR\|SLA_CL\|HD` shape, the largest clean sub-family found — CR-based/PCLEVEL-based SLA formulas and every STR/DEX/CON-scaling family, e.g. `ConstrictBonusDamage\|STR` on 266 of the 280, are structurally unreachable today: monster ingest is `completeness: "chassis_only"` and carries no ability-score field at all, so those need an ingest widening, not a fixture). Mutation-proved live (corrupted the Demon (Balor) fixture's expected caster level to 99, confirmed the binary's cleared-count dropped and it reported `FAIL bestiary:monster:demon_balor`, reverted) and via a new permanent test (`a_wrong_expected_caster_level_makes_the_bar_check_fail`). Guarded regen delta, measured: **+7 `done`, -7 `held`** (`doneness_verdict()` replay 6,076→6,083 of 38,521, 15.7732%→15.7914%); `docs/work-inventory.json` restored per the wave rule, not committed. Scale plan and full re-derivation in `progress.md`'s `SD31-E6-F11-002` receipt. Card stays READY — the seam now exists and cleared its first batch, but the ability-score-scaling majority (266 of 280) needs an ingest-widening cycle before a fixture can honestly cover it. **SD31-E6-F5-001 landed 2026-08-15** (`sd31-e6-equipment`, branch pushed, not yet merged): verified F11's "row 12" `ultimate_equipment` claim one record deep (0 `data/corpus/ultimate_equipment/` dir before this cycle) and onboarded the book — `cache_gen::ultimate_equipment` (new module) dumps the already-shipped, PI-screened `rules_tables::ultimate_equipment::equipment_tables`/`equipmod_tables` (1,369 + 180 records) to `data/corpus/ultimate_equipment/equipment/*.json` via `gen_cache_ultimate_equipment`, `enrich_equipment_raw_tokens` widened to enrich it, `OBSERVABLE_BOOK_DIRS` widened so the equipment-effect wiring probe observes the book. **Corrected the "~60 units" estimate** (`OPEN-ISSUES.md` row 22): the guarded regen's own doneness-verdict replay measured **+1,264 units to `done` board-wide** (equipment 2,650→3,908, equipment_modifier 911→917; board 6,076→7,340, 15.77%→19.05%) — the dominant lever is `corpus_literal_sweep`'s `static` literal-verified bar over all 1,549 new records, not only the narrow `derived` BONUS:STAT population row 12 named. `corpus_literal_sweep` CLEAN (0 findings) after fixing 2 records a pre-existing shared-parser `.COPY=`-merge defect had corrupted (`OPEN-ISSUES.md` row 30, renumbered from this cycle's own row 23 at integration — resolved a row-anchor collision with the race/seam lanes' rows appended to the same file this wave, logged for a future dedicated fix, out of this card's bounded scope). F5/F6's ~962/213 genuine `not-started` residue (other books) is untouched — this cycle closed the named immediate win only; card stays READY. **SD31-E6-F2-001 landed 2026-08-15** (own worktree branch, not yet merged): F2 `spell` — traced end to end before ingesting (`OPEN-ISSUES.md` row 22): the engine's spell catalog only chains FIVE books (CRB/APG/ACG/ARG/UI), so 1,548 of the kind's 1,561 `not-started` units cannot reach `done` through any existing instrument until a sixth `SPELL_LIST` module is built; the remaining 13 are PCGen `.CLEARALL` copy-variant spells that genuinely state no level (`crb::spell_list::SpellListEntry.level` is non-optional `u8`), so ingesting them would invent a number. Pivoted to the real lever: built `src/bin/enrich_spell_raw_tokens.rs` (book-agnostic `raw_tokens` population, reuses `corpus_literal_sweep`'s own token-closure code), enriched 1,173 spell corpus records across the five modeled books, corpus_literal_sweep stayed CLEAN (3,635→4,808 examined, 0 findings). Measured: `spell` `done` 47→56 (+9), corpus-wide `done` 6,076→6,085 (+9). Root-caused why only 9 of 120 `static`-held candidates promoted rather than overclaiming: 101 have a pre-existing `source.line` citation defect pointing at a `.MOD` override row instead of the base declaration (`OPEN-ISSUES.md` row 33, renumbered from this cycle's own row 23 at integration, worked example `accelerate_poison`). F9 `monster_ability`: found and did NOT act on two structural findings instead of ingesting unsafely — 486 of the kind's not-done units (`advanced_class_guide`+`core_essentials`) are corpus-shape-misclassified via a shared, cross-kind `refine_kind()`/`MONSTER_ABILITY_TYPE_FACETS` heuristic (out of this card's file territory to fix), and every sampled `computed`\|`not-started` unit in the genuinely-real bestiary books (Bestiary 1/2) is orphaned — its owning monster is unmodeled in that book's own chassis, so no player surface could ever show it even if hand-added (the DoD-8 "twin" failure) — `OPEN-ISSUES.md` row 34 (renumbered from this cycle's own row 24 at integration; also stamped with the correct cycle-id `SD31-E6-F2-001`, corrected from the original mis-stamp `SD31-E6-F9-001`).  **`SD31-W3-INTEGRATE-001` landed 2026-08-15**: merged all 5 worktree branches above onto `tranche/11`, fixed 3 CONFIRMED equipment-lane defects — `miser_s_mask` was shipping another item's 18,000 gp/2 lb (`ue_equip_magic_items.lst:714` is two items glued by a missing newline), corrected to its own 3,000 gp/1 lb with a regression test; corrected the receipt's "both SD-30 PI invocation contracts called" overclaim (only the blacklist half + `DESCISPI:` half are wired, `NAMEISPI:YES` on 2 records — Otyugh Hide, Elysian Shield — needs an operator ruling, `OPEN-ISSUES.md` row 38); corrected `scripts/verify-baselines.env`'s 37+2 un-enriched-record accounting to the true 39+3=42. Corrected both lanes' false `AUDIT_EXIT=0`/`VERIFY_EXIT` claims to the true state (trap-report exits 2 on 1,040 pre-existing, unrelated findings — `OPEN-ISSUES.md` row 41; the spell lane's background gate had died, not completed). Corpus-wide guarded regen at the fixed tip: board `done` **6,076 → 7,355 (+1,279)**, 19.09% of 38,521. Card stays READY for F2/F3/F4/F9 as scoped above; F5/F6's own remaining `not-started` residue and the Mitre of the Hierophant gap (`OPEN-ISSUES.md` row 40) are untouched by this integration cycle.** **`SD31-PI-REPAIR-001` landed 2026-08-16** (own worktree, branch `sd31/pi-fix`, pushed, not yet merged): row 38 FIXED — `cache_gen::ultimate_equipment.rs` computed `declared.name` and never read it; now a `NAMEISPI:YES` row is dropped (not redacted, matching `decisions.md §50.3`'s "a key cannot be redacted" ruling), re-dumped for real (`1368` equipment, was `1369`; `otyugh_hide.json` deleted from disk). Corrected row 38's own "2 records" claim to the true population of 1 (re-derived corpus-wide, `retro.py correction`); logged a new, un-owned finding (`OPEN-ISSUES.md` row 46, `rules_core::rules_tables::equipment_gap_tables.rs`'s own unscreened `Elysian Shield` literal). Built a new `declared-pi-audit` `verify.sh` stage (`src/bin/declared_pi_shipping_audit.rs`) so this defect shape — a screening contract asserted in prose and never called in code — cannot recur silently for either row 38's or row 39's shape; mutation-proved both check shapes (8 unit tests + one live end-to-end proof against the real wired gate). Full receipt: `progress.md` "SD31-PI-REPAIR-001".** **SD31-E6-F3-002 landed 2026-08-15** (`sd31/sweep-attrib-race-e6f3-002`, own worktree, pushed, not yet merged): fixed the `corpus_literal_sweep --json-out` book-attribution bug row 22 traced (`OPEN-ISSUES.md` row 22, now Resolved) — the writer derived `book` from `record.source_path`'s single-level parent directory instead of the same `book_dir_of()` grouping the binary's own `by_book` pass already trusts, so every nested `race`/`race_trait` row (`core_essentials/races/<race>/*.lst`) attributed to a race NAME, not a book, and could never satisfy `v06_work_inventory`'s `(book, file, line)` join. New `short_book_of()` helper (last segment of `book_dir_of`), 6 tests including a real-enumeration collision check (0 found corpus-wide) and a synthetic collision proof. Self-corrected mid-cycle from an initially wrong fix (derived book from the shipped-record's own `data/corpus/` directory, `"core_rulebook"`) after checking the committed inventory and finding the real join key is the PCGen-ORACLE-derived book, `"core_essentials"` — `retro.py correction` filed. Measured via a real before/after `--json-out` diff: **330 triples corrected, 5,997 unchanged (zero regressions)**. Guarded regen: board `done` **7,355 → 7,367 (+12)**; `race` `done` **0 → 7**, off zero, all 7 CRB races; `race_trait` `done` **484 → 489 (+5)**. DoD-8: Dwarf character-creation form screenshot shows the racial ability modifiers, size, and vision rendering live. Investigated row 27's claim that this fix would also clear `v06_corpus_trap_report --audit`'s 1,040 `wiring-class-mismatch` findings board-wide — **found FALSE, unrelated root cause** (stale STORED `wiring_class` JSON field vs. this cycle's book-attribution bug; proven by running the audit with the fix applied and reverted, byte-identical output both times, `259 0 mod-record; 0 950 wiring-class-mismatch` — also corrects the count itself, 950 not 1,040, already stale before this cycle) — logged as `OPEN-ISSUES.md` row 46, DoD item 3 unchanged by this card. Full receipt: `progress.md` `SD31-E6-F3-002`. F3/F4 stay gated per the per-race chassis gate for every OTHER race/book; this cycle's fix is race-lane-independent infrastructure, not a new chassis batch.** **SD31-E6-F1-002 landed 2026-08-16** (`sd31/monster-widen-SD31-E6-F1-002`, own worktree, not yet merged): resolved `OPEN-ISSUES.md` row 44's "zero production callers" objection against `SD31-E6-F11-002`'s seam — `spell_like_ability_caster_level()` is now called from `apps/desktop/src-tauri/src/monster_catalog.rs::map_chassis_monster`, serves `MonsterCatalogEntryDto::spell_like_ability_caster_level` over the real `list_monster_catalog` Tauri command, and renders on `MonsterCatalogScreen.tsx` (`· Spell-like abilities CL <N>`) — proven by two new Rust tests and DoD-8 on-screen verification (see receipt). Added a `has_spell_like_abilities` presence gate (row-level `BONUS:VAR|SLA_CL|` token check, TDD-caught and fixed against Linnorm (Crag) before commit — an earlier `SPELLS:`-keyed draft broke 3 of the seam's own committed fixtures) so the function never hands a caster level to a monster with no spell-like abilities. Then widened `MonsterStatBlock` with `stat_adjustments: &[StatAdjustment]` (every `BONUS:STAT` token, verbatim, reusing `companion_chassis::StatAdjustment` rather than duplicating it) across all 13 registered monster books via `scripts/transcribe_monster_tables.py`, mutation-proved against the live pinned oracle (re-reads Demon (Balor)'s row fresh at test time, independent of the transcriber). Re-derived row 26's own headline fresh at this tip: `derived|grounded|monster` is **386, not 280** (`retro.py correction` emitted), and a fresh per-record check (not an assumed ratio) finds **104 of 386** are the genuine ability-modifier-scaling shape row 26 named — and confirmed by worked example (Animated Object (Medium)) that this ingest still cannot honestly fixture-cover them: `BONUS:STAT` is a DELTA against a base ability score no row in this corpus states, and computing a final score/modifier without it would be exactly the fabrication `SD31-E6-F11-002` already correctly refused. Did not regenerate `data/corpus/**/monster/*.json` this cycle (deliberate: the compiled `MONSTER_BOOKS` table, not the JSON cache, is what `v06_work_inventory` and the desktop app both read for this kind, so the widening is fully live without it — a JSON regen stays a separate, PI-review-gated follow-on). Board `done`/held counts unchanged by this cycle's own diff (0 new fixtures added; the 7 already-`done` units now rest on firmer ground, not more units). Card stays READY — the 104-unit ability-scaling family and the ~7-unit arithmetic-wrapper SLA_CL/`SR:10+TL` sub-lever both remain open follow-ons (`OPEN-ISSUES.md` row 47). **SD31-E6-F2-002 landed 2026-08-15** (`sd31-spell-reach`, own worktree, branch `sd31/spell-reach-e6-f2-002`, pushed, not yet merged): the two follow-ons `SD31-E6-F2-001` left open. **(a) The 101-unit `.MOD`-row citation defect (row 33), fixed.** Built `src/bin/repair_spell_citations.rs` (TDD, 6 tests): re-points a mis-cited spell record's `source.line` from a `.MOD` bookkeeping row to the row that actually declares it, regenerating `raw_tokens` via `corpus_literal_sweep::token_closure` from the correct base row (which still recovers the `.MOD` row's rich `DESC:` text via the closure's own identity lookup — no content lost). Ran corpus-wide: **881 repaired** (broader than the 101-unit target; 0 overlap with 36 genuine misses, a different citation shape). Target population verified before/after: **101 mismatch → 0 mismatch, 101 match**. `corpus_literal_sweep` stayed CLEAN. **(b) The missing `SPELL_LIST` capability, built and landed for one real book: `ultimate_magic`.** Rather than out-of-scope-ing the wave-3 finding a second time, built `src/bin/ingest_ultimate_magic_spells.rs` (TDD, 12 tests) — reuses the existing tested `pcgen_import::lst_parser::spell` parser (not reimplemented), derives `level` as the min across `CLASSES:`/`DOMAINS:` tokens (the ACG precedent), screens every record's NAME as well as description with BOTH SD-30 PI contracts (the safety-critical warning this dispatch named — did not reproduce `ultimate_equipment.rs`'s confirmed hole), ships real corpus gaps as `None` rather than fabricated (25 records with no class/domain token, 15 with an unrecognized `SCHOOL:Masterpiece` value). **269 real base spell records** wired into `spell_resolver::spell_catalog_rows()` as its 6th book (`SPELL_BOOK_UM`, TDD'd RED-then-GREEN). Ran the full "count change needs a sweep" grep across `apps/desktop`/`tests/` for every hardcoded five-book reference and fixed 4 files (`spell_catalog.rs`'s 3 pinned tests incl. `the_catalog_serves_every_ingested_book_not_only_crb` 1286→1555; `SpellCatalogScreen.tsx`'s `BOOK_ORDER`/`BOOK_LABELS`; its own test's independent `CHAINED_BOOK_CODES` oracle; `sd27_known_spells_must_be_on_the_class_spell_list.rs`'s own independent chain, re-derived not guessed: 1555/913) — `apps/desktop/src-tauri`: `cargo test --locked spell_catalog::` → 19/19 passed. Guarded regen (local, uncommitted per wave rule): board `done` **7,355 → 7,471 (+116)**; `spell` `done` **56 → 172 (+116)** — 101 from the citation repair (`literal-verified` 9→110) + 15 from UM's text-only `Masterpiece` records (`display`+`text-complete`, the existing "text-only features are complete" ruling, not a new exception). `ultimate_magic` spell `not-ingested` **291 → 22** (the 22 remainder are `.COPY=` variants, a different, named residual — `OPEN-ISSUES.md` row 47). Reachable ceiling unchanged at 98.95%. **(c) The 13 `.CLEARALL` units, re-verified one record deep and PROPOSED (not signed) as a Structural Exclusion Register entry** with all four `decisions.md §3` items — `OPEN-ISSUES.md` row 46, `RULING-NEEDED`; not ingested, not fabricated. Named the remaining scope precisely rather than declaring the capability "built" in general: **19 books, 1,257 `spell` units** still outside the now-6-book catalog chain (`OPEN-ISSUES.md` row 48), and `class_spell_levels.rs` not yet extended for `ultimate_magic` (same row). DoD-8 (on-screen verification) deferred — `run-desktop/SKILL.md` bars running `driver.sh` concurrently with this cycle's own full gate; logged rather than faked. Full receipt: `progress.md` "SD31-E6-F2-002". Card stays READY: F3/F4/F9 untouched by this cycle; F2 itself has real remaining scope (19 books) named in row 48.** **SD31-E6-F5-002 landed 2026-08-16** (`worktree-wf_1d83a743-99e-2`, own worktree branch, pushed, not yet merged): built the full book×kind map (`artifacts/SD31-E6-F5-002-book-kind-map.md`) and found `equipment_gap_tables.rs`/`feat_gap_tables.rs` — already-shipped, oracle-verified join tables for 8 other books' equipment/equipment_modifier residue (704 rows) and 7 books' feat residue (83 rows) — never dumped to `data/corpus/`. Built `cache_gen::equipment_gap` + `gen_cache_equipment_gap`: **701/704 rows resolved (99.6%)** to real citations (127 equipment + 574 equipment_modifier) across core_rulebook/advanced_players_guide/advanced_class_guide/advanced_race_guide/ultimate_combat/ultimate_intrigue/ultimate_psionics/ultimate_wilderness, with the dispatch's PI-name-screening correction applied (a required-field name PI hit excludes the whole record, never redacts in place — 0 of 701 real rows hit it, proven via synthetic tests). Corrected the dispatch's own equipment resolver shape via TDD (added the `.COPY=` fallback `cache_gen::ultimate_equipment` already had but this new module initially lacked: 175→701 resolved). **Caught and fixed a real defect before commit**: the generator's first `write_json` clobbered 2 pre-existing `core_rulebook` records on a slug collision (`OPEN-ISSUES.md` row 47) — reverted, fixed with a skip-on-exists guard + regression test. **Blocked on a pre-existing, out-of-territory defect**: `corpus_literal_sweep` fatals on ANY Dreamscarred-Press `lst_token` record (4-segment path vs. its hardcoded 5-segment assumption), confirmed pre-existing via a 5-wave-old `ultimate_psionics/monster` record (`OPEN-ISSUES.md` row 46) — the sanctioned guarded regen could not run this cycle; board-`done` delta for this cycle's 699 shipped records is unmeasured, honestly reported rather than inferred. `feat_gap_tables` (83 rows) mapped but not yet dumped — the clear next lever. Built `Trap::MultiCostRow` (`src/pcgen_import/corpus_traps.rs`) — the Mitre-of-the-Hierophant corpus-shape guard the card named, proven against the real historical defect's exact byte content; did NOT add a "Mitre of the Hierophant" table entry (needs the forbidden `cache_gen/ultimate_equipment.rs`'s citation resolver widened, out of file territory). Card stays READY. **`SD31-W4-INTEGRATE-001` landed 2026-08-16**: merged the 5th wave-4 worktree branch (`SD31-E6-F5-002` equipment_gap, `worktree-wf_1d83a743-99e-2`, 697 equipment/equipment_modifier records across 8 books) onto `tranche/11` alongside the other 4 -- all 5 branches now landed. Fixed 3 CONFIRMED cross-lane defects: the class_feature lane's own NAME PI blacklist-scan hole (14 records exposed, 2 unmarked -- `OPEN-ISSUES.md` row 48, SAFETY-CRITICAL, fixed and re-verified 0 exposed), a corpus-wide 413-record raw_tokens DESC PI leak spanning every prior PI-screening cycle (description correctly redacted, raw_tokens still carrying the original prose -- row 63, fixed and re-verified `declared-pi-audit: CLEAN`), and the UPSI `book_dir_of` sweep-abort that had silently prevented `corpus_literal_sweep` from ever completing over lane B's 697 records (row 60, fixed, sweep now CLEAN at 19422 records examined). Also fixed 2 corpus-fidelity defects the newly-unblocked sweep surfaced (a shared parser's same-name-row merge shipping tokens from the wrong corpus line, 3 records reverted to `raw_tokens: []` pending a root-cause fix -- row 61), 3 disabled-`#`-row equipment records shipping a raw KEY: as their name (row 62), and 2 pre-existing (pre-dating wave 4 entirely) test-pin defects in `equipment_resolver.rs`/`character_hub.rs` (row 66, both an over-claimed and an under-counted collision from the same 8 ACG records). Reconciled all cross-lane LICENSE.json double-counts and created 6 missing LICENSE.json compliance artifacts (row 64). Guarded regen at the fixed tip: board `done` **7,355 → 7,603 (+248)**, 19.09% → 19.74% of 38,521. Reachable ceiling unchanged at 98.95%. Full receipt: `progress.md` `SD31-W4-INTEGRATE-001`.** **SD31-E6-F4-003 landed 2026-08-16** (`sd31/racetrait2-SD31-E6-F4-003`, own worktree, pushed, not yet merged): widened `ingest_race_traits.rs`'s `IN_SCOPE_RACES` 24→30 (Catfolk, Kitsune, Ratfolk, Strix, Suli, Wayang) and ingested ARG's real alternate-trait rows for that SD-31-E6-F4-002 chassis batch — 24 new records (Catfolk 6, Kitsune 2, Ratfolk 4, Strix 6, Suli 5, Wayang 1), `race_trait` `advanced_race_guide` on-disk 259→283. **Discovered and fixed a genuine mutual-destruction hazard before it could bite**: `advanced_race_guide/race_trait/<race>/` is now shared by BOTH `ingest_races.rs` (standard-tier) and `ingest_race_traits.rs` (alternate-tier) for these 6 races, and each binary's pre-existing per-race `remove_dir_all` clear would silently delete the other's already-shipped files on its next run. Fixed with a real, content-keyed partition (`is_racial_default` — verified corpus-wide, not assumed: this binary never writes `true`, `ingest_races.rs` never writes `false` for these 6 races) via two new scoped-clear functions, `clear_own_alternate_trait_files`/`clear_own_standard_trait_files`, each with its own regression coverage; ran both binaries in both orders afterward and confirmed both tiers coexist (15 files/race — 9 standard + 6 alternate for Catfolk). Extended `race_resolver.rs`'s `ALTERNATE_TRAIT_REPLACE_FLAGS` table with the real 19 selectable + 5 grant-linked flags (Suli's `Energy Strike` fires 2 on one row; Strix's `Wing-Clipped` grants `Wing-Clipped ~ Strix ~ Flight`), re-derived clean against the disk-backed resolver. Full "count change needs a sweep" swept and fixed: `race_resolver.rs`'s own 5 pinned tests, `ingest_race_traits.rs`'s 2, `ingest_apg_race_traits.rs`'s 1, `tests/sd27_alternate_racial_trait_reachability.rs`'s 6, `tests/sd27_aasimar_globalvar_gate_closes_the_dead_affordance.rs`'s 1, `tests/v06_work_inventory.rs`'s 1, `apps/desktop/src-tauri/src/reach_gate.rs`'s 1 (both live), `apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs`'s 1 (doc text), `apps/desktop/src/characterHub/raceCreationCoverage.test.ts`'s 1, and `scripts/classify_race_trait_rows.py`'s own stale 18-race screening list (pre-existing drift from BEFORE this cycle, corrected to 30 while touching the area). **One cross-lane blocker reported, not fixed** (`OPEN-ISSUES.md` row 150): `pilot_compute.rs` (lane 1, out of this card's file territory) has no `ALTERNATE_TRAIT_SAVE_BONUSES`/skill-bonus table entries for Strix's 4 records, so `tests/sd27_alternate_racial_trait_reachability.rs`'s `every_alternate_whose_bonus_lands_on_a_total_this_engine_computes_is_named_and_really_applies` is genuinely RED (named+reachable, not yet wired to a computed total) — left red rather than weakened, exact fix named for the next lane-1 cycle. DoD-8: two on-screen screenshots, Catfolk's and Strix's real alternate-trait text rendering on the Race Traits screen (`artifacts/SD31-E6-F4-003/dod8-catfolk-alternates.png`, `dod8-strix-alternates.png`) — the app's own header also independently confirms "349 alternate racial traits across 31 races" and Strix correctly shows "Wing-Clipped ... Replaces Flight. Grants Flight". Full receipt: `progress.md` `SD31-E6-F4-003`.** **`SD31-W9-INTEGRATE-001` landed 2026-08-17** (primary checkout): merged the remaining 3 wave-9 lanes -- `sd31/feat-companion-e6-f8-002` (F8 feat: 242-unit companion held-mass trace, 190 feat units moved off `not-started` via the new known-magnitude-suffix fallback + gap-table generation), `sd31/monster2-SD31-E6-F9-002` (F9 monster: `transcribe_monster_tables.py` truncate-before-compute fix, script-only, zero corpus records touched), `sd31/equip-class-SD31-E6-F10-001` (F10/class-adjacent: Inner Sea Gods spell book onboarded, 92 real PI-screened records as the catalog's 9th book, `spell` `done` **1149 -> 1157**). Found and fixed a CONFIRMED precedence-1 PI exposure (`pick_your_poison.json`'s `Cayden CaiLean` deity-name typo variant, `OPEN-ISSUES.md` row 163) and a feat-rung gap letting 9 records reach `done` while serving a placeholder marker string instead of prose (`OPEN-ISSUES.md` row 166). Guarded regen at the fully-merged, fully-fixed tip: board `done` **10,759 -> 10,958 (+199)**, 27.9302% -> 28.4468% of 38,521. Reachable ceiling unchanged at 98.95%. Full receipt: `progress.md` `SD31-W9-INTEGRATE-001`.** **SD31-E6-F4-004 landed 2026-08-17** (`sd31/racetrait3-SD31-E6-F4-004`, own worktree, pushed, not yet merged): ARG's own 4-race follow-on chassis batch (Gillman, Nagaji, Vanara, Vishkanya) -- `arg_races.lst`'s full 37-race `.MOD` roster is now completely chassis-covered except Dhampir (heritage-shaped, pre-existing exclusion). Changeling and Samsaran deliberately excluded: each hits a genuinely new corpus shape (a third heritage axis; a `BONUS:ABILITYPOOL`- expressed gate the globalvar reader does not parse) rather than being guessed at -- both named in `ingest_races.rs`'s own doc comment. `race` chassis 31->35, standard `race_trait` records 297->335 (+38). `RACE_SIZES` extended (all 4 Medium); caught and fixed a real live creation-flow gap (Gillman read `Blocked` with an `unknown_race` diagnostic before the fix). Full count-change sweep across 14 files including a TypeScript coverage test (`raceCreationCoverage.test.ts`'s custom `assertEqual` helper, invisible to a `toBe`/`expect(` grep, caught only by running the real frontend suite). Guarded regen: board `done` **10,958 -> 10,995 (+37)**; `race_trait` `not-started` 2886->2848, `done` 704->741, `held` 13->14. **`race` kind did NOT move (96/7 unchanged) -- traced and filed as `OPEN-ISSUES.md` row 167 (RULING-NEEDED): `race` kind's done-measurement is frozen on the pre-corpus-expansion 7-variant `RaceId` enum, not the corpus-driven `RaceCorpus` mechanism `race_catalog.rs`/`reach_gate.rs` actually use -- confirmed pre-existing across all 3 prior chassis batches (Bestiary 2, Skinwalker, ARG's first 6), not caused by this cycle; up to ~28 units could move `not-started`->`done` with zero new ingest work once lane 2's `v06_work_inventory.rs` learns to check reachability the corpus-driven way.** DoD-8: 4 screenshots including a full Gillman character-creation-to-Computed proof (real AC/BAB/saves, ability-score modifiers matching the corpus chassis exactly). Gate: `VERIFY_EXIT=1`, 26/27 stages pass; the one FAIL (`site-dashboard-check`) is the confirmed pre-existing per-worktree structural hazard, row 153. `v06_corpus_trap_report --audit`: exit 2, baseline reproduces exactly (1 mod-record, 1225 wiring-class-mismatch), not worsened. Full receipt: `progress.md` `SD31-E6-F4-004`.** | sd31-e6-heldcells / sd31-race-lane / sd31-e6-seam / sd31-e6-equipment / sd31-e6-spell-mab / sd31-w3-integrate / sd31-pi-fix / sd31/sweep-attrib-race-e6f3-002 / sd31-monster-widen / sd31-spell-reach / worktree-wf_1d83a743-99e-2 / sd31-w4-integrate / sd31/racetrait2-SD31-E6-F4-003 / sd31/feat-companion-e6-f8-002 / sd31/monster2-SD31-E6-F9-002 / sd31/equip-class-SD31-E6-F10-001 / sd31-w9-integrate / sd31/racetrait3-SD31-E6-F4-004 | 2026-08-16, 2026-08-17 | `SD31-E6-F11-001` / `SD31-E6-F4-001` / `SD31-E6-F11-002` / `SD31-E6-F5-001` / `SD31-E6-F2-001` / `SD31-W3-INTEGRATE-001` / `SD31-PI-REPAIR-001` / `SD31-E6-F3-002` / `SD31-E6-F1-002` / `SD31-E6-F2-002` / `SD31-E6-F5-002` / `SD31-W4-INTEGRATE-001` / `SD31-E6-F4-003` / `SD31-E6-F8-002` / `SD31-E6-F9-002` / `SD31-E6-F10-001` / `SD31-W9-INTEGRATE-001` / `SD31-E6-F4-004` / `SD31-E6-F9-004` / `SD31-E6-F10-003` / `SD31-E6-F2-008` (wave 11, 2026-08-17) |

**Wave 5 landings (`SD31-W5-INTEGRATE-001`, 2026-08-16), four more lanes merged onto `tranche/11`:**
- `SD31-ATTRIB-001` (book attribution, merged first per dispatch): re-attributed the 1,610-unit
  `core_essentials` reporting bucket to each unit's true source book; `core_rulebook` race 0 -> 7,
  `core_essentials` residual 1,610 -> 634 (-> 644 after this integration cycle's own gathlain
  correction, row 89). Zero doneness transitions (attribution is a pure reporting-field relabel;
  `doneness_verdict` never consults `book`). Adversarial review CLEAN on gaming/PI; the +232-unit
  `bestiary`/`beastiary` spelling-divergence metric regression it surfaced (pre-existing, not
  introduced) stays an open follow-up (row 73's own trailing note).
- `SD31-E6-F2-003` (spell lists): chained Occult Adventures into the spell catalog as the 7th book,
  144 records transcribed verbatim, 0 units moved to `done` (all left honestly at held/in-progress).
  Adversarial review CLEAN on gaming/PI.
- `SD31-E6-F9-001` (monster_ability): fixed the row-34 misclassification (156 units demoted, exactly
  `advanced_class_guide`+`ultimate_wilderness`, zero from any monster book) and enriched 1,616
  records with real `raw_tokens` across 12 books (+102 `done`). The cleanest lane this wave —
  adversarial review independently reproduced both figures exactly and confirmed a genuine DoD-8
  screenshot.
- `SD31-E6-F5-003` (equipment residual): 620 new equipment/equipment_modifier records across 4
  books, `equipment` `held` -1,010 -> 473. Adversarial review CLEAN on gaming/PI but found 50 of the
  620 (8.1%) cite the wrong corpus row (real values, wrong provenance) — logged with remedy at
  OPEN-ISSUES row 90, owning epic `epic-6-ingest-lanes`, not fixed this integration cycle (needs its
  own TDD pass on the shared `equipment_gap::find_citation` helper). DoD-8 not captured at the
  branch tip; the integration cycle attempted to discharge it directly (see this row's own receipt
  in `progress.md` for the outcome).

**Wave 6 landings (`SD31-W6-INTEGRATE-001`, 2026-08-16), five lanes merged onto `tranche/11`:**
- `SD31-E4-F1-002` (class wiring, `sd31/e4-classwire2`): Gunslinger base chassis + supersession
  wiring (Grit, Nimble, Gun Training, Gunslinger Initiative; Pistolero + Mysterious Stranger
  archetypes). Found and reported (out of file territory) a THIRD `class_feature` structural
  blocker beyond rows 78/Decision-7's cell — `v06_work_inventory.rs`'s `modelled_class_books()`
  hardcodes only CRB/APG/ACG, so Gunslinger's records never even reach the id-suffix check
  (`OPEN-ISSUES.md` rows 96/97, renumbered from the branch's own 94/95). Adversarial review CLEAN
  on gaming/PI; the integration cycle raised `scripts/verify-baselines.env` to the real merged
  actuals as its own DoD-item-7 commit and re-derived the clearance-table Gunslinger row
  (0/17 → 3/17).
- `SD31-ATTRIB-002` (attribution, `cycle/sd31-attrib-002`): re-verified the row-68/73 residual at
  644 (matches wave 5 exactly); found and reported a 516-unit further-attribution opportunity in
  `ce_abilities_race.lst`'s mid-file `SOURCELONG:` directives (not fixed, out of file territory).
  **Integration-cycle correction (CONFIRMED finding, not gaming): the branch's own conclusion that
  `advanced_race_guide:race == 1` "IS correct, not a residual bug" was wrong on its second half —
  the unit is `Race Builder`, ARG's chargen-system scaffold row (`arg_races.lst:53`, no `RACE:`
  token), not a playable race. ARG owns 0 real races (37 reprints, gate-enforced in
  `race_resolver.rs`); the residual 1 IS a classifier artifact. Corrected in both `OPEN-ISSUES.md`
  row 98 and the branch's own `progress.md` receipt; new row 99 files the classifier follow-up.**
- `SD31-E6-F2-004` (spell+monster, `sd31/spell-monster-e6-f2-004`): monster `raw_tokens`
  enrichment (both SD-30 PI contracts wired from the production path, mutation-proved) + Ultimate
  Combat as the spell catalog's 8th book (269 base spell records). Adversarial review: NOT GAMED,
  PI CLEAN.
- `SD31-E6-F5-004` (equipment repair, `sd31-equip-repair/E6-F5-004`): fixed rows 90/92's
  equipment mis-citation for real (39 records re-cited, `KNOWN_KEY_MISMATCH_DEBT` 10 → 0) and
  landed a `corpus_literal_sweep` typed-field cross-check that caught one real corpus-fidelity
  defect (`poison_black_smear`'s fabricated `cost_gp=0.0`, corrected to `None` — the real corpus
  row has no `COST:` token at all). **Integration-cycle fix (CONFIRMED finding): the re-citation
  deleted `raw_tokens` on all 39 records without re-enriching, silently shrinking
  `corpus_literal_sweep`'s examined population 21716 → 21677 while leaving the baseline floor
  unmoved — a hard `verify.sh` "population shrank" ratchet fail the branch's own gate never
  reached. Ran `enrich_equipment_raw_tokens` post-merge: 39 enriched, population restored to
  22937+. Also caught by this cycle's own first full-gate run and fixed: the branch's
  `poison_black_smear` data fix left `tests/sd24_equipment_coverage_audit.rs`'s pinned
  `has_cost==4` assertion stale (now 3, re-verified against the pinned oracle).**
- `SD31-E6-F7-001` (`companion`/`feat`/`monster_ability`, own worktree, branch
  `sd31/companion-feat-monster-ability-e6f7f8f9`): built
  `enrich_companion_raw_tokens.rs` (`companion`'s counterpart to `SD31-E6-F9-001`'s
  `enrich_monster_ability_raw_tokens.rs`) — 922 `companion` records enriched with real `raw_tokens`
  corpus-wide, `corpus_literal_sweep` CLEAN (22638 examined, +922 exact match). **Adversarial review
  CONFIRMED PI CONTRACT VIOLATION (no exposure): the production write path called neither SD-30 PI
  contract — a substituted author-time grep, not a production-path call. Ported lane A's
  `screen_field_value`/`declared_product_identity` call sites via TDD (6 new tests, 2 mutation
  proofs against synthetic Demon-Lord-shaped rows, confirmed RED before the fix) in the integration
  cycle; the review's own independent audit had already confirmed 0/922 written records carried any
  PI hit, so the already-shipped 922 are unaffected — this closes the gap for the NEXT companion
  book.** Guarded regen, measured: `companion` `held` 506 → 441 (-65), `done` 416 → 481 (+65);
  board-wide `done` **7,340 → 7,405 (+65)**. **Found, traced one record deep, and reported (not
  fixed — lane 1's file) a real cross-lane join-key bug** blocking the other 34 of the 99 targeted
  `static`+`grounded` units: `apply_done_rung_stamps`'s `Static` arm joins `sweep_verified` on the
  re-attributed reporting `book` field instead of `source_book` (`OPEN-ISSUES.md` row 104,
  renumbered from the branch's own 94). **Fixed by the integration cycle** — traced one record deep
  and found the branch's own one-line suggestion (`book`→`source_book` unconditionally) would have
  REGRESSED a different population (7 CRB races' `literal-verified` stamps: `corpus_literal_sweep`'s
  `short_book_of` mirrors `book` for `RACE_TRUE_BOOK`-nested rows but `source_book` for root-level
  `ce_*.lst` rows — two genuinely different shapes). Fixed with an OR-join trying both fields; two
  new tests lock in each shape. **Render-readiness report for lane 1's `Kind::Companion`
  prose done-bar rung** (`OPEN-ISSUES.md` row 105, renumbered from 95): of the 223 zero-magnitude
  `grounded` `companion` units `SD31-D7-PROSE-001` named, 210 have real, corpus-sourced,
  render-certified text (201 via `description`, 9 via `description_variants` only — a rung checking
  `description` alone would under-claim those 9), 13 genuinely carry nothing to show a player. Added
  `no_served_description_variant_leaks_pcgen_syntax` (`companion_catalog.rs`, TDD) to close the one gap
  in the render certification the readiness report needed (the `description_variants` half was
  previously pinned on one record only). Traced one `held` unit per kind end to end (`feat`'s
  `static`+`grounded` 15-unit population has no `data/corpus/**/feat/*.json` at all for most books,
  `monster_ability`'s dominant `held` cell is Epic 2's `display`+`grounded` verdict-path blocker, same
  as `companion`'s own 182/958 share of it — neither is this card's file territory). DoD-8: real
  companion ability description on the live Companion Catalog screen. Full receipt: `progress.md`
  `SD31-E6-F7-001`.**

**Wave 6 integration's OWN fixes (`SD31-W6-INTEGRATE-001`), beyond the merges above:** 20 of 947
`monster_ability` units the tranche/11-direct `SD31-D7-PROSE-002` rung promoted sit on corpus rows
declaring a character-specific computed `DESC:` argument (`13+Con`, `CONSCORE`, `BreathWeaponDC`,
`SR`, `Mythic_Rank`, ...) that the render path silently drops, leaving a hole in the sentence on
screen — refused via a new `chassis_monster_ability_unresolved_desc_keys` fact (both the declared-
variable-list shape and the bare-`%N`-with-no-list shape). 5 `equipment_modifier` units shipped the
raw PCGen token `%CHOICE` verbatim (the equipment render path had no leak guard at all) — widened
`leaked_pcgen_syntax` to also catch `%<UPPERCASE-KEYWORD>` and wired a new
`corpus_json_description_leaks_pcgen_syntax` check into the equipment verdict arm (self-caught and
fixed a false positive from this same check: raw `%%` PCGen escapes collapse to one literal `%` on
render and must be checked against the RENDERED text, not the raw token — 3 real
`core_rulebook:equipment:*` units were briefly wrongly demoted before the fix). **This cycle's first
full-gate run caught a live consequence: `equipment_catalog.rs`'s own PRE-EXISTING pinned test
`no_catalog_serves_a_description_carrying_raw_pcgen_syntax` went RED the moment `leaked_pcgen_syntax`
was widened — proving the equipment catalog genuinely ships `%CHOICE` verbatim to a live player TODAY
(6 records, not 5). Fixed at the root rather than deferred: widened `render_pcgen_desc` itself to drop
an unresolved `%<KEYWORD>` the same no-fabrication way it already drops an unresolved `%N` (no
`PcgenDisplayValues` slot exists for a chargen-time player choice like a bloodline). Checked against
all four render consumers before landing: full lib 1894/1894, desktop 448/448 (was 446). Updated the
equipment-lane's own pinned "54-leak" fixture to 58 (4 real ACG `%CHOICE` occurrences the widened
detector now also counts in the raw tables). Re-ran the guarded regen after: board figure unchanged
(the render fix and the verdict-rung's own widened refusal, `!dropped_args.is_empty()`, net to zero
doneness movement, as they should — `OPEN-ISSUES.md` row 108, superseded in place).** Row 87/95's
flat-magnitude question re-sized at this tip (row 107): **~824 units total**
(645/937 `monster_ability` + 179/308 `equipment`/`equipment_modifier`) ride on the operator's answer,
not the single named unit the conservative default excluded. Board headline at this tip (producer's
own `doneness_verdict`): **38,521 units, done 9,488 (24.63%)** — up from wave 5's 7,340 (19.05%);
of the +2,148 net movement, **257 units are recovered from wave 5's anti-gaming demotion** (144
equipment_modifier + 112 equipment + 1 spell, all genuinely real descriptions the old `.lst`-closure-
only check could not see) and **2,688 are genuinely new** (real paths that did not exist even before
the demotion: 1,029 monster_ability, 826 monster, 591 equipment, 141 race_trait, 99 companion, 2
spell) — **0 units regressed off the pre-demotion baseline population.** Reachable ceiling unchanged
at 98.95% (38117/38521), same 9 `ambiguous|*` dead-end cells. Trap report unchanged at 1,191
wiring-class-mismatch (row 65's baseline exactly). **Full gate: `VERIFY_EXIT=0`, `RESULT: PASS`,
23/23 stages green** (root-lib 1894, root-full 6685/563 suites, desktop 448, reach 27, corpus-sweep
23859/0 findings, clippy root:47/desktop:7/0 errors, class-dump 31/31) — launched three times, the
first two runs each caught a real defect (a stale pinned test inherited from the equipment-repair
branch; a live `%CHOICE` leak in the desktop equipment catalog, fixed at the root in
`render_pcgen_desc`) before the third run went clean. Baseline floors raised to the final measured
actuals in a separate DoD-item-7 commit. Full receipt: `progress.md`
`SD31-W6-INTEGRATE-001`.
| `epic-7-book-onboarding` | READY | Book Onboarding, 100 % mandate | onboard the 7 `future_state` books — PI screen cited clean per book before any record is written | — | — | — |
| `epic-8-cloud-fanout` | READY (per lane shape, after one local proof cycle) | Cloud Fan-Out Protocol (grind **and** capability lanes) | local-proof-then-cloud-scale protocol; local orchestrator owns all `tranche/11` merges (updated from `tranche/10`, `decisions.md §6`); DoD-8 and dashboard-producer work stay local | — | — | — |
| `epic-10-version-numbering` | COMPLETE | Build Version Numbering | version-bump 0.11.0 for the `tranche/11` cut (`decisions.md §6`) — package.json/tauri.conf.json/Cargo.toml/Cargo.lock, the publish-workflow VERSION stamp, and the full test-fixture literal surface (8 files); full gate green (`VERIFY_EXIT=0`, 19/19 stages, `artifacts/sd31-s7-version-verify.log`) | sd31-ready-s7-version | 2026-08-15 | `SD31-S7-VERSION-001` |
| `epic-9-closure` | READY (gated on every other card) | Closure and the 100 % Exit Gate | `epic-0` audit at closing tip → reachable ceiling 100 % or signed register entries → **F3 bundle code review of this package's own diff (added 2026-08-15)** → closure receipt + promotion PR (opened, not merged) | — | — | — |

## The two gates that exist because of the merge

`decisions.md §2` inverted an ordering in which the capability builds were scheduled *after* the lanes
depending on them. Both dependencies were cross-package handoffs; both are now **internal hard gates**,
and a cycle claiming across an open one is out of protocol exactly as a PI-gate violation would be:

1. **`epic-1-race-chassis` → `epic-6-ingest-lanes` F3/F4.** No `race` or `race_trait` ingest cycle
   claims a book before Epic 1 has landed a chassis covering the races that book's rows reference. The
   gate opens **per race batch**, not all-or-nothing — Epic 1-F3 names each landed batch here as it
   lands, so ingest starts as soon as any chassis is real.
   **Open, batch 1 (SD31-E1-F1-001, 2026-08-15): Fetchling, Grippli, Ifrit, Oread, Sylph, Undine
   (Bestiary 2) — `epic-6-ingest-lanes` F3/F4 may claim any book whose `race`/`race_trait` rows
   reference only these 6 plus the original 18 CRB/Bestiary-1 races. Still closed for Dhampir
   (Bestiary 2) and every other unmodelled race — a book mixing modelled and unmodelled races is
   only PARTIALLY open, per-record, not claimable whole.**
   **Open, batch 2 (SD31-E6-F4-001, 2026-08-15): Skinwalker (Bestiary 5) — CHASSIS + STANDARD TIER
   ONLY.** `race` kind gains 1 unit (Skinwalker); `race_trait` gains 9 standard-tier rows (6 reach
   `computed`+`grounded`=`done`, 3 stay `display`+`grounded`=`held`). **Skinwalker's heritage-shaped
   alternates (65 of the 86 chassis-blind rows this race carries, plus 3 further Inner Sea Races rows)
   remain gated — `ingest_race_traits.rs`'s existing `subrace_grants()` mechanism (built for
   Aasimar/Tiefling) cannot be reused as-is: Skinwalker has no `<race>_abilities_globalvar_subrace.lst`
   file at all, and each heritage sets its `Skinwalker_Replace*` flags directly on its own constituent
   rows via a `PREMULT` gate rather than declaring them in a separate file `subrace_grants()` reads.
   This is genuinely new mechanism work, not a config widening — see `progress.md`'s `SD31-E6-F4-001`
   receipt for the full worked comparison. So this gate is OPEN for Skinwalker's chassis/standard-tier
   rows only; its heritage rows stay closed pending that mechanism build.**
2. **`epic-2-verdict-paths` → `epic-3-measurement` F4 and `epic-5-chassis-sweep` F3.** No
   `unknown`-bucket characterization or disposal cycle claims before Epic 2 is `COMPLETE`.

## Cross-SD gate discipline (SD-30's PI gate — satisfied, still cited)

`SD-30-class-feature-archetype-bundle`'s `epic-3-pi-gate` closed `COMPLETE` on 2026-08-14 (all of
F1-F4; SD-30 closed the same day, PR #363 open). The hard block it imposed on `epic-5-chassis-sweep`,
`epic-6-ingest-lanes` and `epic-7-book-onboarding` is therefore **discharged at package level** — but
per-book citation is still required: a cycle claiming a book cites SD-30's `progress.md` receipt for
that book's screen, and calls the documented invocation contracts (`SD-30 decisions.md §52.3` for the
blacklist sweep, `§53.5` for the declared-PI reader) from the production ingest path before writing any
generated record. Discharged is not the same as absent: a cycle that writes records without calling the
readers is out of protocol.

## Deferral is not available to a cycle

The phrase "or named a successor for the remainder" is struck from this package (`decisions.md §2`
item 5). A unit leaves the 100 % denominator only through the **Structural Exclusion Register**
(`acceptance-and-verification.md AT-31-100`), which requires the proving command, the named missing
capability with why building it is impossible rather than merely expensive, an `epic-0` audit run
reproducing it, and **operator sign-off**. A cycle may propose an exclusion; only the operator grants
one. An unsigned proposal leaves the unit in the denominator and its epic open.

## Cycle claims (cycle-supervisor protocol)

Identical procedure to `SD-30-class-feature-archetype-bundle/kanban.md`'s "Cycle claims" section
(edit `Status`→`IN-FLIGHT`, `Claimed-by`, `Claimed-at`, `Cycle-id`; append to `progress.md`; on
completion edit `Status`→`COMPLETE` and append the receipt). `epic-4-mechanism` and
`epic-5-chassis-sweep` cycles name the class explicitly in `Cycle-id`; `epic-1-race-chassis` cycles
name the race batch; `epic-6-ingest-lanes` cycles name the kind and book.

## Operator override slot

Operator may add or remove cards directly by editing this file. Cycle dispatch honors the post-edit
state.

## Wave 7 integration status (`SD31-W7-INTEGRATE-001`, 2026-08-16)

Five worktree branches merged onto `tranche/11` in the required order (dissolution, register, class
wiring, spell+racetrait, feat+equip+class), all confirmed content-present by direct symbol grep, not by
merge status alone. Board re-derived at the merged, fixed tip: **9,488 → 9,780 `done` / 38,521
(24.63% → 25.39%)**, zero units regressed off `done`, reachable ceiling **98.95% unchanged**.

Per-epic landing, named honestly rather than declared closed:

- **`epic-1-race-chassis`** — unchanged this wave; no race-chassis lane merged. Still `READY (first
  batch landed, more races remain)`.
- **`epic-4-mechanism`** — `sd31/classwire3-e4f1-003` landed: Ninja's real chassis
  (`class_ninja.rs`) + Scout archetype supersession, 9 new `build_pilot_headless_receipt` tests.
  **Shortfall named, not closed**: Ninja `class_feature` records cannot reach `done`/`held` at all —
  `modelled_class_books()` (`v06_work_inventory.rs`, lane-1 territory, `OPEN-ISSUES.md` row 96) still
  names only CRB/APG/ACG, so board credit for this wiring is structurally blocked until that lane-1
  fix lands. DoD-8 also blocked one level earlier: `CLASS_OPTIONS` (frontend) carries no `ninja` entry.
  Reachability of the computation itself is proven by the 9 headless-receipt tests instead.
- **`epic-6-ingest-lanes`** — `sd31-spell-racetrait-e6-f2-005` (660 new spell records, 4 books; 93
  `spell` units `held`→`done`) and `sd31/feat-equip-class-e6-f8-001` (15 `ce_feats.lst` gap rows
  reachable via `RuleSetId::Ce`) both landed. **Shortfall named**: `feat-equip-class`'s own
  `SD31-E6-F8-001-verify.log` ended mid-stage with no `VERIFY_EXIT` (CONFIRMED by wave-7 adversarial
  review) — discharged by this integration cycle's own full-gate run at the merged tip instead of
  re-running the branch in isolation. 7 of its 11 banked `ce_feats` units were found to carry an
  unchecked flat magnitude (Decision 7 PROXY WARNING not discharged) and excluded pending the rows
  69/87/95/107 ruling — see `progress.md`'s receipt.
- **Decision 9 (`core_essentials` dissolution)** — `sd31/dissolve-core-essentials` landed:
  `resolve_true_book_for_core_essentials` now source-line-aware; residual **644 → 128** (unchanged
  count from its own receipt). This wave's regen additionally surfaced a corpus-wide id-namespace
  repair as a side effect: `core_essentials:`-prefixed unit ids fell **1,610 → 128** once `unit_id()`
  minted consistently off the corrected `book` field (previously many re-attributed units still
  carried a stale `core_essentials:` id prefix, Decision 9's own named residual issue).
- **Decision 10 (Supersession Register)** — `sd31/d10-supersession-register` landed; wave-7 review
  found its gate could not detect a fabricated entry (dead oracle re-derivation) and one bad entry
  (`companion` corpus_key `"1"`). Both fixed this integration cycle (separate commit,
  `247b32dba`-shaped): gate now genuinely re-derives 116/116 objects from the pinned oracle;
  regenerated register 117→116 objects, `count_removed` 135→134. **Still PROPOSED, NOT applied to the
  live denominator** — wiring `EXCLUDED_UNIT_IDS` into the production doneness computation is the
  named next step (`OPEN-ISSUES.md` row 127, `SUPERSESSION-REGISTER.md` §11).
- **`epic-9-closure`** — not reached. Reachable ceiling is 98.95%, not 100%; 9 `ambiguous|*` dead-end
  cells remain, all Epic-2-owned. The flat-magnitude interpretive question (rows 69/87/95/107) is the
  single largest lever not yet turned — at least 856 units ride on it (see `OPEN-ISSUES.md`'s
  "Needs an operator ruling" section).

Full command-level detail, every figure's derivation, and the complete finding-by-finding fix record
are in `progress.md`'s `SD31-W7-INTEGRATE-001` receipt.

## Wave 8 integration status (`SD31-W8-INTEGRATE-001`, 2026-08-16)

Five worktree branches merged onto `tranche/11` in the dispatched order (class wiring, race_trait,
equipment, spell, attribution+feed), all confirmed content-present by direct symbol grep, not by merge
status alone. Board re-derived at the merged, fixed tip: **9,780 → 10,759 `done` / 38,521 (25.39% →
27.93%)**, denominator unchanged all package, reachable ceiling **98.95% (38,115/38,521), unchanged**.

**PRECEDENCE-1 PI work, not scope-creep**: fixed the exposure the review confirmed in the
attribution+feed merge (`site/dashboard/units/*.json` publishing 261 declared-PI names) by not
committing that directory at all, plus its two confirmed `--check`-gate bugs (TDD, mutation-proven).
Separately DISCOVERED, while performing the mandated dashboard publish, that the TOP-LEVEL public feed
`site/dashboard/PF1e-dashboard.json` ALSO ships declared-PI names in its manifests/roadmap content —
confirmed pre-existing (unchanged in `HEAD` before this cycle touched anything), sized (56 candidate
names), and logged as `OPEN-ISSUES.md` row 149, **RULING-NEEDED at PRECEDENCE-1**, the most urgent open
item in the whole package right now.

**GAMED verdict, fixed at the source, not merged around**: wave-8 adversarial review returned GAMED on
the equipment merge (`.COPY=` inheritance taught to every direction that lets a unit pass, never to
`wiring_class.rs`, the one place resolving it would raise the bar). Fixed by adding `wiring_class::
build_copy_base_index` and threading it through `token_closure_rows`'s every call site — TDD,
mutation-proven. The fix's scope is corpus-wide, not `equipment_modifier`-scoped: it also corrected an
**independent, pre-existing** identical gaming shape on plain `equipment` (−141 net this wave, a
demotion of records no merged lane ever claimed credit for) and on 2 `spell` `.COPY=` records — see
`progress.md`'s `SD31-W8-INTEGRATE-001` receipt §3 for the full, separated movement accounting.

Per-epic landing, named honestly:

- **`epic-4-mechanism`** — Samurai's real base chassis + Challenge/Resolve/Bonus-Feat wiring landed
  (`sd31/classwire4-e4f1-004`). **Board credit still 0/0**, unchanged: `modelled_class_books()`
  (`OPEN-ISSUES.md` row 96, lane 1's file) still blocks all three UC classes. Fixed a wave-7-carried
  doc-accuracy defect at the source: the module comment's "exactly two hits, both structural"
  full-oracle-grep claim did not reproduce (17 hits, 5 real archetype records, all out-of-scope
  `player_companion` books) — corrected with the real count and a forward-scope note.
- **`epic-6-ingest-lanes`** — Advanced Race Guide 6-race chassis batch landed (Catfolk, Kitsune,
  Ratfolk, Strix, Suli, Wayang; `sd31/racetrait/SD31-E6-F4-002`), `race_trait` net +50. Equipment
  `.COPY=` inheritance recovery landed (`sd31/equipmod-e6f6-001`) — GAMED, fixed this cycle as above,
  `equipment_modifier` net +152. Spell caster-level-linear DURATION seam landed
  (`sd31/spell-held-SD31-E6-F2-006`), `spell` +897, the largest single-lane movement of the whole
  package to date.
- **Decision 9/10** — untouched this wave; no register or `core_essentials` mechanism changes in any
  of the five merged branches. The attribution+feed lane's per-race citation evidence table
  (`SD31-ATTRIB-003-race-evidence.md`) is the live evidence artifact for `OPEN-ISSUES.md` row 140's
  open Inner Sea Races attribution question — still open, ruling still needed.
- **`epic-9-closure`** — not reached. Reachable ceiling still 98.95%, not 100%; same 9
  `ambiguous|*` dead-end cells, all Epic-2-owned. `UNIVERSAL_MODIFIER_CUES`' recall gap
  (`OPEN-ISSUES.md` row 143) is now the single largest lever named but not yet turned.

Full command-level detail, every figure's derivation, and the complete finding-by-finding fix record
are in `progress.md`'s `SD31-W8-INTEGRATE-001` receipt.

## Wave 10 integration status (`SD31-W10-INTEGRATE-001`, 2026-08-17)

Six worktree branches merged onto `tranche/11` in the dispatched order (chooser primitive first,
then inventory gaps, then race_trait, monster+companion, equipment+class, spell+feat), all confirmed
content-present by `git log --oneline origin/tranche/11..<branch>` before merge and by direct diff
inspection after. Board re-derived at the merged, fixed, guarded-regen tip: **10,958 → 11,229 `done`
/ 38,521 (28.4468% → 29.1503%)**, denominator UNCHANGED all wave, reachable ceiling **98.95%
(38,115/38,521), unchanged**. Zero stamp loss (38,540 raw ids identical before/after, traced by full
id-set diff, not a count). See `progress.md`'s `SD31-W10-INTEGRATE-001` receipt for the full
per-kind, per-move table (406 id-level moves, all traced).

**Two CONFIRMED review findings fixed before the regen ran, not merged around**:
1. **Fabricated magnitude** (chooser lane): Battlecry's duration grounded the raw Charisma SCORE
   (18) where the corpus token means the MODIFIER (4) — a 4.5x overstatement, caught before any
   board unit could pay out on it (the inventory-gaps lane's own fix is what makes the unit
   probe-reachable; sequenced so the fix landed first). Fixed, tests corrected, 6/6 green.
2. **Bulk-admits-units-on-a-lie** (spell+feat lane): 159 of Mythic Adventures' 358 ingested feat
   rows were PCGen `VISIBLE:EXPORT` display-plumbing twins served as independently selectable,
   ungated duplicate feats in the player-facing Add Feat picker — 142 of them had reached `done`,
   40% of the whole wave's original headline movement. Fixed at the generator
   (`gen_feat_gap_tables.rs::parse_lst` now skips `VISIBLE:EXPORT` rows), regenerated against the
   pinned oracle (358 → 199, exactly 159 removed), and every downstream count this touches was
   swept and re-verified by real `cargo test` runs across 6 files, not asserted.

**The chooser primitive — is it real, and what did it unlock?** Yes, it is real: `archetype_resolver::
chooser_option_selected` has genuine, non-test production callers on a live path
(`build_pilot_headless_receipt` → `pf1_adapter.rs`'s desktop entry point, confirmed non-`#[cfg(test)]`),
and both its corpus option pools (`ORACLE_MYSTERY_POOL`, `ORACLE_BATTLE_MYSTERY_REVELATION_POOL`)
transcribe the pinned oracle exactly with zero invented entries. What it unlocked THIS wave is one
real consumer, Oracle's Battle Mystery/Battlecry revelation, now correctly grounding a Charisma-
modifier-scaled duration. What it did NOT yet unlock, honestly: its advertised corpus-pool
membership guard is statically unreachable at both current call sites (both pass compile-time
constants that are always members of themselves — a real structural weakness, not a fabrication,
logged `OPEN-ISSUES.md` row 180), and no player-facing Mystery picker exists yet, so DoD-8 could not
be driven for this specific unit (blocked one level earlier, in `pf1_adapter.rs`'s canonical-seed
table — a Path A followup, same shape as the Sorcerer/Cleric/Druid precedent). The primitive itself
is the right foundation for the ~4,520-unit option-pool-with-no-chooser gap the mandate names as the
single largest `class_feature` cause; this wave used it for exactly one pool as a proof, and 4 more
Oracle mysteries plus Sorcerer's/Arcanist's own bloodlines/exploits remain unwired through it.

Per-epic landing, named honestly:

- **`epic-1-race-chassis`** — 4 more race chassis landed this wave (Gillman, Nagaji, Vanara,
  Vishkanya; `sd31/racetrait3-SD31-E6-F4-004`), `race_trait` net +37, `race` unchanged at 7/103
  (structurally frozen on the 18-years-obsolete `RaceId::ALL` enum, `OPEN-ISSUES.md` row 170,
  `RULING-NEEDED`-shaped dispatch decision, not an operator ruling — up to ~28 units could move on
  a single lane-2 fix). Book attribution for the 38 new standard-trait records + `nagaji.json` is
  characterized as mis-filed (should sit under `bestiary_3`/`bestiary_4`/`inner_sea_world_guide`,
  not `advanced_race_guide`) but NOT refiled — race attribution stays FROZEN pending row 140's
  operator ruling (`OPEN-ISSUES.md` row 183).
- **`epic-4-mechanism`** — the chooser primitive (above) landed for Oracle Battle Mystery, the
  first `class_feature` unit grounded through it rather than the hand-rolled
  `oracle_level_with_revelation` shape. Board-verdict impact of the primitive alone this wave: 0
  (the unit's own inventory record never reached the probe until the inventory-gaps lane's
  companion fix landed) — see row 168/`OPEN-ISSUES.md` for the traced cause.
- **`epic-6-ingest-lanes`** — `class_feature` registry widening (Pathfinder Unchained's 4 classes,
  `sd31/e5-f1-003-inventory-gaps`, +46 done), `monster` SLA_CL literal-override fix (+70 done,
  1,196/1,196 values independently re-derived against the pinned oracle, zero mismatches),
  `equipment`+`class` ingest lane (0 board movement this wave — traced, not fabricated; the 389
  `equipment_modifier` unmeasurable population and a `class` structural blocker were both sized,
  not closed), Mythic Adventures onboarded as the feat catalog's first new book (+118 real `feat`
  done after the VISIBLE:EXPORT fix above, +90 to `unmeasurable` for genuinely description-less
  records). `spell`/`feat` held-mass traces named two real, sized, unbuilt levers: `spell`'s
  `range_keyword` shape (206 units, the single largest remaining lever on `spell`'s `held` bucket)
  and `feat`'s 424-unit probe-coverage gap in the feat-effect probe's swept postures.
- **Decision 9/10** — untouched this wave; no register or `core_essentials` mechanism changes in
  any of the six merged branches or this integration cycle's own commits. The Supersession Register
  stays PROPOSED, NOT applied; race attribution stays FROZEN.
- **`epic-9-closure`** — not reached. Reachable ceiling still 98.95%, not 100%; same 9
  `ambiguous|*` dead-end cells, all Epic-2-owned.

**Shortfalls named on open cards, honestly:**
- `race` (epic-1) — frozen at 7/103 by an obsolete instrument, not a chassis gap; the highest-
  leverage single lever this wave found (`OPEN-ISSUES.md` row 170).
- `class_feature`'s option-pool-with-no-chooser population (epic-4) — this wave wired exactly 1 of
  the ~1,847 distinct pool names named in the mandate; 4 more Oracle mysteries alone are a same-
  shape, same-file next step.
- `class` (epic-6-F10) — 158/185 not-started, still never successfully worked; this wave's own
  equipment+class lane confirmed 0 board movement is the honest, traced outcome, not an omission.
- `equipment_modifier` (epic-6-F6) — 389 `unmeasurable` units traced end to end, characterized not
  closed (`OPEN-ISSUES.md` row 173).

Full command-level detail, every figure's derivation, the complete finding-by-finding fix record, and
the full gate log are in `progress.md`'s `SD31-W10-INTEGRATE-001` receipt.

## Wave 11 integration status (`SD31-W11-INTEGRATE-001`, 2026-08-17)

Five real lanes merged onto `tranche/11` (pool-match+slug first, then option-pools+picker, then
monster+companion, equipment+class, spell+feat) — the race_trait lane was dispatched `null` and had
genuinely nothing to merge (both prior race branches 0 commits ahead). Board re-derived at the
merged, fixed, guarded-regen tip: **11,229 → 11,828 `done` / 38,521 (29.1503% → 30.7053%)**,
denominator UNCHANGED, reachable ceiling **98.95% (38,115/38,521), unchanged**. Zero stamp loss
(38,540 raw ids identical before/after). See `progress.md`'s `SD31-W11-INTEGRATE-001` receipt for
the full per-kind, id-level move table.

**One CONFIRMED review finding fixed before the regen ran, not merged around**: the pool-matcher
lane's own 38-unit credit included 20 units (`Shaman Wandering Spirit ~ *` +
`Secondary Shaman Wandering Spirit ~ *`) grounded on a byte-identical computation to the DIFFERENT
corpus record `Shaman Spirit ~ *` — the matcher's two existing guards were scoped to cross-CLASS
collisions only and were structurally blind to a same-class DIFFERENT-SLOT collision (~533 units
corpus-wide sit on the same fault line, including 44 `Unchained Rogue Talent` units decisions.md
§10's own AMENDMENT explicitly forbids crediting off base Rogue). Fixed with a third guard
(`CLASS_FEATURE_POOL_SLOT_QUALIFIERS`) plus a permanent regression test; because the fix landed
before the regen ran, the wrong 20 never reached committed board state — the reported +18
`class_feature` credit is the correct number outright.

**Does the chooser now pay out — the operator's own question, answered plainly:** Yes, modestly,
through a different lane than the mandate expected. The pool-matcher fix genuinely widened
recognition to 249 previously-unmatchable corpus groups, but only **18 units** currently produce an
attributable delta on the sheet (10 `Shaman Spirit`, 3 `Witch Hex`, 2 `Sorcerer Bloodline`, 3
apostrophe joins) — recognition and grounding are different bars, and 863 of the newly-recognised
groups' members still decline as `NoConsumerDelta`. Separately, lane 2 wired 4 MORE Oracle Mystery
pools through `chooser_option_selected` (Stone/Waves/Wind/Heavens, joining wave 10's Battle) — this
moved **zero board units** this wave, the same honest zero wave 10 reported for the primitive's
first wiring. The chooser primitive now has 5 wired pools (not 1); the ~4,520-unit option-pool
population the mandate named as the single largest lever is still there, now sitting on a fixed
matcher, waiting on per-pool consumer-delta wiring.

Per-epic landing, named honestly:

- **`epic-1-race-chassis`** — no wave-11 output (lane A dispatched `null`); `race`/`race_trait`
  unchanged at 7/103 and 741/3,603. `OPEN-ISSUES.md` row 203 records the gap per DoD-6; row 165's
  standing audit (load-only race_trait evidence regardless of `pilot_compute.rs` wiring) is the
  next card lane A owes, not another ingest batch.
- **`epic-4-mechanism` / `epic-5-chassis-sweep`** — the pool-matcher fix (row 168/181) landed for
  real and board-verified (+18 net after this cycle's own same-class-slot correction); the chooser
  primitive gained 4 more wired Oracle Mystery pools (row 186/`SD31-E4-F2-002`), 0 board units this
  wave, honestly reported.
- **`epic-6-ingest-lanes`** — `equipment`+`equipment_modifier` gap widened 8 more books (+382
  `done`, 422 new corpus records, corrected from the lane's own overstated 424/25,165 figure —
  `OPEN-ISSUES.md` row 202); `spell` RANGE caster-level formula seam landed (+199 `done`, `held`
  bucket 593→394); `monster_ability`/`companion` root-cause traces both corrected from overstated
  "fully accounted for" claims to their real, narrower scope (rows 199-200) — the real remaining
  levers are ~443 owned un-transcribed monster abilities and ~288 archetype-owned companion rows,
  not the registry-gap/orphan-drop shapes the lane's own receipts claimed exhausted them.
- **Decision 9/10** — untouched this wave; no register or `core_essentials` mechanism changes.
  Supersession Register stays PROPOSED, NOT applied; race attribution stays FROZEN.
- **`epic-9-closure`** — not reached. Reachable ceiling still 98.95%, not 100%; same 9
  `ambiguous|*` dead-end cells, all Epic-2-owned.
- **Public feed (PI gate)** — `site/dashboard/PF1e-dashboard.json` refreshed at the wave-11 tip;
  `site/dashboard/units/` NOT committed. The refreshed feed still carries row 149's declared-PI
  roadmap names (re-checked directly, 7 confirmed present) — disclosed explicitly in the receipt
  per the mandate's own rule, not silently published.

**Shortfalls named on open cards, honestly:**
- `race` / `race_trait` (epic-1) — zero wave-11 attention; the highest-leverage un-run card this
  wave (`OPEN-ISSUES.md` row 203).
- `class_feature`'s per-pool consumer-delta gap (epic-4/epic-5) — the matcher is fixed, but only 3
  of the newly-recognised pools (Spirit/Hex/Bloodline) produce an attributable delta; ~4,271 units
  across Domain/Blessing/most-of-Bloodline/Mystery still need real chooser wiring per pool.
- `class` (epic-6-F10) — 158/185 not-started, still never successfully worked; equipment+class
  lane's own 0 board movement on `class` this wave is the honest, traced outcome.
- `Elysian Shield` cross-book PI propagation (new, row 197) — RULING-NEEDED, not acted on
  unilaterally.

Full command-level detail, every figure's derivation, the six-lane merge conflict resolution record,
and the full gate log are in `progress.md`'s `SD31-W11-INTEGRATE-001` receipt.

## Wave 12 integration status (`SD31-W12-INTEGRATE-001`, 2026-08-17)

Six lanes merged onto `tranche/11` (pool-consumers first, then feat-matcher, then racetrait5,
transcription, equip-class4, spell3 — the two class_feature lanes merged first per the mandate's
own instruction since they interact). All six lanes had real content this wave — no `null` lane,
unlike wave 11. OPEN-ISSUES.md row-number collisions from four independently-appended "204" rows
(and racetrait5's "205") were resolved by renumbering sequentially in merge order (204-218); every
row kept, none dropped.

Board re-derived at the merged, fixed, guarded-regen tip: **11,828 → 11,829 `done` / 38,521
(30.7053% → 30.7079%)**, denominator UNCHANGED, reachable ceiling **98.95% (38,115/38,521),
unchanged**. This tiny net headline number hides real motion in both directions and must not be
read as "a quiet wave" — see below.

**Two CONFIRMED review findings fixed before the regen ran, precedence order followed exactly
(PI first, then wrongly-credited units):**

1. **PI exposure, `enrich_equipment_raw_tokens.rs`** — the only writer of shipped `raw_tokens` on
   equipment records had NO Product Identity screening at all. 28 `inner_sea_gods` records
   (equip-class4's own new content) shipped a blacklisted deity/place name verbatim in
   `raw_tokens` while `description` was correctly redacted, under `license: "OGL"`. Fixed in the
   production path (both SD-30 contracts now run on every `raw_tokens`/`raw_bonus_chains` value);
   `gen_equipment_gap_tables.rs`'s "Mutation proof" test was ALSO confirmed unable to fail and was
   rewired to drive the real production function. All 28 already-shipped records remediated in
   place; `corpus_literal_sweep` re-run clean (0 findings) after the fix. `OPEN-ISSUES.md` row 219.

2. **`race_trait` credited on insufficient evidence, `v06_work_inventory.rs`** — a real universal
   magnitude record (Gillman/Vanara `~ Speed`, `MOVE:Walk,30`) walked past the universal-modifier
   refusal gate because that gate was only checked on the `text_only` branch; and 262 `computed`
   race_trait units reached `grounded`/board `done` on a LOAD observation alone (the record parsed
   and classified), never a consumer-delta observation — the identical "credit resting on a
   different record's computation" shape wave 11 found, one axis over. Both demoted to
   `ingested-magnitude` (`in-progress`, never `done`), gated on a new, table-derived (not
   hand-typed) `race_ids_with_a_magnitude_consumer()` check in `pilot_compute/mod.rs`. Deliberately
   conservative: race-level, not trait-key-level, so it corrects only the races with genuinely zero
   engine seam. `OPEN-ISSUES.md` row 221.

**The net board number is a wash between a real demotion and real new content, not a quiet wave:**
`race_trait` `done` fell **741 → 490 (-251)** from the fix above; `equipment` `done` rose
**4,754 → 4,998 (+244)** from equip-class4's own 5-book extension; `feat` **1,470 → 1,475 (+5)**
from feat-matcher's Superstition-adjacent gap rows; `monster_ability` **1,366 → 1,369 (+3)** from
the transcription lane (this CONFIRMS, by a live regen rather than narrative, the "+3" figure
adversarial review flagged as unverified in the wave-12 dispatch text — id-traced, not assumed).
`-251 + 244 + 5 + 3 = +1`, exactly the net board movement. Every unit of motion is accounted for on
one side of the ledger or the other; none is blurred into the headline number.

**Does the option-pool chooser now pay out more — re-derived, not re-quoted:** No further movement
this wave. `class_feature` `done` is unchanged at 137 (pool-consumers' Barbarian Superstition
wiring landed 0 board units, honestly — the same-choice-slot cross-variant collision it found is
correctly still refused by `classify()`'s book-attribution check, and lane A did not ask to remove
it); feat-matcher's 249-unit `mod_only_rescue` phantom-duplicate finding (a real, exact,
zero-exception population) was PROPOSED but NOT applied, routed to OPEN-ISSUES row 205 for an
operator ruling per the same propose-then-rule pathway Decision 9/10 established. The ~4,271-unit
option-pool population still sitting on a fixed matcher, waiting on per-pool consumer-delta wiring,
is unchanged from wave 11.

Per-epic landing, named honestly:

- **`epic-1-race-chassis`** — real wave-12 output for the first time in 5 waves: 11 new records
  (Gillman/Nagaji/Vanara/Vishkanya), but net `race_trait` `done` FELL 251 once the load-only
  evidence gate closed — the honest outcome, not the lane's own claimed +11. `race` unchanged at
  7/103, still the standing five-wave stall (`OPEN-ISSUES.md` row 207).
- **`epic-4-mechanism` / `epic-5-chassis-sweep`** — Barbarian Superstition wired as a real
  consumer-delta representative in `pilot_compute/mod.rs`; 0 board units this wave (the credit is
  correctly refused by a pre-existing cross-book guard the lane itself did not ask to weaken).
- **`epic-6-ingest-lanes`** — equipment/class widened 5 more books (+244 `done`, 481 new corpus
  records) with a real PI exposure found and fixed in the same territory (row 219); spell fixed 2
  tautological mutation-proof tests (0 unit movement, test-quality only) and characterized a new
  14-unit `.COPY=` spell ingest gap (row 218, not built this cycle); monster_ability/companion
  transcription landed 168 new monster_ability records across 2 books (+3 `done`), closed a real
  wholesale-regen data-loss hazard in `gen_book_cache.rs` before it reached disk, and fixed a
  `WiringClassIndex` gap that had stamped 168 `core_essentials`-attributed citations `ambiguous`
  regardless of their real shape; feat landed 7 new gap rows across 2 books (+5 `done`) and
  root-caused (not applied) the 249-unit `mod_only_rescue` phantom-duplicate population.
- **Decision 9/10** — untouched this wave; no register or `core_essentials` mechanism changes.
  Supersession Register stays PROPOSED, NOT applied; race attribution stays FROZEN.
- **`epic-9-closure`** — not reached. Reachable ceiling still 98.95%, not 100%; same 9
  `ambiguous|*` dead-end cells, all Epic-2-owned.
- **Public feed (PI gate)** — `site/dashboard/PF1e-dashboard.json` refreshed at the wave-12 tip;
  `site/dashboard/units/` NOT committed. The refreshed feed still carries row 149's declared-PI
  roadmap names (re-checked directly, all 7 sampled names confirmed present) — disclosed explicitly
  in the receipt per the mandate's own rule, not silently published.

**Shortfalls named on open cards, honestly:**
- `race` (epic-1) — still 7/103, five-wave stall continues; `OPEN-ISSUES.md` row 207 traces exactly
  why the obvious fix (widening `RaceId::ALL`) would itself be a Decision-1(a) violation.
- `race_trait`'s remaining ~220-unit ambiguous middle (a seamed race's OTHER, non-tabled traits) —
  this wave's fix is deliberately conservative and does not resolve it; named as the next
  race_trait-owning cycle's own follow-up, not left silent.
- `class_feature`'s per-pool consumer-delta gap (epic-4/epic-5) — unchanged from wave 11, still
  only 3 of the newly-recognised pools produce an attributable delta.
- `class` (epic-6-F10) — 158/185 not-started, still never successfully worked this program.
- `mod_only_rescue`'s 249-unit feat-kind phantom-duplicate population (new, row 205) —
  RULING-NEEDED, not acted on unilaterally.
- A different-shaped, ~150-hit raw_tokens PI leak in `class_feature`/`spell` kinds, found while
  sweeping for row 219's shape but NOT remediated (several are false-positive substring matches on
  identifier fields; blind redaction there risks corrupting record identity) — row 220.

Full command-level detail, every figure's derivation, the six-lane merge conflict resolution
record, and the full gate log are in `progress.md`'s `SD31-W12-INTEGRATE-001` receipt.

## Board after wave 13 (`SD31-W13-INTEGRATE-001`, 2026-08-17)

Producer's own `doneness_verdict()`, `EXCLUDED_BOOKS = {beginner_box}`, at the six-lane-merged,
corrected tip:

| kind | total | done | % |
|---|---:|---:|---:|
| class | 185 | 27 | 14.59% |
| class_feature | 15472 | 130 | 0.84% |
| companion | 1696 | 680 | 40.09% |
| equipment | 6208 | 4998 | 80.51% |
| equipment_modifier | 1580 | 380 | 24.05% |
| feat | 2610 | 1475 | 56.51% |
| monster | 1270 | 910 | 71.65% |
| monster_ability | 2951 | 1369 | 46.39% |
| race | 103 | 7 | 6.80% |
| race_trait | 3603 | 497 | 13.79% |
| spell | 2843 | 1356 | 47.70% |
| **TOTAL** | **38521** | **11829** | **30.71%** |

**Denominator 38,521 (unchanged). Done 11,829 (unchanged) — a genuine net-zero reconciliation**, not
an absence of movement: **+8** class_feature (fixture seam, real new capability — the mandate's own
named highest-leverage gap, now closed for the first time) **+7** race_trait (Changeling/Samsaran
chassis, real new content) **-15** class_feature (PU roster-id audit, a correction: units previously
credited on the generic roster id alone, not a real per-feature magnitude) **+0** everywhere else
(ingest6, register-race, provenance/PI — none move doneness). `8 + 7 - 15 = 0`.

**The fixture seam now genuinely lets a `derived` unit reach `done`** — 8 class_feature units carry
`wiring_class=derived, status=fixture-verified` in the committed inventory today. This closes the
mandate's own "BINDING CONSTRAINT": prior waves wired real pools that landed `derived+grounded`,
capped at `held` for lack of an evaluator seam; that seam now exists and 8 units cross it. Row 225
re-derives the remaining corpus-wide population at 265 matching formulas (not the earlier 23-record
undercount) — most still uncovered.

**PI (precedence 1)**: 3 declared-PI names leaked in the committed `site/dashboard/units/` equipment
shard (`Bow of Erastil`, `Legendsbane`, `Witherfang`) with the new gate reporting CLEAN over them —
found and fixed (`.MOD`/`.COPY=` row-operator normalization, per-book index), mutation-proven on the
exact 3 names, 20 more real leaks found and redacted by the same fix. Two residual gaps disclosed, not
fixed: exact-leaf matching still misses declared-PI names embedded in derived labels (~23 hits, all
pre-existing, un-fixed), and 1,482 units' cited coordinates cannot be resolved at all (uncounted before
this wave, now logged).

Reachable ceiling: **98.95%** (38115/38521), unchanged. Trap report: **1 mod-record, 1,225
wiring-class-mismatch**, byte-identical to baseline.

**Shortfalls named on open cards, honestly:**
- `race` — still 7/103, six-wave-plus stall continues (`OPEN-ISSUES.md` row 207).
- `class_feature`'s corpus-wide 265-record BASE+VAR/DIVISOR population — only 8 covered; the census
  this wave corrected was previously undercounted by an order of magnitude (row 225).
- `find_level_var_alias` (both the Rust seam and its Python deriver) picks the first book-wide match
  with no tie to the record owning the formula — two live multi-definition collisions exist today,
  clearing the bar by coincidence, not by construction (row 234 item 4).
- The PU roster-id matcher's `CLASS_FEATURE_ID_MAGNITUDE_SUFFIXES` table still cannot recognize
  `fast_movement_bonus_feet`/`rage_rounds_per_day`/`rage_powers_known`-shaped ids, so 3+ of the 24
  demoted PU units have REAL dedicated magnitude functions the matcher simply cannot see yet — the
  demotion is correct, the underlying matcher gap is not closed.
- `class` — 158/185 not-started, still never successfully worked this program.
- PI gate residuals (embedded-name blind spot, 1,482 unresolved-coordinate units) — row 234.

Full command-level detail, every figure's derivation, the six-lane merge conflict resolution record,
and the full gate log are in `progress.md`'s `SD31-W13-INTEGRATE-001` receipt.
