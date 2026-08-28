# Cycle 5 — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`companion_absent_from_core_rulebook_companion_tables` mechanism)

Mechanism owned this cycle: `companion_absent_from_core_rulebook_companion_tables` — one of the
nine bucket-B mechanisms `decisions.md §14` decomposed AT-34-E3-001 into. Four prior cycles ran
this mechanism (100 → 28, 28 → 28, 28 → 14, 14 → 2, all four receipts READ, not repeated:
`AT-34-E3-001_companion_absent_cycle_receipt.md`, `_2.md`, `_3.md`, `_4.md`). This cycle's mandate,
verbatim: build a genuine level-progression record type for the remaining 2 `cr_classes_companion.lst`
monster-class rows (`Companion`, `Shadow Companion`), verified generically against `ultimate_magic`
(3 rows) and `book_of_the_damned_volume_1` (2 rows) — cycle 4's own named second and third
consumers — or state precisely why it cannot be built safely in one cycle.

Population re-derived at HEAD (not transcribed from any prior receipt), **before** any change:

```
python3 -c "
import json
d=json.load(open('docs/work-inventory.json'))
u=d['units']
cr=[x for x in u if x['book']=='core_rulebook' and x['status']=='engine-does-not-hold' and x['evidence']=='companion_absent_from_core_rulebook_companion_tables']
print(len(cr))"
-> 2
```

Matches the fourth cycle's own after-figure exactly.

