# Cycle AT-33-E6-001 (attempt 11) — epic-6-closure / final-acceptance scan, post-fold re-run

- **Commit SHA:** recorded on landing (see `progress.md` entry `sd33-fold-rescan`).
- **Scanned HEAD:** `cef0ca1b39c883db7fd351a49cbf60e0d2393f00` (`origin/tranche/13`), read in a clean
  `git worktree add --detach` at `.claude/worktrees/sd33-r11-scan`.
- **Files touched:** this receipt, `progress.md`, `kanban.md` (row 19), retro events.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (no `src/` change in this cycle's own diff — a scan)
- **Wired-integration audit result:** OK_NO_TOKENS (same)
- **Acceptance criterion (verbatim):** every criterion `AT-33-E1-001`..`AT-33-E5-003` is `complete`,
  every kanban card is `complete`, `## Open blockers` holds no active entry, and the bundle's own
  gates are green.

## Gate result: **FAIL** — one shortfall, and it is SD-33's own, not inherited

Attempt 10 passed at `1bfb80d7b7`. Four commits have landed since under the operator's
2026-08-26 fold ruling (`948976aacb` fold-undine, `6e2f2f076b`/`56bbebe3d4` fold-skinwalker,
`cef0ca1b39` fold-inventory). Everything the ruling put at risk was checked specifically. Six of
the seven risk items are clean and are recorded below with the commands that show it. The
seventh — "counts moved" — is **red**.

---

## SHORTFALL 1 (BLOCKING) — a live count assertion is red at HEAD

```
$ cargo test --locked --lib
test rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::\
f1_population_matches_the_current_true_formula_bearing_count_not_the_stale_sd32_census ... FAILED

assertion `left == right` failed  (src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs:616)
  left: 6257
 right: 6260

test result: FAILED. 2844 passed; 1 failed; 14 ignored; finished in 53.88s
LIB_EXIT=101
```

**2,844 of 2,845** lib tests pass; **1 of 2,845** fails. Attempt 10 measured **2,837 of 2,837**,
all green.

### Root cause, derived by execution rather than read off the diff

The assertion's own re-derive command, run against the two inventories:

```
$ python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus
  family rollup: F1 6257          <- the committed HEAD inventory (cef0ca1b39)
$ python3 scripts/shape_ledger.py --inventory <git show 56bbebe3d4:docs/work-inventory.json> \
      --corpus-root data/corpus
  family rollup: F1 6260          <- the pre-regen inventory the assertion was pinned against
```

The Skinwalker fold (`6e2f2f076b`) correctly re-pinned this assertion **6278 → 6260**, measured
against the `docs/work-inventory.json` that was committed at that moment. The **next** fold commit,
`cef0ca1b39` (fold-inventory), then regenerated that same file — **89 of 49,438** units moved
status — and three of them left F1's `not_done_population()` gate:

```
$ <diff the F1 member id set between the two shape_ledger runs>
F1 before 6260, after 6257
left F1: bestiary_5:race_trait:skinwalker_speed
         ultimate_psionics:equipment_modifier:plusn_svs
         ultimate_psionics:equipment_modifier:special_quality_severis_enhancement_bonus
entered F1: (none)
```

One of the three is fold-attributable, two are the regen drift `cef0ca1b39`'s own receipt
discloses. The lib suite was **not re-run against the tree `cef0ca1b39` committed**: that receipt
reports `lib 2845 passed, 0 failed`, which is a true measurement of the tree *before* its own
`docs/work-inventory.json` write, not of the tree it landed. This is exactly the hazard the
dispatch names — *a record-count change compiles clean while leaving other files' hardcoded
assertions red* — arriving one commit later than the count change that caused it.

### It is SD-33's own debt, not the inherited set

```
$ git log --oneline f652db7ac7..HEAD -- src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs
6e2f2f076b fix(sd33): fold SD31-E6-F4-005's 45 recovered Skinwalker heritage records ...
6d8d6afe9e fix(sd33): AT-33-E6-001 Shortfall 1 -- map (ambiguous, unmeasurable) doneness pair ...
347e9d1a34 feat(sd33): AT-33-E3-001..004 -- engine-coverage gap closes, root-caused (rows 9-12)
```

**3 commits since the `tranche/13` cut**, so the "0 of 31 inherited failing targets carry a commit
since the cut" test that has excused the standing red set every attempt does **not** excuse this
one. `src/lib.rs` was green in attempt 10's own enumeration of the inherited set; it is red now.

### What closes it

Re-derive F1's population against the committed `docs/work-inventory.json` and re-pin the
assertion (6,260 → 6,257) with the mechanism written into its doc comment, exactly as
`6e2f2f076b` did for 6,278 → 6,260 — then re-run `cargo test --locked --lib` **after** the last
commit that writes `docs/work-inventory.json`, not before it. Per the dispatch: **no
release-notes update and no PR change** were made by this scan.

---

## RISK 1 — the Epic 5 population. **CLEAN.**

The fold added **0** units to the inventory and **0** rows to Epic 5's population. Derived as a
set, both directions, not as a size:

```
$ <id-keyed join of the two committed work-inventory.json blobs>
before 49438 units, after 49438 units;  added 0, removed 0
$ <fixture-verified|literal-verified id set  vs  the combined file's unit_id set>
pop 8330 (fixture 1741 + literal 6589)   rows 8330
pop-minus-rows 0   rows-minus-pop 0
per-file: fixture 0/0 both directions, literal 0/0 both directions
$ <Counter over each results file>
fixture-verified.combined-oracle-results.json  1741  dups 0  {'unverifiable':1345,'agree':396}
literal-verified.oracle-results.json           6589  dups 0  {'unverifiable':6174,'agree':415}
AT-33-E5-003.combined-oracle-results.json      8330  dups 0  {'unverifiable':7519,'agree':811}
reasonless unverifiable: 0 / 0 / 0
```

Rows hold at **1,741 / 6,589 / 8,330**; the unexamined set is **empty in both directions**, per
file and combined; **0 of 8,330** disagree; **0 reasonless `unverifiable` of 7,519**; **0**
duplicate `unit_id`s. No oracle re-run was owed: `fixture-verified` (1,741) and `literal-verified`
(6,589) are byte-identical counts before and after the regeneration, and none of the 89 moved
units landed in either status.

**Why the fold added no units, verified rather than assumed:** the inventory is a census of the
pinned PCGen `.lst` oracle, not a walk of `data/corpus/**`. All 74 Skinwalker units already
existed as `not-ingested`. The fold moved them:

```
$ <status counter over the 173 bestiary_5 race_trait / skinwalker units>
before  not-ingested 161, ingested-magnitude 6, text-complete 3, literal-verified 2, grounded 1
after   not-ingested 116, grounded 37, text-complete 17, literal-verified 2, ingested-magnitude 1
```

**45 of 161** left `not-ingested` — exactly the 45 records the ruling put in scope.

## RISK 2 — counts moved. **RED, see Shortfall 1.**

The sweep across `tests/`, `src/`, `apps/`, `scripts/` did run and did land 8 fixes (verified: the
old figures 831, 370, 6278, 18 all appear only inside `<old> -> <new>` provenance comments, never
as a live assertion). It missed this one target, because the count that broke it moved in a
*later* commit than the sweep.

Two stale prose figures found and recorded, neither load-bearing, neither a live assertion:
`src/bin/v06_work_inventory.rs:4308` still says "every one of the **831** currently-ingested
`race_trait` records" (now 910), and `src/rules_core/race_resolver.rs:3105`'s assertion *message*
says "same as the other **370**" (now 415, correctly re-pinned at `:2680` and `:3378`).

## RISK 3 — corpus integrity. **CLEAN, and the population grew.**

```
$ cargo run --locked --bin corpus_literal_sweep ; echo SWEEP_EXIT=$?
corpus-literal-sweep: 48699 records examined of 51473 read, 413288 tokens compared (9 synthesized),
                      51460 digests checked, 0 findings
corpus-literal-sweep: CLEAN
SWEEP_EXIT=0
```

**48,699 examined**, up from attempt 10's **48,634** — **+65**, exactly the fold's 65 new records.
Digests **51,395 → 51,460** (+65); tokens compared **412,734 → 413,288** (+554). Not a vacuous
pass: the new records are inside the examined population.

**Detection re-proven live, on one of the folded records**, against a `cp -al` hardlink copy under
`--repo-root`:

```
$ <plant {"key":"SOURCEPAGE","value":"p.999999"} into werebat_kin_ability_scores.json>
corpus-literal-sweep: 48699 records examined ..., 413289 tokens compared ..., 1 findings
corpus-literal-sweep: MISMATCH data/corpus/bestiary_5/race_trait/skinwalker/werebat_kin_ability_scores.json:
                      token not byte-present in corpus token closure: SOURCEPAGE:p.999999
$ <restore>   git status --porcelain -> (empty);  sweep -> 48699 examined, 0 findings, CLEAN
```

Tokens compared moved 413,288 → **413,289** (the one planted token) and the finding named the
exact record and token. **Residue note, recorded because it nearly went unnoticed:** `cp -al`
hardlinks, so writing the mutant through the scratch path truncated the *shared inode* and dirtied
the real `data/corpus/**` file. Caught by the `git status --porcelain` that follows every git
write, restored with `git checkout --`, and both trees verified clean. A future probe must
`rm` the scratch file before writing it, or copy without `-l`.

## RISK 4 — license / PI on every new record. **CLEAN, checked by field, all 65.**

```
$ <load every data/corpus/*.json in 6e2f2f076b and tally its fields>
corpus files in the fold commit: 66 (65 new + adopted_race_skinwalker.json, modified)
license:     OGL 58,  PI-REDACTED 8
pi_field:    null 58, "description" 8
pi_marker:   null 58, "redacted" 8
records with empty raw_tokens: 0
records missing any of the 10 required top-level keys: 0
```

The redaction count is cross-checked against the oracle independently of the generator's own
claim: `grep -c 'DESCISPI:YES' skinwalker_abilities_race_subrace.lst` → **8**, matching the 8
`PI-REDACTED` records exactly. `data/corpus/bestiary_5/LICENSE.json` moves
`records_processed` 279 → **344** (+65) and `records_redacted` 9 → **17** (+8), both consistent.
Nothing lost a license stamp and nothing lost `raw_tokens`.

## RISK 5 — the Skinwalker records are real. **2 hand-traced by this scan, both verified.**

Pin confirmed first: `scripts/pcgen-oracle-pin.env` names
`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`;
`git -C ~/workspace/repos/pcgen rev-parse HEAD` → identical. The record's cited
`source.sha256` is `a5b5c3f65d28e5a01b935ffc2a177752184be170e340fdddc14cdbf3ba6c3b27`;
`sha256sum` of the pinned `.lst` returns the same digest. (The SD-32 operator-supplied oracle slot
carries a byte-identical copy of this file — `md5sum` agrees — so the two oracle roots the
instruments resolve do not disagree here.)

**`skinwalker_werebat_kin`, cited `…skinwalker_abilities_race_subrace.lst:15`.** Line 15 tab-split
carries a display name plus **13** tokens: `KEY`, `CATEGORY`, `TYPE`, `PREMULT`, `DESC`,
`TEMPLATE`, `ABILITY`, and **6** `ASPECT`s. The record carries **13 of 13** `raw_tokens`,
byte-for-byte, in the same order, and nothing else; `data.name` is the line's own field-1
`Werebat-Kin (Bloodmarked)`. **Verified.**

**`werebat_kin_ability_scores`, cited `…:19`.** Line 19 carries **9** tokens: `KEY`, `CATEGORY`,
`TYPE`, `PREABILITY`, `DESC`, `BONUS:STAT|INT|2|TYPE=Racial`, `BONUS:STAT|WIS|-2|TYPE=Racial`,
`SOURCEPAGE:p.xx`, `FACT:Skinwalker_ReplaceAbilityScores|True`. The record carries **7 of 7**
non-`BONUS` tokens verbatim and both `BONUS` chains in `raw_bonus_chains`
(`["STAT","INT","2","TYPE=Racial"]`, `["STAT","WIS","-2","TYPE=Racial"]`) — **9 of 9** accounted
for, the same split attempt 10 hand-verified on `hellscourge`. `sets_replace_flags`
`["Skinwalker_ReplaceAbilityScores"]` is the row's own `FACT` token. **Verified.**

One thing checked because it looked wrong: the selector's `sets_replace_flags` lists 4 flags while
its only flag-bearing token is a **negated** `!PREFACT` guard. It is not read from that guard —
`ingest_race_traits.rs` derives it from the selector's own resolved replacement grants, and the
negated guard is read separately by `race_trait_picker::exclusion_guard_flags`. The generator
also, correctly, **does not** synthesize the Aasimar/Tiefling-shaped `PREVAREQ:<flag>,0` token
onto a selector that already carries its own literal `PREMULT` — which is the fabrication
`corpus_literal_sweep` refused (36 findings) and the lane fixed rather than suppressed.

## RISK 6 — the Undine fixtures execute. **3 entries, 30 sample points, all run.**

The dispatch's "103" is corrected, and the lane's own correction is confirmed independently:
103 is the count of raw `Undine` **string occurrences**, not entries.

```
$ grep -c -i undine tests/fixtures/rules_core/derived-evaluator-fixtures.json     -> 106
$ git show 075c4543c9:tests/fixtures/.../derived-evaluator-fixtures.json | grep -c -i undine -> 0
$ <len(race_trait_formula_entries), sum(len(expected_at_sample_points))>          -> 3, 30
$ git show worktree-wf_be4660f2-72a-3:tests/fixtures/.../derived-evaluator-fixtures.json | <same>
                                                                                 -> 3, 30
```

**3 of 3** entries on the branch reached HEAD; **0** dropped; the three `unit_id`s match exactly
(`undine_acid_breath`, `undine_nereid_fascination`, `undine_ooze_breath`). Each carries 3 formulas
× 10 sample points = **90 scalar assertions**; `run_race_trait_formula_bar_check` asserts
`fixtures_total == 3` and `cleared.len() == 3`. All 3 clear in this scan's own `--lib` run (the
suite's single failure is Shortfall 1, in a different module).

## RISK 7 — fixture discipline. **CLEAN, verified at the source, not from the receipt.**

`scripts/derive_race_trait_formula_fixtures.py` imports only `argparse, hashlib, json, math, os,
re, sys, pathlib` — **no engine module, no Rust table, no file under `data/corpus/`** (the artifact
the engine actually evaluates). The expected values are computed by its own per-formula Python
functions.

The pinned bytes were read directly and compared to the committed fixture:

```
$ sha256sum <pinned>/advanced_race_guide/arg_abilities_race.lst
ea475d7c207e2c4b182019997689ca48a090019620250e43bc4e85b0ff77c3a3   == entry's upstream_lst_sha256
$ sed -n '776p' <that file> | tr '\t' '\n'
  KEY:Undine ~ Acid Breath
  BONUS:VAR|Undine_AcidBreath_Times|1
  BONUS:VAR|Undine_AcidBreath_Dice|min(floor((TL+1)/2),5)
  BONUS:VAR|Undine_AcidBreath_DC|10+(TL/2)+CON
```

Byte-identical to the entry's `formulas`. Two expected values re-computed by hand under PCGen's
integer-division convention: `TL=1,CON=0` → `Dice=min(floor(2/2),5)=1`, `DC=10+0+0=10`; `TL=2,CON=2`
→ `Dice=min(floor(3/2),5)=1`, `DC=10+1+2=13` — both match. **Not a mirror.**

## The rest of the scan

### Build and suites

```
$ cargo test --locked --no-run ; echo NO_RUN_EXIT=$?        -> 0
$ cargo test --locked --no-run 2>&1 | grep -c "Executable tests/"  -> 543
$ ls tests/*.rs | wc -l                                            -> 543
$ cargo test --locked --lib                 -> 2844 passed; 1 FAILED; 14 ignored   (Shortfall 1)
$ cd apps/desktop/src-tauri && cargo test --locked   # own CARGO_TARGET_DIR
                                            -> 548 passed; 0 failed;  DESKTOP_EXIT=0
```

**543 of 543** integration targets build; desktop **548 of 548** pass, unchanged from attempt 10.

### `## Open blockers` — **0 active entries.**

```
$ grep -n "^## Open blockers" progress.md                     -> 302
$ awk 'NR>302 && /^## /{print NR; exit}' progress.md          -> 470  (## Cycles)
$ sed -n '302,470p' progress.md | grep -n "^###\|^<details>\|^</details>"
  50:<details>  53:### corpus_literal_sweep mismatch on 10 weapon records …  83:</details>
  98:<details> 101:### rending_claw_blades … EQMOD-resolution gap …          167:</details>
```

Both `###` entries lie **inside** `<details>` historical blocks. **0 active of 2 historical**, and
the section's own text records both as CLEARED. No fold lane filed a new one.

### `kanban.md` — **24 of 24** rows `complete`

Rows 1–21 as attempt 10 left them, plus the ruling's three new rows: 22 `fold-skinwalker`,
23 `fold-undine`, 24 `fold-inventory`. No row is `not-started`, `in-progress`,
`returned-to-backlog`, or `blocked-escalated`. Row 19 is the card this cycle re-opens: it reads
`complete` on attempt 10's PASS, and that PASS no longer covers the tree.

### Denominator gate

```
$ python3 scripts/denominator_gate.py --check
files_checked=66   violations=0
```

**0 violations of 66 files** (60 at attempt 10; the 6 new fold receipts are in scope, not skipped).
Detection re-proven live inside that scope and removed:

```
$ <append "PROBE: coverage is 100% complete." to fold-inventory_cycle_receipt.md>
VIOLATION .../fold-inventory_cycle_receipt.md:227: PROBE: coverage is 100% complete.
files_checked=66  violations=1
$ <restore>   files_checked=66  violations=0 ;  git status --porcelain -> (empty)
```

### The Epic 5 ledger

```
$ python3 scripts/box_ledger.py --check \
    --oracle-results .../epic-5-reverification/AT-33-E5-003.combined-oracle-results.json
uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False
BOX_EXIT=0
$ jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json   -> 0
$ jq '.units|length' docs/work-inventory.json                                  -> 49438
```

Work-inventory `unknown` **0 of 49,438**, unchanged across the regeneration.

### Open deferrals — **3 open, 0 of 3 defer live DoD scope**

`python3 scripts/retro.py summary --since 2026-08-24 --json` → `deferrals.open = 3`, the same
three attempt 10 left standing (`REGISTERED_POOL_GROUPS` widening; `pf1e_dashboard_producer.py`
`unmeasurable` recognition; the COMBAT non-AC aggregation surface). Each carries a revisit
condition (**3 of 3**). No fold lane opened a new one.

### The full workspace run, and the inherited failing set

```
$ cargo test --locked --no-fail-fast ; echo NFF_EXIT=$?
NFF_EXIT=101
$ <attribute every "test result:" line back to its own "Running" line>
599 executables reporting
total: 7,987 passed, 47 failed  = 8,034 of 8,034 executed tests
FAILING SUITES: 30, carrying 47 failures
```

**30 of 599** suites fail, carrying **47 of 8,034** executed tests. Attempt 10 measured **31** and
**49** against **8,026** executed. The set did not grow — but it **changed composition**, and the
change is what Shortfall 1 is:

| Target | Attempt 10 | Attempt 11 | Attribution |
|---|---|---|---|
| `src/bin/ingest_races.rs` | 43/**1** | green | fixed by the Skinwalker fold |
| `tests/sd27_alternate_racial_trait_reachability.rs` | 13/**2** | green | fixed by the Skinwalker fold |
| `src/lib.rs` | 2837/**0** | 2844/**1** | **broken by the fold — Shortfall 1** |

−2 suites / −3 failures, +1 suite / +1 failure = the observed 31→30, 49→47. Two genuine fixes and
one genuine break, not a wash.

The other **29 of 30** are the inherited set, re-derived here rather than carried from attempt 10:

```
$ for f in <each of the 29 targets>; do git log --oneline f652db7ac7..HEAD -- "$f" | wc -l; done \
    | awk '{s+=$1} END {print s}'
0
```

**0 of 29** carry a single commit since the `tranche/13` cut. `src/lib.rs` carries **3**. That is
the whole difference between debt this criterion may leave standing and debt it may not.

---

## Movement, four buckets

- **Closure 0.** The criterion is not satisfied; kanban row 19 moves `complete` →
  `blocked-escalated`.
- **Reclassification 0.**
- **Reachability 0** (this cycle is a scan; it wired nothing).
- **Instrument-correction 1** — `fold-inventory_cycle_receipt.md`'s fold-attribution split is
  corrected below. Moves no gate, but it is the figure a future reader would copy.

### Instrument correction — the fold-attribution split, 14/75, is 50/39

That receipt attributes the 89 moved units with a substring test on the unit id
(`'skinwalker' in id`). **36 of the 50** bestiary_5 units the fold really moved are named
`were<x>_kin_<field>` and carry no `skinwalker` substring at all, so the heuristic silently
credited them to unrelated drift:

```
$ <id-keyed join, split by book prefix rather than by substring>
status-changed total: 89
  bestiary_5 50   (of which 'skinwalker' in id: 14; were*_kin_*: 36)
  other books 39  (ultimate_psionics 11, advanced_players_guide 10, ultimate_equipment 7,
                   advanced_race_guide 5, core_rulebook 2, mythic_adventures 2,
                   inner_sea_gods 1, ultimate_intrigue 1)
$ <check each of the 50 against the fold's own new corpus filenames>
all 50 bestiary_5 ids have a matching data/corpus/bestiary_5/race_trait/skinwalker/*.json: True
```

The honest split is **50 fold-attributable / 39 drift**, not 14/75. The conclusion the receipt drew
from it is unaffected and independently re-derived above (Epic 5's population is untouched), but
the fold's real footprint on the inventory is three and a half times what it reported — and one of
the three units that left F1 sits inside the 36 it missed.

---

## `scripts/verify.sh`

Not run end-to-end: attempt 10, attempt 9 and the corpus-sweep lane each recorded
`site-dashboard-check` hanging on the producer's own unbounded `v06_work_inventory` call, and this
scan's verdict does not rest on it. The load-bearing stages were each run directly and are reported
above — `corpus-sweep` (**0 findings of 48,699 records examined**), `denominator-gate` (**0
violations of 66 files**), `root-lib` (**2,844 of 2,845, 1 FAILED**), `desktop` (**548 of 548**),
plus `cargo test --locked --no-run` (**543 of 543**) and the full `--no-fail-fast` workspace run.

---

## Status: blocked-escalated

- **Notes:** Eleven scans have now run on this bundle; ten of them halted it, and this is the
  tenth. The bundle was closed once, on attempt 10, and that PASS was correct **for the tree it
  scanned**. The fold ruling changed the tree, and the change carried a real regression: a live
  count assertion that the Skinwalker fold re-pinned correctly against the inventory of its own
  moment, and that the fold-inventory commit then invalidated by regenerating that inventory
  without re-running the suite afterwards. Nothing about the folded *content* is short — the 65
  Skinwalker records hand-trace clean to the pinned oracle, keep their license and PI stamps,
  and are genuinely inside the sweep's grown examined population; the 3 Undine fixtures all
  arrived, all execute, and are not a mirror. The bundle is one re-pin and one re-run from green.
- **Next-cycle plan:** a one-file cycle re-pinning
  `formula_interpreter_corpus_wide.rs:616` to **6,257** with its mechanism, then
  `cargo test --locked --lib` **after** the final `docs/work-inventory.json` write; then
  `AT-33-E6-001` attempt 12. `release-notes.md` and PR #377 stay untouched until that passes.
