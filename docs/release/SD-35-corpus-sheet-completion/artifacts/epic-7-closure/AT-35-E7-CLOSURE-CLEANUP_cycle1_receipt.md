# AT-35-E7-CLOSURE-CLEANUP — cycle 1

**Cycle start / HEAD scanned:** `20ea567623ba323dbe5f3b3dfb01076a6765d48b`
**Date:** 2026-09-15
**Brief:** close the eight shortfalls `AT-35-E7-001_cycle3_receipt.md` reported (S1–S8), each
re-verified at HEAD before it was acted on, because HEAD had moved since that scan ran.

- **Scope gate:**
  `SCOPE_GATE: python3 scripts/cycle_scope_gate.py --min 500` at cycle start `20ea567623`
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  `remaining_non_done=0` — the whole remainder, which is empty. Not a floor exemption.
  `closed=0 relabeled=0 rust_lines_changed=<see below> ratio=n/a builds_recorded=1
  pcgen_live_files=0` — `pcgen_live_files` non-increasing against every prior cycle (it is at the
  floor).

---

## The finding that changed the shape of this cycle

The brief framed **S2/S3** as paperwork: the 142 `_refused.json` entries were said to be *"ids with
no INPUT record — reprint pointers whose canonical printing already renders"*, to be split into a
second named list with the acceptance wording amended. **The proof the brief itself demanded
refuted its premise**, and that is recorded as
`correction 1789474976892-at-35-e7-closure-cleanup-2ab584`.

They are reprints — but the canonical printing **is not itself an inventory unit** and has **no
rule file** either, so **all 142 rendered nothing at all**. "142 `DONE` by another route"
(`refused_non_done=0`) was an atlas reading of `status` + `evidence`, not a rendered sheet line:
exactly the substitution `decisions.md §1` forbids. A ruling excusing them would have excused a
real 142-unit rendering hole.

**Proved exhaustively, not on a sample, before anything was changed.** For all 142, the corpus
record found in another book's directory carries the **same source row** as the unit — same
`source_file` basename, same `source_line`:

```
row_exact=142   ambiguous=0   (python3 over docs/work-inventory.json + a full walk of data/corpus)
```

`advanced_race_guide` declares **33** race units; only **12** race records sit in
`data/corpus/advanced_race_guide/race/`, because `elf` is `data/corpus/core_rulebook/race/elf.json`
— the same `elf_races.lst:6` row, filed under the book that owns the `.lst`.
`sheet_rule::load_population` keyed **both** its lookups on the unit's own `book`, so both missed.

**The fix is a predicate widening on the `B18` precedent (`decisions.md §21`): the PREDICATE moves,
never the rows.** A corpus record is identified by the source ROW it was ingested from, not by the
directory it was filed under, so `load_population` gains a third lookup keyed on
`(source_file basename, source_line)` narrowed to the unit's `kind`, taken **only when
unambiguous**. No id listed, no book exempted, nothing deleted, and `_refused.json` is **not** split
into a second list — there is nothing left to put in one.

**Measured over the whole population before it was trusted:**

| | count | denominator |
|---|---:|---|
| units missing both book-keyed lookups | 831 | 49,450 inventory units |
| resolved by the new fallback | **142** | those 831 — **precisely the refused set** |
| ambiguous (≥2 candidate records) | **0** | those 831 |
| still unresolved, keeping `source_row_in_tree` unchanged | **689** | those 831 |

It cannot silently re-join a unit that was already converting. Ruling recorded as
`decisions.md §23`.

---

## S1 — the kanban bar. Operator ruling S1 recorded, then 56 rows swept from their receipts

Re-derived at HEAD before acting: **60 of 107** rows not `complete` (51 `in-progress`, 4 `blocked`,
1 `blocked-escalated`, 3 `partial`, 1 `not-started`) — up from the scan's 57 of 104, the difference
being rows appended since.

