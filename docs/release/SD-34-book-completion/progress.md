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

`tranche/14` cut at `571307724f`, `0.14.0` stamped, launch checklist items 1-9, 11, 12 run.
Item 10 (widest build scope + inherited test baseline) is a separate lane's obligation and is
not reported here. Epic 1 dispatch underway.

**12 of 27 criteria complete. 12 of 27 kanban rows complete.** Epic 1 is closed at 8 of 8;
Epic 2 is closed at 4 of 4 (AT-34-E2-001..004). Epic 3 (Core Rulebook to zero) is underway:
AT-34-E3-001 (row 13) ran one cycle, cleared one of its eleven mechanisms (29 of 1035 bucket-B
units), and escalated the remaining ten for further per-mechanism cycles — see Cycle 13 and
`## Open blockers` below.

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

### AT-34-E3-001 — bucket B does not close in one cycle: eleven mechanisms, not one

Filed 2026-08-27 by the `AT-34-E3-001` cycle. `core_rulebook` bucket B
partitions into eleven distinct mechanisms (populations and root causes in
Cycle 13 above and `artifacts/epic-3-core-rulebook/AT-34-E3-001_cycle_
receipt.md`). This cycle cleared one (`template`+`ability`, 29 of 1035) with
a verified engine fix. The remaining ten sum to 1006 and are not closable
in one further cycle either — two require real class-feature engine
modelling (726 combined), one requires race/race-trait modelling (141),
one requires extending `companion_chassis` (100), one requires modelling
17 NPC/prestige classes as full `ClassId` entries (17), and two need an
explicit ruling before any code is written: `deity` (21, every corpus
record PI-redacted — resolving by source-coordinate risks a different code
path exposing the redacted real name, the nearest precedent being SD-32's
§28 PI ruling) and `domain` (1, a corpus record missing entirely, requiring
the guarded `gen_book_cache` generator).

**Requested disposition:** decompose `AT-34-E3-001` into further dispatched
cycles, one or a small group of mechanisms at a time (matching the "one
bucket per cycle, cheapest-first" discipline `workflow-instruction.md
§2.4` already applies one level up, at the epic level) — cheapest-first
order recommended in the cycle receipt's "Next-cycle plan". This is not a
request to narrow the criterion's zero bar; it is a request to run it as
more than one cycle, per `workflow-instruction.md §8`'s "a blocker bigger
than one cycle is a sequencing problem, not an exemption."

### AT-34-E1-007 — RESOLVED 2026-08-27 by orchestrator ruling (`decisions.md §13`)

<details>
<summary>Archived — the blocker as filed, and the ruling that cleared it</summary>

The cycle-7 lane filed this asking how AT-34-E1-007 closes when the stage it wires reports
`records_examined=27638 defects=10196 traps=407` against the live corpus. It offered two
dispositions: (1) authorize a corpus-regeneration wave, or (2) rule the criterion satisfied by
the stage's mechanical wiring, independent of whether the corpus is clean.

**Ruling: option 1. The criterion is not narrowed.** Option 2 is a carve-out — a gate that passes
because its bar moved — which `../../governance/blocker-closure-doctrine.md` rejects. A large
blocker is a sequencing problem, not an exemption.

**Verified independently before ruling**, from the orchestrating session, not transcribed from the
lane: the audit re-run gives `findings=10603`, `DEFECT=10196 of 10603`, `TRAP=407 of 10603`, and
`wiring-class-mismatch=7015 of 10196` across **34 of 37** books (largest:
`advanced_players_guide` 875, `core_rulebook` 798, `beastiary` 783, `ultimate_psionics` 759).
`git log -1 b32926f2af` confirms `SD30-CARRY-001` drove this same check `177 -> 0` on 2026-08-14;
nothing has re-run `--audit` since. The remaining `3,181 of 10,196` (`mod-record` 2,117,
`key-differs-from-name` 650, `shared-name-distinct-records` 249, `disabled-line` 165) are SD-33's
registered out-of-DoD debt and stay registered, not absorbed.

**Decomposed into `AT-34-E1-008`** (kanban row 8): drive `wiring-class-mismatch` to zero across all
34 books via the guarded `gen_book_cache` path, per-record PI/`raw_tokens` survival verified,
`corpus_literal_sweep` examined-population moving by exactly the record delta. Epic 1 gains
`data/corpus/**` write scope for that criterion only. AT-34-E1-007's own `exits 0` bar is
unchanged and it closes when AT-34-E1-008 lands.

The bundle is un-paused. No later epic proceeded past the blocked card while it was open.

</details>

**Active: `AT-34-E3-001` (filed above, 2026-08-27) — awaiting an operator ruling on
decomposing the criterion into further per-mechanism cycles.**
