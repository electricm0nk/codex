---
canonical: true
owner: god-emporer
bundle_id: SD-35
date: 2026-09-07
---

# SD-35 Epic Breakdown — 7 epics, 30 criteria

Criterion IDs follow the program convention `AT-35-E<epic>-<nnn>`. Every criterion states its
**evidence obligation** — the command or artifact that proves it.

**Standing rules that bind every criterion:**

- **The sheet rule** (`decisions.md §1`): DONE is a rendered sheet line — final number, dice, or
  words. No per-unit proof machinery.
- **No PCGen in live code** (`decisions.md §11`): PCGen is read by the converter and the test
  oracle only. `scripts/pcgen_residue_gate.py` never increases; it reads zero at closure.
- **The batch floor** (`decisions.md §2`): a cycle is one mechanism corpus-wide, 500 units
  minimum, `scripts/cycle_scope_gate.py` output in the receipt.
- **One build per cycle** (`decisions.md §3`); the full gate once per epic.
- **Every figure states its denominator** in the same construct (`decisions.md §8`).
- **Only a move into DONE is closure** (`decisions.md §9` L10); a bucket-to-bucket move is a
  relabel.

Populations below are at 2026-09-07 HEAD `5f6b18f4e3` and are re-derived at each epic's
dispatch. Epic 2's corpus-wide conversion is expected to shrink every later epic's population
before it starts; a criterion's bar is "to zero", not "this many".

---

## Epic 1 — Tax cut

**Gated on:** launch gates. **Gates:** Epic 2. Every later build pays less after this epic, and
the two counters that must only go down (batch size up, PCGen residue down) exist from the
first cycle, and SD-34's unrun closure is cleared. **Parallel:** three file-disjoint lanes —
`E1-003` (`tests/`), `E1-006` (`docs/retro/` + two `references/README.md`), and
`E1-001`/`E1-002`/`E1-005` (`scripts/`); `E1-004` after all three (`workflow-instruction.md §3`).

### AT-35-E1-001 — the batch floor is a script with a nonzero exit

`scripts/cycle_scope_gate.py --min 500 <scope flags>` reads `docs/work-inventory.json`, applies
the scope filter (any combination of `--bucket`, `--kind`, `--evidence-prefix`, `--token`,
`--book`), prints the scoped population **and** the whole remaining non-DONE population, and
exits non-zero when the scoped population is under the floor and is not the whole remainder.
`--receipt --before <inventory> --after <inventory>` computes units closed (id-set moved into
DONE), units relabeled, `rust_lines_changed / units_closed` from `git diff --stat`, the
live-side PCGen file count from `pcgen_residue_gate.py`, and prints the receipt rows of
`workflow-instruction.md §7` verbatim.

**Evidence:** RED→GREEN — a 12-unit scope exits 1; a 500-unit scope exits 0; a 12-unit scope
that is the entire remainder exits 0. `scripts/tests/test_cycle_scope_gate.py` covers all three.
Wired into `scripts/verify.sh` as `cycle-scope-gate-selftest`.

### AT-35-E1-002 — citation pins are content-anchored, and every `--check` is a `verify.sh` stage

`scripts/completion_atlas.py`, `scripts/shape_engine_boundary.py`, and
`scripts/missing_engine_tables.py` pin `file:line` citations into `src/bin/v06_work_inventory.rs`.
Every code edit shifts them; wave 51 found two of three drifted at HEAD. Replace the line-number
pin with a **content anchor** — the exact source lines the bucket keys on, inside a named
function — resolved by search at check time, so a refactor that moves code keeps the citation
green and a change to the cited content still fails closed (SD-34 AT-34-E1-002 condition 6
preserved). Same cycle: add `--by-kind` and `--by-evidence` output modes to
`completion_atlas.py` (only `--check`, `--by-book`, `--book` exist at authoring).

Wire `shape_engine_boundary.py --check`, `missing_engine_tables.py --check`, and
`scripts/tests/test_shape_engine_boundary.py` into `scripts/verify.sh` as real stages, and give
`site-dashboard-check` the timeout wrapper SD-33's register asked for. Every place that asserts
the stage count moves in the same cycle.

**Evidence:** RED→GREEN — move a cited function 50 lines by inserting a comment block, all three
`--check`s stay green; change one cited condition, all three fail. `scripts/verify.sh --list`
shows the new stages; `scripts/verify.sh --only shape-engine-boundary` exits 0 with population
printed.

### AT-35-E1-003 — the two templated test families are table-driven, and build time is measured

