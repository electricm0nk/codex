# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-002 (bucket C, "held and computed, never surfaced")

- **Commit SHA:** `a26d8b35d0` (source + tests, this cycle's own commit, rebased forward
  through several sibling-lane commits with no conflict and no further change to this cycle's
  own files — `git show --stat` confirms each rebase-picked-up commit touches only
  `apps/desktop/**`, `src/rules_core/trait_effects.rs`, `src/rules_core/skill_allocation.rs`,
  or `scripts/completion_atlas.py`'s own citation pins, never this cycle's own two files)
- **Files touched:** `src/rules_core/pilot_compute/mod.rs` (one new `pub fn`
  `generic_pool_group_selection_observed_keys`, reading the real corpus key a
  `push_generic_pool_group_selection_magnitude` explanation carries off its own `detail`
  field — no change to any existing function's behavior), `src/bin/v06_work_inventory.rs` (one
  new `EngineFacts` field `cleric_domain_generic_member_wired: BTreeSet<String>`; a new const
  `CORE_RULEBOOK_CLERIC_DOMAIN_ADJECTIVES` (33 real domain slugs); a new probe
  `probe_cleric_domain_generic_member_wiring`; one new grounding rung in `classify()`'s
  `Kind::ClassFeature` magnitude arm; 6 new tests — 2 classify()-level proofs (positive +
  negative control), 4 live-fixture probe proofs), this receipt,
  `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`.
  **`docs/work-inventory.json` and `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
  are deliberately NOT committed this cycle** — this dispatch's own file-ownership rule assigns
  their regeneration to the wave's single shared regeneration cycle (Wave 13 lost a lane to
  exactly this collision). Every figure below comes from a real, local, uncommitted regen run
  of this cycle's own source, restored (`git restore`) before each commit.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (`git diff --unified=0 -- src/bin/v06_work_inventory.rs
  src/rules_core/pilot_compute/mod.rs | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  — zero matches, run against this cycle's own working-tree diff before the first commit).
- **Wired-integration audit result:** OK_NO_TOKENS (same diff, same two files,
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` — zero matches).
- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "**370** units the engine holds and
  computes but never surfaces. **Evidence:** per unit, the explanation or display path that now
  carries it. A unit the player still cannot see is not cleared, whatever the engine holds."
  (370 is `epic-breakdown.md`'s own stale figure; re-derived fresh at this cycle's start SHA,
  `core_rulebook` bucket C is **351**, not 370 — carried from the prior cycle's own receipt,
  confirmed unchanged since, `decisions.md §12` L2.)
- **Status:** partial

## Population, re-derived (not quoted)

At this cycle's start (post-rebase HEAD, `core_rulebook` bucket C unchanged since the prior
`AT-34-E3-002` cycle closed): **351** of 6,701 (`python3 scripts/completion_atlas.py --book
core_rulebook --check`). All 351 carry evidence `no_explanation_id_and_no_diagnostic_names_
this_feature`, kind `class_feature` (confirmed by direct read of `docs/work-inventory.json`,
`decisions.md §12` L1).

**Not re-deriving the whole 351-unit partition from scratch** (the prior cycle's own table,
confirmed still accurate for every sub-cause this cycle did not touch) — this cycle targeted
the single largest named sub-cause, `domain_power_display_record_not_wired` (96 units), and
confirmed by direct corpus read that it decomposes into two shapes: 33 bare `"<Domain> Domain"`
header records (the domain-selection chassis record itself, carrying `DEFINE`/`BONUS:VAR`
tokens but no independent granted-power magnitude of its own) and 63 `"<Domain> Domain ~
<Power>"` member records (the domain's own granted power, a REAL, separate `.lst` line from the
`"Domain Power ~ <Power>"` records the prior cycle's own Ranger/Monk-adjacent `domain_power`
mechanism already covers — confirmed by direct JSON read of both, e.g.
`data/corpus/core_rulebook/class_feature/domain_power/strength_surge.json` line 739 vs.
`data/corpus/core_rulebook/class_feature/strength_domain/strength_surge.json` line 3259, two
genuinely different `.lst` records, not a duplicate to collapse).

## Mechanism found and closed this cycle: the generic pool-group-selection pass already covers Cleric Domain, classify() just never asked it

