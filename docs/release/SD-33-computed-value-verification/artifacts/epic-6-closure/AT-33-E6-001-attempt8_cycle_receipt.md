# Cycle AT-33-E6-001 (attempt 8) — epic-6-closure / AT-33-E6-001

- **Commit SHA:** recorded on landing (see `progress.md` entry `sd33-r8-acceptance-scan`)
- **Scanned tree:** clean detached worktree at `origin/tranche/13` = `47a37804c0`
  (`.worktrees/sd33-r8-scan`), `git status --porcelain` empty at checkout. The shared checkout
  at `/home/ubuntu/workspace/repos/codex` was NOT used — see "Environment finding".
- **Files touched:** this receipt; `progress.md`; `kanban.md` (row 19 notes only, status stays
  `blocked-escalated`); `docs/retro/events/sd33-r8-acceptance-scan.jsonl`.

## Gate result: **FAIL** (attempt 8). Eighth consecutive correct halt. One shortfall.

Attempt 7's single surviving shortfall — the red lib suite — is **CLOSED**, verified by
execution, and closed by real work rather than by an edited expectation. Both halves were
re-derived independently by this scan and both hold.

**A different shortfall is now visible, and it was visible only because the lib suite went
green.** The full workspace test build does not compile, so **0 of 543 integration test
targets execute**. It is SD-33's own Epic 5 commit that broke it, not an inherited gap. The
suite-green lane reported this item but mis-attributed it to the wrong commit and to
"pre-existing", and on that basis declined to fix it. The attribution is wrong; the item is
this bundle's own debt, and it is the same shape as the shortfall attempt 7 just spent a wave
closing.

### Figures

| Figure | Value | Denominator | Re-derive |
|---|---:|---|---|
| Executed lib tests passing | 2,836 | of 2,836 executed lib tests | `cargo test --locked --lib` |
| Executed lib tests failing | 0 | of 2,836 executed lib tests | `cargo test --locked --lib` |
| Lib tests ignored | 14 | of 2,850 declared lib tests | `cargo test --locked --lib` |
| Desktop crate tests passing | 548 | of 548 desktop crate tests | Check 2 below |
| **Integration suites executed** | **0** | **of 543 integration test targets** | **Shortfall 1 below** |
| Blessed units carrying an oracle row | 8,330 | of 8,330 blessed units | Check 3 command |
| Units missing an oracle row | 0 | of 8,330 blessed units | Check 3 command |
| `fixture-verified` rows | 1,741 | of 1,741 fixture-verified units | Check 3 command |
| `literal-verified` rows | 6,589 | of 6,589 literal-verified units | Check 3 command |
| Examined units at `disagree` | 0 | of 8,330 examined units | `box_ledger.py --check` |
| Duplicate `unit_id` | 0 | of 8,330 rows | Check 3 command |
| `agree` rows with `ours != oracle` | 0 | of 811 `agree` rows | Check 3 command |
| Reasonless `unverifiable` | 0 | of 7,519 `unverifiable` rows | Check 3 command |
| Active `## Open blockers` entries | 0 | of 0 entries in that section | Check 5 below |
| Denominator gate violations | 0 | of 55 files checked | `verify.sh --only denominator-gate` |
| work-inventory `unknown` | 0 | of 49,438 work-inventory units | `jq` below |
| `(ambiguous, unmeasurable)` units | 11 | of 49,438 work-inventory units | Check 1 below |
| F1 population | 6,278 | of 6,278 formula-bearing units in the live inventory | Check 1 below |
| Kanban rows `complete` | 18 | of 18 rows 1-18 | `kanban.md` table |
| Kanban-cited receipts present | 31 | of 31 cited receipt paths | Check 6 below |
| Changed corpus files losing license/PI metadata | 0 | of 137 changed corpus records | Check 6 below |
| Changed corpus files whose `raw_tokens` shrank | 0 | of 137 changed corpus records | Check 6 below |

### Four buckets

- **Closure 0** — no `docs/work-inventory.json` `status` field changed this cycle.
- **Reclassification 0** — no unit moved kind or population.
- **Reachability 0** — no unit newly rowed by this cycle (a scan does not row units).
- **Instrument-correction 0** — no instrument changed. Two live detection probes were planted
  and removed (denominator gate; producer fail-closed), leaving no residue.

---

## Shortfall 1 — the workspace test build does not compile; 0 of 543 integration suites run