Fable review R10: 89 `tests/sd18_*_widening.rs` and 95 `tests/sd13_*progression*.rs` are
near-byte-identical templates (~80k of 187k test lines). Consolidate each family into one
table-driven binary with the same assertions. `BASELINE_ROOT_TEST_BINARIES` and the tests floor
in `scripts/verify-baselines.env` move in the same commit.

**Evidence:** cold `cargo test --locked --no-run` wall time (`CARGO_INCREMENTAL=0`, fresh
`CARGO_TARGET_DIR`) measured before and after, both in `artifacts/epic-1-tax-cut/build-time.json`
with the command and the machine's load. Test count after equals before, proven by a
name-by-name diff of `cargo test -- --list`. Zero new warnings. The inherited failing-suite
baseline re-derived after the change.

### AT-35-E1-004 — the ratio row, and the gates point at this package

`workflow-instruction.md §7`'s receipt carries the `rust_lines_changed / units_closed` and
`pcgen_live_files` rows, produced by `cycle_scope_gate.py --receipt`. `scripts/denominator_gate.py`
(`BUNDLE_DIR` at :101 and `DEFAULT_GLOBS` at :121 still point at **SD-33** — never advanced to
SD-34) and the `figure-provenance` stage default to SD-33 **and** SD-34 **and** this package;
nothing already scanned stops being scanned.

**Evidence:** `scripts/verify.sh --only denominator-gate` default run lists every SD-35 `.md`
in `files_checked`, `violations=0`; `--only figure-provenance` exits 0 across the package.

### AT-35-E1-005 — the PCGen residue gate exists, with its baseline recorded

`scripts/pcgen_residue_gate.py --check` (`technical-design.md §0`, `§6`) greps the live side —
`src/rules_core/**` minus `cache_gen/`, `src/saved_character/**`, `src/campaign/**`,
`src/homebrew_authoring/**`, `apps/desktop/**` — for the PCGen surface (`raw_tokens`,
`PcgenFormulaEvaluator`, `render_pcgen_desc`, `bonus_stack_reader`, `pre_tokens`, and PCGen
token-syntax literals), prints `live_files` and `live_hits`, and fails when either exceeds the
baseline in `scripts/pcgen-residue-baseline.env`. `--closure` fails on anything above zero.
`--rebaseline` is allowed only after a cycle that reduced the count. The baseline is recorded
in this cycle from the real grep (a coarse count at authoring found 78 files —
`content-unit-inventory.md §6`).

**Evidence:** RED→GREEN — plant one `raw_tokens` read in `src/rules_core/`, the gate fails;
remove it, the gate passes; `--closure` fails at the baseline (correct — Epic 6 is what makes
it pass). Wired into `verify.sh` as `pcgen-residue-gate`. `scripts/tests/test_pcgen_residue_gate.py`.

### AT-35-E1-006 — SD-34's unrun closure is folded: retrospective written and cited, open rows mapped

