# Cycle AT-33-E6-001 (attempt 9) — epic-6-closure / AT-33-E6-001

- **Commit SHA:** recorded on landing (see `progress.md` entry `sd33-r9-acceptance-scan`)
- **Scanned tree:** clean detached worktree at `origin/tranche/13` = `a0e1c017dd`
  (`.worktrees/sd33-r9-scan`), `git status --porcelain` empty at checkout. The shared checkout at
  `/home/ubuntu/workspace/repos/codex` was NOT used — see "Environment finding".
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (no code changed this cycle)
- **Wired-integration audit result:** OK_NO_TOKENS (no code changed this cycle)
- **Acceptance criterion:** every criterion `AT-33-E1-001`..`AT-33-E5-003` is `complete`, every
  kanban card rows 1-18 is `complete`, and the bundle carries no open blocker.
- **Files touched:** this receipt; `progress.md`; `kanban.md` (row 19 notes only, status stays
  `blocked-escalated`); `docs/retro/events/sd33-r9-acceptance-scan.jsonl`.

## Gate result: **FAIL** (attempt 9). Ninth consecutive halt. One decisive shortfall.

Attempt 8's single surviving shortfall — the workspace test build — is **CLOSED**, verified by
execution in this scan, and closed by real work rather than by a weakened assertion. The
`build-green` lane's report was accurate on every figure this scan re-derived.

**Closing it made a different, larger shortfall visible, and the same lane filed it rather than
cleared it.** SD-33's own wave-6 corpus regeneration leaves the repo's `corpus-sweep`
verification gate RED at **105 findings across 10 of 137 changed corpus records**, and that
finding sits in `progress.md`'s `## Open blockers` as an active entry plus a `deferral` retro
event. Rows 16/17/18 are `complete` over it. This criterion admits no
`complete`-or-filed-under-Open-blockers path.

### Figures

| Figure | Value | Denominator | Re-derive |
|---|---:|---|---|
| Executed lib tests passing | 2,836 | of 2,836 executed lib tests | `cargo test --locked --lib` |
| Executed lib tests failing | 0 | of 2,836 executed lib tests | `cargo test --locked --lib` |
| Lib tests ignored | 14 | of 2,850 declared lib tests | `cargo test --locked --lib` |
| `--no-run` exit | 0 | of 1 invocation | `cargo test --locked --no-run` |
| Integration targets BUILT | 543 | of 543 `tests/*.rs` targets | Check 1 below |
| Integration targets EXECUTED | 543 | of 543 `tests/*.rs` targets | Check 1 below |
| Total suites executed | 599 | of 599 built executables | Check 1 below |
| Workspace tests passing | 7,974 | of 8,023 executed workspace tests | Check 1 below |
| Workspace tests failing | 49 | of 8,023 executed workspace tests | Check 1 below |
| Workspace tests ignored | 67 | of 8,090 declared workspace tests | Check 1 below |
| Failing targets | 31 | of 599 executed suites | Check 2 below |
| Failing targets reproducing identically at the cut | 31 | of 31 failing targets | Check 2 below |
| Failing targets caused by SD-33 | 0 | of 31 failing targets | Check 2 below |
| `cargo test --locked` exit | 101 | of 1 invocation | Check 1 below |
| Desktop crate tests passing | 548 | of 548 desktop crate tests | Check 3 below |
| **`corpus-sweep` findings** | **105** | **across 10 of 137 changed corpus records** | **Shortfall 1** |
| **Active `## Open blockers` entries** | **1** | **of 1 entry in that section** | **Shortfall 1** |
| Blessed units carrying an oracle row | 8,330 | of 8,330 blessed units | Check 4 below |
| Units missing an oracle row | 0 | of 8,330 blessed units | Check 4 below |
| `fixture-verified` rows | 1,741 | of 1,741 fixture-verified units | Check 4 below |
| `literal-verified` rows | 6,589 | of 6,589 literal-verified units | Check 4 below |
| Examined units at `disagree` | 0 | of 8,330 examined units | `box_ledger.py --check` |
| Duplicate `unit_id` | 0 | of 8,330 rows | Check 4 below |
| `agree` rows with `ours != oracle` | 0 | of 811 `agree` rows | Check 4 below |
| Reasonless `unverifiable` | 0 | of 7,519 `unverifiable` rows | Check 4 below |
| Denominator gate violations (pre-receipt) | 0 | of 57 files checked | Check 5 below |
| Denominator gate violations (incl. this receipt) | 0 | of 58 files checked | Check 5 below |
| work-inventory `unknown` | 0 | of 49,438 work-inventory units | Check 6 below |
| Kanban rows `complete` | 18 | of 18 rows 1-18 | `kanban.md` table |
| Kanban-cited receipts present | 33 | of 33 cited receipt paths | Check 6 below |
| Changed corpus records losing license/PI metadata | 0 | of 137 changed corpus records | Check 6 below |
| Changed corpus records whose `raw_tokens` shrank | 0 | of 137 changed corpus records | Check 6 below |