`AGENTS.md`, *Concurrency and Measurement*: "**Verify at the widest build scope the repo
has.** `cargo build --lib` green is not a completed phase: `cargo test` builds bin targets,
and one broken bin meant **0 of 502 suites ran** while the phase reported COMPLETE." That is
this failure exactly, one scope out.

```
$ cargo test --locked --no-run
error[E0609]: no field `affects` on type `&WeaponEnhancementBonus`
error[E0609]: no field `bonus` on type `&WeaponEnhancementBonus`
error[E0609]: no field `affects` on type `&WeaponEnhancementBonus`
error[E0609]: no field `bonus` on type `&WeaponEnhancementBonus`
error: could not compile `codex` (test "sd20_equipment_equipmods") due to 4 previous errors
EXIT=101

$ ls tests/*.rs | wc -l
543
```

**It is SD-33's own Epic 5 work, not an inherited gap.** The struct carried `affects`/`bonus`
at the `tranche/13` cut and the test target compiled; SD-33's `2f1d52f22d`
(`AT-33-E5-finalize-wave5` — the `heavy_hammer` fix, rows 17/18's own commit) split those two
fields into `tohit_bonus`/`damage_bonus` and never updated the one integration test that reads
them:

```
$ for c in f652db7ac7 66984fe7bc 2f1d52f22d 7d439876b7; do
    git show $c:src/rules_core/equipment_effects/equipmods.rs \
      | sed -n '/pub struct WeaponEnhancementBonus/,/^}/p' | grep -E '^\s*pub [a-z_]+:' | tr '\n' ' '; echo " <- $c"; done
    pub affects: String,     pub bonus: i16,     pub natural_attack_only: bool,     pub weapon_prof_scope: Option<String>,  <- f652db7ac7
    pub affects: String,     pub bonus: i16,     pub natural_attack_only: bool,     pub weapon_prof_scope: Option<String>,  <- 66984fe7bc
    pub tohit_bonus: Option<i16>,     pub damage_bonus: Option<i16>,     pub natural_attack_only: bool,     pub weapon_prof_scope: Option<String>,  <- 2f1d52f22d
    pub tohit_bonus: Option<i16>,     pub damage_bonus: Option<i16>,     pub natural_attack_only: bool,     pub weapon_prof_scope: Option<String>,  <- 7d439876b7

$ git log --oneline f652db7ac7..HEAD -- tests/sd20_equipment_equipmods.rs
(empty — no SD-33 commit ever updated the caller)
```

**The suite-green lane's own report on this item is wrong on both attribution and inheritance.**
It recorded the cause as "wave-6 commit `7d439876b7` (closed Epic 5 work)" and the item as an
"unrelated pre-existing gap from wave 6", and declined to fix it on that basis. The table above
shows `7d439876b7` did not perform the split — `2f1d52f22d` did, one wave earlier — and that
the target compiled at the cut, so nothing about it is pre-existing. The lane's supporting
check (`git diff --stat HEAD` empty for both files) tests only whether *that cycle* touched the
files, which was never the question; the question is whether *this bundle* broke them, and it
did. Recorded as a correction under §2.3 rather than left in the record.

**Why this blocks, in the criterion's own words.** Rows 16/17/18 (`AT-33-E5-001`/`-002`/`-003`)
are `complete` over a workspace test build their own commit `2f1d52f22d` broke — the identical
`complete`-with-a-deferred-half shape that made attempt 7's Shortfall 1 blocking for row 14,
which this bundle then correctly fixed rather than deferred. `AGENTS.md`'s Blocker Discipline
test — *was this scope in the Definition of Done when the work was scoped?* — answers yes:
this criterion's own scope is "the Rust suite is green for whatever `src/rules_core/` changed
this wave", and `src/rules_core/equipment_effects/equipmods.rs` is precisely what changed.

**The fix is small and named**: update `tests/sd20_equipment_equipmods.rs:94-111` to read
`tohit_bonus`/`damage_bonus` (both `Option<i16>`) instead of `affects`/`bonus`, preserving the
assertions' real intent — the `+1 Weapon` case asserts `DAMAGE,TOHIT` at magnitude 1, i.e.
`tohit_bonus == Some(1)` and `damage_bonus == Some(1)`; the Adamantine case asserts `TOHIT`
only, i.e. `tohit_bonus == Some(1)` and `damage_bonus == None`. Not attempted here: this is a
scanning cycle, and rewriting an assertion whose semantics I would also be the one to verify is
exactly the shape this gate exists to catch.

---

## Prior shortfall CLOSED this wave (verified by execution, not by report)

### Attempt 7's Shortfall 1 — 4 of 2,836 lib tests red → **CLOSED, 0 of 2,836 red**

```
$ cargo test --locked --lib
test result: ok. 2836 passed; 0 failed; 14 ignored; 0 measured; 0 filtered out; finished in 52.91s
EXIT=0
```

### Group A — the mapping gap was closed by mapping, and fail-closed SURVIVES

The dispatch's blocking condition for this half was that the gap must not have been closed by
making the producer swallow unknown pairs. **It was not.** Probe planted live against the real
function and removed:

```
$ python3 -c "import sys; sys.path.insert(0,'scripts')
from observer.pf1e_dashboard_producer import _doneness_verdict_uncapped as f
for pair in [('ambiguous','totally-made-up-status-xyz'),('made-up-class-xyz','totally-made-up-status-xyz')]:
    try: print('NO_RAISE (BAD):',pair,'->',f(*pair))
    except ValueError as e: print('RAISES (GOOD):',pair,'->',e)"
RAISES (GOOD): ('ambiguous', 'totally-made-up-status-xyz') -> doneness: unmapped 'ambiguous' + 'totally-made-up-status-xyz'
RAISES (GOOD): ('made-up-class-xyz', 'totally-made-up-status-xyz') -> doneness: unknown wiring_class 'made-up-class-xyz'
```

The fix maps exactly the real renamed vocabulary word and nothing wider: `status in ("unknown",
"unmeasurable") -> DONENESS_UNMEASURABLE`, checked first, with `unknown` retained for older
already-generated inventory snapshots. Every other genuinely-unmapped pair still raises.

**`docs/work-inventory.json` was not edited to make the pair disappear.** Its only commit on
this branch is still `00ca087775`, and the 11 units still carry the pair — they are now mapped,
not removed:

```
$ git log --oneline f652db7ac7..HEAD -- docs/work-inventory.json
00ca087775 fix(sd33): AT-33-E4-002 -- 4,224 unknown units reclassified to zero

$ jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json
0

$ python3 -c "import json,collections
u=json.load(open('docs/work-inventory.json'))['units']
c=collections.Counter((x.get('wiring_class'),x.get('status')) for x in u)
print('ambiguous+unmeasurable:', c[('ambiguous','unmeasurable')], 'of', len(u), 'work-inventory units')"
ambiguous+unmeasurable: 11 of 49438 work-inventory units
```

The 11 units were **not** reclassified, so the Epic-4-code-path alternative does not apply.

**A second, previously silent defect was fixed in the same commit and is recorded here because
it moves a reported figure:** 310 of 49,438 `(display, unmeasurable)` units had been falling
through the `display` branch's catch-all into `in-progress` instead of the honest
`unmeasurable`. That is an instrument correction in the dashboard's favour of honesty, not a
coverage change; it does not touch this criterion's 8,330-unit oracle population.

**F1's retarget was derived, not fitted.** The `6_308` → `6_278` pin move was re-derived live
by this scan from the live inventory, independently of the test:

```
$ python3 scripts/shape_ledger.py --inventory docs/work-inventory.json | grep -E 'F1 '
  F1      6278  Flat-constant magnitude (bare literal)
```

### Group B — the catalog count was DERIVED from the data, not fitted to the actual

Re-derived by this scan, three independent ways, all agreeing on 8,119:

```
$ grep -n '^//! Total:' src/rules_core/rules_tables/equipment_gap_tables.rs
25://! Total: 1973 rows.

$ grep -cE '^\s*EquipmentGapRow\s*\{|^\s*row\(' src/rules_core/rules_tables/equipment_gap_tables.rs
1973

$ git show f652db7ac7:src/rules_core/rules_tables/equipment_gap_tables.rs | grep -n '^//! Total:'
25://! Total: 1973 rows.

$ git log --oneline f652db7ac7..HEAD -- src/rules_core/rules_tables/equipment_gap_tables.rs
(empty — no SD-33 commit ever touched the generated table)
```

6,146 hand-authored (asserted separately in the same test, unchanged and still passing) + 1,973
generated gap rows = **8,119**. The generated table's own header and an independent count of its
row constructors agree at 1,973, and the count was already 1,973 **at the cut**, so the `8_100`
pin was stale on arrival — inherited drift, not caused by wave 6's `data/corpus` regeneration.
Cross-confirmed by a fourth, differently-computed source that already asserted the right
number in a separate cargo workspace:

```
$ grep -rn '8119' apps/desktop/src-tauri/src/equipment_catalog.rs
1005:        assert_eq!(response.entries.len(), 8119);
```

This is the legitimate case: the expectation moved because the derivation says so, and the
derivation is recorded in the test's own doc comment rather than swapped bare.

### The sweep — both moved counts grepped across `tests/`, `src/`, `apps/`, `scripts/`

```
$ for n in 8100 8_100 8119 8_119 6308 6_308 6278 6_278; do echo "### $n"; \
    grep -rn --include='*.rs' --include='*.py' --include='*.ts' --include='*.tsx' --include='*.js' \
    "\b$n\b" tests/ src/ apps/ scripts/; done
### 8100
apps/desktop/src-tauri/src/equipment_catalog.rs:992:        // derived `EXPECTED_PER_BOOK` sum): 8025 -> 8100 (+75 = the 9
### 8119
apps/desktop/src-tauri/src/equipment_catalog.rs:1000:        // touched anything) -> 8119 (+19, `BB` above; `beginner_box`
apps/desktop/src-tauri/src/equipment_catalog.rs:1005:        assert_eq!(response.entries.len(), 8119);
### 8_119
src/rules_core/equipment_resolver.rs:880:        assert_eq!(rows.len(), 8_119);
### 6308
src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs:553:    /// (`family rollup: F1 6308`) — and fails if the module's own census
### 6278
src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs:572:    /// -> `family rollup: F1 6278`, matching this test's own live
src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs:587:            f1.population, 6278,
```

**0 stale live assertions remain of 2 counts moved.** The two surviving old-number hits are both
prose: the desktop crate's derivation-history comment (`8025 -> 8100 -> 8119`, a deliberate
chain) and the F1 test's own doc comment explaining what the stale pin used to say. Neither is
an assertion.

