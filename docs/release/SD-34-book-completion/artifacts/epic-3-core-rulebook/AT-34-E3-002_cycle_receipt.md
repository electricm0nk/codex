# Cycle 5 — Epic 3 (Core Rulebook to zero) / AT-34-E3-002 (bucket C, "held and computed, never surfaced")

- **Commit SHA:** this cycle's own commit, on `tranche/14` tip `f918018170` (wave 18) at cycle
  start, no rebase needed.
- **Files touched:** `src/bin/v06_work_inventory.rs` (one new `EngineFacts` field
  `bard_versatile_performance_generic_member_wired: BTreeSet<String>`, one new const
  `CORE_RULEBOOK_BARD_VERSATILE_PERFORMANCE_MEMBERS` (9 real Perform types, each mapped to its
  real engine selection slug and detail-name substring), one new probe
  `probe_bard_versatile_performance_generic_member_wiring`, one new `classify()` rung
  book-scoped to `core_rulebook`, 8 new tests (5 probe + 3 classify(), RED confirmed then
  GREEN), `scripts/completion_atlas.py` (10 citation line pins re-derived after this cycle's
  own insertion hunks shifted every one — exact-line-content grep against `git show HEAD:...`,
  never guessed), this receipt, `docs/release/SD-34-book-completion/progress.md`,
  `docs/release/SD-34-book-completion/kanban.md`. **`docs/work-inventory.json` and
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` are
  deliberately NOT committed this cycle** — this dispatch's own file-ownership rule assigns
  their regeneration to the wave's single shared regeneration cycle. Every figure below comes
  from a real, local, uncommitted regen run of this cycle's own committed source, restored
  (`git restore`) before this commit.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (`git diff --unified=0 HEAD -- src/bin/v06_work_inventory.rs
  scripts/completion_atlas.py | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` —
  zero matches, run against this cycle's own working-tree diff. Also re-run against the full
  `merge-base(HEAD, origin/develop)...HEAD` range on the same two files per the dispatch's own
  audit template — zero matches there too, across every prior cycle's own history on these
  files).
- **Wired-integration audit result:** OK_NO_TOKENS (same two diffs, same
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` — zero matches
  on this cycle's own diff; the wide-range diff surfaces 14 pre-existing `placeholder`
  mentions, all from prior cycles' own already-audited "%N-placeholder" / "vacuous-placeholder"
  corpus vocabulary — a real PF1 corpus concept, not a code stub, and none inside this cycle's
  own hunks).
- **Acceptance criterion (verbatim, `epic-breakdown.md` §AT-34-E3-002):** "**370** units the
  engine holds and computes but never surfaces. **Evidence:** per unit, the explanation or
  display path that now carries it. A unit the player still cannot see is not cleared,
  whatever the engine holds." (370 is `epic-breakdown.md`'s own stale figure, already retired
  by wave 16/17 — re-derived fresh at this cycle's start, `core_rulebook` bucket C was **296**
  committed / **242** true-reachable once cycle 4's own already-shipped, not-yet-regenerated
  source is applied, `decisions.md §12` L2.)
- **Status:** partial

## Population, re-derived (not quoted)

At this cycle's start, the **committed** `docs/work-inventory.json` (last regenerated at
wave-16, `dda3438857`) reads `core_rulebook` bucket C = **296**
(`python3 scripts/completion_atlas.py --book core_rulebook --check`). The prior
`AT-34-E3-002` cycle (cycle 4, commit `d97420888e`, Sorcerer Bloodline generic-pool-group
reuse) landed its own source fix on `tranche/14` but — per this dispatch's own
file-ownership rule — never committed the regenerated inventory, so the committed figure is
stale by that cycle's own **54** closures. A live, local, uncommitted regen against this
cycle's *starting* source (before this cycle's own edit) reproduces cycle 4's own reported
**242** exactly, confirming its receipt's own claim rather than trusting it. **Not
re-deriving the whole 242-unit remainder from scratch** — this cycle's own whole-inventory
before/after diff (below) independently re-confirms all ten of cycle 4's named sub-causes are
UNCHANGED except the one this cycle targets, so restating cycle 4's own figures for the other
nine is evidence-backed by this cycle's own proof, not carried forward on trust.