### Four buckets

- **Closure 0** — no `docs/work-inventory.json` `status` field changed this cycle.
- **Reclassification 0** — no unit moved kind or population.
- **Reachability 0** — no unit newly rowed by this cycle (a scan does not row units).
- **Instrument-correction 0** — no instrument changed. One live detection probe was planted and
  removed (denominator gate), leaving no residue.

---

## Shortfall 1 (BLOCKING) — the `corpus-sweep` gate is RED on SD-33's own corpus regeneration, and the finding was filed rather than cleared

Re-run live by this scan against the pinned oracle, not taken from the lane's report:

```
$ cargo run --locked --bin corpus_literal_sweep
corpus-literal-sweep: MISMATCH data/corpus/ultimate_equipment/equipment/hellscourge.json: token not byte-present in corpus token closure: COST:5
corpus-literal-sweep: MISMATCH .../hellscourge.json: ... CRITMULT:x2
corpus-literal-sweep: MISMATCH .../hellscourge.json: ... CRITRANGE:1
corpus-literal-sweep: MISMATCH .../hellscourge.json: ... DAMAGE:1d4
corpus-literal-sweep: MISMATCH .../hellscourge.json: ... EQMOD:Material ~ Steel
corpus-literal-sweep: MISMATCH .../hellscourge.json: ... PROFICIENCY:WEAPON|Scorpion Whip
corpus-literal-sweep: ... 65 further findings suppressed
corpus-literal-sweep: 105 findings across 10 records
SWEEP_EXIT=1
```

**It is SD-33's own work.** All 10 records are inside this bundle's own 137-record corpus diff.
Each moved `data.raw_tokens`/`data.raw_bonus_chains` from `[]` — empty, and therefore vacuously
passing a sweep whose population is "every token the record itself claims" — to fully populated,
written by `src/bin/enrich_equipment_raw_tokens.rs` (+243 lines this bundle, wave 6). The
populated tokens do not byte-match the closure that `corpus_literal_sweep`'s independent
`.MOD`-chain builder derives from the pinned oracle `.lst`. That builder is unchanged since the
cut, so the gate's verdict moved because the data moved.

**The disposition is the defect, not the finding.** The finding itself is real, well-evidenced and
correctly root-caused by the `build-green` lane. But it was written down twice as *not done*:

```
$ sed -n '/^## Open blockers/,$p' progress.md   # real heading at line 304
ACTIVE ### entries outside <details>: 1
  (309, '### `corpus_literal_sweep` mismatch on 10 weapon records — filed `sd33-r8-build-green`, 2026-08-25')

$ python3 -c "...print type/subject for docs/retro/events/sd33-r8-build-green.jsonl..."
deferral | corpus_literal_sweep 105 findings across 10 weapon equipment records (raw_tokens populated ...
```

`AGENTS.md` Blocker Discipline decides this in one question — *was this scope in the Definition of
Done when the work was scoped?* Epic 5's corpus extraction is the bundle's own DoD, and
`corpus-sweep` is the repo's own verification of exactly that extraction. So it is a blocker, and
`AGENTS.md` is explicit that filing one "**pauses** the work; it is not a disposition, not a
closure path, and never a licence to proceed past the blocked item." The bundle's own
`kanban.md` says the same: `blocked-escalated` "is not a closure state and it does not satisfy
AT-33-E6-001."

**Why this blocks rows 16-18 specifically.** `AT-33-E5-001`/`-002`/`-003` are `complete` while the
corpus their own commits regenerated fails the repo's own corpus gate — the identical
`complete`-with-a-deferred-half shape that made attempt 7's shortfall blocking for row 14 and
attempt 8's blocking for rows 16-18, both of which this bundle then correctly fixed rather than
deferred. Deferring it on the ninth attempt would be the first time this bundle took the exit it
has refused eight times.

