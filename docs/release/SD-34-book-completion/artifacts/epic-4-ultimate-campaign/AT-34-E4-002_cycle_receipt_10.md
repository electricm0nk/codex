# Cycle 10 — Epic 4 (Ultimate Campaign to zero) / AT-34-E4-002

- **Commit SHA:** `7714a6a5ef` (eighth trait slice + picker wiring for it;
  `c1cbfa0698` is the picker-gap-fix-only commit this cycle made first).
  progress/kanban update follows in a further commit per §5.
- **Provenance.** Worktree opened at `ea2b3396f2` (the tranche cut, stale).
  `git fetch origin && git log --oneline -1 origin/tranche/14` showed
  `15485e5197` (wave 22) — well past the dispatch brief's own stated
  baseline (`a0cbc2388a`). `git reset --hard origin/tranche/14` (clean
  worktree, no local work to lose) before touching anything. Re-derived
  the split fresh at that HEAD: `python3 scripts/completion_atlas.py
  --book ultimate_campaign --check` read `DONE=203 M=37 D=2 U=21 X=2`,
  population 265, unclassified 0 — matching the dispatch brief's own
  stated baseline exactly (cycle 9's closure had already been folded by
  the wave-21 shared regen, confirmed in `progress.md`'s wave-21 entry).
  Read `AT-34-E4-002_cycle_receipt_9.md` as the newest receipt per the
  dispatch brief. The dispatch brief itself (wave 22, commit `9ae5a08fd4`)
  named the picker gap cycle 9 found as this cycle's own explicit first
  task, ahead of any new bucket slice — followed verbatim.
- **Files touched (picker-gap fix, commit `c1cbfa0698`):**
  `apps/desktop/src-tauri/src/trait_picker.rs` (+351/-20: `list_available_
  character_traits` gains two new chained passes — `INITIATIVE_TRAIT_
  BONUSES`/`CONCENTRATION_TRAIT_BONUSES` merged by `trait_id` into a new
  `other_pillars: Vec<TraitOtherPillarBonusDto>` field, and `ABILITY_DIFF_
  SKILL_TRAIT_BONUSES` into a new `ability_substitution: Option<...>`
  field — plus a corrected module/command doc comment retracting cycles
  7/8's false "surfaces every selected trait generically" claim; 18
  new/updated tests), `apps/desktop/src/boundary/loadCharacterTraits.ts`
  (+48/-3: matching TypeScript types + corrected doc comment),
  `apps/desktop/src/characterHub/CreateCharacterForm.tsx` (+26/-12: the
  rendered suffix now branches on `otherPillars`/`abilitySubstitution`,
  never silently dropping a shape it cannot render).
- **Files touched (eighth trait slice, commit `7714a6a5ef`):**
  `src/rules_core/trait_effects.rs` (+423/-14: module doc comment's
  "Eighth slice" section — including this cycle's own correction of
  receipt 9's "needs a per-subschool caster-level pillar this crate does
  not have" characterization, true for 6 of the 7 remaining records but
  not this one — new `TraitCasterLevelSkillBonus` struct, the 1-entry
  `CASTER_LEVEL_SKILL_TRAIT_BONUSES` table, `find_caster_level_skill_by_
  trait_id`, `caster_level_skill_bonuses_from_traits`,
  `caster_level_subschool_facts_from_traits`, `caster_level_subschool_
  fact_explanation_id`, `caster_level_skill_trait_magnitude_is_grounded_
  for_corpus_key`, 9 new tests), `src/rules_core/pilot_compute/mod.rs`
  (+21/-0: `ground_orphan_trait_facts` gains a loop over `caster_level_
  subschool_facts_from_traits`, the same standalone-fact idiom `feat_
  effects::spell_focus_facts_from_choices` already established for a
  per-school DC bonus), `src/rules_core/skill_allocation.rs` (+18/-0:
  eighth fold-in loop in `allocate_skill_ranks`, reusing the SAME
  consumer the first three slices established, for Eldritch Delver's
  separate flat two-skill token only), `src/bin/v06_work_inventory.rs`
  (+52/-5: eighth `.or_else` classifier fallback onto `caster_level_
  skill_trait_magnitude_is_grounded_for_corpus_key`, doc-comment update,
  1 new positive-classifier test), `apps/desktop/src-tauri/src/trait_
  picker.rs` (+106/-10 on top of the picker-gap-fix commit: a ninth
  chained iterator over `CASTER_LEVEL_SKILL_TRAIT_BONUSES` reusing
  `other_pillars` for the caster-level half and `skills`/`bonus` for the
  flat half — the same fix-the-gap-for-your-own-new-record discipline
  cycle 9 established — 3 new tests), `apps/desktop/src/characterHub/
  CreateCharacterForm.tsx` (+11/-7 on top: the rendered suffix now
  combines a skill/save/choice part AND an `otherPillars` part instead of
  treating them as mutually exclusive, so Eldritch Delver's two halves
  both render).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` on this cycle's own
  diff (against this cycle's own starting HEAD `15485e5197`, scoped to
  `src/rules_core/`, `src/bin/`, `apps/desktop/src-tauri/src/trait_
  picker.rs`, `apps/desktop/src/characterHub/CreateCharacterForm.tsx`,
  `artifacts/epic-4-ultimate-campaign/`, excluding `**/__tests__/**`/
  `**/*.test.*` — zero hits).
- **Wired-integration audit result:** `OK_NO_TOKENS` on this cycle's own
  diff (same scope — zero hits).
- **Acceptance criterion (verbatim, epic-breakdown.md AT-34-E4-002):**
  `python3 scripts/completion_atlas.py --book ultimate_campaign --check`
  exits 0 with `DONE=265 of 265`, every other bucket zero, plus
  `artifacts/epic-4-ultimate-campaign/ultimate-campaign-completion-
  manifest.json`. **Not met this cycle** — real, incremental,
  fixture-verified progress on top of nine prior cycles: `DONE=204 of
  265` (functional, per the classify()-level bin test below; committed
  `docs/work-inventory.json` remains at `DONE=203` — this cycle's own
  regeneration is local/uncommitted only, per this dispatch's
  file-ownership rule assigning it to the wave's shared regeneration
  cycle, same as every prior cycle in this module), remainder
  `M:36 U:21 X:2 D:2` = 61. The completion manifest artifact remains out
  of scope until every bucket clears.

## Figures + their re-derive commands

| Figure | Value | Command / denominator |
|---|---:|---|
| `ultimate_campaign` bucket split, re-derived at cycle start (this cycle's own reset point) | `DONE=203, M=37 (trait 7 + ability 30), U=21, D=2, X=2, V=0` of 265 | `python3 scripts/completion_atlas.py --book ultimate_campaign --check` at `origin/tranche/14` HEAD `15485e5197` |
| Desktop picker gap, re-confirmed before any code change | `list_available_character_traits` chained 4 of 7 `trait_effects` tables (`FLAT_SKILL_TRAIT_BONUSES`, `SKILL_CHOICE_TRAIT_BONUSES`, `FAMILY_CHOICE_TRAIT_BONUSES`, `SAVE_TRAIT_BONUSES`, `SITUATIONAL_SKILL_TRAIT_BONUSES` — 5 actually, cycle 9's own seventh slice already chained) — `INITIATIVE_TRAIT_BONUSES`, `CONCENTRATION_TRAIT_BONUSES`, `ABILITY_DIFF_SKILL_TRAIT_BONUSES` (fifth/sixth slices) absent, leaving 7 already-grounded traits unselectable | direct read of `apps/desktop/src-tauri/src/trait_picker.rs`'s `use` block and `list_available_character_traits` body before this cycle's own edits |
| Desktop picker gap, after this cycle's fix | 7 of 7 `trait_effects` tables now chained (52 options before the eighth slice, 53 after); every one of the 7 previously-unreachable traits (Tactician, Arcane Temper, Desperate Resolve, Bruising Intellect, Planar Savant, Pragmatic Activator, Precise Treatment) now returned by `list_available_character_traits` and round-trips through its real compute path | `cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bin codex-desktop -- trait_picker` (18 new/updated tests, all passing — see Build scope) |
| Picker gap's effect on the `DONE`-bucket count | **0** — all 7 traits were already `grounded` at the `classify()` level before this cycle (`initiative_or_concentration_trait_magnitude_is_grounded_for_corpus_key`/`ability_diff_skill_trait_magnitude_is_grounded_for_corpus_key` were already wired as classifier rungs since cycle 8); this cycle closes a no-stub UI-reachability gap, not a bucket | `grep -n "initiative_or_concentration_trait_magnitude_is_grounded_for_corpus_key\|ability_diff_skill_trait_magnitude_is_grounded_for_corpus_key" src/bin/v06_work_inventory.rs` (both present before this cycle's own diff) |
| The 1 `ultimate_campaign` `trait_content` record whose remaining tokens mix `BONUS:CASTERLEVEL\|SUBSCHOOL` and `BONUS:SKILL`, re-confirmed against the live corpus JSON | `trait_eldritch_delver`: `SKILL\|Knowledge (dungeoneering),Knowledge (history)\|1\|TYPE=Trait` + `CASTERLEVEL\|SUBSCHOOL.Teleportation\|1\|TYPE=Trait` | direct read of `data/corpus/ultimate_campaign/trait_generic/trait_eldritch_delver.json`'s own `data.raw_tokens` |
| Re-check of receipt 9's "needs a per-subschool caster-level pillar this crate does not have" characterization of the whole remaining group | **False for Eldritch Delver** (this crate already grounds the analogous shape — `feat_effects::spell_focus_facts_from_choices`'s own per-school DC standalone fact — so the caster-level half needed only a new small table + standalone-fact producer, not a new pillar concept); **true for the other 6** (3 `VAR` records name 7 engine variables of which this crate grounds a total for exactly 1, `OracleChannelDC`; 2 `ABILITYPOOL` records need a genuinely new heterogeneous choice mechanism) | direct read of `pilot_compute::oracle_channel_dc`'s own doc comment (lines ~16829-16851) plus the 3 `VAR` records' and 2 `ABILITYPOOL` records' live corpus JSON; logged as a retro `correction` (see Notes) |
| Unit genuinely promoted M → DONE (`grounded`), this cycle | **1** in `ultimate_campaign` (`Trait ~ Eldritch Delver`); 0 corpus-wide payoff elsewhere (checked: `grep -rl "Trait ~ Eldritch Delver" data/corpus/` finds it ONLY under `ultimate_campaign/trait_generic/` — no other book or `Kind` carries this `KEY`) | classify()-level bin test (`a_caster_level_and_skill_trait_bonus_promotes_a_held_trait_record_to_grounded`), asserting `verdict.status == "grounded"` for the record's real corpus `KEY`; corpus-wide `grep -rl "Trait ~ Eldritch Delver" data/corpus/` |
| `ultimate_campaign` bucket state after this cycle (functional, per classify()-level tests; NOT baked into the committed `docs/work-inventory.json` this cycle — see Sweep population) | `DONE 203→204, M 37→36` (`trait` M `7→6`; `ability` M unchanged `30`), all other buckets unchanged (`D:2 U:21 X:2 V:0`) | `cargo test --locked --bin v06_work_inventory` (the 1 new positive-classifier test, asserting the exact `grounded` status + evidence string for the record's real corpus key) |
| `completion_atlas.py --check` corpus-wide (committed `docs/work-inventory.json`, unchanged by this cycle) | `population=49438 unclassified=0` | `python3 scripts/completion_atlas.py --check` |
| `corpus_literal_sweep --json-out` | `clean:true records_examined:48708` — unchanged from cycle 9's own baseline, no `data/corpus/**` file touched this cycle | `cargo run --locked --bin corpus_literal_sweep -- --json-out <report>` |
| `derived_evaluator_fixture_check --json-out` | `1839 unit(s) cleared over 2580 fixture row(s); 0 failed; 0 not ingested` — unchanged | `cargo run --locked --bin derived_evaluator_fixture_check -- --json-out <report>` |
| Row-count command output (see below) | `1` distinct trait id in the new table | see next section |
| Denominator gate against this package | `files_checked=15 violations=11` — all 11 pre-existing verbatim-quoted-corpus-prose false positives in `progress.md` (the "75% chance..." pattern `AT-34-E3-004` already flagged, growing by one each cycle as this progress entry itself re-cites the pattern in this receipt-style report; no new bare-percentage violation) | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` |

## Row-count command output

```
$ awk '/pub static CASTER_LEVEL_SKILL_TRAIT_BONUSES/,/^\];/' src/rules_core/trait_effects.rs \
    | grep -oE 'trait_id: "trait:[a-z_]+"' | sort -u
trait_id: "trait:trait_eldritch_delver"
$ awk '/pub static CASTER_LEVEL_SKILL_TRAIT_BONUSES/,/^\];/' src/rules_core/trait_effects.rs \
    | grep -oE 'trait_id: "trait:[a-z_]+"' | sort -u | wc -l
1
```

This cycle's own artifact is the one new table; its 1 distinct `trait_id`
is exactly the `ultimate_campaign` `M → DONE`-bucket delta this cycle
claims.

## Build scope verified

`cargo build --locked --lib`: exit 0 (only pre-existing warnings, no new
ones from this cycle's code). `cargo test --locked --lib -- trait_effects
skill_allocation`: **84/84 passed** (9 new `trait_effects` tests, on top
of cycle 9's own 75/75 baseline). `cargo
test --locked --bin v06_work_inventory`: **502/502 passed** (re-run twice
for stability; 1 of the 502 is this cycle's own new positive classifier
test; unchanged negative control `Trait ~ Fate's Favored` remains
`ingested-magnitude`, confirmed not promoted).
`apps/desktop/src-tauri` (separate cargo workspace, tested explicitly,
own `CARGO_TARGET_DIR`): `cargo test --locked --manifest-path
apps/desktop/src-tauri/Cargo.toml --bin codex-desktop -- trait_picker`:
**42 passed, 1 failed** (`trait_picker`'s own 23/23 tests pass —
`grep -c "^test trait_picker::tests::" <output>`; `race_trait_picker`'s
own 19/20 tests pass, the 1 failure being `race_trait_picker::...the_
menu_command_carries_all_fourteen_adopted_race_options_thirteen_with_
real_grants`, the identical pre-existing failure every prior
`AT-34-E4-002` cycle (3/4/5/6/9) has already attributed as pre-existing
and unrelated — `race_trait_picker.rs` is a file this cycle never
touched). TypeScript: `npx tsc
--noEmit` exits clean (no errors). Frontend: `node scripts/run-tests.mjs`
— **96/100 test files passed** (4 pre-existing failures — `raceCreation
Coverage.test.ts` (a corpus record-count fixture off by 9, unrelated to
traits), `buildVersionTriple.test.ts` ×2 and `buildLabelFixtureFreshness.
test.ts` (Cargo.toml/package.json version-stamp drift, `0.11.0` vs
`0.14.0` — a release-lane concern, not this epic's territory) — none
touching the 3 files this cycle changed; unchanged before/after this
cycle's own edits). `cargo test --locked --no-run` (full workspace,
widest build scope): run at this cycle's final HEAD `7714a6a5ef` in the
background (`timeout 580`) — **exit 0**. `apps/desktop/src-tauri --no-run`
(separate workspace, own `CARGO_TARGET_DIR`): **exit 0**.

**RED→GREEN evidence (TDD, §6 step 3).** Temporarily changed `ground_
orphan_trait_facts`'s new loop to iterate `caster_level_subschool_facts_
from_traits(&[])` (an always-empty slice) instead of the real `selected_
traits`, and re-ran the two most load-bearing new tests:
`eldritch_delver_caster_level_fact_reaches_the_real_explanations_vector`
FAILED (`left: None, right: Some(1)`) and `eldritch_delver_is_genuinely_
grounded_by_fixture_execution_on_both_pillars` FAILED (`left: None,
right: Some(2)`) — both RED for the intended reason (the standalone fact
genuinely not reaching the explanations vector, not a typo or missing
import). Reverted the one line; both GREEN again, confirmed by a second
full `trait_effects` run (70/70 filtered to that module).

## Sweep population

`corpus_literal_sweep --json-out` → `clean:true records_examined:48708`
(unchanged — no `data/corpus/**` file touched this cycle; `decisions.md
§12` L8 does not apply). `derived_evaluator_fixture_check --json-out` →
unchanged (`1839/2580, 0 failed`). **This cycle did not run the local
`docs/work-inventory.json` regeneration pipeline to completion** —
consistent with every prior cycle in this module, per this dispatch's
own file-ownership rule assigning that regeneration to the wave's shared
closing cycle. The functional `DONE 203→204, M 37→36` figure in this
receipt is therefore derived from the classify()-level bin test (which
ACTUALLY BUILDS the real `unit` and runs it through the real `classify()`
function, asserting the exact `grounded` status and evidence string), not
from an end-to-end whole-corpus regen diff — the same discipline cycle 9
used and reported as materially weaker than a full regen-and-restore
check, honestly repeated here.

## Oracle pin

Not applicable — no figure here came from the pinned PCGen oracle
checkout; every figure was derived from the live repo's `data/corpus/`
tree and this cycle's own executed fixture tests (`trait_effects.rs`'s
`eldritch_delver_caster_level_fact_reaches_the_real_explanations_vector`
and `caster_level_skill_trait_magnitude_is_grounded_for_corpus_key`,
which genuinely build fixture characters and run them through the real
`compute_pilot_base_chassis`/`skill_allocation::allocate_skill_ranks`
consumers; and `trait_picker.rs`'s own 18 new/updated tests, most of
which call the real `list_available_character_traits` and cross-check
against the real `trait_effects` compute functions).

- **Status:** partial

## Movement, four buckets (`decisions.md §9`)

- **Closure:** 1 unit in `ultimate_campaign` (`M → DONE`, functional —
  not yet in the committed inventory, per Sweep population above), via a
  real, fixture-executed caster-level-by-subschool standalone fact
  (mirroring `feat_effects::spell_focus_facts_from_choices`'s own
  per-school DC precedent) plus a new dedicated fold-in loop into the
  SAME `skill_allocation::allocate_skill_ranks` consumer the first three
  slices already established. Genuine compute-and-apply closure, not a
  relabelling: the record is re-verified by `caster_level_skill_trait_
  magnitude_is_grounded_for_corpus_key`, which builds a real character
  selecting exactly that trait and runs it through the real engine — and
  `Trait ~ Eldritch Delver` specifically is only reported grounded
  because BOTH its flat two-skill token AND its caster-level-by-
  subschool token fixture-execute correctly, never just one (the record
  was deliberately kept OUT of `FLAT_SKILL_TRAIT_BONUSES` for exactly
  this reason — see Notes).
- **Reclassification:** 0.
- **Reachability:** 0 net bucket movement from the picker-gap fix itself
  (all 7 traits it makes selectable were already `grounded` before this
  cycle) — but a real, load-bearing doctrine fix: a compute path with no
  UI reaching it is a no-stub violation per `docs/governance/no-stub-mvp-
  doctrine.md`, and this cycle closes it for all 7, not just its own new
  record. Reported separately from the 1 genuine `M → DONE` closure per
  this dispatch's own "never conflate a diagnostic fix with a bucket
  closure" discipline.
- **Instrument-correction:** 0.

## Notes

- **The picker gap was fixed FIRST, exactly as the dispatch brief
  required**, before any new bucket slice was attempted.
  `list_available_character_traits` now chains all 7 `trait_effects`
  tables (`FLAT_SKILL_TRAIT_BONUSES`, `SKILL_CHOICE_TRAIT_BONUSES`,
  `FAMILY_CHOICE_TRAIT_BONUSES`, `SAVE_TRAIT_BONUSES`, `SITUATIONAL_
  SKILL_TRAIT_BONUSES`, `INITIATIVE_TRAIT_BONUSES`/`CONCENTRATION_TRAIT_
  BONUSES` merged, `ABILITY_DIFF_SKILL_TRAIT_BONUSES`), plus this
  cycle's own new eighth (`CASTER_LEVEL_SKILL_TRAIT_BONUSES`) — 8 of 8.
  **Cycles 7 and 8's own receipts were wrong** when they stated "the
  existing trait picker already surfaces every selected trait
  generically" — that claim held only for a trait selected some other
  way (e.g. a saved-character round-trip already carrying the id), never
  for a brand-new character choosing among the picker's own rendered
  options, since the command itself never returned those 7 options at
  all. Retro-logged as a `resolution` against cycle 9's own `incident`
  (see command below) — cycle 9 found and logged the gap but, per its
  own scope discipline, did not fix it; this cycle does.
- **Eldritch Delver's flat-skill half is deliberately NOT a member of
  `FLAT_SKILL_TRAIT_BONUSES`.** `Kind::Trait`'s `classify()` rung is an
  `.or_else` chain that returns on the first `Some` — a record present in
  `FLAT_SKILL_TRAIT_BONUSES` would report `grounded` on its skill half
  alone, before its caster-level half is ever checked, exactly the
  "8 closures where measurement found 1" part-credit failure this
  bundle's own doctrine warns against. Verified directly: a dedicated
  test (`eldritch_delver_is_not_also_a_member_of_flat_skill_trait_
  bonuses`) asserts the record's `trait_id` is absent from that table.
- **`U(21), D(2), X(2)` were not touched, reopened, or reclassified.**
  Verified: the 1 record this cycle promotes is the only one referenced
  by any new code, and it does not appear in `CASTER_LEVEL_SKILL_TRAIT_
  BONUSES`'s own table alongside a `U`/`D`/`X` starting status (checked
  against the committed inventory's own per-key status).
- **The `ultimate_campaign/ability/eldritch_delver.json` duplicate corpus
  record was checked and confirmed out of scope, same as every prior
  cycle's own equivalent check.** `grep -rl "Trait ~ Eldritch Delver"`
  finds only `ultimate_campaign/trait_generic/trait_eldritch_delver.json`
  — no `ultimate_campaign/ability/` sibling exists for this particular
  record (unlike the fifth/sixth slices' traits), so there is no
  duplicate to check against at all.
- **No stubs.** The new standalone-fact producer reaches a real, executed
  consumer (`pilot_compute::ground_orphan_trait_facts`'s own
  `explanations` vector, the same channel initiative/concentration/
  situational already prove genuine), and the separate flat two-skill
  token reaches the real, pre-existing `skill_allocation::allocate_skill_
  ranks` consumer. The desktop picker was extended specifically so this
  cycle's own new record does not add an 8th instance of the exact gap
  this cycle just fixed for the 7 pre-existing ones.
- **`git status --porcelain` before every write; no `git add -A`; no `git
  stash`.** Two commits this cycle: the first (`c1cbfa0698`) staged and
  committed exactly the 3 files the picker-gap fix touched; the second
  (`7714a6a5ef`) staged and committed exactly the 6 files this cycle's
  own eighth-slice work touched. `docs/work-inventory.json` was never
  regenerated to completion this cycle, so there was nothing to `git
  restore` on it. Rebased once (`git fetch origin tranche/14 && git
  rebase origin/tranche/14`) between the two commits when a concurrent
  `AT-34-E3-002` cycle 9 push landed first — fast-forward, no conflict.
- Retro events: `python3 scripts/retro.py correction --subject
  "AT-34-E4-002_cycle_receipt_9.md next-cycle-plan" --claimed "Trait ~
  Eldritch Delver needs a per-subschool caster-level pillar this crate
  does not have" --actual "the shape already has a direct precedent
  (feat_effects::spell_focus_facts_from_choices); only the flat-skill
  half needed a new dedicated table; true for the OTHER 6 remaining
  records, not this one" --verified-by "cargo test --locked --lib --
  trait_effects (eldritch_delver_is_genuinely_grounded_by_fixture_
  execution_on_both_pillars, GREEN)"` (run with
  `RETRO_ACTOR=sd34-at-34-e4-002`); `python3 scripts/retro.py resolution
  --resolves 1788207553252-sd34-at-34-e4-002-cf2950 --how "Chained
  INITIATIVE_TRAIT_BONUSES/CONCENTRATION_TRAIT_BONUSES and ABILITY_DIFF_
  SKILL_TRAIT_BONUSES into list_available_character_traits, plus a
  corrected doc comment" --outcome "all 7 named traits now selectable
  through the desktop character creator; 18 new/updated tests prove
  reachability AND real compute-path round-trip"`.

## Next-cycle plan

The `ultimate_campaign` remainder is `M:36 U:21 X:2 D:2` = 61 non-DONE
(functional; committed inventory still reads `M:37` until the wave's
shared regen folds this cycle in), 6 in `trait_content`, 30 in
`ability_content`. Named by sub-cause, from this cycle's own
re-derivation against the live corpus JSON (not carried forward from
receipt 9's lumped characterization, which this cycle's own retro
`correction` found overstated for the record it closed):

- **3 `VAR`-only records**, genuinely three DIFFERENT-shaped problems,
  not one group:
  - `trait_fate_s_favored` (`VAR|Global_LuckBonus|1`) — needs a general
    "every luck bonus in play increases by 1" cross-cutting modifier;
    this crate tracks no bonus-type ledger to hook into at all.
  - `trait_loyalty_across_lifetimes` (`VAR|L_A_L_Eidolon|1`) — modifies
    an EIDOLON (a summoner's companion creature); this crate models no
    eidolon mechanic whatsoever, confirmed by a repo-wide grep for
    `eidolon` finding no compute path.
  - `trait_sacred_conduit` (7 `VAR` tokens: `OracleChannelDC`,
    `UndeadServitudeDC`, `ClericChannelPositiveEnergyDC`,
    `PaladinChannelPositiveEnergyDC`, `ClericChannelNegativeEnergyDC`,
    `PowerOverUndeadCommandUndeadDC`, `PowerOverUndeadTurnDC`) — this
    crate grounds a total for exactly ONE of the seven
    (`pilot_compute::oracle_channel_dc`, confirmed by that function's own
    doc comment naming the other five/six as having "no consumer here").
    A record leaves M only when EVERY token computes, so promoting this
    record needs SIX new DC totals built from scratch, not a small
    extension of the one that exists — genuinely the most expensive
    single remaining `trait_content` record, not "the natural next
    slice" as a same-shape-group label would suggest.
- **2 `ABILITYPOOL`-only records** (`trait_blood_of_dragons` — a
  three-way CHOOSE among a skill bonus, low-light vision, and a
  situational save bonus; `trait_deathtouched` — similarly heterogeneous)
  — needs a genuinely new player-choice mechanism where the options are
  DIFFERENT SHAPES (not the closed same-shape-skill-list pattern
  `SKILL_CHOICE_TRAIT_BONUSES`/`FAMILY_CHOICE_TRAIT_BONUSES` already
  established). This crate already grounds each of the three individual
  effect shapes elsewhere (skill bonus, low-light-vision standalone facts
  per race, situational save bonus is the same shape `BONUS:SITUATION`
  already covers for skills) — the missing piece is the CHOOSE-among-
  heterogeneous-options wiring itself, not any one effect shape.
- **1 corpus data gap** (`trait_shadow_whispers`) — unrelated to any
  compute path, not chased.
- **30 `ability_content` records** (18 Drawback incl. `default`, 10
  Retrain, 2 Retraining) — house rule bookkeeping / GM-adjudicated
  narrative penalties or a different mechanic entirely, out of scope per
  cycle 3's own direct reading, unchanged.

Re-run `python3 scripts/completion_atlas.py --book ultimate_campaign
--check` after each sub-wave; current functional remainder is
`M:36 U:21 X:2 D:2` summing to `61 of 265` non-DONE.