**Ruling recorded** in `decisions.md §22`, in `acceptance-and-verification.md §3a`, and in
`kanban.md`'s own preamble: **a "card" is a CRITERION row.** The bar was written against the 29
rows this board opened with; the per-cycle rows `workflow-instruction.md §5` appends are the
mechanical receipt trail, read as evidence for a criterion and never an item the bundle owes.

**Why it terminates, which is the actual defect.** Every cycle that runs appends a row (`§5`; `§6`
step 8). A cycle dispatched to close the open rows is itself a cycle, so it appends one more. The
set grows by at least one per attempt, so **a bar counting cycle rows cannot be met by running more
cycles** — visible in the scans' own figures: cycle 2 measured 55, cycle 3 measured 57, and the
whole difference was three census rows appended in between.

**Then the sweep — 56 rows, each from its own receipt, never in bulk.** Every non-`complete` row's
receipt path was checked to resolve first (**60 of 60 exist on disk**). Each row's Notes now carry
the receipt already cited plus the HEAD command showing its named remainder is zero:

| rows | family | what set them |
|---:|---|---|
| 49 | Epic 6 residue lanes (AT-35-E6-002/003, SWEEP, FINISH, RULED) | `pcgen_residue_gate.py --check --closure` → `live_files=0 live_hits=0 verdict=PASS`; every token type each row named as its remainder reads zero, under rulings B14/B15/B16/B17 |
| 1 | row 26, the `AT-35-E6-003` **criterion** row | its own Evidence clause re-derived: `root apps/desktop files=0 hits=0`, desktop + frontend + the 19 on-screen tests green in this cycle's `verify.sh` |
| 1 | row 59, `blocked-escalated` | its deliverable WAS the measurement and the escalation; **the ruling it asked for landed as B14** (`decisions.md §17`) |
| 1 | row 76, `blocked` | its deliverable WAS the finding that the gate could not see six live `render_pcgen_desc_*` call sites; **that became ruling B16** (`§19`), and B15 (`§18`) settled its `#[cfg(test)]` question |
| 3 | rows 101–103, `partial` census | `epic-breakdown.md` records the census **closed** in three cycles; re-derived at HEAD: `population_census_final.py` → `absent_total=0`, `corpus_wide: 48864 of 48864 = 100.0000%`; `population_census_255.py` → `population=255 absent_total=0`; `b18_ten_render_proof.py` → `admitted=12 render=12 failures=0` |
| 1 | remaining Epic 2/3/5 non-DONE hand-off row | atlas `DONE 49450`, converter `refused=0`, token-coverage `non_done=0` |

**Left open on purpose — 4 rows, all of them the closure rows themselves:** 28, 100 and 107
(`AT-35-E7-001`, `blocked` — the scan has not re-run and PASSED) and 29 (`AT-35-E7-002` +
`AT-35-E7-003`, `not-started`). Board now reads **103 complete · 3 blocked · 1 not-started** of 107,
plus this cycle's own row 108.

## S2 and S3 — `_refused.json` empty and zero refused units, met VERBATIM

`§3a` is **not** amended on this point and the ruling both prior scans asked for is **not** given;
see the finding above and `decisions.md §23`.