This cycle targeted cycle 4's own named sub-cause `versatile_performance_not_computed`
(**9** units), re-confirmed by direct corpus read before building anything: the real PF1 Core
Rulebook Bard class feature "Versatile Performance" grants three choice slots (levels 2/6/10)
letting a Bard substitute a chosen Perform skill's bonus for two other named skills. The
corpus carries one non-`~` header record (`"Bard ~ Versatile Performance"`, `visible: false`
— **not** targeted this cycle, a different corpus shape, named in the remainder below) plus
**9** real `"Versatile Performance ~ <Type>"` member records, one per PF1's own canonical
Perform subtype (Act, Comedy, Dance, Keyboard Instruments, Oratory, Percussion Instruments,
Sing, String Instruments, Wind Instruments) — confirmed by direct
`docs/work-inventory.json` read, all 9 unique to `core_rulebook`, zero cross-book collisions.

## Mechanism: a DIFFERENT pre-existing engine pass than cycles 3/4 reused, not a re-run of theirs

`pilot_compute::mod.rs`'s Versatile Performance slot loop (SD13-E5, already shipped) is wired
unconditionally for any Bard at level ≥2: it already emits a real
`class_chassis.bard.versatile_performance_choice` recognition-record explanation whose own
`detail` text names the specific selected Perform type verbatim (`"...selection names {name},
whose verified associated skills are {pair}"`), gated at the level-2/6/10 slots. This is a
DIFFERENT engine mechanism from the generic pool-group-selection pass
(`push_generic_pool_group_selection_magnitude`) cycles 3 and 4 reused for Cleric Domain and
Sorcerer Bloodline: its explanation `id` is FIXED per slot (not per-member — every one of the
nine members produces the same `class_chassis.bard.versatile_performance_choice` id), so
member identity can only be read from the `detail` text, never from `id` alone. The value is
a genuine `+0` for every member — PF1's Versatile Performance is a skill-SUBSTITUTION rule,
not an additive bonus, and the substitution engine itself (using the Perform bonus in place of
the associated skills' bonuses) is a separate, larger burden this cycle does not build (the
same "closes the naming gap only" disposition the acceptance criterion's own evidence bar
requires — "the explanation ... path that now carries it", never a magnitude that was not
actually computed).

`v06_work_inventory`'s `classify()` had never once asked this pass a question, for the same
reason cycles 3/4's own receipts named: `canonical_seeds_for("bard")` seeds no Versatile
Performance selection at all, so the canonical per-class sweep that fills
`EngineFacts::explanation_ids` alone can never observe it.

**The fix**, a new probe + rung, NOT a reuse of the generic pool-group bridge (that bridge's
own `generic_pool_group_selection_observed_keys` function parses a literal
`` corpus key `<key>` `` marker `push_generic_pool_group_selection_magnitude`'s own format
string writes — the Bard mechanism's `detail` text carries no such marker, so it cannot be
reused as-is; a small, probe-local, single-purpose bridge was written instead):

1. `probe_bard_versatile_performance_generic_member_wiring` (new): selects each of the 9 real
   Perform types in turn on a real Bard (level ≥2), over the same real
   `compute_pilot_base_chassis` pipeline every other probe in this file uses, and credits a
   corpus key ONLY when the resulting explanation's own `detail` text genuinely names that
   exact type (`detail.contains("names {name},")`) — never a slug reconstruction.
2. A genuine finding this cycle's own scan caught: THREE of the nine real corpus names
   (`"Percussion Instruments"`, `"String Instruments"`, `"Wind Instruments"` — PF1's own
   canonical Perform subtype names, matching the corpus verbatim) do NOT match the engine's
   own shorter display name for the same type (`"Percussion"`, `"String"`, `"Wind"` —
   `pilot_compute::mod.rs`'s own `BARD_VERSATILE_PERFORMANCE_TYPES` table, confirmed by
   reading it directly, not assumed). Fixed with a probe-local mapping table
   (`CORE_RULEBOOK_BARD_VERSATILE_PERFORMANCE_MEMBERS`, corpus name → engine selection slug →
   engine detail-name substring) entirely inside `v06_work_inventory.rs` — `pilot_compute.rs`
   itself was NOT touched, so this stays a probe-local correspondence, never a change to the
   engine's own display strings (in territory: explanation-id/diagnostic-naming wiring only).