**The fix is named and scoped** (and is the `build-green` lane's own next-cycle plan item 1):
a dedicated cycle with write scope to `src/bin/enrich_equipment_raw_tokens.rs` **or**
`src/rules_core/corpus_literal_sweep.rs` — whichever of the two `.MOD`-identity fold
implementations is wrong — reconciling them, then re-running
`cargo run --locked --bin corpus_literal_sweep` to `0 findings`. Not attempted here: this is a
scanning cycle, and reconciling two extraction implementations whose output I would also be the
one to verify is exactly the shape this gate exists to catch.

---

## Shortfall 2 (REPORTED, attribution verified INHERITED) — `cargo test --locked` exits 101

CHECK 1's literal requirement is that both build-scope commands exit 0. The second does not:

```
$ cargo test --locked ; echo EXIT=$?
     Running unittests src/bin/ingest_races.rs
thread '...every_committed_race_record_on_disk_deserializes_through_the_shape_b_v1_schema' panicked at
  src/bin/ingest_races.rs:2160:17: assertion failed: record.data.key.starts_with(&record.data.race_key)
test result: FAILED. 43 passed; 1 failed; 0 ignored; ...
EXIT=101
```

Per CHECK 2 this is reportable rather than blocking **because the inheritance was verified by this
scan, not accepted as an assertion** — see Check 2. It is recorded as a shortfall anyway so that
the closure owns the fact plainly: this bundle cannot claim a green workspace suite, only a green
*build* and a failure set identical to the one it inherited.

---

## Prior shortfall CLOSED this wave (verified by execution, not by report)

### Attempt 8's Shortfall 1 — 0 of 543 integration targets executed → **CLOSED, 543 of 543 execute**

## Check 1 — the widest build scope actually runs

```
$ cargo test --locked --no-run ; echo EXIT=$?
NO_RUN_EXIT=0

$ ls tests/*.rs | wc -l
543
$ grep 'Executable ' <no-run log> | grep -c 'tests/'
543
```

**Executed, not merely compiled** — counted from the run's own result lines, not from the exit
code:

```
$ cargo test --locked --no-fail-fast
$ grep -c '^ *Running ' <log>        -> 599     (executables that reported results)
$ grep -c '^test result' <log>       -> 600     (599 suites + the doc-test summary)
$ <sum of the N passed / M failed / K ignored fields across all result lines>
NFF TOTALS passed=7974 failed=49 ignored=67
NFF_EXIT=101
```

599 of 599 built executables reported a result, of which **543 of 543 are the `tests/*.rs`
integration targets** — the population attempt 8 measured at 0 of 543.

### How it was fixed — assertions STRENGTHENED, not weakened

```
$ git diff 3fc992a727..HEAD -- tests/sd20_equipment_equipmods.rs
-    assert_eq!(bonus.affects, "DAMAGE,TOHIT");
-    assert_eq!(bonus.bonus, 1);
+    assert_eq!(bonus.tohit_bonus, Some(1));
+    assert_eq!(bonus.damage_bonus, Some(1));
...
-    assert_eq!(bonus.affects, "TOHIT");
-    assert_eq!(bonus.bonus, 1);
+    assert_eq!(bonus.tohit_bonus, Some(1));
+    assert_eq!(bonus.damage_bonus, None);
```

Both cases match the dispatch's independently-stated expectation exactly: `+1 Weapon` asserts
`tohit_bonus == Some(1) && damage_bonus == Some(1)`; `Adamantine` asserts
`tohit_bonus == Some(1) && damage_bonus == None`. Nothing was loosened, deleted or `#[ignore]`d —
the count of asserted facts rose from 2 to 2 per case while the *Adamantine* case gained a real
negative assertion (`damage_bonus == None`) that the old single `affects` string only implied.

### Sibling search — the lane's search re-run by this scan

```
$ grep -rn --include='*.rs' -E '\.(affects|bonus)\b' tests/ src/ apps/ | grep -iE 'enhanc|weapon_enh'
(exit 1 — no matches)
$ grep -rn --include='*.rs' '\.affects' tests/ src/ apps/
(no output)
```

**0 residual references of 2 old field names, across `tests/`, `src/` and `apps/`.** The lane's
own report is credited correctly here: the second site
(`tests/sd20_tabletop_readiness_integration.rs:1528-1529`) binds the struct to a local named
`enhancement`, so a name-grep for `weaponenhancement` could not have found it and only the full
`--no-run` build did. Its diff is the same shape and equally unweakened.

## Check 2 — failures that were invisible behind the broken build

31 of 599 executed suites fail, carrying 49 of 8,023 executed test failures.

**The lane's `pre-existing` claim is CONFIRMED, and confirmed on evidence this scan re-derived
rather than on the claim.** Two independent checks agree:

```
# (a) the failing SET at HEAD vs the lane's recorded cut run, normalised for target-dir paths/timings
$ diff <(normalise cut_31.txt) <(normalise my nff.log failing targets)
NORMALIZED_IDENTICAL: 31 targets, same pass/fail counts, same order at f652db7ac7 and HEAD

# (b) 0 of 31 failing targets carry any commit since the cut
$ for f in <each of the 31 target files>; do git log --oneline f652db7ac7..HEAD -- "$f" | wc -l; done
0   (for all 31, including src/bin/ingest_races.rs)
```

The set is identical, the per-target `N passed; M failed` pairs are identical, and no failing
target's source moved since the cut. **0 of 31 failing targets are caused by SD-33.**

**The 2 that WERE SD-33's own were fixed, not reported** — verified by execution here:

```
$ <from my own full run>
sd25_monk_level_up_explanation_filter_audit: test result: ok. 6 passed; 0 failed; ...
v06_work_inventory:                          test result: ok. 16 passed; 0 failed; 1 ignored; ...
```

Both passed at the cut and failed at HEAD before this wave; both now pass. The Monk fix rewrote a
stale "never surfaces" claim into a positive proof over all 5 real level-up transitions — a
strengthening, and the right disposition for a test whose premise SD-33 legitimately invalidated.

## Check 3 — nothing else moved

```
$ cd apps/desktop/src-tauri && cargo test --locked     # own CARGO_TARGET_DIR
test result: ok. 548 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 87.36s
DESKTOP_EXIT=0
```

548 of 548, exit 0. The desktop crate is a separate cargo workspace and was tested explicitly.

## Check 4 — Epic 5 is undisturbed

```
$ python3 scripts/box_ledger.py --check --oracle-results .../AT-33-E5-003.combined-oracle-results.json ; echo EXIT=$?
uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False
EXIT=0

$ <re-derive rows, distinct ids, verdicts, and the unexamined SET from work-inventory>
fixture-verified.combined-oracle-results.json rows 1741 distinct 1741 expected 1741 {'agree': 396, 'unverifiable': 1345}
literal-verified.oracle-results.json           rows 6589 distinct 6589 expected 6589 {'agree': 415, 'unverifiable': 6174}
AT-33-E5-003.combined-oracle-results.json      rows 8330 distinct 8330 expected 8330 {'agree': 811, 'unverifiable': 7519}
BLESSED 8330 ROWED 8330 UNEXAMINED_SET_SIZE 0 SAMPLE []  ROWED_NOT_BLESSED 0
agree 811 agree-with-mismatch 0
unverifiable 7519 reasonless 0
disagree 0
```

Row counts hold at **1,741 / 6,589 / 8,330**. The unexamined set is **EMPTY as a set**, not merely
zero as a count — both directions are computed and printed, and `ROWED_NOT_BLESSED` is 0 of 8,330
too. **0 of 8,330** duplicate `unit_id`; **0 of 811** `agree` rows with `ours != oracle`; **0 of
7,519** reasonless `unverifiable`.

## Check 5 — the denominator gate is wide, unrelaxed, and still detects

```
$ scripts/verify.sh --only denominator-gate
    PASS  denominator-gate  (files_checked=57 violations=0)
RESULT: PASS  (EXIT=0)

$ git log --oneline e6f3705b3e..HEAD -- scripts/denominator_gate.py scripts/verify.sh
(empty — neither the matcher nor the stage config changed since attempt 7)
```

Scope **widened again**, from 55 files at attempt 8 to **57 of 57 files checked**, with the matcher
untouched — widened, not blinded. Detection re-proven live, probe planted in a scanned receipt and
removed via `git checkout`:

```
$ <bare hundred-percent token appended to AT-33-E6-001-attempt8_cycle_receipt.md>
VIOLATION .../AT-33-E6-001-attempt8_cycle_receipt.md:381: Probe line: ...
files_checked=57  violations=1
$ git checkout -- <that file>   # tree back to clean
files_checked=57  violations=0
```

Re-run once more after this cycle's own documents were written, so the gate has scanned the
receipt making these claims: **0 violations of 58 files checked** (57 + this receipt), exit 0.

## Check 6 — the rest of the scan

| Item | Result | Command |
|---|---|---|
| work-inventory `unknown` | **0 of 49,438 units** | `jq '[.units[]\|select(.status=="unknown")]\|length'` |
| Kanban-cited receipts | **0 missing of 33 cited paths** | path-existence loop over `kanban.md` |
| Hardcoded exclusion lists | **`EXCLUDED_BOOKS = frozenset()`, size 0** — no carve-out in the closure instruments | `python3 -c "...print(repr(P.EXCLUDED_BOOKS), len(...))"` |
| Corpus integrity | **137 of 137** changed records are modifies; **0 of 137** lost license/PI metadata; **0 of 137** shrank `raw_tokens` | corpus audit vs `f652db7ac7` |
| Epic 3 artifact | at the SD-33 path (`artifacts/epic-3-engine-coverage/`) | `ls` |
| SD-32 gate-2-engines file | **UNTOUCHED** — 0 commits since the cut under `docs/release/SD-32*/` | `git log f652db7ac7..HEAD -- 'docs/release/SD-32*/'` |
| Deferral posture | 8 forward-scope rows, **0 of 8** defer DoD scope; each carries an owner and a revisit condition. **Separately: 1 `deferral` retro event DOES defer DoD scope — Shortfall 1.** | `forward-scope-register.md`; `sd33-r8-build-green.jsonl` |

## Criterion / card status at attempt 9

| Row | Criterion | Status | Blocks? |
|---|---|---|---|
| 1-15 | `AT-33-E1-001`..`AT-33-E4-003` | complete | no |
| **16-18** | **`AT-33-E5-001`/`-002`/`-003`** | **complete** | **YES — `complete` over a `corpus-sweep` gate their own wave-6 regeneration turned RED (Shortfall 1)** |
| 19 | `AT-33-E6-001` | blocked-escalated | this card |
| 20-21 | `AT-33-E6-002`, `AT-33-E6-003` | not-started | Epic 6's own, gated on row 19 |

## Environment finding

**Fourth consecutive wave** in which the shared checkout at `/home/ubuntu/workspace/repos/codex`
was unusable for scanning. At the start of this cycle it sat **8 commits behind
`origin/tranche/13`** (`HEAD` = `06e858b0e6`, origin = `a0e1c017dd`) with **158 entries** in
`git status --porcelain` this agent did not create — including **137 staged `data/corpus/**`
modifications** and a **staged deletion of four Epic 5 artifacts** (`AT-33-E5-003`'s and
`AT-33-E5-last39-weapon`'s receipts and both their oracle-results JSONs). Per `AGENTS.md`'s "One
writer per tree", nothing was written there; the entire scan ran in a clean detached worktree, and
the shared tree was left exactly as found.

`AGENTS.md` Rule 8 — *a warning is not a control* — now applies with four firings on the record:
attempts 6, 7 and 8 each wrote this warning into a receipt and the condition recurred and worsened
(154 entries then, 158 now). The mechanism, not another caution, is the outstanding item: dispatch
scanning cycles into a purpose-made worktree rather than naming the shared checkout as `BRANCH`.

**Second environment finding, carried forward and worked around this cycle.** `RETRO_ACTOR` does
not survive between tool calls in this harness — each `bash` invocation is a fresh shell, so the
`export` in §2.1 is gone by the next call. Attempt 8 recorded this after `verify.sh` auto-attributed
its gate run to `sd31-transcribe`. This cycle set `RETRO_ACTOR=...` inline on every `verify.sh` and
`denominator_gate.py` invocation instead; the resulting events are correctly attributed to
`sd33-r9-acceptance-scan`. The §2.1 environment block as written is not sufficient on its own.

## Disposition

**Gate FAIL. No retrospective, no sweep, no PR.** Kanban row 19 stays `blocked-escalated`.

One item stands between this bundle and closure:

1. **Shortfall 1** — reconcile `src/bin/enrich_equipment_raw_tokens.rs`'s `.MOD`-identity fold
   against `src/rules_core/corpus_literal_sweep.rs`'s independent closure-builder for the 10
   affected records, fix whichever is wrong, and re-run
   `cargo run --locked --bin corpus_literal_sweep` to `0 findings`. Then clear the
   `## Open blockers` entry and re-run this scan. Rows 16-18 are honestly `complete` at that point.

Shortfall 2 (`cargo test --locked` exit 101) is inherited, verified identical to the cut in both
set and counts, and is reported rather than blocking per CHECK 2 — but it is not a green suite and
this bundle should not describe it as one.

Everything else this criterion checks is closed and was verified by execution this cycle,
including the whole of attempt 8's surviving shortfall.

- **Status:** blocked-escalated
- **Movement, four buckets:** closure 0, reclassification 0, reachability 0,
  instrument-correction 0 (see "Four buckets" above).
- **Notes:** The `build-green` lane's work is sound and its report was accurate on every figure
  re-derived here; the disagreement is solely with the *disposition* of the corpus-sweep finding,
  which `AGENTS.md` and this bundle's own `kanban.md` both classify as a pause, not a closure.
- **Next-cycle plan:** the reconciliation cycle named in Shortfall 1, then attempt 10 of this scan.