- **Commit SHA:** `<PENDING — filled in a follow-up commit>`
- **Files touched:**
  - `src/rules_core/rules_tables/companion_chassis.rs` — added `CompanionClassRecord` (a new
    record type: `key`, `output_name`, `hit_dice`, `max_level`, `type_segments`, `visible_no`,
    `source_page`, `ability_grants`, `fact_class_type`, `source_file`, `source_line`), a
    `companion_classes: &'static [CompanionClassRecord]` field on `CompanionBook` (added, `&[]`,
    to all 16 registered books except the 3 real consumers), `companion_class_resolve`, and
    module-doc updates recording the new shape. Replaced
    `companion_absent_2_sub_causes_are_named_and_sum_exactly` with
    `companion_absent_from_core_rulebook_companion_tables_reaches_zero` (proves the mechanism at
    0, and that both former keys resolve as real `CompanionClassRecord`s) and added
    `companion_class_record_generalizes_to_its_three_real_consumers` (proves the type against all
    3 named consumers, including the corpus's own bare-numbered level-advancement row shape, and
    that every OTHER registered book carries none).
  - `src/rules_core/rules_tables/crb/companion_data.rs`,
    `src/rules_core/rules_tables/ultimate_magic/companion_data.rs`,
    `src/rules_core/rules_tables/book_of_the_damned_volume_1/companion_data.rs` — regenerated via
    `python3 scripts/transcribe_companion_tables.py <book>`; each now carries a `COMPANION_CLASSES`
    static array (2/3/2 rows respectively) transcribed verbatim from the book's own
    `*_classes_companion.lst` rows. Verified byte-for-byte against a second back-to-back
    regeneration (see determinism fix below) — no further diff.
  - `src/rules_core/rules_tables/crb/mod.rs`, `.../ultimate_magic/mod.rs`,
    `.../book_of_the_damned_volume_1/mod.rs` — added `companion_classes_static()` accessor,
    updated `pub use`, updated each module's own doc comment recording that its former "CLASS rows
    NOT transcribed" bullet is now historical (round-8-era), with a pointer to re-derive the live
    count from `docs/work-inventory.json` rather than the comment.
  - `scripts/transcribe_companion_tables.py` — the class-row screen (`decisions.md §65.1` in
    SD-29's own decisions.md) changed from DROP-AND-NAME to an actual build: `class_units` gathered
    (sorted by source line, mirroring `creatures`/`abilities`), `parse_class_row`/`tokens_all`/
    `parse_hit_dice` helpers added, `COMPANION_CLASSES` emission added, module-doc block updated.
    Also fixed a **pre-existing determinism bug**, found this cycle by diffing two back-to-back
    regenerations of the SAME unmodified book before trusting any regen: Shape 7's book-wide-grant
    loop (`for key in BOOK_WIDE_GRANTS.get(book, set())`) walked a Python `set` whose iteration
    order CPython randomizes per process (`str` hash-seed randomization), reordering every touched
    creature's `ability_keys` list run to run with no corpus reason. Fixed with `sorted(...)`;
    verified deterministic across two more regenerations of all 3 target books afterward (set
    membership was never wrong — only presentation order was — so no prior committed
    `companion_data.rs` content was incorrect, only non-reproducible if regenerated again).
  - `src/bin/v06_work_inventory.rs` — `chassis_companion_keys` construction (used by the
    `Kind::Companion` dispatch arm's `holds_key` check) now folds `book.companion_classes` keys
    into the same per-book set `companions`/`companion_abilities` already share, so a class-row key
    resolves the same way the other two shapes do.
  - `scripts/completion_atlas.py` — this cycle's `v06_work_inventory.rs` insertion shifted all ten
    hardcoded `BUCKET_DEFINITIONS` citations by a uniform +8 lines; every one re-derived with
    `grep -n` against the post-edit file and fixed in this same cycle
    (`citation_failures=0` after).
  - `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs` — one pre-existing hardcoded
    population assertion (`f1_population_matches_the_current_true_formula_bearing_count_...`)
    re-pinned 5401 → 5400: `ultimate_magic:companion:black_blade` (`BONUS:HP|CURRENTMAX|5`, F1's
    own bare-literal shape) left bucket B for `literal-verified` this cycle, leaving the
    "not-done units considered" F1 population (`shape_ledger.py`'s own denominator) by exactly 1.
    Retro `correction` event emitted with `--verified-by`.
  - `docs/release/SD-34-book-completion/decisions.md` — added `§17` (the ADR for building
    `CompanionClassRecord`, citing SD-29's own §65.1 refusal and this mechanism's own cycle
    lineage as the condition that made building it safe).
  - `docs/work-inventory.json` — regenerated (`v06_work_inventory` release binary, with
    `CORPUS_LITERAL_SWEEP_REPORT` and `DERIVED_FIXTURE_CHECK_REPORT` set) — the ONLY units it moved
    are the 7 companion-class rows (verified below).
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_companion_absent_cycle_receipt_5.md` (this file)
  - `docs/retro/events/sd34-at-34-e3-001.jsonl` — one `resolution` event closing cycle 4's own
    deferral of this remainder, one `correction` event for the F1 re-pin.
  - `docs/release/SD-34-book-completion/progress.md`, `kanban.md` (prepend/update, §5)

## What was built

**A genuine third `CompanionBook` table, `companion_classes: &'static [CompanionClassRecord]`.**
`*_classes_companion.lst` rows are PCGen monster CLASS definitions — the hit-dice progression a
creature row's `MONSTERCLASS:` token names. They are neither a creature (no `SIZE:`/`MOVE:`/natural
attacks) nor an ability (no `DESC:`). SD-29's own companion-lane round 8
(`docs/release/SD-29-corpus-wide-catch-up-lanes/decisions.md §65.1`) found this shape, refused it
outright for three rounds, then widened the refusal to DROP-AND-NAME rather than model it, stating
explicitly that modelling it is "a new record type... which a round taking one should declare up
front — this round does not take it." Three later `AT-34-E3-001` cycles (this mechanism's own
cycles 2–4) re-confirmed the same finding; cycle 4 named all three real corpus-wide consumers
precisely. This cycle takes the work SD-29 declared and deferred.

**The record type, and what it deliberately does not do.** `CompanionClassRecord` carries `key`,
`output_name`, `hit_dice`, `max_level` (verbatim — `"20"`, `"NOLIMIT"`, `"1"` all occur, none fed
into a formula), `type_segments`, `visible_no`, `source_page`, `ability_grants` (every `ABILITY:`
token's payload, verbatim and in row order — the same "visible rather than lost" discipline
`CompanionAbilityRecord::type_segments` uses for an unmodelled shape), `fact_class_type`,
`source_file`, `source_line`. It computes nothing: `hit_dice`/`max_level` are never fed into a BAB,
save or hit-point formula, the same discipline `CompanionRecord::monster_class`'s own doc states
for the identical shape read from the creature side. Reaching `grounded` (bucket B → D/M/V)
settles nothing about whether the record is later computed or displayed — that is a different
mechanism's job (`decisions.md §2a`).

**The second, unplanned shape this file also carries: bare-numbered level-advancement lines.**
`um_classes_companion.lst:13` and `botd1_classes_companion.lst:8` are both single lines under a
`###Block: Level Advancement` comment — e.g. `1<TAB>ABILITY:FEAT|AUTOMATIC|CMB Output` — with no
`CLASS:` prefix at all. `v06_work_inventory::enumerate_file`'s own directive screen only skips a
first field shaped `TOKEN:` (all-caps/digits before a colon); a bare `1` has no colon, so it is
never treated as a directive and becomes its own record, keyed `"1"`, under `Kind::Companion`
(same file, same kind, per `row_shape`). `CompanionClassRecord` models this uniformly with the
`CLASS:` header shape (every field but `ability_grants` empty for this row) rather than needing a
fourth type — proven correct by `companion_class_record_generalizes_to_its_three_real_consumers`,
which asserts the bare-numbered row's exact fields for both `ultimate_magic` and
`book_of_the_damned_volume_1`.

**Verified against all three named consumers, generically — not ad hoc for `core_rulebook`.**
`scripts/transcribe_companion_tables.py`'s class-row build is the SAME code path run once per book
(`core_rulebook`, `ultimate_magic`, `book_of_the_damned_volume_1`), producing 2/3/2 rows
respectively, matching each book's own raw `.lst` content exactly (re-checked by hand against the
raw file for every field of every row before trusting the generated output). This is the "build it
generically... or state precisely why not" the mandate asked for — taken, not declined.