`push_generic_pool_group_selection_magnitude` (`src/rules_core/pilot_compute/mod.rs`, shipped
SD-32 T12 Epic 8) is a GENERIC, already-tested compute pass wired at six real pools — Cleric
Domain, Sorcerer Bloodline, Bloodrager Bloodline, Oracle Mystery, Warpriest Blessing, Shaman
Spirit — that resolves every real corpus `"<group> ~ <member>"` record's magnitude through the
shared PCGen formula-chain resolver once a player selects that group, with **zero** per-member
hand-written formulas. For Cleric Domain specifically it is wired unconditionally (any cleric,
any domain, `min_level: 1`) inside `explain_cleric_level1_spell_baseline`. Its own census test
(`pool_group_closure_census_across_all_six_pools`, re-run this cycle) confirms **47 of 72**
real Cleric Domain groups corpus-wide carry at least one resolvable member.

`v06_work_inventory`'s `classify()` had **never once asked this pass a question** — no probe,
no rung, no `EngineFacts` field referenced any of its output. Every domain member record kept
reading `engine-does-not-hold`, `no_explanation_id_and_no_diagnostic_names_this_feature`,
regardless of what the engine actually computed, because the classifier's only Domain-shaped
rung (`group == "Domain Power"`) matches a DIFFERENT corpus key shape (`"Domain Power ~
<Power>"`) that exists in the corpus ALONGSIDE, not instead of, the domain-prefixed shape this
cycle closes.

**The fix**, in two parts:

1. `generic_pool_group_selection_observed_keys` (`pilot_compute/mod.rs`), a new `pub fn`
   bridge in the exact same spirit as the pre-existing `domain_power_probe_catalog` bridge: it
   reads the real corpus key a matching `ComputationExplanation` carries off its own `detail`
   field (which embeds `` corpus key `<key>` `` verbatim — the pass's own pre-existing format
   string, never a new one) — so it can never invent a key the engine did not itself name.
2. `probe_cleric_domain_generic_member_wiring` (`v06_work_inventory.rs`), a new probe that
   selects each of the 33 real Core Rulebook Cleric Domain adjectives in turn
   (`CORE_RULEBOOK_CLERIC_DOMAIN_ADJECTIVES`, confirmed by direct corpus scan) on a real cleric
   fixture, over the same real `compute_pilot_base_chassis` pipeline every other probe in this
   file uses, and collects every corpus key the bridge function observed. One new `classify()`
   rung checks `facts.cleric_domain_generic_member_wired.contains(&unit.key)` directly — the
   set holds full `"<Domain> Domain ~ <Power>"` strings, so no group-prefix guard is needed:
   a false positive is structurally impossible, since the set is populated only from real
   engine `detail` strings, never from a static list.

## RED -> GREEN

RED (confirmed for the intended reason): temporarily changed the new rung's containment check
to look up a key no probe could ever produce (`format!("RED-CHECK-{}", &unit.key)`) and re-ran
`an_air_domain_power_record_the_probe_observed_reaches_grounded` — failed with
`left: "engine-does-not-hold", right: "grounded"` (the pre-existing fallthrough this cycle
closes), confirming the test fails because the fix is absent, not for an unrelated reason.
Restored the rung; the test (and all others) passes.

```
cargo test --locked --bin v06_work_inventory air_domain
running 3 tests
test class_feature_text_complete_rung_tests::an_air_domain_power_record_the_probe_observed_reaches_grounded ... ok
test class_feature_text_complete_rung_tests::an_air_domain_power_record_the_probe_never_observed_is_unaffected ... ok
test cleric_domain_generic_member_probe_tests::the_probe_observes_air_domain_lightning_arc_against_the_real_fixture ... ok
test result: ok. 3 passed; 0 failed
```

The live-fixture probe tests (`cleric_domain_generic_member_probe_tests`) run the REAL probe
against the REAL fixture and the REAL compute pipeline (no isolated `EngineFacts` mock):

```
cargo test --locked --bin v06_work_inventory cleric_domain_generic_member
running 4 tests
test cleric_domain_generic_member_probe_tests::the_probe_credits_healing_domain_rebuke_death_off_its_real_uses_per_day_chain ... ok
test cleric_domain_generic_member_probe_tests::the_probe_does_not_credit_an_unrelated_class_feature_record ... ok
test cleric_domain_generic_member_probe_tests::the_probe_observes_air_domain_lightning_arc_against_the_real_fixture ... ok
test cleric_domain_generic_member_probe_tests::print_the_real_observed_set_for_this_cycles_own_receipt ... ok
test result: ok. 4 passed; 0 failed
```