| check | before | at HEAD after the fix |
|---|---|---|
| `sheet_rule_convert --check` | `records=49450 converted=49308 refused=142 rules=70147 var_tables=5293` | **`records=49450 converted=49450 refused=0 rules=70317 var_tables=5294 verdict=PASS`** |
| `_refused.json` | 142 entries, `by_token_type={'no_corpus_record': 142}` | **`refused=0`, `entries` empty, `by_token_type={}`** |
| `token_coverage.py --check` | `refused=142 refused_non_done=0` | **`non_done=0 tokened=0 token_less=0 refused=0 refused_non_done=0 token_types=233 shapes=0 verdict=PASS`** |
| `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | 0 | **0** |

`token_coverage.py` needed **no split and no teaching**: its `REFUSED_SET` and `SHAPE_TOTALS`
checks are set equalities and hold at zero exactly as they held at 142. Its stale doc comment was
corrected to record 829 → 142 → 0 with the mechanism for each step.

**The package diff is contained and is an improvement, not a churn:** 144 files added, 121
modified, of 54,759. The 121 are rules that gained a `granted_by` edge to a record that now exists
— e.g. `bestiary_4:template:kitsune` gains `{"Rule": "bestiary_4:race_trait:kitsune_type"}` and
changes in no other way. `advanced_race_guide:race:elf` now renders `Walk 30 ft.` with its
`BaseSize`/`IsPC` facts and its `Humanoid`/`Base`/`PC`/`Core` tags, having rendered nothing before.

**Independent confirmation from a different instrument:** the regenerated completion manifest's
`by_sheet_rule_content` had `no_rule: 142`; that key is now **absent** — no unit is without a rule.

**Honest counter-movement, with its denominator:** `_defects/unresolved-references.json`
11,819 → **11,925** (+106) and `_defects/undefined-variables.json` 738 → **739** (+1), because 142
new records bring their own references. `inline-formula-in-prose` is unchanged at 110. These are
ledgers, not gates; no gate pins them.

**Count sweep (a count change that is not swept is a trap this bundle has hit before).** Every old
figure — `49308`, `70147`, `5293`, `54616`, `142` — grepped across `tests/ src/ apps/ scripts/` and
the bundle's `.md` files. **No live code, test, script or gate pins any of them**; they survive only
in historical receipts and `progress.md` prose, where they are correct records of what was true
then. The two live pins that DID exist were updated:

- `src/bin/v06_work_inventory.rs`'s `REFUSED_ID` (a pinned member of the refusal set) →
  `NO_RULE_ID`, and `a_refused_record_and_a_record_with_no_rule_are_left_alone` now **asserts the
  refusal set is EMPTY** and keeps checking "refused ⇒ no rule" over whatever it holds, so a
  refusal reappearing fails loudly instead of quietly shrinking what the test covers.
- `scripts/token_coverage.py`'s population note.

## S7 — the completion manifest regenerated at HEAD

`AT-35-E5-005_completion_manifest.py` re-run (11.97 s):
`generated_at_head` `39dfd59b7e` → **`20ea567623`** (HEAD), `units` **49,438 → 49,450**,
`by_bucket` `{DONE: 49450, A..Z: 0}`, `non_done=0`. `49,438` is pinned in no live code — only in
`scripts/tests/test_denominator_gate.py`'s deliberate fixture strings and one comment.

## S5 — both Epic 4 receipts now carry a scope-gate line, re-derived from nothing

Both receipts already carried the gate's **output** (`scoped=0 remaining_non_done=0 floor=500
verdict=PASS_WHOLE_REMAINDER`); what was missing was the `SCOPE_GATE:` line naming the command.
That line was added from the block already present — **no figure was invented and no verdict was
re-derived**, and the receipts say so in the line itself. Coverage now:

```
126 receipts under artifacts/, 0 missing a SCOPE_GATE / cycle_scope_gate line
```

## S6 — the three named artifacts PRODUCED, from real runs

`§3a` named three files that had never been written. Rather than amend the bar, all three now exist
at their named paths, each carrying `consolidated_from`:

| artifact | source | `PCGEN_ORACLE_SHA` | lines compared / agree / disagree |
|---|---|---|---:|
| `epic-2-sheet-rule/oracle-parity-epic2.json` | Epic 2's real run, verbatim | `7f818006e3…` | 42 / 41 / 1 |
| `epic-6-pcgen-exit/oracle-parity-before.json` | `AT-35-E6-001_cycle1`'s real run, verbatim | `7f818006e3…` | 146 / 145 / 1 |
| `epic-6-pcgen-exit/oracle-parity-after.json` | **a real re-run executed by this cycle at HEAD** | `7f818006e3…` | **159 / 157 / 2** |

The "after" is not a copy: `sheet_rule_parity --roster … --output ours.json` ran at HEAD against the
regenerated package, then `sheet_parity.py compare` joined it to the PCGen exports **pinned** at
that SHA (the exports are the oracle side and are pinned, so PCGen was not re-run and no parity run
is fabricated). Against Epic 6's close (`AT-35-E6-004_cycle3_sheet-parity-after.json`):

```
lines     compared 156 -> 159   agree 154 -> 157   disagree 2 -> 2   unverifiable 67 -> 67
chassis   382 / 376 / 6 / 140  ->  382 / 376 / 6 / 140   (unchanged)
disagreements: the SAME 8, id for id
```

**The 142 records this cycle joined added three compared lines, all three agreeing with the oracle,
and introduced no new disagreement.** None of the three shows `disagree=0`; `§3a`'s second clause
applies and every disagreement is named in the artifact's own array with both values and its oracle
key. `§3a` amended only to name the paths and to say which of the three is a re-run.

## S4 — 89 open deferrals → **17**, each resolved from evidence or named

`retro.py summary --since 2026-09-07` re-derived at HEAD: **open 89, resolved 1, total 90** (the
scan reported 88; one more had been emitted since). After this cycle: **open 17, resolved 73,
total 90.** Every one of the 72 resolutions cites a command re-run at HEAD, not a judgement:

| resolved | family | the evidence cited in each `resolution` |
|---:|---|---|
| 57 | PCGen residue remainders | `pcgen_residue_gate.py --check --closure` → `live_files=0 live_hits=0 shipped_data_hits=0 verdict=PASS`, plus `§3a`'s independent grep finding only doc comments and `#[cfg(test)]` regions |
| 12 | non-DONE / converter-refusal hand-offs | atlas `DONE 49450`, `sheet_rule_convert --check` `refused=0`, `token_coverage.py --check` `non_done=0` |
| 3 | population census | the three census scripts **re-executed**, not quoted: `absent_total=0` on both, `admitted=12 render=12 failures=0` |
| 1 | the 10 bucket-V oracle disagreements | carried into Epic 6 and every one named in `oracle-parity-after.json`'s `disagreements` array |

