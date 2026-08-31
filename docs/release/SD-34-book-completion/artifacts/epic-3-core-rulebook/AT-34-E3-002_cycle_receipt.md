# Cycle 4 — Epic 3 (Core Rulebook to zero) / AT-34-E3-002 (bucket C, "held and computed, never surfaced")

- **Commit SHA:** this cycle's own commit (source + tests + `scripts/completion_atlas.py`
  citation re-pin), rebased onto `origin/tranche/14` tip `1e6a67390e` with no conflict.
- **Files touched:** `src/bin/v06_work_inventory.rs` (one new `EngineFacts` field
  `sorcerer_bloodline_generic_member_wired: BTreeSet<String>`; a new const
  `CORE_RULEBOOK_SORCERER_BLOODLINE_ADJECTIVES` (10 real bloodline slugs); a new probe
  `probe_sorcerer_bloodline_generic_member_wiring`; one new grounding rung in `classify()`'s
  `Kind::ClassFeature` magnitude arm, explicitly book-scoped to `core_rulebook`; 4 new probe
  tests + 3 new classify()-level tests, RED confirmed then GREEN), `scripts/completion_atlas.py`
  (10 citation line pins re-derived after this cycle's own six pure-insertion hunks shifted
  every one — `diff -u0` hunk headers, summed cumulatively, confirmed by reading each new
  line's real content), this receipt, `docs/release/SD-34-book-completion/progress.md`,
  `docs/release/SD-34-book-completion/kanban.md`.
  **`docs/work-inventory.json` and
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` are
  deliberately NOT committed this cycle** — this dispatch's own file-ownership rule assigns
  their regeneration to the wave's single shared regeneration cycle (wave 13 lost a lane to
  exactly this collision). Every figure below comes from a real, local, uncommitted regen run
  of this cycle's own source, restored (`git restore`) before this commit.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (`git diff --unified=0 -- src/bin/v06_work_inventory.rs
  scripts/completion_atlas.py | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — zero
  matches, run against this cycle's own working-tree diff before the first commit).
- **Wired-integration audit result:** OK_NO_TOKENS (same diff, same two files,
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` — zero matches).
- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "**370** units the engine holds and
  computes but never surfaces. **Evidence:** per unit, the explanation or display path that now
  carries it. A unit the player still cannot see is not cleared, whatever the engine holds."
  (370 is `epic-breakdown.md`'s own stale figure; re-derived fresh at this cycle's start,
  `core_rulebook` bucket C was **296**, not 370 — `decisions.md §12` L2, and the criterion's own
  "one wiring gap repeated 357 times" premise was already retired by wave 16/17: it is a residue
  of distinct shapes, not one mechanism.)
- **Status:** partial

## Population, re-derived (not quoted)

At this cycle's start (post-rebase HEAD `1e6a67390e`, `core_rulebook` bucket C unchanged since
the prior `AT-34-E3-002` cycle closed): **296** of 6,701
(`python3 scripts/completion_atlas.py --book core_rulebook --check`).

**Not re-deriving the whole 296-unit partition from scratch** (the prior cycle's own table,
confirmed still accurate by this cycle's own isolation proof below for every sub-cause this
cycle did not touch) — this cycle targeted the prior cycle's own named next-cheapest candidate,
`bloodline_power_or_bloodline_feat_not_computed` (77 units), and re-confirmed its exact
77-unit membership by direct corpus read before building anything:
- **70** real `"<Bloodline> Bloodline ~ <Power>"` pool-member records across the ten PF1 Core
  Rulebook Sorcerer bloodlines (Aberrant, Abyssal, Arcane, Celestial, Destined, Draconic,
  Elemental, Fey, Infernal, Undead).
- **3** `"Sorcerer Bloodline Feat ~ <X>"` records (Deadly Aim, Toughness, Weapon Focus) — corpus
  keys `ground_sorcerer_bloodline_feat_pool`'s own doc comment already names as failing the
  eligible-feat owner match at a different corpus-key group entirely (out of that mechanism's
  scope, unaffected by it).
- **4** `"Sorcerer Elemental Bloodline (<Air|Earth|Fire|Water>)"` records — a structurally
  different, `" ~ "`-free, `static`-wiring-class flag record (`BONUS:VAR|...|1`, the player's
  own elemental-choice commitment), not a pool member at all.

70 + 3 + 4 = 77, matching the prior cycle's own count exactly.

## Mechanism reused this cycle: the SAME generic pool-group-selection pass, for its SECOND real pool

`push_generic_pool_group_selection_magnitude` (`src/rules_core/pilot_compute/mod.rs`, shipped
SD-32 T12 Epic 8) is wired unconditionally for **Sorcerer Bloodline** (any sorcerer, any
bloodline, `min_level: 1`), exactly as the prior cycle's own Cleric Domain fix used it for
Cleric Domain — one of the same six real pools this pass has covered since SD-32. Its own
call site (`push_generic_pool_group_selection_magnitude(..., SORCERER_BLOODLINE_CHOICE_ID,
"Sorcerer", "Bloodline", "bloodline:", "class_feature.sorcerer.bloodline.generic", 1, ...)`,
inside `explain_sorcerer_level1_chassis`'s own bloodline branch) already resolves every real
corpus `"<Bloodline> Bloodline ~ <Power>"` record's magnitude through the shared PCGen
formula-chain resolver once a player selects that bloodline, purely additively alongside the
hand-modelled Arcane/Draconic branches — with **zero** per-member hand-written formulas.

`v06_work_inventory`'s `classify()` had never once asked it a Sorcerer-Bloodline-shaped
question, for the identical reason the Cleric Domain fix named: the canonical per-class sweep
only ever seeds `"bloodline:arcane"` for sorcerer (`canonical_seeds_for`'s single seed), so it
alone can never observe any other bloodline's own generic-pass explanations.

**The fix**, mirroring the prior cycle's own two-part bridge exactly:

1. `probe_sorcerer_bloodline_generic_member_wiring` (new), a probe in the exact same spirit as
   `probe_cleric_domain_generic_member_wiring`: selects each of the 10 real Core Rulebook
   bloodline adjectives in turn on a real sorcerer fixture, over the same real
   `compute_pilot_base_chassis` pipeline every other probe in this file uses, and reads the
   real corpus key a matching explanation carries off its own `detail` field via the
   **pre-existing, unmodified** `generic_pool_group_selection_observed_keys` bridge — the same
   bridge function the prior cycle proved generic and reusable, reused here with zero new
   bridge code, exactly as that cycle's own "next-cycle plan" named.
2. One new `classify()` rung checking `facts.sorcerer_bloodline_generic_member_wired.contains(&unit.key)`
   — **explicitly book-scoped to `unit.book == "core_rulebook"`**, unlike the Cleric Domain
   rung (see collision finding below).

## A real corpus-key collision this cycle's own scan found (the Cleric Domain cycle's did not)

The probe's real observed set (57 keys) contains three strings — `"Draconic Bloodline ~
Bloodline"`... specifically `"Draconic Bloodline ~ Bloodline"`(sic, see exact list below) —
that also exist as **`advanced_class_guide`/`ultimate_magic`** corpus keys for an unrelated
Bloodrager/crossblood-archetype mechanism, confirmed by direct `docs/work-inventory.json` read:
zero `core_rulebook` unit carries any of the three. A shared name is not a duplicate
(`decisions.md`'s supersession-register precedent) — these are genuinely different records that
happen to share a string, and `class_feature_record_tokens_pre_gate_safe()`'s own flat,
book-agnostic keying means the ALREADY-SHIPPED engine resolver has resolved this ambiguity one
way or another since SD-32, independent of this cycle. This cycle's own closure claim must stay
provably isolated to its own named `core_rulebook` population regardless, so the new rung is
gated on `unit.book == "core_rulebook"` — a real, tested guard (`a_same_named_record_in_a_
different_book_is_not_credited`), not merely observed-and-ignored. The Cleric Domain cycle's own
receipt proved zero collisions in ITS observed set by scan; this cycle's scan found three, so the
gate is explicit in code here rather than merely asserted in prose.

The three collision keys (all `advanced_class_guide`/`ultimate_magic`, confirmed `engine-does-
not-hold` in both books both before and after this cycle's regen — untouched):
`Draconic Bloodline ~ Bloodrager`, `Draconic Bloodline ~ Crossblooded`, `Draconic Bloodline ~
Crossblooded Rager`.

## RED -> GREEN

RED (confirmed for the intended reason): temporarily changed the new rung's containment check to
look up a key no probe could ever produce (`format!("RED-CHECK-{}", &unit.key)`) and re-ran
`an_aberrant_bloodline_power_record_the_probe_observed_reaches_grounded` — failed with
`left: "engine-does-not-hold", right: "grounded"` (the pre-existing fallthrough this cycle
closes), confirming the test fails because the fix is absent, not for an unrelated reason.
Restored the rung; the test (and all others) passes.

```
$ cargo test --locked --bin v06_work_inventory an_aberrant_bloodline_power_record_the_probe_observed_reaches_grounded
running 1 test
test class_feature_text_complete_rung_tests::an_aberrant_bloodline_power_record_the_probe_observed_reaches_grounded ... FAILED
left: "engine-does-not-hold"
right: "grounded"
```

After restoring the rung:

```
$ cargo test --locked --bin v06_work_inventory bloodline
running 6 tests
test class_feature_text_complete_rung_tests::an_aberrant_bloodline_power_record_the_probe_never_observed_is_unaffected ... ok
test class_feature_text_complete_rung_tests::an_aberrant_bloodline_power_record_the_probe_observed_reaches_grounded ... ok
test sorcerer_bloodline_generic_member_probe_tests::the_probe_observes_a_real_arcane_bloodline_member_alongside_the_other_nine ... ok
test sorcerer_bloodline_generic_member_probe_tests::the_probe_does_not_credit_an_unrelated_class_feature_record ... ok
test sorcerer_bloodline_generic_member_probe_tests::print_the_real_observed_set_for_this_cycles_own_receipt ... ok
test sorcerer_bloodline_generic_member_probe_tests::the_probe_observes_aberrant_bloodline_acidic_ray_against_the_real_fixture ... ok
test result: ok. 6 passed; 0 failed

$ cargo test --locked --bin v06_work_inventory a_same_named_record_in_a_different_book_is_not_credited
test class_feature_text_complete_rung_tests::a_same_named_record_in_a_different_book_is_not_credited ... ok
test result: ok. 1 passed; 0 failed
```

Full `class_feature`-scoped suite: `cargo test --locked --bin v06_work_inventory class_feature`
— **138 passed, 0 failed** (up from 134 pre-cycle). Full bin suite: **461 passed, 0 failed**
(up from 453 pre-cycle — the 8 new tests this cycle added).

## Live regen (local, uncommitted — see file-ownership note above)

Same guard the prior cycle's own receipt documents. Ran `corpus_literal_sweep --json-out`
(CLEAN, `48,708` records examined, `0` findings — matching the unchanged baseline exactly,
since this cycle touches no `data/corpus/**` file) and `derived_evaluator_fixture_check
--json-out` (`2,580` fixture rows, `1,839` cleared, `0` failed — same unchanged baseline)
first, then pointed `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` at the two
reports and regenerated (`--allow-stamp-loss` never passed).

**Mixed disposition, confirmed by reading the diff rather than assumed**: of the 54 closed
units, **44** carry `wiring_class` `computed`/`derived` and land `grounded` -> bucket **DONE**
for real; the other **10** carry `wiring_class: "static"` (confirmed by the corpus's own JSON
field: the 10 `"<Bloodline> Bloodline ~ Feat Tracker"` records plus `Elemental Bloodline ~
Elemental Movement (Air)`) and are correctly upgraded by the pre-existing
`apply_done_rung_stamps` machinery from `grounded` to `literal-verified` -> bucket **V**, the
same static/sweep-verification path the prior Cleric Domain cycle's own 17 closures (and the
earlier Monk cycle's own 6) went through.

**Isolation confirmed by a whole-inventory before/after diff keyed on unit id** (not sampled —
a real Python diff over both full 49,438-unit JSON documents):

```
before count: 49438 after count: 49438
added: 0 removed: 0
changed: 54
changed by book: {'core_rulebook': 54}
44 ('engine-does-not-hold', 'grounded', 'generic_pool_group_selection_probe_observed_a_real_computed_magnitude')
10 ('engine-does-not-hold', 'literal-verified', 'generic_pool_group_selection_probe_observed_a_real_computed_magnitude')
```

Every one of the 54 changed ids is `core_rulebook:class_feature:<bloodline>_<power>`, status
`engine-does-not-hold` -> (`grounded` x44 / `literal-verified` x10), evidence
`no_explanation_id_and_no_diagnostic_names_this_feature` ->
`generic_pool_group_selection_probe_observed_a_real_computed_magnitude` in every case. No other
book, no other kind, no other kind of status transition anywhere in the 49,438-unit corpus —
the book-scope guard in the code (see collision finding above) is what makes this true, not
coincidence: the probe's raw observed set carried 57 keys, 3 of which belong to other books and
are excluded by that guard (57 − 3 = 54, matching exactly).

**Corpus-wide isolation** (same regen, same before/after documents):

```
before: DONE=24353 A=449 B=11769 C=4277 D=2955 M=4965 U=202 V=279 X=170 Z=19
after:  DONE=24397 A=449 B=11769 C=4223 D=2955 M=4965 U=202 V=289 X=170 Z=19
```

DONE +44, C −54, V +10 — every other bucket unchanged, matching `core_rulebook`'s own deltas
exactly, corpus-wide.

## Figures + their re-derive commands

| Figure | Value | Command | Denominator |
|---|---:|---|---|
| `core_rulebook` bucket C before | 296 | `python3 scripts/completion_atlas.py --book core_rulebook --check` at this cycle's start (post-rebase HEAD `1e6a67390e`) | of 6,701 `core_rulebook` units |
| `core_rulebook` bucket C after | **242** | same command, post-regen (local, uncommitted) | of 6,701 |
| `core_rulebook` bucket DONE before/after | 4,383 / **4,427** | same command | of 6,701 (delta +44 — the 44 `computed`/`derived` closures) |
| `core_rulebook` bucket V before/after | 104 / **114** | same command | of 6,701 (delta +10 — the 10 `static` closures, sweep-verified) |
| `core_rulebook` buckets A/B/D/M/U/X/Z before/after | unchanged (0/470/366/957/10/115/0 both times) | same command | of 6,701 — confirms isolation: nothing outside C/DONE/V moved |
| Corpus-wide bucket C before/after | 4,277 / **4,223** | `python3 scripts/completion_atlas.py --check` | of 49,438 (delta −54, matching `core_rulebook`'s own delta exactly) |
| Corpus-wide bucket DONE / V before/after | 24,353 / 24,397 (+44); 279 / 289 (+10) | same command | of 49,438 — both deltas match `core_rulebook`'s own exactly; every OTHER bucket (A/B/D/M/U/X/Z) unchanged corpus-wide |
| Whole-inventory before/after diff, keyed on unit `id` | 0 added, 0 removed, exactly 54 changed (all `core_rulebook`) | (Live regen section) | of 49,438 |
| `bloodline_power_or_bloodline_feat_not_computed` sub-cause, this cycle's own re-confirmed membership | 77 (70 pool members + 3 feat-pool stragglers + 4 elemental-choice flags) | direct `docs/work-inventory.json` read, bucket-C units matching the 10 canonical bloodline group prefixes / `Sorcerer Bloodline Feat ~` / `Sorcerer Elemental Bloodline (` | of 296 |
| Probe's own real observed set (live-fixture test) | 57 | `cargo test --locked --bin v06_work_inventory sorcerer_bloodline_generic_member_probe_tests::print_the_real_observed_set -- --nocapture` | of 70 pool-member candidates |
| Of the 57 observed, cross-book collisions excluded by the `core_rulebook` guard | 3 | direct `docs/work-inventory.json` read (`advanced_class_guide`/`ultimate_magic` own the keys) | of 57 |
| Net NEW closures this cycle | **54** | 57 observed − 3 excluded collisions | of 70 |
| `completion_atlas.py --check` (corpus-wide, post-regen) | `population=49438 unclassified=0 overlap=0` | `python3 scripts/completion_atlas.py --check` | of 49,438 |
| `completion_atlas.py --check` `citation_failures` | 0 (10→0, re-derived this cycle after 6 pure-insertion hunks shifted every pin) | `python3 scripts/completion_atlas.py --check` | of 10 citations |
| `cargo test --locked --bin v06_work_inventory` (full) | `461 passed; 0 failed` | `cargo test --locked --bin v06_work_inventory` | of 461 |
| `cargo test --locked --bin v06_work_inventory class_feature` | `138 passed; 0 failed` | `cargo test --locked --bin v06_work_inventory class_feature` | of 138 |
| `cargo test --locked --no-run` (workspace) | exit 0 | `cargo test --locked --no-run` | — |

## Row-count command output (this cycle's own artifact)

```
$ python3 scripts/completion_atlas.py --book core_rulebook --check
book=core_rulebook population=6701 unclassified=0 overlap=0
  DONE: 4427
  A: 0
  B: 470
  C: 242
  D: 366
  M: 957
  V: 114
  U: 10
  X: 115
  Z: 0
```

Bucket C: **242**, not zero. **Status: partial**, remainder named below (populations sum
exactly to 242). This live command output was produced by the local, uncommitted regen and is
NOT reflected in the currently-committed `docs/work-inventory.json` (restored via `git restore`
before this commit, per the file-ownership rule) — the committed inventory still reads C=296
until the wave's shared regeneration cycle re-runs the pipeline against this cycle's own
committed source.

## Build scope verified

`cargo test --locked --no-run` (workspace) exits **0**. `cargo test --locked --bin
v06_work_inventory` 461/461 pass. Desktop crate (`apps/desktop/src-tauri`) not tested this
cycle: no file under that tree, nor any file it depends on, was touched by this cycle's own
two-file diff (confirmed: `grep -rl "sorcerer_bloodline_generic_member\|CORE_RULEBOOK_SORCERER_BLOODLINE" apps/`
— zero matches).

## Sweep population

`corpus_literal_sweep`: 48,708 examined, before and after — unchanged, since no
`data/corpus/**` file was added or regenerated this cycle.

## Oracle pin

N/A — no figure in this receipt came from the pinned PCGen oracle corpus.

## Movement, four buckets

- **Closure:** **44** — the 44 units whose own `wiring_class` is `computed`/`derived` moved
  `engine-does-not-hold` (bucket C) -> `grounded` (bucket **DONE**). Nothing remains for these.
- **Reclassification:** **10** — the 10 units whose own `wiring_class` is `static` moved bucket
  C -> bucket **V** (`grounded` upgraded to `literal-verified` by the pre-existing
  `apply_done_rung_stamps`, since `corpus_literal_sweep` independently byte-verified their
  `(book, file, line)`). Bucket V is not `DONE` — the same honest disposition the Cleric Domain
  and Monk cycles' own receipts already established for this exact shape.
- **Reachability:** **54** (one new grounding rung + one new probe now answer `grounded` for
  these exact corpus keys, reusing an explanation an already-shipped, already-tested generic
  compute pass genuinely emits — no new compute path, no new formula, for either the 44 or the
  10).
- **Instrument-correction:** 0 this cycle.

**Bucket C's own delta (296 -> 242, −54) equals Closure + Reclassification (44 + 10 = 54)
exactly** — the row-count command's own output above is the ground truth this movement report
is checked against, not the other way around.

## Remainder — 242 of 296, named by sub-cause, populations sum exactly

Re-derived fresh at this cycle's own close (`decisions.md §12` L2) — every sub-cause this cycle
did not touch is confirmed UNCHANGED by direct proof (the whole-inventory diff above shows
**zero** changes outside the bloodline sub-cause, so restating these figures is evidence-backed,
not carried-forward-by-assumption):

| Sub-cause | Population | Status / next step |
|---|---:|---|
| `bloodline_power_or_bloodline_feat_not_computed` (remainder after this cycle) | **23** | This cycle closed 54 of 77. 23 remain, three real shapes: **16** pool-member records the generic pass's own formula chain genuinely does not reach (`Abyssal Bloodline ~ Added Summonings`, `Arcane Bloodline ~ School Power`/`School Power Choice`, `Draconic`/`Fey`/`Infernal Bloodline ~ Bloodline Arcana`, `Elemental Bloodline ~ Elemental Body`/`Elemental Movement`/`Elemental Movement (Earth\|Fire\|Water)`/`Elemental Resistance`, `Fey Bloodline ~ Fey Magic`/`Laughing Touch`/`Woodland Stride`, `Undead Bloodline ~ One of Us` — the same "generic pass resolves most, not all, members" shape the Cleric Domain cycle's own 8-unit deferral already established, named next-candidate: investigate why `Elemental Movement (Air)` resolved but Earth/Fire/Water did not — likely a single-element-default gap in how the probe seeds the elemental sub-choice, itself possibly reusable); **3** `Sorcerer Bloodline Feat ~ {Deadly Aim, Toughness, Weapon Focus}` (confirmed out of the feat-pool catalog's own scope by that mechanism's own doc comment — a corpus-naming quirk, not a formula gap; unstarted); **4** `Sorcerer Elemental Bloodline (Air\|Earth\|Fire\|Water)` (a different, `" ~ "`-free corpus shape — a static player-choice-commitment flag, not a pool member; unstarted, needs its own mechanism). |
| `monk_unarmed_damage_no_formula_in_engine` | 42 | Unchanged (confirmed by the isolation diff — zero Monk records changed this cycle). Genuine engine gap. |
| `domain_power_display_record_not_wired` | 41 | Unchanged (confirmed by the isolation diff). 33 bare domain headers + 7 Druid sub-choices + 1 zero-token record. |
| `base_class_standalone_feature_not_computed` | 36 | Unchanged (confirmed by the isolation diff). Unstarted. |
| `prestige_class_standalone_feature_not_computed` | 31 | Unchanged (confirmed by the isolation diff). Unstarted. |
| `other_named_group_or_standalone` | 21 | Unchanged (confirmed by the isolation diff). Unstarted. |
| `rage_power_not_computed` | 13 | Unchanged (confirmed by the isolation diff). Unstarted. |
| `npc_class_standalone_feature_not_computed` | 10 | Unchanged (confirmed by the isolation diff). Unstarted. |
| `rogue_talent_not_computed` | 10 | Unchanged (confirmed by the isolation diff). Unstarted. |
| `versatile_performance_not_computed` | 9 | Unchanged (confirmed by the isolation diff). Unstarted. |
| `monk_unarmed_damage_small_cross_book_attribution_undecided` | 6 | Unchanged (confirmed by the isolation diff). Still open, cross-book attribution question. |

**Sum check:** 23 + 42 + 41 + 36 + 31 + 21 + 13 + 10 + 10 + 9 + 6 = **242**, matching the
row-count command's own remainder exactly (296 total − 54 closed = 242).

## Notes

- **This cycle's fix is deliberately minimal and additive**: one new `EngineFacts` field, one
  new probe (built by copying the prior cycle's own Cleric Domain probe's shape and swapping
  the class/pool constants — the SAME `generic_pool_group_selection_observed_keys` bridge
  function, unmodified), one new `classify()` rung. Zero changes to any anti-fabrication,
  description-quality, or collision guard, and zero changes to
  `push_generic_pool_group_selection_magnitude` itself or any of its 6 pre-existing pool
  wirings.
- **Generic-mechanism reuse proven end to end**: the prior cycle's own receipt predicted this
  exact reuse ("Sorcerer Bloodline is one of the SAME six pools ... this cycle's own bridge
  function is already generic enough to reuse directly") and it held — zero new bridge code,
  only a new probe + a new `classify()` rung, matching the dispatch brief's own generic-work
  framing (build generically, report corpus-wide movement).
- **A genuine new finding this cycle's own scan caught and guarded against, not assumed safe**:
  a real cross-book corpus-key collision (3 keys) the Cleric Domain cycle's own scan did not
  encounter. Handled with an explicit, tested `unit.book == "core_rulebook"` guard rather than
  by asserting the observed set happened to be clean.
- **Territory respected:** no `CharacterInput` field was added or changed; no trait/ability
  compute path was touched (this cycle's own touched files are `src/bin/v06_work_inventory.rs`
  and `scripts/completion_atlas.py` only); the EQUIPMENT magnitude sub-causes (owned by a
  sibling lane) were not touched.
- **Not attempted this cycle**: the remaining 23-unit bloodline residue (three further sub-
  shapes, named above rather than force-closed), and every other named sub-cause in the 242-unit
  remainder table. `monk_unarmed_damage_no_formula_in_engine` (42) is now the largest remaining
  named sub-cause and the next-cheapest candidate is the 16-unit generic-pass-formula-chain
  residue within this cycle's own bloodline sub-cause (same mechanism, a narrower per-record
  question), not a new mechanism.

## Next-cycle plan

1. Investigate the 16-unit generic-pass-formula-chain residue within `bloodline_power_or_
   bloodline_feat_not_computed` — in particular why `Elemental Movement (Air)` resolved but
   Earth/Fire/Water did not (a likely single-default-element seeding gap in how a future probe
   would need to select the elemental sub-choice, itself potentially closable with the SAME
   bridge).
2. `monk_unarmed_damage_no_formula_in_engine` (42, largest remaining named sub-cause) is a
   genuine engine gap requiring a new formula, not a reuse of this mechanism — a different shape
   of work than this cycle's and the Cleric Domain cycle's own reuse.
3. Re-derive the remainder partition fresh before picking (`decisions.md §12` L2) — this
   receipt's own table is this cycle's fresh derivation, but the NEXT cycle must re-run it fresh
   again rather than trust this one.
