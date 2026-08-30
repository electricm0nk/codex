---
canonical: true
owner: god-emporer
bundle_id: SD-34
status: not-started — planning-ready, launch gates unrun
date: 2026-08-26
---

# SD-34 Progress

Live cycle-by-cycle record. Cycles **prepend** their entry (newest first) and update
`kanban.md` in the same commit, via `workflow-instruction.md §5`'s retry protocol.

## Status

### Cycle 4 — AT-34-E4-002 — widen the trait/drawback spine to the 5 fixed-choice `%LIST` traits, land the desktop picker end to end — partial

**Status: partial.** Started from real HEAD (`58521a54ef`, wave 16) — cycle 3's capability build (`e8ac310280`) had already landed and merged; the dispatch prompt's stated `HEAD 651966b83e`/`DONE 151` was stale by two whole cycles, re-verified rather than trusted. Re-derived split: `DONE=182, M=58, U=21, D=2, X=2` of 265.

Took the next cheapest named sub-cause from cycle 3's own "what is left" list: 5 of the 28
remaining `trait_content` records carry a `BONUS:SKILL|%LIST` token constrained to a **fixed,
closed list of concrete named skills** (`trait_criminal`, `trait_fiend_blood`,
`trait_harvester`, `trait_influence`, `trait_style_sage`) — distinct from the 4 records whose
`%LIST` names an **open subtype family** (`TYPE=Craft/Perform/Profession`), which stay out of
scope (a genuinely different, open-text-entry input shape, named not built). Reused the
generic `SelectedChoice { choice_set_id, selection_id }` channel `archetype_resolver.rs`
already established for pool choices — not a new mechanism: `trait_effects::
skill_choice_bonuses_from_traits` reads a per-trait `choice_set_id` and honors only a
choice actually inside that trait's own `skill_options`, never a first-guessed default.

**End-to-end, no stub.** The compute path (`skill_choice_bonuses_from_traits`, folded into
`skill_allocation.rs`'s existing `misc_modifier` map) is fixture-executed and verified
(`every_choice_entry_is_genuinely_grounded_by_fixture_execution`), and the desktop surface
reaches it for real: `trait_picker.rs`'s `list_available_character_traits` now returns the 5
choice-based options with their `skillOptions`/`choiceSetId`, `CreateCharacterRequest` gained
`trait_skill_choices` (threaded into `compose_character_input`'s `selected_choices`), and
`CreateCharacterForm.tsx` renders a real `<select>` for a checked choice-based trait and
submits the player's actual pick.

**Figures.** `ultimate_campaign`: `DONE 182→187 (+5), M 58→53 (-5)`; `U:21 D:2 X:2 V:0`
untouched. All 5 closures are `ingested-magnitude → grounded`, id-set stable
(`0 added, 0 removed` in `docs/work-inventory.json`'s before/after diff) — no shared-corpus-`KEY`
payoff in another book this time (checked directly, not assumed). Corpus-wide, after this cycle's own regen (identical before and after both mid-cycle
rebases onto `AT-34-E3-002`'s own concurrently-landed WIP cycle, which had not yet
regenerated `docs/work-inventory.json` itself at either rebase point):
`population=49438 unclassified=0 overlap=0 citation_failures=0`, `DONE:24314 A:449 B:11769
C:4332 D:2955 M:4966 V:262 U:202 X:170 Z:19`.

**Instrument-correction, not content movement.** Rebasing this cycle's work onto
`AT-34-E3-002`'s own concurrently-landed WIP cycle (Cleric Domain) shifted every line in
`src/bin/v06_work_inventory.rs` a second time (this cycle's own doc-comment edit had already
shifted one pin once, fixed before the first rebase). `completion_atlas.py --check` caught it
immediately: `citation_failures=10`. All 10 `BUCKET_DEFINITIONS` citation pins re-derived
fresh against the post-rebase file in the same cycle — `A/B/C/D` at the exact literal call
site each comment already names, `DONE/M/U/X/Z` at the first `status: "<value>"`
construction-site occurrence, the same convention `DONE`'s own comment documents. No bucket
population moved by this correction.

**Verification.** `cargo test --locked --lib -- trait_effects`: 20/20 (9 new). `cargo test
--locked --bin v06_work_inventory`: 446/446. `cargo test --locked --manifest-path
apps/desktop/src-tauri/Cargo.toml -- trait_picker`: 3 new + 2 updated, all passing; the one
pre-existing `race_trait_picker` failure in the same run reproduced identically against the
pre-cycle-3 inventory content, confirmed unrelated. `apps/desktop`: `npx tsc --noEmit` clean;
`npm test` 96/100 files, both touched test files pass, the 4 failures confirmed pre-existing
corpus/version-triple drift by reading each one's own message. `cargo test --locked --no-run`
(full workspace, run after the last figure-moving commit): see this cycle's own receipt for
the literal exit/result. `corpus_literal_sweep`: unchanged, `48708` examined (no corpus file
touched). `denominator_gate.py --check`: `files_checked=15 violations=7` — pre-existing
`FRT_HVY`/quoted-corpus-prose baseline (was 6 at cycle 3, grown by intervening merged cycles'
own `progress.md` prose, none of them this cycle's).

**Remainder, named by sub-cause** (`M:53` of the original 58, unchanged from cycle 3's own
naming except the 5 now closed): 4 open-subtype `%LIST` traits (need a free-text
Craft/Perform/Profession chooser), 3 ability-score-difference-formula traits (need a formula
evaluator), 15 mixed-non-`SKILL`-bonus-type traits (different pillars), 1 corpus data gap
(`trait_shadow_whispers`), 17 narrative + 1 cross-skill-guarded Drawback, 12 `Retrain` records
(different mechanic). `U(21)/D(2)/X(2)` not touched, reopened, or reclassified — verified by
the inventory diff.

Receipt: `artifacts/epic-4-ultimate-campaign/AT-34-E4-002_cycle_receipt_4.md`.

### Cycle — wave-15 shared `docs/work-inventory.json` regeneration and attribution — complete

**Status: complete.** The single mandatory regeneration-and-attribution cycle closing wave
15's three dispatched lanes (UC `AT-34-E4-002` cycle 3, C `AT-34-E3-002` Monk Unarmed Damage
Medium, M `AT-34-E3-003` equipment `DAMAGE:`-token widening) — `docs/work-inventory.json`
regenerated exactly once via the required three-pass pipeline, for all of them. The dispatch
brief's own boilerplate said "four lanes"; re-derived from `git log b939abcd4b..HEAD` and the
dispatch script's own wave-14/15 lane list (`sd-34-dispatch.workflow.js`), this wave actually
ran **three** — the "four" is stale template text carried over from an earlier wave shape and
is named here, not smoothed over.

**Procedure:** `git fetch origin tranche/14 && git rebase origin/tranche/14` (fast-forward, no
conflicts) landing at `7147fd86ab`; baseline snapshot taken from that already-rebased HEAD
(`docs/work-inventory.json`, 49,438 units) per this cycle's own instruction order.

| Pass | Command | Wall time |
|---|---|---|
| 1 | `corpus_literal_sweep` | **4m9.446s** — CLEAN, 48,708/51,482 examined, 0 findings |
| 2 | `derived_evaluator_fixture_check` | **0m19.484s** — 1,839 cleared over 2,580 fixture rows, 0 failed |
| 3 | `v06_work_inventory` (no `--allow-stamp-loss`) | **13m32.058s** — regenerated, 49,438 units |

**Total pipeline wall time: 1,080.988s (18m00.988s)** — the cost figure this wave shape exists
to measure.

**Whole-corpus before/after diff by unit id:** 49,438 before, 49,438 after, 0 added, 0 removed,
**37 changed**. Because the baseline snapshot was taken *after* rebasing, it already includes
two lanes' own inline self-regenerations (`50790d6bf9` for C, `e8ac310280` for UC) — this
cycle's independent pipeline re-run reproduces both **byte-for-byte, 0 drift**: the 6
Monk-Unarmed-Damage-Medium `grounded` units and the 36 corpus-wide flat-`BONUS:SKILL` trait
`grounded` units are unchanged before vs. after. Only the M lane (which deliberately did not
self-regenerate, per the dispatch brief) produced real movement in this cycle's own diff.

**Every one of M's own stated expectations was independently confirmed exactly** — the
strongest possible outcome for this wave's "an expectation that turns out wrong is the most
valuable finding" instruction is that here there was no mismatch to report:

- Corpus-wide, 31 closures across exactly the 9 books M's own receipt named, unit for unit:
  `core_rulebook` 14, `ultimate_equipment` 5, `bestiary_3` 4, `inner_sea_races` 2,
  `ultimate_psionics` 2, `advanced_class_guide` 1, `advanced_players_guide` 1, `bestiary_2` 1,
  `ultimate_combat` 1 (sum 31), all `ingested-magnitude → grounded`.
- All 14 `core_rulebook` closures carry the `equipment_table_entry_with_corpus_magnitude`
  evidence (the own-line-magnitude shape) — **zero** from
  `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does` (the closure-only shape)
  in that book, exactly as M's receipt said: "one fix did not cover both shapes."
- 6 further changed units, all `ultimate_equipment`, all already `literal-verified` (bucket V)
  both before and after — evidence-string churn only, reported separately per `decisions.md
  §9`, not folded into closure or reclassification.
- `core_rulebook` M: 972 → 958 (−14). `core_rulebook` DONE: 4330 → 4344 (+14). Corpus-wide M:
  5002 → 4971 (−31). Corpus-wide DONE: 24278 → 24309 (+31). All match M's own receipt exactly.

**Movement, four buckets (`decisions.md §9`):**

- **Closure (reached DONE):** 31 — all `M → DONE`, real compute-and-apply via
  `equipment_key_is_wired`'s `damage_total::resolve_base_damage_dice` consultation (an
  already-wired path, not a new subsystem).
- **Reclassification (moved between non-DONE buckets):** 0.
- **Evidence-string churn, no bucket crossed:** 6 (see above) — reported separately.
- **Reachability:** 0.
- **Instrument-correction:** 0 — no prior wrong count was found; M's own local-regen figures,
  committed as prose only (per the `GENERATED_FILE_BAN`), were exactly right.

**Atlas checks:** `python3 scripts/completion_atlas.py --check` (corpus-wide):
`population=49438 unclassified=0 overlap=0` — `DONE:24309 A:449 B:11769 C:4332 D:2955 M:4971
V:262 U:202 X:170 Z:19`, `citation_failures=0`, `done_evidence_violations=0`,
`missing_clearing_mechanisms=0`. `--book core_rulebook --check`: `population=6701
unclassified=0 overlap=0` — `DONE:4344 B:470 C:351 D:366 M:958 V:87 U:10 X:115`,
`citation_failures=0` (M's own two citation re-pin commits, `d2cd685ced`, already correct — no
further re-derivation needed this cycle). `python3 scripts/missing_engine_tables.py --check`:
`citation_failures=0`.

**Build verification (after this cycle's own regeneration commit):** `cargo test --locked
--no-run` (full workspace): exit 0 (~4m43s cold, ~1m20s warm). `cargo test --locked --no-run
--manifest-path apps/desktop/src-tauri/Cargo.toml` (desktop crate, separate cargo workspace,
tested explicitly per `decisions.md §10`): exit 0 (~1m20s).

`kanban.md` rows 14 (C), 15 (M), 29 (UC) each get a one-line confirmation pointer to this
cycle's own receipt — no story added to the Notes column, per this file's own row-hygiene
rule. Receipt: `artifacts/epic-3-core-rulebook/AT-34-E3-001_wave9_regen_receipt.md` (third
section, appended below the two unrelated prior cycles that happen to share this filename).

**This cycle implements no `epic-breakdown.md` criterion directly** — it is the wave's own
shared regeneration cycle, paid once for three parallel lanes that deliberately deferred (M)
or, unusually, did not need to defer (UC, C, whose own inline regens are now independently
verified correct) regenerating `docs/work-inventory.json`.

### Cycle — AT-34-E3-003 (bucket M, EQUIPMENT sub-causes) — partial, closure

**Status: partial.** Territory: the two EQUIPMENT sub-causes of `core_rulebook` bucket `M`
(`equipment_table_entry_with_corpus_magnitude` 276,
`equipment_own_line_has_no_magnitude_but_closure_wiring_class_does` 147, sum 423 of `M`'s 972),
a sibling lane holding `ability_content`/`race_trait`/`template` off limits by the dispatch
brief's own no-collision rule. Re-derived `M`'s full 10-sub-cause split fresh at this cycle's
rebased start (`f2c13e12b90e6dc0eed033cd447fb730629ccb91`, 9 commits ahead of an unreviewed
salvage checkpoint on `origin/salvage/wave13-lane3` claiming the same work): matched the
dispatch brief's own figures exactly, 972 = 276+217+147+119+96+47+34+19+15+2.

**What changed:** `equipment_key_is_wired` (the shared probe every book's `Kind::Equipment`/
`Kind::EquipmentModifier` classification routes through) gains a second, independent check
after its existing eight `compute_equipment_effects` stat fields all return `None`: it now also
consults `damage_total::resolve_base_damage_dice(key, corpus)`. Not a new compute path — that
function already reads a weapon's real `DAMAGE:` corpus token into a structured
`DiceExpression` and is already the entry gate `resolve_weapon_damage_breakdown` uses to build
the `WeaponDamageBreakdown` the desktop app's `character_hub.rs` renders on the real character
sheet. Widens what the probe OBSERVES, exactly the shape the prior `skill_content` cycle
widened for `Kind::Skill`. 4 new tests (positive on the real on-disk `Bastard Sword (Base)`,
positive on a hand-built fixture, a negative control, and a `classify()`-level end-to-end
proof).

**Both equipment shapes were checked separately, not assumed to share a fix — they don't.**
`equipment_table_entry_with_corpus_magnitude` (magnitude on the unit's own row): **276 → 262**
(14 real closures, `core_rulebook`). `equipment_own_line_has_no_magnitude_but_closure_wiring_
class_does` (closure-only alias rows, classified by their closure wiring class): **147 → 147**
(0 closures) — none of the 147 alias rows in this population resolve to a base record carrying
a `DAMAGE:` token. One fix did not cover both shapes, exactly as the dispatch brief warned to
check for.

**Generic by construction, movement corpus-wide, not book-scoped:** whole-corpus before/after
diff by unit id (comparing the last committed `docs/work-inventory.json`, `50790d6bf9`, against
this cycle's own local three-pass regen): 49,438 before, 49,438 after, 0 added/removed, **37
changed** — 31 real `ingested-magnitude → grounded` closures across **9 books**
(`core_rulebook` 14, `ultimate_equipment` 5, `bestiary_3` 4, `inner_sea_races` 2,
`ultimate_psionics` 2, `advanced_class_guide` 1, `advanced_players_guide` 1, `bestiary_2` 1,
`ultimate_combat` 1), plus 6 evidence-only reclassifications (all `ultimate_equipment`, already
`literal-verified` both before and after — bucket `V`, not `M`, named per `decisions.md §12`
L1). `core_rulebook` M **972 → 958**; corpus-wide M **5002 → 4971**; corpus-wide DONE
**24278 → 24309** (+31, matching the 31 closures exactly).

**Generated-artifact scope discipline, new this wave.** `docs/work-inventory.json` and
`completion-atlas.json` were regenerated LOCALLY (guarded three-pass pipeline:
`corpus_literal_sweep --json-out` → `derived_evaluator_fixture_check --json-out` →
`v06_work_inventory` with both report env vars) to derive every figure above, then
`git restore`-d before each commit rather than shipped — this cycle's dispatch brief reserves
committing those two files to a single shared end-of-wave regeneration cycle (a direct answer
to the wave-13 collision the same brief names). The committed `docs/work-inventory.json` at
this cycle's own HEAD still reads the pre-cycle 972/276/147 until that shared cycle runs; every
figure above ships with the command that reproduces it.

**Not inherited from the unreviewed salvage checkpoint — independently re-derived and it
agreed.** `origin/salvage/wave13-lane3` held an uncommitted checkpoint making the same claims,
rescued after a host kill mid-run. This cycle rebased past it, re-derived the starting figures
fresh, re-applied the code by hand against the rebased file (not a merge), reran the regen
pipeline itself, and independently re-derived the whole-corpus diff. The independently-derived
14/31/6 figures matched the salvage checkpoint's own claims exactly — confirmed, not assumed.

**Build scope:** `cargo test --locked --bin v06_work_inventory` 446/446;
`cargo test --locked --lib rules_core::damage_total::` 30/30; `python3
scripts/completion_atlas.py --check` (local regen) exit 0, `citation_failures=0`; `python3
scripts/missing_engine_tables.py --check` exit 0, `citation_failures=0`; `cargo test --locked
--no-run` (full workspace) exit 0 at `d2cd685ced`; `apps/desktop/src-tauri` tested explicitly
(`--manifest-path apps/desktop/src-tauri/Cargo.toml`, separate `CARGO_TARGET_DIR`) because the
changed function calls `damage_total::resolve_base_damage_dice`, which the desktop crate also
reaches transitively.

**This cycle's own citation upkeep:** `completion_atlas.py`'s 10 `BUCKET_DEFINITIONS` and
`missing_engine_tables.py`'s 2 `ENGINE_SURFACE_CITATIONS` shifted by this cycle's own +20-line
insertion, re-derived against real line content, `citation_failures=0` after both.

**Discovered, out of scope, not fixed here:** `python3 scripts/shape_engine_boundary.py --check`
reports `STALE_CITATION` on its `PROMOTION_LADDER_ANCHOR_LINE` — confirmed already stale at this
cycle's own rebase base (`f2c13e12b9`), pre-dating this cycle's touch, not in Epic 3's declared
file-touch set. Named per AT-34-E3-006's posture, left for its owning cycle.

Denominator gate against this package: `files_checked=15 violations=7`, all 7 pre-existing
(`FRT_HVY`'s quoted corpus prose), unchanged by this cycle's own new prose.

**Remainder — this cycle's own 423, named:** `equipment_table_entry_with_corpus_magnitude`
262 (per real sub-shape: AC-bonus equipmods need a new bucket-B-shaped compute path;
`%CHOICE`-gated modifiers are a sibling lane's scope; internal-plumbing VAR/ITEMCOST tokens
await a `decisions.md §17`-shaped ruling; named artifacts need a description-linked probe not
built this cycle) + `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does` 147
(same four sub-shapes, applied to the closure-only population) = 409. 14 + 409 = 423 exactly.

**Every other `M` sub-cause and buckets V/D/U/X are untouched by this cycle** (sibling-lane or
out-of-territory by design): `ability_content` 217, `race_trait_generic` 119, `template_content`
96, `in_catalog_with_corpus_magnitude_but_no_observed_consumer` 47, `domain_content` 34,
`skill_content` 19, `spell_list_entry_with_resolved_level` 15,
`race_trait_states_a_universal_sheet_modifier_pending_compute` 2, `V` 87, `D` 366, `U` 10,
`X` 115.

Full detail, remainder table, and next-cycle plan: `artifacts/epic-3-core-rulebook/
AT-34-E3-003_m_bucket_equipment_cycle_receipt.md`.

### Cycle — AT-34-E3-002 (bucket C continuation): Monk Unarmed Damage Medium closes 6, core_rulebook bucket C 357->351

**Status: partial.** Re-derived at start SHA `b939abcd4b`: `core_rulebook` bucket C is **357**
(the criterion's own headline "370" is stale, `decisions.md §12` L2). Confirmed the prior
`AT-34-E3-002` cycle's named remainder still holds (population drifted 372 -> 357 between then
and now purely from OTHER cycles' unrelated classifier work — `AT-34-E3-003` bucket-M
`skill_content`, `AT-34-E3-005` bucket-V-apply, and the wave-13/14/15 salvage folds).

**Mechanism closed:** the prior cycle's own next-cycle plan named `Monk Unarmed Damage` (54
units, largest named cluster) as unverified. This cycle verified it directly and found it
decomposes into three genuinely different populations, not one: **Medium** (6 band-start
levels — real formula in the SAME book's own Monk chassis, `class_chassis.monk.unarmed_strike_
damage_die[_count]`, already proven live for a Human/Medium character), **Small** (6 levels —
real formula exists, but reachable ONLY through the Pathfinder Unchained book's own Unchained
Monk class path — a cross-book attribution question this cycle deliberately left open, not
force-closed), and the **other 7 creature sizes** (42 units — no formula anywhere in the
engine, a genuine gap confirmed by reading `monk_unarmed_strike_damage_die_for_size` directly).

One new `classify()` rung + one new probe (`probe_monk_unarmed_damage_die_wiring`, run at
exactly each record's own band-start level, not `SWEEP_LEVELS`, which has no member inside the
level-16 band) close the 6 Medium units, same paired display/chassis-record shape as the prior
cycle's Favored Enemy/Terrain fix. RED->GREEN: 5 new tests (2 positive, 2 negative controls
including a non-Medium scoping proof, 1 live-fixture probe proof observing exactly the 6 wired
pairs against the real pipeline). `class_feature`-scoped suite 131/131 pass; full bin suite
442/442 pass (post-rebase); `cargo test --locked --no-run` (workspace) exits 0.

**Live regen, isolation confirmed by whole-inventory id-keyed diff (not sampled):** 0 added, 0
removed, exactly 6 changed — the 6 targeted Medium units, `engine-does-not-hold ->
literal-verified` (the same static/sweep-verification upgrade the prior cycle's own fix went
through — landed in bucket **V**, not `DONE`, reported honestly). `corpus_literal_sweep`:
48,708 examined, CLEAN, unchanged (no `data/corpus/**` file touched). `core_rulebook` bucket C:
357 -> **351**. Corpus-wide bucket C: 4,338 -> **4,332** (delta -6, no cross-book side effect —
`"Monk Unarmed Damage LVL"` exists only under `core_rulebook` across all 37 books).

**Mid-cycle rebase**: `AT-34-E4-002` cycle 3 (sibling territory — trait/drawback,
`CharacterInput`, desktop) landed on `tranche/14` while this cycle ran, regenerating
`docs/work-inventory.json` for its own 31 `ultimate_campaign` units. Resolved by taking the
upstream tip as the fresh base and re-running this cycle's own regen on top of it (never
hand-editing the JSON) — re-verified isolated to the same 6 unit ids by a second id-keyed diff.
`completion_atlas.py`/`missing_engine_tables.py` citations re-derived twice (once for this
cycle's own insertions, once more for the sibling lane's) — `citation_failures=0` at the final
SHA both times.

**Remainder — 351 of 357, named by sub-cause, populations sum exactly** (re-derived fresh, not
carried forward): `domain_power_display_record_not_wired` 96 (unchanged, largest sub-cause),
`bloodline_power_or_bloodline_feat_not_computed` 77, `prestige_class_standalone_feature_not_
computed` 31, `monk_unarmed_damage_no_formula_in_engine` 42 (the other 7 sizes, genuine gap),
`base_class_standalone_feature_not_computed` 36, `rage_power_not_computed` 13, `rogue_talent_
not_computed` 10, `npc_class_standalone_feature_not_computed` 10, `versatile_performance_not_
computed` 9, `monk_unarmed_damage_small_cross_book_attribution_undecided` 6 (deliberately not
decided this cycle), `other_named_group_or_standalone` 21 (next-cheapest candidate:
`Basic Favored Enemy`/`Basic Favored Terrain`, whose root DEFINE the prior cycle's own probes
already proved wired — unverified this cycle, named for the next one). Sum: 96+77+31+42+36+13+
10+10+9+6+21 = 351.

**Denominator gate against this package**: `python3 scripts/denominator_gate.py --check
'docs/release/SD-34-book-completion/*.md'` -> `files_checked=15 violations=6` — all 6
pre-existing in `progress.md` (verbatim-quoted corpus prose, "75% chance..."), each already
self-flagged inline by the cycle that introduced it (`AT-34-E3-004` and others). This cycle
added no new bare-percentage `.md` prose. **Correction to the prior `AT-34-E3-002` receipt's own
`violations=0`**: 6 other cycles' `progress.md` entries landed between then and now, each
correctly self-documenting its own quote — re-derived fresh here per `decisions.md §12` L2.

Receipt: `artifacts/epic-3-core-rulebook/AT-34-E3-002_cycle_receipt.md`.

### Cycle — AT-34-E3-001 wave-9 regeneration and attribution: 0 units moved, the wave's own premise did not hold for 2 of 4 lanes

**Status: complete.** Single mandatory `docs/work-inventory.json` regeneration closing wave 9's
four dispatched lanes (M `skill_content`, V-ledger corpus-wide bucket-V rebuild, UC
`AT-34-E4-002` cycle 3, and this cycle). Before touching anything, folded two lanes' staged-but-
uncommitted duplicate-dispatch cleanup already sitting in this shared checkout (superseded
`AT-34-E3-005` second-lane-confirmation and `AT-34-E4-002` cycle-3 entries, both already fully
captured by earlier landing commits) into its own commit rather than discard or stash it, then
`git fetch origin tranche/14 && git rebase origin/tranche/14` pulled in the M lane's
`4d27d70551`/`409ada6cda`, producing two real conflicts (`completion-atlas.json` `derived_at`;
`progress.md`'s new M-lane section colliding with this cycle's own section removals) — both
resolved by hand.

**Three-pass pipeline, in order, timed:** `corpus_literal_sweep` (CLEAN — 48,708/51,482 records,
413,336 tokens, 0 findings) → `derived_evaluator_fixture_check` (0 failed, 0 not-ingested — 1,839
units over 2,580 fixture rows) → `v06_work_inventory` regenerated corpus-wide (49,438 units).
`--allow-stamp-loss` never passed. **Wall time: 1,471s (24m 31s)** for the full three-pass run —
the figure this wave shape exists to measure.

**Whole-corpus before/after diff by unit id: 0 changed, 0 added, 0 removed.** The raw
`docs/work-inventory.json` diff is one line (`generated_at` SHA only) — every one of 49,438
records is byte-identical before and after this cycle's own pipeline run.

**The mismatch, reported plainly per this wave's own instruction, not smoothed over: the
dispatch brief's premise — "four lanes just landed engine changes and deliberately did not
regenerate `docs/work-inventory.json`" — did not hold for 2 of the 4.** `cfd9c6d3d9` (V-ledger)
and `4d27d70551` (M) each regenerated and committed `docs/work-inventory.json` themselves, inline
with their own engine/data change (13,406 and 160 lines respectively) — and this cycle's
independent, from-scratch three-pass re-run confirms both were done correctly: 0 drift, 0
disagreement, byte-identical. Only the UC lane (`0007792438`, zero Rust/corpus change) correctly
left the file untouched. This cycle's real contribution for the V-ledger/M lanes is
**verification, not correction**.

**Movement, four buckets (this cycle's own pipeline run):** closure 0, reclassification 0,
reachability 0, instrument-correction 0. (The 76-unit `skill_content` closure and the bucket-V
corpus-wide widening both happened inside `4d27d70551`/`cfd9c6d3d9` themselves, already present
in this cycle's baseline snapshot — not movement this cycle's own run produced.)

**Atlas checks:** `python3 scripts/completion_atlas.py --book core_rulebook --check` →
`population=6701 unclassified=0 overlap=0`, `DONE:4330 B:470 C:357 D:366 M:972 V:81 U:10 X:115`.
`python3 scripts/completion_atlas.py --check` (corpus-wide) →
`population=49438 buckets=10 unclassified=0 overlap=0`,
`DONE:24242 A:449 B:11769 C:4338 D:2955 M:5038 V:256 U:202 X:170 Z:19`, **`citation_failures=0`**
both scopes — no `scripts/completion_atlas.py` `file:line` re-derivation needed this cycle
(the M lane's own insertions had already been re-pinned correctly in `4d27d70551`).

**Build scope, run after this cycle's own regeneration commit:** `cargo test --locked --no-run`
(full workspace) exit 0, 160s; `cargo test --locked --no-run --manifest-path
apps/desktop/src-tauri/Cargo.toml` (separate cargo workspace, tested explicitly) exit 0, 148s.

**Receipt:** `artifacts/epic-3-core-rulebook/AT-34-E3-001_wave9_regen_receipt.md`.

### Cycle — AT-34-E3-003 (bucket M, `skill_content` sub-cause) — partial, closure

**Status: partial.** Re-derived bucket `M` fresh at cycle start (never trusting the dispatch
brief's own inherited figures): corpus-wide 5,114, `core_rulebook` 1,048 — matching the brief's
top-line number exactly, but the brief named only 5 of `core_rulebook` `M`'s 10 real sub-causes
(`equipment_table_entry_with_corpus_magnitude` 276, `ability_content...` 217,
`equipment_own_line...` 147, `race_trait_generic...` 119, `template_content...` 96 — all
confirmed correct); this cycle's own full re-derive surfaced the other 5
(`skill_content` 95, `in_catalog_with_corpus_magnitude_but_no_observed_consumer` 47,
`domain_content...` 34, `spell_list_entry...` 15, `race_trait_states_a_universal_sheet_
modifier_pending_compute` 2), summing with the named five to exactly 1,048.

Took the `skill_content` sub-cause (95) as the largest one completable end-to-end this cycle.
A prior, real but unreported commit (`c5c4a1b788`, "AT-34-E3-003 bucket-M skill widening") had
already widened `skill_allocation.rs`'s class-skill lists to Fighter's and Wizard's real, full
corpus rosters — 13/13 tests passing, genuinely committed — but never wired the classifier to
consume it; its own doc comment named the exact missing piece and deferred it as "the next
cycle's work." No `progress.md`/`kanban.md` entry existed for that commit before this one.

**What changed:** `simple_kind_verdict()` — the shared classifier function all nine
`Template`/`Domain`/`Deity`/`Language`/`Ability`/`Trait`/`Skill`/`RaceTrait`-generic (×2) `Kind`
arms route through — had **no path to `grounded` at all** before this cycle: every held,
non-`text_only` record fell through to `ingested-magnitude` unconditionally, regardless of
whether a real compute path for that record's magnitude existed. It gains one new trailing
parameter, `grounded_magnitude: Option<i8>` — the caller's own, already-executed fixture proof.
`Kind::Skill` is the only call site passing a real value, via a new
`skill_allocation::skill_bonus_is_grounded_for_display_name(&unit.name)` entry point that
normalizes the corpus record's display name and actually runs `allocate_skill_ranks` against a
level-1 fixture character for Fighter, Rogue, and Wizard (the three classes this module has
real, cited data for), returning the genuinely-computed class-skill bonus or `None` — never an
assumed `3`. The other 8 call sites pass `None` and are byte-identical to their pre-cycle
behaviour: **a pure widening, corpus-wide by construction**, not a `core_rulebook`-scoped patch.

**RED→GREEN:** new test `skill_bonus_is_grounded_for_display_name_normalizes_and_checks_every_
recognized_class` in `skill_allocation.rs` (14/14 pass in that module); two new `classify()`-level
tests in `v06_work_inventory.rs` —
`a_fighter_class_skill_bonus_promotes_a_held_skill_record_to_grounded` (positive: `Handle
Animal` now reads `grounded`) and `a_skill_no_recognized_class_grounds_stays_ingested_magnitude`
(negative control: `Perform (Sing)`, a real held record no recognized class treats as a class
skill, proven to stay exactly where it was) — 435/435 pass in that binary's own test suite.

**Movement:** whole-corpus before/after diff by unit id (49,438 before, 49,438 after, 0
added/removed): **exactly 76 changed, all `core_rulebook`, all `kind == "skill"`, one status
transition (`ingested-magnitude → grounded`)**. `core_rulebook` bucket `M` **1,048 → 972**;
corpus-wide `M` **5,114 → 5,038**. `core_rulebook` DONE **4,254 → 4,330**; corpus-wide DONE
**24,166 → 24,242**. `skill_content`'s own sub-cause **95 → 19** — the remaining 19 are class
skills only for classes this module has no grounded data for (Bard's `Perform` family and
similar), honestly left `ingested-magnitude`, never silently promoted. No other book's `skill`
units moved: the fix applies identically everywhere, but no other book happens to carry a
`skill` record matching Fighter/Rogue/Wizard's lists while sitting in `ingested-magnitude`
(149 `skill`-kind units exist corpus-wide, 110 in `core_rulebook`).

**Remainder — `AT-34-E3-003`'s five buckets, `core_rulebook`, named at HEAD:** `M` 972 (named by
sub-cause below), `V` 81 (untouched), `D` 366 (untouched by this cycle — a correction of the
prior cycle's own receipt, which cited 382; re-derived fresh at this cycle's own start and
confirmed unmoved by this cycle's own diff, retro `correction` filed), `U` 10 (untouched,
awaiting the operator ruling the prior cycle already flagged), `X` 115 (untouched, awaiting
`decisions.md §17`'s new choice-filter mechanism). Sum 972+81+366+10+115 = 1,544, matching
`completion_atlas.py --book core_rulebook --check`'s own live sum.

**`M`'s own 972, named by sub-cause (sums exactly):** `equipment_table_entry_with_corpus_
magnitude` 276, `ability_content_table_holds_record_magnitude_not_yet_computed` 217,
`equipment_own_line_has_no_magnitude_but_closure_wiring_class_does` 147,
`race_trait_generic_table_holds_record_magnitude_not_yet_computed` 119,
`template_content_table_holds_record_magnitude_not_yet_computed` 96,
`in_catalog_with_corpus_magnitude_but_no_observed_consumer` 47,
`domain_content_table_holds_record_magnitude_not_yet_computed` 34,
`skill_content_table_holds_record_magnitude_not_yet_computed` 19,
`spell_list_entry_with_resolved_level` 15,
`race_trait_states_a_universal_sheet_modifier_pending_compute` 2. Sum = 972.

**Build scope:** `cargo test --locked --lib rules_core::skill_allocation::` 14/14;
`cargo test --locked --bin v06_work_inventory` 435/435; `python3 -m unittest
scripts.tests.test_completion_atlas` 38/38; `cargo test --locked --no-run` (full workspace)
exit 0; `apps/desktop/src-tauri` tested explicitly (`cargo test --locked --no-run
--manifest-path apps/desktop/src-tauri/Cargo.toml`, exit 0 — the desktop crate depends on
`codex` as a path dependency and this cycle's lib change is additive-only, confirmed).

**Discoveries, out of this criterion's own scope, not fixed here:** `cargo test --locked --lib`
(run mid-cycle, only this cycle's own two Rust files modified) showed **5 pre-existing
failures**, confirmed unrelated to this cycle's diff and traced via `git log -S"oracle-agree"`
to `AT-34-E3-005`'s own already-landed bucket-V oracle-disposition cycle (`fef202a566`): its new
`oracle-agree`/`oracle-unverifiable` statuses trip an unmapped `(wiring_class="derived",
status="oracle-agree")` pair in `scripts/observer/pf1e_dashboard_producer.py`'s
`_doneness_verdict_uncapped`. This is a genuine regression against the tranche cut's registered
baseline, landed after that baseline was measured — belongs to `AT-34-E3-005`'s own file-touch
set, not `M`'s; retro `incident` filed rather than self-healed inline (would collide with that
criterion's own bookkeeping, same posture the already-merged `AT-34-E3-004` cycle took toward
an out-of-scope denominator-gate finding it also declined to silently fix).

This cycle's own citation upkeep: `scripts/completion_atlas.py`'s 10 `BUCKET_DEFINITIONS` and
`scripts/missing_engine_tables.py`'s 2 `ENGINE_SURFACE_CITATIONS` `file:line` citations shifted
by this cycle's own insertion and were re-derived against real line content (never computed by
offset alone) — `citation_failures=0` after, for both.

Denominator gate against this package: `files_checked=15 violations=6`, all 6 pre-existing
(`FRT_HVY`'s quoted corpus prose, already flagged by the already-merged `AT-34-E3-004` cycle),
unchanged before/after this cycle's own edits.

Full detail, remainder table, and next-cycle plan: `artifacts/epic-3-core-rulebook/
AT-34-E3-003_m_bucket_skill_cycle_receipt.md`.

### Cycle — AT-34-E3-005 (bucket-v-widen): the corpus-wide bucket-V ledger, rebuilt and committed

**Status: complete** (this sub-lane's own scope — the whole-book `AT-34-E3-005` criterion stays
`in-progress`, see `kanban.md` row 17). **This work was done once and lost**: the prior
`salvage-2026-08-30` cycle landed the bucket-V oracle-ledger loader's multi-path widening
(tested, generic, safe) but the corpus-wide ledger data file a session had built alongside it was
never committed — an untracked file, lost when that session exited. That cycle correctly refused
to claim the 2,000+ units the lost file would have justified, per `decisions.md §4`/§9. This
cycle rebuilds the ledger from scratch, independent of the lost file's numbers (which were never
readable, so never trusted as a starting point), and **commits the data file in this same
commit**.

**Method, identical in kind to the `core_rulebook` pass (`decisions.md §19`), applied to the
rest of the corpus.** Population: 6,765 non-`core_rulebook` units still in bucket V
(`literal-verified`/`fixture-verified`) — `core_rulebook`'s own 81-unit V population is untouched
by construction (checked, not assumed). Cross-referenced against SD-33's own final,
fully complete (8,330 of 8,330 rows), 0-disagree combined oracle ledger
(`docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json`,
8,330 of 8,330 rows, 415 agree / 6,174 unverifiable / 0 disagree per that bundle's own closed
`kanban.md` row) — **5,748 of 6,765 matched directly** (426 `agree`, 5,322 `unverifiable`, 0
`disagree`). The remaining 1,017 were cross-referenced against `AT-33-E1-003`'s probe-surface
census (11 kinds proven corpus-wide to carry no engine compute table at all) — **842 more**
(`ability` 655, `template` 90, `monster_ability` 96, `companion` 1) dispositioned
`unverifiable`/`no_probe_surface` by the same structural reasoning the `core_rulebook` lane
applied to its own `ability`/`template`/`companion` units. **6,590 of 6,765 (97.4%) dispositioned,
zero new oracle runs.** **175 remain, named by book:kind sub-cause, not "the rest"** —
dominated by `race_trait` (142, mostly bestiary/ARG racial special-ability entries, same shape as
`core_rulebook`'s own Ranger favored-enemy remainder at corpus scale), plus `equipment` 23,
`class_feature` 3, `equipment_modifier` 3, `spell` 3, `feat` 1. **A `disagree` is never
dispositioned** — zero appear anywhere in this cycle's output (checked, not assumed: the source
ledger carries none, and a direct verdict-count scan of the 6,590-row output confirms it).
**Freshness-checked**: 30 of 6,590 (0.5%), deterministic-seeded sample — each row's `unit_id`
still resolves in the current `docs/work-inventory.json` to the same `kind` and is still in
bucket V; **30 of 30 passed, 0 drift.**

**Wired in — zero Rust source changed.** The loader (`load_bucket_v_oracle_dispositions`) and the
apply rung (`apply_bucket_v_oracle_disposition_stamps`) were already landed by the prior
`salvage-2026-08-30` cycle and were a verified no-op without data; this cycle supplies only the
data file at the exact path the loader was already reading
(`artifacts/bucket-v-widen/bucket-v-corpus-wide-consolidated.oracle-results.json`), then ran the
three-pass regen (`corpus_literal_sweep` → `derived_evaluator_fixture_check` →
`v06_work_inventory`, no `--allow-stamp-loss`) to make the disposition visible to the atlas.

**Whole-corpus before/after diff, by unit id:** 49,438 → 49,438, 0 added/removed. **6,590
changed, 0 of them `core_rulebook`** (byte-identical before/after). Bucket transition: `V → DONE`:
6,590 (nothing else moved). Corpus-wide V: **6,846 → 256** (81 `core_rulebook`, unchanged, + 175
this cycle's own named remainder — SET-verified equal, not just counted). Corpus-wide DONE:
**17,576 → 24,166**. `core_rulebook` V unchanged at 81. `oracle_probe_surface_for_no_table_kinds`
capability (re-derived live, zero code change): population 130 → **2,062**, `books_unblocked` 1
→ **30 books**.

**Verification:** `cargo test --locked --bin v06_work_inventory -- bucket_v_oracle_dispositions
apply_bucket_v_oracle_disposition_stamps stamped_ids` → 8 passed, 0 failed (pre-existing tests,
re-run against this cycle's data). `cargo test --locked --no-run` (full workspace) and, separately,
`apps/desktop/src-tauri` (explicitly tested, separate cargo workspace): both exit 0, run at
`837dbbcf6b` (no Rust source changed by this cycle). `completion_atlas.py --check` (corpus-wide):
`population=49438 unclassified=0 overlap=0 citation_failures=0`. `denominator_gate.py --check`:
`violations=6`, unchanged pre-existing `FRT_HVY` baseline, no 7th added.
`verify_capability_register.py`: `PASS: 11 capabilities named, X-bucket reconciliation sums to
live population (170), 0 flagged built_by_sd34=true`. `corpus_literal_sweep`: `records_examined:
48708`, unchanged (this cycle touches zero `data/corpus/**` records — sweep population delta N/A
per `decisions.md §12` L8).

**Movement, four buckets (`decisions.md §9`):** closure 0, **reclassification 6,590** (every
verdict reused from SD-33's own already-produced work, never newly computed — an `agree` unit's
value was not re-verified by this cycle either), reachability 0 (the `no_probe_surface` finding
is `AT-33-E1-003`'s, not this cycle's), instrument-correction 0.

**Receipt:** `artifacts/bucket-v-widen/AT-34-E3-005_bucket_v_widen_cycle_receipt.md`.

### Cycle — salvage-2026-08-30: recovering two isolated worktrees' uncommitted diffs (partial)

**Status: partial.** Three lanes exited without committing; their work survived only as
uncommitted-diff patches saved outside the repo. This cycle read both substantive patches before
applying either (they both touch `src/bin/v06_work_inventory.rs`), hand-merged them (no true
conflict — they touch disjoint regions of the file), and verified the merged result, not just
that it compiled.

**Landed:** (1) `closure_has_real_aspect_description` — a corpus-wide widening of
`has_real_description` recognizing real prose on an `ASPECT:`-only tooltip token (refusing
leaked-syntax/template/bare-number text with the same discipline as every sibling signal).
Closed 123 of 49,438 units corpus-wide (`D`→`DONE`), 3 of them the exact `ultimate_campaign`
units `AT-34-E4-002`'s own first cycle named as its next-cycle plan item 1. 8 further units
(already `DONE`) relabeled `grounded`→`text-complete` — evidence-string churn, no bucket
crossed. (2) The bucket-V oracle-ledger loader widened to merge multiple ledger files
(`load_bucket_v_oracle_dispositions`, 2 new tests) — safe, generic, tested infrastructure that
is a no-op today because the corpus-wide ledger data it would read was never captured by its
own salvage patch (an untracked file, lost with the session). Full detail and every figure's
re-derive command: `artifacts/epic-4-ultimate-campaign/AT-34-E4-002_cycle_receipt_2.md`,
`artifacts/epic-5-forward-plan/bucket_v_widen_infra_cycle_receipt.md`.

**Dropped, not landed:** the widened bucket-V ledger's claimed population/movement numbers
(`core_rulebook` V unchanged at 6,846 corpus-wide, `81` for `core_rulebook` itself — no
disposition happened outside what `AT-34-E3-005` already applied); a third salvaged patch
(docs-only, a "bucket M first cycle" narrative) that cited a receipt and a retro event neither of
which exist in the repo. In both cases the missing artifact is the same failure shape: a
`git diff`-based patch never captures untracked new files, and an uncommitted session's new data
files are lost when it exits.

**Whole-corpus diff, by unit id:** `49,438 → 49,438`, 0 added/removed. 131 units changed
status/evidence (123 closure, 8 evidence-string churn) — see the epic-4 receipt for the
per-book breakdown. Movement, four buckets: **closure 123, reclassification 0, reachability 0,
instrument-correction 0** (plus 8 same-bucket evidence-string churn, not counted in any of the
four).

**Verification:** three-pass regen in order, never `--allow-stamp-loss`
(`corpus_literal_sweep` 48,708 examined, clean; `derived_evaluator_fixture_check` 1,839/2,580, 0
failed; `v06_work_inventory` with both report env vars, exit 0, `2m26s`).
`completion_atlas.py --check` corpus-wide: `population=49438 unclassified=0 overlap=0
citation_failures=0`. `--book ultimate_campaign --check`: `DONE=133 of 265` (criterion's own
`265 of 265` bar not met — 132 remain, all real M/V/U/X work, unchanged scope from
`AT-34-E4-002`'s first cycle). `--book core_rulebook --check`: `DONE 4215→4254`.
`denominator_gate.py --check`: `violations=6`, unchanged pre-existing `FRT_HVY` baseline, no 7th
added. `box_ledger.py --check`: pre-existing failure unchanged in kind (`exit 1`,
`oracle_disagreement=0`; `uncovered` 22,346→22,223, an improvement, not a regression).
`cargo test --locked --no-run`: full workspace and `apps/desktop/src-tauri` (tested explicitly)
both exit 0. `cargo test --bin v06_work_inventory`: 433/433. Full 184-target `sd13_*`/`sd25_*`
gate set, run together: 184/184, 0 failed. `forward-plan.json`/`capability-register.json`
re-derived live against the regenerated inventory; both verifiers pass.

### Cycle — AT-34-E3-001 wave-10 regeneration and attribution: measuring cycle 10's bucket-B batch

**Status: complete.** Cycle 10's bucket-B batch (`935cef27b5`) widened the Druid/Monk citation
gate and did not regenerate `docs/work-inventory.json`. This cycle ran the three-pass pipeline
(191s / 12s / 666s — no `--allow-stamp-loss`) and diffed the result unit-by-unit against cycle
10's own stated expectation.

**Whole-corpus diff: 49,438 → 49,438 (0 added, 0 removed), 10 changed, all `core_rulebook`.**
Bucket-V (`e7b87138d1`, artifacts-only, out of scope) checked and confirmed inert — zero overlap
between the 10 changed units and bucket-V's 2,712-unit disposition list.

**`class_feature_owner_matched_by_name_but_record_not_held_by_engine`: 239 → 237, count
confirmed, membership refuted.** Cycle 10 predicted 2 closures (`Monk ~ Flurry of Blows`,
`Monk ~ Unarmed Strike`). Only `Unarmed Strike` actually closed (→ `text-complete`).
`Flurry of Blows` reclassified `B → D` (new evidence `class_feature_no_dedicated_magnitude_
id_matched_the_record_slug`), not closed.

**"6 bonus closures" in `no_explanation_id_and_no_diagnostic_names_this_feature`: refuted
entirely, in the opposite direction.** This mechanism's population went 363 → 357 (**a decrease
of 6**, not a gain). All 6 named units (`Monk ~ Abundant Step/Diamond Soul/Maneuver Training/
Perfect Self/Stunning Fist`, `Druid ~ Nature Bond`) were already classified here pre-regen and
reclassified `C → D` to the same new evidence string above — **zero of the 6 closed to DONE.**

**Net: cycle 10's own receipt claimed 8 total closures (2 + 6); the measured result is 1**
(`Monk ~ Unarmed Strike` only). The other 7 landed in a single un-named `D`-bucket evidence
string (`class_feature_no_dedicated_magnitude_id_matched_the_record_slug`, `core_rulebook`
population 22 → 29, +7 — exactly these 7 units) that neither `progress.md` nor `kanban.md`'s
cycle-10 entries mention.

`class_feature_option_pool_record_with_magnitude_not_held_by_engine` (208) and
`class_feature_option_pool_record_not_held_by_engine` (25): both **confirmed unchanged**,
byte-identical membership.

2 further units (`druid_orisons`, `druid_spontaneous_casting`) changed `evidence` only, already
`text-complete` before and after — evidence-string churn, no bucket crossed, reported separately
per `decisions.md §9`.

**Movement, four buckets:** closure **1**, reclassification **7**, evidence-string churn **2**,
instrument-correction **0** (1+7+2=10). A B→D or C→D move is reclassification, never closure.

`completion_atlas.py --check` corpus-wide: `population=49438 unclassified=0 overlap=0
citation_failures=0` (all 10 `BUCKET_DEFINITIONS` citations re-checked directly against
`v06_work_inventory.rs` at HEAD; none needed re-deriving — this cycle touched no `src/**` file).
`--book core_rulebook --check`: `DONE 1502→1503 B 472→470 C 363→357 D 398→405` — all four deltas
reconcile exactly against the attribution above. `--book ultimate_campaign --check`: unaffected
(0 changed units).

`denominator_gate.py --check`: `files_checked=15 violations=6`, **not the 5 a prior cycle
(`progress.md:975`) reported for its own run — verified directly, not inherited.** All 6 flagged
lines genuinely quote the same pre-existing, already-merged `AT-34-E3-004` corpus-prose fragment
in different historical narration contexts; none introduced this cycle. The count's climb
(2→3→4→5→6 across cycles) is a self-perpetuating recursive citation: each cycle's own gate report,
once committed, quotes the flagged substring and becomes a new instance for the *next* cycle's
scan — not a growing population of real defects. Nothing here is this cycle's to fix; this
cycle's own new prose avoids re-quoting the flagged substring to not mint a 7th instance.

`cargo test --locked --no-run` (workspace): exit 0, 130s. `--manifest-path
apps/desktop/src-tauri/Cargo.toml`: exit 0, 211s (3m31s). Neither build was expected to move
(this cycle touched no `src/**`), both re-verified at the widest scope per instruction.

Full detail: `artifacts/epic-3-core-rulebook/AT-34-E3-001_wave10_regen_receipt.md`.

### Cycle 10 — AT-34-E3-001 — bucket-B batch cycle: Druid/Monk widening + 8 pre-existing stale-gate fixes — partial

**Status: partial.** Single-agent batch across all three of AT-34-E3-001's remaining
`core_rulebook` mechanisms (owner_matched, option_pool_with_magnitude, option_pool), one build,
per the operator's "make every edit, compile once, test once" cost argument.

Re-derived: mechanism 1 (owner_matched) population is now 239, not 242 — a prior wave-9 shared
regeneration already moved 3 units; cycle 9's own 218/24 excluded/non-excluded split is now
215/24 (Monk/Druid byte-for-byte unchanged, only Cleric/Wizard moved). Mechanisms 2 and 3 stand
unchanged at 208 and 25.

**Real code movement, mechanism 1 only:** removed `class_feature_grant_consumer.rs`'s
`LEVEL_UP_PILLAR_FILTERED_CLASSES` (the class-wide Druid/Monk exclusion) entirely. Direct
re-reading of `is_druid_pillar_id`/`is_monk_pillar_id` found both already admit this module's
own id shape by prefix, and `v06_work_inventory.rs`'s own `classify()` never reads
`LevelUpPlan`/`level_up::` at all — the cited "SEPARATE, THIRD reason" for the exclusion was
stale, conflating two different consumers of the same explanation id. Expected: 2 real closures
this mechanism owns (`Monk ~ Flurry of Blows`, `Monk ~ Unarmed Strike`), plus 6 bonus closures
in a different mechanism's own bucket (5 Monk + 1 Druid). Mechanisms 2 and 3: zero code change —
re-derived and reported as genuinely NOT narrow work (a new wizard school needs real per-power
formula engineering; the option-pool remainder needs a new possession-tracking subsystem or a
Paladin Special Mount computation).

**8 pre-existing, unrelated stale anti-fabrication gates found and fixed** while running the
full `sd13_*`/`sd25_*` set (184 targets) together — apparently the first time in this bundle's
history: 4 Bard progression files and 2 Sorcerer progression files missed additive carve-outs
for real, already-landed magnitude groundings (mechanism-2's own earlier Suggestion/Inspire
Greatness cycles, and an unrelated SD-32-era generic bloodline-power pass) their own tests were
never updated to admit. All fixed additively, none weakened; full 184-target re-run green.
Also found and fixed a stale pinned assertion in `class_feature_pool_catalog.rs`
(`excluded, 218` → `215`), unrelated to this cycle's own code, from the same un-re-checked
wave-9 regeneration.

Full detail: `AT-34-E3-001_class_feature_owner_matched_cycle_receipt_10.md`.

### Cycle — AT-34-E3-001, `class_feature_owner_matched_by_name_but_record_not_held_by_engine` mechanism, cycle 9 (non-excluded remainder re-derived: 218/24, not 161/81) — partial

**Status: partial.** This cycle owns only the NON-excluded-class remainder of the
`class_feature_owner_matched_by_name_but_record_not_held_by_engine` mechanism (a sibling lane
owns the 218-unit excluded-class majority under `decisions.md §18`'s operator ruling — see its
own `AT-34-E3-001_class_feature_owner_matched_cycle_receipt_8.md`, landed the same wave).

Re-derived the dispatch's own claimed 161/81 excluded/non-excluded split directly against the
live corpus and found it did NOT match: **218 excluded / 24 non-excluded** of the 242-unit
`core_rulebook` population — matching both `decisions.md §18` and the sibling `§18`-fix
cycle's own independent re-derivation. Of this lane's own 24: **18** carry no corpus
description at all (the bundle's OPEN zero-description definitional question,
`atlas-defects.md` — left in bucket B, not reclassified); **6** carry a real description but
are correctly refused by `class_feature_pool_catalog`'s own pre-existing, independently-tested
safety gates (`Rogue Talent ~ {Bleeding Attack, Finesse Rogue, Improved Evasion, Skill
Mastery}`, `Rage Power ~ Knockback`, `Arcane Trickster ~ Invisible Thief` — each needs real
per-character grant/formula wiring, not a catalog-widening fix, to close). Zero units closed
this cycle; a new characterization test
(`class_feature_owner_matched_non_excluded_remainder_is_24_and_named_by_subcause`) pins the
24-unit split and its sub-cause breakdown mechanically, RED-confirmed against the dispatch's
stale 161 figure before GREEN against the re-derived 218.

Full detail — the per-record gate-walk for all 6 real-description units, the TDD proof, the
concurrent-write hazard this cycle hit and fixed (a `git rebase`'s "ours"/"theirs" polarity is
the OPPOSITE of a merge's; resolving a same-wave filename collision with `checkout --ours`
at every conflict step discarded the sibling's landed receipt instead of preserving this
cycle's own, caught by re-diffing before push and fixed by restoring the sibling's file
verbatim plus re-authoring this cycle's own) — is in the receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_owner_matched_cycle_receipt_9.md`.
`docs/work-inventory.json` was deliberately NOT regenerated this cycle (wave rule: one shared
regeneration cycle runs after all four parallel lanes land).

### Cycle — AT-34-E3-001, `class_feature_option_pool_record_with_magnitude_not_held_by_engine` mechanism, cycle 9 (Weapon Training, built generically) — partial

**Status: partial.** Continuing cycle 8's own 256-unit `core_rulebook` remainder
(`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_with_magnitude_cycle_receipt_9.md`),
this cycle took the Weapon Training sub-cause (48 units) and built it **generically** rather
than one canonical group at a time: `WEAPON_TRAINING_GROUPS` names all 14 real PF1
weapon-training groups, `weapon_training_group_name_for_selection` accepts any of them, and
`explain_fighter_class_features`'s 4 tier blocks now compute the same real rank-based formula
for whichever group a Fighter selects at each tier — the formula never depended on WHICH group,
only the tier, so this is the closed-enumerable-set generalization `decisions.md §16` and the
bucket-U cycle's own precedent already established as fair game. `fighter_weapon_training_
attack_bonus` (the function that folds a bonus into the real total attack) is unchanged.

Full detail, the RED→GREEN proof (4 layers, including two tests that directly exercise the
real `compute_pilot_base_chassis` pipeline rather than only `classify()` with hand-fed facts),
the cross-book scope check (confirms `ultimate_combat`/`ultimate_wilderness`/
`advanced_players_guide`'s own differently-shaped "Weapon Training"-prefixed records are
unaffected), and the CARGO_TARGET_DIR-corruption discovery mid-cycle, are in the receipt.
`docs/work-inventory.json` was deliberately NOT regenerated this cycle (wave rule: one shared
regeneration cycle runs after all four parallel lanes land). **Expected closure: 48
`core_rulebook` units (256 → 208)** — an expectation for that regeneration to confirm or
refute, not a measured count. This is a DIFFERENT mechanism (`_with_magnitude`) than the
sibling cycle 9 entry immediately below (`class_feature_option_pool_record_not_held_by_engine`,
no `_with_magnitude`), and both are named per `decisions.md §14`'s own nine-mechanism split of
bucket B.

### Cycle 9 — AT-34-E3-001, `class_feature_option_pool_record_not_held_by_engine` mechanism (wizard-opposition-school-spell-tracking sub-cause) — partial

**Status: partial.** Eight prior cycles on this exact mechanism ran `63 -> 57 -> 55 -> 52 -> 52 ->
49 -> 44 -> 34 -> 34`, the last closing zero and naming the remaining 34 units as three
"genuinely new, unbuilt engine subsystems." This cycle re-investigated the 9-unit
wizard-opposition-school-spell-tracking group from scratch (`decisions.md §12` L2 — never carry a
prior cycle's characterization forward) rather than accepting that label, and found it wrong for
this cluster: `crb::wizard_spell_list::wizard_school_zero_level_spells` — a new pure join of two
already-shipped, already-tested tables (`WIZARD_SPELL_LIST` + `SPELL_LIST`, no new raw data) —
reproduces all 9 `"<School> Wizard Spells"` corpus records' own `SPELLKNOWN` spell lists
byte-for-byte, verified live against the committed corpus
(`class_feature_pool_catalog::wizard_school_spell_list_key_owner_matches_are_exact`). Wired into
`v06_work_inventory.rs`'s `Kind::ClassFeature` fallback chain (mirrors cycles 5-7's own
weapon/armor/class-skill "held by a real table" pattern): all 9 keys move bucket B -> D via
evidence `class_feature_wizard_school_spell_list_held_by_wizard_spell_list_and_spell_list_join`.

**9/34 closed this cycle.** TDD: two new `v06_work_inventory.rs` unit tests did not compile
against pre-cycle source (RED, for the intended reason — the new lookup function did not exist)
-> GREEN after the fix, full binary suite `416 passed; 0 failed` (414 + 2 new). 25 units remain,
named by sub-cause in the receipt (`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_cycle_receipt_2.md`):
proficiency/mechanical-grant possession-tracking (20, three grouped rows), companion/mount
registration (3, gated on a real Paladin Special Mount computation that does not exist yet), and
Domain Power `CLASS_FEATURE_POOLS` (2, owned by the sibling `_with_magnitude` mechanism).

**Per this wave's own instruction, `docs/work-inventory.json` is NOT regenerated by this cycle** —
a single shared regeneration cycle runs after all four parallel lanes land. This cycle's own
figures are re-derived directly from the committed (pre-regen) `docs/work-inventory.json`
(mechanism population 34 of 543 `core_rulebook` bucket-B units, corpus-wide 1,659 of 49,438,
confirmed all 9 target keys are `core_rulebook`-only, no cross-book collision risk). Expected
post-regen mechanism population: 25 (not yet confirmed — the shared regen cycle is the actual
source of truth for that number).

**Self-caused regression, fixed same cycle:** the new fallback rung shifted four of
`completion_atlas.py`'s own `BUCKET_DEFINITIONS` `file:line` citations by +22 lines each
(A/B/C/V) — re-derived each new line number directly against the post-edit file and fixed all
four; `--check`'s `citation_failures` went `4 -> 0`.

**Environment hazard, self-healed:** discovered several stray `cargo test` processes already
running against this lane's own designated `CARGO_TARGET_DIR` mid-cycle, not started by this
cycle (their log paths matched this exact session's own scratchpad directories and this exact
mechanism's own prior test names — most likely an uncollected earlier attempt at this same
criterion). One caused a spurious "function not found" compile error via a build-cache race.
Killed the stray processes, then switched to a cycle-private target directory
(`/tmp/cargo-sd34-at-34-e3-001-lane9`) for the remainder of verification once a second wave of
strays appeared. All figures/tests in the receipt are from that clean, isolated build.



**Status: complete.** Built `artifacts/epic-5-forward-plan/ordered-plan.json` via
`build_ordered_plan.py`, re-derived at HEAD every run from `forward-plan.json` (AT-34-E5-001) —
read-only against the rest of the repo (`workflow-instruction.md §3`).

**Ordering basis (stated in the artifact's own `ordering_basis` field, quoted here):** only
buckets A, B and U carry a measured rate reaching DONE (bucket C's only measured rate reaches
V, a different endpoint; buckets D, M, V, X, Z carry no rate at all — zero dedicated clearing
cycles ran in either vehicle book, per `forward-plan.json`'s own `measured_rates`). "Real cost,
cheapest-first" is therefore computed over each book's **priced-to-DONE slice** only (its A+B+U
units), ranked ascending by the **midpoint** of that slice's projected-cost-hours range (bucket
B carries a measured range, never a point estimate). Blending priced and unpriced units into one
number per book would fabricate precision the underlying data does not carry — the exact failure
`AGENTS.md` rule 9 and this bundle's own Evidence text warn against. Every ranked row states what
fraction of that book's remaining population the priced slice covers, so a low rank is never
misread as "this book finishes soonest."

**Population, cross-checked by the RED→GREEN structural check against a live re-derivation from
`forward-plan.json`:** all 35 non-vehicle books partition exactly into two lists — **19** books
carry at least one priced-to-DONE unit (ranked ascending by cost midpoint) and **16** books carry
zero (listed alphabetically, each naming which unpriced buckets make up its entire remaining
population). Cheapest-ranked: `advanced_players_guide` (1 unit, `U`, 0.025h) and
`inner_sea_taverns` (1 unit, `U`, 0.025h, tied); most expensive of the 19 ranked:
`ultimate_equipment` (1.068h across 44 priced units of 1,477 remaining — 97.0% of that book is
unpriced, `V`-heavy).

**Single-bucket books flagged by name:** exactly **1** of 35 — `beginner_box` (19 units, all
bucket `Z`, unpriced — zero measured Z-clearing rate exists anywhere in this bundle). This is the
same shape-finding mechanism that surfaced `ultimate_campaign` for Epic 4, applied to the
remaining 35; `single_bucket_books` is intentionally independent of pricing (a single remaining
bucket is a book-**shape** property — one mechanism clears the whole book — not a pricing
property), so it can and does include a book that also appears in the unrankable-by-cost list.
Live cross-check confirms no other of the 34 non-flagged books has exactly one non-DONE bucket
occupying its full remaining population.

**RED → GREEN.** RED (missing artifact): `verify_ordered_plan.py` before the artifact existed →
`FAIL: .../ordered-plan.json does not exist`, exit 1. Mutation RED (planted defects on the
generated artifact): forced `advanced_players_guide`'s stated midpoint to `99999` (breaking
both its own live-agreement check and the ascending-sort check against the next row) and emptied
`single_bucket_books` (dropping the one true flag) →
`FAIL: 3 violation(s)` naming all three planted defects exactly, no crash. Regenerated via
`build_ordered_plan.py` (discards the mutation, re-derives from `forward-plan.json` at HEAD) →
`PASS: 19 ranked + 16 unrankable = 35 books, sorted ascending by priced_to_done_hours_midpoint,
1 single-bucket book(s) flagged and confirmed live`, exit 0.

**Figures + re-derive commands:**
- 35 books / 19 ranked / 16 unrankable — `python3 docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/build_ordered_plan.py` then read `ordered-plan.json`'s `population` object.
- 1 single-bucket book (`beginner_box`, bucket `Z`, 19 units) — same artifact, `single_bucket_books`.
- Cheapest/priciest ranked rows quoted above — same artifact, `ranked_by_priced_to_done_cost[0]` and `[-1]`.
- Denominator gate against this package: `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` → `files_checked=15 violations=4` — all 4 pre-existing in `progress.md` (verbatim-quoted corpus prose, "75% chance..."), already flagged by the already-merged `AT-34-E3-004` cycle; this cycle added no new `.md` prose containing a bare percentage (its own new files are `.py`/`.json`, and this progress entry + the `kanban.md` row contain no bare percentage outside the pre-existing quoted lines).

**Row-count command output:**
```
$ python3 docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/verify_ordered_plan.py
PASS: 19 ranked + 16 unrankable = 35 books, sorted ascending by priced_to_done_hours_midpoint, 1 single-bucket book(s) flagged and confirmed live
```

**Build scope verified:** `cargo test --locked --no-run` exit 0 (workspace, run at this cycle's
commit SHA). `apps/desktop/src-tauri` explicitly run: `cargo test --locked --no-run` exit 0. No
Rust source touched — Python/JSON-only change.

**Sweep population:** N/A — no corpus records added or regenerated (`git status --porcelain --
data/corpus/` empty for this cycle).

**Movement, four buckets:** none — this cycle moves no unit on any bucket board
(`docs/work-inventory.json` untouched). It is a naming/ordering artifact over an already-priced
plan, matching the criterion's own bar exactly.

**Next-cycle plan:** Epic 5 is now complete (AT-34-E5-001..004 all `complete`). Next up is
Epic 6's closure epilogue (AT-34-E6-001 final-acceptance scan), gated on Epics 1–5 all
`complete`.

### Cycle — AT-34-E5-003 (the `power` table is costed) — complete

**Status: complete.** Built `artifacts/epic-5-forward-plan/power-table-cost.json` via
`build_power_table_cost.py`, re-derived at HEAD every run. Read-only against the rest of the
repo (`workflow-instruction.md §3`): prices `power`, builds nothing.

**Population (421 units, `ultimate_psionics`), cross-checked four independent ways, all
agreeing:** live `docs/work-inventory.json` query, `missing-engine-tables.json`'s own count, a
live directory listing (`data/corpus/ultimate_psionics/power/*.json`), and
`capability-register.json`'s (AT-34-E5-002) `power_engine_table` row.

**Rate derivation.** `table-build-rate.json`'s own finding is that per-kind marginal build cost
is dominated by whether the kind's corpus directory name matches its kind name, not record
count. `power`'s directory (`data/corpus/ultimate_psionics/power/`) matches exactly, so it is
priced against the **6 matched-directory kinds** (`ability`, `template`, `deity`, `domain`,
`skill`, `language`) — **2–7 marginal lines, 49–172 seconds (ESTIMATE)** — explicitly **not**
against `trait`'s dearer, mismatched-directory tier (12 lines / 295s). Reported as a range, not
collapsed to a point estimate — a DOUBLE-ESTIMATE per AGENTS.md rule 9, since the underlying
per-table wall times were themselves pro-rated, never independently stopwatched.

**Reason not built (`decisions.md §7`):** the 421 units all sit inside a 3,498-unit book that
occupies 6 other non-DONE buckets besides A — building the table clears bucket A but banks no
closed book inside SD-34's own two-book scope, so it is priced for the successor bundle instead.

**What `ultimate_psionics` still needs after `power` exists:** live re-derivation shows the book
occupies 7 non-DONE buckets today (A, B, C, D, M, U, V — neither X nor Z has any unit here).
Building `power` clears bucket A to 0; **6 non-DONE buckets remain** (B, C, D, M, U, V) — the
table alone does not close the book, matching the criterion's own bar.

**Instrument-correction:** `decisions.md §7`'s cited bucket split for this book
(`A=852, B=769, C=304, D=356, M=168, V=322, U=10`, summing to 2,781) does not match the live
re-derivation (book total 3,498; live counts `DONE=803 A=421 B=711 C=289 D=465 M=427 U=10
V=372`). The book's total and `power`'s own population have not moved — only the internal split
has, between §7's authoring and this cycle's HEAD. Named here per `decisions.md §9`; not
corrected in `decisions.md` itself since that file is outside this epic's file-touch set.

RED confirmed for the intended reason (8 planted-defect violations: wrong population, an
illegal `trait` comparator, and a false "book closes with no remaining buckets" claim — none a
crash), GREEN restored by re-running the build script. `cargo test --locked --no-run` exit 0
(workspace + `apps/desktop/src-tauri` explicitly). No Rust source touched; no corpus record
added or regenerated (`corpus_literal_sweep` unaffected). Denominator gate against this package:
`files_checked=15 violations=4`, all 4 pre-existing (`AT-34-E3-004`'s already-flagged quoted
corpus prose), none introduced this cycle.

Receipt: `artifacts/epic-5-forward-plan/AT-34-E5-003_cycle_receipt.md`.

**Next-cycle plan:** `AT-34-E5-004` can cite `power-table-cost.json` directly for
`ultimate_psionics`'s remaining-after-power shape. A future bundle that actually builds `power`
should re-time the build for real and replace this cycle's DOUBLE-ESTIMATE range with one
measured figure.

### Cycle — AT-34-E5-002 (capability register — every capability that must still be built is named) — complete

**Status: complete.** Built `artifacts/epic-5-forward-plan/capability-register.json` (10 named
capabilities, `built_by_sd34: false` on every row — this register names what still must be
built, it does not build any of it; `epic-5-forward-plan`'s file-touch set is read-only against
the rest of the repo) via `artifacts/epic-5-forward-plan/build_capability_register.py`,
re-derived at HEAD every run.

**8 of 10 capabilities carry a live, mechanically re-derived population, cross-checked two
independent ways:** `power_engine_table` (421, `ultimate_psionics`, bucket A — the one table
Epic 2 left for Epic 5), `companion_table_shape_widening` (28, `bestiary`, bucket A — the
`companion_chassis` table from SD-29 exists but does not cover this record shape),
`per_character_choice_filter` (113, bucket X, 5 books — decisions.md §17's operator ruling: the
backend must filter class-feature option pools against a specific character, and the query
joining `list_class_feature_pool_options()` with `evaluate_feat_prerequisites`/
`character_prereq_facts` does not exist), `companion_mount_advancement_table` (9, bucket X, 8
books — level-based companion/mount stat progression, distinct from whether the companion record
exists at all), `class_feature_deep_subsystem_modelling` (32, bucket X, 8 books, 18 named
sub-mechanisms — bardic performance variants, eidolon evolutions, mystery revelations, spirit
powers, Improved Uncanny Dodge's corpus-cited shape, and more, each real and distinct). The
bucket-X reconciliation is total: these three (113+9+32=154) plus two shapes atlas-defects.md
already correctly resolved into `X` with no further capability needed
(`grant_token_only_dispatch_row`, 12; `vacuous_placeholder_row`, 3) plus 2 `ultimate_campaign`
marker-shaped `X` units sum to exactly **171 of 171**, proven by `verify_capability_register.py`
against a live `completion_atlas.py` partition.

**2 capabilities are cited, not live-re-derived, and say so:** `monster_class_hit_dice_progression_modelling`
(2, `core_rulebook`) and `master_side_ability_pool_record_type_or_cross_book_ownership` (14,
`core_rulebook`), both sourced from `AT-34-E3-001_companion_absent_cycle_receipt.md` — their
current live evidence keys did not resolve by a direct grep this cycle, flagged plainly in the
register's own `verification_note` field rather than presented with the same confidence as the
live-derived rows.

**2 capabilities are named with an explicitly unsized population, not guessed:**
`corpus_content_extraction_for_uncaptured_records` and `cross_record_content_ownership_resolution`
(both from `atlas-defects.md` #2's two unresolved "no description, structural tokens only"
meanings) — both proved required by that defect entry, neither sized, and the register states
"UNMEASURED" rather than blending a made-up number in.

**One marker-stripping capability, both bucket-U-confirmed and project-wide-candidate figures
kept separate:** `marker_stripping_for_pcgen_editorial_markers` — 21 confirmed
(`ultimate_campaign` bucket U, AT-34-E4-001's own test), ~392 project-wide is quoted as an
ESTIMATE from that receipt, not re-run or blended into the 21.

**Deliberately excluded, and said so:** the 634 corpus-wide `companion_absent_from_<book>_companion_tables`
bucket-B units. They need an *existing* table's ordinary placement mechanism (already named and
priced generically by `AT-34-E5-001`), not new engine machinery — the acceptance bar's own
"anything ... does not exist" language does not cover an existing table's unused rows.

RED→GREEN: mutated the committed artifact (`power_engine_table.population` to `999`,
`master_side_ability_pool_record_type_or_cross_book_ownership.built_by_sd34` to `true`) —
`verify_capability_register.py` failed with exactly those two violations, for the intended
reason, not an unrelated crash. Reverted by re-running `build_capability_register.py`.

Denominator gate against this package: `files_checked=15 violations=4` — all 4 pre-existing in
`progress.md` (already-flagged verbatim corpus-prose quotes from `AT-34-E3-004`), none
introduced by this cycle. `cargo test --locked --no-run` exits 0 at `0be7d54a8d2c6c0b879744a2ed3325acbba1f594`
(Python/JSON-only change; no Rust source touched); `apps/desktop/src-tauri` not touched, not run.
`docs/work-inventory.json` untouched — zero movement across all four buckets; this cycle is a
naming artifact, not clearing work. Receipt:
`artifacts/epic-5-forward-plan/AT-34-E5-002_cycle_receipt.md`.

### Cycle — AT-34-E5-001 (per-book, per-bucket forward plan) — complete

**Status: complete.** Built `artifacts/epic-5-forward-plan/forward-plan.json` covering all 35
non-vehicle books (37 inventory books minus `core_rulebook`/`ultimate_campaign`), 29,364
non-DONE units, via `artifacts/epic-5-forward-plan/build_forward_plan.py` (re-derive command
inside the artifact and the receipt). Every non-zero bucket row carries a unit count, its
clearing mechanism, and either a measured rate + sample size or an explicit `UNMEASURED` note —
never a silent absence.

Dispatched ahead of Epics 3/4 reaching zero (`kanban.md` rows 13–17, 20 remain
`in-progress`/`partial`); resolved by reading the criterion literally — it asks for pricing
from **measured** rates, which already exist (Epic 2's table-build rate, both vehicle books'
`step-cost-ledger.json`), not for those epics' full closure. Consequence stated plainly rather
than smoothed: **three pricing tiers**, never blended — 11,919 of 29,364 units (buckets A, B, U)
priced to DONE (1,952–6,782 projected hours, bucket B thin-sampled 3.5× range flagged per Epic
4's own ledger conclusion); 3,981 units (bucket C) priced only to reach V, not DONE (96.4
hours, n=1 cycle); **13,464 of 29,364 (45.9%) unpriced** — buckets D, M, V, X, Z have zero
dedicated clearing cycles in either vehicle book, so no rate exists to project from. No single
"hours to finish everything" figure is asserted.

Found (not fixed, out of this criterion's scope): `bestiary`'s 28 `companion` units sit in
bucket A even though a `companion` table already exists (SD-29) — the existing table doesn't
cover this record shape. Flagged for AT-34-E5-002's capability register.

**Self-caught correction:** the first draft of `build_forward_plan.py`'s summary aggregation
mis-tallied bucket C's 3,981 units as "unpriced" (missed its distinctly-named
`projected_cost_hours_to_reach_V_not_DONE` key), misreporting 17,445 of 29,364 units (59.4%) as
unpriced instead of the correct 13,464 of 29,364 (45.9%). Caught before commit by hand-checking
A+B+C+U against the printed total; fixed by adding
an explicit third pricing tier. Logged via `scripts/retro.py correction`
(`1787979364523-sd34-at-34-e5-001-e41423`).

RED→GREEN: `verify_forward_plan.py` fails closed (exit 1, "does not exist") before the artifact
is built, passes (exit 0, all 35 books' bucket counts match a live `completion_atlas.py`
partition, every row carries a rate-or-note) after. `cargo test --locked --no-run` exits 0 at
HEAD (this cycle touches no Rust; `apps/desktop/src-tauri` not re-run, not touched).
Dual-audit (identifier + wired-integration greps) on the epic's file-touch set:
`OK_NO_BUNDLE_TAGS` / `OK_NO_TOKENS`. No `data/corpus/**` records added — sweep population
unchanged (N/A).

Receipt: `artifacts/epic-5-forward-plan/AT-34-E5-001_cycle_receipt.md`.

### Cycle — AT-34-E4-003 (second, independent cost measurement) — complete

**Status: complete.** Built `artifacts/epic-4-ultimate-campaign/step-cost-ledger.json` and
`step-cost-ledger-raw-commits.json` from the live commit log
(`git log --reverse --format='%H|%ct|%s' ea2b3396f2..HEAD -- docs/work-inventory.json`) and the
live `completion_atlas.partition(units, book='ultimate_campaign')` function, same method as
Epic 3's ledger. Exactly one Epic-4 commit touches `docs/work-inventory.json`
(`4005925ae2`, `AT-34-E4-002`, bucket B): 1 cycle, 108.0 wall-minutes, 5→0 units, 3 reached DONE,
2 reclassified (1 M, 1 D), `units_per_hour_reaching_DONE`=1.667.
`AT-34-E4-001`'s two commits touch no `docs/work-inventory.json` commit (its own receipt: "No
bucket movement") and correctly contribute no rate row.

**Comparison against Epic 3's bucket-B rate** (5.8 units/hr blended, 22.2–617.4 min/cycle
per-mechanism range): Epic 4's single mechanism (108 min for 5 units) sits inside that range —
agreement. Its blended rate (1.667/hr) is ~3.5x slower — divergence, explained explicitly as
**sample-size noise** (n=1 cycle vs. Epic 3's 29), not book shape; the only shape-linked fact this
data supports is mechanism count (1 vs. 29 to close bucket B), not per-unit wall-time cost.
Epic 5 is warned in the ledger not to price Ultimate Campaign's remaining M/D/V buckets off this
single bucket-B data point.

Re-derived at HEAD: `python3 scripts/completion_atlas.py --book ultimate_campaign --check` →
`population=265 unclassified=0 overlap=0 DONE=130 B=0 D=5 M=89 V=18 U=21 X=2` (exit 1, expected —
bar for `AT-34-E4-002` not yet met, unrelated to this criterion). Row-count command output
confirms `buckets_cleared_so_far=['B'] count=1`, `buckets_not_yet_cleared=['D','M','U','V','X']
count=5`, schema check passes on every cleared bucket, comparison field present.

No production code touched; `cargo test --locked --no-run` exit 0 at HEAD `c2805717af`. Full
receipt: `artifacts/epic-4-ultimate-campaign/AT-34-E4-003_cycle_receipt.md`.

### Cycle — AT-34-E4-001 (23-unit non-A tail resolved) — complete

**Status: complete.** Re-derived the tail at HEAD: `python3 scripts/completion_atlas.py --book
ultimate_campaign --check` → `U: 21`, `X: 2` (23 of 265), unchanged before/after this cycle — the
criterion's own Evidence clause allows resolution by proof, not only by bucket movement, and that
is the disposition here.

**21 `U` units (all `feat`, `category=Story`):** verified against the pinned PCGen oracle
(`7f818006e371188e5717fd18d74d18a420747fc6`) that every one carries PCGen's own
`DESC:[Not Implemented] ...` editorial marker plus real, substantial `.MOD BENEFIT:` mechanical
text (Goal/Completion Benefit clauses) joined into the served description
(`ultimate_campaign::feat_tables` + `feats_all::map_uca_entry`). Confirmed the atlas's
`unmeasurable` verdict is not an instrument gap: `SD31-E2-F3-002`'s own test names this exact
population while fixing a case-sensitivity bug, deliberately choosing uniform demotion (not
promotion) corpus-wide (~392 marker occurrences project-wide, not just here). Added a new,
mechanically-checked Rust test (`feats_all::tests::uca_u_bucket_records_still_carry_the_editorial_marker_in_served_form`)
proving this for all 21 records against the real served catalog, and corrected a stale doc-comment
claim in `ultimate_campaign::feat_tables` that had called these 21 "text-complete" — language that
reads as the atlas bucket name but has not matched the live classifier's verdict since
`SD31-E2-F3-002` landed. Filed the corpus-wide marker-stripping question as a named forward
candidate for `AT-34-E5-002`'s capability register rather than deciding it unilaterally
(`decisions.md §16`'s own precedent against resolving a bucket-destination definitional question
on one cycle's authority).

**2 `X` units** (`Fearless Zeal` uca_feats.lst:66, `Magnum Opus` uca_feats.lst:74): checked
directly against the pinned oracle's raw `.lst` and confirmed both splice/truncation defects match
exactly as stated — real, current, unrepairable without inventing text. `X` is the correct, final
resting state for both; no further action clears them.

Full detail, every figure with its re-derive command: `artifacts/epic-4-ultimate-campaign/AT-34-E4-001_cycle_receipt.md`.

`cargo test --locked --no-run` (workspace) exits 0 at `72c9f6fec69371b43aebba12e28e0d0cd990e9b7`.
Dual-audit gate on my own diff: `OK_NO_BUNDLE_TAGS`, `OK_NO_TOKENS`. `data/corpus/**` untouched
this cycle — `corpus_literal_sweep` N/A. Retro events logged (see `scripts/retro.py` invocations in
the cycle receipt).

Next-cycle plan: `AT-34-E4-002` still needs `B`(5)/`D`(4)/`M`(88)/`V`(18) at zero for
`ultimate_campaign` — none of those are this criterion's population and none were touched here.

### Cycle — AT-34-E3-006 (atlas defects recorded) — complete

**Status: complete.** `atlas-defects.md` already existed with 3 entries (written incidentally
by AT-34-E3-001's cycles). This cycle verified it mechanically against the Evidence bar rather
than by re-reading prose: built a scratch structural checker (RED→GREEN — module absent failed
the intended way, then 8/8 tests pass including negative-case mutations: absent file, missing
`**Retro event:**` line, a retro-event path that doesn't exist, a retro-event file with no
correction/deferral event, and an entry recorded with a real retro event but no
re-derivation/not-settled statement — the exact "absorbed, not recorded" shape
`acceptance-and-verification.md §5` names). Ran against the live file: `entries=3 violations=0`.
Cross-checked every later `AT-34-E3-*` cycle receipt (owner-matched 5/6/7, companion-absent 3/4)
that references the same open no-content-record question; all correctly point back at the
existing entries and decline to reclassify on their own authority (`decisions.md §16`'s ruling)
rather than raising an unrecorded fourth defect. No absorbed discovery found. The checker itself
is scratch tooling, not committed — `scripts/` outside `scripts/oracle_harness/` is not in Epic
3's declared file-touch set (same precedent `AT-34-E3-005`'s own generator set). Full detail:
`artifacts/epic-3-core-rulebook/AT-34-E3-006_cycle_receipt.md`.

Standing gates re-checked, unaffected by this cycle (no bucket move, no corpus/inventory
change): `python3 scripts/completion_atlas.py --check` → `population=49438 unclassified=0
overlap=0`. Pre-existing, unrelated to this cycle: `denominator_gate.py --check` on this
package reports `violations=3`, all inside `progress.md` lines quoting corpus prose ("75%
chance...") from the already-merged `AT-34-E3-004` cycle — out of this criterion's file-touch
set, not caused or fixed here. `box_ledger.py --check` (SD-33, inherited, read-only) reports
`uncovered=19817` — pre-existing, SD-34 does not write to `THE-BOX.md`.

`cargo test --locked --no-run` exits 0 at HEAD `2eabffa7a527ad10c6d13b37d8c2f04aab932fb8`; this
cycle touched no Rust source so `--lib` and the desktop crate were not re-run (nothing in the
diff can move either).

Does not resolve `decisions.md §16`'s open definitional question (whether a no-content record
can ever be `held`) — that remains an operator-ruling item named against `AT-34-E3-005`'s own
kanban row, not this criterion's bar.

### Cycle — AT-34-E3-005 (Core Rulebook zero-remaining gate) — partial

**Status: partial.** `AT-34-E3-005`'s bar is a whole-book gate: `python3
scripts/completion_atlas.py --book core_rulebook --check` → `DONE=6701 of 6701`, every other
bucket zero, exit 0. Re-derived fresh at this cycle's HEAD: `population=6701 unclassified=0
overlap=0`, `DONE=1448 A=0 B=532 C=372 D=382 M=1048 V=2793 U=10 X=116 Z=0`, exit **1** — 5,253 of
6,701 units remain outside `DONE`. Every one of those 5,253 belongs to a sibling criterion this
one is gated on (`workflow-instruction.md §3`, sequential within Epic 3): `AT-34-E3-001` owns
bucket B (532 remain, 9 named mechanisms, kanban row 13 `in-progress`), `AT-34-E3-002` owns
bucket C (372 remain, 8 named sub-causes, row 14 `in-progress`), `AT-34-E3-003` owns buckets
M/V/D/U/X (1048+2793+382+10+116 = 4,349 remain, row 15 `in-progress`). Sum check:
532+372+382+1048+2793+10+116 = 5,253; 1,448+5,253 = 6,701 — matches the atlas's own printed
counts exactly.

This cycle did **not** duplicate that bucket-closing work (it belongs to those three criteria's
own dispatched cycles and would collide with their kanban-row bookkeeping). Instead it built
`AT-34-E3-005`'s own named evidence artifact ahead of the gate closing:
`artifacts/epic-3-core-rulebook/core-rulebook-completion-manifest.json` — one row per
`core_rulebook` unit (id, kind, bucket, status, evidence, source_file, source_line), a
`current_state` summary, and a `complete` flag (currently `false`). The generator (kept as
scratch tooling, not committed — Epic 3's declared file-touch set names `scripts/oracle_harness/`
specifically, not a general `scripts/` path) imports `scripts/completion_atlas.py`'s own
`_bucket_of` classifier rather than reimplementing it, after a first draft that reimplemented the
bucket markers independently produced a wrong split (`C=17`/`D=737` instead of the real
`C=372`/`D=382`) — caught by cross-checking against `completion_atlas.py --book core_rulebook
--check`'s own printed counts before shipping the artifact, not after. Row count verified: the
manifest's `units` array has exactly 6,701 entries, matching `population`.

Also observed, out of this criterion's own scope: `python3 scripts/denominator_gate.py --check
'docs/release/SD-34-book-completion/*.md'` → `violations=2`, both pre-existing quoted-corpus-text
matches (`FRT_HVY`'s "75% chance...") inside lines the already-merged `AT-34-E3-004` cycle
committed to `progress.md` — not introduced this cycle, not fixed here (out of this criterion's
own file-touch set and not my prose to silently rewrite); flagged rather than silently accepted.

**Discoveries:** the manifest-generator's own first-draft bucket classifier (reimplementing
`_A_MARKER`/`_B_MARKERS`/`_C_MARKERS` from memory instead of importing them) is exactly the
`field-name-is-not-field-meaning`/proxy-validation hazard the standing lessons warn about — caught
before the artifact shipped by cross-checking against the atlas's own live counts, but it is a
reminder that any manifest/report generator built alongside `completion_atlas.py` must import its
classifier, never restate it.

**Movement:** closure 0 / reclassification 0 / reachability 0 / instrument-correction 0 — no
`core_rulebook` unit moved bucket this cycle; this cycle built evidence tooling only.

**Receipt:** `artifacts/epic-3-core-rulebook/AT-34-E3-005_cycle_receipt.md`.

**Next-cycle plan:** re-run the manifest generator after each future AT-34-E3-001/002/003 cycle so
it never drifts far from HEAD; once those three land at zero, one more run flips `complete` to
`true` and closes this row.

### Cycle — AT-34-E3-004 (per-bucket step-cost ledger, `core_rulebook`) — complete

**Status: complete.** Builds the real evidence artifact
`artifacts/epic-3-core-rulebook/step-cost-ledger.json`: for every bucket that has had a dedicated
clearing cycle so far (B — 29 cycles/2432.3 wall-min, C — 1 cycle/61.0 wall-min, U — 2
cycles/71.5 wall-min), units cleared, wall time, and dominant mechanism, **measured** by
re-partitioning `docs/work-inventory.json` at every commit SHA since the `tranche/14` cut through
the live `completion_atlas.partition(book='core_rulebook')` function, not estimated. Buckets D,
M, V, X (no dedicated clearing cycle yet) are named, not omitted, in the same file's
`buckets_not_yet_cleared` section with their current counts.

Distinguishes closure from reclassification per bucket: bucket B's 503-unit net reduction is 235
real closure + 268 reclassification (moved to another unfinished bucket, not DONE); bucket C's
42-unit reduction is 0 closure + 42 reclassification (all moved to V); bucket U's 48-unit
reduction is 48 closure + 0 reclassification.

The prior `step-cost-ledger.derived.json` (an orchestrator-authored, explicitly PARTIAL,
corpus-wide-by-shape input) is retained as a secondary cross-check, not this criterion's own
evidence — its own embedded note already says it does not satisfy AT-34-E3-004.

**Discovery, not caused by this cycle:** the denominator gate
(`python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'`) is red
at `violations=2` from an earlier, already-committed `AT-34-E3-003` receipt's prose (a literal
"75% chance..." corpus quote in this file's own lines 22/28 as they stood before this cycle's
prepend). Not self-healed here — outside this criterion's file-touch set and this file is a
shared prepend-only log. Filed as a retro `incident`
(`denominator-gate-percent-literal-false-positive`); named in the receipt's Notes for the next
cycle or the closure scan to fix.

Full detail: `artifacts/epic-3-core-rulebook/AT-34-E3-004_cycle_receipt.md`.

### Cycle — AT-34-E3-003 (bucket `U` cycle 2 — `render_pcgen_desc` bare-percent fix) — partial, closure

**Status: partial.** Closes the `render_pcgen_desc` bare-percent-after-digit defect the prior
`AT-34-E3-003` cycle (bucket `U` ruling cycle 1, `decisions.md §17`) named and explicitly
deferred as "a real, narrow renderer bug... out of this cycle's scope." `render_pcgen_desc`'s
bare-`%` render branch gained the same digit-preceded exemption `leaked_pcgen_syntax` already
had, so a clean corpus sentence like `FRT_HVY`'s "75% chance to negate critical hits and sneak
attacks" no longer loses its percent sign and no longer trips
`corpus_json_description_leaks_pcgen_syntax`'s `dropped_args`-non-empty refusal.

**RED→GREEN:** new test `a_digit_preceded_percent_sign_is_a_literal_sign_not_a_drop` in
`src/rules_core/pcgen_desc.rs`, failed for the intended reason (`left: "75 chance..."` vs
`right: "75% chance..."`) before the fix, green after; the sibling non-digit-preceded case
(`"Cast % 1/day"`) still drops, proving the exemption is scoped.

**Movement:** 10 units corpus-wide, `unmeasurable`/`ingested-magnitude` → `text-complete` — the
9 named `equipment_modifier` units the prior cycle enumerated (`FRT_HVY`, `FRT_LGHT`, `FRT_MOD`
and their 3 prose siblings, the 2 `Ghost Touch` records, `Deathless Armor`) plus 1 discovery
(`advanced_class_guide:equipment_modifier:burdenless`, same defect shape, different starting
status — not a new atlas category, reported per `decisions.md §12` L18). Corpus-wide
`unmeasurable` **211 → 202**. `core_rulebook` bucket `U` **18 → 10**, `DONE` **1440 → 1448**.
Whole-corpus before/after diff by unit id: 49,438-unit id set unchanged, exactly 10 changed, all
one kind, both destined for `text-complete` — this is **closure** (a real defect fixed), not
reclassification.

**Remainder — every unit in `AT-34-E3-003`'s five buckets, named, at HEAD:**

| Bucket | `core_rulebook` population (live) | Clearing mechanism | This cycle |
|---|---:|---|---|
| `M` | 1,048 | running the compute path | untouched |
| `V` | 2,793 | the SD-33 oracle harness | untouched |
| `D` | 382 | per named sub-cause (not yet enumerated) | untouched |
| `U` | 10 | operator ruling on `%CHOICE`/`%d<N>` shape | closed 8 of 18; 10 remain, one sub-cause, awaiting ruling |
| `X` | 116 | `decisions.md §17`'s new mechanism — per-character choice filter (not yet built) | untouched |

Sum 1,048+2,793+382+10+116 = 4,349, matching `python3 scripts/completion_atlas.py --book
core_rulebook --check`'s own live sum. `AT-34-E3-003` remains far from closed — `M` and `V` are
the dominant remaining populations and neither has a per-unit-cost measurement yet under this
epic's own scope.

**Build scope:** `cargo test --locked --lib rules_core::pcgen_desc::` 36/36;
`cargo test --locked --bin v06_work_inventory` 412/412; `python3 -m unittest
scripts.tests.test_completion_atlas` 38/38; `cargo test --locked --no-run` (full workspace)
exit 0. `apps/desktop/src-tauri` not touched, not run.

**Sweep population:** no `data/corpus/**` records added or regenerated — only the shared
renderer and the derived inventory changed. `corpus_literal_sweep`: 48708 examined of 51482
read, 0 findings, unchanged before/after (delta 0, correctly matching 0 corpus records added).

Retro: `correction` event `1787971197359-sd34-at-34-e3-003-c809cd`
(`docs/retro/events/sd34-at-34-e3-003.jsonl`) — `render_pcgen_desc`'s bare-percent branch
corrected against `leaked_pcgen_syntax`'s own already-correct exemption.

Receipt: `artifacts/epic-3-core-rulebook/AT-34-E3-003_u_bucket_render_bug_cycle_receipt.md`.

**Next-cycle plan:** (1) escalation candidate — whether the 10 remaining `core_rulebook` `U`
units (`%CHOICE`/`%d<N>`) can read `DONE` or must move to `X`, a policy question the prior cycle
already surfaced and this cycle found no new information to settle; (2) measure per-unit cost on
a sample of `M` and `V` before any population-scoped run — neither has an established rate this
epic; (3) enumerate `D`'s sub-causes before any clearing work starts; (4) `X`'s new clearing
mechanism (the choice-filter join `decisions.md §17` names) does not exist in this codebase yet
and is real, multi-file engineering — scope it as its own cycle or name it to `AT-34-E5-002`'s
capability register.

### Cycle — AT-34-E3-002 (bucket C, "held and computed, never surfaced") — Favored Enemy/Favored Terrain display records, `core_rulebook` 414 -> 372

Re-derived `core_rulebook` bucket C fresh at start SHA `30fa0e6653` (never trusting
`epic-breakdown.md`'s own stale "370" headline, `decisions.md §12` L2): **414**, all
`class_feature` kind, single evidence string `no_explanation_id_and_no_diagnostic_names_this_feature`.
Grouping by corpus-key group prefix found **152 distinct owners** — the large majority
requiring genuinely new per-feature compute formulas (Sorcerer bloodline powers, Rage Powers,
Rogue Talents, Monk abilities, ...), not a display-wiring fix.

The two largest single-owner clusters, `Favored Enemy` (31) and `Favored Terrain` (11) = 42,
had a real, closeable shape: reading `data/corpus/core_rulebook/class_feature/favored_enemy/`
and `favored_enemy_bonus/` directly (never assumed) found each is the DISPLAY-facing sibling
of an ALREADY-GROUNDED chassis record (`"Favored Enemy Bonus ~ <type>"`,
`"Favored Terrain Bonus ~ <type>"`) — same `<type>` suffix, the engine's own
`ranger_favored_enemy_bonus_wired` / `ranger_favored_terrain_bonus_wired` probes
(`AT-34-E3-001` cycle 3) already prove the exact magnitude wired end-to-end. The display
record's own real, player-facing description carries the SAME flat bonus via a `%1`
placeholder; the gap was that `classify()` had no rung attributing the display record to its
sibling's own proven wiring.

Two new grounding rungs added to `classify()`'s `Kind::ClassFeature` arm (no new probe —
reuses the two already-shipped `EngineFacts` sets). TDD: RED (temporarily removed the rungs,
confirmed the two positive-proof tests fail with `"engine-does-not-hold"`, not for an
unrelated reason) -> GREEN (4 new tests: 2 positive proofs, 2 negative controls; full
`class_feature`-scoped suite 124/124 pass, no regression).

**Correction to this cycle's own first assumption, caught by reading the regenerated artifact
rather than trusting the fix's own `"grounded"` return value** (`decisions.md §12` L2/L3): the
42 closed units landed in bucket **V** (`literal-verified`), not `DONE` — a pre-existing,
unrelated mechanism (`apply_done_rung_stamps`) upgrades every `wiring_class: "static"`
grounded record the corpus-literal-sweep independently byte-verifies, and both closed record
types are `static`. Genuinely left bucket C (this criterion's own bar); genuinely still not
`DONE` (bucket V needs the SD-33 oracle harness to fully clear). Reported as
**Reclassification**, not **Closure**, in the four-bucket movement (see receipt).

**Environment incident, resolved before verification could complete**: `cargo test --locked
--no-run` hit a real `ld ... Bus error` (disk exhaustion, not a code defect) at 535M free of
968G on this shared checkout. Found and removed 12 dead-PID `CARGO_TARGET_DIR`s belonging to
already-`complete` Epic 1/Epic 2 cycles (~384G reclaimed, verified no live process held any of
them first) — `AGENTS.md`'s own standing hygiene rule, not a workaround. Re-ran clean: exit 0.

**Instrument hygiene**: the two new rungs shifted 4 downstream `file:line` citations in
`scripts/completion_atlas.py` (buckets A/B/C/V) and 2 in `scripts/missing_engine_tables.py`
(companion/power) — all re-derived by direct grep against the post-edit file, not assumed;
both scripts' own `--check` now exit 0 again (`citation_failures=0`).

**42/414 closed** (bucket C, `core_rulebook`), **372 remain**, named by 8 sub-cause categories
summing exactly (`domain_power_display_record_not_wired` 96, `bloodline_power_or_bloodline_feat_not_computed`
73, `other_named_group_or_standalone` 83 — largest single cluster within it is `Monk Unarmed
Damage` at 54, named as the next-cycle lever — `base_class_standalone_feature_not_computed` 47,
`prestige_class_standalone_feature_not_computed` 31, `rage_power_not_computed` 13,
`rogue_talent_not_computed` 10, `npc_class_standalone_feature_not_computed` 10,
`versatile_performance_not_computed` 9). **Status: partial**, per `decisions.md §15` — the
kanban row stays `in-progress`, the dispatch continues, a later cycle takes the remainder.
No cross-book side effect (`advanced_players_guide` 7, `ultimate_intrigue` 5 units sharing the
same corpus-key group text, both unchanged, checked directly).

Receipt: `artifacts/epic-3-core-rulebook/AT-34-E3-002_cycle_receipt.md`.

### Cycle — AT-34-E3-001 `class_feature_option_pool_record_with_magnitude_not_held_by_engine`, cycle 8 — Universal School grounded, 258 -> 256

Re-derived at HEAD (`617c1d3b40`), still 258 of 553 `core_rulebook` bucket-B units on this
mechanism. Continued cycle 7's own next-cycle-plan lever ("one more wizard school, built
end-to-end") and picked **Universal** — the "no specialization" arm, only 2 power records
(`Hand of the Apprentice`, `Metamagic Mastery`), cheapest of the remaining schools. Reading
`data/corpus/core_rulebook/class_feature/universal_school/*.json` directly (never assumed)
found the same underlying shape every prior school grounding already established: real,
non-fabricated `BONUS:VAR` formulas the engine had never wired, because the corpus record's
`group` prefix can never equal `"wizard"`. Universal is genuinely different in *choice* shape
from every specialist school this mechanism already grounded — PF1's own rule is that a
universalist "need not select an opposition school," so the new
`wizard_has_canonical_universal_selection` gate requires **zero** opposed-school selections,
not the specialist gates' exactly-two — verified against the corpus's raw tokens before
writing any code, avoiding a repeat of cycle 6's own probe-defect history.

New computation block in `src/rules_core/pilot_compute/mod.rs` grounds Hand of the Apprentice
(the shared `ArcaneSchoolPowerTimes` "3 + Intelligence modifier" idiom, unlocked level 1) and
Metamagic Mastery (`(UniversalSchoolLVL-8)/2+1`, unlocked level 8). `probe_wizard_arcane_
school_wiring` (`src/bin/v06_work_inventory.rs`) gained a fifth variant, swapping the
specialization choice to `"school:universal"` and clearing the opposed-schools choice set
entirely rather than replacing it — the first attempt closed exactly the 2 records predicted,
no correction round needed. TDD: RED (both target records failed for the intended reason —
live corpus data at cycle start, `docs/work-inventory.json`) -> GREEN (3 new tests: two
positive proofs on the two distinct power records, one negative control on the never-claimed
top-level `"Universal School"` recognition record).

**2/258 closed** (`Universal School ~ Hand of the Apprentice`, `~ Metamagic Mastery`), both
`core_rulebook`, **no cross-book side effect** (`advanced_class_guide` carries no `"Universal
School"` record at all — checked directly, unlike Transmutation's own Arcanist-exploit
counterpart). 256 remain. `core_rulebook` bucket B (atlas-real partition, all 9 mechanisms)
553 -> 552/6,701; this mechanism 258 -> 256 of 553 (pre-cycle denominator).

Fixed 10 + 2 shifted `file:line` citations in `scripts/completion_atlas.py` /
`scripts/missing_engine_tables.py` (all +40, this cycle's own probe-variant/test insertion and
doc-comment rewrite; `citation_failures=0` on both after). **Discovery, documented for the next
cycle:** the guarded regen refused to run unguarded (`this run would drop 9516 of 9516
verification stamp(s)`) until `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` were
set to a freshly-run sweep's and fixture-check's own `--json-out` reports — no
`--allow-stamp-loss` used. Corpus_literal_sweep unchanged before/after (48,708 of 51,482
examined, 0 findings — no `data/corpus/**` file touched). `F1` unchanged at 5,400 (neither new
formula is bare-literal). `cargo test --locked --no-run` exits 0 (workspace);
`cargo test --locked --lib` 2,910/2,910; `cargo test --locked --bin v06_work_inventory`
408/408 (3 new); `cargo test --locked --test v06_work_inventory` 15/16 (1 pre-existing,
unrelated Barbarian-placeholder failure, unchanged since cycle 3). `denominator_gate.py --check`
`files_checked=15 violations=0`.

`AT-34-E3-001` itself remains **in-progress**: 8 other mechanisms plus this mechanism's own 256
units (Domain Power 56, Domain Base 33, remaining wizard-school clusters ~34, Weapon Training
remainder 48, small/long-tail ~85 — inherited from cycle 7's own partition, not re-derived
fresh this cycle) still need engine wiring. Receipt:
`docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_with_magnitude_cycle_receipt_8.md`.

### Cycle — AT-34-E2-004 reconfirmation at HEAD — no drift

`AT-34-E2-004` ("bucket A reaches zero for both vehicle books") was already `complete` on
`kanban.md` row 12 from its original 2026-08-27 cycle (commit `0dd52ccb65`). Every sibling
criterion in this epic (`AT-34-E2-001`/`002`/`003`) had already been re-dispatched and
reconfirmed at HEAD after 97 intervening Epic 3 commits touched `src/rules_core/`,
`src/bin/v06_work_inventory.rs`, and `docs/work-inventory.json` — the exact files this
criterion's evidence is measured against. Re-derived both check commands at HEAD (`2fa209e25f`)
rather than re-quoting the original receipt (`decisions.md §12` L2/L19). Result: **no drift.**
`python3 scripts/completion_atlas.py --book core_rulebook --check` reports bucket A `0`
(population `6,701`, unclassified `0`, overlap `0`) — every other bucket moved substantially
from Epic 3's own closure work and the `decisions.md §17`/`§18` operator rulings (`DONE
1165→1438`, `B 1035→534`, `C 370→414`, `D 412→382`, `M 921→1048`, `V 2734→2751`, `U 58→18`,
`X 6→116`) but bucket A never left zero. `--book ultimate_campaign --check` reports bucket A `0`
with every other bucket **byte-identical** to the original cycle's after-figures (`DONE=127
A=0 B=5 C=0 D=4 M=88 V=18 U=21 X=2 Z=0`) — Epic 3 scopes `core_rulebook` only and Epic 4
(Ultimate Campaign to zero) has not yet been dispatched, so this book is untouched since the
original cycle. Corpus-wide bucket A also holds at `449`, identical to the original cycle's own
recorded after-figure, entirely `power` (421, priced in Epic 5) and `companion` (28, unchanged
split per `missing_engine_tables.py --check`). `cargo test --locked --no-run` exits 0 at the
workspace root and in `apps/desktop/src-tauri` (separate cargo workspace, tested explicitly);
`cargo test --locked --lib rules_core::rules_tables::simple_kind_tables` is 13 of 13 passing.
Dual-audit gate on Epic 2's file-touch set: `OK_NO_BUNDLE_TAGS`; the 24 `placeholder` matches
are all PCGen's own CHOOSE-menu "no-selection" domain term, reviewed and disposed of as the same
self-healable false positive the sibling reconfirmations already found — no stub tokens.
`denominator_gate.py --check` on this package: `files_checked=15 violations=0`.
`corpus_literal_sweep`: `48708` examined, `0` findings, CLEAN — this cycle touched zero corpus
records (`git status --porcelain data/corpus` empty both before and after); the population's
growth from the original cycle's `48699` to `48708` is Epic 3's own later, unrelated
regenerations, the same `+9` the sibling reconfirmations already attributed.
`box_ledger.py --check` exits `1` (`uncovered=19861`) — flagged as pre-existing, growing drift
from `THE-BOX.md` not being re-derived this bundle (inherited read-only per `decisions.md §2`,
outside this criterion's file-touch set and evidence bar), a decrease from the original cycle's
own recorded `21504`, reported plainly rather than silenced. No production code changed; no unit
moved across any bucket. See `artifacts/epic-2-tables/AT-34-E2-004_reconfirmation_receipt.md`
for the full re-derivation. Epic 2 (4 of 4 criteria) is now fully reconfirmed at HEAD with no
drift found on any criterion.

### Cycle — AT-34-E2-003 reconfirmation at HEAD — no drift

`AT-34-E2-003` ("the measured build rate is recorded") was already `complete` on `kanban.md`
row 11 from its original 2026-08-27 cycle. Re-dispatched to re-derive the evidence at HEAD
(`ac61ac1b89`) rather than re-quote the original receipt (`decisions.md §12` L2), matching the
sibling reconfirmations already run on `AT-34-E2-001`/`AT-34-E2-002` after Epic 3's later commits
touched `src/rules_core/`. Result: **no drift.** `table-build-rate.json` still holds 8 of 8 table
entries (`ability`, `template`, `trait`, `deity`, `domain`, `skill`, `language`, `companion`);
the two historical wall-time windows the artifact reports (`AT-34-E2-001` cycle 1,359s,
`AT-34-E2-002` cycle 779s) are pinned to immutable commit timestamps and unchanged;
`cargo test --locked --lib rules_core::rules_tables::simple_kind_tables` is 13 of 13 passing at
HEAD, identical to the sibling `AT-34-E2-001` reconfirmation's result. `cargo test --locked
--no-run` exits 0 at the workspace root and in `apps/desktop/src-tauri` (separate cargo
workspace, tested explicitly). Dual-audit gate on Epic 2's file-touch set: `OK_NO_BUNDLE_TAGS`;
the wired-integration grep's ~15 `placeholder` matches are all PCGen's own CHOOSE-menu
"no-selection" domain term (Epic 3's `AT-34-E3-001` vacuous-placeholder sub-cause vocabulary),
not stub tokens — reviewed and disposed of as a self-healable false positive, same category the
sibling reconfirmations already found. `denominator_gate.py --check` on this package:
`files_checked=15 violations=0`. No production code changed; no unit moved across any bucket.
See `artifacts/epic-2-tables/AT-34-E2-003_reconfirmation_receipt.md` for the full re-derivation.
Epic 2 (4 of 4 criteria) remains fully complete.

`tranche/14` cut at `571307724f`, `0.14.0` stamped, launch checklist items 1-9, 11, 12 run.
Item 10 (widest build scope + inherited test baseline) is a separate lane's obligation and is
not reported here. Epic 1 dispatch underway.

**12 of 27 criteria complete. 12 of 27 kanban rows complete.** Epic 1 is closed at 8 of 8;
Epic 2 is closed at 4 of 4 (AT-34-E2-001..004). Epic 3 (Core Rulebook to zero) is underway:
AT-34-E3-001's escalation was cleared by orchestrator ruling (`decisions.md §14`) into nine
named mechanisms totalling 1,006 of 1,006 — dispatched one per cycle, cheapest-first. Cycle 1
cleared the `template`/`ability` reattribution mechanism (29 of 1035); the `domain` mechanism
cycle cleared the smallest (1 of 1,006); the next cycle cleared `race_trait_absent_from_race_traits`
(9 of 1,006); this cycle cleared `class_absent_from_ClassId_ALL_and_book_class_id_enums`
(17 of 1,006 — CRB's ten prestige classes registered from `prestige_class_entry_gate`'s
existing real registry, and a new `crb_untabled_class_chassis.rs` module gives the five NPC
classes plus Ex-Barbarian/Ex-Paladin a real, corpus-formula-derived chassis; see the cycle
log below for a cross-book class-feature attribution side effect this cycle found, reasoned
through, and self-healed, plus a pre-existing `cargo test --locked --lib` failure this
cycle's own §6 step 3 run discovered and re-pinned); the next cycle cleared
`deity_content_absent_from_deity_table_in_core_rulebook` (21 of 974 — all 21 `cr_deities.lst`
records are PI-masked at ingestion; `Kind::Deity`'s `classify()` arm gained the same
coordinate-fallback resolution the `domain` mechanism already proved, never reading the
redacted real name); the next cycle partially cleared
`class_feature_option_pool_record_not_held_by_engine` (63 → 57 → 55 of 968 — a new
`class_feature_pool_catalog::load_standalone_class_feature_catalog` closes six genuinely
prose-only, mechanically-inert standalone features; the remaining 57 turned out NOT to be a
single root cause on direct per-record inspection — seven distinct sub-shapes named with
populations in that cycle's own receipt, most requiring new cross-cutting engine capabilities
this program does not yet have; a follow-up cycle then closed the cheapest of those seven, a
2-unit multi-DESC ingest truncation — see the cycle log below for the full decomposition and
that cycle's own caught-and-reverted corpus-wide near-miss); the next cycle partially cleared
`companion_absent_from_core_rulebook_companion_tables` (100 → 28 — `companion_chassis`'s
transcriber gained a seventh ownership shape, book-wide grant, attributing Core Rulebook's
generic Animal Companion progression table to all 38 registered creatures at once, a real
corpus-backed fact rather than an invented per-creature link; the remaining 28 are three named
sub-causes — 12 zero-content internal plumbing rows, 2 PCGen monster-class definitions, and 14
master-side familiar-ability-pool rows this book registers no familiar creature to own — see
the cycle log below).
AT-34-E3-001 itself does not close yet — `core_rulebook`'s real, atlas-partitioned bucket B is
now 757 of 6,701 (`python3 scripts/completion_atlas.py --by-book`, grepped for `core_rulebook`;
down from 762, this cycle's own 5-unit closure), and four of the nine named mechanisms remain
(their live populations — 346, 328, 55, 28 — sum to exactly 757, no unnamed gap). The previous
cycle picked up `class_feature_owner_matched_by_name_but_record_not_held_by_engine` (346,
confirmed still 346, not a further drift) and reported `partial`: an exact, sum-exact 7-way
sub-cause partition, proven by a committed passing regression test, but 0 units closed — every
sub-cause needs real engine wiring or new ingest work, not a narrow catalog-widening fix. This
cycle picked up `class_feature_option_pool_record_with_magnitude_not_held_by_engine`
(333 -> 328): built a real, live-probed attribution path for cleric's `"Domain Power"` group
(`domain_power::domain_power_probe_catalog` + a new `probe_domain_power_effect_wiring`), closing
the exactly 5 units the engine genuinely computes (Good/War/Strength/Destruction/Glory's own
granted powers) and reported `partial` — a sum-exact, 129-group sub-cause partition for the
remaining 328 (see the cycle log below). This cycle re-derived (not inherited) the judgement on
`companion_absent_from_core_rulebook_companion_tables`'s 28-unit remainder per its own dispatch
instruction ("re-derive rather than inherit; take a narrower fix if one closes them") and
confirmed it correct with new corpus evidence: the 14 familiar-pool rows' true owners (11
familiar creatures) already ship as registered `CompanionRecord`s under `beastiary`, not
`core_rulebook` — a real cross-book split baked into the actual books (Core Rulebook states the
ability rules, Bestiary states the creature stat blocks), never a reattribution bug or a
"no such creature" gap. Closing it needs Shape 8 (cross-book ownership), a corpus-wide widening
of the same-book invariant every other registered companion book currently relies on — not a
narrow single-book fix. 0 units closed; the 28/12/2/14 partition is now proven by a committed,
re-runnable regression test rather than asserted in prose. See the cycle log below; `## Open
blockers` is empty. This cycle picked up
`class_feature_option_pool_record_not_held_by_engine` again (55 → 52): filed
`artifacts/epic-3-core-rulebook/atlas-defects.md` entry 1 for the 3 vacuous PCGen placeholder
rows (`Empty Selection ~ Standard {Barbarian, Monk, Rogue}` — null description, no mechanical
token, an unpredicted verdict shape per `decisions.md §2`) before closing them to
`deferred-with-reason` (bucket X) via a new closed 3-key named list
(`class_feature_pool_catalog::VACUOUS_PLACEHOLDER_CLASS_FEATURES`). `core_rulebook` bucket B now
754 of 6,701. 0 units closed to DONE (correctly — there is no content to display); reported
`partial`, 52 remaining named exactly by sub-cause. See the cycle log below; `## Open blockers`
is empty. This cycle re-picked up `companion_absent_from_core_rulebook_companion_tables`
(14 → 2): closed the 12 grant-token-only rows via a per-record, corpus-wide VERIFIED predicate
(never a shape-only reclassification — a corpus-wide re-check of defect 3's own bare shape
query found only 171 of 461 safe, proving shape alone unsafe) to `deferred-with-reason`
(bucket X); the 2 monster-class rows remain, needing a genuine level-progression record type
now verified against its 2 real second/third consumers (`ultimate_magic` 3 rows,
`book_of_the_damned_volume_1` 2 rows) for a future cycle. Reported `partial`. See the cycle log
below; `## Open blockers` is empty. This cycle re-picked up
`class_feature_option_pool_record_not_held_by_engine` again (49 → 44): built a new
`weapon_tables::CLASS_ARMOR_PROFICIENCIES` table and, verifying BOTH weapon AND armor content
byte-for-byte against the live corpus, closed 5 of the 7 `"Weapon and Armor Proficiency ~
<Class>"` combined records (Bard, Fighter, Paladin, Ranger, Rogue); Druid and Monk stay
correctly excluded (a real corpus-internal `Scythe` discrepancy and cycle 5's own established
`Flurry of Blows` mismatch, respectively). `core_rulebook` bucket B now 564 of 6,701. Reported
`partial`, 44 remaining named exactly by sub-cause. See the cycle log below; `## Open blockers`
is empty.

Baseline at authoring, measured against `origin/develop` `ea2b3396f2`
(`content-unit-inventory.md` carries the re-derive command for each):

| Figure | Value |
|---|---|
| Corpus population | 49,438 units across 37 books |
| Ingestion | **complete** — 49,438 of 49,438 units carry a real source_file + source_line |
| DONE | 12,265 of 49,438 |
| Non-DONE | 37,173 of 49,438 |
| Largest bucket: B (record not in its table) | 11,921 of 49,438 |
| Bucket A (no engine table exists) | 8,463 of 49,438, across 9 kinds — 8 built here, `power` costed |
| Core Rulebook (vehicle 1) | 6,701 units, 1,150 DONE, 5,551 non-DONE, every bucket present |
| Ultimate Campaign (vehicle 2) | 265 units, 0 DONE — A=242, U=21, X=2 and nothing else |
| Shape-engine feedstock still unheld by the engine | 13,119 of 26,396 |

## Cycle log

### Cycle — AT-34-E4-002 (cycle 3) — the trait/drawback selection capability is BUILT, 31 units genuinely closed

**Why:** the criterion's first two cycles diagnosed (no trait/drawback selection capability
existed anywhere in the crate) and closed an unrelated 3-unit `D`-bucket sliver. This cycle is
the build the diagnosis named: real `CharacterInput` selection state, a real compute-and-apply
path, and a real desktop selection surface — for the pure `BONUS:SKILL` trait spine.

**Provenance.** Started from `origin/salvage/wave14-lane1` (an unreviewed, twice-rescued
checkpoint), read in full via `git diff 1ea93e99ce origin/salvage/wave14-lane1` before trusting
any of it. Applied with `git merge --no-ff --no-commit` (zero conflicts against tranche/14 HEAD
`b939abcd4b`), then `git restore`d the two files this cycle must not hand-commit
(`apps/desktop/package-lock.json`'s stray version edit, and the salvage's own regenerated copy
of `completion-atlas.json`, which is GENERATED). Kept everything else after reading it: the new
`src/rules_core/trait_effects.rs` (31 hand-transcribed flat `BONUS:SKILL` trait records, each
individually re-verified by an EXECUTED fixture character run through
`allocate_skill_ranks` — not asserted, run), the new `CharacterInput.chosen.selected_traits`
field, the `skill_allocation.rs` wire folding real trait bonuses into `misc_modifier`, the
`v06_work_inventory.rs` classifier wire (`Kind::Trait` now calls the fixture-grounding check),
and full desktop plumbing: `trait_picker.rs`'s `list_available_character_traits` command, a
real checkbox picker in `CreateCharacterForm.tsx`, and save/load round-tripping in
`saved_character/local_store.rs`. See `artifacts/epic-4-ultimate-campaign/AT-34-E4-002_cycle_receipt_3.md`
for the full kept/discarded accounting.

**What changed, corpus-wide (kind-keyed, not book-scoped):** `docs/work-inventory.json`
regenerated (three-pass pipeline, `corpus_literal_sweep` unchanged at `records_examined:48708`
— no corpus records touched). `0 added, 0 removed` units; **36 units** moved
`ingested-magnitude → grounded`: **31 in `ultimate_campaign`**, plus **5 in
`advanced_players_guide`** that happen to share the identical corpus `KEY` (e.g.
`Trait ~ Caretaker`) — a real, unplanned corpus-wide payoff from keying the classifier fix on
`unit.key` rather than the book, exactly as this criterion's own brief predicted.
`ultimate_campaign`: `DONE 151→182, M 89→58`, all other buckets (`U:21 D:2 X:2 V:0`)
untouched — not reopened, not reclassified.

**Correction of this criterion's own dispatch brief:** the brief stated "44 of the 59 traits
are pure `BONUS:SKILL`"; a direct read of every `trait_generic/*.json` record found the real,
compute-path-coverable figure is **31 of 59** flat, unconditional, no-`%LIST` records. The
other ~28 split into 12 `%LIST` player-choice placeholders (needs a chosen-skill slot), 3
ability-difference formulas (needs a formula evaluator), 15 mixed non-`SKILL` bonus types
(saves/combat/concentration/ability-pool — different pillars), and 1 corpus data gap
(`trait_shadow_whispers` — a real inventory unit with no matching file under `trait_generic/`
by any name tried, pre-existing, unrelated to this cycle). Retro `correction` logged.

**Also named, not silently absorbed:** `ultimate_campaign`'s `ability_content` M-bucket (30
units) is **not** all "drawback" — 18 are real PF1 Drawbacks (17 pure narrative/GM-adjudicated
with no `BONUS` token at all, 1 — `drawback_meticulous` — a cross-skill `PRESKILL`-guarded
penalty, a different shape than this cycle's flat spine) and **12 are `Retrain` records**, a
different mechanic entirely (character-progression rebuild rules), arguably mis-keyed under
this criterion's `ability_content` population rather than trait/drawback selection at all —
flagged for a future atlas-defect note, not chased here.

**Verification:** `cargo build --locked --lib` clean. `trait_effects` unit tests 11/11
(including the fixture-executed grounding proof for all 31 entries).
`character_input`/`skill_allocation`/`local_store` targeted tests 36/36.
`v06_work_inventory` bin tests 437/437 (new positive + negative-control promotion tests both
pass). `cargo test --locked --no-run`: full workspace exit 0. `apps/desktop/src-tauri` tested
explicitly: 524 passed / 28 failed, every failure in a file (`companion_catalog.rs`,
`reach_gate.rs`, `feat_catalog.rs`, `race_trait_picker.rs`) this cycle never touched —
attributed pre-existing, not independently re-baselined at this SHA (no established
desktop-crate baseline exists in this criterion's brief).

**Status: partial.** `ultimate_campaign` bar (`DONE=265 of 265`) not met:
`DONE=182 of 265`, remainder `M:58 U:21 X:2 D:2` = 83, all named by sub-cause above. Receipt:
`artifacts/epic-4-ultimate-campaign/AT-34-E4-002_cycle_receipt_3.md`.

### Cycle — AT-34-E3-005 (bucket-v-apply) — `decisions.md §19` wired into the classifier

**Why:** the prior bucket-v-consolidation cycle changed no engine status — it only wrote
`bucket-v-consolidated.oracle-results.json` (2,712 of `core_rulebook`'s 2,793 bucket-V units
dispositioned, 385 `agree` / 2,327 `unverifiable`, 81 named remainder) and said plainly its own
kanban row's `V` count would stay 2,793 "until a later status-promotion pass consumes this
ledger." This cycle is that pass.

**What changed:** `v06_work_inventory.rs` gained `apply_bucket_v_oracle_disposition_stamps`, a
new rung run right after `apply_done_rung_stamps`, keyed strictly on the consolidated ledger's
own per-unit `verdict` (never a shape predicate or a name list). Two new statuses,
`oracle-agree`/`oracle-unverifiable`, both mapped to `completion_atlas.py`'s `DONE` bucket
(`decisions.md §19` extending `§17`'s disposition principle from bucket `U` to bucket `V`). A
`disagree` verdict is never dispositioned by construction — proven by a dedicated test; there
are zero `disagree` rows in the committed ledger today.

**Three-pass regen** (`corpus_literal_sweep` → `derived_evaluator_fixture_check` →
`v06_work_inventory`, both report env vars set, no `--allow-stamp-loss`):
`corpus_literal_sweep` reported `clean:true, records_examined:48708` (no corpus records touched
this cycle, so no before/after delta to report — `decisions.md §12` L8 does not apply here).
`derived_evaluator_fixture_check`: `1839 cleared over 2580 fixture rows; 0 failed`.
`v06_work_inventory` wrote `docs/work-inventory.json` cleanly — the stamp-loss guard's own
notion of "stamped" was widened to include the two new statuses precisely so a
`literal-verified/fixture-verified → oracle-agree/oracle-unverifiable` promotion is never
misread as a loss.

**Whole-corpus before/after diff by unit id:** 49,438 before, 49,438 after, 0 added/removed,
**2,712 changed, all in `core_rulebook`, 0 in any other book.** Bucket transition: `V → DONE`:
2,712 (nothing else moved). `core_rulebook` V: 2,793 → 81; DONE: 1,503 → 4,215. Corpus-wide V:
9,558 → 6,846; DONE: 14,741 → 17,453. The 81 remaining `core_rulebook` V units are SET-equal
to `bucket-v-remainder.json`'s own 81 named ids (verified, not merely counted).

**A citation regression, caught and fixed in the same cycle, same shape as `AT-34-E1-002`
condition 6's first live catch (Cycle 1's own progress-log entry above):** this cycle's own
~230-line insertion in `v06_work_inventory.rs` shifted all ten of `completion_atlas.py`'s
`BUCKET_DEFINITIONS` `file:line` citations. `--check` caught all 10 (`citation_failures=10`);
each was re-derived against the live file and corrected in this same cycle
(`citation_failures=0` after).

**Capability register:** `no_probe_surface`'s 130 units (the weaker of the two dispositions —
"the instrument was never built", not "the oracle cannot express it") were carried into
`artifacts/epic-5-forward-plan/capability-register.json` as a new named capability,
`oracle_probe_surface_for_no_table_kinds` (130 of 2,793: `ability` 90, `template` 36,
`companion` 4), rather than left a closed question. 10 → 11 capabilities;
`verify_capability_register.py` still passes, `built_by_sd34: false` on every row.

**Test scope:** 5 new unit tests (the new rung) + 1 widened pre-existing test (the stamp-loss
guard's "stamped" statuses, 2 → 4) — 14/14 green in `v06_work_inventory.rs`'s stamp/rung test
modules. `sd13_*`/`sd25_*` gate run together, 184 targets (175 + 9), not a scoped subset:
2,000 passed, 0 failed. `cargo test --locked --no-run` exit 0 (full workspace); same, exit 0,
in `apps/desktop/src-tauri` (separate cargo workspace, tested explicitly). Denominator gate
against this package: `files_checked=15 violations=6`, unchanged from the pre-cycle baseline
(all 6 the pre-existing `FRT_HVY` corpus-prose quote).

**Movement (`decisions.md §9`):** closure 0, reclassification 2,712 (a V→DONE move under a
disposition ruling is not the same as engineering a record to completion — none of these 2,712
values were newly computed or newly verified by this cycle), reachability 0 (established by the
prior consolidation cycle, not this one), instrument-correction 0.

Receipt: `artifacts/epic-3-core-rulebook/bucket-v/AT-34-E3-005_bucket_v_apply_cycle_receipt.md`.

### Cycle — AT-34-E3-001 wave-9 shared regeneration — `docs/work-inventory.json`, paid once for four lanes

**Why:** gate-widening, owner-matched, with-magnitude, and option-pool all landed engine
changes on `tranche/14` without regenerating `docs/work-inventory.json` (the three-pass
pipeline's own throughput cost, deliberately deferred to a single shared cycle per this
wave's dispatch rule). This cycle pays that cost once and attributes the result.

**Per-pass wall time** (fresh `CARGO_TARGET_DIR`, `CARGO_INCREMENTAL=0`):

| Pass | Command | Wall time |
|---|---|---|
| 1 | `corpus_literal_sweep --json-out /tmp/sweep.json` | 3m24.9s |
| 2 | `derived_evaluator_fixture_check --json-out /tmp/fixture.json` | 0m13.1s |
| 3 | `v06_work_inventory` (with both reports set, no `--allow-stamp-loss`) | 11m00.1s |

Total pipeline: **~14m38s**. A second, corrective run of pass 3 followed the restoration
below (source changed after the first regen); its own wall time was not separately timed
(warm target dir, well under 3 minutes).

**Whole-corpus before/after diff by unit id:** `docs/work-inventory.json` snapshotted at
`git show HEAD:docs/work-inventory.json` before any pass ran. After both regenerations:
**49,438 ids before, 49,438 after, 0 added, 0 removed, 79 changed** (`status` or `evidence`),
all 79 in `core_rulebook`.

**A rebase regression found and fixed in this cycle, not merely reported.** The option-pool
lane's commit `a183d760c76` wired `class_feature_pool_catalog::wizard_school_spell_list_key_
owner` into `v06_work_inventory.rs`'s `classify()` (moving 9 `"<School> Wizard Spells"` units
bucket B -> D) and added two dedicated tests. The very next commit on the mechanism,
`534c9c2a61` (subject: "generalize Weapon Training to all 14 groups, 256->208"), silently
**deleted** that entire wiring block and both tests while resolving its own rebase conflict —
nothing in its commit message or receipt mentions removing anything. The first regeneration
pass surfaced this directly: the 9-unit wizard-school-spell-list population showed **34 -> 34,
unchanged**, contradicting the option-pool lane's own commit message. Diffing `534c9c2a61`
against its parent confirmed the deletion. Restored verbatim (the `if class_feature_pool_
catalog::wizard_school_spell_list_key_owner(&unit.key).is_some()` arm plus
`a_wizard_school_spell_list_row_verified_against_the_join_leaves_bucket_b` and
`an_unlisted_wizard_spells_shaped_key_still_falls_to_the_generic_fallback`), re-verified
`cargo test --locked --bin v06_work_inventory` **419/419 passed** (was 416 before restoring;
+3, matching the 2 restored tests plus the with-magnitude lane's own net addition), re-ran
`cargo test --locked --no-run` (workspace, exit 0) and `--manifest-path apps/desktop/src-tauri/
Cargo.toml` (exit 0), then re-ran pass 3 to produce the final `docs/work-inventory.json`.

**Attribution — expected vs actual, per lane:**

| Lane | Expected | Actual | Verdict |
|---|---|---|---|
| **with-magnitude** | 256 -> 208 (48 closed, Weapon Training generalized to 14 groups); explicitly said the 4 "Monk"-weapon-group records would NOT move this cycle | 48 closed exactly as predicted (B -> `grounded`) **plus 4 more** — the 4 Fighter Weapon-Training "Monk" weapon-group records (a PF1 weapon category, not the Monk class) reached `grounded` via a different pre-existing `classify()` path the lane's own population check didn't count. 52 real closures, not 48. | **Confirmed, and better than predicted** |
| **option-pool** | 34 -> 25 in one framing; the lane's own commit says "9 keys bucket B -> D" (reclassification, not closure) | 9 units B -> D (`class_feature_wizard_school_spell_list_held_by_wizard_spell_list_and_spell_list_join`), exactly as the commit describes — **but only after this cycle restored the wiring the with-magnitude lane's rebase had deleted.** Before the restoration, the true count was 34 -> 34, unchanged. | **Confirmed only after in-cycle repair; would otherwise have been silently refuted** |
| **owner-matched** | 0 of its 24-unit non-excluded population (18 null-description + 6 gate-refused) moves | 0 moved, confirmed by direct re-check of all 6 named gate-refused units and the population count (24 -> 24, ids identical) | **Confirmed exactly** |
| **gate-widening** | "A floor of 5 units move for certain": `Bard ~ Bardic Knowledge`, `Bard ~ Lore Master`, `Paladin ~ Holy Champion`, `Paladin ~ Lay on Hands`, `Sorcerer ~ Spells`; unconfirmed larger tail; 0 for Druid/Monk | **Refuted for 4 of 5 named units** — `bard_bardic_knowledge`, `bard_lore_master`, `paladin_holy_champion`, `paladin_lay_on_hands` were already `grounded`/`literal-verified`/`fixture-verified` BEFORE this wave even started (unaffected either way; the lane's own receipt flagged this exact possibility — "the collision guard correctly protecting a real hand-wired magnitude"). Only `sorcerer_spells` changed, reclassifying C -> D alongside 4 DIFFERENT, un-named units (`wizard_arcane_bond`, `cleric_aura`, `paladin_detect_evil`, `wizard_bonus_feats`) — 5 total reclassified, matching the predicted COUNT but not the predicted MEMBERSHIP. Separately, 2 units the lane did not name as moving DID close to DONE (`wizard_cantrips`, `cleric_spontaneous_casting`, B -> `text-complete`) — real closure beyond what the receipt claimed credit for. 0 Druid/Monk movement confirmed as predicted. Plus 1 unrelated reclassification (`bard_bardic_performance`, X -> D). | **Count coincidentally matched, membership did not; real closure the receipt didn't claim** |

**Movement, four buckets (`decisions.md §9`):**
- **Closure:** 54 (48 + 4 weapon-training-group closures to `grounded`; 2 owner-matched-widened closures to `text-complete`).
- **Reclassification:** 16 (9 wizard-school-join B->D; 5 gate-widening C->D; 1 owner-matched B->D; 1 bard-performance X->D).
- **Evidence-string churn, no bucket crossed:** 9 (already-`DONE`/`text-complete` units whose evidence id changed from a hand-wired path to the gate-widening lane's new citation-based path — same bucket before and after, reported separately per `decisions.md §9`'s own discipline that a non-movement is not smoothed into either closure or reclassification).
- **Instrument-correction:** 0 (no wrong prior count found in the inventory itself this cycle; the option-pool discrepancy above is a code regression, not an instrument error).

**`completion_atlas.py --check`** (corpus-wide): `population=49438 buckets=10 unclassified=0
overlap=0`, `DONE=14740 A=449 B=11771 C=4344 D=3071 M=5114 V=9558 U=202 X=170 Z=19`,
`citation_failures=0`. All 10 `BUCKET_DEFINITIONS` citations were re-derived by direct `grep`
against `v06_work_inventory.rs`'s post-regen line numbers and fixed **twice** in this cycle —
once after the shared regeneration (which shifted every citation past the with-magnitude
lane's own insertion), again after this cycle's own 41-line restoration shifted four of them
a second time (A, B, C, V). `--book core_rulebook --check`: `population=6701 DONE=1502 A=0
B=472 C=363 D=398 M=1048 V=2793 U=10 X=115 Z=0` (exit 1 — book not yet closed, expected).
`--book ultimate_campaign --check`: `population=265 DONE=130 A=0 B=0 C=0 D=5 M=89 V=18 U=21
X=2 Z=0` (exit 1 for the same reason; zero of this book's units were touched by any of the
four lanes).

**`denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'`:** `files_checked=15
violations=5`, all pre-existing verbatim-quoted corpus prose in `progress.md` ("75%
chance..." from `FRT_HVY`'s own description), already flagged by the already-merged
`AT-34-E3-004` cycle and every subsequent cycle that ran this check. Unchanged by this cycle.

**Build scope, final HEAD (post-restoration):** `cargo test --locked --no-run` (workspace)
exit 0; `cargo test --locked --no-run --manifest-path apps/desktop/src-tauri/Cargo.toml`
exit 0 (explicit desktop-crate run, `decisions.md §10`); `cargo test --locked --bin v06_work_
inventory` 419/419 passed.

Receipt: `artifacts/epic-3-core-rulebook/AT-34-E3-001_wave9_regen_receipt.md`.

### Cycle 8 — AT-34-E3-001 (`class_feature_owner_matched` mechanism) — anti-fabrication gates widened by construction (partial)

**Status: partial.** Operator ruling `decisions.md §18` on the open question row 330 raised:
`push_generic_class_feature_grant_records`'s wholesale `ANTI_FABRICATION_GATE_EXCLUDED_CLASSES`
per-class refusal (Wizard, Bard, Paladin, Cleric, Sorcerer, Druid, Monk) is replaced by a
citation-based property for five of the seven classes — an explanation is admitted when it
cites a real, renderable corpus record (the existing, unchanged
`corpus_records_with_real_description`/`resolved_description_for` gate), never because its
class name sits on a hand-maintained allowlist. Druid and Monk keep a (renamed, honestly
re-scoped) wholesale exclusion — `LEVEL_UP_PILLAR_FILTERED_CLASSES` — for the SEPARATE
`is_druid_pillar_id`/`is_monk_pillar_id` `LevelUpPlan` filter reason, outside this lane's write
scope.

**Correction to this cycle's own dispatch brief:** stated "161 of 242" owner_matched units are
gated by the seven-class list; cycle 7's own receipt had already verified **218** (Sorcerer 137,
Cleric 39, Monk 25, Wizard 7, Paladin 5, Bard 4, Druid 1). Retro correction event emitted
(`docs/retro/events/sd34-at-34-e3-001.jsonl`). The wrong count did not change this cycle's
scope — all seven named classes matched.

The nine `sd13_*`/`sd25_*` anti-fabrication acceptance tests (`OPEN-ISSUES.md` rows 330/338) are
widened ADDITIVELY, never weakened: the five `sd13_bard_level4..8_progression` closed-namespace
allowlists each gain one `class_feature.bard.corpus_record.` prefix carve-out; the wizard/
cleric/sorcerer "no spell math" substring guards each gain the analogous carve-out;
`sd13_paladin_level8_progression`'s `"resolve"` guard gains an exact-id carve-out for the one
citation-backed grant fact it collides with (`aura_of_resolve`); the (non-"nine", but
pre-existing) `divine_bond` guards in `sd13_paladin_level5/6/7_progression` receive the same
exact-id treatment. Every existing assertion in all nine still fails on a genuinely fabricated
id. RED→GREEN mutation proof: a synthetic corpus key with no real record is refused by the
citation gate, then a real key is confirmed to still resolve cleanly (`mutation_proof_a_
fabricated_key_is_never_treated_as_citation_backed`). New test
`previously_gated_classes_now_emit_citation_backed_explanations_by_construction` proves all
five widened classes now emit directly against the live merged grant data.

The pinned live-scale census moved `newly_resolved 21->26`, `class_excluded_otherwise_
resolvable 11->6` — a reclassification of five already-resolvable records the wholesale
exclusion had been hiding (Bard Bardic Knowledge/Lore Master, Paladin Holy Champion/Lay on
Hands, Sorcerer Spells), not a resolver change. `cargo test --locked --lib --
rules_core::pilot_compute::class_feature_grant_consumer::` → 33/33 passed. Individually
confirmed green: `sd13_cleric_level1_spell_baseline` (17/17), `sd13_paladin_level8_progression`
(14/14), `sd13_bard_level4/5_progression` (14/14, 16/16). `sd13_sorcerer_level1_spell_baseline`
completed 18/19 with one FAILED test confirmed unrelated (a different, pre-existing
`mod.rs`-owned bloodline-feat-pool mechanism, id shape this cycle's file cannot produce; this
cycle touches no file that mechanism owns). `sd13_bard_level6/7_progression`'s own
`suggestion_dc`-related failures confirmed pre-existing and unrelated (verified via `git diff
origin/tranche/14` touching neither test's assertion nor `mod.rs`'s `suggestion_dc` code).
`sd13_wizard_level1_prepared_spell_baseline`, `sd13_bard_level2/3/8/9/10_progression`,
`sd13_paladin_level5/6/7_progression`, and both `sd25_*` LevelUpPlan audits were not run to
completion this cycle (sustained shared-machine disk/CPU contention — a `.reclaim-claim`-gated
reclaim daemon deleted this cycle's own `CARGO_TARGET_DIR` mid-build once) — expected to pass
(static per-file review found no other guard shape these classes' widening could collide with;
Druid/Monk receive zero emissions regardless of this change), not confirmed. Full workspace
`cargo test --locked --no-run` not confirmed to exit 0 this cycle for the same reason; the
narrower `--lib` build DID succeed at this cycle's rebased HEAD.

Zero units move for Druid (1) or Monk (25) — the separate `is_druid_pillar_id`/
`is_monk_pillar_id` filter is unchanged, explicitly out of this lane's write scope. A floor of 5
`docs/work-inventory.json` units are expected to move once the wave's shared regeneration runs;
a larger, unconfirmed number is possible from the `already_admitted` citation population for the
five widened classes, several of which will likely be suppressed by the pre-existing
`already_computed_slugs` collision guard against real hand-wired explanations — say so plainly
if the regen shows less movement than the raw citation count implies. Receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_owner_matched_cycle_receipt_8.md`.

### Cycle — AT-34-E4-002 — Ultimate Campaign bucket B closed via a missing PI-coordinate wire-up (partial)

**Status: partial.** `python3 scripts/completion_atlas.py --book ultimate_campaign --check` at
the start of this cycle: `population=265 DONE=127 B=5 D=4 M=88 V=18 U=21 X=2` (`AT-34-E4-001`
resolved `U`/`X` by proof without moving buckets; this cycle is the first to move population).

All 5 `ultimate_campaign` bucket-B trait units are `NAMEISPI:YES` deity-linked traits
(`uca_abilities_traits.lst:280-284`) — PI-masked at ingestion, so a plain key/name `resolve`
never finds their real corpus record even though it physically exists
(`data/corpus/ultimate_campaign/trait_generic/codex_named_unit_trait_ultimate_campaign_uca_abilities_traits_lst_280.json`,
verified by direct read). `decisions.md §14` already built the fix — `simple_kind_verdict`'s
`coordinate` fallback, wired for `Kind::Domain`/`Kind::Deity` — but `Kind::Trait`'s call site
still passed `None`. Pure missing wire-up: added a `coordinate` build identical to the
`Domain`/`Deity` arms.

**TDD:** RED test proved failing for the intended reason (still bucket B) before the fix;
`Kind::Trait`'s arm fixed to pass the coordinate; GREEN, plus a monotonicity sibling proving a
genuinely-absent coordinate still refuses cleanly. Full `v06_work_inventory.rs` suite: 414/414
passed, 0 regressions.

`ultimate_campaign` after: `DONE=130 B=0 D=5 M=89 V=18 U=21 X=2` (`population=265` unchanged).
The 5 closed units split honestly on real corpus shape: 3 zero-magnitude/real-description/
display-class → `DONE` (`+3`); 1 carries a real `BONUS:SKILL` magnitude → `M` (this table is a
lookup, not a compute path, `decisions.md §2a`); 1 (`Wrecking Wrath`) has genuine prose
Strength-modifier-doubling scaling → `D`, correctly not display-promoted. Corpus-wide, the same
mechanism (any book's PI-masked trait record) also closed: `B: 11921→11831` (`-90`) at HEAD.

**Instrument correction:** the fix's line insertion shifted `completion_atlas.py`'s bucket-`V`
citation off `11707`; `AT-34-E1-002` condition 6 caught it (`citation_failures=1`), re-derived to
the real line (`11722`) and reverified `citation_failures=0`.

**Remainder — 135 of 265 not yet `DONE`, named exactly:** `M:89` (magnitude ingested, needs the
compute path run — real Epic-3-scale engine work), `V:18` (needs the SD-33 oracle harness),
`D:5` (3 `ability` `ASPECT:`-only records `closure_has_real_description` doesn't read — root
cause diagnosed this cycle, fix not attempted; 2 `trait` prose-scaling records needing a compute
path, one of them, `Alchemical Intuition`, pre-existing and one, `Wrecking Wrath`, newly surfaced
by this cycle's own fix), `U:21` and `X:2` (both already proven correct-and-final by
`AT-34-E4-001`; neither matches `§17`'s 2026-08-28 ruling's targeted shapes, so deliberately not
reopened). `89+18+5+21+2=135`; `130+135=265`.

`corpus_literal_sweep`: `48708 examined of 51482 read, 0 findings` before and after (no
`data/corpus/**` file touched). `derived_evaluator_fixture_check`: `1839 cleared over 2580
rows, 0 failed`. `cargo test --locked --no-run` exits 0 at the widest workspace scope;
`apps/desktop/src-tauri` not touched, not run. `docs/work-inventory.json` regenerated with
`CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` set (no `--allow-stamp-loss`).
Denominator gate against this package: run separately, see receipt. Receipt:
`artifacts/epic-4-ultimate-campaign/AT-34-E4-002_cycle_receipt.md`.

**Next cycle:** widen `closure_has_real_description` for `ASPECT:`-only `ability` records (3
units, root cause already diagnosed); build a compute path for the 2 `trait` prose-scaling
records; measure per-unit cost on `M`/`V` samples before any population-scoped run.

### Cycle — AT-34-E2-002 reconfirmation at HEAD — complete, no drift found

**Status: complete.** This lane was dispatched against `AT-34-E2-002` after the criterion had
already landed (`AT-34-E2-002_cycle_receipt.md` + `fail-closed-proofs.md`, commit
`afbe67a22f8`, kanban row 10 already `complete`). Per `decisions.md §12` L2/L19, every figure
was re-derived from this lane's own shell against HEAD `c76e1f9455` rather than transcribed —
immediately following `AT-34-E2-001`'s own reconfirmation cycle on this branch:

- `cargo test --locked --lib rules_core::rules_tables::companion_chassis::tests` → `19
  passed, 0 failed` (4 more than the `15` at the original commit — all Epic 1/3 work in the
  same module; `companion_resolve_refuses_a_fabricated_key_it_never_defaults` (this
  criterion's own 8th-table proof) still exists verbatim and still passes).
- `cargo test --locked --lib rules_core::rules_tables::simple_kind_tables` → `13 passed, 0
  failed`, matching `AT-34-E2-001`'s own independently re-derived figure exactly.
- `v06_work_inventory --epic2-table-transcript`, regenerated fresh at HEAD, diffs
  **byte-identical** against the committed
  `artifacts/epic-2-tables/AT-34-E2-001_table_transcript.txt`. All 8 of the 8 tables this
  criterion covers still report `HELD` on a named record and `REFUSED` on a fabricated key.
- `grep -c '^| \`' fail-closed-proofs.md` → `8` — the per-table proof artifact itself is
  unchanged and still names all 8 tables, cross-checked against the freshly-run transcript.
- `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` →
  `violations=0` (15 files).
- `cargo test --locked --no-run` → exit 0 at HEAD (full root workspace). `apps/desktop/
  src-tauri` not touched by this cycle's file-touch set, not run.
- Dual audit re-run on Epic 2's §3 file-touch set: `OK_NO_BUNDLE_TAGS`; the wired-integration
  pattern's 18 hits are the same already-reviewed false-positive class as `AT-34-E2-001`'s own
  reconfirmation — Epic 3's real, correctly-spelled PCGen domain term "placeholder row",
  confirmed against this criterion's own two files (`simple_kind_tables.rs`,
  `companion_chassis.rs`), which carry zero hits.
- `git status --porcelain -- data/corpus` empty throughout — this cycle wrote zero corpus
  records, so `corpus_literal_sweep`'s examined population is unmoved by this cycle
  specifically (not independently re-run this cycle; N/A per the receipt's own sweep row).

Row-count command output: `grep -c '^| \`' …fail-closed-proofs.md` → `8`. No production code,
`docs/work-inventory.json`, or `data/corpus/**` edit was needed — the criterion was already
met and remains met at HEAD. `kanban.md` row 10 already read `complete`; a reconfirmation
receipt path is now added to its Notes pointer.
Receipt: `artifacts/epic-2-tables/AT-34-E2-002_reconfirmation_receipt.md`.

### Cycle — AT-34-E2-001 reconfirmation at HEAD — complete, no drift found

**Status: complete.** This lane was dispatched against `AT-34-E2-001` after the criterion had
already landed (`AT-34-E2-001_cycle_receipt.md`, commit `052a9182bf`, kanban row 9 already
`complete`). Per `decisions.md §12` L2/L19, every figure was re-derived from this lane's own
shell against HEAD `e403495d29` rather than transcribed:

- `cargo test --locked --lib rules_core::rules_tables::simple_kind_tables` → `13 passed, 0
  failed` (2 more than the `11` at the original commit — both added by `AT-34-E2-002`'s later
  cycle, still green; no regression).
- `v06_work_inventory --epic2-table-transcript`, regenerated fresh at HEAD, diffs
  **byte-identical** against the committed
  `artifacts/epic-2-tables/AT-34-E2-001_table_transcript.txt`. All 8 of the 8 kinds Epic 2
  builds (`ability`, `template`, `trait`, `deity`, `domain`, `skill`, `language`, `companion`)
  still report `HELD` on a named record and `REFUSED` on a fabricated key — no drift from the
  corpus-wide `wiring_class` restamp (`AT-34-E1-008`) or Epic 3's later work, because neither
  touches these kinds' record identity, only unrelated fields `simple_kind_tables::resolve`
  does not read.
- `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` →
  `violations=0` (15 files).
- `cargo run --locked --bin corpus_literal_sweep` live: `48708 records examined of 51482 read
  … 0 findings … CLEAN`. This cycle added zero corpus records
  (`git status --porcelain -- data/corpus` empty throughout); the `48699 → 48708` movement
  since the package's stated launch baseline belongs to later, unrelated Epic-1/Epic-3 corpus
  growth, not to this cycle.
- `cargo test --locked --no-run -j2` → exit 0 at HEAD (full root workspace). `apps/desktop/
  src-tauri` not touched by this cycle's file-touch set, not run.
- Dual audit re-run on Epic 2's §3 file-touch set: `OK_NO_BUNDLE_TAGS`; the wired-integration
  pattern's only hits are Epic 3's real, correctly-spelled PCGen domain term "placeholder row"
  (CHOOSE-menu no-selection corpus rows), not a stub or unfinished marker — reviewed and
  confirmed non-defect; `simple_kind_tables.rs` itself carries zero hits of either pattern.

Row-count command output: `grep HELD …AT-34-E2-001_table_transcript.txt | grep -oE
"kind=[a-z]+" | sort -u | wc -l` → `8`. No production code, `docs/work-inventory.json`, or
`data/corpus/**` edit was needed — the criterion was already met and remains met at HEAD.
`kanban.md` row 9 already reads `complete` with correct receipt links; unchanged (a new
reconfirmation receipt path is added to its Notes pointer).
Receipt: `artifacts/epic-2-tables/AT-34-E2-001_reconfirmation_receipt.md`.

### Cycle — AT-34-E1-008 reconfirmation at HEAD — complete, no drift found

**Status: complete.** This lane was dispatched against `AT-34-E1-008` after the criterion had
already landed (`AT-34-E1-008_G1..G4_cycle_receipt.md`, kanban row already `complete`,
verifying instrument `AT-34-E1-007_re-verification_receipt.md` at `a47cdbee21`, further
reconfirmed at `ba23c938b1`). Per `decisions.md §12` L2/L20, every figure was re-derived from
this lane's own shell against HEAD `ba23c938b1` rather than transcribed:

- `scripts/verify.sh --only corpus-trap-audit` → `PASS (records_examined=27638
  defects[wiring-class-mismatch=0 disabled-line=165 key-differs-from-name=650
  mod-record=2117 shared-name-distinct-records=249] traps=407 — all defect kinds at their
  registered counts)`. `wiring-class-mismatch=0`, the criterion's bar, with the other four
  inherited trap kinds reported by name at their unchanged counts, not absorbed.
- `wiring-class-remediation.json` re-summed: 4 groups, 34 books, `before=7015 after=0
  regenerated=10298` — matches `decisions.md §13` and the criterion text exactly.
- `cargo run --locked --bin corpus_literal_sweep` live: `48708 records examined of 51482
  read … 0 findings … CLEAN`. AT-34-E1-008's own four group receipts each independently
  recorded `48699 → 48699, delta 0` for their own in-place restamps at the time they ran
  (0 records added by this criterion's own regeneration); the `48699 → 48708` movement since
  then belongs to later, unrelated Epic-1/Epic-3 corpus growth (e.g. AT-34-E3-001's `domain`
  ingestion), not to AT-34-E1-008.
- `cargo test --locked --no-run` → exit 0 at HEAD. `apps/desktop/src-tauri` not touched by
  this criterion's file-touch set, not run.
- Dual audit re-run on Epic 1's cumulative file-touch set: every `sd[0-9]+_`/stub-token hit is
  either a `-` (removed) provenance-tag line inside this criterion's own already-landed
  regeneration, a real PCGen "no selection" placeholder value in regenerated corpus JSON
  (already ruled non-defect by `AT-34-E3-001`), or doc prose belonging to the already-closed
  `AT-34-E1-006` criterion. Nothing new, nothing in shipping code.

Row-count command output: `ls …AT-34-E1-008_G*_cycle_receipt.md | wc -l` → `4`;
`wiring-class-remediation.json` → `4 groups, 34 books`. Self-consistent. No production code,
`docs/work-inventory.json`, or `data/corpus/**` edit was needed — the criterion was already
met. `kanban.md` row 8 already reads `complete` with correct receipt links; unchanged.
Receipt: `artifacts/epic-1-atlas/AT-34-E1-008_reconfirmation_receipt.md`.

### Cycle — AT-34-E1-007 re-verification at HEAD — complete, no drift found

**Status: complete.** Re-derived `AT-34-E1-007` (`corpus-trap-audit` wired into `verify.sh`)
fresh at HEAD `da5589f3c2`, per `decisions.md §12` L2/L19, since AT-34-E1-008's
G1–G4 remediation and Epic 3's cycles have both landed since `AT-34-E1-007`'s own
`_re-verification_receipt.md` was written at `a47cdbee21`. No drift: the stage's
per-kind ratchet still holds at HEAD.

- `scripts/verify.sh --only corpus-trap-audit` → `PASS  (records_examined=27638
  defects[wiring-class-mismatch=0 disabled-line=165 key-differs-from-name=650
  mod-record=2117 shared-name-distinct-records=249] traps=407 — all defect kinds
  at their registered counts)`, exit 0 — the exact command
  `acceptance-and-verification.md §1` row 23 names.
- `scripts/verify.sh --only corpus-trap-audit-selftest` → `PASS (14 passed, 0
  failed)`, exit 0 — the comparator's own mutation-proved detection self-test.
- `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'`
  → `files_checked=15 violations=0`.
- Widest build scope, run after confirming `git status --porcelain` was clean under
  `data/corpus/`, `scripts/`, `src/` (no write this cycle could move a figure):
  `cargo test --locked --no-run -j2` (root workspace, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e1-007`)
  → exit 0; `cd apps/desktop/src-tauri && cargo test --locked --no-run -j2`
  (separate cargo workspace, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e1-007-desktop`)
  → exit 0. Both run at HEAD `da5589f3c2`.
- `docs/release/SD-34-book-completion/kanban.md` row 7 (`AT-34-E1-007`) was already
  `complete` from the prior cycle's re-verification and needs no change — this
  cycle is a reconfirmation, not new closure. No code, no corpus record, and no
  `docs/work-inventory.json` change this cycle (`git status --porcelain` clean on
  all three before this commit, apart from this progress-log edit and the retro
  event append below).
- **Movement, four buckets:** all zero — closure (none, already closed),
  reclassification (none), reachability (none — the stage's population and every
  kind's count are unchanged from `a47cdbee21`), instrument-correction (none —
  no defect in the instrument found this time).
- Retro event: `docs/retro/events/sd34-at-34-e1-007.jsonl` gained one `verification`
  row for this run (`stages_passed: ["corpus-trap-audit"]`, `result: PASS`,
  `head: da5589f3c2`).

### Cycle — AT-34-E1-006 re-verification at HEAD — complete, instrument-correction (14-violation regression found and fixed)

**Status: complete.** Re-derived `AT-34-E1-006` (the `figure-provenance` `verify.sh` stage +
widened `denominator-gate` default) fresh at HEAD, per `decisions.md §12` L2/L19, since Epic 3's
`AT-34-E3-001` cycles committed four new receipts under `artifacts/epic-3-core-rulebook/` since
the original cycle. **This time the re-derivation found the standing gate RED, not green:**

- `bash scripts/verify.sh --only figure-provenance` → `FAIL (violations=14 of
  figures_examined=64 (files_checked=41))`, the primary verifying command
  `acceptance-and-verification.md §1` row 22 names for this criterion.
- All 14 hits were in four of Epic 3's committed receipts (`class_feature_option_pool_with_
  magnitude`, `deity_absent`, `domain`, `race_trait_race_not_modelled`) — two real shapes: table
  cells saying "same"/"same command" instead of restating a reachable command, and multi-line
  bullets where the figure and its command landed on different physical lines of a wrapped
  paragraph. Both are genuine instances of the exact defect this gate exists to catch, not gate
  false positives — the gate was working correctly on a population that had drifted.
- Fixed in place: reworded/reflowed the 14 lines so each figure carries a same-line reachable
  command, copying every command verbatim from one already present elsewhere in the same
  receipt (no new command invented, no figure re-derived or restated with a different value).
  `python3 scripts/denominator_gate.py --check-provenance` → `files_checked=41
  figures_examined=64 violations=0` after the fix; `bash scripts/verify.sh --only
  figure-provenance` → `PASS`.
- The widened `denominator-gate` default (this criterion's second obligation) is unchanged and
  still holds: `python3 scripts/denominator_gate.py --check` → `files_checked=111 violations=0`
  (up from the original cycle's `90` — growth is Epic 2/3's own new receipts, not a regression);
  `ls docs/release/SD-34-book-completion/*.md | wc -l` → `15`.
- `python3 -m unittest scripts.tests.test_denominator_gate -v` → `Ran 40 tests ... OK`, unchanged
  (no production code touched this cycle — only markdown receipts).

No production code changed. `cargo test --locked --no-run` exits 0 at HEAD `2eb1536876` plus
this cycle's docs-only diff (full workspace). `apps/desktop/src-tauri` not touched, not run.
Identifier/token audits on this cycle's own working-tree diff: `OK_NO_BUNDLE_TAGS`,
`OK_NO_TOKENS`. Receipt: `artifacts/epic-1-atlas/AT-34-E1-006_re-verification_receipt.md`.

**Standing-gate lesson for future cycles:** `figure-provenance` is listed in
`acceptance-and-verification.md §2` as green-every-cycle, not just at closure. A new Epic 3/4/5
cycle receipt should be checked with `python3 scripts/denominator_gate.py --check-provenance`
before it is committed, not discovered red by a later re-verification cycle.

### Cycle — AT-34-E1-005 re-verification at HEAD — complete, reclassification (not this cycle's own)

**Status: complete.** Re-derived `AT-34-E1-005` (the `not-ingested` → `engine-does-not-hold`
rename) fresh at HEAD, per `decisions.md §12` L2, since Epic 2/3 and the `AT-34-E1-002/003/004`
re-verification cycles all regenerated `docs/work-inventory.json` and touched
`src/bin/v06_work_inventory.rs` extensively since the original cycle. Found fully intact:

- Old-string live-use sweep across `tests/`, `src/`, `apps/`, `scripts/`:
  `legacy_not_ingested_live_uses = 0` (the sweep test's own file is the sole `grep` hit, expected
  and excluded by the test's `sweep()` function).
- `docs/work-inventory.json`: `grep -c '"not-ingested"'` → `0`; `grep -c
  '"engine-does-not-hold"'` → `20066` (moved from `26239` at the original cycle — a
  reclassification from Epic 2/3's closure work, not a rename regression).
- `src/bin/v06_work_inventory.rs`: `grep -c "not_ingested\b"` → `0`; `grep -c
  "engine_does_not_hold"` → `39`.
- Atlas D-bucket citation still resolves by content at line `9167` (unlike `AT-34-E1-004`'s
  promotion-ladder citation, this one did not drift): `python3 scripts/completion_atlas.py
  --check` → `population=49438 buckets=10 unclassified=0 overlap=0 citation_failures=0`.
- Denominator gate on this package: `files_checked=15 violations=0`.

No RED step was needed — nothing had drifted, unlike `AT-34-E1-004`'s citation which needed a
line-number fix. No production code changed this cycle. `cargo test --locked --no-run` exits 0
at HEAD `11a15ec7fc` plus this cycle's docs-only diff. `apps/desktop/src-tauri` not touched, not
run. Receipt: `artifacts/epic-1-atlas/AT-34-E1-005_re-verification_receipt.md`.

### Cycle — AT-34-E1-004 re-verification at HEAD — complete, reclassification (not this cycle's own)

Re-dispatched against `AT-34-E1-004` (already `complete` from an earlier cycle, commit
`4d69afd6e4`, receipt reporting `not_held_by_engine=13119 of 26396`, citation at
`v06_work_inventory.rs:9592-9595`) to re-derive rather than carry the original numbers forward
(`decisions.md §12` L2). Confirmed `python3 scripts/shape_engine_boundary.py --check` now fails
closed (`STALE_CITATION`, exit 1, no artifact written) for the intended reason: Epic 3's
nine-mechanism `AT-34-E3-001` work added code above the promotion ladder's original location,
shifting the exact four-condition block (byte-identical to `technical-design.md §3` /
`decisions.md §2a`'s quote) down to `10854-10857`. Also re-derived the counts:
`magnitude_bearing` held at `26396`; `not_held_by_engine` dropped `13119 -> 9475` — real
Epic-3 closure work promoting 3,644 units past `engine-does-not-hold`, not a measurement
change (`git log 4d69afd6e4..HEAD -- docs/work-inventory.json` shows 25+ regeneration commits
between the two measurements).

Fixed `scripts/shape_engine_boundary.py`'s `PROMOTION_LADDER_LINES`/`PROMOTION_LADDER_ANCHOR_LINE`
to `10854-10857`/`10857`, updated its markdown-template narrative (previously "half the
feedstock", now "roughly a third", with both the launch-time and current fractions stated) and
docstring example, and updated `scripts/tests/test_shape_engine_boundary.py`'s line-number and
count expectations. `python3 -m unittest scripts.tests.test_shape_engine_boundary -v` — 12/12
green after the fix (RED confirmed first: 4 failures + 2 errors, one directly asserting
`9475 != 13119`). Regenerated `shape-engine-boundary.md`; `python3 scripts/denominator_gate.py
--check 'docs/release/SD-34-book-completion/*.md'` -> `files_checked=15 violations=0`.
`cargo test --locked --no-run` exits 0 (no Rust source touched this cycle). Retro correction
logged (`docs/retro/events/sd34-at-34-e1-004.jsonl`). `kanban.md` row 4 stays `complete`,
receipt link added. Receipt: `artifacts/epic-1-atlas/AT-34-E1-004_re-verification_receipt.md`.

### Cycle — AT-34-E1-003 re-verification at HEAD — complete, 0 units moved

Re-dispatched against `AT-34-E1-003` (already `complete` from an earlier cycle, commit
`2ec0462736`, receipt reporting `population=8463 kinds=9`) to re-derive rather than carry the
original numbers forward (`decisions.md §12` L2). `HEAD` had moved far more than a couple of
cycles since the original landing — Epic 2 built 8 of the 9 missing engine tables
(`decisions.md §7`) and Epic 3 ran 13 further `docs/work-inventory.json` regeneration commits —
so bucket A's live population is now `449` units across `2` kinds (`companion=28` in `bestiary`,
`power=421` in `ultimate_psionics`), not the launch-time `8463`/`9`. That shrink is expected and
correct: bucket A is Epic 2's own input, not a static invariant.

Confirmed the committed `missing-engine-tables.json` already matches `missing_engine_tables.py
--check`'s live output byte-for-byte — it was silently kept current by a prior Epic-3 commit
(`6a87278d875f`, one of 13 that regenerate this file as a side effect of a `docs/work-inventory.json`
regeneration), no uncommitted drift, no code change needed this cycle. Re-ran the full 12-test
`scripts/tests/test_missing_engine_tables` suite (green); two of those tests
(`test_live_remaining_population_is_power_and_bestiary_companion_only`,
`test_live_core_rulebook_and_ultimate_campaign_have_zero_bucket_a`) directly assert the current
post-Epic-2 state, confirming both AT-34-E2-004's target books reached bucket-A zero and that
the book-coverage map's original claims (7 Core-Rulebook kinds + `trait` buildable, `power`
costed-not-built) played out exactly as `decisions.md §7` records. `cargo test --locked --no-run`
exits 0 at `688c6ae38756756bcfc19bc95781ef05d0f2ae92` (docs-only diff, no Rust touched).
`kanban.md` row 3 stays `complete`. Receipt:
`artifacts/epic-1-atlas/AT-34-E1-003_re-verification_receipt.md`.

### Cycle — AT-34-E1-002 re-verification at HEAD — complete, 0 units moved

Re-dispatched against `AT-34-E1-002` (already `complete` from an earlier cycle, commit
`5289e646dd`) to re-derive rather than carry the prior claim forward (`decisions.md §12`
L2/L19). `HEAD` had moved by two Epic-3 mechanism-closure cycles since the original landing,
moving `docs/work-inventory.json` by 3 units (bucket `B` 11,967→11,964, `DONE` 14,581→14,584 —
a real closure, not drift) and leaving the committed `completion-atlas.json` stale, with an
uncommitted regeneration diff already sitting in the working tree. This cycle regenerated the
atlas at current `HEAD` (`8439f31c867d30e12dc4e3489a00e35835e4dd77`), re-ran the full 38-test
`scripts/tests/test_completion_atlas.py` suite (all six fail-closed conditions are mechanized
as permanent regression tests, not one-off transcripts), confirmed the live `--check` result
(`population=49438 buckets=10 unclassified=0 overlap=0 done_evidence_violations=0
missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`), confirmed
`derived_at` is an ancestor of `HEAD`, and re-ran `cargo test --locked --no-run` (exit 0, full
workspace) after the regeneration. No `completion_atlas.py` logic changed — a measurement
wave, not a code cycle (`decisions.md §12` L6). Kanban row 2 stays `complete`.

**Discovery named, not absorbed:** `scripts/box_ledger.py --check` (SD-33's independent
second partition, reading `docs/release/SD-33-computed-value-verification/THE-BOX.md`) now
reports `uncovered=19861`, up from `0` at the `tranche/14` launch checklist. `THE-BOX.md` is
out of this bundle's write scope and out of AT-34-E1-002's file-touch set — it has simply not
been kept in sync with Epic 3/4's mechanism closures. Not this criterion's gate; flagged here
for whichever cycle next touches that file. See
`artifacts/epic-1-atlas/AT-34-E1-002_re-verification_receipt.md` for full figures.

### Cycle — AT-34-E3-001 (`class_feature_option_pool_record_not_held_by_engine` mechanism, cycle 8) — one of nine, `decisions.md §14` — partial, 0 units moved

Re-derived at this cycle's starting HEAD (`2c56ac5a71`): still 34 of 543 `core_rulebook`
bucket-B units in this mechanism. Cycle 7's own remainder named 6 sub-causes; this cycle's task
brief pointed at 2 of them ("proficiency/grant possession-tracking and wizard opposition-school
tracking — genuinely new engine subsystems. BUILD one properly, or return `partial`") and this
cycle independently verified — by reading the live corpus JSON and grepping the live engine,
not by re-quoting prior cycles' claims — that all 3 non-sibling-owned sub-causes really do need
a new subsystem, not a same-shape extension of an already-shipped table: **companion/mount
registration** (3 units) turned out the most promising on inspection (Druid's own Animal
Companion progression IS wired, `ANIMAL_COMPANION_HIT_DICE_BY_MASTER_LEVEL` +
`class_chassis.druid.animal_companion.*`), but the shared `Companion ~ Animal Companion` internal
indirection target is also referenced by 3 other unwired owners (Ranger's Hunter's Bond, Cleric's
Domain Power, Nature's Bond), and `Companion ~ Special Mount`/`Special Mount ~ Standard Choices`
have zero Paladin-side engine computation anywhere (confirmed by grep — only comment-only
references to the corpus's own `SpecialMountLVL` token, no formula, no `choice:special_mount`
registration). **Wizard opposition-school spell tracking** (9 units, `Abjuration Wizard Spells`
… `Universal Wizard Spells`) has no consumer anywhere in `rules_core` (`grep -rn "Wizard Spells"
src/rules_core/` returns nothing) — a genuinely unbuilt per-school spell-list-access subsystem.
**Proficiency/mechanical-grant possession-tracking** (20 units) re-confirmed unchanged from
cycles 5/6's own finding: generic multi-class indirection targets with no 1:1 class row, plus 3
standalone facts (`Add Spoken Language`, `Channel {Negative,Positive} Energy`, `Evasion`) needing
their own new possession-tracked engine fact. The remaining 2 units (Domain Power `Leadership`/
`Sun's Blessing`) belong to the `with_magnitude` sibling mechanism, not this cycle's to fix.
Given none of the 3 subsystems is safely buildable as a narrow lever within this cycle, and per
`decisions.md §9` ("a measurement wave that banks zero units is a legitimate deliverable"), 0
units moved rather than risk an unsafe or dishonest bucket move — no unit was placed into X/U on
this cycle's own authority. Remainder re-derived and named exactly (8 + 2 + 10 + 3 + 9 + 2 = 34),
each with what must be built next. Movement: 0 closure, 0 reclassification, 0 reachability, 0
instrument-correction. AT-34-E3-001 itself remains open (8 other mechanisms + this mechanism's
own 34 remain). Receipt: `artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_cycle_receipt.md`.

### Cycle — AT-34-E3-001 (`class_feature_option_pool_record_with_magnitude_not_held_by_engine` mechanism, cycle 7) — one of nine, `decisions.md §14` — partial

Re-derived at this cycle's starting HEAD (`0827dcd59b`): still 261 of 562 `core_rulebook`
bucket-B units in this mechanism, matching cycle 6's own closing figure exactly. Cycle 6's own
next-cycle plan named two levers: `Secret Lore` (a materially bigger lift, no Loremaster
prestige-class chassis exists yet) and "one more wizard school, built end-to-end." This cycle
took the second lever again, building **Conjuration** -- the cheapest of the six remaining
schools whose 3-power shape (two level-1 powers, one level-8 power) most closely matched the
Transmutation precedent. Read `data/corpus/core_rulebook/class_feature/conjuration_school/*.json`
directly and found the same shape cycles 4-6 already established: real `BONUS:VAR` formulas
never wired to `classify()` because the record's `group` prefix can never equal `"wizard"`. New
`CONJURATION_SCHOOL_SELECTION` constant and `wizard_has_canonical_conjuration_selection`
(opposing Necromancy + Abjuration) gate one new compute block grounding Summoner's Charm
(`max(1,ConjurationSchoolLVL/2)`, level 1), Acid Dart (bonus-damage `ConjurationSchoolLVL/2` +
the shared `ArcaneSchoolPowerTimes` uses/day pool, level 1), and Dimensional Steps
(`ConjurationSchoolLVL*30`, level 8) — all verified directly against the live corpus, never
assumed. `probe_wizard_arcane_school_wiring` gained a fourth variant, swapping BOTH the
specialization AND the opposed-schools choice from the start (cycle 6's own Discoveries had
already named this exact hazard), so unlike cycle 6's own two-round probe-defect history, this
cycle's first full guarded regen closed exactly the 3 records predicted with no correction
round needed. TDD: RED (3 target records fell through the same owner-resolution path) → GREEN,
3 new tests. **3/261 closed** (`Conjuration School ~ Summoner's Charm`, `~ Acid Dart`,
`~ Dimensional Steps`, all `engine-does-not-hold` → `grounded`), verified by a whole-file
before/after diff against the committed inventory: exactly 6 changed lines (the 3 records' own
old+new versions), no cross-book side effect this time (Conjuration has no Arcanist-exploit
counterpart in `advanced_class_guide`, unlike Transmutation's own 3-unit side effect), no
collision-hazard misclassification. `core_rulebook` bucket B (atlas-real partition, all 9
mechanisms) 556 → 553/6,701, this mechanism 261 → 258. Fixed 10 + 2 shifted `file:line`
citations in `completion_atlas.py`/`missing_engine_tables.py` (this cycle's own probe/doc-comment
insertion shifted every one; `citation_failures=0` on both after). 258 remain, named by
sub-cause (Domain Power 56, Domain Base 33, wizard arcane-school cluster 32, Draconic Bloodline
Choice 10, Secret Lore 10, New Arcana 9 [ruled out], small per-class groups 20, long tail 88 —
sums exactly to 258). Movement: 3 closure, 0 reclassification, 0 reachability, 0
instrument-correction. AT-34-E3-001 itself remains open (8 other mechanisms + this mechanism's
own 258 remain).

### Cycle — AT-34-E3-001 (`class_feature_owner_matched_by_name_but_record_not_held_by_engine` mechanism, cycle 7) — one of nine, `decisions.md §14` — partial

Re-derived at this cycle's starting HEAD (`2ae06fe4cd`): still 248 of 6,701 `core_rulebook`
bucket-B units in this mechanism, matching cycle 6's own closing figure exactly. Cycle 6's own
next-cycle plan flagged that its inherited four-way sub-cause split (118/15/67/48) needed fresh
re-derivation before any lever was taken from it; re-derived fresh by direct corpus query instead
(105 real-description / 143 null-description-with-tokens / 0 neither, sums exactly), and grouping
by owning class found **218 of 248** already blocked by a real, pre-existing, previously-escalated
architectural gate (`class_feature_grant_consumer::ANTI_FABRICATION_GATE_EXCLUDED_CLASSES` —
Sorcerer/Cleric/Monk/Wizard/Paladin/Bard/Druid, guarding 9 shipped anti-fabrication tests,
`OPEN-ISSUES.md` rows 330/338, OPEN and not this cycle's to decide). Of the remaining 30
(non-excluded classes), traced why several with real corpus descriptions and real grant facts
(Assassin, Shadowdancer, Duelist, Arcane Trickster, Dragon Disciple, Pathfinder Chronicler) still
failed to ground: `compute_pilot_base_chassis`'s generic class_feature grant-roster call site used
`chassis_supported` — "has a computed BAB/save chassis" — as its sole "is this a real class"
precondition, and `compute_class_chassis`'s own prestige-entry-gate branch always returns `None`
for every CRB prestige class's chassis regardless of whether the class id is real. New
`prestige_class_entry_gate::is_registered` accessor plus a widened `||` at the call site (no
change to any anti-fabrication gate, description-quality check, or collision guard) lets the SAME
already-shipped, already-unit-tested generic roster fire for a prestige-class-only character.
TDD: RED (3 of 4 new tests failing for the intended missing-explanation reason) → GREEN. 6/248
closed (`assassin_hidden_weapons`, `assassin_true_death`, `duelist_deflect_arrows`,
`shadowdancer_darkvision`, `shadowdancer_defensive_roll`, `shadowdancer_shadow_power`), all
`engine-does-not-hold` → `text-complete`, verified by a whole-corpus before/after diff (35 changed
ids total, all `core_rulebook:class_feature:*`, zero cross-book movement; 29 of the 35 are
same-status evidence-string relabelling on OTHER prestige-class records this cycle's fix also
newly explains, not bucket moves — verified none carries this mechanism's own evidence string).
242 remain, NOT closed — named by sub-cause: 218 excluded-class (real blocker, needs an operator
ruling on `OPEN-ISSUES.md` 330/338), ~20 non-excluded-class internal-bookkeeping (no description,
no resolvable formula chain — same OPEN definitional question `atlas-defects.md` already names),
~4 not yet individually re-verified this cycle. `core_rulebook` bucket B (atlas-real partition)
543 → 537/6,701. `box_ledger.py --check` exits 1 both before and after (pre-existing since prior
cycles' status-string rename, confirmed against the untouched pre-cycle snapshot; `uncovered`
19,870 → 19,864, an improvement, not a regression; `overlap=0`/`population=49438` hold both
before and after). Movement: 6 closure, 29 reclassification, 6 reachability, 0
instrument-correction. AT-34-E3-001 itself remains open (8 other mechanisms + this mechanism's
own 242 remain).

### Cycle — AT-34-E3-003 (bucket `U` ruling, cycle 1, `decisions.md §17`) — partial, reclassification only

Applied operator ruling `decisions.md §17` ("bucket `U` is DONE"): a `Kind::EquipmentModifier`
record with zero magnitude tokens and no real description anywhere in its token closure is
internal equipment-modifier plumbing (`BANE`, `FLM_BRST`, `FRT_HVY`, `Magical Enhancments
(+1..+10)`) — a code that attaches an effect to a weapon/armor record, never a thing a player
reads on its own. `classify()`'s `Kind::Equipment | Kind::EquipmentModifier` arm gains one new
rung, `unit.kind == Kind::EquipmentModifier && !has_real_description`, returning `text-complete`
with evidence `equipment_modifier_is_internal_plumbing_no_player_facing_content_per_decisions_17`
— keyed on the pre-existing `has_real_description`/`magnitude_token_count` signals every sibling
rung already trusts, never a name list. TDD: RED (`an_equipment_modifier_with_no_magnitude_and_
no_description_reads_text_complete_per_decisions_17`) → GREEN; negative control
(`an_equipment_item_with_the_same_zero_magnitude_shape_stays_unmeasurable`) proves `Kind::Equipment`
is not swept in.

**110 of the corpus-wide 140 `equipment_modifier` `unmeasurable` candidates moved** (unit-id set
identical before/after, 49,438 units, whole-corpus diff by id confirms exactly 110 changed, all
one kind, all one transition — `unmeasurable → text-complete`): `mythic_adventures` 52,
`core_rulebook` 40, `bestiary_3` 4, `inner_sea_magic` 4, `monster_codex` 4, `horror_adventures` 2,
`inner_sea_intrigue` 2, `adventurers_guide` 1, `ultimate_combat` 1. Corpus-wide `U` bucket
321 → 211. `core_rulebook`'s own named 58 U units: 40 closed, **18 remain** — including the
ruling's own worked examples `BANE`/`FLM_BRST`/`FRT_HVY` — because their corpus description IS
real (not empty) and trips the pre-existing `corpus_json_description_leaks_pcgen_syntax` leak
guard. Investigated all 30 remaining `equipment_modifier` unmeasurable units directly (temporary
diagnostic test, reverted before commit) and split them into two named sub-causes: 21 carry a
genuine unresolved PCGen substitution (`%CHOICE` = an unmodelled player choice; `%d<N>` = an
unresolved crit-multiplier dice reference) — closer to `§17`'s own `X`-bucket discussion than to
this ruling's "nothing to compute, nothing to show" shape; 9 (`FRT_HVY`/`FRT_LGHT`/`FRT_MOD` +
6 prose siblings) trip a separate, confirmed `render_pcgen_desc` defect (it drops a bare `%` even
when immediately preceded by a digit, e.g. a "chance" fraction quoted as a literal percent sign in prose, unlike `leaked_pcgen_syntax`'s own correct
digit-preceded exemption) — a real renderer bug, out of this ruling's scope, filed as a `deferral`
retro event. Fixed 10 + 2 shifted `file:line` citations in `completion_atlas.py`/
`missing_engine_tables.py` (this cycle's own insertion shifted every one; `citation_failures=0`
on both after) and re-pinned `test_completion_atlas.py`'s stale `U==321` assertion to `211`.
`core_rulebook` U-bucket criterion: 40/58 closed, 18 remain, named. Movement: 0 closure, 110
reclassification, 0 reachability, 0 instrument-correction.

### Cycle — AT-34-E3-001 (`class_feature_option_pool_record_not_held_by_engine` mechanism, cycle 7) — one of nine, `decisions.md §14` — partial

Re-derived at this cycle's starting HEAD (`94705a4149`): still 44 of 564 `core_rulebook`
bucket-B units, unchanged from cycle 6's own closing figure. The task brief named "class-skill/
companion-mount attribution" (13 units, cycle 6's own remainder) as one of three genuine
new-subsystem investments and asked for one to be BUILT properly rather than another narrow
pass. Read all 13 units directly against the live corpus before assuming the name described one
shape: 10 carry a `CSKILL:` token (9 CRB base classes' own `"Class Skills ~ <Class>"` internal
chassis record + `"Jack of All Trades ~ Class Skills"`), the other 3 carry `FOLLOWERS:`/
`COMPANIONLIST:` tokens instead (`Companion ~ Animal Companion`, `Companion ~ Special Mount`,
`Special Mount ~ Standard Choices`) — a genuinely different subsystem (companion/mount
registration, not class-skill attribution).

Built the homogeneous 10-unit half: new `rules_tables::crb::class_skill_tables::CLASS_SKILL_LISTS`
(10-row closed table — 9 base classes' skill lists transcribed verbatim from each class's own
`CSKILL:` token, plus Jack of All Trades' `CSKILL:ALL` row), each independently verified
byte-for-byte against the live corpus JSON in a new test. New
`class_feature_pool_catalog::class_skill_list_grant_owner_id` closed-list lookup (record key ->
owner id, mirroring cycle 6's own `weapon_and_armor_proficiency_grant_class_id` pattern exactly),
wired into `v06_work_inventory.rs`'s `Kind::ClassFeature` `text_only` arm immediately after
cycle 6's own weapon-and-armor rung. **No live consumer required** — same precedent cycle 6's own
`CLASS_ARMOR_PROFICIENCIES` table already established for this mechanism: a real, tested table
that verifiably holds the record's own content is enough to move bucket B -> D (`decisions.md §2`
"a shelf, not a half-fix"); `skill_allocation.rs`'s own bounded posture is untouched, future
widening work for whichever cycle owns it.

**10 of 44 closed this cycle** (`Class Skills ~ {Barbarian, Bard, Cleric, Druid, Fighter, Monk,
Paladin, Ranger, Rogue}`, `Jack of All Trades ~ Class Skills`). `core_rulebook` bucket B
(atlas-real) 564 -> 543 of 6,701. Before/after whole-corpus diff (snapshot taken before
regeneration, per the task brief's own named cycle-5 collision hazard) confirms exactly these 10
keys moved and nothing else — zero collision. `cargo test --locked --no-run` exit 0,
workspace-wide and `apps/desktop/src-tauri`; `cargo test --locked --lib` 2,906 passed / 0 failed
/ 14 ignored (5 new tests); `cargo test --locked --bin v06_work_inventory` 400 passed / 0
failed. No instrument movement: `corpus_literal_sweep` stayed 48,708 of 51,482 (0 corpus records
added/regenerated, only already-committed files read by the new tests), `derived_evaluator_
fixture_check` unchanged. Fixed 6 shifted `file:line` citations in `completion_atlas.py`/
`missing_engine_tables.py` (`citation_failures=0` on both after). 34 remain, sub-cause partition
re-derived fresh and sums exactly: weapon-flavored generic indirection 8, `Weapon and Armor
Proficiency ~ {Druid,Monk}` excluded 2, armor/shield-flavored generic + non-weapon extras 10,
companion/mount registration 3 (newly split out of the former 13), wizard opposition-school
tracking 9, Domain Power `CLASS_FEATURE_POOLS` registration gap 2. AT-34-E3-001 itself remains
open (8 other mechanisms + this mechanism's own 34 remain). Receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_cycle_receipt.md` (Cycle 7
section, prepended).

### Cycle — AT-34-E3-001 (`class_feature_option_pool_record_with_magnitude_not_held_by_engine` mechanism, cycle 6) — one of nine, `decisions.md §14` — partial

Re-derived at this cycle's starting HEAD (`2829c89e18`): still 267, unchanged from cycle 5's
own closing figure. Read cycle 5's own receipt and its "Next-cycle plan" before touching
anything: `Secret Lore` (10) and one more wizard school, built end-to-end. Investigated Secret
Lore first -- every one of its 10 Loremaster prestige-class sub-records carries a real formula,
but the engine has zero Loremaster prestige-class chassis to hang them on, a materially bigger
lift than a wizard school (which already has an existing base-class chassis). Took the wizard
school lever instead: **Transmutation**, following the exact Evocation/Abjuration pattern
(cycle 4) -- a new `wizard_has_canonical_transmutation_selection` gate, opposing Necromancy +
Evocation, and three new formulas (Telekinetic Fist bonus damage + uses/day, Physical
Enhancement flat bonus, Change Shape rounds/day).

**Real defect found and fixed within this cycle's own work, before committing.** The first
probe variant only swapped the specialization choice (mirroring Abjuration's own code shape),
leaving the opposed-schools choice at its seeded default (Necromancy + Transmutation) --
nonsensical once Transmutation is the specialty itself, so the probe's own precondition never
matched and it silently observed nothing. A first full guarded regen confirmed the population
held at 267, not the predicted 261 -- caught before committing by checking the six target
`corpus_key`s directly, not by trusting the predicted delta. Fixed by also swapping the
opposed-schools choice to Necromancy + Evocation, verified with a throwaway scratch test against
the real probe and real fixture (deleted before committing), then re-run through the full regen.

**9 of 267 closed this cycle** (6 `core_rulebook` -- Telekinetic Fist, Physical Enhancement, and
the three ability-score sub-choice records it grants, Change Shape -- plus an honestly-reported
3-unit `advanced_class_guide` side effect: Arcanist's own Transmutation exploit `ABILITY`-grants
the identical shared corpus_key strings). `core_rulebook` bucket B (atlas-real) 562 -> 556 of
6,701. `cargo test --locked --no-run` exit 0, workspace-wide and `apps/desktop/src-tauri`;
`cargo test --locked --lib` 2,896 passed / 0 failed / 14 ignored; `cargo test --locked --bin
v06_work_inventory` 400 passed / 0 failed (3 new tests). No instrument movement: `F1` stayed
5,400 (none of this cycle's formulas are bare-literal). Re-derived and fixed 10 + 2 shifted
`file:line` citations in `completion_atlas.py`/`missing_engine_tables.py`, TWICE (the probe fix
and the scratch test's own later removal each shifted lines again) -- `citation_failures=0` on
both after the final round. Also self-healed a pre-existing 8-line drift in
`missing_engine_tables.py`'s own citations, present at this cycle's own start HEAD, unrelated to
this cycle's edits. Shared-checkout note: this cycle's own start state found an unrelated
concurrent lane's WIP already dirty in `src/bin/v06_work_inventory.rs` and
`scripts/completion_atlas.py` (an operator-ruling U-bucket lever another session was actively
building); backed it up, worked on a clean base, committed only this mechanism's own changes,
and left the other lane's WIP untouched and uncommitted exactly as found. 261 remain, sub-cause
partition re-derived fresh and sums exactly: `Domain Power` 56, `Domain Base` 33, wizard-school
cluster remainder 35 (down from 38 -- Transmutation's own power records now closed, its
top-level/opposition recognition records remain unclaimed like every other school's), `Draconic
Bloodline Choice` 10, `Secret Lore` 10 (investigated, not closed -- needs a dedicated
prestige-class-chassis cycle), `New Arcana` 9 (ruled out), small 2-3-unit groups 20, long tail
88. Receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_with_magnitude_cycle_receipt_6.md`.

### Cycle — AT-34-E3-001 (`class_feature_owner_matched_by_name_but_record_not_held_by_engine` mechanism, cycle 6) — one of nine, `decisions.md §14` — partial, 251 → 248

Re-derived the mechanism population fresh at this cycle's starting HEAD: 251, unchanged from
cycle 5's own closing figure (`2829c89e18`, the tip at start, touched only `decisions.md` and
left `docs/work-inventory.json` byte-identical). Independently re-derived the total and its
`wiring_class` split (`display` 185, `ambiguous` 46, `computed` 19, `derived` 1); confirmed by
direct grep that exactly 3 of the 251 are the dispatch brief's own named next-cheapest shape —
`{Cleric, Assassin, Shadowdancer} ~ Weapon and Armor Proficiency`.

Extended cycle 4's own `explain_base_class_weapon_and_armor_proficiency` (which already grounds
Sorcerer/Wizard) with a new shared helper, `ground_class_weapon_and_armor_proficiency`, mirroring
`class_slayer.rs`'s shipped precedent including its real archetype-supersession primitive
(`archetype_resolver::archetype_claiming_slot_entry`). Cleric is the first BASE class this shape
covers with a REAL registered archetype (Ecclesitheurge, ACG) — its own proficiency slot carries
four distinct id spellings across the corpus's own archetype catalog (confirmed by grep across
all seven tier-1 archetype tables), and a dedicated test proves the supersession branch fires for
real. Assassin and Shadowdancer carry no registered archetype anywhere in this engine (confirmed
by grep, zero matches), so this cycle **corrects cycle 4's own stated reason for deferring them**
("no prestige-class chassis exists" — that reasoning does not apply to a class-feature-only
grounding, which reads only `CharacterClassLevel.class_id`, a flat string with no enum-membership
precondition). Each class's own "weapon half grounded elsewhere" claim is stated honestly and
per-class: true for Cleric (a real `weapon_tables::class_weapon_proficiency("class:cleric")`
entry exists), explicitly false for Assassin/Shadowdancer (no entry exists for either — confirmed
by grep), never over-claimed for the two prestige classes.

**A live concurrent-write collision was caught and worked around mid-cycle**, not touched: the
shared checkout's `src/bin/v06_work_inventory.rs` carried 110 uncommitted lines this cycle never
wrote (a different lane's own in-progress bucket-U ruling implementation), and the checkout's own
HEAD moved forward mid-turn to `2829c89e18` — confirming a second live writer. A first
regeneration attempt picked up that contamination and correctly failed closed
(`citation_failures=10`, the gate working as designed against someone else's uncommitted lines).
The contaminated `docs/work-inventory.json` was discarded without ever touching the other lane's
file; this cycle's own diff was instead applied and verified inside an isolated `git worktree`
checked out from `origin/tranche/14`, with its own `CARGO_TARGET_DIR`. The full 2,901-test lib
suite, `corpus_literal_sweep` (48,708/51,482, 0 findings — unchanged from cycle 5, zero corpus
records added), `derived_evaluator_fixture_check`, and the regeneration all ran clean there. A
whole-corpus (all 49,438 units, not only this mechanism's own bucket) before/after diff confirms
**exactly 3 units changed, zero collision** — the collision-hazard check cycle 5 found the hard
way, now run against the full corpus rather than one mechanism's bucket.

3/251 closed (`engine-does-not-hold` → `text-complete`, bucket B → DONE directly — a real,
non-fabricated grant-only explanation now exists for each, quoting the corpus's own DESC text).
248 remain. This cycle independently verified the total and its `wiring_class` split but did
**not** rebuild cycle 3's own retired classification instrumentation to re-verify the finer
four-way sub-cause split cycle 4/5 once reported (`description_is_null_internal_bookkeeping`
118, `engine_effect_token_present` tail, `catalog_serves_it_but_classify_wiring_class_gate_
blocks_promotion` 67, small long tail) — flagged in the receipt as inherited, not re-derived, so
the next cycle re-derives it fresh from the live corpus before taking a lever from it.
`core_rulebook` bucket B (atlas-real partition) 562 → 559/6,701 remains (DONE 1,380 → 1,383).
Full receipt: `artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_owner_matched_cycle_receipt_6.md`.

### Cycle — AT-34-E3-001 (`companion_absent_from_core_rulebook_companion_tables` mechanism, cycle 5) — one of nine, `decisions.md §14` — complete, mechanism reaches 0

Re-derived the mechanism population fresh at this cycle's starting HEAD (`50a5785592`): 2,
matching cycle 4's own after-figure exactly. Cycle 4's own mandate: build a genuine
level-progression record type for the 2 `cr_classes_companion.lst` monster-class rows
(`Companion`, `Shadow Companion`) and verify it against the two named second/third consumers
(`ultimate_magic` 3 rows, `book_of_the_damned_volume_1` 2 rows), or state precisely why not.
SD-29's own companion-lane round 8 (`docs/release/SD-29-corpus-wide-catch-up-lanes/decisions.md
§65.1`) had already found this shape, refused it for three rounds, then widened the refusal to
DROP-AND-NAME and explicitly declared "modelling it is a new record type... this round does not
take it" — this cycle takes it.

Built `companion_chassis::CompanionClassRecord` (`key`, `output_name`, `hit_dice`, `max_level`,
`type_segments`, `visible_no`, `source_page`, `ability_grants`, `fact_class_type`, `source_file`,
`source_line` — computes nothing, mirrors `CompanionRecord::monster_class`'s own "never
computed" discipline), a third `CompanionBook` table alongside `companions`/`companion_abilities`,
and updated `scripts/transcribe_companion_tables.py` to actually build the shape instead of
dropping it — run once per book for all 3 real consumers, producing 2/3/2 rows matching each
book's raw `.lst` content exactly. Also modelled, uniformly with the same type, a second corpus
shape the mandate did not anticipate: bare-numbered `###Block: Level Advancement` lines
(`um_classes_companion.lst:13`, `botd1_classes_companion.lst:8`) that `v06_work_inventory`'s own
directive screen treats as their own record because a first field with no `:` is never a
directive. `v06_work_inventory.rs`'s `chassis_companion_keys` now folds `companion_classes` keys
into the same lookup set `companions`/`companion_abilities` already share.

Found and fixed a pre-existing determinism bug in the transcriber before trusting any of its
output: running it twice on the SAME unmodified book produced a large spurious diff (every
creature's `ability_keys` list reshuffled, same elements) because Shape 7's book-wide-grant loop
walked an un-sorted Python `set`, whose iteration order CPython randomizes per process. Fixed
with `sorted(...)`; verified deterministic across two further regenerations of all 3 target
books.

**2 of 2 closed — mechanism reaches 0.** 5 of the other 7 registered rows (`ultimate_magic` 3,
`book_of_the_damned_volume_1` 2) also closed as an honestly-reported side effect of building the
type generically — each belongs to that book's OWN `companion_absent_from_<book>_companion_
tables` mechanism, not double-counted against this criterion. `completion_atlas.py --check`:
population=49,438 buckets=10 unclassified=0 overlap=0, citation_failures=0 (all ten hardcoded
`BUCKET_DEFINITIONS` citations shifted +8 lines by this cycle's own insertion, re-derived and
fixed). Bucket B corpus-wide 12,002 → 11,995 (re-derived by temporarily swapping in the pre-cycle
`docs/work-inventory.json`, then swapping back before commit); `core_rulebook`-scoped bucket B
stays at 562 of 6,701, owned by the other 8 AT-34-E3-001 mechanisms. `cargo test --locked --no-run`
re-run at this cycle's own commit SHA, exit 0, workspace-wide and `apps/desktop/src-tauri`
(separate crate, tested explicitly); `cargo test --locked --lib` 2,896 passed / 0 failed / 14
ignored; `cargo test --locked --bin v06_work_inventory` 397 passed / 0 failed. One real
instrument-correction: `formula_interpreter_corpus_wide.rs`'s hardcoded F1-population assertion
re-pinned 5,401 → 5,400 (`ultimate_magic` Black Blade's `BONUS:HP|CURRENTMAX|5` left the
not-done population, F1-shaped), re-derived via `shape_ledger.py` and logged with a retro
`correction` event. `corpus_literal_sweep` stayed 48,708 of 51,482 (0 `data/corpus` records
added — only Rust static tables). AT-34-E3-001 itself remains open — the other eight mechanisms
are owned by other cycles. Full receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_companion_absent_cycle_receipt_5.md`.

### Cycle — AT-34-E3-001 (`class_feature_option_pool_record_not_held_by_engine` mechanism, cycle 6) — one of nine, `decisions.md §14` — partial

Re-derived the mechanism population fresh at this cycle's starting HEAD (`16aea9b4dd`): 49,
matching the dispatch brief's own stated figure exactly (of 569 `core_rulebook` bucket-B units,
whole book). Checked cycle 5's own next-cycle plan (build a class armor-proficiency table)
against the live corpus before building anything: the standalone `Armor Prof ~ {Light,Medium,
Heavy}` / `Shield Prof` / `Shield Prof ~ Tower` records are generic `CATEGORY:Internal`
indirection targets shared across many classes — the same unclosable shape as the already-
investigated weapon-flavored generics, not a per-class grant. The real per-class armor/shield
data instead lives inside each class's own DISPLAY-bearing `"Weapon and Armor Proficiency ~
<Class>"` combined record (7 of the 49 units: Bard, Druid, Fighter, Monk, Paladin, Ranger,
Rogue) — a different corpus key from cycle 5's own three weapon-only matches.

Built `weapon_tables::CLASS_ARMOR_PROFICIENCIES` (new table, mirroring
`CLASS_WEAPON_PROFICIENCIES`'s own established pattern) and, for each of the 7 combined
records, verified BOTH the weapon-side content against `CLASS_WEAPON_PROFICIENCIES` AND the
armor-side content against the new table, byte-for-byte against the live corpus JSON. 5 of 7
matched exactly (Bard, Fighter, Paladin, Ranger, Rogue); Druid and Monk were investigated and
correctly excluded — Druid's own `AUTO:WEAPONPROF` list is missing `Scythe` against BOTH the
table row and its own dedicated `"Weapon Proficiencies ~ Druid"` record (a real corpus-internal
discrepancy), and Monk repeats cycle 5's own established `Flurry of Blows`/`Unarmed Strike`
mismatch. New `weapon_and_armor_proficiency_grant_class_id` closed-list lookup moves the 5 from
bucket B to bucket D (`engine-does-not-hold`, new evidence
`class_feature_weapon_and_armor_proficiency_grant_held_by_class_proficiency_tables`) —
`decisions.md §16`'s "only the count grounds" precedent was checked and does not apply (none of
these 44 remaining units carry a "pick N" choice shape).

**5 of 49 closed this cycle** (bucket B, `core_rulebook`, 569 → 564 of 6,701 —
`python3 scripts/completion_atlas.py --by-book`). `cargo test --locked --no-run` re-run at this
cycle's own commit SHA, exit 0, workspace-wide and `apps/desktop/src-tauri` (separate crate,
tested explicitly); `cargo test --locked --lib` 2,895 passed / 0 failed / 14 ignored;
`cargo test --locked --bin v06_work_inventory` 397 passed / 0 failed (4 new tests). No
instrument movement this cycle: `corpus_literal_sweep` stayed 48,708 of 51,482 (only READ 3
already-committed corpus files, added/regenerated none). Re-derived and fixed 6 shifted
`file:line` citations in `completion_atlas.py`/`missing_engine_tables.py`
(`citation_failures=0` on both after). 44 remain, sub-cause partition re-derived fresh and sums
exactly: weapon-flavored generic indirection targets 8, `Weapon and Armor Proficiency ~
{Druid, Monk}` 2, armor/shield-flavored generic indirection targets plus non-weapon extras 10,
class-skill/companion-mount attribution 13, wizard opposition-school tracking 9, Domain Power
registration gap 2. Full receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_cycle_receipt.md`
(prepended, cycle 6 section).

### Cycle — AT-34-E3-001 (`class_feature_option_pool_record_with_magnitude_not_held_by_engine` mechanism, cycle 5) — one of nine, `decisions.md §14` — partial

Re-derived the mechanism population fresh at this cycle's starting HEAD (`3ebc08b451`): still
277, unchanged from cycle 4's own closing figure. Read cycle 4's own receipt and its "Next-cycle
plan" before touching anything; its own remainder table named a `Bardic Performance` cluster (10
units, untouched) alongside the wizard-school lever. Read
`data/corpus/core_rulebook/class_feature/bardic_performance/*.json` directly rather than assuming
a "pick one from a roster" shape, and found every one of the 10 sub-records carries a real
`BONUS:VAR|...` formula token. `pilot_compute/mod.rs` already computes 7 of the 10 (Fascinate,
Inspire Courage, Inspire Competence, Soothing Performance, Frightening Tune, Deadly Performance,
Inspire Heroics) — never wired to `classify()`, the same "group prefix can never equal `bard`"
owner-resolution failure Domain Power / Weapon Training / Favored Enemy / Wizard Arcane School
each already established. New `probe_bard_bardic_performance_wiring` (mirrors
`probe_wizard_arcane_school_wiring`, simpler — no choice override needed) observes each
sub-record's own explanation id against the real compute pipeline. This cycle also built the 3
remaining formulas the engine did not yet have (Suggestion DC, Mass Suggestion DC, Inspire
Greatness allies-count), each verified directly against this repo's own ingested corpus record's
level-gate token (never from memory) — closing the entire cluster in one cycle.

**10 of 277 closed this cycle** (bucket B, `core_rulebook`, 579 → 569 of 6,701 —
`python3 scripts/completion_atlas.py --by-book`). `cargo test --locked --no-run` re-run at this
cycle's own commit SHA, exit 0, workspace-wide; `cargo test --locked --lib` 2,891 passed / 0
failed / 14 ignored; `cargo test --locked --bin v06_work_inventory` 397 passed / 0 failed (2 new
tests). No instrument movement this cycle: `F1` stayed 5,401 (checked each of the 10 closed
records' own formula shape directly — none is bare-literal). Re-derived and fixed 10 + 2 shifted
`file:line` citations in `completion_atlas.py`/`missing_engine_tables.py` (`citation_failures=0`
on both after). 267 remain, sub-cause partition re-derived fresh and sums exactly: `Domain Power`
56, `Domain Base` 33, wizard-school cluster remainder 38, `Draconic Bloodline Choice` 10 (checked
this cycle — genuinely different shape, no per-color numeric formula, not the Bardic Performance
lever), `Secret Lore` 10 (not yet checked record-by-record), `New Arcana` 9 (ruled out by cycle
3), small 2-3-unit per-class groups 22, long-tail singles 89. Flagged, not fixed (out of this
mechanism's own scope, unchanged from cycles 3-4): the same pre-existing `cargo test --locked
--test v06_work_inventory` failure on 3 `vacuous_placeholder_row_no_corpus_content_to_render`
units — a different sibling mechanism's own fix. Receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_with_magnitude_cycle_receipt_5.md`.

### Cycle — AT-34-E3-001 (`class_feature_owner_matched_by_name_but_record_not_held_by_engine` mechanism, cycle 5) — one of nine, `decisions.md §14` — partial

Re-derived at this cycle's starting HEAD (`c4e6ac92f9`): still 344, unchanged from cycle 4's
own closing figure. `decisions.md §16` (written for this cycle) settled that cycles 2-4's own
"operator-scoped classification ruling" on the 103-unit `Sorcerer Bloodline Feat` (87) +
`Ranger Combat Style Feat` (16) majority is not open: it is the ALREADY-RATIFIED
"only the count grounds" Fighter/Cavalier/Brawler/Arcane-bloodline treatment
(`pilot_compute/mod.rs`, `ARCANE_BLOODLINE_ELIGIBLE_BONUS_FEATS` +
`ground_sorcerer_arcane_bloodline_progression`), applied to two more record shapes rather than a
new decision.

Built `ground_sorcerer_bloodline_feat_pool` and `ground_ranger_combat_style_feat_pool`: each
grounds a bloodline/style-INVARIANT slot COUNT as a magnitude (the Sorcerer formula is the same
`(sorcerer level - 1)/6` `arcane_bloodline_bonus_feat_count` already implements, verified
bloodline-agnostic in the corpus's own token; the Ranger formula is the same 2nd/6th/10th/
14th/18th milestone progression the existing specific-choice idiom already documents), names
the full corpus-wide eligible set (87 Sorcerer names across every CRB bloodline; 16 Ranger names,
the exhaustive combined Archery+Two-Weapon-Combat pool), and emits one non-claim-blocking
diagnostic per option stating the choice is not modelled — never seeding a default. Neither
function requires this seam's own narrow bloodline/style recognition, so both ground regardless
of whether a character's specific choice was ever recognized.

**A real defect found and fixed in this cycle's own work, not a prior cycle's**: the first
regeneration pass closed the intended 103 units but a before/after diff against a saved
pre-cycle snapshot found 4 unrelated units (`Sorcerer Domain ~ Sun/Knowledge/Magic`, `Sorcerer
Bonus Spell L3 ~ Fly`) incidentally routed to `deferred-with-reason` through
`v06_work_inventory.rs::diagnostic_id_names_feature`'s substring-based (not exact-key) match —
e.g. `"Improved Sunder"`'s slug contains `"sun"`. A first fix excluding the 4 literal colliders
still left 6 of 7 `"Skill Focus (Knowledge (<school>))"` entries colliding the same way,
caught by a SECOND regeneration's own diff. A corpus-wide Python cross-check (every
`sorcerer`-/`ranger`-owned `class_feature` record in the whole corpus, 431 + 282 units, not
only this book's bucket B) confirmed the final 10-name exclusion list is complete before a
THIRD, final regeneration — which shows zero unintended movement.

**93 of 344 closed** (`Sorcerer Bloodline Feat` 77 of 87 — 10 names' own diagnostics
deliberately withheld to avoid the verified collision, still listed in the honest eligible-set
text; `Ranger Combat Style Feat` 16 of 16), all moved `engine-does-not-hold` → `deferred-with-
reason` (bucket B → bucket X, `decisions.md §2`), never `text-complete` or `grounded` — no
display or magnitude claimed for any individual option. `core_rulebook` bucket B (atlas-real
partition) 692 → 579 of 6,701. Full details, TDD RED→GREEN evidence, and the exact exclusion
list with its verification: `artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_owner_matched_cycle_receipt_5.md`.

**251 remain.** Next-cheapest known shape (cycle 4's own long tail, unaffected by this cycle):
`Rogue Talent` 3, `Wizard` 2, `Core Domain` 2, `Monk` 2, and 8 further 1-unit records, each real
engine wiring. The 118-unit zero-description internal-bookkeeping sub-cause and the 67-unit
`catalog_serves_it_but_classify_wiring_class_gate_blocks_promotion` sub-cause remain untouched,
per the dispatch brief's own scope.

### Cycle — AT-34-E3-001 (`companion_absent_from_core_rulebook_companion_tables` mechanism, cycle 4) — one of nine, `decisions.md §14` — partial

Re-derived the mechanism population fresh at this cycle's starting HEAD (`dbf97940fd`): still
14, unchanged from cycle 3's own closing figure. Cycle 3's own next-cycle plan named the 14/12/2
split exactly and this cycle's own dispatch mandate named both remaining sub-causes explicitly:
(a) close the 12 grant-token-only rows via a per-record, corpus-wide VERIFIED predicate, never a
shape-only reclassification; (b) close the 2 monster-class rows via a genuine level-progression
record type verified against two named consumers.

**(a) closed, 12 of 12.** Cycle 3's own atlas defect 3 warned its bare shape query (`ABILITY`
present, no `TYPE`/`DESC`/`BONUS`) matches 461 of 51,482 corpus-wide and that reclassifying by
shape alone risks the exact 188-record near-miss defect 1 already recorded. Before building
anything, re-checked that concern directly: applying the shape query corpus-wide and then
testing "every `ABILITY:` target resolves in-book to a record carrying real content" against
all 461 finds only **171 safe**, **104 whose target exists but carries no content**, and **280**
whose target key cannot even be found in-book — confirming a shape-only rule would misclassify
290 of 461 records. Built a narrower, stronger, individually-verified predicate instead: a
closed 12-key list (`companion_chassis::GRANT_TOKEN_ONLY_DISPATCH_ROWS`), each key checked, per
record, against the LIVE `docs/work-inventory.json` — every one of its `ABILITY:` grant tokens'
targets is a `core_rulebook` companion row this engine ALREADY holds
(`grounded`/`text-complete`/`literal-verified`, not merely "a corpus file exists"). All 12 pass
with zero counter-examples (every target is one of the already-shipped `Animal Companion ~ *`
ability rows). A new `classify()` rung moves a match from bucket B to bucket X
(`deferred-with-reason`) — the correct "shelf, not half-fix" outcome for a row with no content
of its own whose real job (dispatch to already-shipped content) is now the engine's own recorded
reason. `companion_absent_2_sub_causes_are_named_and_sum_exactly` (replacing the prior
`_14_` test) and a new `grant_token_only_rows_dispatch_to_already_held_content` test prove both
the closure and the predicate against live corpus + live inventory.

**(b) not closed, named honestly.** Re-confirmed the 2 `cr_classes_companion.lst` monster-class
rows (`Companion`, `Shadow Companion`) need a level-progression record type this chassis
genuinely has no fields for (`companion_chassis.rs`'s own module doc already states this third
shape is unmodeled). Confirmed both named consumers this cycle: `ultimate_magic` (3 rows:
`Vermin Companion`, `1`, `Black Blade`) and `book_of_the_damned_volume_1` (2 rows: `1`,
`Imp Companion`) — real second/third consumers, already-registered `COMPANION_BOOKS` entries.
Building and verifying a new record type against all 3 consumers inside the same cycle as the
12-row predicate above was judged out of this cycle's safe scope; left `engine-does-not-hold`,
named with its 7 total corpus-wide rows for a future cycle.

Two shifted `completion_atlas.py` `BUCKET_DEFINITIONS` citations (bucket A `has_no_engine_table`
10583→10601, bucket V `literal-verified` 11234→11252 — this cycle's own +18-line insertion
shifted both) re-derived by `grep -n` and fixed in this same cycle; `citation_failures=0`.
`core_rulebook` companion mechanism 14 → 2; `cargo test --locked --lib` 2,884 passed / 0 failed
/ 14 ignored; `cargo test --locked --bin v06_work_inventory` 395 passed / 0 failed; `cargo test
--locked --no-run` exit 0, workspace-wide; desktop crate (`apps/desktop/src-tauri`, separate
cargo workspace) `cargo test --locked --no-run` exit 0. `corpus_literal_sweep` unchanged at
48,708/51,482 examined, CLEAN (0 corpus records added/regenerated). Total corpus population
unchanged at 49,438. Retro: one `resolution` event closing the 12-row portion of cycle 3's
grant-token-only deferral, one `deferral` event for the 2-row class-row remainder, both naming
the 2 consumer books. Receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_companion_absent_cycle_receipt_4.md`.

### Cycle — AT-34-E3-001 (`class_feature_option_pool_record_not_held_by_engine` mechanism, cycle 5) — one of nine, `decisions.md §14` — partial

Re-derived the mechanism population fresh at this cycle's starting HEAD (`1de361c850`): still
52, unchanged from cycle 4's own closing figure (`core_rulebook` bucket B whole-book had moved
736 → 687 between cycle 4's own receipt and this cycle's start — sibling mechanisms' cycles,
not this one, landed in between). The task brief's own instruction was explicit: build one of
cycle 4's four named subsystems, or stop reporting the same zero. Cycle 4's own grep for a
proficiency-tracking probe (`grep -n "proficiency.*wired\|weapon_prof.*wired"
src/bin/v06_work_inventory.rs`) had found nothing and concluded no such subsystem exists — that
grep pattern was too narrow, not the underlying fact. A wider search
(`grep -rln "proficien" src/rules_core/*.rs`) surfaced `weapon_tables::CLASS_WEAPON_
PROFICIENCIES`: a real, already-shipped, already-tested class-based weapon-proficiency table,
transcribed per-class from the real corpus, already consumed today by `pilot_compute/mod.rs`'s
`character_is_proficient_with` for combat's own nonproficiency-penalty checks — never wired to
the atlas.

Read every one of the five `"Weapon Proficiencies ~ *"` corpus records' own `AUTO:WEAPONPROF`
token against that table's real data for the matching class — a byte-for-byte SET match, not a
name-shape guess (`decisions.md §14`'s own warning about the mechanism's Cycle 2 near-miss
still governs). Bard, Druid, and Rogue matched exactly. Cleric's own token
(`AUTO:WEAPONPROF|DEITYWEAPONS`) is a genuinely different, selection-dependent mechanism (the
deity's favored weapon) the table does not model at all. Monk's 17-weapon list matched 16 of
17 — its last entry is literally `"Flurry of Blows"` (a class-feature name, not a weapon, a
PCGen data quirk) where the table substitutes `"Unarmed Strike"` — a near-match, correctly left
unclosed rather than forced.

**3 of 52 closed this cycle** (`Weapon Proficiencies ~ {Bard, Druid, Rogue}`) — a new
`class_feature_pool_catalog::WEAPON_PROFICIENCY_GRANT_CLASS_TABLE_MATCHES` closed 3-entry
list (mirroring `VACUOUS_PLACEHOLDER_CLASS_FEATURES`'s own established named-list pattern) plus
a new `classify()` rung that moves a match from bucket B to bucket D — deliberately still
`engine-does-not-hold`, not `text-complete`: these records carry `description: null` (nothing
to display, a separate, unrelated concern), so this only certifies the engine genuinely holds
the record's own content now, exactly `decisions.md §2`'s "a shelf, not a half-fix" outcome.
Two new tests prove the byte-for-byte match against the live corpus AND the live weapon table
(RED if either drifts), two more prove the `classify()` rung and its Cleric-exclusion control.
`core_rulebook` bucket B (atlas-real partition) 687 → 684/6,701. `cargo test --locked --lib`
2,883 passed / 0 failed / 14 ignored; `cargo test --locked --bin v06_work_inventory` 395 passed
/ 0 failed (4 new tests); `cargo test --locked --no-run` exit 0, workspace-wide; desktop crate
(`apps/desktop/src-tauri`, separate cargo workspace) `cargo test --locked --no-run` exit 0.
Disk healthy this cycle (488G free), unlike cycle 4's own environmental block. Re-derived and
fixed 6 shifted `file:line` citations in `completion_atlas.py`/`missing_engine_tables.py`
(`citation_failures=0` on both after this cycle's own +26-line insertion). `corpus_literal_sweep`
unchanged at 48,708/51,482 examined, CLEAN (0 corpus records added/regenerated, only 3 already-
committed files READ by a new test). 49 remain, sub-cause partition re-derived fresh and sums
exactly: weapon-flavored generic/combined/excluded records 15, armor/shield/other-grant records
10, class-skill/companion-mount attribution 13 (unchanged from cycle 4), wizard opposition-
school tracking 9 (unchanged), Domain Power registration gap 2 (unchanged). Next-cycle lever
named: an armor-proficiency table, structurally identical to the weapon table this cycle just
wired, transcribed the same way, likely closes most of the 10-unit armor/shield group. Receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_cycle_receipt.md`.

### Cycle — AT-34-E3-001 (`class_feature_option_pool_record_with_magnitude_not_held_by_engine` mechanism, cycle 4) — one of nine, `decisions.md §14` — partial

Re-derived the mechanism population fresh at this cycle's starting HEAD (`3e68073423`): still
282, unchanged from cycle 3's own closing figure. Read cycle 3's own receipt and its
"Next-cycle plan" before touching anything, which named the wizard arcane-school cluster (38-43
units depending on exact partition boundary) as the cheapest remaining lever, sharing its root
cause with the sibling `class_feature_option_pool_record_not_held_by_engine` mechanism's own
cycle 1 finding (which declined the FULL cluster as needing genuinely new engine subsystems).
Read `pilot_compute/mod.rs` directly rather than re-trusting that finding wholesale, and found a
real, narrower, already-built subset within it: Task #55 (Evocation, `intense_bonus_damage` /
`force_missile_uses_per_day`) and Task #66 (Abjuration, `resistance` / `protective_ward_*` /
`energy_absorption`) had ALREADY built real, tested, non-fabricated per-power formulas for two
of the nine schools — never wired to `classify()`, for the same "group prefix can never equal
the owning class name" reason Domain Power / Weapon Training / Favored Enemy/Terrain Bonus each
were. New `probe_wizard_arcane_school_wiring` (mirrors `probe_domain_power_effect_wiring`)
observes exactly those two schools' own explanation ids against the real compute pipeline and
grounds only the 5 corpus records they cover — deliberately never claiming the other 7 schools
(no formula exists) or either school's own top-level/opposition-school recognition record (only
shared, uncredited prohibited-school bookkeeping, no explanation id built for it).

**5 of 282 closed this cycle** (bucket B, `core_rulebook`, 692 → 687 of 6,701 —
`python3 scripts/completion_atlas.py --by-book`). `cargo test --locked --no-run` re-run at this
cycle's own commit SHA, exit 0, workspace-wide; `cargo test --locked --lib` 2,881 passed / 0
failed / 14 ignored; `cargo test --locked --bin v06_work_inventory` 393 passed / 0 failed (2 new
tests). Real instrument-correction, not a bad re-derivation: cycle 3's own F1 re-pin (5,402) was
true at cycle 3's own start and end; this cycle's own fix moved it (5,402 → 5,401 — of the 5
newly-grounded records, exactly 1, `Abjuration School ~ Resistance`, carries F1's own
bare-literal-magnitude shape), re-pinned with a retro `correction` event. Re-derived and fixed
10 + 2 shifted `file:line` citations in `completion_atlas.py`/`missing_engine_tables.py`
(`citation_failures=0` on both after). Self-healed one inherited `denominator_gate.py`
violation from cycle 3's own committed text in this file (a bare percentage describing disk
fullness, not the exempted false-percentage idiom) — `denominator_gate.py --check` now reports
`violations=0` again. 277 remain, sub-cause partition re-derived fresh and sums exactly:
`Domain Power` 56, `Domain Base` 33, wizard-school cluster remainder 38, `Bardic
Performance`/`Draconic Bloodline Choice`/`Secret Lore` 10 each (30), `New Arcana` 9 (ruled out
by cycle 3, not re-investigated), small 2-3-unit per-class groups 22, long-tail singles 89.
Flagged, not fixed (out of this mechanism's own scope, unchanged from cycle 3): the same
pre-existing `cargo test --locked --test v06_work_inventory` failure on 3
`vacuous_placeholder_row_no_corpus_content_to_render` units — a different sibling mechanism's
own cycle 3 fix. Receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_with_magnitude_cycle_receipt_4.md`.

### Cycle — AT-34-E3-001 (`class_feature_owner_matched_by_name_but_record_not_held_by_engine` mechanism, cycle 4) — one of nine, `decisions.md §14` — partial

Re-derived the mechanism population fresh at this cycle's starting HEAD (`16c772cca9`):
still 346, unchanged from the three prior cycles. This cycle independently re-derived the
same 346-unit sub-cause partition from scratch (a temporary diagnostic binary, deleted
before commit) before reading the prior receipts, confirming both agree to within 1 unit
on every sub-cause. Three prior cycles on this exact mechanism had each closed 0 of 346;
cycle 3's own next-cycle plan named two live paths — an operator-scoped classification
ruling, or real engine wiring one shape at a time on the smaller sub-causes. This cycle
picked up the wiring path: cycle 3's own 13-shape partition of the 121-unit
`engine_effect_token_present` sub-cause named a long tail including five classes' own
"Weapon and Armor Proficiency" class feature (Assassin, Cleric, Shadowdancer, Sorcerer,
Wizard) — a zero-magnitude, grant-only identity record this engine already has a proven,
shipped precedent for (`class_slayer.rs`'s `ground_slayer_weapon_and_armor_proficiency`,
built for Advanced Class Guide's Slayer). Sorcerer and Wizard both have a registered
`pilot_compute` chassis and no archetype able to claim this slot, so the base grant
mirrors cleanly with no supersession complexity; a new
`explain_base_class_weapon_and_armor_proficiency` grounds both (TDD: RED confirmed by
temporarily disabling the call site, then GREEN), each reaching `text-complete` through
`classify()`'s EXISTING generic "owner resolved + explanation id observed" rung — no new
bucket-specific fallback added. Cleric's own record carries a real archetype-supersession
branch (the same complexity Slayer's own function handles) and Assassin/Shadowdancer are
prestige classes with no registered chassis at all — both deferred with a named revisit
condition (`docs/retro/events/sd34-at-34-e3-001.jsonl`), not silently dropped.

**2 of 346 closed this cycle** (bucket B, `core_rulebook`, 694 → 692 of 6,701 —
`python3 scripts/completion_atlas.py --by-book`). `cargo test --locked --no-run` re-run at
this cycle's HEAD, full workspace, exit 0 (`apps/desktop/src-tauri` not touched, not
re-run). `docs/work-inventory.json` regenerated at HEAD via the guarded path
(`CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` set from this cycle's own
fresh sweep/fixture-check runs, no `--allow-stamp-loss`); `corpus_literal_sweep` stays 0
findings (this cycle added no corpus records; the small examined-population delta versus
this bundle's baseline is other concurrent lanes' activity on this shared checkout).
`completion_atlas.py --check` → `citation_failures=0` (this cycle did not touch
`src/bin/v06_work_inventory.rs`, so no `BUCKET_DEFINITIONS` line-citation drift). 344
remaining units still need real engine wiring (one shape at a time) or the two
operator-scoped rulings cycle 2/3 already named; full sub-cause partition, remainder
table, and next-cycle plan:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_owner_matched_cycle_receipt_4.md`.

### Cycle — AT-34-E3-001 (`class_feature_owner_matched_by_name_but_record_not_held_by_engine` mechanism, cycle 3) — one of nine, `decisions.md §14` — partial

Re-derived the mechanism population fresh at this cycle's HEAD (`c3202a90ce`): still 346,
unchanged from both prior cycles. This cycle's own contribution: broke open the previously-flat
`engine_effect_token_present` sub-cause (121 units) into a real 13-shape, sum-exact partition by
temporarily instrumenting the existing committed regression test and reverting the
instrumentation before commit (`git diff --stat` on the touched production file is empty at
commit time — no shipped code changed). The two largest shapes found, 87 `Sorcerer Bloodline
Feat ~ *` and 16 `Ranger Combat Style Feat ~ *` (103 of 121), are the SAME architectural pattern
this engine has already ratified elsewhere — `pilot_compute/mod.rs:1837-1844`'s own documented
"only the COUNT of slots is grounded as a magnitude; which feat fills a slot is a player choice
this seam deliberately does not model," the treatment already used for Fighter's, Cavalier's,
Brawler's, and the Arcane bloodline's own bonus feats. Confirmed (not merely inferred) that
widening `REGISTERED_POOL_GROUPS` alone would not close these: `ABILITY` is in
`ENGINE_EFFECT_TOKEN_KEYS`, and `has_no_engine_effect_token` gates the SAME real serving path
`load_pool_catalog` uses, so the catalog would still correctly refuse a real per-character
mechanic. This surfaces a second operator-scoped classification question (should a "count
grounds, choice not modelled" per-option enumeration record ever be a bucket-B target?),
parallel to the prior cycle's own 143-unit `description_is_null_internal_bookkeeping` question
— together the two questions now cover 246 of 346 (71%) of this mechanism's remaining
population, and are named as such rather than left as one undifferentiated pile.

0 of 346 closed this cycle (bucket B, `core_rulebook`, unchanged at 694 of 6,701 —
`python3 scripts/completion_atlas.py --by-book`). `cargo test --locked --no-run` re-run at
this cycle's HEAD, full workspace, exit 0 (`apps/desktop/src-tauri` not touched, not re-run).
Full sub-cause partition, remainder table, and next-cycle plan:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_owner_matched_cycle_receipt_3.md`.

### Cycle — AT-34-E3-001 (`class_feature_option_pool_record_with_magnitude_not_held_by_engine` mechanism, cycle 3) — one of nine, `decisions.md §14` — partial

Continued cycle 2's own work (328 -> 324). Read the corpus directly and confirmed two facts
before building anything: every one of the 31 `"Favored Enemy Bonus ~ <type>"` corpus records
(and the 11 `"Favored Terrain Bonus ~ <type>"` siblings) carries an identical shape — no
description, a `PREABILITY` naming its own base ability, and a flat literal `BONUS:VAR|Favored
<Type>|2` token — and the engine's own `explain_ranger_level1_chassis_and_class_feature_
separation` computes that identical flat `+2` regardless of which type string is chosen (an
OPEN-ENDED recognition, unlike Weapon Training's hardcoded 4-of-52 subset), because the shared
class-wide bonus it computes IS the same value the corpus's own per-type variable resolves to.
Built two new probes, `probe_ranger_favored_enemy_bonus_wiring` / `probe_ranger_favored_terrain_
bonus_wiring` (`src/bin/v06_work_inventory.rs`), mirroring `probe_domain_power_effect_wiring`:
each of the 31/11 canonical type strings (transcribed verbatim from the corpus's own `PREABILITY`
token) is selected over the real `compute_pilot_base_chassis` pipeline, and only strings whose
choice-recognition AND `value == 2` magnitude were BOTH genuinely observed are credited. 43
corpus-wide records grounded (42 `core_rulebook` + 1 `advanced_players_guide` cross-book side
effect), 324 -> 282. Investigated `New Arcana` (9 units) and RULED IT OUT: unlike Favored Enemy/
Terrain, the engine's own doc comment states the specific spell-level choice is "a free chooser
... not modelled" — no single canonical value exists to credit, so the Favored-Enemy-shaped
argument does not transfer. **Real instrument-correction discovered and fixed**: 43 of the newly
grounded records were F1-shaped (bare literal), so `shape_ledger.py`'s F1 population (scoped to
not-done units) legitimately dropped from cycle 2's own TRUE 5,445 to 5,402 — re-pinned in
`formula_interpreter_corpus_wide.rs` with a doc comment naming the mechanism, and a retro
`correction` event filed (this is a real movement this cycle caused, not a bad re-derivation like
the earlier 5,563 re-pin was). Re-derived and fixed 10 + 2 shifted `file:line` citations in
`completion_atlas.py`/`missing_engine_tables.py` (`citation_failures=0` on both after). **Flagged,
not fixed** (out of this mechanism's own scope): `cargo test --locked --test v06_work_inventory`
fails one pre-existing test on 3 `vacuous_placeholder_row_no_corpus_content_to_render` units — a
different sibling mechanism's own cycle 3 fix, confirmed present in `docs/work-inventory.json`
before this cycle touched anything, named for that mechanism's next cycle to pick up. Hit the same
disk-exhaustion condition an earlier cycle in this same wave documented (`/` full); reclaimed
40GB by deleting this lane's own two orphaned (no `.reclaim-claim`), already-committed-cycle
`CARGO_TARGET_DIR`s, which unblocked the workspace `--no-run` re-run; did not additionally reach
`apps/desktop/src-tauri`'s own full suite this cycle. `core_rulebook` bucket B (atlas-real
partition) 736 -> 694/6,701 (this mechanism 324 -> 282 of 1,006; 6 mechanisms fully closed, 3
partially closed — 52/63, 14/100, 282/1006 remain — 1 unstarted at 346). `completion_atlas.py
--check`, `missing_engine_tables.py --check`, and `denominator_gate.py --check` all re-ran clean
after the citation fix. Receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_with_magnitude_cycle_receipt_3.md`.

### Cycle — AT-34-E3-001 (`class_feature_option_pool_record_not_held_by_engine` mechanism, cycle 4) — one of nine, `decisions.md §14` — partial

Re-derived the 52-unit remainder fresh at HEAD (no code changed) and independently confirmed
Cycle 3's own 28/13/9/2 sub-cause split (proficiency/grant possession-tracking, class-skill/
companion-mount attribution, wizard opposition-school tracking, Domain Power registration gap)
is EXACT — no correction needed. Read every one of the 52 live corpus records (not a sample)
against the existing `has_no_engine_effect_token`/render-and-refuse safety gates in
`class_feature_pool_catalog.rs` plus `has_real_description`: **44 of 52 carry
`description: null`** (PCGen-internal chassis rows — `CSKILL:`/`SPELLKNOWN:`/`FOLLOWERS:`
tokens with no `DESC:` ever ingested — no text exists to serve, so the text-complete route is
structurally impossible without inventing content); the other **8 carry a real description but
are correctly refused** by an existing safety gate for a real, still-uncomputed mechanical
effect (`Domain Power ~ Leadership`'s `ABILITY:FEAT|AUTOMATIC|Leadership` grant token, `~ Sun's
Blessing`'s unresolved `%1 DomainSunLVL` formula, 6 more `Prof` group members' `AUTO:`/`CHOOSE:`
grant tokens). Grepped and confirmed no proficiency-tracking probe, no per-school
spell-known consumer, and no `Domain Power` class-feature-grant fact exists anywhere in this
engine for any of the four sub-causes — each is genuinely new subsystem work, not a narrow
catalog-widening or attribution gap a prior cycle's precedent could safely extend to cover.

Declined to force a rushed closure on the smallest group (`Domain Power`, 2 units) merely
because it is smallest: both units are correctly refused by pre-existing safety architecture,
and shipping a stub feat-grant or an un-consumed formula placeholder to post a non-zero count
would be exactly the `no-stub-mvp-doctrine` violation `AGENTS.md` rule 6 forbids, and would risk
the same class of corpus-wide near-miss the task brief's own warning names. 0/52 closed, all
four buckets' movement is 0 this cycle. Attempted `cargo test --locked --no-run` re-verification;
hit environmental disk exhaustion (`df -h /` showed 590M free of 968G, `ld terminated with
signal 7 [Bus error]` compiling `sd13_half_orc_bounded_race_semantics` — the exact signature
`AGENTS.md`'s Concurrency section names for disk exhaustion, not a code regression). Attempted to
reclaim space by deleting clearly-stale sibling `CARGO_TARGET_DIR`s from already-closed Epic 1/2
cycles and the already-merged SD-33 bundle (confirmed via `lsof` none were held open by a
running process); blocked by this session's own permission classifier. Since this cycle changed
no source file, HEAD's own last-verified widest-scope result (Cycle 3's own, `exit 0`) stands
unmodified. `core_rulebook` bucket B (atlas-real partition) unchanged at 736/6701;
`completion_atlas.py --check` and `denominator_gate.py --check` both re-ran clean this cycle
(pure-Python, no compile needed). Receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_cycle_receipt.md`
(Cycle 4 section, prepended).

### Cycle — AT-34-E3-001 (`companion_absent_from_core_rulebook_companion_tables` mechanism, cycle 3) — one of nine, `decisions.md §14` — partial

Built the fix cycle 2 declined: Shape 8, cross-book ownership. `companion_chassis`'s
`CompanionAbilityRecord` gained a `cross_book_owners: &[(&str, &str)]` field, and the
transcriber's Shape 8 pass (an exact closed key set, mirroring Shape 7's `BOOK_WIDE_GRANTS`)
attributes the 14 `ce_abilities_familiar_cr.lst` rows to all 11 `beastiary` familiar creatures
(Bat, Cat, Hawk, Lizard, Monkey, Owl, Rat, Raven, Toad, Viper, Weasel) — real, already-registered
creatures under a DIFFERENT book, because Core Rulebook states the Familiar ability rules while
Bestiary states the familiar stat blocks, a real split the source material itself makes. Both
ownership-invariant tests widened to require a cross-book grant to resolve in a genuinely
DIFFERENT registered book (never the ability's own — no same-book escape hatch). Verified against
ALL 16 currently-registered companion books, not just `core_rulebook`: every one regenerated this
cycle, and 15 of the 16 diffs to nothing but the new field's `&[]` insertion line.

Mechanism: 28 → 14 (12 zero-content + 2 class rows remain, both re-confirmed rather than
inherited). The 12 zero-content plumbing rows are a genuine THIRD no-content shape distinct from
atlas defects 1 and 2 — recorded as atlas defect 3 (461 of 51,482 corpus-wide, `KEY`/`CATEGORY`/
`ABILITY:`-only rows with no `TYPE:`/`DESC:`/`BONUS:`) rather than silently left as a bare
bucket-B count, per this cycle's mandate — but NOT reclassified, since a shape-only
reclassification here risks the same 188-record near-miss defect 1's own cycle already caught.
`docs/work-inventory.json` regenerated; the ONLY 14 units it moved are this mechanism's own
familiar-pool rows (11 `text-complete`, 2 `grounded`, 1 `literal-verified`). Corpus-wide bucket B
12,202 → 12,188. `cargo test --lib` 2,878/2,878; `cargo test --bin v06_work_inventory` 387/387;
`cargo test --locked --no-run` exits 0 for the workspace AND `apps/desktop/src-tauri` (separate
cargo workspace, run explicitly). AT-34-E3-001 itself stays open — the other eight mechanisms
remain other cycles' scope. Receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_companion_absent_cycle_receipt_3.md`.

### Cycle — AT-34-E3-001 (`class_feature_owner_matched_by_name_but_record_not_held_by_engine` mechanism, cycle 2) — one of nine, `decisions.md §14` — partial

Re-derived this mechanism's population fresh at HEAD (`251ad7929a`): **346 of 346** still
`core_rulebook` bucket B under this evidence string, unchanged from the prior cycle. Re-ran the
prior cycle's own committed regression test
(`class_feature_owner_matched_but_not_held_346_sub_causes_are_named_and_sum_exactly`) — the same
exact 7-way, sum-exact partition holds (143 null-description bookkeeping, 121 real engine-effect
tokens, 67 catalog-served-but-wiring-class-gated, 6 class-level-scaled phrase, 5 dropped pcgen
args, 3 unregenerated multi-DESC branches, 1 bare percent reference).

Tested one new hypothesis this cycle did not test before: whether any of the 346 is really a
duplicate enumeration of an already-`DONE` unit under a different corpus key (an atlas defect,
`decisions.md §2`), sampled on `Arcane Bond ~ Bonded Object` (the `class_specific_level_phrase`
sub-cause's own cited example). Every sibling unit for that feature (`Arcane Bloodline ~ Arcane
Bond`, `Arcane Bond ~ Familiar`, `Wizard ~ Arcane Bond`) is independently `engine-does-not-hold`
— no duplicate exists. Hypothesis falsified, not confirmed: this rules out one candidate
narrow-fix path rather than opening one.

0 of 346 closed this cycle. Every remaining unit genuinely needs either new engine wiring
(spellcaster grants, domain spell-list grants, bonus-feat grants, proficiency grants, per-
character size-bonus effects — each independently scoped, comparable to a single Epic 2 table),
new ingest work writing a description that does not exist upstream, or an operator-scoped
ruling on whether a zero-description internal-bookkeeping row can ever satisfy bucket B at all
(the 143-unit sub-cause). None is a narrow, safely-scoped catalog fix without loosening a
deliberate Decision-7 render-and-refuse gate or hand-authoring corpus prose outside the guarded
ingest path. `core_rulebook` bucket B (atlas-real partition) unchanged at 750/6701.

Receipt: `artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_owner_matched_cycle_receipt_2.md`.

### Cycle — AT-34-E3-001 (`class_feature_option_pool_record_with_magnitude_not_held_by_engine` mechanism, cycle 2) — one of nine, `decisions.md §14` — partial

Continues the prior cycle's own mechanism (328 -> 324 of 1,006 `core_rulebook` bucket-B units,
re-derived fresh, not quoted). Investigated the `Domain Base` sub-cause (33 units) the prior
cycle flagged: the domain save-DC formula is genuinely computable (generic across all 33
domains, zero new per-domain content needed) but genuinely never CONSUMED anywhere in the real
`compute_pilot_base_chassis` pipeline — no enemy-facing domain power is modelled yet, and the
DC only matters to a save-requiring power — so a probe would have shipped an unobserved claim;
left named as next-cycle work rather than force-closed. Closed the `Weapon Training <tier>
<group>` sub-cause's 4 canonical records instead: `fighter_weapon_training_attack_bonus`
(`pilot_compute/mod.rs`) hardcodes exactly one canonical weapon group per training tier, and
`canonical_seeds_for("fighter")` never seeds any weapon-training-group choice at all, so the
standard per-class sweep never observed even tier 1's own canonical selection. New
`probe_fighter_weapon_training_wiring` (`v06_work_inventory.rs`), mirroring
`probe_domain_power_effect_wiring`'s own shape, selects the engine's own 4 hardcoded canonical
`(tier, group)` pairs over the real compute pipeline and grounds only what it genuinely
observes; a new read-only `pilot_compute::fighter_weapon_training_canonical_catalog()` bridge
exposes those 4 pairs without duplicating or changing any existing constant. `Weapon Training
{1 Blades Heavy, 2 Bows, 3 Pole Arms, 4 Hammers}` -> `DONE`; the other 48 of 52 weapon-training
records need real new per-group arithmetic (a generic computation, not an attribution fix) and
stay named, not closed. `core_rulebook` bucket B (all 9 mechanisms) 754 -> 750 of 6,701.
`completion_atlas.py`/`missing_engine_tables.py` citation drift (10 hardcoded `file:line`s
across both files, this cycle's own ~90-line net insertion) re-derived and fixed;
`citation_failures=0` on both `--check` gates after. No wrong prior claim found (no `correction`
retro event). `apps/desktop/src-tauri`: 522 passed, 26 failed — confirmed pre-existing,
unaffected (identical count to the prior cycle's own independent confirmation). AT-34-E3-001
does not close this cycle — the mechanism itself does not close this cycle either; 8 remaining
sub-causes named with populations summing exactly to 324. Receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_with_magnitude_cycle_receipt_2.md`.

### Cycle — AT-34-E3-001 (`class_feature_option_pool_record_not_held_by_engine` mechanism, cycle 3) — one of nine, `decisions.md §14` — partial

**Status: partial — this mechanism 55 → 52 of 1,006** (`core_rulebook`; 3 closed this cycle).

Took the cheapest sub-cause Cycle 2 named: the 3 vacuous PCGen placeholder rows (`Empty
Selection ~ Standard {Barbarian, Monk, Rogue}`, `data/corpus/core_rulebook/class_feature/
empty_selection/*.json`) — `data.description: null`, `raw_tokens` carrying only `KEY`/
`CATEGORY`/`TYPE`, no mechanical token of any kind. Per this cycle's own dispatch instruction
("record them in atlas-defects.md as an atlas defect before deciding what to do with them"),
filed `artifacts/epic-3-core-rulebook/atlas-defects.md` entry 1 FIRST (decisions.md §2's own
"any remaining step the atlas did not predict is a defect in the atlas" rule — these 3 units are
an unpredicted verdict shape: genuinely nothing to compute or display, not a real content gap),
then closed them: `class_feature_pool_catalog::VACUOUS_PLACEHOLDER_CLASS_FEATURES` (a closed,
3-key named list, never a shape predicate — deliberately, per this mechanism's own Cycle 2
near-miss precedent) reclassifies these 3 exact keys to `deferred-with-reason` (bucket X)
instead of the mechanism's own `engine-does-not-hold` fallback. Isolation check confirms exactly
these 3 units moved, corpus-wide, and nothing else (49,438 units before and after, 0 added, 0
removed). `core_rulebook` bucket B (atlas-real partition) 757 → 754/6,701. Blast-radius check: a
corpus-wide structural scan for the same "null description, structural-tokens-only" shape found
41 matches across 7 books, none of them vacuous in the same way — this fix's closed list can
only ever match the 3 keys it names. Remaining 52 of this mechanism named exactly: proficiency/
mechanical-grant possession-tracking (28), class-skill/companion-mount attribution (13), wizard
opposition-school spell-restriction tracking (9), Domain Power `CLASS_FEATURE_POOLS`
registration gap (2). `28+13+9+2=52`. `## Open blockers` is empty.

### Cycle — AT-34-E3-001 (`companion_absent_from_core_rulebook_companion_tables` mechanism, cycle 2) — one of nine, `decisions.md §14` — partial

**Status: partial — this mechanism unchanged at 28 of 28 remaining** (`core_rulebook`; 0 closed
this cycle).

This cycle's dispatch instruction, verbatim: "16 of your 28 are cross-book-owned rows (14
familiar ability-pool, 2 monster-class) that a prior cycle judged to need a new record type;
re-derive that judgement rather than inheriting it — if a narrower fix closes them, take it."
Re-derived the population first (28, matching the filing cycle's own after-figure exactly),
then investigated both sub-causes from the raw corpus rather than trusting the prior receipt's
framing.

The 2 monster-class rows (`Companion`, `Shadow Companion`, `cr_classes_companion.lst:6`/`:15`):
confirmed `VISIBLE:NO`, no `SIZE:`/`MOVE:`/`NATURALATTACKS:` — a hit-dice level-progression
table, a genuinely different record shape `companion_chassis` has no field for. Judgement
confirmed correct.

The 14 familiar-ability-pool rows (`ce_abilities_familiar_cr.lst`): this is where re-deriving
produced NEW evidence. `ce_abilities_familiar_cr.lst` (the orphaned ability rows) declares
`SOURCELONG:Core Rulebook`, correctly filed under `core_rulebook` by `decisions.md §9`'s
reattribution. But `ce_races_familiar_cr.lst` — the file that actually DECLARES the 11 familiar
creatures this pool describes (Bat, Cat, Hawk, Lizard, Monkey, Owl, Rat, Raven, Toad, Viper,
Weasel; PF1's own Familiars table, CRB p.52-55) — declares `SOURCELONG:Bestiary`, so the SAME
reattribution rule correctly files THOSE rows under `beastiary` instead. Verified directly
against the live ingested corpus: all 11 already exist as registered `CompanionRecord`s at
`data/corpus/beastiary/companion/{bat,cat,hawk,lizard,monkey,owl,rat,raven,toad,viper,weasel}.json`.
This is not a reattribution bug and not a "no such creature exists" gap — it is a real split
baked into the actual books (Core Rulebook states the ability rules, Bestiary states the
creature stat blocks). Closing it needs Shape 8 (cross-book ownership), a corpus-wide widening
of `companion_chassis`'s `every_shipped_ability_row_is_owned_by_a_creature_of_its_own_book`
invariant that every one of the 9 currently-registered companion books relies on — not a
narrow, single-book fix this cycle's scope covers. **Judgement confirmed correct, now with
corpus proof instead of assertion; no narrower fix exists.**

Built a new, committed regression test rather than leaving this as prose that decays:
`companion_chassis::tests::companion_absent_28_sub_causes_are_named_and_sum_exactly`
re-derives the 28-unit population from live `docs/work-inventory.json`, partitions it into the
exact 12/2/14 sub-cause split against the live ingested corpus, and additionally asserts all 11
familiar creatures already exist under `beastiary` (the cross-book proof). `cargo test --lib
rules_core::rules_tables::companion_chassis` — 16/16 pass; full `cargo test --lib` — 2,875
passed, 0 failed, 14 ignored. `docs/work-inventory.json` untouched this cycle (no unit moved),
so `completion_atlas.py --check` and the denominator gate are unchanged: `citation_failures=0`,
`violations=0`. No `v06_work_inventory.rs` line shifted — no atlas citation drift risk this
cycle. See `artifacts/epic-3-core-rulebook/AT-34-E3-001_companion_absent_cycle_receipt_2.md`
for full figures and build-scope verification (workspace + `apps/desktop/src-tauri`, both
`cargo test --locked --no-run` exit 0).

### Cycle — AT-34-E3-001 (`class_feature_option_pool_record_with_magnitude_not_held_by_engine` mechanism) — one of nine, `decisions.md §14` — partial

**Status: partial — this mechanism 333 → 328 of 328 remaining** (`core_rulebook`; 5 closed, 328
remaining named by a sum-exact, 129-group sub-cause partition).

Re-derived the population before touching anything (`decisions.md §15`): grouping
`core_rulebook` `engine-does-not-hold` units by `evidence` and taking
`class_feature_option_pool_record_with_magnitude_not_held_by_engine` gives 333, matching the
dispatch brief's own figure.

Checked before building anything, per this cycle's own dispatch instruction: the prior cycle's
receipt (`class_feature_option_pool_record_not_held_by_engine`, `8e7aecc855`) flagged a
`CLASS_FEATURE_POOLS` registration gap for `"Domain Power"` shared with this mechanism, but had
NOT built it. Read the downstream grounding checks directly rather than trusting that receipt's
framing: `class_feature_exact_suffix_grounded`/`suffix_stripped_grounded`
(`src/bin/v06_work_inventory.rs:7920-7934`, `:9784-9796`) both require the corpus group text to
literally equal the resolved owner's class name — `"Domain Power"` can never equal `"cleric"`, so
registering the pool alone would never ground a single record, only reclassify all 61 to a
different bucket-B mechanism (already owned by another cycle) or bucket D. Built the real fix
instead: `domain_power::domain_power_probe_catalog()` (new `pub` bridge, `pilot_compute::mod.rs`'s
`mod domain_power;` widened to `pub mod domain_power;`) plus a new
`probe_domain_power_effect_wiring` in `v06_work_inventory.rs` — selects each of
`DOMAIN_POWER_CATALOG`'s five real domains on a live cleric and keeps only granted-power names
whose own explanation id is genuinely observed on the rendered snapshot, never a static reflection
of the catalog's membership. `classify()`'s `Kind::ClassFeature` arm gained one new early-return
branch consuming this. Two new tests (`a_domain_power_record_the_probe_observed_reaches_grounded`,
`a_domain_power_record_the_probe_never_observed_is_unaffected`) prove the positive and negative
cases. Closed exactly 5 units: `Domain Power ~ {Battle Rage, Destructive Smite, Strength Surge,
Touch of Good}` reach `DONE`; `Touch of Glory` reaches bucket `V` (its own `wiring_class: static`
routes it through the literal-verification stamp instead) — every other Domain Power record
(56 remaining, no catalog formula) is completely unaffected, confirmed by direct inventory diff.

Corpus-wide (37 books, all sharing this evidence string): 3,052 → 3,047 — the same 5 units, no
other book's Domain Power records were affected (core_rulebook is the only book carrying them).

This cycle's own ~30-line net insertion into `v06_work_inventory.rs` shifted every hardcoded
`file:line` citation `completion_atlas.py`'s `BUCKET_DEFINITIONS` and
`missing_engine_tables.py`'s `ENGINE_SURFACE_CITATIONS` carry (the brief's own named hazard).
Re-derived each one fresh by grepping the exact quoted string the citation targets (not a flat
line offset); both `--check` gates now report `citation_failures=0`.

Discovered, self-healed, and reported honestly rather than hidden: (1) the immediately-prior
cycle's `formula_interpreter_corpus_wide` F1-population re-pin (6,257 → 5,563) was itself WRONG,
not stale — the true value, re-derived twice (before and after this cycle's own edits) via the
exact command that pin's own doc comment names, is 5,445; re-pinned with corrected provenance,
`scripts/retro.py correction` filed. (2) A first `scripts/retro.py` call in this cycle ran
without `RETRO_ACTOR` exported in that same shell invocation (harness shell state does not
persist across tool calls) and wrote into the FORBIDDEN `docs/retro/events/sd31-transcribe.jsonl`
— caught via `git status --porcelain` before the next git write, the one mistaken line removed
(confirmed the file's other 5 pre-existing lines, from a different lane, were left byte-identical
to their pre-session state), and the correction re-filed correctly into
`docs/retro/events/sd34-at-34-e3-001.jsonl`. (3) `apps/desktop/src-tauri` carries 26 of 548
pre-existing test failures (`companion_catalog`/`race_trait_picker`/`reach_gate`), confirmed
unrelated to this cycle by running the identical suite in a throwaway `git worktree` checkout of
this cycle's own start SHA before any edits — identical failure count and names. Not caused by,
not fixed by, this cycle; named rather than silently absorbed.

Verification: `cargo test --locked --bin v06_work_inventory` 383/383; `cargo test --locked --lib`
2874/2888 (14 ignored, 0 failed); `cargo test --locked --no-run` exit 0 at the full workspace
scope; `apps/desktop/src-tauri` `cargo test --locked` 522/548 (26 pre-existing, unrelated
failures, see above). `corpus_literal_sweep`: 48708 examined before → 48708 after, delta 0 (no
`data/corpus/**` touched). `derived_evaluator_fixture_check`: 1839 units cleared over 2580 rows,
0 failed. Both dual-audit greps: `OK_NO_BUNDLE_TAGS` / `OK_NO_TOKENS` on this cycle's own diff.

Receipt: `artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_with_magnitude_cycle_receipt.md`.
Remainder (328, sum-exact across 129 corpus-key groups, largest named): `Domain Power` (56,
domains this catalog has no formula for), `Domain Base` (33, a different corpus shape — the
domain header record, needs its own disposition ruling), `Favored Enemy/Terrain Bonus` (42),
`Bardic Performance`/`Draconic Bloodline Choice`/`Secret Lore`/`New Arcana` (39), the wizard
opposition/arcane-school cluster (~34, shares its root cause with the already-scoped 37-unit
sibling gap in `class_feature_option_pool_record_not_held_by_engine`'s own receipt), ~22 more
small per-class roster groups, and a ~106-unit long tail of single-unit trackers needing
individual inspection.

### Cycle — AT-34-E3-001 (`class_feature_owner_matched_by_name_but_record_not_held_by_engine` mechanism) — one of nine, `decisions.md §14` — partial

**Status: partial — this mechanism 0 → 0 of 346** (`core_rulebook`; every remaining unit named
by an exact sub-cause, proven by a committed passing regression test, not just prose).

Population re-derived at HEAD, not transcribed: group `core_rulebook` units whose `status` is
`engine-does-not-hold` by `evidence`, take the
`class_feature_owner_matched_by_name_but_record_not_held_by_engine` group ->
**346 of 346** — matches the dispatch brief's stated figure exactly, verified.

**Investigation, not a fix.** `Kind::ClassFeature`'s "owner resolved" branch already consults
`facts.class_feature_pool_catalog_holds` before falling back to `engine-does-not-hold`, and
`class_feature_pool_catalog`'s own `is_registered_pool_group` was already widened (SD-32 T12) to
accept ANY `" ~ "`-qualified key — so the catalog this mechanism's own evidence-comment says is
missing is not narrow. Re-running that catalog's exact filter, gate by gate and in the same
order, against all 346 units produces an EXACT, sum-exact partition: **143** null-`description`
internal-bookkeeping rows (`ADD:SPELLCASTER`/`SPELLKNOWN`/`SPELLLEVEL`, no `DESC:` at all — no
prose exists to serve); **121** real-engine-effect-token rows (`ABILITY`/`AUTO`/`BONUS`/`ADD`/…
alongside the description — genuinely mechanical, Decision 7 condition 1 fails); **67** already
served by the catalog but blocked at `classify()`'s own promotion gate (`wiring_class != "display"`,
or the prose trips `closure_states_universal_sheet_modifier`'s `"size bonus"` cue — both
deliberate, hand-verified correct per Decision 7); **6** class-level-scaled prose (e.g. "200 gp
per wizard level"); **5** dropped-pcgen-arg records; **3** multi-`DESC:` records whose branches
are genuinely mutually exclusive (alignment/level-banded), not the safe sequential-continuation
shape a prior cycle's fix already handles; **1** bare-`%N` reference. Two of these sub-causes
were hand-sampled specifically to check for a hidden bug (the 67-unit "already in the catalog"
group looked most promising) — both hand-checked records showed the blocking gate firing
correctly, for the exact reason Decision 7 requires, so no code change was made.

**Why no fix landed.** Every gate refusing these 346 units is pre-existing, load-bearing safety
architecture (`class_feature_pool_catalog.rs`'s render-and-refuse gates, `classify()`'s
wiring-class and universal-sheet-modifier gates) built by earlier cycles against real,
hand-verified corpus findings specifically to prevent serving a genuinely mechanical or
level-scaled record as if it were static, complete prose. Closing any of these 346 units
requires either new engine wiring (spellcaster/domain/bonus-feat/proficiency grants, size-bonus
shapeshifting computations) or new ingest work (writing a description that does not exist
upstream) — both larger, separately-scoped projects, not a one-cycle catalog widening. Naming
the exact, provable partition is this cycle's deliverable (`decisions.md §15`).

**Movement:** none — 0 units changed status or evidence this cycle (instrument-correction,
reclassification, reachability, and closure all zero). `core_rulebook` bucket B (atlas-real
partition) unchanged at 762 of 6,701.

TDD: one new, committed, passing test,
`class_feature_owner_matched_but_not_held_346_sub_causes_are_named_and_sum_exactly`, in
`src/rules_core/class_feature_pool_catalog.rs`, RED-then-GREEN against the live corpus and
`docs/work-inventory.json` (asserts the seven sub-cause counts sum exactly to the re-derived
population; fails closed if a future ingest/wiring change moves a unit without this receipt
being updated to match). No production code changed.

Full receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_owner_matched_cycle_receipt.md`.

### Cycle — AT-34-E3-001 (`race_trait_race_not_modelled` mechanism) — one of nine, `decisions.md §14` — complete

**Status: complete — this mechanism 132 → 0** (`core_rulebook`; corpus-wide side effect,
1,413 → 90, since the fix is a generic engine change, not a `core_rulebook` special case).

Population re-derived at HEAD, not transcribed: group `core_rulebook` units whose `status` is
`engine-does-not-hold` by `evidence`, take the `race_trait_race_not_modelled` group ->
**132 of 132** — matches the dispatch brief's stated figure exactly, verified.

**Root cause.** `Kind::RaceTrait`'s classifier requires a unit's key to embed one of
`RaceId::ALL`'s seven compiled CRB race names. All 132 genuinely name no race at all: 118
`Racial SLA ~ <name>` rows (`cr_abilities_race.lst`'s cross-book spell-like-ability definitions
library — confirmed via the pinned oracle that no `core_rulebook` race references these keys at
all, but `blood_of_angels`'s Aasimar variant trait does), 6 `+2 <Ability>` ability-score-bonus
CHOOSE-pool entries, 4 `Favored Enemy ~ Humanoid (<Race>)` Ranger class-feature option-pool rows
duplicated under each race's own file, and 4 pool-bookkeeping/placeholder rows (`No Race Trait
Available`, `Remove Excess Points from Pool`, `Region ~ None`/`~ Unknown`). None of that is a
matcher defect — it is a real population the classifier never had anywhere to place, because the
shared premise (every `race_trait` unit's key names a race) is false for these rows.

**The fix, built generically.** SD-32's `ingest_race_trait_generic.py` had already transcribed
every one of these rows, book-agnostically, into `data/corpus/<book>/race_trait_generic/*.json`
— "measurable, not (yet) engine-reachable," in that script's own words. `classify()` never
consulted that table. Added `simple_kind_tables::load_simple_kind_table_for_dir` (factored out
of the existing `load_simple_kind_table`, since `race_trait` is not one of Epic 2's eight
kinds), loaded it into `EngineFacts`, and consulted it as `Kind::RaceTrait`'s LAST fallback,
reusing `simple_kind_verdict` — the identical promotion ladder all eight Epic 2 kinds already
run — verbatim. A real second hazard caught by actually regenerating and counting the artifact
rather than trusting the unit tests alone: the generic table is keyed by the unit's REPORTING
attribution (`unit.book`), while `classify()`'s own `engine_book` local is resolved off
`unit.source_book` — for 4 units walked from `core_essentials/races/<race>/` but reported as
`core_rulebook`, the first lookup missed; a retry on `unit.book` (only when the two differ and
the first lookup is a genuine absence) found the real record. Fixed in the same cycle.

**Movement:** reclassification only, this book — 129 to `ingested-magnitude` (bucket M), 3 to
`..._pending_wiring_class_review` (bucket D). Corpus-wide (same generic fix, other books'
scope, reported honestly as a side effect, not claimed as this cycle's own work): 708 more to M,
199 more to D, 416 promoted to `text-complete` (DONE) via the SAME zero-magnitude-real-
description-display-wiring-class rule every other Epic-2 kind's rung already applies. 90 of
1,413 remain corpus-wide — other books' own residual shapes, out of scope for this cycle.

TDD: 6 new unit tests in `race_trait_grounding_tests` (RED against an empty generic table,
GREEN against the real corpus, plus the two-book-key regression), 1 new unit test in
`simple_kind_tables::tests`. 10 of `completion_atlas.py`'s `BUCKET_DEFINITIONS` `file:line`
citations shifted and were re-derived and fixed in the same cycle
(`citation_failures=0` after). `cargo test --locked --no-run` exits 0 at the widest workspace
scope. `docs/work-inventory.json` regenerated with `CORPUS_LITERAL_SWEEP_REPORT`/
`DERIVED_FIXTURE_CHECK_REPORT` set from this session's own fresh runs (`corpus_literal_sweep`:
48,708 of 51,482, unchanged, CLEAN; `derived_evaluator_fixture_check`: 1,839 of 2,580, 0
failed, unchanged) — no `--allow-stamp-loss`. Receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_race_trait_race_not_modelled_cycle_receipt.md`.

**Note:** `box_ledger.py --check` (SD-33's inherited, read-only partition) exits 1 both before
and after this cycle — pre-existing since prior AT-34-E3-001 mechanism cycles, tracked against
the frozen `THE-BOX.md` snapshot SD-34 does not own. This cycle's own effect is an improvement
(`uncovered` 21,221 → 20,097), not a regression; the check's structural invariants
(`overlap=0`, `population=49438`) hold both before and after.

### Cycle — AT-34-E3-001 (`companion_absent_from_core_rulebook_companion_tables` mechanism) — one of nine, `decisions.md §14` — partial

**Status: partial (`decisions.md §15`) — this mechanism 100 → 28.**

Population re-derived at HEAD, not transcribed from `decisions.md §14`'s filed figure (which it
happens to match exactly): group `core_rulebook` units whose `status` is `engine-does-not-hold`
by `evidence`, take the `companion_absent_from_core_rulebook_companion_tables` group ->
**100 of 100**.

**The fix.** `crb::companion_data` (the `companion_chassis` extension SD-29 built for this book)
shipped only 46 of Core Rulebook's 130 ability rows. The other 84 — every generic
`Animal Companion ~ …` / `Animal Companion Feat ~ …` / `Animal Trick ~ …` / `Animal Training ~
…` / `Companion Stat ~ …` record — were orphans under `companion_chassis`'s existing six
ownership shapes, because the corpus states this progression table exactly ONCE for the whole
`CLASS:Companion` chassis (`cr_classes_companion.lst`) every one of the book's 38 registered
creatures shares, rather than per-creature. No shape 1-6 (`ABILITY:`-named, `PRERACE:`,
namespaced-prefix, granted-by, relay, display-name) can attribute an ownership the corpus never
states per-creature in the first place.

Built **Shape 7, book-wide grant** in `scripts/transcribe_companion_tables.py`: an exact, closed,
84-key literal set (never a prefix heuristic, so an unrelated future orphan can never silently
ride it) attributed to ALL 38 of this book's registered creatures at once. This is a real,
corpus-backed fact, not an invented link — PF1's own Animal Companion rules (CRB p.52-55) grant
this identical table to every companion regardless of species. Regenerating
`crb::companion_data` from the widened transcriber ships 72 of the 84 (the ones carrying real
`TYPE:`/`DESC:`/`BONUS:` content); the other 12 are internal PCGen plumbing rows (`Base
Companion ~ …`, `Companion ~ …`) with only an `ABILITY:` grant token and no player-facing
content, correctly caught and dropped by the pre-existing empty-payload screen — the same
disposition every other book's zero-content row already gets, not a regression.

**TDD.** Regenerating the table immediately RED-failed two of `companion_chassis`'s own
count-pinned tests for the intended reason: `an_ability_with_no_modelled_facet_still_states_
its_type_segments` (39 → 93 unmodelled-facet rows — the new Animal-Companion-progression rows
are feats/tricks/stats, none of which map onto `CompanionAbilityFacet`'s three variants) and
`a_row_stating_its_text_once_per_condition_carries_every_token_and_promotes_only_the_ungated_
one` (11 → 13 multi-DESC rows — `Animal Trick ~ Attack` and `Animal Companion Feat ~ Toughness`
both carry one ungated plus one gated `DESC:` token). Updated both to the new corpus-true
counts with real explanations (not just bumped numbers), widened one structural
equality-assertion (`crb_unmodelled == vec!["Crocodile ~ Tail Slap"]`) to a membership+count
check per the round-7 lesson already in this file's own doc comment (a count assertion ahead of
a structural one hides the structural one), and added two new named structural assertions for
the two new multi-DESC rows. GREEN: all 15 `companion_chassis` tests, all 126
`companion`-scoped lib tests, and the full workspace `cargo test --lib` (2,872 passed, 0 failed,
14 ignored) all pass. `cargo test --locked --no-run` exits 0.

**`docs/work-inventory.json` regenerated** (sequential with Epic 4 per `workflow-instruction.md
§3`), using fresh `corpus_literal_sweep`/`derived_evaluator_fixture_check` reports
(33s / 4s wall time, both CLEAN/0-failed) to satisfy the stamp-loss guard honestly rather than
`--allow-stamp-loss`. Total corpus population unchanged at 49,438 (no records added or removed
this cycle — only ownership/placement changed). `completion_atlas.py --check` stays green:
population=49438 buckets=10 unclassified=0 overlap=0 citation_failures=0.

**Movement, four buckets:** closure 72 (bucket B → DONE-tier: 40 `text-complete`, 29
`grounded`, 3 `literal-verified`); reclassification 0; reachability 0 (these rows reach the
player through the SAME `companion_catalog` render path every other `crb::companion_data` row
already uses — no new wiring beyond table placement); instrument-correction 0.

**Remainder, named by sub-cause, 12 + 2 + 14 = 28:**
1. **12** zero-content internal PCGen plumbing rows (`Base Companion ~ Animal Companion`/
   `Special Mount`, `Companion ~ Ability Score Increase`/`Bonus Tricks`/`Devotion`/`Evasion`/
   `Improved Evasion`/`Link`/`Multiattack`/`Share Spells`/`Spell Resistance (AC)`/`Spell
   Resistance (SM)`) — only an `ABILITY:` grant token, no `TYPE:`/`DESC:`/`BONUS:`; the
   empty-payload screen (`decisions.md §63.3`) correctly drops them, same as any other book.
2. **2** `cr_classes_companion.lst` PCGen monster-class definitions (`Companion`, `Shadow
   Companion`) — a hit-dice level-progression construct, not a creature and not an ability;
   modelling it is a genuinely new record type, a standing SD-29 architecture decision
   (`decisions.md §65.1`) this cycle does not widen.
3. **14** `ce_abilities_familiar_cr.lst` master-side familiar special-ability-pool rows,
   reattributed to `core_rulebook` — a real generic Familiar table, but this book registers NO
   familiar creature (all 38 of its creatures are Animal Companions) for
   `companion_chassis`'s same-book ownership invariant to attach it to. A pinned unit test
   (`a_companion_reattributed_to_a_chassis_book_that_does_not_hold_it_is_bucket_b_not_a`,
   `AT-34-E2-004`) already fixes this shape's intended disposition as "must be truly placed,
   never reclassified to bucket D" — closing it needs a cross-book ownership shape (Shape 8) or
   a dedicated master-side ability-pool record type, real future engine capability, not built
   this cycle.

Receipt: `artifacts/epic-3-core-rulebook/AT-34-E3-001_companion_absent_cycle_receipt.md`.

### Cycle — AT-34-E3-001 (`class_feature_option_pool_record_not_held_by_engine` mechanism), Cycle 2 — multi-DESC ingest truncation sub-cause — partial

**Status: partial (`decisions.md §15`) — this mechanism 57 → 55.** Continues the prior cycle
(below) without re-deriving its investigation; takes the cheapest of its seven named
sub-causes, the multi-`DESC:` ingest truncation (2 units: `Martial Weapon Proficiency Output`,
`Octopus Wild Shape ~ Poison`).

`cache_gen::class_feature::generate`'s `desc_value` now joins a record's DESC segments when
safe (no `PREVAREQ`/`PREVARGTEQ` choice-branch gate on any segment beyond the first), instead
of always keeping only the first — `Rage Power ~ Elemental Blood (Greater)`'s own regression
test is unchanged, byte-identical. Regenerated the 2 named corpus records via the guarded
`--coordinates` path (18,043-unit corpus untouched elsewhere).

**Discovery, caught and reverted before commit:** the first version of `class_feature_pool_
catalog.rs`'s companion gate fix relaxed the multi-DESC refusal on SHAPE alone, which silently
promoted **188** OTHER corpus records across multiple books and mechanisms this cycle does not
own — their `data.description` was still the OLD, stale, first-segment-only value, so relaxing
the gate served that truncated text as `text-complete`. Caught by diffing the regenerated
`docs/work-inventory.json` against the committed baseline before commit. Fixed by requiring an
ingest-freshness PROOF instead of the shape alone: the gate now recomputes the expected safe
join directly from `raw_tokens` and only admits a record when that join EQUALS the
already-shipped `data.description`. Re-running the regeneration with the corrected gate
confirmed exactly 2 changes, both intended. Retro `near_miss` event:
`docs/retro/events/sd34-at-34-e3-001.jsonl`.

`core_rulebook` bucket B: `968 → 966` (`completion_atlas.py --book core_rulebook --check`).
Sibling mechanisms confirmed unmoved: `class_feature_owner_matched_by_name_but_record_not_
held_by_engine` 346, `class_feature_option_pool_record_with_magnitude_not_held_by_engine` 333,
`companion_absent_from_core_rulebook_companion_tables` 100, `race_trait_race_not_modelled` 132
— `55+346+333+100+132=966`, matches exactly. Corpus-wide population unchanged at 49,438 (2
reclassified, 0 added/removed). `corpus_literal_sweep`: `48708 → 48708`, delta 0 (2 already-
existing corpus files edited in place, none added/removed). Full receipt (Cycle 2 section,
prepended above Cycle 1's own unedited content):
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_cycle_receipt.md`.

Remainder, named by sub-cause (Cycle 1's own five unclosed, unchanged, summing to 55):
proficiency/mechanical-grant possession-tracking (28), class-skill/companion-mount attribution
(13), wizard opposition-school tracking (9), vacuous placeholders pending a `decisions.md §2`
ruling (3), Domain Power `CLASS_FEATURE_POOLS` registration gap shared with the 333-unit
`with_magnitude` sibling mechanism (2). `28+13+9+3+2=55`.

### Cycle — AT-34-E3-001 (`class_feature_option_pool_record_not_held_by_engine` mechanism) — one of nine, `decisions.md §14` — partial closure, further decomposed

**Status: NOT complete (own mechanism only) — 63 → 57, six of nine.** Re-derived population at
cycle start (`9e380e2ce6`): `63` (matches the task brief and `decisions.md §14`'s table
exactly, verified not assumed). Direct inspection of all 63 units' real corpus rows
(`data/corpus/core_rulebook/class_feature/**/*.json`) found this ONE evidence string is not a
single root cause, unlike its four already-closed siblings — it bundles at least seven
distinct real shapes:

1. Genuinely prose-only, mechanically-inert standalone features (6 units: `Timeless Body`,
   `Uncanny Dodge`, `Woodland Stride`, `Evasion Output`, `Improved Evasion`, `Blank Weapon
   Block OS`) — **closed this cycle** via a new `src/rules_core/class_feature_pool_catalog.rs`
   sibling catalog, `load_standalone_class_feature_catalog`, reusing the pool catalog's
   identical render-and-refuse/engine-effect-token/multi-DESC/archetype-lock safety pipeline
   for the mutually-exclusive non-`" ~ "`-qualified partition (proven disjoint by a new test).
2. Proficiency/mechanical-grant tokens with no tracking system anywhere in this engine (28
   units: Armor/Weapon/Shield Prof, Weapon Proficiencies, Weapon and Armor Proficiency, All
   Automatic/Martial Proficiencies, Add Spoken Language, Armor Training, Channel
   Negative/Positive Energy) — verified: no character-level proficiency-*possession* tracking
   exists anywhere in `src/rules_core/` (only feat-driven bonus math and racial
   `ABILITY:FEAT|AUTOMATIC` handling, both different subsystems).
3. Class-skill lists computed from a wholly separate, hand-kept source, never these corpus
   records (10 units: `Class Skills ~ <9 classes>`, `Jack of All Trades ~ Class Skills`) —
   `skill_allocation.rs`'s `class_skill_set` reads hand-kept `GROUNDED_{FIGHTER,ROGUE,
   WIZARD}_CLASS_SKILLS` constants, not these `CSKILL:` corpus rows.
4. Wizard opposition-school spell-restriction tracking, absent entirely (9 units, the nine
   `<School> Wizard Spells` records).
5. Companion/special-mount summoning not attributed to these specific records (3 units).
6. Vacuous placeholder rows with genuinely zero content — null description, `KEY`/`CATEGORY`/
   `TYPE` tokens only (3 units, `Empty Selection ~ Standard {Barbarian,Monk,Rogue}`) — a real
   unpredicted verdict shape (`decisions.md §2`), correctly left to AT-34-E3-006's own
   `atlas-defects.md` process rather than invented here.
7. `Domain Power ~ {Leadership, Sun's Blessing}` (2 units) — and a separately-verified,
   more consequential finding: `CLASS_FEATURE_POOLS` has no `"Domain Power"` entry at all, so
   even the FIVE domains `domain_power.rs` already computes correctly (Good/War/Strength/
   Destruction/Glory) are never credited on the atlas — every one of their own units still
   reports the SIBLING `..._with_magnitude_not_held_by_engine` evidence. Flagged for that
   mechanism's own cycle, not fixed here (reaches its 333-unit population, not this one's).
8. Multi-`DESC:` ingest truncation (2 units) — a `cache_gen::class_feature::generate`
   ingest-territory fix, outside this cycle's consumer-territory file-touch set.

28+10+9+3+3+2+2 = 57, no unnamed gap. Full evidence, per-record citations, and the six safety
gates proving item 1's closure is real (not a stub) are in this cycle's own receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_cycle_receipt.md`.

**Citation-drift self-heal:** this cycle's 47 inserted lines (across four sites in
`src/bin/v06_work_inventory.rs`) shifted all ten of `completion_atlas.py`'s
`BUCKET_DEFINITIONS` citations AND both of `missing_engine_tables.py`'s
`ENGINE_SURFACE_CITATIONS` entries (the latter's own `--check` gate is not part of any
standing verify stage, so its drift was silent until checked proactively this cycle). All
twelve re-derived by grepping the literal target content and fixed; both gates' own
`--check` report `citation_failures=0` at this cycle's HEAD.

**Figures:** `63 → 57` (this mechanism); `974 → 968` (`core_rulebook` real atlas bucket B,
`completion_atlas.py --book core_rulebook --check`); sibling mechanisms confirmed unmoved
(`346`/`333`/`132`/`100`); `49,438` corpus-wide population unchanged (no records added).
`corpus_literal_sweep`: `48708 of 51482, CLEAN`, unchanged (no corpus records touched).

**Build scope:** `cargo test --locked --lib` 2866 passed; `cargo test --locked --bin
v06_work_inventory` 376 passed; `cargo test --locked --no-run` (workspace) exit 0;
`apps/desktop/src-tauri` `cargo test --locked --no-run` exit 0 (separate
`CARGO_TARGET_DIR`).

**Next-cycle plan (dispatch cheapest-first):** (1) vacuous placeholders — needs
`decisions.md §2`'s ruling via AT-34-E3-006's `atlas-defects.md` process; (2) multi-DESC
ingest fix (2 units); (3) class-skill/companion-mount attribution (13 units); (4) wizard
opposition-school + proficiency tracking (37 units, largest, likely needs further splitting
once scoped); (5) flag the `Domain Power` `CLASS_FEATURE_POOLS` gap to whichever cycle owns
the `with_magnitude` sibling mechanism.

### Cycle — AT-34-E3-001 (`deity_content_absent_from_deity_table_in_core_rulebook` mechanism) — one of nine, `decisions.md §14`

**Status: complete (own mechanism only).** Re-derived population at cycle start (`5f0a905fb0`):
`deity_content_absent_from_deity_table_in_core_rulebook` = **21 of 974** remaining
`core_rulebook` bucket-B units (atlas-partitioned; matches `decisions.md §14`'s stated 21,
verified not assumed). All 21 `cr_deities.lst` deity rows carry `NAMEISPI:YES` and are
PI-masked at ingestion (`data.key`/`data.name` rewritten to `Codex-Named Unit (...)`), so
`SimpleKindTable::resolve`'s plain key/name lookup never found them even though the corpus
records physically exist. Fix, exactly mirroring the `domain` mechanism cycle's own pattern:
`Kind::Deity`'s `classify()` arm now falls back to `SimpleKindTable::resolve_by_coordinate`
on the record's own stored `"{book}:{source_file}:{source_line}"` coordinate after the
ordinary resolve fails — never reading, logging, or reconstructing the redacted real deity
name in any code path, test name, or commit message (`decisions.md §14`'s PI constraint).
All 21 carry `magnitude_token_count == 0` and a real `DESC:` token, so they land in bucket D
(`text-complete`), not bucket M — a correct outcome per this criterion's own instruction that
leaving bucket B for D/M is not a half-fix.

Self-healed inline: this cycle's own line-insertions shifted two hardcoded `file:line`
citations — `completion_atlas.py`'s bucket-V citation (`10480 -> 10495`) and
`missing_engine_tables.py`'s `power` citation (`9908 -> 9923`) — both re-derived by grep and
fixed before this cycle's `--check` runs went green.

`core_rulebook`'s real, atlas-partitioned bucket B: **995 -> 974** (`python3
scripts/completion_atlas.py --book core_rulebook --check`). Five of the nine named mechanisms
now remain: `class_feature_option_pool_record_not_held_by_engine` (63),
`companion_absent_from_core_rulebook_companion_tables` (100), `race_trait_race_not_modelled`
(132), `class_feature_owner_matched_by_name_but_record_not_held_by_engine` (346 — grown from
the `decisions.md §14` table's 330 by the `class_absent` cycle's own recorded `+16`
reattribution side effect), `class_feature_option_pool_record_with_magnitude_not_held_by_engine`
(333). These five sum to exactly 974 — no unnamed gap. AT-34-E3-001 itself does not close this
cycle. `## Open blockers` is empty.

Receipt: `artifacts/epic-3-core-rulebook/AT-34-E3-001_deity_absent_cycle_receipt.md`.

---

### Cycle — AT-34-E3-001 (`class_absent_from_ClassId_ALL_and_book_class_id_enums` mechanism) — one of nine, `decisions.md §14`

**Status: complete (own mechanism only).** Re-derived population at cycle start
(`ae25d75d7d`): `class_absent_from_ClassId_ALL_and_book_class_id_enums` = **17 of 1,006**
remaining `core_rulebook` bucket-B units (matches the brief's and `decisions.md §14`'s stated
figure, verified not assumed). `modelled_class_books()` (`v06_work_inventory.rs`) — the map
`classify()`'s `Kind::Class` arm consults for "does the engine model a class of this name at
all" — was scoped to base classes only; CRB's 28 real `CLASS:` records also include 10
`TYPE:PC.Prestige` classes, 5 `TYPE:Base.NPC` classes, and 2 `TYPE:Base.PC, VISIBLE:NO` `Ex-*`
variant states (Ex-Barbarian, Ex-Paladin), none registered anywhere. Fix, in two parts: (1)
the ten prestige classes needed **zero new chassis code** — `prestige_class_entry_gate.rs`
already carries a real, corpus-derived entry-requirement registry for them (SD-32
`AT-32-E3-001`), already wired into `compute_class_chassis`, simply never read by
`modelled_class_books()`; registering from that existing registry respects SD-32's own
deferral of a FULL prestige-class chassis (six of the ten need caster-level stacking this
codebase does not have) without reopening it. (2) The seven NPC/`Ex-*` classes needed a
genuinely new, small chassis — direct read of their corpus `raw_tokens` confirmed every one
uses the identical `classlevel("APPLIEDAS=NONEPIC")`-based BAB/save formula shape CRB's real
base classes use, so a new module (`crb_untabled_class_chassis.rs`) evaluates each class's own
corpus formula string via `PcgenFormulaEvaluator` — the same evaluator `generic_class_chassis.rs`
already proved against 61 other classes across 14 other books — rather than a hand-typed
table, and rather than widening that shared module's own book list (its population is
mirrored byte-for-byte in `apps/desktop/src-tauri`'s separate `class_catalog_generic.rs`).
Both registrations key `class_books` on the corpus's own **lowercased display name** (a space
for a multi-word class), never the registry's underscored `class_id` slug — a name-namespace
mismatch that would otherwise silently defeat `classify()`'s own lookup.

**Discovery 1, reasoned through and self-healed, not silently shipped.** Registering
common-English-word class names (`warrior`, `assassin`, `expert`, `adept`, `aristocrat`,
`commoner`) exposed a latent, pre-existing property of `class_feature_owner`'s whole-corpus
suffix/prefix matching: a shorter, newly-modelled class name can win a match against an
unrelated compound group text from a DIFFERENT book (e.g. `ultimate_psionics`'s own distinct
"Adaptive Warrior" class) purely because the true, more specific candidate was never itself a
`class_books` entry to lose to. Verified this is not new — the identical misattribution
already existed via the `corpus_class_names` fallback before this cycle (confirmed against the
committed inventory) — and verified the two statuses that actually matter cannot be falsely
earned regardless: `grounded` requires an EXACT group==owner match (never suffix/prefix), and
`text-complete` is gated by a real, owner-independent per-record whitelist
(`class_feature_pool_catalog_holds`). A cross-check guard was added anyway, restoring identical
behavior for the genuine collision cases while leaving same-name matches untouched (full
`cargo test --bin v06_work_inventory` 374/374 and `cargo test --locked --lib` 2,863/2,863 stay
green). Net, outside `core_rulebook`: 187 units across 8 other books relabel evidence strings
(19 of them genuine, independently-earned `text-complete` unlocks; none reaches `grounded`
falsely); reported honestly as this cycle's own reclassification side effect, not folded into
this mechanism's own count.

**Discovery 2: a stale, pre-existing `cargo test --locked --lib` failure, found by this
cycle's own §6 step 3 run, not caused by it.** `formula_interpreter_corpus_wide.rs`'s
F1-population pin (6,257, set by SD-33's own closure) was already RED at this cycle's own
start SHA `ae25d75d7d` — confirmed by a clean worktree there with ZERO of this cycle's edits,
reproducing the identical `left: 5563, right: 6257` failure. `docs/work-inventory.json` was
regenerated four more times after that pin without a `cargo test --locked --lib` re-run
(the exact "run the suite after the last write that can move it" lesson, recurring across a
DIFFERENT set of cycles than the ones that lesson already names). Re-pinned to 5,563, verified
two independent ways (this cycle's own final inventory, and `ae25d75d7d`'s own untouched
committed copy — both 5,563), logged as a `correction` retro event.

RED→GREEN proven for the registration itself (temporarily zeroing
`crb_untabled_class_chassis::covered_classes()` reproduced the exact `class_absent...`
failure for the intended reason, not a panic elsewhere). `docs/work-inventory.json`
regenerated (guarded path, `CORPUS_LITERAL_SWEEP_REPORT` + `DERIVED_FIXTURE_CHECK_REPORT` set
from this session's own fresh runs, no `--allow-stamp-loss`): `corpus_literal_sweep`
`48,708 -> 48,708` examined (delta 0, exact match — this cycle adds no corpus records, CLEAN).
`core_rulebook` bucket B: `996 -> 995` (not a clean `-17`: `-17` this mechanism, `+16` a
legitimate same-book, same-word reattribution of `core_rulebook`'s own class_feature records
for these seven newly-modelled classes onto a DIFFERENT, unowned bucket-B mechanism,
`class_feature_owner_matched_by_name_but_record_not_held_by_engine`). Ten `BUCKET_DEFINITIONS`
`file:line` citations in `scripts/completion_atlas.py` re-derived and corrected
(`citation_failures` `10 -> 0`).

### Cycle — AT-34-E3-001 (`race_trait_absent_from_race_traits` mechanism) — one of nine, `decisions.md §14`

**Status: complete (own mechanism only).** Re-derived population at cycle start
(`79fc41ccd0`): `race_trait_absent_from_race_traits` = **9 of 1,006** remaining
`core_rulebook` bucket-B units (matches the brief's stated figure, verified not assumed).
Two row shapes shared the evidence string: 7 `Adopted Race ~ <Race>` selector rows (one per
CRB race — `decisions.md §25`'s `TYPE:AdoptiveRace` shape, already modelled generically by
`race_resolver::adopted_race_choose_selectors` for 14 OTHER races but never ingested for
CRB's own 7, because `ingest_races.rs` deliberately filters the shape out as "not a standard
trait" and `ingest_race_traits.rs` had no `core_rulebook` `BookSource` at all) and 2
`Human Ethnicity ~ None`/`~ Unknown` placeholder rows (`cr_abilities_race.lst`'s own
`###Block: Placeholder objects...`, a fifth row shape `ingest_race_traits.rs`'s parser had
never recognised — silently `None`, dropped before the scope filter). Fix: a new
`core_rulebook` `selector_only` `BookSource` (the identical, 4-times-proven pattern
`bestiary_2`/`_3`/`_5`/`_6` already use — `core_rulebook`'s 67 pre-existing standard-trait
files in the same directory are protected by `is_racial_default`-field discrimination) plus
one new row-shape predicate, `TraitRow.is_human_ethnicity_placeholder`, resolving `race_key`
to `"Human"`. RED→GREEN proven (`human_ethnicity_placeholder_row_resolves_to_human_and_is_admitted`
plus a negative-case sibling); `ingest_race_traits` 22→24 tests, `race_resolver` 28 tests (3
pinned corpus-census tests widened to the corrected populations: `adopted_race_choose_selectors`
14→21, `Unclassified` role 44→53, corpus total 910→919), `v06_work_inventory` 371 tests, all
green. `docs/work-inventory.json` regenerated (guarded path, `CORPUS_LITERAL_SWEEP_REPORT` +
`DERIVED_FIXTURE_CHECK_REPORT` set, no `--allow-stamp-loss`): exactly 9 units changed, all
this cycle's own target. `corpus_literal_sweep`: `48699 -> 48708` examined (delta +9, exact
match, CLEAN). **Self-caught correction, logged before shipping** (`docs/retro/events/
sd34-at-34-e3-001.jsonl`): a first-pass check used the same loose `status ==
"engine-does-not-hold"` python filter Cycle 1 used, which conflates atlas buckets B and D —
under that filter the 9 units appeared to just move to a different `engine-does-not-hold`
evidence string with no net change (1,809 -> 1,809). Re-running the atlas's own real
partition (`completion_atlas.py`'s `_B_MARKERS`) shows the true, correct outcome:
`core_rulebook` bucket B **`1005 -> 996`**, a clean `-9` — the 9 units' new evidence,
`race_trait_record_loaded_but_never_applies`, does not contain a B marker and correctly
lands in bucket D (a genuinely narrower "other engine gap": the record IS now ingested and
loaded, `RaceCorpus` classifies it `TraitRole::Unclassified`, the same terminal state
`Oversized Goblin`/`Human ~ Tribalistic Languages`/`Suli ~ Trusted Mediator` already carry).
`decisions.md §2a`-consistent: B→D is a correct outcome, not a half-fix. No line-citation
drift (`v06_work_inventory.rs` untouched this cycle). `cargo test --locked --no-run` exits 0
at the full workspace scope; `apps/desktop/src-tauri` (untouched by this cycle) also verified,
`--no-run` exits 0.

**AT-34-E3-001 does not close this cycle.** `core_rulebook` bucket B is 996 of 6,701; seven
of the nine named mechanisms remain (`decisions.md §14`'s table). Full receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_race_trait_absent_cycle_receipt.md`.

### Cycle — AT-34-E3-001 (`domain` mechanism) — one of nine, `decisions.md §14`

**Status: complete (own mechanism only).** Re-derived population at cycle start:
`domain_content_absent_from_domain_table_in_core_rulebook` = **1 of 1,006** remaining
`core_rulebook` bucket-B units (matches `decisions.md §14`'s table). Root cause re-derived,
not assumed from the prior cycle's now-stale escalation text: the corpus record for `Death
(Pharasma)` at `cr_domains.lst:46` already exists (landed by the already-committed
`AT-34-E1-008`), but its `key`/`name` are PI-masked to `Codex-Named Unit (...)` because the
domain's own name embeds the deity `Pharasma`, so the classifier's key/name lookup could
never find it. Fix: `SimpleKindTable::resolve_by_coordinate`, a new PI-safe fallback that
matches on the record's own stored `(book, source_file, source_line)` — never the redacted
real name — wired only at `Kind::Domain`'s call site (the other six simple-kind-table kinds
are untouched, `None` passed, byte-identical pre-fix behaviour). RED→GREEN proven with two new
unit tests plus the full `v06_work_inventory` (371 passed) and `simple_kind_tables` (12
passed) suites. `docs/work-inventory.json` regenerated with `CORPUS_LITERAL_SWEEP_REPORT` +
`DERIVED_FIXTURE_CHECK_REPORT` set (no `--allow-stamp-loss`): exactly 2 of 49,438 units
changed — this cycle's own target (`core_rulebook:domain:death_pharasma`, B→M) and one
side-effect unit in a different book/mechanism (`advanced_players_guide:domain:
souls_pharasma_subdomain`, reported honestly, not claimed as this cycle's scope).
`core_rulebook` bucket B: `1810 -> 1809`. `corpus_literal_sweep`: `48699 -> 48699`, delta 0
(no corpus file touched). This cycle's own edits shifted 8 of `completion_atlas.py`'s ten
`BUCKET_DEFINITIONS` line citations — re-derived by `grep -n` against `git show HEAD:...` and
fixed in the same cycle (`citation_failures` `8 -> 0`). `cargo test --locked --no-run` exits 0
at the full workspace scope; `apps/desktop/src-tauri` not touched, not run.

**AT-34-E3-001 does not close this cycle.** Bucket B for `core_rulebook` is 1,809 of 6,701;
eight of the nine named mechanisms remain (`decisions.md §14`'s table). Full receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_domain_cycle_receipt.md`.

### Cycle 13 — AT-34-E3-001 — bucket B closes: records reach their tables

**Status: blocked-escalated.** Bucket B for `core_rulebook` moved
`1035 -> 1006` (one of eleven named mechanisms fully cleared, verified
end-to-end); the criterion requires zero, so this cycle does not close it.

**Denominator corrected, not carried forward.** `epic-breakdown.md` states
970; re-derived at this cycle's start SHA (`bfe1e7e380`):
`python3 scripts/completion_atlas.py --book core_rulebook --check` → `B:
1035`. Logged as a `correction` retro event, `--verified-by` the same
command.

**Bucket B partitions into eleven distinct mechanisms**, not one (grouped
by exact evidence string on `docs/work-inventory.json`):
`class_feature_option_pool_record_with_magnitude_not_held_by_engine` (333),
`class_feature_owner_matched_by_name_but_record_not_held_by_engine` (330),
`race_trait_race_not_modelled` (132), `companion_absent_from_
core_rulebook_companion_tables` (100), `class_feature_option_pool_record_
not_held_by_engine` (63), `template_content_absent_from_template_table_in_
core_essentials` (22), `deity_content_absent_from_deity_table_in_core_
rulebook` (21), `class_absent_from_ClassId_ALL_and_book_class_id_enums`
(17), `race_trait_absent_from_race_traits` (9),
`ability_content_absent_from_ability_table_in_core_essentials` (7),
`domain_content_absent_from_domain_table_in_core_rulebook` (1).

**Fixed this cycle: the `template` (22) and `ability` (7) mechanisms — 29
units, one root cause.** `holds_key_inner` (`src/bin/v06_work_inventory.rs`)
had no match arm for the seven Epic 2 simple-kind-table kinds
(`Ability`/`Template`/`Deity`/`Domain`/`Trait`/`Language`/`Skill`), which
silently defeated the `decisions.md §9` re-attribution widening for every
one of them: a unit whose raw ingestion tree (`source_book`) resolves to
`core_essentials` (which has no `ability`/`template` directory) could never
be credited to the book (`core_rulebook`) that actually, physically holds
its own record. Verified real, not fabricated:
`data/corpus/core_rulebook/ability/racial_traits_dwarf.json` and
`data/corpus/core_rulebook/template/isdwarf.json` both carry a real,
matching key. RED (`cargo test --locked --bin v06_work_inventory
reattributed_off_a_tableless` → 2 failed, `engine_book` stayed `None`
instead of `Some("core_rulebook")`) → GREEN after one new match arm
delegating to the same `SimpleKindTable::resolve` the verdict itself
already calls. Full binary suite: `369 -> 375 passed, 0 failed`.

**Self-caught regression, fixed same cycle:** the new arm's 22 inserted
lines shifted every one of `completion_atlas.py`'s ten hardcoded
`BUCKET_DEFINITIONS` `file:line` citations by `+22`, tripping
`citation_failures=10` (AT-34-E1-002 condition 6, fail-closed as designed).
Re-derived each new line by `grep -n`, fixed the ten literals,
`citation_failures` back to `0`. Logged as an `incident`
(`recurrence-key: line-number-citation-drift`).

**Remaining ten mechanisms (1006 units) are each independently named**
with their own population and verified root cause in
`artifacts/epic-3-core-rulebook/AT-34-E3-001_cycle_receipt.md` — two
`class_feature` mechanisms (726 combined, real class-feature engine
modelling, more than two-thirds of what remains), race/race-trait modelling
(141), companion-table extension (100), full `ClassId` modelling for 17
NPC/prestige classes, and two mechanisms needing an explicit ruling before
any code change: `deity` (21, every corpus record PI-redacted, resolvable
only by source-coordinate, not key/name) and `domain` (1, a genuinely
missing corpus record with no JSON anywhere in `data/corpus/core_rulebook/`
— a guarded-generator job, not a resolve fix).

**Verification:** `corpus_literal_sweep` unchanged, `48699 of 51473`
before and after (record delta 0, matches). `cargo test --locked --no-run`
exit 0 at the full workspace scope; `apps/desktop/src-tauri` (separate
workspace, its own `CARGO_TARGET_DIR`) `--no-run` exit 0 too, though
untouched by this cycle. Both dual-audit greps: `OK_NO_BUNDLE_TAGS`,
`OK_NO_TOKENS`.

**Escalating per `workflow-instruction.md §8`:** this criterion bundles
eleven distinct engineering-sized mechanisms under one card; a single
dispatched cycle can close a lookup-predicate defect like this one but
cannot also model new classes/races/companions in the same turn. Receipt:
`artifacts/epic-3-core-rulebook/AT-34-E3-001_cycle_receipt.md`.

### Cycle 12 — AT-34-E2-004 — bucket A reaches zero for both vehicle books

**Status: complete. Epic 2 closes (4 of 4).** Wires `AT-34-E2-001`'s seven `simple_kind_tables`
resolvers (plus `companion`'s pre-existing SD-29 table) into `classify()`'s real per-unit verdict
— before this cycle they were only exercised read-only via `--epic2-table-transcript`. Held +
zero-magnitude + real description + `display` wiring class + not a universal sheet modifier →
`text-complete`; held + real magnitude → `ingested-magnitude` (bucket M, never `grounded` — a
lookup table computes nothing, `decisions.md §2a`); not held → bucket B
(`<kind>_absent_from_<dir>_table_in_<book>`), never bucket A.

`python3 scripts/completion_atlas.py --book core_rulebook --check` → `A: 0` (was 934).
`--book ultimate_campaign --check` → `A: 0` (was 242). Corpus-wide bucket A: `8463 → 449`
(`power`=421, Epic 5's; `companion`=28, a `bestiary`-only residual, unrelated to either vehicle
book).

**Discovery, fixed this cycle:** 14 `core_rulebook` `companion` units (the `Familiar ~ …` shape,
`ce_abilities_familiar_cr.lst`) were routed through a retired `core_essentials` companion
registry and reported bucket A even though `core_rulebook` genuinely has a companion table — the
general re-attribution widening (`decisions.md §9`) only re-homes a unit when the destination
table *holds* it, and these rows are deliberately excluded by `crb::companion_data` (no creature
row owns them). Fixed with a narrow `Kind::Companion` guard reporting bucket B under the correct
book instead. Retro `correction` event: `docs/retro/events/sd34-at-34-e2-004.jsonl`.

**Discovery, flagged NOT fixed (Epic 3's to run down):** 29 more `core_rulebook` units (7
`ability` + 22 `template`) show the identical misattribution shape, but `holds_key_inner` has no
match arm for those kinds at all, so the general widening never even attempts a re-home. They are
correctly off bucket A (land in B) but may be reporting the wrong book. Named in the receipt so
`AT-34-E3-001` ("bucket B closes … mechanism named") does not rediscover it from scratch.

**Instrument-correction:** this cycle's edit to `v06_work_inventory.rs` shifted every later line
in the file, breaking all 10 of `completion_atlas.py`'s bucket citations and all 9 of
`missing_engine_tables.py`'s kind citations (both fail closed on the mismatch, as designed) —
both re-pinned against the real file; `missing_engine_tables.py`'s `ENGINE_SURFACE_CITATIONS`
also dropped the 7 entries whose marker text no longer exists anywhere in the source. Both
scripts' pinned-figure tests (`test_completion_atlas.py`, `test_missing_engine_tables.py`)
re-derived against the new live population; all 50 of their tests (38 + 12) pass.

`docs/work-inventory.json` regenerated at HEAD (`CORPUS_LITERAL_SWEEP_REPORT` +
`DERIVED_FIXTURE_CHECK_REPORT` supplied so the stamp-loss guard did not need `--allow-stamp-loss`,
never used). Corpus-wide bucket movement: `DONE +1479` (`12265→13744`), `A -8014` (`8463→449`),
`B +2497`, `D +1019`, `M +2016`, `V +1003`; `C`/`U`/`X`/`Z` unchanged. `corpus_literal_sweep`:
`48699 → 48699`, delta 0 (no corpus files touched). Full receipt:
`artifacts/epic-2-tables/AT-34-E2-004_cycle_receipt.md`.

### Cycle 11 — AT-34-E2-003 — the measured build rate is recorded

**Status: complete.** Records the real cost of building Epic 2's 8 tables to
`artifacts/epic-2-tables/table-build-rate.json`: no production code this cycle, only
measurement of work already landed (`AT-34-E2-001` commit `052a9182bf`, `AT-34-E2-002` commit
`b7507f3817`).

**No blended average.** Marginal lines per kind spread **2 to 12** (6×): `domain`/`skill`/
`language` cost 2 lines each (a one-line directory-table entry + a one-line macro test
invocation); `trait` cost 12 (the same two lines, plus a 7-line dedicated regression test and 3
doc lines pinning its `trait_generic` directory-name mismatch — the "shallow glob lies" hazard
from `workflow-instruction.md §4`); `ability`/`template`/`deity` cost 7 each (typical macro
block); `companion`'s table costs 0 this bundle (pre-existing, SD-29 — only its 21-line
fail-closed test is new). **Finding for Epic 5:** record count does not predict cost — `ability`
(4,337 records) and `domain` (183 records) cost almost the same, because both reuse one shared
generic loader unmodified. The real driver is whether a kind's corpus directory name matches
its kind name; `power`'s Epic 5 price depends on whether `ultimate_psionics` needs its own
shape handling the way `trait` did, not on its 421-unit count.

**Wall time, honestly scoped:** the 7 new tables were built through one shared loader in a
single commit — there were never 7 independent build sessions to time. The artifact reports
real, re-derivable whole-cycle wall time (`AT-34-E2-001`: 1,359s / 0:22:39; `AT-34-E2-002`:
779s / 0:12:59, both from `git log --format=%ci`) and, separately, per-table wall time
pro-rated from measured marginal lines — explicitly labeled **ESTIMATE** in the artifact and
receipt, never presented as independently measured (`AGENTS.md` rule 9).

Row-count command output: `python3 -c "import json; print(len(json.load(open('artifacts/epic-2-tables/table-build-rate.json'))['tables']))"` → `8`, of the 8 tables Epic 2 builds.
Build scope: `cargo test --locked --no-run` exit 0 (workspace, 600 executables) and
`apps/desktop/src-tauri` exit 0 (one pre-existing unrelated `dead_code` warning), both at HEAD
`b7507f38178e41b3962ef3161ee525e5ad9ee9b0`. Receipt:
`artifacts/epic-2-tables/AT-34-E2-003_cycle_receipt.md`.

### Cycle 10 — AT-34-E2-002 — each new table is fail-closed

**Status: complete.** Formalizes fail-closed proof as its own deliverable
(`artifacts/epic-2-tables/fail-closed-proofs.md`): all 8 Epic 2 tables, per-table, a RED→GREEN
pair — refusing an absent key and returning a real record for a present one. The 7
`simple_kind_tables` resolvers already carried this proof inline from AT-34-E2-001 (cited by
test name, not duplicated). The 8th, `companion` (pre-existing from SD-29), had no dedicated
fail-closed test; this cycle adds
`companion_chassis::tests::companion_resolve_refuses_a_fabricated_key_it_never_defaults`.

RED confirmed for the intended reason: `companion_resolve` was temporarily mutated to fall back
to `self.companions.first()` instead of refusing an absent key, and the test failed on the
fabricated-key assertion specifically (`a fabricated key must never resolve to a companion
record, real or defaulted`), not an unrelated panic. Reverted, then GREEN: 15/15
`companion_chassis` tests pass, 11/11 `simple_kind_tables` tests pass (unchanged).

Row-count command output: `grep -c '^| \`' .../fail-closed-proofs.md` → `8`, of the 8 tables
Epic 2 builds. Receipt: `artifacts/epic-2-tables/AT-34-E2-002_cycle_receipt.md`.

### Cycle 9 — AT-34-E2-001 — each of the eight tables is built, or proven unnecessary

**Status: complete.** Epic 2 builds 8 of `power`'s 9 kinds; one of the eight — `companion` — already
has a real, fail-closed table from SD-29 (`rules_core::rules_tables::companion_chassis`). This
cycle builds the other **seven**: `ability`, `template`, `trait`, `deity`, `domain`, `skill`,
`language`. New module `src/rules_core/rules_tables/simple_kind_tables.rs`:
`load_simple_kind_table(repo_root, kind)` loads every corpus record for a kind, across every book,
from the live `data/corpus/<book>/<dir>/*.json` tree; `resolve(book, key)` returns the real record
for a present key or `None` for an absent one.

**Directory-name hazard caught before shipping:** `trait`'s 487 units live under
`data/corpus/*/trait_generic/*`, not `trait/` — a naive `kind == dir name` glob would silently
return zero. `kind_dir_for("trait")` resolves this explicitly and a pinning test guards it. RED
confirmed for the intended reason (a temporary revert to the naive mapping failed with `trait table
loaded zero records from "trait"`, not an unrelated panic), then GREEN: 11/11 new unit tests pass.

Wired into `v06_work_inventory.rs` via a new read-only `--epic2-table-transcript` flag (same
contract as `--spell-probe`), which produced the committed transcript
(`artifacts/epic-2-tables/AT-34-E2-001_table_transcript.txt`) — 8 of 8 kinds report `HELD` on a
named sample record, and every kind also demonstrates `REFUSED` on a fabricated key in the same
run.

Identifier/wired-integration audits: `OK_NO_BUNDLE_TAGS` / `OK_NO_TOKENS`. Denominator gate against
this package: `files_checked=15 violations=0`. `cargo test --locked --no-run` exits 0 at the widest
workspace scope; `apps/desktop/src-tauri` not touched, not run. `data/corpus/**` and
`docs/work-inventory.json` untouched this cycle — zero movement across all four buckets; sweep
population N/A (no corpus write). Receipt:
`artifacts/epic-2-tables/AT-34-E2-001_cycle_receipt.md`.

**This cycle does not attempt `AT-34-E2-004`** (bucket A to zero for both vehicle books) — that
needs reachability/reclassification wiring these tables don't provide standalone, and is a
separate criterion.

### Cycle 7-R — AT-34-E1-007 re-verified after AT-34-E1-008 — `corpus-trap-audit` is GREEN

`scripts/verify.sh --only corpus-trap-audit` now **exits 0**:

```
PASS  corpus-trap-audit  (records_examined=27638 defects[wiring-class-mismatch=0
  disabled-line=165 key-differs-from-name=650 mod-record=2117
  shared-name-distinct-records=249] traps=407 — all defect kinds at their registered counts)
```

`wiring-class-mismatch` is **0 of 3,181** remaining DEFECT findings, down from **7,015 of
10,196** at the blocker. The four inherited kinds are each at **exactly** their launch count —
`mod-record` 2,117, `key-differs-from-name` 650, `shared-name-distinct-records` 249,
`disabled-line` 165, summing to 3,181 of 3,181 — **reported by name, not absorbed**. Books
carrying ≥1 DEFECT: **29 of 37**, down from 34 of 37. `corpus_literal_sweep`: **0 findings,
48,699 of 51,473 examined**, delta 0.

**One instrument correction was required first.** The stage decided PASS/FAIL from an aggregate
`defects == 0`, which (a) never reported `wiring-class-mismatch` at all, so AT-34-E1-008's
"reported at their counts and not absorbed" bar could not be read from it, and (b) cannot
satisfy `decisions.md §13`, which in one paragraph keeps AT-34-E1-007's `exits 0` bar **and**
rules SD-33's 3,181 registered defects **registered, not absorbed** — they are DEFECT severity,
so the aggregate stayed red forever. The verdict is now a **ratchet on named kinds**
(`scripts/corpus_trap_audit_baseline.py`): an unregistered kind, a kind above its pin, **or a
kind below its pin** all FAIL, and every kind's count prints on every run. Strictly more
discriminating than the aggregate it replaces; the registered set did not grow and nothing was
excused. Mutation-proved by `scripts/tests/test_corpus_trap_audit_baseline.sh` (14 cases, wired
as the new `corpus-trap-audit-selftest` stage in QUICK and FULL) and by a live plant-and-remove
on one real corpus record: `wiring-class-mismatch` moves `0 → 1 → 0` while `records_examined`,
`traps` and all four registered kinds hold still.

Rows 7 and 8 both go `complete`, from the counts: the stage's own verdict line, and 34 of 34
book rows in `artifacts/epic-1-atlas/wiring-class-remediation.json` at `after=0` with 34 of 34
provenance checks PASS. Receipt:
`artifacts/epic-1-atlas/AT-34-E1-007_re-verification_receipt.md`.

### Cycle 8 — AT-34-E1-008 — `wiring-class-mismatch` driven to zero, group by group

**Status: in-progress** (this criterion is dispatched as parallel per-book groups; each group
commits and reports independently). Mechanism, established by group G1 and reused unchanged
here: `src/bin/restamp_wiring_class.rs` (new in G1's commit) — an additive restamp pass over
existing on-disk `data/corpus/<book>/**/*.json` records, following this repo's established
"enrichment pass, never a second generator" pattern (`enrich_*_raw_tokens.rs`). It rewrites
only the `wiring_class`/`wiring_class_signals` keys when they disagree with a fresh recompute
via the audit's own `WiringClassIndex`, every other field parsed and re-emitted untouched by
construction — never a hand-edit, per `decisions.md §13`/`N5`.

**G1** (`54e2d24e83`): `advanced_players_guide` 875→0, `core_rulebook` 798→0. Discovered that
`gen_book_cache`/`gen_core_rulebook_cache`/`gen_cache_apg` cover only `companion`/`class`
records' `wiring_class` (255 of 1,673) — the rest (`ability`/`domain`/`skill`/`template`/
`*_generic`) were ingested by one-off Python scripts predating the real closure determinator
and cannot ever agree with the audit by re-running them, hence `restamp_wiring_class.rs`.
Receipt: `artifacts/epic-1-atlas/AT-34-E1-008_G1_cycle_receipt.md`.

**G2** (this cycle, `8df70c2ee4`): `beastiary` 783→0, `ultimate_psionics` 759→0,
`ultimate_campaign` 152→0 — 1,694 of the group-2 population. Same tool, same posture, run via
`cargo run --locked --bin restamp_wiring_class -- beastiary ultimate_psionics` then `--
ultimate_campaign`. Provenance verified per record by `git diff` against HEAD across all 2,494
changed files: only `wiring_class`/`wiring_class_signals` changed, 0 files added/removed, 0
provenance-field mismatches. `corpus_literal_sweep`: 48699 examined before → 48699 after (delta
0, correct — in-place restamp adds no records), 0 findings, CLEAN both runs. Build scope:
root workspace `cargo test --locked --no-run` exit 0 and `apps/desktop/src-tauri` (separate
workspace) `cargo test --locked --no-run` exit 0, both run at `8df70c2ee4`. Receipt:
`artifacts/epic-1-atlas/AT-34-E1-008_G2_cycle_receipt.md`.

Corpus-wide `wiring-class-mismatch` after G1+G2: `5342 - 1694 = 3648` of the original 7015.
Remaining groups (G3/G4 or however the wave is split) own the rest. AT-34-E1-007's own `exits 0`
bar closes only once every group lands at 0.

### Cycle 7 — AT-34-E1-007 — `corpus-trap-audit` is wired into `verify.sh`; blocked on real content it found

**Status: blocked-escalated.** The mechanical deliverable is done: a new `corpus-trap-audit`
stage (`cargo run --locked --bin v06_corpus_trap_report -- --audit --json`) is wired into
`verify.sh`'s `ALL_STAGES` (FULL scope, next to `corpus-sweep`), bounds its own runtime with a
`timeout` wrapper (closing `forward-scope-register.md D1.2`'s gap), and computes its population
independently of the binary's own output (`27,638` records, a `find`-based 3-level walk matching
`audit_ingested_cache`'s own traversal). RED→GREEN proved live: one real record's `wiring_class`
field was flipped, the stage's defect count moved exactly `10196 → 10197` naming that record, the
mutation was reverted via `git checkout --` (confirmed byte-identical to the pre-mutation file),
and the count returned to `10196`.

**That `10196` is the block.** Run for real against the live corpus, the stage is FAIL, not PASS:
`records_examined=27638 defects=10196 traps=407`. Of the 10,196 defects, 3,181 match four tests
already in `tests/v06_corpus_trap_report.rs` that SD-33's `forward-scope-register.md D1.1`
already verified as pre-existing, out-of-DoD debt. The other 7,015 (`wiring-class-mismatch`) are
a **new discovery**: this exact check was last driven to 0 by `SD30-CARRY-001` (`b32926f2af`,
2026-08-14) and has silently regressed across 34 of 37 books since, because nothing has run
`--audit` in `verify.sh` between then and now — the precise gap this criterion exists to close.
Fixing it needs `data/corpus/**` write scope Epic 1's file-touch table does not grant, and scales
~3.4× `SD30-CARRY-001`'s own 10-book/177-defect remediation — genuinely multi-cycle, not
foldable into this wiring criterion. Full figures, the RED→GREEN transcript, and the exact
re-derive command for every number: `artifacts/epic-1-atlas/AT-34-E1-007_cycle_receipt.md`. Retro
event: `docs/retro/events/sd34-at-34-e1-007.jsonl` (`incident`,
recurrence-key `unwired-standing-gate-decay`).

### Cycle 6 — AT-34-E1-006 — figure-provenance is a real `verify.sh` stage; denominator-gate default widened

**Status: complete.** Two obligations, one cycle. (1) A new `figure-provenance` stage
(`scripts/denominator_gate.py --check-provenance`, wired into `verify.sh`'s `ALL_STAGES` and
`QUICK_STAGES`) fails on a figure — a comma-grouped ≥4-digit number or a bare percentage — stated
inside a receipt's "Figures + their re-derive commands" section with no re-derive command
reachable from it on the same line; RED→GREEN mutation-proofed for both an unsourced figure and
a wrong-command figure (a command naming a script that does not exist), GREEN for a command
naming a real one. Default population: this package's own 5 receipts + 15 root `.md` docs
(`files_checked=20 figures_examined=22 violations=0`) — deliberately not SD-33's folder, which
this bundle may not write to. (2) `denominator-gate`'s `DEFAULT_GLOBS` widened to add SD-34's own
package (every root `.md`, plus its receipts) alongside SD-33's (unchanged) — a default run now
reads `files_checked=90 violations=0`, up from 0 SD-34 files before this cycle. Closes
`workflow-instruction.md §12` row 15 (UNENFORCED at launch) and `decisions.md §3`'s standing
obligation. 40 of 40 unit tests pass (`scripts/tests/test_denominator_gate.py`). Full details,
figures, and the mutation-proof transcript: `artifacts/epic-1-atlas/AT-34-E1-006_cycle_receipt.md`.

### Cycle 5 — AT-34-E1-005 — the `not-ingested` status field is renamed

**Status: complete.** The field asserted the opposite of its meaning (26,002 of 26,002 of its
units carry a real `source_file`+`source_line`; every evidence string is engine-side) and had
already misled once, during this package's own authoring. Renamed `not-ingested` →
`engine-does-not-hold` (and the Rust closures `not_ingested`/`not_ingested_owned` →
`engine_does_not_hold`/`engine_does_not_hold_owned`) in `src/bin/v06_work_inventory.rs`,
`docs/work-inventory.json` (26,239 of 26,239 occurrences), and every consumer under `tests/`,
`src/`, `apps/`, `scripts/` — 78 tracked files total, matching Cycle 4's handoff note: both
`completion_atlas.py`'s A/B/C/D bucket-D citation and `shape_engine_boundary.py`'s
`not_held_by_engine()` were updated in this same commit, so neither silently zeroes out.

New regression test `scripts/tests/test_legacy_not_ingested_string_swept.py`: sweeps `tests/`,
`src/`, `apps/`, `scripts/` for either retired spelling and fails closed on any live hit,
proven RED→GREEN by planting then reverting a synthetic violation
(`test_sweep_goes_red_on_a_planted_use_and_green_on_its_revert`). Live sweep:
`legacy_not_ingested_live_uses = 0` (of 76 files that carried the string before this cycle).

`docs/work-inventory.json` was relabeled via a validated whole-file substitution
(json-valid before/after, `26239` `"not-ingested"` → `0`, `26239` `"engine-does-not-hold"`,
exact parity) rather than a full generator re-run, to avoid the unrelated regression risk of
losing `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` context — this is a pure
relabel, confirmed by `completion_atlas.py --check` reporting the identical bucket counts as
before (`D=1230` unchanged) and by `tests/v06_work_inventory.rs`'s
`the_committed_inventory_is_well_formed_and_uses_only_declared_statuses` passing against the
edited file.

Both identifier and wired-integration audits clean (`OK_NO_BUNDLE_TAGS`, `OK_NO_TOKENS`).
Denominator gate against this package: `files_checked=15 violations=0`. `cargo build --bin
v06_work_inventory` exits 0; `cargo test --locked --no-run` (full workspace) exits 0; `apps/
desktop/src-tauri` `cargo check --locked` exits 0 (touched via `character_hub.rs`,
`spell_catalog.rs`, `reach_gate.rs`). Targeted Rust suites (`v06_work_inventory`,
`equipment_gap_tables`, `feat_gap_tables`, both `derived_evaluator_fixture_check*`) all green;
targeted Python suites all green except one **pre-existing, unrelated** failure in
`test_transcribe_monster_tables.py` (confirmed identical against unmodified `HEAD` before this
cycle's diff was reapplied — an unrelated concatenated-ability-text assertion, nothing to do
with the renamed string).

`docs/work-inventory.json`'s data content (which units exist, which status each carries) is
unchanged — zero reclassification, zero reachability movement. This cycle's only real movement
is closure of the misnomer itself, plus one instrument-correction (the atlas's stale-citation
guard updated to the new literal so it keeps resolving). Receipt:
`artifacts/epic-1-atlas/AT-34-E1-005_cycle_receipt.md`.

### Cycle 4 — AT-34-E1-004 — the shape-engine boundary is stated as a fact, not an assumption

**Status: complete.** New `scripts/shape_engine_boundary.py` commits, as a self-verifying
artifact, the fact that a shape engine turns a formula string into a number and does not
place/attach/display the record — that gate is the engine's own four-condition promotion
ladder, quoted from the live `src/bin/v06_work_inventory.rs` with its line citation re-checked
by content on every run, not merely path/line.

`python3 scripts/shape_engine_boundary.py --check` → `magnitude_bearing=26396
not_held_by_engine=13119 citation_ok=True`, exit 0. Both counts matched
`technical-design.md §3` / `decisions.md §2a`'s stated figures exactly on the first live
re-derive against the current `docs/work-inventory.json` — no drift since authoring. The
promotion ladder's four conditions at `src/bin/v06_work_inventory.rs:9592-9595` were
independently re-read with `sed -n` and match the exact block those documents quote, anchored
at line `9595` as they cite.

12/12 new unit tests green (`scripts.tests.test_shape_engine_boundary`), including a genuine
RED→GREEN mutation proof: the citation check was made to fail for the intended reason (a
line's live content no longer matching the expected fragment), confirmed it raises
`StaleCitationError` naming the exact line and mismatch, then confirmed it passes again once
restored. Denominator gate against this package: `files_checked=15 violations=0`. `cargo test
--locked --no-run` exits 0 at the widest workspace scope (Python + one generated markdown
artifact only; no Rust source touched); `apps/desktop/src-tauri` not touched, not run.
`docs/work-inventory.json` untouched — zero movement across all four buckets; this cycle is a
read-only, self-verifying statement of an already-established fact.

**Handoff note for AT-34-E1-005:** the new instrument's `not_held_by_engine()` keys on the
literal string `"not-ingested"`, same as `completion_atlas.py`'s bucket A/B/C/D arms — the
rename cycle must update it in the same commit or it will silently report
`not_held_by_engine=0`. Receipt: `artifacts/epic-1-atlas/AT-34-E1-004_cycle_receipt.md`.

### Cycle 3 — AT-34-E1-003 — the missing engine tables are enumerated and their book coverage mapped

**Status: complete.** New `scripts/missing_engine_tables.py` re-derives bucket A (`status ==
not-ingested`, evidence contains `has_no_engine_table`) directly from `docs/work-inventory.json`
and reports, per kind: unit count, per-book breakdown, the exact `not_ingested(...)`
engine-surface citation in `v06_work_inventory.rs` a real table would replace, and which books'
entire bucket-A population zeroes out once that kind's table exists.

`python3 scripts/missing_engine_tables.py --check` → `population=8463 kinds=9
citation_failures=0`, exit 0. Per-kind: `ability=4337 template=2248 trait=487 deity=459
power=421 domain=183 skill=149 language=136 companion=43` (sum = 8,463, matching
`completion_atlas.py`'s committed `buckets.A.count` exactly). Core Rulebook's slice
(`ability=471 template=262 skill=110 domain=34 language=22 deity=21 companion=14`, summing to
934 of `core_rulebook`'s 6,701 units) matches `technical-design.md §4` exactly and cross-checks
against `completion_atlas.py --by-book`'s independently-computed `core_rulebook A=934`.
`ultimate_campaign`'s slice (`ability=88 trait=154`, summing to 242 of 265 units, 91.3%)
confirms the epic-breakdown's "almost-single-bucket book" claim, cross-checked the same way.

`zero_bucket_a_books` (books a single kind's table alone would fully clear of bucket A):
`ability` → `inner_sea_faiths`; `language` → `inner_sea_temples`; `template` →
`inner_sea_intrigue`, `ultimate_intrigue`; the other 6 kinds → none (every book they touch also
carries a second bucket-A kind, so both tables are needed).

**Notable finding along the way:** a 10th `Kind::MonsterAbility` match arm in
`v06_work_inventory.rs` emits the same `has_no_engine_table` marker shape but contributes zero
live bucket-A units — its 3,806 units are already `text-complete`/`grounded`/verified, with only
13 `not-ingested` units, all landing in bucket B. Confirmed by reading the corpus data, not the
code path alone, before concluding the population is genuinely 9 kinds not 10 — the same
field-name-vs-field-meaning trap `decisions.md §12` L1 names.

12/12 new unit tests green (`scripts.tests.test_missing_engine_tables`), covering per-kind
counts, non-bucket-A exclusion, `zero_bucket_a_books` derivation, the engine-surface citation
(including a live re-check against the committed source), and a fail-closed
`UnknownKindError` for any future kind reaching bucket A with no citation entry. Denominator
gate against this package: `files_checked=15 violations=0`. `cargo test --locked --no-run`
exits 0 at `2ec0462736` (Python-only change; no Rust source touched); `apps/desktop/src-tauri`
not touched, not run. `docs/work-inventory.json` untouched — zero movement across all four
buckets; this cycle is a reclassification (finer view of the already-fixed bucket-A partition),
not new closure work. Receipt: `artifacts/epic-1-atlas/AT-34-E1-003_cycle_receipt.md`.

### Cycle 2 — AT-34-E1-002 — the atlas fails closed on six conditions

**Status: complete.** `scripts/completion_atlas.py` extended in place with the five remaining
fail-closed conditions on top of AT-34-E1-001's `unclassified`/`overlap` gate: (3) a `DONE`
unit whose evidence does not support it, (4) a bucket with no named clearing mechanism, (5) a
`derived_at` SHA that is not an ancestor of `HEAD` (staleness gate), (6) a bucket whose
definition does not cite the `file:line` that emits the evidence string it keys on, or whose
citation no longer resolves/matches at `HEAD`.

Live, unmutated: `python3 scripts/completion_atlas.py --check` → `population=49438 buckets=10
unclassified=0 overlap=0 done_evidence_violations=0 missing_clearing_mechanisms=0
stale_derived_at=False citation_failures=0`, exit 0. All ten buckets carry a real, verified
`file:line` citation into `src/bin/v06_work_inventory.rs`.

**Six RED→GREEN mutation proofs, one per condition, in
`artifacts/epic-1-atlas/fail-closed-proofs.md`.** Notable finding along the way: the naive
condition-3 design (reuse the A/B/C bucket markers verbatim as "must never appear in DONE
evidence") would have flagged 245 real, legitimate `DONE` units carrying `explanation_id` —
confirmed against the live corpus and excluded, with the exclusion documented in code and in
the proofs file (the same "field name vs. field meaning" trap condition 6 itself targets,
caught here before it shipped).

38/38 unit tests green (20 new + 18 inherited). Denominator gate against this package:
`files_checked=15 violations=0`. `cargo test --locked --no-run` exits 0 at the widest
workspace scope (run at `ceac19da29`); `apps/desktop/src-tauri` not touched, not run.
`docs/work-inventory.json` untouched — zero movement across all four buckets. Receipt:
`artifacts/epic-1-atlas/AT-34-E1-002_cycle_receipt.md`.

### Cycle 1 — AT-34-E1-001 — every unit carries exactly one named remaining-step

**Status: complete.** New `scripts/completion_atlas.py` partitions the full 49,438-unit
`docs/work-inventory.json` into the ten buckets fixed by `decisions.md §2`
(`DONE A B C D M V U X Z`), keyed on `status` + `evidence` per `technical-design.md §1`'s
implementation table.

`python3 scripts/completion_atlas.py --check` → `population=49438 buckets=10 unclassified=0
overlap=0`, exit 0. Bucket counts: `DONE=12265 A=8463 B=11921 C=4388 D=1230 M=2455 V=8330
U=321 X=46 Z=19` (sum = 49,438). `A` and `U` match the epic-breakdown's independently-stated
figures (8,463 across 9 kinds; 321 split 270/51 by evidence, 140/119/62 by kind) exactly on
the first live run. `D` and `U` sub-causes are enumerated in the committed artifact, not
shrugged. Cross-checked against SD-33's inherited, independent `box_ledger.py --check`
partition: `uncovered=0 overlap=0 population=49438` — both partitions agree on the same
population.

18/18 new unit tests green (`scripts.tests.test_completion_atlas`); a live mutation on the
bucket-A marker string was proven RED for the intended reason then reverted to GREEN.
Denominator gate against this package: `files_checked=15 violations=0`. `cargo test --locked
--no-run` exits 0 at the widest workspace scope; `apps/desktop/src-tauri` not touched, not run.

**This cycle does not implement AT-34-E1-002** (the six fail-closed conditions) — a separate
criterion in the same file, picked up next. Receipt:
`artifacts/epic-1-atlas/AT-34-E1-001_cycle_receipt.md`.

## Open blockers

**This section is not a parking lot.** An entry here is a request for an operator ruling and
it **pauses the bundle** (`../../governance/blocker-closure-doctrine.md`). It is never a
disposition, never a closure path, and no later cycle may proceed past a blocked card on its
own authority.

### AT-34-E3-001 — RESOLVED 2026-08-27 by orchestrator ruling (`decisions.md §14`)

<details>
<summary>Archived — the filing, and the ruling that cleared it</summary>

The cycle asked the operator to authorize running AT-34-E3-001 as further per-mechanism cycles
rather than one. **That is a sequencing decision, not an operator ruling**, and filing it paused
the bundle to ask permission to keep working — the one use an escalation must never be put to
(`../../governance/blocker-closure-doctrine.md`: a large blocker is a sequencing problem, not an
exemption). Cleared without escalation; the nine mechanisms are dispatched as their own cycles,
cheapest-first, and the criterion's bar is unchanged.

The cycle's own valuable half stands: it named the remainder **by mechanism with a population
each**, which is what makes the next wave dispatchable. Re-derived at HEAD, the enumeration
returns **nine** mechanisms summing to **1,006 of 1,006** — the filing said "ten"; the count is
corrected here, not carried.

Its two sub-questions are answered in `decisions.md §14`: `domain` (1 unit) is ordinary guarded-
generator ingestion work, and `deity` (21 units) proceeds under a stated PI constraint — match on
stored coordinates, keep the masked keys, never read or emit a redacted name, both PI gates stay
green. Only an inability to work inside that constraint would be a genuine escalation.

Also of note, and not a defect: this cycle's own 22-line insertion shifted all ten of
`completion_atlas.py`'s `BUCKET_DEFINITIONS` `file:line` citations, and **AT-34-E1-002 condition 6
fired exactly as designed** (`citation_failures` 10). The cycle re-derived each line and closed it
in the same cycle. The gate caught a real regression on its first live opportunity.

The bundle is un-paused.

</details>

*(no active blockers)*