3. One new `classify()` rung checking
   `facts.bard_versatile_performance_generic_member_wired.contains(&unit.key)` —
   **book-scoped to `core_rulebook`**, matching cycles 3/4's own precedent. A direct corpus
   scan (this cycle's own) found ZERO cross-book collisions for these nine exact keys (unlike
   cycle 4's three), but the guard costs nothing and is tested regardless
   (`a_versatile_performance_key_in_a_different_book_is_not_credited`).

## RED → GREEN

RED (confirmed for the intended reason): temporarily changed the new rung's containment check
to look up a key no probe could ever produce
(`facts.bard_versatile_performance_generic_member_wired.contains(&format!("RED-CHECK-{}", &unit.key))`)
and re-ran `an_act_versatile_performance_record_the_probe_observed_reaches_grounded` — failed
with `left: "engine-does-not-hold", right: "grounded"` (the pre-existing fallthrough this
cycle closes), confirming the test fails because the fix is absent, not for an unrelated
reason. Restored the rung; the test (and all others) passes.

```
$ cargo test --locked --bin v06_work_inventory an_act_versatile_performance_record_the_probe_observed_reaches_grounded
running 1 test
test class_feature_text_complete_rung_tests::an_act_versatile_performance_record_the_probe_observed_reaches_grounded ... FAILED
left: "engine-does-not-hold"
right: "grounded"
```

After restoring the rung:

```
$ cargo test --locked --bin v06_work_inventory versatile_performance -- --nocapture
running 8 tests
test class_feature_text_complete_rung_tests::an_act_versatile_performance_record_the_probe_never_observed_is_unaffected ... ok
test class_feature_text_complete_rung_tests::a_versatile_performance_key_in_a_different_book_is_not_credited ... ok
test class_feature_text_complete_rung_tests::an_act_versatile_performance_record_the_probe_observed_reaches_grounded ... ok
test bard_versatile_performance_generic_member_probe_tests::the_probe_does_not_credit_an_unrelated_class_feature_record ... ok
test bard_versatile_performance_generic_member_probe_tests::the_probe_resolves_all_three_shortened_display_name_members ... ok
bard_versatile_performance_generic_member_wired (9 keys):
  Versatile Performance ~ Act
  Versatile Performance ~ Comedy
  Versatile Performance ~ Dance
  Versatile Performance ~ Keyboard Instruments
  Versatile Performance ~ Oratory
  Versatile Performance ~ Percussion Instruments
  Versatile Performance ~ Sing
  Versatile Performance ~ String Instruments
  Versatile Performance ~ Wind Instruments
test bard_versatile_performance_generic_member_probe_tests::print_the_real_observed_set_for_this_cycles_own_receipt ... ok
test bard_versatile_performance_generic_member_probe_tests::the_probe_resolves_all_nine_real_perform_types ... ok
test bard_versatile_performance_generic_member_probe_tests::the_probe_observes_a_real_act_versatile_performance_member ... ok

test result: ok. 8 passed; 0 failed
```

Full `class_feature`-scoped suite: `cargo test --locked --bin v06_work_inventory class_feature`
— **142 passed, 0 failed**. Full bin suite: **470 passed, 0 failed** (this cycle's own net-new
count is exactly 8 tests, confirmed by `git diff` hunk count on this cycle's own two-file
diff; the pre-cycle absolute count already carried one test added by a concurrent sibling
lane's own commit between cycle 4 and this cycle's start, so the pre/post absolute-count delta
is not a clean 8 — the 8-test claim is proven by this cycle's own diff, not by subtracting
stale absolute counts).

## Live regen (local, uncommitted — see file-ownership note above)

Same guard cycles 3/4's own receipts document. Ran `corpus_literal_sweep --json-out` (CLEAN,
`48,708` records examined, `0` findings — matching the unchanged baseline exactly, since this
cycle touches no `data/corpus/**` file) and `derived_evaluator_fixture_check --json-out`
(`2,580` fixture rows, `1,839` cleared, `0` failed — same unchanged baseline) first, then
pointed `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` at the two reports and
regenerated (`--allow-stamp-loss` never passed).