`decisions.md §12`. SD-34 merged (PR #383, `fe5ae6cd4a`) without its closure epilogue. This
docs-only cycle writes `docs/retro/sd34-book-completion-retrospective.md` from
`python3 scripts/retro.py summary --since 2026-08-27 --json` in `docs/retro/sd31-retrospective.md`'s
shape (raw tally, what the data says, what worked, what did not, named changes for the next
bundle), cites it from `../SD-34-book-completion/references/README.md` **and** this package's
`references/README.md`, maps SD-34's 17 open `kanban.md` rows to the SD-35 criterion that owns
their units, and records in SD-34's `progress.md` that the bundle closed by operator merge with
its epilogue folded here. Any "changes for the next bundle" the retrospective names that
`decisions.md §9` does not already carry are added to `§9` in the same cycle. **The 29 open
deferrals** `retro.py summary --since 2026-08-27 --json` reports at the cut (SD-34 recorded 3;
the rest are SD-34 wave deferrals) are each dispositioned: resolved (with the resolving SHA), or
mapped to the SD-35 criterion whose population owns the units, so `deferrals.open` for the SD-34
window reads 0 or names only SD-35-owned items.

**Evidence:** the file exists; `grep -c sd34-book-completion-retrospective` ≥ 1 in both
`references/README.md` files; the row map's unit counts sum to `completion_atlas.py --book
core_rulebook --check` + `--book ultimate_campaign --check` non-DONE totals at `4c6c57eb9f`;
SD-34's `progress.md` frontmatter `status` reads closed-by-fold; the 29-deferral disposition
table in `artifacts/epic-1-tax-cut/sd34-deferral-dispositions.json` with every id. `SCOPE_GATE:
EXEMPT` in the receipt (`decisions.md §2`).

---

## Epic 2 — The sheet rule, made mechanical

**Gated on:** Epic 1. **Gates:** Epics 3–6. **Population:** all 23,315 non-DONE units.

**Inputs already built (`decisions.md §15`):** `artifacts/epic-2-sheet-rule/token-mapping/mapping-table.v1.json`
(249 rows, judged and synthesized — AT-35-E2-001 transcribes it), `SYNTHESIS.md` (the 12
resolved conflicts), `blockers.md` (B1–B10 with owners; three pending rulings R1–R3 built to
their recommendations until ruled). `technical-design.md §1–§2` are schema v2. Owners from
`blockers.md`: B2, B5, B7, B9 → AT-35-E2-001 (tool-side reads of un-ingested DEFINE rows,
prestige level lines, and the `.lst` row for the 1,008 no-`raw_tokens` units; the `_pfs/` skip
and KEY-based `.MOD` match in the mod index **before** the first conversion); B10 → AT-35-E2-004
(refusals counted per shape); B1 → AT-35-E2-005 (oracle export tokens for skill, speed, DR, DC,
spells-per-day before the parity run); B3 → AT-35-E4-001 (ABILITYCATEGORY input for
`BONUS:ABILITYPOOL`); B4, B6, B8 → AT-35-E4-001 (character facts, PI residue per R2, small
shape refusals).

### AT-35-E2-001 — the converter exists, and writes our schema

`src/bin/sheet_rule_convert.rs` + `src/pcgen_import/sheet_rule/` (`technical-design.md §1`).
Input: every corpus record's token closure. Output: `data/sheet_rules/<book>/<kind>/<key>.json`,
one `SheetRule` per record in **our** schema — `label`, `value` (`Number(Expr)` /
`Dice` / `Text`), substituted `prose`, `applies`, `provenance`. **No PCGen token, formula
string, or variable name appears in the output.** A token type with no mapping row makes the
record `Refused { token_type }`, written to `data/sheet_rules/_refused.json` and counted per
token type. The converter calls the existing PCGen formula parser to get an AST and maps the
AST to `Expr`; it never emits the source string. **The mapping rows are transcribed from
`token-mapping/mapping-table.v1.json`** (`decisions.md §15`); a mapping not in the table is a
table defect to record, not a rule to invent in the cycle.

**Evidence:** `cargo run --locked --bin sheet_rule_convert -- --check` exits 0 with
`records=49438 converted=<n> refused=<n>` summing to the corpus; `grep -rlE
'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` is **0**. Unit tests on real
records for each value form: the racial SLA DC converting to `Sum([Const(10), SpellLevel,
AbilityMod(Cha)])`; a weapon converting to `Dice{"1d8", Some(Const(2))}`; a `%CHOICE` trait
converting to `Text` with `Choice` in `applies`. One converter gate **per kind** reading the
live corpus directory: every record of that kind converts or is in `_refused.json`.

### AT-35-E2-002 — the live evaluator and the sheet section, proven once per kind

`src/rules_core/sheet_rule.rs` evaluates `Expr` against the computed character — a match over
the enum, no parser, no strings (`technical-design.md §2`). One generic "Rules and features"
section in `apps/desktop/src/characterHub/CharacterSheet.tsx` lists every held rule's line,
grouped by kind, on the existing sheet IPC. The `Not computed` lane keeps only records with no
rule.

**Evidence:** `cargo test --locked --lib sheet_rule` — one test per value form (the racial SLA
rule evaluating to `Resolved(15)` at Charisma 14 and spell level 3; the weapon to `Dice("1d8+2")`;
the choice trait to `Words`). 19 frontend tests, one per kind, asserting a held record's label
and value are in the DOM for a fixture character. `pcgen_residue_gate.py --check` unchanged
from baseline (this cycle adds zero live PCGen). Desktop crate tested explicitly.

### AT-35-E2-003 — `sheet-complete` is a status every consumer understands

Add `sheet-complete` to `v06_work_inventory`'s `status_vocabulary` with the meaning in
`technical-design.md §3`. Add the classifier rung above `engine-does-not-hold` /
`ingested-magnitude`, below `grounded`. `completion_atlas.py`'s DONE set gains it, its
DONE-evidence check requires `sheet_rule_rendered:<form>`. **Every consumer that raises on an
unknown status is found by grep and updated in the same cycle.**

**Evidence:** `grep -rln "oracle-unverifiable" src scripts apps tests | wc -l` before equals
`grep -rln "sheet-complete" src scripts apps tests | wc -l` after. `completion_atlas.py --check`
`unclassified=0 overlap=0 done_evidence_violations=0`.

### AT-35-E2-004 — the token-coverage ledger

`scripts/token_coverage.py --check` writes `artifacts/epic-2-sheet-rule/token-coverage.json`:
per top-level PCGen token type — units carrying it (non-DONE, all 37 books), units converted,
units refused **because of this token**, and the converter mapping row that handles it. Sums
checked: every non-DONE unit appears under at least one token or under `token-less`; the
refused set across all tokens equals `_refused.json`'s id set. **This is a tool-side script
reading the corpus; it is allowed to read PCGen.**

**Evidence:** `--check` exits 0 with sums printed; RED→GREEN on a planted double-count. Wired
into `verify.sh` as `token-coverage`. The remainder is named by token type from here on.

### AT-35-E2-005 — the first corpus-wide conversion, measured, oracle-checked, zero new mappings

Run the converter over every record, regenerate `data/sheet_rules/`, regenerate the inventory
once. Report: units moved into DONE by bucket and kind (id-set diff), the refused report by
token type, the projected remainder per token type, and the pass's wall time. **No mapping row
is added in this cycle.** Then run the oracle harness over the fixture roster and compare every
`Number` value the evaluator produces against PCGen's exported total for the same character —
this is the "use PCGen to test the rewrite" check, and it runs here first so a wrong mapping is
caught before Epics 3–4 build on it.

**Evidence:** the receipt with `cycle_scope_gate.py --receipt` rows; `token-coverage.json`
re-derived; `completion_atlas.py --check` before and after; the oracle comparison with
`PCGEN_ORACLE_SHA`, `compared=<n> agree=<n> disagree=<n>` and every disagreement named with its
`Expr` and PCGen's value.

---

## Epic 3 — Place and surface

**Gated on:** Epic 2. **Population at authoring:** B 11,589 + C 4,180 = 15,769 of 23,315.
Cycles scoped by **kind × evidence family**, 500 units minimum, all 37 books.

### AT-35-E3-001 — `class_feature` bucket B reaches zero

7,866 units at authoring across three evidence families
(`class_feature_owner_matched_by_name_but_record_not_held_by_engine`,
`class_feature_option_pool_record_with_magnitude_not_held_by_engine`,
`class_feature_option_pool_record_not_held_by_engine`). Under the new design "held" means the
character's holdings include the rule — `SheetRule.applies` says which class/level/choice holds
it, derived at convert time from the corpus's own `CLASS` / `ABILITYCATEGORY` declarations, not a
hand-kept list. Widen the converter's `applies` derivation and the live holdings lookup. SD-33's
open deferral 1 (1,128 unmatched pool-group prefixes) closes here.

**Evidence:** `completion_atlas.py --by-kind` reports `class_feature` B at 0; movement by
id-set diff; every cycle's `cycle_scope_gate.py` output.

### AT-35-E3-002 — every other kind's bucket B reaches zero

3,723 units at authoring: template 1,092, companion 634, feat 490, ability 475, spell 391,
race_trait 319, class 118, equipment 74, race 56, language 33, monster 27, monster_ability 13,
skill 1. Same mechanism per kind.

**Evidence:** as E3-001, per kind.

### AT-35-E3-003 — bucket C reaches zero

4,180 units at authoring, all `class_feature`, evidence
`no_explanation_id_and_no_diagnostic_names_this_feature`. Under the sheet rule a held rule with
a line is `sheet-complete` regardless of explanation id; the classifier's C rung is replaced.
Any C unit that does not convert is refused by token type and moves to Epic 4's ledger.

**Evidence:** `completion_atlas.py --check` reports C at 0; the refused report for any residue.

### AT-35-E3-004 — the rate ledger

`artifacts/epic-3-place-and-surface/rate-ledger.json`: per cycle — mechanism, scoped
population, units closed, units relabeled, wall time, `rust_lines_changed / units_closed`,
`builds_recorded` (must be 1), `pcgen_live_files` (must not rise).

---

## Epic 4 — Resolve and verify

**Gated on:** Epic 3. **Population at authoring:** M 4,334 + V 392 = 4,726 of 23,315.

### AT-35-E4-001 — bucket M reaches zero, by token family

4,334 units at authoring: ability 1,483, race_trait 697, spell 558, feat 518, equipment_modifier
443, template 305, trait 123, equipment 99, domain 67, skill 29, deity 9, race 3. Each cycle
adds **converter mapping rows** for one token family (`BONUS:SKILL|…`, `BONUS:SAVE|…`, `DR:`,
`SR:`, `SPELLS:`, `TEMPBONUS:` → `Text` with its condition, `%CHOICE` aliases → `Choice`),
re-runs the converter, regenerates. Every new `Number` mapping is oracle-checked on the fixture
roster in the same cycle.

**Evidence:** `completion_atlas.py --check` reports M at 0; `token-coverage.json` shows every
compute-bearing token type with a mapping row or a named refusal with count; the oracle
comparison per cycle with disagreements named.

### AT-35-E4-002 — bucket V goes through the oracle harness once

392 units at authoring. One corpus-wide run of `scripts/oracle_harness/`, per-unit cost
measured on the first 50 and projected wall time stated before the full run. Verdicts booked as
`oracle-agree` / `oracle-unverifiable`. **This run compares the live evaluator's values, not the
old string-formula path's** — so it is also the parity proof for whatever Epic 6 has not yet
retired.

**Evidence:** V at 0; the harness receipt with `PCGEN_ORACLE_SHA`; `oracle_disagreement=<n> of
392`, every disagreement named.

### AT-35-E4-003 — the rate ledger

`artifacts/epic-4-resolve-and-verify/rate-ledger.json`, same shape as E3-004.

---

## Epic 5 — Residues

**Gated on:** Epic 4. **Population at authoring:** A 449 + D 1,982 + U 202 + X 168 + Z 19 =
2,820 of 23,315. Bundled into cycles of 500 or "everything left".

### AT-35-E5-001 — bucket A reaches zero: the `power` and `companion` tables

`power` (421, `ultimate_psionics`) and `companion` widening (28, `bestiary`). The tables load
`SheetRule.applies`, not tokens. Fail-closed: real record or named refusal.

**Evidence:** `python3 scripts/missing_engine_tables.py --check` reports `population=0`; the
refusal/success transcript pair.

### AT-35-E5-002 — bucket D reaches zero, sub-cause by sub-cause

1,982 at authoring: template 595, class_feature 471, deity 408, race_trait 183, ability 108,
language 81, domain 80, class 29, skill 21, trait 6. The largest sub-cause,
`class_feature_of_unmodelled_corpus_class` (634 units, 60 classes), is **one generic class
chassis converted from corpus `CLASS` records** (hit die, progressions, class skills, level
table) into our schema — not sixty hand-written functions (`decisions.md §7`). `deity` renders
as `Text`. Every sub-cause enumerated by `completion_atlas.py --by-evidence`.

**Evidence:** D at 0; every sub-cause named with its mechanism and count.

### AT-35-E5-003 — buckets U and Z reach zero

U 202: per sub-cause, the instrument correction or a proven statement that the record carries
nothing a player reads. Z 19: `beginner_box` gets a compiled rule set through the guarded
generator path, then converts.

**Evidence:** U and Z at 0; `corpus_literal_sweep` examined-count moved by exactly the
`beginner_box` record delta.

### AT-35-E5-004 — bucket X reaches zero: the per-character choice filter

168 at authoring. SD-34 `decisions.md §17`'s operator requirement stands: the backend filters
the valid options for *this* character at level-up. Build the join over `SheetRule.applies`
(prerequisites are `Applies`, converted from `PRE*` at ingest — no `pre_tokens` on the live
side) and expose it on the existing level-up IPC.

**Evidence:** X at 0; a desktop test: a level-3 fixture's option list excludes a failed-prereq
option and includes a met one.

### AT-35-E5-005 — the corpus reaches 49,438 of 49,438, and the capability register is closed

**Evidence:** `completion_atlas.py --check` → `DONE=49438 of 49438`, every other bucket zero.
`artifacts/epic-5-residues/completion-manifest.json` — one row per unit. SD-34's
`capability-register.json` re-derived: every row `built: true` or
`unnecessary-under-sheet-rule: <reason>`.

---

## Epic 6 — PCGen exit

**Gated on:** Epic 5. **Gates:** Epic 7. **Scope is the LIVE side only.** The converter, the
parser, the generators, the oracle harness, and the pinned checkout are kept for Starfinder
(`decisions.md §11`, what is kept); a cycle that deletes any of them is a defect. **Population:** the live-side PCGen surface — 78 files
by coarse grep at authoring; the exact baseline from AT-35-E1-005's first run. By the time this
epic starts every unit is `sheet-complete` and every value the live side needs is in
`data/sheet_rules/`; this epic removes the old path. **Oracle-checked before and after:** the
harness runs on the fixture roster at the start and the end; drift is a defect in the exit.

### AT-35-E6-001 — the formula evaluator and its callers leave the live side

`PcgenFormulaEvaluator` and `formula_interpreter*.rs` move to `src/pcgen_import/` (the
converter's parser). Every live caller (14 files at authoring — `racial_sla.rs`,
`domain_power`, trait/feat effects, the pilot_compute formula paths) is replaced by
`sheet_rule::evaluate` over converted `Expr`, or deleted where the sheet line already carries
the value. `bonus_stack_reader.rs` and `feat_prereqs/pre_tokens.rs` move with it.

**Evidence:** `pcgen_residue_gate.py --check` shows `PcgenFormulaEvaluator`,
`bonus_stack_reader`, `pre_tokens` at 0 live hits; the oracle comparison agrees before and after;
full workspace suite green.

### AT-35-E6-002 — the generators and the token closure leave `rules_core`

`src/rules_core/cache_gen/**` and `wiring_class.rs` relocate to `src/pcgen_import/`
behavior-identically (they are converter code that lives on the wrong side). Every `src/bin`
generator's import path follows.

**Evidence:** `pcgen_residue_gate.py --check` shows zero `raw_tokens` hits under
`src/rules_core/`; `gen_book_cache` output byte-identical before and after on one book.

### AT-35-E6-003 — the desktop crate and the prose renderer leave PCGen behind

The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers of
`raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc` is
deleted from the live side; its `%N` substitution already happened in the converter.

**Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
crate and frontend suites green; the 19 on-screen tests still pass.

### AT-35-E6-004 — the gate reads zero

**Evidence:** `python3 scripts/pcgen_residue_gate.py --check --closure` → `live_files=0
live_hits=0 verdict=PASS`, wired as the stage's closure mode from this cycle on. The oracle
comparison at the end of the epic agrees with the one at its start. `cargo tree` for the
desktop crate shows no dependency on the converter modules. **And the tool side is intact**
(`decisions.md §11`, what is kept): `cargo build --locked --bin sheet_rule_convert --bin
gen_book_cache` exits 0; `python3 scripts/oracle_harness/run.py --help` exits 0;
`scripts/pcgen-oracle-pin.env` unchanged; `git diff --stat <epic-6-start-sha>..HEAD --
src/pcgen_import scripts/oracle_harness src/oracle_validation` shows moves and additions,
**zero net deletions of function bodies** (a moved file is not a deleted one).

---

## Epic 7 — Closure epilogue

**Gated on:** Epics 1–6 all `complete`. Fires **once**.

### AT-35-E7-001 — final-acceptance scan

Every criterion `AT-35-E1-001` … `AT-35-E6-004` is `complete` and every `kanban.md` card is
`complete`. **There is no "complete or filed under Open blockers".** The scan re-derives every
`complete` from the repo, re-runs `completion_atlas.py --check`, `token_coverage.py --check`,
`sheet_rule_convert --check`, and `pcgen_residue_gate.py --check --closure` at HEAD, samples the
completion manifest and re-evaluates each sampled rule, re-derives failure attribution from
`git` against the `tranche/15` cut SHA, greps the closure instruments for hardcoded exclusion
lists, and confirms every cycle receipt carries a scope-gate line. **The full `verify.sh` runs
here.**

**If anything is short, the cycle stops** — no retrospective, no sweep, **no PR**.

### AT-35-E7-002 — retrospective written and cited

`docs/retro/sd35-corpus-sheet-completion-retrospective.md`, grounded in
`python3 scripts/retro.py summary --since <bundle-launch-date> --json`. Must state: build time
before/after, units per cycle (min, median, max), lines per unit distribution, the `Words`
share per kind, and the PCGen residue count per epic. **Cited from `references/README.md` in
the same cycle.**

### AT-35-E7-003 — sweep, architecture docs, graphify, PR, release notes

Full worktree/branch sweep with counts; architecture-docs refresh (`rules-engine.md`,
`rules-data-tables.md`, `corpus-ingest.md`, `desktop-app.md`, `status.md`, `testing.md` — the
boundary in `technical-design.md §0` becomes a stated architecture fact) and graphify per
`../template/template.md §6`; PR to `develop`; release notes and version confirmation
(`0.15.0`). Retrospective and sweep before the PR. The operator merges.