**The 17 that stay open are NAMED, with counts that sum to 17** — `deferral
1789476280499-at-35-e7-closure-cleanup-c6f962`:

- **10 — the worktree sweep.** Not closed, because it genuinely has not run: `ls
  .claude/worktrees/ | wc -l` → **16**, `du -sh` → **13G** at HEAD. A dispatched agent's permission
  classifier has refused `git worktree remove` on the shared checkout five times running. Scope is
  **owned, not dropped**: `AT-35-E7-003`, live kanban row 29.
- **4 — real content/inventory holes, each a named record set and not a bucket:** the 2
  `~ Elemental Fist` selector/label-split records (the other 24 of that deferral's 26 *are*
  answered by this cycle's census re-derivation, and saying so is not the same as closing the
  deferral); `advanced_players_guide:spell:wall_of_thorms`, which **still has no rule file at
  HEAD** (`mythic_adventures:spell:elemental_body_iiimod` does);
  `reference_library_catalog.rs`'s 1,150 of 9,679 descriptions; the `docs/work-inventory.json`
  rung-ordering defect.
- **3** — `AT-35-E1-003`'s 244 `grounding_ref` citations (→ `AT-35-E7-003`) and `AT-35-E7-001`'s own
  two (step-9 gate re-proving, and cycle 2's zero-close), which close when the scan re-runs.

**No refused token type is deferred**: this cycle's converter refusal set is empty.

## S8 — the one red test, CLASSIFIED: **(a) environmental**, and here is why that is not "it passed once"

`tests/sd26_pilot_case_verification.rs::full_pipeline_runs_end_to_end_and_finds_one_genuine_attack_bonus_mismatch`.

**Clean re-run at HEAD:**
```
test full_pipeline_runs_end_to_end_and_finds_one_genuine_attack_bonus_mismatch ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; finished in 38.62s
```
plus the full-suite run in this cycle's `verify.sh` (stage table below) — two independent passes.

Cycle 3 was right to refuse the easy "the box could not reach adoptium" reading, because that URL
returned HTTP 200 on immediate re-query. The classification does not rest on the pass. It rests on
**what failed and where**, and on **what this bundle actually did to that file**:

1. **The failure was not in our code and not in our repository.** The panic is at the
   `run_pcgen_character` call (line 377), *before* any comparison logic runs, and its message is
   PCGen's **own** gradle build in `~/workspace/repos/pcgen`: `Could not create task
   ':extractJavaFXLocal'. > Could not create task ':downloadJavaFXLocal'. > Unable to process url:
   https://api.adoptium.net/...`. `:downloadJavaFXLocal` is a **download** task against a remote
   API — its only failure mode is that remote's availability during the 16-minute build window, and
   it does not run at all once its artifact is cached, which is the whole of the 968.67 s → 38.62 s
   difference.
2. **This bundle's only edit to that file cannot reach a gradle download.**
   `git show b91d16a66b -- tests/sd26_pilot_case_verification.rs` → **1 file changed, 1 insertion,
   1 deletion**, and the one line is:
   ```
   -    let actual = codex::rules_core::cache_gen::apg::sha256_file(path)
   +    let actual = codex::pcgen_import::cache_gen::apg::sha256_file(path)
   ```
   the same function, renamed by `AT-35-E6-002`'s module move, inside
   `assert_pilot_pcg_fixture_is_pinned` — a SHA-256 of a local fixture, which runs *before* PCGen
   is invoked and which passed in the failing run too.

So it is **(a)**, on evidence rather than on a green tick: **not (b)** — the bundle's edit is a
one-line import path to an identical function and the failure was in another repository's build
tool; **not (c)** — it is not a pre-existing failure, because the test passes at HEAD. Neither the
word "flaky" nor the phrase "the N known environmental failures" appears in this classification: one
suite, one test, one named cause, in a named external repository.

**Standing weakness this cycle does not fix and does not hide:** the test's result depends on a
network fetch inside PCGen's build, so it can fail again for the same reason on a cold gradle cache.
That is a property of driving the real PCGen engine as an oracle, which `decisions.md §11` keeps
deliberately.

---

## Verification — ONE pass, at the end, at the widest build scope

Everything above landed first; then a single full `scripts/verify.sh` in a private
`CARGO_TARGET_DIR` (`/tmp/cargo-sd35-AT-35-E7-CLOSURE-CLEANUP`, `CARGO_INCREMENTAL=0`).
`builds_recorded=1`.

```
SUMMARY
  passed:  49  preflight-disk preflight-oracle oracle-pin-selftest producer-selftest
               pi-redaction-selftest provenance-selftest site-dashboard-selftest
               site-dashboard-pin site-dashboard-check site-dashboard-pi-gate
               build-public-status-selftest site-public-status-check site-public-status-pi-gate
               site-asset-stamp-check reachability-audit-selftest reachability-audit
               groundtruth-guard-selftest supersession-gate-selftest
               shape-coverage-standing-gate-selftest shape-coverage-standing-gate
               cycle-scope-gate-selftest shape-engine-boundary-selftest shape-engine-boundary
               missing-engine-tables denominator-gate figure-provenance pcgen-residue-gate
               token-coverage-selftest token-coverage pi-sweep declared-pi-audit audit-selftest
               reclaim-selftest driver-selftest corpus-sweep-selftest corpus-trap-audit-selftest
               root-lib root-full desktop reach corpus-sweep sheet-rules-check corpus-trap-audit
               supersession-gate frontend-install frontend-test frontend-typecheck clippy
               class-dump
  FAILED:  0

RESULT: PASS
logs in /tmp/codex-verify-XAwdmH
```

**49 of 49 stages PASS, `FAILED: 0`.** Stages that carry this cycle's figures, quoted from their
own lines:

| stage | line |
|---|---|
| `sheet-rules-check` | `records=49450 converted=49450 refused=0 rules=70317 var_tables=5294 verdict=PASS (117.6s)` — the HEAD `sheet_rule_convert --check` `§3a` asks for, independently confirming S2 |
| `token-coverage` | `non_done=0 tokened=0 token_less=0 refused=0 refused_non_done=0 token_types=233 shapes=0 verdict=PASS` — S3, verbatim |
| `pcgen-residue-gate` | `live_files=0 live_hits=0 verdict=PASS` |
| `root-lib` | `3390 passed` (was `3389 passed; 1 failed`) |
| `root-full` | `8926 passed across 419 suites, all 365 tests/*.rs suites executed`; `grep -c 'test result: FAILED'` over its log → **0** of **420** `Running`/`Doc-tests` lines. `tests/sd26_pilot_case_verification.rs` ran and passed here — the S8 test's **third** independent pass |
| `desktop` | `570 passed` (was `569 passed; 1 failed`) |
| `reach` | `32 passed` |
| `frontend-test` | `101/101 files`; `frontend-typecheck` `tsc --noEmit clean` |
| `clippy` | `root:0 desktop:0 warnings, 0 errors` |
| `denominator-gate` | `files_checked=360 violations=0` |
| `figure-provenance` | `files_checked=290 figures_examined=624 violations=0` |
| `corpus-sweep` / `corpus-trap-audit` | `0 findings`; `all defect kinds at their registered counts` |
| `reachability-audit` | `reachable ceiling 100.00%`, which its own line states as `(49450 / 49450)` — every inventory unit |
| `class-dump` | `31/31 computing` |

**An earlier verify run in this cycle was RED on two stages, and that is recorded rather than
buried.** `root-lib` and `desktop` each failed on **one** count pin the join fix legitimately
moved. Both were fixed in `1a3ffba353` by **re-deriving the whole set**, never by adjusting a
number to fit, and each is attributed in the code:

- `feat_prereqs.rs` — a starting Fighter's eligible feats **540 → 537**, catalog records with no
  converted rule **12 → 1**. Eleven `bestiary:feat:*` records now carry a converted gate; exactly
  three are correctly DENIED with the character's own value in the line (`Awesome Blow` on Power
  Attack + Improved Bull Rush + size 5 + Str 25 vs. size 4 / Str 14; `Craft Construct` on two
  item-creation feats; `Snatch` on size 6 vs. 4), and the other eight are named so the eleven sum.
  The one record still unconverted is **named, not bucketed**: `Transfer Feat to Familiar`.
- `reach_gate.rs` — **22 entries DELETED** from `BARE_RECORD_FINDINGS` (11 × `beastiary1/abilities`
  + the same 11 × `beastiary1/templates`: six `Aasimar ~ *-Blooded`, five `Tiefling ~ *-Spawn`).
  The gate reported them itself — *"these records now carry real fields — delete them from
  BARE_RECORD_FINDINGS"* — which is what it exists to force. **Deleted, not relaxed**; no
  assertion widened.

Both are the same shape as ruling B18's move one ruling earlier: a record that previously had no
converted rule now carries one, so a gate that could not be checked can be.

The two figures that outrank finishing, re-read after everything landed:

```
python3 scripts/pcgen_residue_gate.py --check --closure
  -> live_files=0 live_hits=0 shipped_data_files=0 shipped_data_hits=0 verdict=PASS
python3 scripts/completion_atlas.py --check
  -> population=49450 unclassified=0 overlap=0; DONE: 49450; every other bucket 0;
     done_evidence_violations=0 missing_clearing_mechanisms=0 citation_failures=0
```

Neither moved.