---

## Check 2 — the other suites

| Suite | Command | Result |
|---|---|---|
| Root lib | `cargo test --locked --lib` | **2,836 of 2,836 passed**, 0 failed, 14 ignored, exit 0 |
| Root workspace | `cargo test --locked --no-run` | **FAILS TO COMPILE**, exit 101 — **0 of 543 integration test targets executed** (Shortfall 1) |
| Desktop crate | `cargo test --locked` in `apps/desktop/src-tauri`, own `CARGO_TARGET_DIR` | **548 of 548 passed**, 0 failed, exit 0 |

The desktop crate is a separate cargo workspace and was tested explicitly; a root sweep does
not cover it. The workspace failure is attributed to its exact target and commit above, not
bucketed as environmental.

## Check 3 — Epic 5 is undisturbed

```
$ python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json; echo EXIT=$?
uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False
EXIT=0

$ <re-derive rows, distinct ids, verdicts, and the unexamined set from work-inventory>
fixture-verified.combined-oracle-results.json rows 1741 distinct 1741 expected 1741 {'agree': 396, 'unverifiable': 1345}
literal-verified.oracle-results.json rows 6589 distinct 6589 expected 6589 {'agree': 415, 'unverifiable': 6174}
AT-33-E5-003.combined-oracle-results.json rows 8330 distinct 8330 expected 8330 {'agree': 811, 'unverifiable': 7519}
POP 8330 UNEXAMINED_SET_SIZE 0 SAMPLE []
agree 811 agree-with-mismatch 0
unverifiable 7519 reasonless 0
disagree 0
```

