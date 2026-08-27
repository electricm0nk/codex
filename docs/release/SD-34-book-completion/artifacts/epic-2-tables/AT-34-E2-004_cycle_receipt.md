# Cycle 12 — Epic 2 (Build eight of the nine tables) / AT-34-E2-004

- **Commit SHA:** `0dd52ccb65`
- **Files touched:** `src/bin/v06_work_inventory.rs` (wires the seven `AT-34-E2-001` tables plus
  `companion`'s pre-existing SD-29 table into `classify()` for real classification, not just the
  read-only `--epic2-table-transcript` diagnostic), `scripts/completion_atlas.py` (10 bucket
  citation line numbers re-pinned — my edit shifted every line below it in the same file),
  `scripts/missing_engine_tables.py` (`ENGINE_SURFACE_CITATIONS` trimmed from 9 entries to the 2
  still live — `companion`, `power` — the other 7 kinds can no longer emit `has_no_engine_table`
  at all, so a citation pointing at that marker can never resolve again),
  `scripts/tests/test_completion_atlas.py` and `scripts/tests/test_missing_engine_tables.py`
  (pinned-figure tests re-derived against the new live population), `docs/work-inventory.json`
  (regenerated at HEAD — required to prove the criterion; not in Epic 2's own file-touch set,
  but Epic 1's touch-set entry declares it "regenerates" and no epic file-touch row forbids a
  later epic from re-running its own writer instrument), this receipt, `kanban.md`, `progress.md`.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "**Evidence:** `python3
  scripts/completion_atlas.py --book core_rulebook --check` and `--book ultimate_campaign
  --check` each report bucket A at zero, with movement stated in four buckets (closure /
  reclassification / reachability / instrument-correction)."

## What this cycle did

`AT-34-E2-001`/`AT-34-E2-002` built the seven `simple_kind_tables` resolvers and proved them
fail-closed, but wired them only into a read-only `--epic2-table-transcript` diagnostic —
`classify()`'s real per-unit verdict arms for `Kind::Skill`/`Template`/`Deity`/`Domain`/
`Language`/`Ability`/`Trait` still unconditionally returned
`engine_does_not_hold("<kind>_content_has_no_engine_table")` regardless of the table. This cycle
wires the tables into `classify()` itself:

- **Held + zero-magnitude + real description + `display` wiring class + not a universal sheet
  modifier** → `text-complete` (the same promotion gate every other kind's zero-magnitude rung in
  this file already uses — no new rule invented).
- **Held + carries a real magnitude** → `ingested-magnitude` (bucket M), never `grounded` —
  `decisions.md §2a`: a corpus lookup table computes nothing; crediting a magnitude-bearing
  record as fully done because a description-only lookup found it would be exactly the
  over-claim that decision warns against.
- **Held but neither of the above** (structurally excluded from both gates) → falls to bucket D,
  honestly, rather than forced into either shape.
- **Not held** → `engine_does_not_hold("<kind>_absent_from_<dir>_table_in_<book>")` — bucket B,
  never A.

Shared logic extracted to `simple_kind_verdict()`, called once per kind with the kind's own
label/table.

### Discovery: a companion misattribution, fixed

`core_rulebook` reported 14 `companion` units still in bucket A even after all the above — the
`Familiar ~ …` shape (`ce_abilities_familiar_cr.lst`, 14 keys), real, named,
**deliberately-excluded** content per `crb::companion_data`'s own "NOT transcribed" list (ability
rows no creature row of that book owns). Root cause: these rows physically live under
`core_essentials` (a companion registry `SD31-CE-COMPANION-001` retired), and `classify()`'s
general re-attribution widening (`decisions.md §9`) only re-homes a unit to its reported book
when that book's table **observably holds** it — which these rows, by design, do not. So
`engine_book` stayed `core_essentials`, which has no companion chassis entry at all, and the
`Kind::Companion` fallback arm fired: `companion_content_has_no_engine_table` — bucket A, "no
table for this kind," even though `core_rulebook` genuinely has one and simply does not hold this
row.

Fixed with a new, narrowly-scoped `Kind::Companion` guard: when the reported book differs from
the resolved `engine_book` **and** the reported book's own chassis registration exists, report
`companion_absent_from_<book>_companion_tables` (bucket B, the same evidence shape the
registry-driven arm above already emits) instead of falling to the no-table arm. Verified by two
new unit tests (`a_companion_reattributed_to_a_chassis_book_that_does_not_hold_it_is_bucket_b_not_a`,
its sibling proof `..._stays_bucket_a` pinning the arm does NOT fire when the reported book truly
has no table either).

Retro `correction` event emitted (`RETRO_ACTOR=sd34-at-34-e2-004`,
`docs/retro/events/sd34-at-34-e2-004.jsonl`).

### Known limitation, NOT fixed this cycle (flagged for Epic 3)

29 more `core_rulebook` units (7 `ability` + 22 `template`) show evidence
`..._absent_from_<kind>_table_in_core_essentials` — the identical root-cause shape as the
companion case above, for kinds whose `holds_key_inner` (the general classify() reattribution's
own holds-check) has no match arm at all, so the general widening can never even attempt a
re-home for them. These are correctly **off bucket A** (they land in bucket B, which is what this
criterion requires), but their `engine_book` may still be wrong for the SAME reason the companion
case was — meaning some of them may in fact be held under `core_rulebook`'s real ability/template
table and are being reported `absent` under the wrong book's name instead. This does not block
`AT-34-E2-004` (bucket A is zero either way) and is exactly the shape `AT-34-E3-001` ("bucket B
closes: records reach their tables … the mechanism that placed them named") exists to run down —
named here so it is not rediscovered from scratch.

## RED → GREEN

RED (before this cycle's fix, confirmed against the committed inventory + the seven tables):
`facts.simple_kind_tables` did not exist; every `Kind::Ability`/`Template`/`Trait`/`Deity`/
`Domain`/`Skill`/`Language` unit read `engine-does-not-hold` /
`<kind>_content_has_no_engine_table` unconditionally, and
`a_held_zero_magnitude_ability_record_promotes_to_text_complete` (written first) failed to
compile / failed the assertion against the old arm.

GREEN: 8 new unit tests added (`a_held_zero_magnitude_ability_record_promotes_to_text_complete`,
`a_held_ability_record_with_a_real_magnitude_is_ingested_magnitude_not_grounded`,
`an_ability_record_absent_from_the_table_is_bucket_b_not_a`,
`an_ability_record_with_no_table_loaded_at_all_is_bucket_b_not_a_and_does_not_panic`,
`a_held_trait_record_through_the_trait_generic_directory_promotes_to_text_complete`,
`a_companion_reattributed_to_a_chassis_book_that_does_not_hold_it_is_bucket_b_not_a`,
`a_companion_reattributed_to_a_book_with_no_chassis_table_stays_bucket_a`, plus the pre-existing
companion-rung suite), all pass. Full `v06_work_inventory` bin suite: 366 passed, 0 failed. Its
own integration suite (`--test v06_work_inventory`): 16 passed, 1 ignored (pre-existing,
unrelated — the double-run timestamp-only test), 0 failed.

## Figures + their re-derive commands (every one, with its denominator)

| Figure | Command | Denominator |
|---|---|---|
| `core_rulebook` bucket A: `934 → 0` | `python3 scripts/completion_atlas.py --book core_rulebook --check` | of 6,701 `core_rulebook` units |
| `ultimate_campaign` bucket A: `242 → 0` | `python3 scripts/completion_atlas.py --book ultimate_campaign --check` | of 265 `ultimate_campaign` units |
| Corpus-wide bucket A: `8463 → 449` | `python3 scripts/completion_atlas.py --check` | of 49,438 total units |
| Remaining bucket-A population: `power=421, companion=28` | `python3 scripts/missing_engine_tables.py --check` | of the 449 units still in bucket A corpus-wide |
| `core_rulebook` per-bucket after: `DONE=1165 A=0 B=1035 C=370 D=412 M=921 V=2734 U=58 X=6 Z=0` | `python3 scripts/completion_atlas.py --book core_rulebook --check` | of 6,701 |
| `ultimate_campaign` per-bucket after: `DONE=127 A=0 B=5 C=0 D=4 M=88 V=18 U=21 X=2 Z=0` | `python3 scripts/completion_atlas.py --book ultimate_campaign --check` | of 265 |
| Corpus-wide per-bucket before: `DONE=12265 A=8463 B=11921 C=4388 D=1230 M=2455 V=8330 U=321 X=46 Z=19` | `git show HEAD~1:docs/work-inventory.json` piped through the same `_bucket_of` logic (see progress.md's own recorded baseline, identical) | of 49,438 |
| Corpus-wide per-bucket after: `DONE=13744 A=449 B=14418 C=4388 D=2249 M=4471 V=9333 U=321 X=46 Z=19` | `python3 scripts/completion_atlas.py --check` | of 49,438 |
| 8 of 8 new unit tests pass | `cargo test --bin v06_work_inventory ability_record trait_record` (+ the two companion tests, run individually) | this cycle's own new test set |
| `366` `v06_work_inventory` bin tests pass, 0 fail | `cargo test --bin v06_work_inventory` | full bin test module |
| `16` `v06_work_inventory` integration tests pass, 1 ignored (pre-existing), 0 fail | `cargo test --test v06_work_inventory` | that file's own test count |
| `corpus_literal_sweep`: `48699 → 48699`, delta `0` | `cargo run --locked --bin corpus_literal_sweep` (run once, before the inventory regen that consumed its `--json-out` report — this cycle touched no `data/corpus/**` file) | of 51,473 total corpus files |
| `38` `scripts.tests.test_completion_atlas` tests pass | `python3 -m unittest scripts.tests.test_completion_atlas -v` | full module |
| `12` `scripts.tests.test_missing_engine_tables` tests pass | `python3 -m unittest scripts.tests.test_missing_engine_tables -v` | full module |
| `denominator_gate` clean on this package | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` → `files_checked=15 violations=0` | this package's markdown files |
| `box_ledger.py --check` still exits 0 (inherited, read-only) | `python3 scripts/box_ledger.py --check` | of 49,438 (unchanged partition; `uncovered=21504` — pre-existing drift from `THE-BOX.md` not being re-derived this bundle, not introduced by this cycle) |

## Row-count command output

```
$ python3 scripts/completion_atlas.py --book core_rulebook --check
book=core_rulebook population=6701 unclassified=0 overlap=0
  DONE: 1165
  A: 0
  B: 1035
  C: 370
  D: 412
  M: 921
  V: 2734
  U: 58
  X: 6
  Z: 0

$ python3 scripts/completion_atlas.py --book ultimate_campaign --check
book=ultimate_campaign population=265 unclassified=0 overlap=0
  DONE: 127
  A: 0
  B: 5
  C: 0
  D: 4
  M: 88
  V: 18
  U: 21
  X: 2
  Z: 0
```

Bucket A reads `0` for both books, directly off the count — not a self-assessment. (The command's
own exit code is `1` for a `--book --check` run, because that mode's exit condition is "every
non-DONE bucket is zero," a full-book-closure bar `epic-breakdown.md` assigns to `AT-34-E3-005`/
Epic 4, not to this criterion. `acceptance-and-verification.md`'s row for `AT-34-E2-004` names the
evidence as "bucket A at 0," which the printed counts satisfy exactly; `§5` of that same document
warns against narrowing a gate to pass, so this note states the exit-code shape plainly rather
than silently reporting only the exit code.)

## Build scope verified

- `cargo test --locked --no-run` (workspace root, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e2-004`):
  exit 0, every test executable listed built (600 executables, matching AT-34-E2-003's own
  count), zero `error` lines.
- `apps/desktop/src-tauri` (separate cargo workspace), `cargo test --locked --no-run`
  (`CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e2-004-desktop`): exit 0, one pre-existing `dead_code`
  warning (`SavedCharacterMutationOpDescriptor.op`, unrelated field), zero errors.
- Run at HEAD `1bc9884acf86b53a2b409aa21101df53988aa95b` plus this cycle's uncommitted diff (both
  scopes re-run after the last commit that can move a figure this receipt depends on —
  `decisions.md §12` L7 — since the inventory regeneration is itself part of this cycle's last
  move).

## Sweep population

`corpus_literal_sweep`: **48699 examined before → 48699 examined after**, delta `0`. Correct:
this cycle added or removed zero `data/corpus/**` records (`git status --porcelain data/corpus`
empty both before and after) — only `src/bin/v06_work_inventory.rs` (code), two `scripts/*.py`
instruments, their tests, and the derived `docs/work-inventory.json` changed. `0` findings,
CLEAN both times (`corpus-literal-sweep: CLEAN`, `"clean":true` in the `--json-out` report used
to regenerate the inventory without stamp loss).

## Oracle pin

N/A — no figure in this receipt was derived from the pinned PCGen oracle corpus; bucket V units
remain proxy-verified only, unchanged in kind by this cycle (more units REACHED V, none left it).

- **Status:** complete
- **Movement, four buckets:**
  - **Closure:** `142` units reached `DONE` this cycle (`core_rulebook` `+15`, `ultimate_campaign`
    `+127`) — genuinely zero-magnitude, real-description, `display`-class records the seven
    tables now resolve and promote to `text-complete`, or (for the companion fix) a
    misattributed-but-genuinely-registry-matched record reclassified correctly.
  - **Reclassification:** `1,034` units moved off bucket A into a still-open bucket without
    reaching `DONE` (`core_rulebook` `919` = `934 − 15`; `ultimate_campaign` `115` = `242 − 127`)
    — corpus-wide, `A` fell by `8,014` (`8463 → 449`), landing as `B +2,497`, `D +1,019`,
    `M +2,016`, `V +1,003`, `DONE +1,479` (these two books' `+142` is the `DONE` slice of that
    corpus-wide `+1,479`; the rest belongs to the other 35 books the same code change reaches).
  - **Reachability:** N/A — this cycle wires table *resolution*, not a display/explanation path;
    bucket C (held-and-computed-but-unsurfaced) is untouched (`370 → 370` for `core_rulebook`),
    exactly as expected — that is `AT-34-E3-002`'s criterion, not this one's.
  - **Instrument-correction:** `scripts/completion_atlas.py`'s 10 bucket citations and
    `scripts/missing_engine_tables.py`'s 9 kind citations all pointed at line numbers this
    cycle's own source edit shifted (a large insertion above them moved every later line by a
    constant offset, then the companion/kind-arm rewrite shifted everything after that by a
    second, larger amount) — both instruments' `--check` failed closed on the mismatch exactly as
    designed (`AT-34-E1-002` condition 6 / its `missing_engine_tables.py` sibling), and both are
    re-pinned against the real, current file. `missing_engine_tables.py`'s
    `ENGINE_SURFACE_CITATIONS` additionally lost 7 entries whose marker text no longer exists
    anywhere in the source (the 7 kinds this cycle closed can never re-emit
    `has_no_engine_table`), rather than leaving them permanently unresolvable.
- **Notes:**
  - The companion-misattribution fix and the flagged-but-unfixed ability/template sibling shape
    are both genuinely **discoveries**, not predicted by any prior cycle's plan — reported per
    `decisions.md §2`'s rule that an unpredicted step is examined honestly rather than folded
    silently into "building the table."
  - `docs/retro/events/sd31-transcribe.jsonl` observed dirty in this shared checkout at cycle
    start (another lane's file, per `workflow-instruction.md`'s standing note) — left untouched,
    confirmed via `git status --porcelain` immediately before every git write this cycle.
  - `docs/work-inventory.json`'s regeneration required `CORPUS_LITERAL_SWEEP_REPORT` and
    `DERIVED_FIXTURE_CHECK_REPORT` (fresh `--json-out` reports from `corpus_literal_sweep` and
    `derived_evaluator_fixture_check`, run first) to avoid the stamp-loss guard's refusal — a bare
    regen with neither set would have refused to write (`--allow-stamp-loss` was never used, per
    standing instruction). The `V` bucket's `+1,003` corpus-wide growth is a direct, attributable
    consequence of this cycle's own classify() fix: the guard only upgrades a unit already at
    `ingested-magnitude`/`grounded`/`text-complete` with `wiring_class == Static`/`Derived`, and
    these units were at `engine-does-not-hold` (ineligible) before this cycle regardless of which
    reports were supplied.
- **Next-cycle plan:** Epic 2 is closed (4 of 4 criteria: `AT-34-E2-001..004`). Epic 3
  (`AT-34-E3-001`, Core Rulebook bucket B closure) picks up next, and should examine the 29
  `core_essentials`-misattributed `ability`/`template` `core_rulebook` units flagged above as part
  of its "mechanism, not per record" placement work.