**Isolation confirmed by a whole-inventory before/after diff keyed on unit id** (not sampled —
a real Python diff over both full 49,438-unit JSON documents, before = the COMMITTED HEAD
inventory via `git show HEAD:docs/work-inventory.json`, after = this cycle's own local regen
against HEAD's committed source plus this cycle's own edit):

```
before count: 49438 after count: 49438
added: 0 removed: 0
changed: 67
changed by book: {'core_rulebook': 63, 'ultimate_campaign': 4}
```

The 67 changes decompose into exactly three independently-attributable groups, distinguished
by their own evidence strings (never conflated):

- **54** `core_rulebook` changes carrying evidence
  `generic_pool_group_selection_probe_observed_a_real_computed_magnitude` — cycle 4's own
  ALREADY-COMMITTED Sorcerer Bloodline fix (44 to DONE, 10 to V), landed on `tranche/14`
  before this cycle started, simply never regenerated into the committed JSON until now. **Not
  this cycle's work.**
- **4** `ultimate_campaign` changes carrying evidence
  `trait_content_magnitude_computed_and_verified_by_fixture_execution_flat_{1,2}` — a sibling
  lane's own already-committed trait/ability-compute fix (AT-34-E4-002), outside this cycle's
  territory and outside `core_rulebook`. **Not this cycle's work; not touched.**