Row counts hold at **1,741 / 6,589 / 8,330**; the unexamined set is **EMPTY as a set**, not
merely zero as a count — the difference is computed and printed. **0 of 8,330** duplicate
`unit_id`. The independent consistency audit (`box_ledger` gates on the `verdict` field alone)
finds **0 of 811** `agree` rows with `ours != oracle` and **0 of 7,519** reasonless
`unverifiable`.

## Check 4 — the denominator gate is wide, unrelaxed, and still detects

```
$ scripts/verify.sh --only denominator-gate
    PASS  denominator-gate  (files_checked=55 violations=0)
RESULT: PASS  (EXIT=0)

$ git log --oneline e6f3705b3e..HEAD -- scripts/denominator_gate.py scripts/verify.sh
(empty — neither the matcher nor the stage config changed since attempt 7)
```

Scope **widened** from 53 files at attempt 7 to **55 of 55 files checked**, with the matcher
untouched — widened, not blinded. Detection re-proven live, probe planted in a scanned
`*_cycle_receipt.md` and removed:

```
$ <bare hundred-percent token appended to a scanned receipt>
VIOLATION .../AT-33-E6-001-attempt7_cycle_receipt.md:365: Probe line: coverage reached 100% across the board.
files_checked=55
violations=1
$ <probe removed>
files_checked=55
violations=0
```