**Correction caught by this cycle's own live-fixture proof, not assumed** (retro `correction`
event logged, `RETRO_ACTOR=sd34-at-34-e3-002`): `domain_power.rs`'s own module doc names dice
notation (Healing Domain's Rebuke Death heal amount) as a refusal reason for the DIFFERENT,
narrower bespoke `DOMAIN_POWER_CATALOG` grammar. This cycle initially assumed the same refusal
applied to the generic resolver and wrote a negative-control test asserting Rebuke Death stays
uncredited — the live run disproved it: the generic pass resolves Rebuke Death's real
`RebukeDeathTimes` uses-per-day `BONUS:VAR` chain (the same `DomainPowerTimes|3+WIS` chain
every other domain power's own uses-per-day resolves through) independently of the separate,
still-unresolved dice-notation heal amount. Corrected to a positive assertion instead —
crediting a record for one genuinely-resolved independent terminal, while a DIFFERENT terminal
on the same record stays unresolved, is exactly `resolve_pool_member_all_magnitudes`'s own
documented, already-shipped, already-tested contract (see `all_magnitudes_resolves_every_
reachable_independent_terminal_on_a_multi_terminal_record`), not an over-credit this cycle
introduced.

Full `class_feature`-scoped suite: `cargo test --locked --bin v06_work_inventory class_feature`
— **134 passed, 0 failed**. Full bin suite (post-rebase, at this cycle's closing SHA):
**453 passed, 0 failed**. Pre-existing generic-pool-group tests
(`pilot_compute::generic_pool_group_selection_wiring_tests`, 30 tests) unaffected — this cycle
added a new *reader* of that pass's output, changing none of its own logic.

## Live regen (local, uncommitted — see file-ownership note above)

Regen required the sweep/fixture-check env vars, same guard prior cycles' own receipts
document. Ran `corpus_literal_sweep --json-out` (CLEAN, `48,708` records examined, `0` findings
— matching the unchanged baseline exactly, since this cycle touches no `data/corpus/**` file)
and `derived_evaluator_fixture_check --json-out` (`2,580` fixture rows, `1,839` cleared, `0`
failed — same unchanged baseline) first, then pointed `CORPUS_LITERAL_SWEEP_REPORT` /
`DERIVED_FIXTURE_CHECK_REPORT` at the two reports and regenerated (`--allow-stamp-loss` never
passed).

**Mixed disposition, confirmed by reading the diff rather than assumed**: of the 55 closed
units, **38** carry `wiring_class` `computed`/`derived` and land `grounded` -> bucket **DONE**
for real (e.g. `air_domain/lightning_arc.json`: `"wiring_class": "computed"`); the other **17**
carry `wiring_class: "static"` (confirmed by the corpus's own JSON field) and are correctly
upgraded by the pre-existing `apply_done_rung_stamps` machinery from `grounded` to
`literal-verified` -> bucket **V**, the same static/sweep-verification path the prior Monk
cycle's own 6 closures went through. Reported honestly per-record rather than assumed from the
fix's own return value.

**Isolation confirmed by a whole-inventory before/after diff keyed on unit id** (not sampled —
a real Python diff over both full 49,438-unit JSON documents):

```
$ python3 -c "... diff before_regen (git show HEAD:docs/work-inventory.json) vs after_regen (local uncommitted regen) ..."
before count: 49438 after count: 49438
added: 0 removed: 0
changed: 55
changed by book: {'core_rulebook': 55}
```

Every one of the 55 changed ids is `core_rulebook:class_feature:<domain>_<power>`, status
`engine-does-not-hold` -> (`grounded` x38 / `literal-verified` x17), evidence
`no_explanation_id_and_no_diagnostic_names_this_feature` ->
`generic_pool_group_selection_probe_observed_a_real_computed_magnitude` in every case. No other
book, no other kind, no other kind of status transition anywhere in the 49,438-unit corpus.

**No cross-book side effect:** every one of the observed corpus keys this cycle's probe can
possibly credit exists ONLY under `core_rulebook` in the entire 49,438-unit corpus — confirmed
by a direct scan of `docs/work-inventory.json` across all 37 books for each of the 56 corpus
keys the probe's own live-fixture test observed (`the_probe_observes_...` test's own printed
set), zero matches in any other book.

## Figures + their re-derive commands

| Figure | Value | Command | Denominator |
|---|---:|---|---|
| `core_rulebook` bucket C before | 351 | `python3 scripts/completion_atlas.py --book core_rulebook --check` at this cycle's start (post-rebase HEAD) | of 6,701 `core_rulebook` units |
| `core_rulebook` bucket C after | **296** | same command, post-regen (local, uncommitted) | of 6,701 |
| `core_rulebook` bucket DONE before/after | 4,344 / **4,382** | same command | of 6,701 (delta +38 — the 38 `computed`/`derived` closures) |
| `core_rulebook` bucket V before/after | 87 / **104** | same command | of 6,701 (delta +17 — the 17 `static` closures, sweep-verified) |
| `core_rulebook` buckets B/D/M/U/X/Z before/after | unchanged (470/366/958/10/115/0 both times) | same command | of 6,701 — confirms isolation: nothing outside C/DONE/V moved |
| Corpus-wide bucket C before | 4,332 | `python3 scripts/completion_atlas.py --check` | of 49,438 |
| Corpus-wide bucket C after | **4,277** | same command, post-regen | of 49,438 (delta -55, matching `core_rulebook`'s own delta exactly — confirmed no cross-book side effect) |
| Corpus-wide bucket DONE / V before/after | 24,314 / 24,352 (+38); 262 / 279 (+17) | same command | of 49,438 — both deltas match `core_rulebook`'s own exactly; every OTHER bucket (A/B/D/M/U/X/Z) unchanged corpus-wide |
| Whole-inventory before/after diff, keyed on unit `id` | 0 added, 0 removed, exactly 55 changed (all `core_rulebook`) | (Live regen section) | of 49,438 |
| Domain member bucket-C units this cycle's mechanism could reach | 63 | direct scan of `docs/work-inventory.json`, `core_rulebook` units whose `corpus_key` contains `Domain` and `" ~ "` | of 96 (`domain_power_display_record_not_wired`) |
| Probe's own real observed set (live-fixture test) | 56 | `cargo test --locked --bin v06_work_inventory cleric_domain_generic_member -- --nocapture` (`print_the_real_observed_set_for_this_cycles_own_receipt`) | of 63 |
| Of the 56 observed, already grounded via a DIFFERENT mechanism (bucket V, not C) before this cycle | 1 (`Liberation Domain ~ Liberation`, `literal-verified`/`class_feature_probe_observed_a_delta_attributable_to_this_record`) | direct `docs/work-inventory.json` read | of 56 |
| Net NEW closures this cycle | **55** | 56 observed − 1 already-elsewhere-grounded | of 63 |
| `completion_atlas.py --check` (corpus-wide, post-regen) | `population=49438 unclassified=0 overlap=0` | `python3 scripts/completion_atlas.py --check` | of 49,438 |
| `cargo test --locked --bin v06_work_inventory` (full, post-rebase) | `453 passed; 0 failed` | `cargo test --locked --bin v06_work_inventory` | of 453 |
| `cargo test --locked --bin v06_work_inventory class_feature` | `134 passed; 0 failed` | `cargo test --locked --bin v06_work_inventory class_feature` | of 134 |
| `cargo test --locked --no-run` (workspace) | exit 0, run at `8e69cdc4ff` (post-rebase) | `cargo test --locked --no-run` | — |
| `denominator_gate.py --check` | `files_checked=15 violations=8` — all 8 pre-existing/self-flagged verbatim-quoted corpus prose in `progress.md` ("75% chance..."), including this cycle's own denominator-gate paragraph's own quote (the same self-referential pattern every prior cycle's own paragraph hits) | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` | of 15 files |

## Row-count command output (this cycle's own artifact)

```
$ python3 scripts/completion_atlas.py --book core_rulebook --check
book=core_rulebook population=6701 unclassified=0 overlap=0
  DONE: 4382
  A: 0
  B: 470
  C: 296
  D: 366
  M: 958
  V: 104
  U: 10
  X: 115
  Z: 0
```

Bucket C: **296**, not zero. **Status: partial**, remainder named below (populations sum
exactly to 296).

## Build scope verified

`cargo test --locked --no-run` (workspace) exits **0**, run at post-rebase SHA `8e69cdc4ff`
(the rebase after this run only touched `apps/desktop/**` TypeScript files, confirmed by
`git show --stat` on every rebase-picked-up commit — no Rust source changed, so this result
remains valid). `cargo test --locked --bin v06_work_inventory` 453/453 pass. Desktop crate
(`apps/desktop/src-tauri`) not tested this cycle: no file under that tree, nor any file it
depends on, was touched by this cycle's own two-file diff (confirmed:
`grep -rl "generic_pool_group_selection_observed_keys\|cleric_domain_generic_member_wired" apps/`
— zero matches).

## Sweep population

`corpus_literal_sweep`: 48,708 examined, before and after — unchanged, since no
`data/corpus/**` file was added or regenerated this cycle.

## Oracle pin

N/A — no figure in this receipt came from the pinned PCGen oracle corpus.

## Movement, four buckets

**A bucket change is not a closure — reported honestly, split by real disposition, not by the
single "296 left bucket C" headline:**

- **Closure:** **38** — the 38 units whose own `wiring_class` is `computed`/`derived` moved
  `engine-does-not-hold` (bucket C) -> `grounded` (bucket **DONE**). Nothing remains for these.
- **Reclassification:** **17** — the 17 units whose own `wiring_class` is `static` moved bucket
  C -> bucket **V** (`grounded` upgraded to `literal-verified` by the pre-existing
  `apply_done_rung_stamps`, since `corpus_literal_sweep` independently byte-verified their
  `(book, file, line)`). Bucket V is not `DONE` — the same honest disposition the prior Monk
  cycle's own receipt already established for this exact shape.
- **Reachability:** **55** (one new grounding rung + one new probe now answer `grounded` for
  these exact corpus keys, reusing an explanation an already-shipped, already-tested generic
  compute pass genuinely emits — no new compute path, no new formula, for either the 38 or the
  17).
- **Instrument-correction:** 0 this cycle (the criterion's own stale "370" headline was already
  named and corrected by the prior cycle's own receipt).

**Bucket C's own delta (351 -> 296, -55) equals Closure + Reclassification (38 + 17 = 55)
exactly** — the row-count command's own output above is the ground truth this movement report
is checked against, not the other way around.

## Remainder — 296 of 351, named by sub-cause, populations sum exactly

Re-derived fresh at this cycle's own close (not carried forward from the prior cycle's own
351-unit table, per `decisions.md §12` L2 — every sub-cause this cycle did not touch is
confirmed UNCHANGED by direct re-scan, and `domain_power_display_record_not_wired` shrinks by
exactly this cycle's own 55 closures):

| Sub-cause | Population | Status / next step |
|---|---:|---|
| `bloodline_power_or_bloodline_feat_not_computed` (Sorcerer bloodline powers, all 10 bloodlines, plus `Sorcerer Bloodline Feat` and the 4 `Sorcerer Elemental Bloodline (<element>)` sub-choice records) | 77 | Unstarted. Unchanged from the prior cycle's own count. **Named next-cycle candidate** — Sorcerer Bloodline is one of the SAME six pools this cycle's mechanism already covers (census: 48/52 groups carry a resolvable member), so this cycle's own bridge function is directly reusable. |
| `monk_unarmed_damage_no_formula_in_engine` (Colossal/Diminutive/Fine/Gargantuan/Huge/Large/Tiny × 6 band-start levels) | 42 | Unchanged from the prior cycle. Genuine engine gap (no formula anywhere in the engine for these 7 sizes), confirmed by the prior cycle's own direct read of `monk_unarmed_strike_damage_die_for_size`. |
| `domain_power_display_record_not_wired` (Cleric/Druid domain, remainder after this cycle) | 41 | This cycle closed 55 of the original 96; 41 remain: **33** bare `"<Domain> Domain"` header records (the domain-selection chassis record itself — carries `DEFINE`/`BONUS:VAR` tokens but no independent granted-power magnitude the generic pass or any other mechanism currently names; a different, harder grounding question than a granted power), **7** `Druid Domain ~ <Element>` sub-choice records (a structurally different Druid-specific choice pool this cycle's Cleric-only probe deliberately did not sweep — named in this cycle's own deferral, not force-closed), **1** `Nobility Domain ~ Inspiring Word` (the sole domain member record with `magnitude_token_count: 0` — no BONUS:VAR chain for the generic resolver to reach at all). |
| `base_class_standalone_feature_not_computed` (bare class-name groups: Paladin, Wizard, Barbarian, Cleric, Druid, Ranger, Rogue, Sorcerer, Bard, Fighter, Alchemist, Monk) | 36 | Unchanged from the prior cycle. Unstarted. |
| `prestige_class_standalone_feature_not_computed` (Dragon Disciple, Arcane Archer, Assassin, Eldritch Knight, Shadowdancer, Arcane Trickster, Pathfinder Chronicler) | 31 | Unchanged from the prior cycle. Unstarted. |
| `other_named_group_or_standalone` (Loremaster, Monk Bonus Feat, Mystic Theurge, Ranger Combat Style, Arcane Bond, Basic Favored Enemy, Basic Favored Terrain, Cleric Extra Channel, Common Favored Terrain, Druid Wild Shape Progression, Druid Wild Shape Times, Duelist, Fighter Level Advanced Feat Tracker, Monk AC Tracker, Paladin Extra Channel, Rogue Archetype Support, Wizard / Remove Scribe Scroll) | 21 | Unchanged from the prior cycle. `Basic Favored Enemy`/`Basic Favored Terrain` remain the next-cheapest candidate the prior cycle already named (root DEFINE records the Favored Enemy/Terrain fix already proved wired) — still unverified. |
| `rage_power_not_computed` (Barbarian Rage Powers) | 13 | Unchanged. Unstarted. |
| `npc_class_standalone_feature_not_computed` (Adept, Aristocrat, Commoner, Expert, Warrior) | 10 | Unchanged. May resolve to a `D`/`X` reclassification on investigation, not `DONE`. |
| `rogue_talent_not_computed` (Rogue Talents) | 10 | Unchanged. Unstarted. |
| `versatile_performance_not_computed` (Bard Versatile Performance) | 9 | Unchanged. Unstarted. |
| `monk_unarmed_damage_small_cross_book_attribution_undecided` (Small band-start levels — real formula, only reachable via the Pathfinder Unchained book's Unchained Monk class) | 6 | Unchanged. The prior cycle deliberately did not decide this cross-book attribution question on its own authority; still open. |

**Sum check:** 77 + 42 + 41 + 36 + 31 + 21 + 13 + 10 + 10 + 9 + 6 = **296**, matching the
row-count command's own remainder exactly (351 total − 55 closed = 296).

## Notes

- **This cycle's fix is deliberately minimal and additive**: one new bridge `pub fn` (no change
  to any existing function's behavior), one new probe, one new `EngineFacts` field, one new
  `classify()` rung. Zero changes to any anti-fabrication, description-quality, or collision
  guard, and zero changes to `push_generic_pool_group_selection_magnitude` itself or any of its
  6 pre-existing pool wirings — this cycle only taught the CLASSIFIER to read output that
  compute pass was already producing.
- **Generic by construction, not by claim**: the bridge function
  (`generic_pool_group_selection_observed_keys`) takes an `id_prefix` parameter and works
  against ANY of the six pools' own explanations, not just Cleric Domain — a future cycle
  closing the `bloodline_power_or_bloodline_feat_not_computed` sub-cause (Sorcerer Bloodline,
  the next-largest named remainder) can reuse this exact bridge with a new probe and a new
  adjective/slug list, zero new bridge code required.
- **`decisions.md §16`** ("only the count grounds") checked against this fix: each domain
  member record is a flat granted-power magnitude (a fixed uses-per-day count or bonus once a
  domain is chosen), not a "pick N from an eligible set" shape — §16 does not apply.
- **Territory respected:** no `CharacterInput` field was added or changed; no trait/ability
  compute path was touched (confirmed: this cycle's own two files are
  `src/rules_core/pilot_compute/mod.rs` and `src/bin/v06_work_inventory.rs` only); the EQUIPMENT
  magnitude sub-causes (owned by a sibling lane) were not touched.
- **A correction this cycle's own live-fixture proof caught and fixed before it shipped**: see
  RED->GREEN section above (Healing Domain ~ Rebuke Death). Logged as a retro `correction` event.
- **A deferral this cycle named rather than force-closed**: 8 domain-shaped units (7 Druid
  Domain sub-choices + 1 zero-token Nobility record) — see remainder table and the retro
  `deferral` event.

## Next-cycle plan

1. `bloodline_power_or_bloodline_feat_not_computed` (largest remaining named sub-cause after
   this cycle) is the next-cheapest candidate for the EXACT SAME mechanism — Sorcerer Bloodline
   is one of the same six pools `push_generic_pool_group_selection_magnitude` already covers
   (census: 48/52 groups carry a resolvable member, the highest ratio of any of the six pools),
   and this cycle's own bridge function is already generic enough to reuse directly.
2. Investigate whether Druid's own domain sub-choice pool (7 units, named in this cycle's
   deferral) is wired through an analogous mechanism of its own.
3. Re-derive the remainder partition fresh before picking (`decisions.md §12` L2) — this
   receipt's own table is this cycle's fresh derivation, but the NEXT cycle must re-run it fresh
   again rather than trust this one.

