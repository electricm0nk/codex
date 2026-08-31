# Cycle 8 — Epic 4 (Ultimate Campaign to zero) / AT-34-E4-002

- **Commit SHA:** `9b0f287698` (code + this receipt); progress/kanban update follows in a second commit per §5
- **Provenance.** Dispatch brief (wave 20) named `AT-34-E4-002_cycle_receipt_7.md` as the newest
  receipt and instructed re-deriving the split fresh rather than trusting any quoted baseline.
  Worktree opened at a stale base (`ea2b3396f2`, the tranche cut) and was rebased onto
  `origin/tranche/14` `c320c61c4f` before any work began. Re-derived split at that HEAD:
  `python3 scripts/completion_atlas.py --book ultimate_campaign --check` read `DONE=196 M=44 D=2
  U=21 X=2`, population 265, unclassified 0 — exactly the dispatch brief's own stated baseline
  (the brief's figures were current this time; wave 19's shared regeneration, committed at
  `e234a221c7`, had already folded cycle 7's own 3-unit closure in before this cycle started).
- **Files touched:** `src/rules_core/trait_effects.rs` (+~250 net: new `TraitAbilityDiffSkillBonus`
  struct, `ABILITY_DIFF_SKILL_TRAIT_BONUSES` 4-entry table, `evaluate_ability_diff_formula`,
  `ability_diff_skill_bonuses_from_traits`, `ability_diff_skill_trait_magnitude_is_grounded_for_
  corpus_key`, module doc "Sixth slice" section + corrected "what this module does NOT cover"
  census, 9 new tests), `src/rules_core/skill_allocation.rs` (+11/-0: fourth `.or_else`-shaped
  fold-in loop in `allocate_skill_ranks`, reusing the SAME consumer the first three slices already
  established), `src/bin/v06_work_inventory.rs` (+~110 net: sixth `.or_else` classifier fallback
  onto `ability_diff_skill_trait_magnitude_is_grounded_for_corpus_key`, doc-comment update, 2 new
  positive-classifier tests — single-token Bruising Intellect and two-token-same-skill Precise
  Treatment — negative-control fixture corrected from `Trait ~ Bruising Intellect` (now covered)
  to `Trait ~ Fate's Favored`, a genuinely still-uncovered `BONUS:VAR`-only record), `scripts/
  completion_atlas.py` (+3/-4: instrument-correction, the bucket-V citation line pin re-derived
  after this cycle's own insertions into `v06_work_inventory.rs` shifted it, `13004 → 13017`, same
  precedent cycle 7 itself set for `12914 → 12924`).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` on this cycle's own diff (against this cycle's
  own starting HEAD `c320c61c4f`, scoped to the 4 files above — zero hits). Re-run at the
  workflow-instruction's own literal §6 formula (`BASE_BRANCH=$(git merge-base HEAD
  origin/develop)` = `ea2b3396f2`, i.e. the whole bundle's diff since the `tranche/14` cut across
  `src/rules_core/`/`src/bin/`) surfaces 20 pre-existing hits, all inside `src/rules_core/
  pilot_compute/class_feature_grant_consumer.rs` — a file this cycle never touched, confirmed by
  walking every hit back to its own `diff --git` header (`awk` grouping, not eyeballing). Identical
  finding to every prior `AT-34-E4-002` cycle's own re-run of this same wider formula.
- **Wired-integration audit result:** `OK_NO_TOKENS` on this cycle's own diff (same 4-file scope —
  zero hits). The wider whole-bundle-history formula surfaces the same pre-existing "placeholder"
  hits every prior cycle already found and explained (PCGen corpus token shapes: `%LIST`
  player-chosen-target placeholders, PI-redaction placeholder, CHOOSE-menu "no selection"
  placeholder rows) — none inside this cycle's own added text.
- **Acceptance criterion (verbatim, epic-breakdown.md AT-34-E4-002):** `python3
  scripts/completion_atlas.py --book ultimate_campaign --check` exits 0 with `DONE=265 of 265`,
  every other bucket zero, plus `artifacts/epic-4-ultimate-campaign/
  ultimate-campaign-completion-manifest.json`. **Not met this cycle** — real, incremental,
  fixture-verified progress on top of seven prior cycles: `DONE=200 of 265` (functional, this
  cycle's own local regen; was `196 of 265` at this cycle's start), remainder `M:40 U:21 X:2 D:2`
  = 65. The completion manifest artifact remains out of scope until every bucket clears.

## Figures + their re-derive commands

| Figure | Value | Command / denominator |
|---|---:|---|
| `ultimate_campaign` bucket split, re-derived at cycle start (this cycle's own rebase point) | `DONE=196, M=44 (trait 14 + ability 30), U=21, D=2, X=2, V=0` of 265 | `python3 scripts/completion_atlas.py --book ultimate_campaign --check` at `origin/tranche/14` HEAD `c320c61c4f` |
| **Correction of receipt_7's own "no formula evaluator exists in this crate" finding** | `formula_interpreter::PcgenFormulaEvaluator` already exists, already crate-wide (5+ existing consumers), never reached from `trait_effects.rs` | `grep -rn "PcgenFormulaEvaluator\|FormulaEvaluator\\b" src/ --include=*.rs \| grep -v pilot_compute/formula_interpreter.rs` — non-empty, `race_trait_formula_binding.rs`/`crb_untabled_class_chassis.rs`/`generic_class_chassis.rs`/`class_feature_grant_consumer.rs`/`pilot_compute/mod.rs`'s own Undine racial-trait formulas all call it |
| Ability variable naming convention confirmed against an existing crate-wide consumer (never guessed) | bare ability abbreviations (`INT`, `CHA`, `WIS`, …) bind to the character's real computed ability **modifier**, not raw score | direct read of `pilot_compute/mod.rs:11930-12020` (`explain_undine_formula_race_trait`, `vars.insert("CON".to_owned(), i64::from(ability_modifiers.constitution))`) |
| The 4 `ultimate_campaign` `trait_content` records whose remaining `BONUS:SKILL` magnitude is an ability-score-difference formula, re-confirmed against the live corpus JSON | `trait_bruising_intellect` (`SKILL\|Intimidate\|max(INT,CHA)-CHA`), `trait_planar_savant` (`SKILL\|Knowledge (Planes)\|max(INT,CHA)-INT`), `trait_pragmatic_activator` (`SKILL\|Use Magic Device\|max(INT,CHA)-CHA`), `trait_precise_treatment` (TWO tokens on the SAME skill: `SKILL\|Heal\|1\|TYPE=Trait` AND `SKILL\|Heal\|max(INT,WIS)-WIS`) | direct read of `data/corpus/ultimate_campaign/trait_generic/{trait_bruising_intellect,trait_planar_savant,trait_pragmatic_activator,trait_precise_treatment}.json`'s own `data.raw_tokens` |
| Units genuinely promoted M → DONE (`grounded`), this cycle | **4** in `ultimate_campaign` (`Trait ~ Bruising Intellect`, `Trait ~ Planar Savant`, `Trait ~ Pragmatic Activator`, `Trait ~ Precise Treatment`); 0 corpus-wide payoff elsewhere (checked: `grep -rl` for all 4 corpus `KEY` strings across `data/corpus/` finds them ONLY under `ultimate_campaign/trait_generic/` and `ultimate_campaign/ability/` — the latter is a DIFFERENT inventory `Kind` (`ability`, not `trait`), already `text-complete` before this cycle and untouched by this cycle's code, confirmed by `docs/work-inventory.json`'s own per-kind status breakdown, no other book carries these 4 `KEY`s at all) | id-set diff of `docs/work-inventory.json` before (committed HEAD) vs. after (this cycle's own local regen): `0 added, 0 removed` units; 4 changed `status`/`evidence`, all `ingested-magnitude → grounded`, `trait` kind only, zero `ability`-kind units touched |
| `ultimate_campaign` bucket state after this cycle (functional, local regen) | `DONE 196→200, M 44→40` (`trait` M `14→10`; `ability` M unchanged `30`), all other buckets unchanged (`D:2 U:21 X:2 V:0`) | `python3 scripts/completion_atlas.py --book ultimate_campaign --check` (post local regen, then `docs/work-inventory.json` restored to committed HEAD before this commit — regeneration is the wave's shared step) |
| `completion_atlas.py --check` corpus-wide (after this cycle's own local regen, before restore) | `population=49438 unclassified=0 overlap=0 done_evidence_violations=0 missing_clearing_mechanisms=0 citation_failures=0` (re-derived after fixing the shifted V-bucket pin, `13004→13017`) | `python3 scripts/completion_atlas.py --check` |
| `corpus_literal_sweep --json-out` | `clean:true records_examined:48708` — unchanged from cycle 7's own baseline, no `data/corpus/**` file touched this cycle | `corpus_literal_sweep --json-out <report>` |
| `derived_evaluator_fixture_check --json-out` | `1839 unit(s) cleared over 2580 fixture row(s); 0 failed; 0 not ingested` — unchanged | `derived_evaluator_fixture_check --json-out <report>` |
| Row-count command output (see below) | `4` distinct trait ids in the new table | see next section |

## Row-count command output

```
$ awk '/pub static ABILITY_DIFF_SKILL_TRAIT_BONUSES/,/^\];/' src/rules_core/trait_effects.rs \
    | grep -oE 'trait_id: "trait:[a-z_]+"' | sort -u
trait_id: "trait:trait_bruising_intellect"
trait_id: "trait:trait_planar_savant"
trait_id: "trait:trait_pragmatic_activator"
trait_id: "trait:trait_precise_treatment"
$ awk '/pub static ABILITY_DIFF_SKILL_TRAIT_BONUSES/,/^\];/' src/rules_core/trait_effects.rs \
    | grep -oE 'trait_id: "trait:[a-z_]+"' | sort -u | wc -l
4
```

This cycle's own artifact is the one new table; its 4 distinct `trait_id`s are exactly the
`ultimate_campaign` `M → DONE`-bucket delta this cycle claims (unlike Arcane Temper's dual-pillar
case, `trait_precise_treatment`'s two tokens share one pillar — the same skill — so they count as
one record, not two).

## Build scope verified

`cargo build --locked --lib`: exit 0 (only pre-existing warnings, no new ones from this cycle's
code). `cargo test --locked --lib -- trait_effects`: **49/49 passed** (9 new: table-shape checks,
no-selected-traits, the equal-modifiers-genuinely-zero case, the core single-token formula case,
the two-token-same-skill sum case, an unrecognized-trait-id case, the fixture-executed grounding
check for every entry including Precise Treatment's two-token case, the ungrounded-key negative
case, plus a cross-table collision check against all earlier pillar tables). `cargo test --locked
--lib -- skill_allocation`: **14/14 passed** (unchanged, confirms the fourth fold-in loop is
byte-identical for every existing fixture). `cargo test --locked --bin v06_work_inventory`:
**484/484 passed** (2 new positive classifier tests — single-token Bruising Intellect, two-token
Precise Treatment — plus the negative control corrected onto a genuinely still-uncovered fixture,
still green). `cargo test --locked --no-run`: full workspace **exit 0**, run at HEAD (post this
cycle's own commits, before any inventory regeneration — `decisions.md §12` L7). `apps/desktop/
src-tauri` (separate cargo workspace) **not run this cycle** — this cycle touched no file under
`apps/desktop/`, so per §2.5 it is honestly reported skipped.

**RED→GREEN evidence (TDD, §6 step 3).** Temporarily replaced `evaluate_ability_diff_formula`'s
real evaluator call with a hardcoded `None` and re-ran the two most load-bearing new tests:
`bruising_intellect_evaluates_the_real_formula_against_real_ability_modifiers` FAILED
(`left: None, right: Some(4)`) and `every_ability_diff_entry_is_genuinely_grounded_by_fixture_
execution` FAILED (`did not ground via real fixture execution`) — both RED for the intended
reason (the formula genuinely not evaluating, not a typo or missing import). Reverted the one
line; both GREEN again, confirmed by a second full `trait_effects` run (49/49).

**Out-of-territory finding, not fixed this cycle.** `cargo test` on `tests/v06_work_inventory.rs`
(the workspace integration suite) surfaced one failure —
`the_committed_inventory_is_well_formed_and_uses_only_declared_statuses`: `unit
"core_rulebook:class_feature:empty_selection_standard_barbarian" defers without naming the
diagnostic it quotes`. This reads the COMMITTED `docs/work-inventory.json` directly (unmodified by
this cycle, confirmed by `git status --porcelain` showing no diff on that file at any point this
cycle), concerns a `core_rulebook` `class_feature` record, and is unrelated to any trait/ability
compute path — outside this cycle's granted territory (`trait/ability compute paths and
CharacterInput`, explicitly not `core_rulebook` class-feature work another epic/lane owns).
Reported here rather than fixed or silently ignored; not part of this cycle's `M → DONE` claim.

## Sweep population

Three-pass pipeline, in order, **no `--allow-stamp-loss`** (local, uncommitted regen only — the
wave's shared closing cycle owns the committed `docs/work-inventory.json`):

1. `corpus_literal_sweep --json-out` → `clean:true records_examined:48708` (unchanged — no
   `data/corpus/**` file touched this cycle; `decisions.md §12` L8 does not apply).
2. `derived_evaluator_fixture_check --json-out` → `1839 unit(s) cleared over 2580 fixture row(s);
   0 failed` (unchanged).
3. `CORPUS_LITERAL_SWEEP_REPORT=... DERIVED_FIXTURE_CHECK_REPORT=... ./v06_work_inventory` → exit
   0, no stamp-loss refusal, `docs/work-inventory.json` regenerated with `0 added, 0 removed`
   units; exactly 4 changed (`status`/`evidence`), all `ultimate_campaign trait_content`. Restored
   to committed HEAD via `git restore docs/work-inventory.json
   docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` before this
   commit.

## Oracle pin

Not applicable — no figure here came from the pinned PCGen oracle checkout; every figure was
derived from the live repo's `data/corpus/` tree and this cycle's own executed fixture tests
(`trait_effects.rs`'s `every_ability_diff_entry_is_genuinely_grounded_by_fixture_execution`, which
genuinely builds fixture characters with asymmetric, hand-chosen ability scores and runs them
through the real `skill_allocation::allocate_skill_ranks` consumer).

- **Status:** partial

## Movement, four buckets (`decisions.md §9`)

- **Closure:** 4 units in `ultimate_campaign` (`M → DONE`, via a real, fixture-executed
  ability-score-difference formula compute path, reusing the SAME `skill_allocation::
  allocate_skill_ranks` consumer the first three slices already established — not a new
  wiring shape). Genuine compute-and-apply closure, not a relabelling: every entry is
  re-verified by `every_ability_diff_entry_is_genuinely_grounded_by_fixture_execution`, which
  builds a real character selecting exactly that trait with asymmetric ability scores and runs it
  through the real engine — and `Trait ~ Precise Treatment` specifically is only reported
  grounded because BOTH of its `BONUS:SKILL|Heal` tokens (the flat `+1` and the formula) are
  summed and applied, not just the formula half.
- **Reclassification:** 0.
- **Reachability:** 0 (this cycle builds the compute path itself — one new formula-evaluation
  producer, folded into an existing consumer — not a display/explanation wire onto an
  already-computed value).
- **Instrument-correction:** 1 (`completion_atlas.py`'s bucket-V citation line pin, shifted by
  this cycle's own insertions into `v06_work_inventory.rs`, `13004 → 13017`; no bucket population
  moved by this correction — same shape as cycle 7's own `12914 → 12924` correction).

## Notes

- **Receipt_7's own "no formula evaluator" finding was stale, and this cycle re-checked it
  before carrying it forward** (`decisions.md §12` L2, "never carry your own number forward" —
  applied here to a qualitative finding, not just a number). `formula_interpreter::
  PcgenFormulaEvaluator` is a real, already-proven recursive-descent evaluator for PCGen's
  `max`/`min`/arithmetic formula grammar, already wired into five other consumers crate-wide
  (`race_trait_formula_binding`, `crb_untabled_class_chassis`, `generic_class_chassis`,
  `class_feature_grant_consumer`, and `pilot_compute/mod.rs`'s own Undine racial-trait formulas)
  — it was simply never reached from `trait_effects.rs`. Filed as a `retro.py correction` below.
  This resolves the ENTIRE 4-record "ability-score-difference-formula" sub-cause receipt_7's own
  next-cycle plan named as future work "gated on... a formula evaluator this crate does not
  have" — the gate did not actually exist.
- **The ability-variable-naming convention (bare abbreviation = modifier, not raw score) was
  confirmed against an existing crate-wide consumer, not assumed.** `pilot_compute/mod.rs`'s own
  Undine racial-trait formula binding (`vars.insert("CON".to_owned(),
  i64::from(ability_modifiers.constitution))`) already establishes this exact convention; this
  cycle's `evaluate_ability_diff_formula` follows it byte-for-byte rather than inventing a
  parallel one.
- **Fixture ability scores are deliberately asymmetric, per record, and hand-derived
  independently of the evaluator under test** — never all-10 (which would make `max(A,B)-B`
  evaluate to 0 for every entry regardless of whether the formula genuinely ran, silently passing
  a broken evaluator the way `flat_skill_trait_magnitude_is_grounded_for_corpus_key`'s own all-10
  fixture correctly can for its shape, which carries no ability dependency at all). `Trait ~
  Planar Savant`'s fixture deliberately swaps which ability is high (CHA, not INT) to prove the
  `-INT` half of the shape is also genuinely evaluated, not just the `-CHA` half every other entry
  shares.
- **`Trait ~ Precise Treatment`'s two tokens share ONE pillar (the same skill, Heal), so they
  sum rather than needing `initiative_or_concentration_trait_magnitude_is_grounded_for_corpus_
  key`'s separate-pillars-must-all-ground discipline.** Both are genuinely applied: the fixture's
  expected value (`5`) is the formula's own `4` plus the flat token's own `1`, never just one.
- **Negative-control fixture corrected, retro-logged as part of the same finding.**
  `a_trait_outside_the_flat_slice_stays_ingested_magnitude` used `Trait ~ Bruising Intellect` as
  its example of an uncovered trait — this cycle's own sixth slice now covers it, so the test
  failed loudly on the first run after this cycle's own code landed (caught immediately, the same
  failure mode a negative control exists to catch, the same pattern cycle 6→7 already hit once
  with `Trait ~ Artisan`). Corrected to `Trait ~ Fate's Favored` (`BONUS:VAR`-only, no
  existing or near-term-planned compute path — a durable control).
- **`U(21), D(2), X(2)` were not touched, reopened, or reclassified.** Verified by the inventory
  diff: zero `ultimate_campaign` units with those starting statuses appear in the 4-unit changed
  set.
- **The `ultimate_campaign/ability/*.json` duplicate corpus records (same `KEY`, same source
  line, different directory) were checked and confirmed out of scope for this closure.** These
  242 files are ingested under a DIFFERENT inventory `Kind` (`ability`, "ability_content"), a
  wholly separate classifier code path in `v06_work_inventory.rs`'s `Kind::Ability` branch that
  this cycle's changes never touch. Direct read of `docs/work-inventory.json`'s per-kind status
  breakdown confirms all 4 of this cycle's targeted `KEY`s are already `text-complete` under
  `Kind::Ability`, unaffected by (and unrelated to) this cycle's `Kind::Trait` closure — the
  "corpus-wide, key on the KIND not the book" check the dispatch brief requires found these 4
  `KEY`s nowhere outside `ultimate_campaign`, in either directory.
- **No stubs.** The new formula-evaluation producer is a real, fixture-executed compute path
  reaching a real, pre-existing consumer (`skill_allocation::allocate_skill_ranks`'s own
  `trait_skill_bonuses` fold) — the same idiom every earlier slice in this module established. No
  desktop UI change was needed this cycle: the existing trait picker already surfaces every
  selected trait generically, and the skill sheet already renders `misc_modifier` for every skill
  (proven by the three earlier flat/choice-slice cycles' own desktop wiring), so a genuinely
  computed skill bonus from this producer reaches the player through the same rendered path
  without a new UI touch.
- **`git status --porcelain` before every write; no `git add -A`; no `git stash`.** Explicit
  `git add` of the 5 touched files only (4 code/instrument files + this receipt).
  `docs/work-inventory.json` and `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/
  completion-atlas.json` were `git restore`d before this commit.

## Next-cycle plan

The `ultimate_campaign` remainder is `M:40 U:21 X:2 D:2` = 65 non-DONE, 10 in `trait_content`
(0 remaining in `ability_content` beyond the pre-existing 30 Drawback/Retrain records already
named out of scope by cycle 3's own direct reading), 30 in `ability_content`. Named by
sub-cause, from cycle 7's own fresh census, unchanged by this cycle (this cycle closed the
ability-formula sub-cause entirely, touching none of the remaining 10):

- **3 `VAR`-only records** (`trait_fate_s_favored`, `trait_loyalty_across_lifetimes`,
  `trait_sacred_conduit` — the last carries 7 VAR tokens, all channel-energy-DC variables) —
  needs a bonus-pool/DC-variable compute pillar this crate does not have.
- **3 `SITUATION`-only records** (`trait_almost_human`, `trait_self_taught_scholar`,
  `trait_trustworthy` — the last also carries a flat `SKILL|Diplomacy|1` token mixed with its
  `SITUATION` Bluff bonus, the same "don't over-claim a partial cover" hazard Precise Treatment's
  own two-token record demonstrated is tractable once both halves are modeled) — needs a
  conditional-situational-check pillar.
- **2 `ABILITYPOOL`-only records** (`trait_blood_of_dragons`, `trait_deathtouched` — both a
  player CHOOSE among several distinct effect types, not a flat magnitude) — needs a bonus
  trait-slot pool mechanic.
- **1 mixed `CASTERLEVEL`+`SKILL` record** (`trait_eldritch_delver` — its `SKILL` half is
  trivially coverable via the existing flat-skill table, but its
  `CASTERLEVEL|SUBSCHOOL.Teleportation` half needs a per-subschool caster-level pillar this crate
  does not have; covering only the `SKILL` half would understate the record, the same discipline
  that kept `trait_precise_treatment` out of scope until this cycle covered both its halves).
- **1 corpus data gap** (`trait_shadow_whispers`) — unrelated to any compute path, not chased.
- **30 `ability_content` records** (18 Drawback incl. `default`, 10 Retrain, 2 Retraining) —
  house rule bookkeeping / GM-adjudicated narrative penalties or a different mechanic entirely
  (character-progression rebuild), out of scope per cycle 3's own direct reading, unchanged.

Cheapest next slice: **`trait_eldritch_delver`'s `SKILL` half** could be folded into the existing
flat-skill table today (its formula-shaped `CASTERLEVEL` half stays uncovered, so the record as a
whole stays `M` per the "every token must compute" discipline) — not a genuine unit closure by
itself, so not attempted this cycle; the next cycle should instead evaluate whether the
`SITUATION`-only shape (3 records) or the `VAR`-only shape (3 records) is cheaper to build a
pillar for, since both are now the largest remaining same-shape groups. Re-run `python3
scripts/completion_atlas.py --book ultimate_campaign --check` after each sub-wave; current
remainder is `M:40 U:21 X:2 D:2` summing to `65 of 265` non-DONE.