## Check 5 — `## Open blockers` holds no entry

```
$ <locate the real '## Open blockers' heading by line, bound it at the next '## ', strip <details> archives>
heading lines: [302]
section lines 302 to 388
ACTIVE ### entries outside <details>: 0
```

A naive `sed -n '/## Open blockers/,$p'` false-matches on the frontmatter `status:` line, which
quotes the string; the real heading is at line 302. **0 of 0 active entries.**

## Check 6 — the rest of the scan

| Item | Result | Command |
|---|---|---|
| work-inventory `unknown` | **0 of 49,438 units** | `jq '[.units[]\|select(.status=="unknown")]\|length'` |
| Kanban-cited receipts | **0 missing of 31 cited paths** | path-existence loop over `kanban.md` |
| Hardcoded exclusion lists | **`EXCLUDED_BOOKS = frozenset()`, size 0** — no carve-out in the closure instruments | `python3 -c "...print(repr(P.EXCLUDED_BOOKS), len(...))"` |
| Corpus integrity | **137 of 137** changed records are modifies; **0 of 137** lost license/PI metadata; **0 of 137** shrank `raw_tokens` | corpus audit vs `f652db7ac7` |
| Epic 3 artifact | at the SD-33 path (`artifacts/epic-3-engine-coverage/`); SD-32's untouched | `ls` + `git log` |
| Deferral posture | 8 open forward-scope rows, **0 of 8** defer DoD scope; each carries a revisit condition | `forward-scope-register.md` |

## Criterion / card status at attempt 8

| Row | Criterion | Status | Blocks? |
|---|---|---|---|
| 1-15 | `AT-33-E1-001`..`AT-33-E4-003` | complete | no — row 14's lib-suite debt is CLOSED this wave |
| **16-18** | **`AT-33-E5-001`/`-002`/`-003`** | **complete** | **YES — `complete` over a workspace test build their own commit `2f1d52f22d` broke (Shortfall 1)** |
| 19 | `AT-33-E6-001` | blocked-escalated | this card |
| 20-21 | `AT-33-E6-002`, `AT-33-E6-003` | not-started | Epic 6's own, gated on row 19 |

## Environment finding

The shared checkout at `/home/ubuntu/workspace/repos/codex` was again unusable for scanning: at
the start of this cycle it sat **8 commits behind `origin/tranche/13`** (`HEAD` = `06e858b0e6`,
origin = `47a37804c0`) with **154 entries** in `git status --porcelain` this agent did not
create, including a staged revert of the corpus-extraction fix. Per `AGENTS.md`'s "One writer
per tree", nothing was written there; the entire scan ran in a clean detached worktree. This is
the **third consecutive wave** in which shared-checkout state, not the work, was the thing most
likely to produce a false reading — `AGENTS.md` Rule 8 applies: a warning in three consecutive
receipts is a missing mechanism, not bad luck.

**Second environment finding, recorded because it silently mis-attributed a retro event.**
`RETRO_ACTOR` does not survive between tool calls in this harness — each `bash` invocation is a
fresh shell, so an `export` in one call is gone by the next. `verify.sh` auto-appends a
verification event and, finding no `RETRO_ACTOR`, attributed this scan's denominator-gate run
to `sd31-transcribe` in `docs/retro/events/sd31-transcribe.jsonl`. The event itself is real and
correct (`head: 47a37804c0`, `worktree: sd33-r8-scan`, `PASS`); only its actor is wrong. Left
in place rather than hand-edited — the log is append-only — and corrected via a `retro.py
correction` event under the right actor.

## Disposition

**Gate FAIL. No retrospective, no sweep, no PR.** Kanban row 19 stays `blocked-escalated`.

One item stands between this bundle and closure:

1. **Shortfall 1** — update `tests/sd20_equipment_equipmods.rs:94-111` to the post-`2f1d52f22d`
   `WeaponEnhancementBonus` shape (`tohit_bonus`/`damage_bonus`, both `Option<i16>`), preserving
   each assertion's real intent, then confirm `cargo test --locked` builds and runs all 543
   integration targets. Rows 16-18 are then honestly `complete`.

Everything else this criterion checks is closed and was verified by execution this cycle,
including attempt 7's entire surviving shortfall.

- **Status:** blocked-escalated
- **Movement, four buckets:** closure 0, reclassification 0, reachability 0,
  instrument-correction 0 (see "Four buckets" above).
