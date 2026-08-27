# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`class_absent_from_ClassId_ALL_and_book_class_id_enums` mechanism)

- **Commit SHA:** `<pending — filled after commit, see follow-up docs commit>`
  (parent `1c4c479110`)
- **Files touched:**
  - `src/rules_core/pilot_compute/crb_untabled_class_chassis.rs` (new) — real
    base-attack-bonus/save chassis for CRB's five NPC classes and two `Ex-*`
    variant states, evaluating each class's own corpus `BONUS:COMBAT|BASEAB`/
    `BONUS:SAVE` formula strings via `PcgenFormulaEvaluator` (the same
    evaluator `generic_class_chassis.rs` already proved against 61 other
    classes), never a hand-typed table.
  - `src/rules_core/pilot_compute/mod.rs` — registers the new module; adds a
    `compute_class_chassis` dispatch arm for it (mirroring the existing
    `untabled_base_class_chassis`/`generic_class_chassis` arms' explanation
    shape).
  - `src/bin/v06_work_inventory.rs` — `modelled_class_books()` gains two new
    loops: CRB's ten prestige classes (from `prestige_class_entry_gate`'s own
    already-real registry — no new chassis code, respecting SD-32's existing
    deferral of full prestige-class chassis) and the seven NPC/`Ex-*` classes
    (from the new module above). Both keyed by the corpus's own **lowercased
    display name** (a space for a multi-word class), not the registry's
    underscored `class_id` slug — see the registration site's own comment for
    why substituting the slug would silently break the `classify()` lookup.
    New test module `modelled_class_books_registry_tests` gains 3 tests
    proving both registrations and the mechanism's own closure.
  - `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs` —
    re-pinned a stale F1-population assertion discovered by this cycle's own
    §6 step 3 full-lib-suite run; **pre-existing, not this cycle's own
    regeneration** (confirmed by a clean-worktree reproduction at this
    cycle's own start SHA `ae25d75d7d`, zero of this cycle's edits applied —
    see "Discoveries" below).
  - `scripts/completion_atlas.py` — ten `BUCKET_DEFINITIONS` `file:line`
    citations re-derived and corrected (this cycle's own insertions in
    `v06_work_inventory.rs` shifted every one).
  - `docs/work-inventory.json` (regenerated at HEAD, guarded regeneration
    path — plain `cargo run --locked --release --bin v06_work_inventory`,
    `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` set from
    this session's own fresh `corpus_literal_sweep`/
    `derived_evaluator_fixture_check` runs, no `--allow-stamp-loss` used or
    needed).
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    (regenerated output of `completion_atlas.py --check`, not hand-edited).
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_class_absent_cycle_receipt.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`
  - `docs/retro/events/sd34-at-34-e3-001.jsonl` (one `correction` event, the
    F1-pin discovery)

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` on the scoped diff —
  `git diff <base>...HEAD -- src/rules_core/ src/bin/ scripts/oracle_harness/
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` → no matches
  in code (the same command widened to also include
  `docs/work-inventory.json` surfaces thousands of pre-existing
  `sd32_class_ingest`/`sd32_simple_filename_kind_ingest` matches — historical
  `wiring_class_signals` **data values** already present in that file before
  this cycle, confirmed by the exact same shape the `race_trait_absent`
  cycle's own receipt already documented and self-healed).

- **Wired-integration audit result:** `OK_NO_TOKENS` on this cycle's own new
  file (`grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|
  hack)\b' src/rules_core/pilot_compute/crb_untabled_class_chassis.rs` → no
  matches). The wider epic-scoped diff against `origin/develop` carries five
  "placeholder" matches, all inside `ingest_race_traits.rs` from the
  `race_trait_absent` cycle (PCGen's own literal `###Block: Placeholder
  objects...` comment, already reviewed and self-healed in that cycle's own
  receipt) — not new, not this cycle's own code.

- **Acceptance criterion (verbatim, `epic-breakdown.md` AT-34-E3-001):** "**970**
  Core Rulebook units whose table exists but which are not in it. **Evidence:**
  the atlas reporting bucket B at zero for `core_rulebook`, and the mechanism
  that placed them named — **by mechanism, not per record.**" This cycle's
  own bar (`decisions.md §14`): drive `class_absent_from_ClassId_ALL_and_
  book_class_id_enums` to zero. **AT-34-E3-001 as a whole does not close
  this cycle** — six of the nine named mechanisms remain (`domain`,
  `race_trait_absent_from_race_traits`, and this cycle's own
  `class_absent...` are now closed); this receipt reports only this cycle's
  own mechanism.

## Re-derived population, not carried forward

Re-derived at this cycle's start SHA (`1c4c479110`), **not** quoted from the
task brief or `decisions.md §14`'s table without checking:

```
$ python3 -c "
import json
with open('docs/work-inventory.json') as f:
    units = json.load(f)['units']
from collections import Counter
c = Counter()
for u in units:
    if u.get('book')=='core_rulebook' and u.get('status')=='engine-does-not-hold':
        c[u.get('evidence')] += 1
print(c['class_absent_from_ClassId_ALL_and_book_class_id_enums'])
"
17
```

Matches the brief's and `decisions.md §14`'s stated figure exactly — verified,
not assumed. The 17 units, listed by `id`:

`core_rulebook:class:{adept, arcane_archer, arcane_trickster, aristocrat,
assassin, commoner, dragon_disciple, duelist, eldritch_knight, ex_barbarian,
ex_paladin, expert, loremaster, mystic_theurge, pathfinder_chronicler,
shadowdancer, warrior}`

## The root cause, and why it splits into two real, pre-existing mechanisms

`modelled_class_books()` (`v06_work_inventory.rs`) is the map `classify()`'s
`Kind::Class` arm consults to decide whether "the engine models a class of
this name at all." It was built from five `ClassId`-family enums plus the
`untabled_base_class_chassis` registry — all scoped to **base classes**. CRB
carries 28 real `CLASS:` records in `cr_classes.lst`/`data/corpus/
core_rulebook/class/`: 11 real base classes (already in `ClassId::ALL`), 10
`TYPE:PC.Prestige` classes, 5 `TYPE:Base.NPC` classes, and 2 `TYPE:Base.PC,
VISIBLE:NO` `Ex-*` variant states (Ex-Barbarian, Ex-Paladin — a fallen
Barbarian/Paladin's post-transgression state, not a player-selectable
class). None of the last 17 was registered anywhere.

**Ten (the prestige classes) needed zero new chassis code.**
`prestige_class_entry_gate.rs` already carries a real, corpus-derived
registry evaluating these ten classes' genuine `PRE*` entry-requirement
tokens (SD-32 `AT-32-E3-001`), already wired into `compute_class_chassis` —
it was simply never read by `modelled_class_books()`. Registering from it
costs zero new compute logic.

**Seven (5 NPC + 2 `Ex-*`) needed a small, genuinely new chassis.** Direct
read of these seven classes' own `data/corpus/core_rulebook/class/*.json`
`raw_tokens` confirmed every one uses the exact same
`classlevel("APPLIEDAS=NONEPIC")`-based `BONUS:COMBAT|BASEAB`/`BONUS:SAVE`
formula shape CRB's eleven real base classes use — real, computable,
corpus-authored formulas, not fabricated data. `generic_class_chassis.rs`
already proves the right-sized method for this exact shape (evaluate the
corpus's own formula string via `PcgenFormulaEvaluator`, not a hand-typed
`BabProgression` classification) against 61 conventional classes across 14
other books; `crb_untabled_class_chassis.rs` reuses that identical method,
scoped to these seven, in its own module rather than widening
`generic_class_chassis.rs`'s own book list (that module's population is
mirrored byte-for-byte in `apps/desktop/src-tauri`'s separate
`class_catalog_generic.rs`; widening the shared list here without updating
that separate crate would silently desynchronize the two).

**Why not one registry for all 17.** CRB's ten prestige classes already have
a documented, deliberate deferral (SD-32 `decisions.md §10`): six of the ten
need a caster-level-stacking mechanism this codebase does not have yet before
a FULL chassis (not just entry gating) can be built for them. Building a
second, competing chassis for them here — even a partial one — would reopen
a decision this cycle has no standing to reverse. The seven NPC/`Ex-*`
classes carry no such deferral and no caster-level dependency at all (their
own BAB/save formulas are plain `classlevel`-based, no spellcasting
involved), so a real chassis for them is in scope and genuinely new work.

## RED → GREEN (for the intended reason, not a compile error)

- RED: `crb_untabled_class_chassis::covered_classes().len()` temporarily
  hard-coded to `0` (verifying the real registration path, not a stand-in) →
  `cargo test --bin v06_work_inventory
  modelled_class_books_registry_tests::all_seven_crb_npc_and_ex_classes_are_registered_from_their_own_registry` →
  **FAILED**, `"Adept must be registered under its own lowercased display
  name" left: None right: Some("core_rulebook")` — the intended reason (the
  map genuinely does not carry the class), not a panic elsewhere. The sibling
  prestige test and the mechanism-closure test failed identically the same
  way for the same reason (`left: "class_absent_from_ClassId_ALL_and_book_
  class_id_enums" right: "class_absent_from_ClassId_ALL_and_book_class_id_enums"`
  for the closure test — i.e. the mechanism did **not** clear, RED for the
  correct reason).
- GREEN after restoring the real registration loops:
  `cargo test --bin v06_work_inventory modelled_class_books_registry_tests`
  → **8 passed; 0 failed** (5 pre-existing + 3 new:
  `all_ten_crb_prestige_classes_are_registered_from_the_entry_gate_registry_itself`,
  `all_seven_crb_npc_and_ex_classes_are_registered_from_their_own_registry`,
  `a_previously_absent_crb_class_leaves_this_mechanism_once_registered`).
- New module's own unit tests: `cargo test --lib crb_untabled_class_chassis`
  → **5 passed; 0 failed** (`warrior_full_bab_matches_the_corpus_classlevel_
  formula_at_level_ten` and `commoner_half_bab_and_all_poor_saves_match_the_
  corpus_formula` pin real numeric BAB/save values against the corpus's own
  formulas by hand-calculation, not against the code's own output).
- Full binary suite: `cargo test --bin v06_work_inventory` → **374 passed;
  0 failed**.
- Full lib suite (after the F1 re-pin below): `cargo test --locked --lib` →
  **2,863 passed; 0 failed; 14 ignored**.

## Discoveries

**1. A cross-book class-feature attribution side effect (self-healed, not a
regression this cycle introduced).** Registering common-English-word class
names (`warrior`, `assassin`, `expert`, `adept`, `aristocrat`, `commoner`)
exposed a latent property of `class_feature_owner`'s whole-corpus, non-book-
scoped suffix/prefix matching: a shorter, now-modelled class name can win a
match against an unrelated compound group text from a **different** book
(e.g. `ultimate_psionics`'s own distinct, still-unmodelled "Adaptive Warrior"
class, `"adaptive warrior"` ends with `" warrior"`) purely because the true,
more specific candidate was never itself a `class_books` entry to lose to.
Verified this is **not new**: the exact same misattribution already existed
via the `corpus_class_names` fallback (pre-existing evidence
`class_feature_of_unmodelled_corpus_class:<the same wrong short name>`,
confirmed against the committed inventory before this cycle) — my change
only moved it between evidence strings/buckets, it did not invent it. Two
mitigations applied:
1. A cross-check guard (`v06_work_inventory.rs`, `classify()`'s
   `Kind::ClassFeature` arm) refuses a `class_books`-based owner match when a
   **wider**, corpus-wide search (`facts.corpus_class_names`, which also
   sees still-unmodelled classes) would have picked a **different**
   candidate — restoring identical behavior for every genuine cross-book
   collision case while leaving same-name matches (including these seven
   classes' own real `core_rulebook` features) untouched. Proven safe: full
   `cargo test --bin v06_work_inventory` (374/374) and `cargo test --locked
   --lib` (2,863/2,863) stay green.
2. Verified the two statuses that actually matter cannot be falsely earned
   regardless: `grounded` (both `class_feature_exact_suffix_grounded` and
   `suffix_stripped_grounded`) requires `group.eq_ignore_ascii_case(&owner)`
   — an **exact** match, never a suffix/prefix one, so a compound group like
   `"adept arcana"` can never reach `grounded` off CRB's plain `"adept"`.
   `text-complete` (`class_feature_pool_catalog_serves_a_rendered_
   description`) is gated by `class_feature_pool_catalog_holds(source_book,
   key)` — a real, independent, per-**record** whitelist built from
   `class_feature_pool_catalog.rs`'s own corpus-derived catalog, checked by
   the record's own `(book, key)`, never by `owner`'s correctness. The 19
   units this cycle newly promotes to `text-complete` outside
   `core_rulebook` are therefore genuinely earned completions that an
   unrelated upstream short-circuit (`class_feature_of_unmodelled_corpus_
   class`) was blocking, not false claims this cycle manufactured.

Net effect outside `core_rulebook`, all reviewed: 187 units across 8 other
books change `(status, evidence)`; of those, 19 are the genuine, independently
-verified `text-complete` unlocks above; the rest are evidence-string
relabeling within the not-done territory (a pre-existing heuristic's
visibility changing, not a new defect). None reaches `grounded` falsely.
This is reported honestly here as this cycle's own reclassification side
effect — a future cycle scoped to any of those other books' own bucket
mechanisms should re-derive from HEAD rather than trust a number quoted
before this cycle.

**2. A stale, pre-existing `cargo test --locked --lib` failure, found by this
cycle's own §6 step 3 run, not caused by it.**
`formula_interpreter_corpus_wide::tests::f1_population_matches_the_current_
true_formula_bearing_count_not_the_stale_sd32_census` asserted F1 population
`6,257` (pinned by SD-33's own closure); this cycle's `cargo test --locked
--lib` found `5,563`. **Confirmed pre-existing, not this cycle's own
regeneration**: a clean worktree at this cycle's own start SHA `ae25d75d7d`,
with **zero** of this cycle's code changes, reproduces the identical `left:
5563, right: 6257` failure — `docs/work-inventory.json` was regenerated at
least four more times after the 6,257 pin
(`0099df7a1e`/`a72c6787e6`/`6eab21d761`/`ae25d75d7d`, each verifying its own
scoped `cargo test --bin v06_work_inventory` but never re-running `cargo test
--locked --lib`, where this pin lives — the exact "run the suite after the
last write that can move it" lesson recurring). Re-derived via the test's own
named command, `python3 scripts/shape_ledger.py --inventory docs/work-
inventory.json --corpus-root data/corpus`, **twice independently** — against
this cycle's own final `docs/work-inventory.json` (5,563) and against
`ae25d75d7d`'s own untouched committed copy (also 5,563) — confirming this
cycle's own edits moved **zero** F1-population units. Re-pinned to 5,563
with the full history preserved in the test's own doc comment; logged as a
`correction` retro event (`docs/retro/events/sd34-at-34-e3-001.jsonl`).

**3. Two more stale `file:line` citations this cycle's own line-insertions
shifted, self-healed in the same cycle (`workflow-instruction.md`'s own
citation-drift warning).** `scripts/missing_engine_tables.py`'s
`ENGINE_SURFACE_CITATIONS` (a citation table separate from
`completion_atlas.py`'s own ten, keyed on the `companion`/`power` bucket-A
markers) pointed at old lines 9772/9833; corrected to 9844/9908.
`src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs`'s own new
doc-comment text (this cycle's F1 fix above) accidentally spelled the
retired `not-ingested` status literal, tripping
`test_legacy_not_ingested_string_swept.py`'s fail-closed sweep
(`AT-34-E1-005`'s own regression guard, working exactly as designed);
reworded to name the rename without repeating the retired spelling. Both
confirmed fixed: `python3 scripts/tests/test_missing_engine_tables.py` and
`python3 scripts/tests/test_legacy_not_ingested_string_swept.py` → `OK`.

**4. A larger, pre-existing `scripts/tests/` citation-drift cluster found,
confirmed NOT this cycle's own regression, and left unfixed (out of
scope).** `python3 -m unittest discover -s scripts/tests -p 'test_*.py'` at
this cycle's own final state: **840 tests, 9 failures, 2 errors** — the same
`test_shape_engine_boundary.py` (4 failures + 2 errors — a DIFFERENT,
larger citation table with its own stale `PROMOTION_LADDER_ANCHOR_LINE` and
a `not_held_by_engine` count assertion off by thousands), `test_box_ledger.py`
(THE-BOX.md's own frozen counts drifting from the ever-growing live
inventory — expected under `decisions.md §2`'s "inherited read-only" rule,
since `box_ledger.py --check` itself still exits 0; only one hardcoded
`unittest` assertion inside `test_check_against_live_committed_files`
demands `uncovered=0`, which was never going to hold once SD-34 started
adding units THE-BOX never partitioned), `test_denominator_gate.py`,
`test_race_trait_remediation.py`, `test_derive_derived_evaluator_fixtures.py`,
and `test_transcribe_monster_tables.py`. **Every one reproduced identically
at this cycle's own start SHA `ae25d75d7d`** (same assertion, same numbers,
same file paths) — confirmed pre-existing, not introduced here. Left
unfixed: `test_shape_engine_boundary.py`'s own citation table is a
substantially larger, separate body of drift (its own anchor line plus a
live-count assertion, unrelated to `Kind::Class`) that belongs to whichever
cycle or `AT-34-E6-001`'s own closure scan next touches that instrument, not
folded into this one-mechanism cycle's own scope.

## Figures, with re-derive commands and denominators

- **Mechanism population:** `17 -> 0` for `core_rulebook`'s
  `class_absent_from_ClassId_ALL_and_book_class_id_enums` (denominator:
  bucket B's own partition, command above).
- **`core_rulebook` bucket B (whole book, atlas-official
  `completion_atlas.py` partition):** `996 -> 995` (denominator:
  `core_rulebook`'s own unit count, `population=6701`, unchanged). **Not** a
  clean `-17`: `-17` (this mechanism) `+16` (a legitimate side effect —
  16 `core_rulebook` `class_feature` records for these same seven now-
  modelled classes, e.g. `Assassin ~ Hidden Weapons`, correctly reattribute
  off "of_unmodelled_corpus_class" onto a **different**, unowned bucket-B
  mechanism, `class_feature_owner_matched_by_name_but_record_not_held_by_
  engine` — an exact same-word, same-book match, not a collision). Command:
  `python3 scripts/completion_atlas.py --book core_rulebook --check`.
- **Corpus-wide bucket B:** `14,118 -> 14,121` (+3 net: -17 this mechanism,
  +16 the `core_rulebook` class_feature reattribution above, plus a small
  net change from the cross-book discovery's own relabeling — every unit
  accounted for in "Discoveries" above). Command:
  `python3 scripts/completion_atlas.py --check`.
- **Corpus-wide population, unaffected:** `49,438` before and after — no unit
  added or removed, only reclassified. Same command as above.
- **`crb_untabled_class_chassis`'s own registered population:** `7 of 7`
  (denominator: the fixed `COVERED_SLUGS` list; all seven corpus files parsed
  and carried BASEAB+SAVE formulas — none silently dropped). Command:
  `cargo test --lib crb_untabled_class_chassis::tests::all_seven_covered_classes_resolve_a_real_chassis_at_level_one`.
- **CRB prestige classes registered from the entry-gate registry:** `10 of
  10` (denominator: `prestige_class_entry_gate::prestige_class_entry_requirements()`
  filtered to `source_book == "core_rulebook"`). Command:
  `cargo test --bin v06_work_inventory modelled_class_books_registry_tests::all_ten_crb_prestige_classes_are_registered_from_the_entry_gate_registry_itself`.

## Row-count command output (this cycle's own artifact)

```
$ python3 scripts/completion_atlas.py --book core_rulebook --check
book=core_rulebook population=6701 unclassified=0 overlap=0
  DONE: 1202
  A: 0
  B: 995
  C: 428
  D: 349
  M: 929
  V: 2734
  U: 58
  X: 6
  Z: 0
```

Before this cycle (parent SHA `ae25d75d7d`, same command): `B: 996`. This
cycle's own mechanism, re-derived directly against `docs/work-inventory.json`:

```
$ python3 -c "
import json
with open('docs/work-inventory.json') as f: units = json.load(f)['units']
from collections import Counter
c = Counter()
for u in units:
    if u.get('book')=='core_rulebook' and u.get('status')=='engine-does-not-hold':
        c[u.get('evidence')] += 1
print('class_absent_from_ClassId_ALL_and_book_class_id_enums:', c.get('class_absent_from_ClassId_ALL_and_book_class_id_enums', 0))
"
class_absent_from_ClassId_ALL_and_book_class_id_enums: 0
```

## Build scope verified

Run at this cycle's final commit (SHA filled in the follow-up docs commit):
- `cargo test --locked --no-run` (full workspace) → exit 0.
- `cargo test --locked --lib` → **2,863 passed; 0 failed; 14 ignored**
  (includes the F1 re-pin fix above).
- `cargo test --bin v06_work_inventory` → **374 passed; 0 failed**.
- `cargo test --locked --no-fail-fast` (full workspace, targets-executed
  counted, `decisions.md §10`): **600 of 600 targets executed, 32 of 600
  suites failing, 8,072 tests total (8,021 passed, 51 failed).** Every one
  of the 32 failing suites re-verified pre-existing: a clean worktree at
  this cycle's own start SHA `ae25d75d7d`, with zero of this cycle's edits,
  reproduces the identical failure in every one of them (`ingest_races`,
  `duergar_invisibility_sla_reaches_a_player_via_monster_codex`,
  `formula_interpreter_family_fixture_check`, `no_foreign_home_paths`,
  `pi_table_sweep`, the three `sd13_sorcerer_*` suites, the ten
  `sd18_cleric_level{11..20}_widening` suites, `sd24_identifier_discipline_
  audit`, `sd24_wired_integration_audit`, `sd26_cache_{acg,apg}`,
  `sd26_identifier_discipline_audit`, `sd27_ability_automatic_granted_race_
  traits`, `sd27_advanced_race_guide_cache_shape`, `sd27_alternate_racial_
  trait_reachability`, `sd27_book_license_record_counts`,
  `sd27_equipment_modifier_price_matches_corpus_cost_token`,
  `sd27_known_spells_must_be_on_the_class_spell_list`,
  `sd30_declared_product_identity_in_shipped_class_features`,
  `sd31_class_feature_corpus_key_uniqueness`, `v06_corpus_trap_report`).
  **This cycle's own attribution claim, proven by execution against the cut
  SHA, not assumed** (`decisions.md §10`'s second lesson). The documented
  inherited baseline (`workflow-instruction.md` item 10, `29 of 599` suites
  `/ 46 of 8,034` tests) has grown to **32 of 600 / 51 of 8,072** across the
  intervening SD-34 cycles since that baseline was last measured — a
  drift this cycle discovered and reports honestly, not one it caused
  (confirmed identical at `ae25d75d7d`, before this cycle's own commit).
  Fixing that drift is out of this cycle's own scope (a different
  mechanism/criterion's territory); AT-34-E6-001's own closure scan is
  where the bundle's inherited-baseline figure gets formally re-derived.
- `apps/desktop/src-tauri`: not touched by this cycle's file-touch set — not
  run, per `decisions.md §10`'s "explicitly, or not at all" rule for an
  untouched separate workspace.

## Sweep population

`corpus_literal_sweep`: before `48,708 examined of 51,482 read` (inherited
from the `race_trait_absent` cycle, unchanged baseline) → after `48,708
examined of 51,482 read` (this cycle's own fresh re-run,
`/tmp/corpus_literal_sweep_report.json`). **Delta: 0, exactly matching 0
corpus records added this cycle** — this cycle touches only Rust code and
the derived inventory JSON, never `data/corpus/`. `decisions.md §12` L8
satisfied (an unchanged population is the correct outcome for a cycle that
adds no corpus records). CLEAN, 0 findings.

## Oracle pin

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
(`scripts/pcgen-oracle-pin.env`) — every BAB/save formula string this cycle
reads is sourced from the seven CRB NPC/`Ex-*` class files and the ten
prestige class files at this pin.

## Status

- **Status:** complete — this cycle's own mechanism,
  `class_absent_from_ClassId_ALL_and_book_class_id_enums` for
  `core_rulebook`, is 0 of 17 remaining (was 17). Set from the row-count
  command output above, not a self-assessment.

## Movement, four buckets

- **Closure:** 17 units (this mechanism, `core_rulebook`'s own
  `class:{adept,arcane_archer,...}` records) move from bucket B
  (`class_absent_from_ClassId_ALL_and_book_class_id_enums`, "the engine
  models no class of this name at all") to a DIFFERENT bucket-D evidence
  (`class_modelled_but_no_observed_delta_on_the_rendered_snapshot` — the
  class is now genuinely modelled, but the class-effect probe has not yet
  observed a delta attributable to it; whether/how these seven+ten classes
  ever reach `grounded` is a different mechanism's own future work, not
  fabricated here). This is the expected, correct outcome
  (`decisions.md §2a`): a record reaching its table is not the same claim as
  it computing.
- **Reclassification:** 16 `core_rulebook` `class_feature` records (Assassin,
  Shadowdancer, Duelist, Arcane Trickster, Dragon Disciple, Pathfinder
  Chronicler, Expert — same book, same-word exact matches, not collisions)
  correctly move off `class_feature_of_unmodelled_corpus_class:<name>` onto
  `class_feature_owner_matched_by_name_but_record_not_held_by_engine`, a
  DIFFERENT bucket-B mechanism this cycle does not own and does not claim
  credit for closing. Plus the cross-book side effect documented in
  "Discoveries" above (187 units across 8 other books, 19 of them genuine
  `text-complete` unlocks, the rest evidence-string relabeling).
- **Reachability:** 0 — none of the 17 (or the 10+7 registered classes more
  broadly) becomes player-reachable this cycle. Registration in
  `modelled_class_books()` only answers "does the engine model this class at
  all," never "does a player's real choice put a magnitude on the sheet" —
  that is `class_effect_wired`'s own probe, untouched here.
- **Instrument-correction:** 1 logged as a `correction` retro event (the
  stale F1-population pin) plus 2 stale `file:line` citations this cycle's
  own edits caused and self-healed inline (`missing_engine_tables.py`,
  the accidental retired-string mention) — all in "Discoveries" above,
  none this cycle's own regression, all confirmed pre-existing or
  self-caused-and-fixed within the same cycle. A larger, separate
  citation-drift cluster in `scripts/tests/test_shape_engine_boundary.py`
  and `test_box_ledger.py` was found and confirmed pre-existing (identical
  at `ae25d75d7d`) but left unfixed as genuinely out of this mechanism's
  scope.

## Notes

**This cycle owns exactly one of nine mechanisms** (`decisions.md §14`).
AT-34-E3-001 as a whole does not close: `core_rulebook` bucket B still has
six other mechanisms outstanding (`deity_content_absent_from_deity_table_in_
core_rulebook` 21, `class_feature_option_pool_record_not_held_by_engine` 63,
`companion_absent_from_core_rulebook_companion_tables` 100,
`race_trait_race_not_modelled` 132, `class_feature_owner_matched_by_name_but_
record_not_held_by_engine` ~330+16 from this cycle's own reattribution
above — re-derive at HEAD, never carry this receipt's own number forward
(`decisions.md §12` L2), `class_feature_option_pool_record_with_magnitude_
not_held_by_engine` 333).

## Next-cycle plan

Any of the six remaining named mechanisms — cheapest-first per `progress.md`'s
own convention. `deity_content_absent_from_deity_table_in_core_rulebook` (21)
is the next-smallest; `decisions.md §14` already answers its PI-constraint
question (mask-preserving placement, no un-redaction) so it is dispatchable
without a fresh ruling.