**The determinism bug, found and fixed before it could corrupt the regen.** Running the transcriber
once and inspecting `git diff` looked clean; running it a SECOND time on the SAME unmodified
book produced a large, spurious diff — every `ability_keys` list on every Core Rulebook creature
record reshuffled (same 76 elements, different order), because Shape 7's book-wide-grant loop
iterated a Python `set` whose order CPython randomizes per process. Caught by the discipline of
running the regen twice before trusting it (never assumed from one green run), not by any existing
test. Fixed with `sorted(...)` on that one loop; the two other set-shaped structures nearby
(`owners[key]` built via `sorted(creature_keys)`, `desc_redacted` used only for membership tests)
were already immune. Verified deterministic with two more regenerations of all 3 target books
after the fix. This was a real, pre-existing generator defect, not something my class-row change
introduced — but it would have poisoned every diff this cycle produced if left unfixed, so it is
fixed in this same cycle rather than reported and left for a successor.

## Row-count command output (before → after, this mechanism)

```
BEFORE: 2   (companion_absent_from_core_rulebook_companion_tables, core_rulebook, engine-does-not-hold)
AFTER:  0   (both keys now `grounded`, resolved through companion_class_resolve)
```

Re-derive command (same as above, run against HEAD after this cycle's regen):
`python3 -c "import json; d=json.load(open('docs/work-inventory.json')); u=d['units']; print(len([x for x in u if x['book']=='core_rulebook' and x['status']=='engine-does-not-hold' and x['evidence']=='companion_absent_from_core_rulebook_companion_tables']))"` -> `0`

**This mechanism reaches ZERO.** AT-34-E3-001 itself remains open — the other eight bucket-B
mechanisms `decisions.md §14` named are owned by other cycles, and `core_rulebook` bucket B is
562 of 6701 (below), not zero.

## Figures + re-derive commands

| Figure | Value | Command |
|---|---:|---|
| Mechanism population (before) | 2 of 2 | see above |
| Mechanism population (after) | 0 of 2 | see above |
| Units closed this cycle, this mechanism | 2 | 2 - 0 |
| `CompanionClassRecord` rows registered, all 3 consumers | 7 (2 + 3 + 2) | `python3 -c "import json;d=json.load(open('docs/work-inventory.json'));u=d['units'];print(len([x for x in u if x.get('source_file') in ('cr_classes_companion.lst','um_classes_companion.lst','botd1_classes_companion.lst')]))"` -> `7` |
| Destination status of the 7 rows (corpus-wide side effect, all 3 books) | `grounded` (6) / `literal-verified` (1, `ultimate_magic` Black Blade) | `git diff docs/work-inventory.json \| grep -E '^[+-]    \{' \| grep -o '"status": "[^"]*"'` |
| `core_rulebook` companion units by status (all statuses) | 184 total, 0 in bucket B | `python3 -c "import json;d=json.load(open('docs/work-inventory.json'));import collections;print(collections.Counter(x['status'] for x in d['units'] if x['book']=='core_rulebook' and x['kind']=='companion'))"` |
| `completion_atlas.py` full check | population=49438 buckets=10 unclassified=0 overlap=0, citation_failures=0 | `python3 scripts/completion_atlas.py --check` |
| `completion_atlas.py` bucket B, corpus-wide (before this cycle's regen, re-derived by temporarily swapping in the pre-cycle `docs/work-inventory.json`) | 12002 | see Notes — temp-swap method |
| `completion_atlas.py` bucket B, corpus-wide (after) | 11995 (delta -7, exactly the 7 rows this cycle moved) | `python3 scripts/completion_atlas.py --check` |
| `completion_atlas.py --book core_rulebook` bucket B | 562 of 6701 (owned by the other 8 AT-34-E3-001 mechanisms — not this cycle's to close) | `python3 scripts/completion_atlas.py --book core_rulebook --check` |
| Total corpus population (unchanged by this cycle) | 49,438 | `python3 -c "import json;print(json.load(open('docs/work-inventory.json'))['totals']['units'] if 'totals' in json.load(open('docs/work-inventory.json')) else len(json.load(open('docs/work-inventory.json'))['units']))"` |
| `corpus_literal_sweep` examined population | 48,708 of 51,482, CLEAN, 0 findings (unchanged — this cycle adds 0 `data/corpus` records, only Rust static tables) | `/tmp/cargo-sd34-at-34-e3-001/release/corpus_literal_sweep --json-out /tmp/sd34-e3-001-reports/sweep_before.json` |
| `derived_evaluator_fixture_check` | 1,839 units cleared over 2,580 fixture rows, 0 failed | `/tmp/cargo-sd34-at-34-e3-001/release/derived_evaluator_fixture_check --json-out /tmp/sd34-e3-001-reports/derived.json` |
| `shape_ledger.py` F1 population (before → after) | 5401 → 5400 | `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus` |
| `denominator_gate.py` against this package | violations=0 | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` |

## Verification

- TDD: RED confirmed for `companion_absent_from_core_rulebook_companion_tables_reaches_zero`
  immediately after the `companion_chassis.rs`/`v06_work_inventory.rs`/transcriber edits and
  BEFORE regenerating `docs/work-inventory.json` — failed for the intended reason (`left: 2,
  right: 0`, the JSON inventory still reflected the pre-fix world). GREEN after the inventory
  regen: `cargo test --lib rules_core::rules_tables::companion_chassis` — **19 passed, 0 failed.**
- Full `cargo test --lib` (workspace lib target): **2,896 passed, 0 failed, 14 ignored** (one
  failure surfaced mid-cycle — the stale F1 pin — diagnosed to this cycle's own status movement,
  re-derived via `shape_ledger.py`, re-pinned, and re-run green; see Notes).
- `cargo test --bin v06_work_inventory`: **397 passed, 0 failed** (up from cycle 4's 395 — the 2
  new companion_chassis tests are `--lib`, not this binary's own suite; the +2 here are pre-existing
  count drift from other concurrent cycles' own already-merged work between cycle 4 and this one).
- **Build scope verified:** `cargo test --locked --no-run` (workspace, all bin/test targets) exits
  0. `apps/desktop/src-tauri` (separate cargo workspace) also run explicitly this cycle:
  `cargo test --locked --no-run` exits 0 (`CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-001-desktop`).
  Both run AFTER the last commit in this cycle that can move a figure this receipt depends on
  (`decisions.md §12` L7) — i.e. after the `docs/work-inventory.json` regen and the F1 re-pin.
- `python3 scripts/completion_atlas.py --check`: `citation_failures=0` — this cycle's own
  `v06_work_inventory.rs` insertion shifted all ten hardcoded citations by a uniform +8 lines;
  each re-derived by `grep -n` against the post-edit file and fixed in this same cycle, per the
  criterion's own line-number-drift warning.
- `docs/work-inventory.json` regen touched ONLY the 7 companion-class units — verified by diffing
  every `"id"` the regen changed on an added/removed line
  (`git diff docs/work-inventory.json | grep -E '^[+-]    \{' | grep -o '"id": "[^"]*"' | sort -u`)
  against the 7 expected ids; they match exactly.
- Identifier audit (this cycle's own working-tree diff, all touched files): `OK_NO_BUNDLE_TAGS`.
- Wired-integration audit (same own-diff scope): `OK_NO_TOKENS`. The epic's CUMULATIVE
  `${BASE_BRANCH}...HEAD` diff over the same file-touch set still shows `placeholder` matches, all
  from EARLIER, already-merged AT-34-E3-001 sub-cycles' vacuous-placeholder-class-feature work
  (real corpus content and a real, already-merged named list, not an implementation stub, and not
  this cycle's own diff — same precedent cycles 3 and 4's own receipts already recorded).

## Movement, four buckets

- **Closure:** 6 units reached `grounded` this cycle (`core_rulebook` `Companion`/`Shadow
  Companion`; `ultimate_magic` `Vermin Companion`/`"1"`; `book_of_the_damned_volume_1` `Imp
  Companion`/`"1"`) — a real bucket B → D-tier close, the engine now holds every one of these
  records. 1 more (`ultimate_magic` `Black Blade`) reached `literal-verified` (bucket B → V),
  riding `corpus_literal_sweep`'s existing CLEAN sweep because its `wiring_class` is `static`.
- **Reclassification:** 0. Every closed unit is a genuine placement (a real, tested `Rust` record
  now backs it), never a shape-only relabel.
- **Reachability:** 0 new `reach_gate` findings — none of these 7 rows carries a `DESC:` token, so
  there is nothing new for any render path to reach; these are chassis/shelf records (bucket D/V),
  not display records.
- **Instrument-correction:** 1 (the F1 population re-pin, 5401 → 5400) — a real movement (this
  cycle's own status change caused it), re-derived via `shape_ledger.py` and logged with a retro
  `correction` event, not a count-only fix to a defect.

- **Status:** complete

## Notes

- **Bucket B corpus-wide before/after, re-derived by temporarily swapping `docs/work-inventory.json`
  for its pre-cycle content** (`git show HEAD:docs/work-inventory.json` into a scratch file,
  swapped in, `--check` run, swapped back to the regenerated file, `--check` re-run) —
  12002 → 11995, delta 7, exactly the 7 rows this cycle moved. The original file was restored
  before any commit; `git status --porcelain docs/work-inventory.json` confirms only the intended
  regenerated content is staged.
- **5 of the 7 closed rows are NOT this mechanism's own population** — `ultimate_magic`'s 3 and
  `book_of_the_damned_volume_1`'s 2 belong to THOSE books' own `companion_absent_from_
  <book>_companion_tables` mechanisms, owned by other cycles/epics, not AT-34-E3-001 (which is
  `core_rulebook`-scoped only). Closing them is a legitimate, honestly-reported side effect of
  building the type generically (`decisions.md §17`'s own reasoning: verifying against all three
  named consumers in one cycle, not stealing another lane's scope) — reported here for honesty,
  not claimed as this criterion's own progress.
- `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` was NOT
  committed this cycle — it is Epic 1's own artifact (not one of SD-34's three shared files per
  the workflow instruction), it was already dirty from another concurrent lane before this cycle
  started (`git status --porcelain` checked first, per shared-checkout discipline), and running
  `completion_atlas.py --check` for verification purposes further updated it on disk as a
  deterministic side effect of reading the live `docs/work-inventory.json` — left uncommitted and
  untouched by this cycle's own `git add`.
- `retro.py`'s `deferrals.open` field is trustworthy as of SD-32's fix — confirmed this cycle:
  `grep -n 'len(open_deferrals)' scripts/retro.py` -> line 772.
- SD-29's own decisions.md (`docs/release/SD-29-corpus-wide-catch-up-lanes/decisions.md`) already
  has entries numbered up to `§70` (that bundle is closed); this cycle's new ADR is `§17` in
  SD-34's OWN `decisions.md`, not a new SD-29 section — an earlier draft of this cycle's own code
  comments mistakenly cited a bare `decisions.md §68`, which collides with a REAL, unrelated §68
  already in SD-29's decisions.md (`68.3`–`68.6`, ingest-monster-count findings). Caught and fixed
  before commit by checking SD-29's own decisions.md contents rather than assuming an unclaimed
  number.

## Next-cycle plan

This mechanism (`companion_absent_from_core_rulebook_companion_tables`) is CLOSED at 0. The
`core_rulebook` book-level bucket B is 562 of 6701, owned by the 8 other AT-34-E3-001 mechanisms
(`class_feature_option_pool_record_not_held_by_engine` 44, `class_feature_option_pool_record_with_
magnitude_not_held_by_engine` 267, `class_feature_owner_matched_by_name_but_record_not_held_by_
engine` 251, and 5 already-fully-closed mechanisms per the kanban row's own history — re-derive the
live per-mechanism split fresh rather than trusting this list, since other cycles may have moved
figures concurrently). AT-34-E3-001 itself remains open until bucket B reaches 0 for the whole
book — the criterion's own closing condition, owned by whichever cycle closes the LAST mechanism.