- **9** `core_rulebook` changes carrying evidence
  `bard_versatile_performance_choice_probe_observed_a_real_named_recognition_record` — **this
  cycle's own closure**, all nine `Versatile Performance ~ <Type>` records, all
  `engine-does-not-hold` → `grounded` (bucket **DONE** directly — every one of the nine carries
  `wiring_class: "computed"`, confirmed by direct `docs/work-inventory.json` read before this
  cycle began, so none are eligible for the static-record V-reclassification path
  `apply_done_rung_stamps` used for 10 of cycle 4's own 54).

54 + 4 + 9 = 67, matching the whole-inventory diff exactly. This cycle's own contribution is
isolated to precisely 9 ids, named individually:
`core_rulebook:class_feature:versatile_performance_{act,comedy,dance,
keyboard_instruments,oratory,percussion_instruments,sing,string_instruments,
wind_instruments}`.

**Corpus-wide isolation** (same regen, same before/after documents):

```
before: DONE=24353 A=449 B=11769 C=4277 D=2955 M=4965 U=202 V=279 X=170 Z=19
after:  DONE=24410 A=449 B=11769 C=4214 D=2955 M=4961 U=202 V=289 X=170 Z=19
```

DONE +57 (44 bloodline + 4 trait + 9 mine), C −63 (54 bloodline + 9 mine — trait's own M→DONE
move does not touch C), M −4 (the sibling trait lane's own M→DONE move, not mine), V +10
(bloodline's own static reclassification, not mine) — every figure accounted for by the three
named groups above, none unexplained.

## Figures + their re-derive commands

| Figure | Value | Command | Denominator |
|---|---:|---|---|
| `core_rulebook` bucket C, committed at cycle start | 296 | `python3 scripts/completion_atlas.py --book core_rulebook --check` against HEAD's committed `docs/work-inventory.json` | of 6,701 |
| `core_rulebook` bucket C, true-reachable at cycle start (cycle 4's already-shipped source, live regen) | 242 | same command, live regen of pre-this-cycle source | of 6,701 |
| `core_rulebook` bucket C after this cycle's own fix | **233** | same command, live regen including this cycle's edit | of 6,701 (delta −9 vs. 242, this cycle's own isolated contribution) |
| `core_rulebook` bucket DONE after this cycle | **4,436** | same command | of 6,701 |
| `core_rulebook` buckets A/B/D/M/V/U/X/Z after this cycle | unchanged from cycle 4's own post-state (0/470/366/957/114/10/115/0) | same command | of 6,701 — confirms isolation: nothing outside C/DONE moved for this cycle's own 9 |
| Corpus-wide bucket C before/after this cycle's own regen | 4,277 / **4,214** | `python3 scripts/completion_atlas.py --check` | of 49,438 (delta −63 = 54 cycle-4 + 9 this cycle, not conflated) |
| Corpus-wide bucket DONE before/after | 24,353 / **24,410** | same command | of 49,438 (delta +57 = 44 + 4 + 9, three independently-named groups) |
| Whole-inventory before/after diff, keyed on unit `id` | 0 added, 0 removed, exactly 67 changed (63 `core_rulebook`, 4 `ultimate_campaign`) | (Live regen section) | of 49,438 |
| This cycle's own isolated closures | **9**, all `core_rulebook`, all `Versatile Performance ~ <Type>` | whole-inventory diff filtered on this cycle's own evidence string | of 9 (100% of the targeted sub-cause) |
| `versatile_performance_not_computed` sub-cause, cycle 4's own re-confirmed membership | 9 (all 9 targeted, all 9 closed) | direct `docs/work-inventory.json` read, bucket-C units matching corpus key prefix `"Versatile Performance ~ "` | of 296 |
| Probe's own real observed set (live-fixture test) | 9 of 9 | `cargo test --locked --bin v06_work_inventory bard_versatile_performance_generic_member_probe_tests::the_probe_resolves_all_nine_real_perform_types` | of 9 real Perform types |
| `corpus_literal_sweep` (before/after, unchanged) | 48,708 examined, 0 findings | `corpus_literal_sweep --json-out` | of 51,482 read |
| `derived_evaluator_fixture_check` (before/after, unchanged) | 1,839 cleared of 2,580 rows, 0 failed | `derived_evaluator_fixture_check --json-out` | of 2,580 |
| `completion_atlas.py --check` (corpus-wide, post-regen) | `population=49438 unclassified=0 overlap=0` | `python3 scripts/completion_atlas.py --check` | of 49,438 |
| `completion_atlas.py --check` `citation_failures` | 0 (10→0, re-derived this cycle after this cycle's own insertion hunks shifted every pin) | `python3 scripts/completion_atlas.py --check` | of 10 citations |
| `cargo test --locked --bin v06_work_inventory` (full) | `470 passed; 0 failed` | `cargo test --locked --bin v06_work_inventory` | of 470 |
| `cargo test --locked --bin v06_work_inventory class_feature` | `142 passed; 0 failed` | `cargo test --locked --bin v06_work_inventory class_feature` | of 142 |
| `cargo test --locked --no-run` (workspace) | exit 0 | `cargo test --locked --no-run` | — |

## Row-count command output (this cycle's own live artifact, uncommitted per file-ownership rule)

```
$ python3 scripts/completion_atlas.py --book core_rulebook --check
book=core_rulebook population=6701 unclassified=0 overlap=0
  DONE: 4436
  A: 0
  B: 470
  C: 233
  D: 366
  M: 957
  V: 114
  U: 10
  X: 115
  Z: 0
```

Bucket C: **233**, not zero. **Status: partial**, remainder named below (populations sum
exactly to 233). This live command output was produced by the local, uncommitted regen and is
NOT reflected in the currently-committed `docs/work-inventory.json` (restored via
`git restore` before this commit, per the file-ownership rule) — the committed inventory
still reads C=296 until the wave's shared regeneration cycle re-runs the pipeline against
this cycle's own committed source (and cycle 4's, and the sibling trait lane's).

## Build scope verified

`cargo test --locked --no-run` (workspace) exits **0**, run at this cycle's own tip.
`cargo test --locked --bin v06_work_inventory` 470/470 pass. Desktop crate
(`apps/desktop/src-tauri`) not tested this cycle: no file under that tree, nor any file it
depends on, was touched by this cycle's own two-file diff (confirmed:
`grep -rl "bard_versatile_performance_generic_member|CORE_RULEBOOK_BARD_VERSATILE_PERFORMANCE" apps/`
— zero matches).

## Sweep population

`corpus_literal_sweep`: 48,708 examined, before and after — unchanged, since no
`data/corpus/**` file was added or regenerated this cycle.

## Oracle pin

N/A — no figure in this receipt came from the pinned PCGen oracle corpus.

## Movement, four buckets

- **Closure:** **9** — the 9 `Versatile Performance ~ <Type>` units, all carrying
  `wiring_class: "computed"`, moved `engine-does-not-hold` (bucket C) → `grounded` (bucket
  **DONE**) directly. Nothing remains for these; all 9 are `+0` recognition records (a
  genuine PF1 rule, not an unfinished formula), so no further magnitude work is owed by this
  sub-cause — the real remaining engine burden (the skill-substitution rule itself) is a
  SEPARATE feature, not part of this bucket's own naming-gap acceptance bar.
- **Reclassification:** 0 this cycle (cycle 4's own 10 static→V reclassifications are cycle
  4's, not this cycle's — see the isolation section above).
- **Reachability:** **9** (one new grounding rung + one new probe now answer `grounded` for
  these exact corpus keys, reusing a real, already-shipped, already-tested engine explanation
  — no new compute path, no new formula).
- **Instrument-correction:** 0 this cycle (the 10 `completion_atlas.py` citation re-pins are a
  bookkeeping side effect of this cycle's own insertions, not a correction of a wrong prior
  figure).

**Bucket C's own delta (242 → 233, −9, measured against the true-reachable baseline that
already includes cycle 4's own unregenerated fix) equals this cycle's own Closure exactly** —
the row-count command's own output above is the ground truth this movement report is checked
against, not the other way around. Against the COMMITTED baseline (296), the combined delta
of −63 (54 cycle-4 + 9 this cycle) is reported above but is NOT this cycle's own closure claim
— only the 9 is.

## Remainder — 233 of 296 (242 true-reachable), named by sub-cause, populations sum exactly

Re-derived fresh at this cycle's own close (`decisions.md §12` L2) — every sub-cause this
cycle did not touch is confirmed UNCHANGED by direct proof (the whole-inventory diff above
shows the ONLY `core_rulebook` changes outside cycle 4's own 54 are this cycle's own 9, so
restating cycle 4's other nine sub-cause figures is evidence-backed, not carried forward by
assumption):

| Sub-cause | Population | Status / next step |
|---|---:|---|
| `bloodline_power_or_bloodline_feat_not_computed` | 23 | Unchanged (confirmed by this cycle's own isolation diff — zero bloodline records changed this cycle beyond cycle 4's own 54). Cycle 4's own named next-candidate: the 16-unit generic-pass-formula-chain residue (why `Elemental Movement (Air)` resolved but Earth/Fire/Water did not). |
| `monk_unarmed_damage_no_formula_in_engine` | 42 | Unchanged (confirmed). Genuine engine gap — needs a new formula, not a naming-only fix; largest remaining named sub-cause. |
| `domain_power_display_record_not_wired` | 41 | Unchanged (confirmed). 33 bare domain headers + 7 Druid sub-choices + 1 zero-token record. |
| `base_class_standalone_feature_not_computed` | 36 | Unchanged (confirmed). Unstarted. |
| `prestige_class_standalone_feature_not_computed` | 31 | Unchanged (confirmed). Unstarted. |
| `other_named_group_or_standalone` | 21 | Unchanged (confirmed). Unstarted. |
| `rage_power_not_computed` | 13 | Unchanged (confirmed). Unstarted. |
| `npc_class_standalone_feature_not_computed` | 10 | Unchanged (confirmed). Unstarted. |
| `rogue_talent_not_computed` | 10 | Unchanged (confirmed). Unstarted. |
| `versatile_performance_not_computed` | **0** (was 9) | **CLOSED this cycle — all 9 members reached `grounded`.** The one remaining artifact in this corpus group, `"Bard ~ Versatile Performance"` (the top-level header record, `visible: false`, a different `" ~ "` shape — class-name-first, not the sub-pool's own group prefix), was deliberately NOT targeted this cycle and is folded into `other_named_group_or_standalone` for the next cycle's own re-derivation (not this receipt's remainder count, since it was already excluded from cycle 4's own 9-unit `versatile_performance_not_computed` figure — confirmed by re-reading cycle 4's own receipt, which named exactly 9, matching this cycle's own corpus scan). |
| `monk_unarmed_damage_small_cross_book_attribution_undecided` | 6 | Unchanged (confirmed). Still open, cross-book attribution question. |

**Sum check:** 23 + 42 + 41 + 36 + 31 + 21 + 13 + 10 + 10 + 0 + 6 = **233**, matching the
row-count command's own remainder exactly (242 true-reachable − 9 closed = 233).

## Notes

- **This cycle's fix is deliberately minimal and additive**: one new `EngineFacts` field, one
  new probe, one new `classify()` rung, one probe-local mapping table. Zero changes to
  `pilot_compute::mod.rs` or any other production compute path — the engine's own Versatile
  Performance mechanism, display-name strings included, is completely untouched; this cycle
  only teaches the CLASSIFIER to consult an explanation that already existed.
- **A different pre-existing mechanism than cycles 3/4's own generic pool-group pass, proven
  rather than assumed to be the same shape**: this cycle's own bridge is new, purpose-built
  for the Bard's fixed-id/detail-named recognition record, because `generic_pool_group_
  selection_observed_keys`'s own literal marker format does not appear in this mechanism's
  `detail` text. Reuse was checked first and genuinely does not apply here — not skipped by
  assumption.
- **A genuine display-name mismatch this cycle's own scan caught**: three of the nine real
  corpus names (`Percussion Instruments`, `String Instruments`, `Wind Instruments`) do not
  match the engine's own shorter display names (`Percussion`, `String`, `Wind`). Handled with
  a probe-local mapping table, proven by a dedicated test
  (`the_probe_resolves_all_three_shortened_display_name_members`) rather than assumed safe.
- **Territory respected:** no `CharacterInput` field was added or changed; no trait/ability
  compute path was touched (this cycle's own touched files are `src/bin/v06_work_inventory.rs`
  and `scripts/completion_atlas.py` only, confirmed by `git status --porcelain` before this
  commit); the EQUIPMENT magnitude sub-causes (owned by a sibling lane) were not touched; the
  `ultimate_campaign` trait-content closures found in this cycle's own live regen belong to a
  DIFFERENT, already-committed sibling-lane cycle and were correctly attributed to it, not
  claimed here.
- **Not attempted this cycle**: every other named sub-cause in the 233-unit remainder table.
  `monk_unarmed_damage_no_formula_in_engine` (42) is the largest remaining named sub-cause and
  is a genuine engine-formula gap, not a naming-only fix like this cycle's — a materially
  different shape of work.

## Next-cycle plan

1. `monk_unarmed_damage_no_formula_in_engine` (42, largest remaining named sub-cause) needs a
   real new formula in the engine, not a classifier-naming fix — a different shape of work
   than this cycle's and cycles 3/4's own reuse. Out of this territory's "naming-only" bar
   unless the engine-side formula work is dispatched separately first.
2. `domain_power_display_record_not_wired` (41) is the next-cheapest CANDIDATE within this
   territory's own shape (a display/header record whose sibling magnitude record may already
   be wired, the same "paired display/chassis" pattern the original Favored Enemy/Terrain
   cycle established) — worth checking whether Domain Power's own header records have a
   sibling `Domain Power ~ <Power>` magnitude record already grounded, the same reuse this
   cycle proved out for a completely different mechanism.
3. Re-derive the remainder partition fresh before picking (`decisions.md §12` L2) — this
   receipt's own table is this cycle's fresh derivation, but the NEXT cycle must re-run it
   fresh again rather than trust this one, especially since the wave's shared regeneration
   cycle has not yet run against this cycle's own commit.
