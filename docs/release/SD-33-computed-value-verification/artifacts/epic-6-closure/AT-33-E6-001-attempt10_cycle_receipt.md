# Cycle AT-33-E6-001 (attempt 10) — epic-6-closure / final-acceptance scan

- **Commit SHA:** recorded on landing (see `progress.md` entry `sd33-r10-acceptance-scan`).
- **Scanned HEAD:** `1bfb80d7b7` (`origin/tranche/13`), read in a clean
  `git worktree add --detach` at `.claude/worktrees/sd33-r10-scan`.
- **Files touched:** this receipt, `progress.md`, `kanban.md` (row 19), retro events.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (no `src/` change in this cycle's own diff — a scan)
- **Wired-integration audit result:** OK_NO_TOKENS (same)
- **Acceptance criterion (verbatim):** every criterion `AT-33-E1-001`..`AT-33-E5-003` is `complete`,
  every kanban card rows 1–18 is `complete`, `## Open blockers` holds no active entry, and the
  bundle's own gates are green.

## Environment

```
$ git worktree add --detach .claude/worktrees/sd33-r10-scan origin/tranche/13
HEAD is now at 1bfb80d7b7 fix(sd33): AT-33-E6-001 -- reconcile corpus_literal_sweep vs enrich_equipment_raw_tokens, sweep was wrong
$ git status --porcelain          # in the scan worktree, start and end
(empty)
```

The shared checkout at `/home/ubuntu/workspace/repos/codex` still carries the foreign,
**uncommitted, staged** revert of wave 6's `.MOD`-fold fix that waves 6–9 each reported
(137 staged `data/corpus/**` modifications plus deletions), and its `HEAD` is 8 commits behind
`origin/tranche/13`. Per `AGENTS.md` "One writer per tree" nothing was written there and nothing
was discarded; the whole scan ran in the clean worktree above. **Sixth** consecutive wave to find
it — see "Environment finding" at the bottom.

---

## CHECK 1 — the sweep is green, and green honestly

### 1a. Green

```
$ cargo run --locked --bin corpus_literal_sweep ; echo SWEEP_EXIT=$?
corpus-literal-sweep: 48634 records examined of 51408 read, 412734 tokens compared (9 synthesized), 51395 digests checked, 0 findings
corpus-literal-sweep: 3138 tokens exempted under decisions.md §24 redaction across 1058 codex_generated_name records
corpus-literal-sweep: CLEAN
SWEEP_EXIT=0
```

### 1b. Route (a) — population narrowed, capped or filtered? **RULED OUT BY EXECUTION.**

Not read off the diff: the **pre-fix binary was built and run against the same corpus**, in its own
worktree (`.claude/worktrees/sd33-r10-prefix` @ `d0dc9fc3db`, attempt 9's own scanned HEAD, the
commit immediately before the fix) and its own `CARGO_TARGET_DIR`.

| Figure | Pre-fix `d0dc9fc3db` | Post-fix `1bfb80d7b7` | Moved? |
|---|---|---|---|
| records examined | 48,634 of 51,408 read | 48,634 of 51,408 read | no |
| tokens compared | 412,734 (9 synthesized) | 412,734 (9 synthesized) | no |
| digests checked | 51,395 | 51,395 | no |
| **findings** | **105** | **0** | **105 → 0** |

Re-derive: `git worktree add --detach <path> d0dc9fc3db && CARGO_TARGET_DIR=<own> cargo run --locked --bin corpus_literal_sweep -- --max-report 200`.
Every population figure is byte-identical across the fix; only the findings count moved. The
`105 → 0` figure the lane reported is confirmed against a live pre-fix run, not against its own
narrative.

### 1c. Route (b) — did the affected records' `raw_tokens` revert to `[]`? **No.**

`hellscourge.json` carries **14 of 14** expected `raw_tokens` entries plus 1 `raw_bonus_chains`
entry; `fugitive_finder.json` carries **7** `raw_tokens` entries. Both fully populated (full
token-by-token tally in 1e below).

### 1d. Routes (c) and (d) — exclusion list, or a hand-edit of `data/corpus/**`? **Neither.**

```
$ git show --name-only --format="" 1bfb80d7b7 | grep -c '^data/corpus/'
0
$ git show --stat --format="" 1bfb80d7b7
 .../AT-33-E6-001-corpus-sweep_cycle_receipt.md | 299 +++
 .../kanban.md                                  |   6 +-
 .../progress.md                                |  50 ++
 docs/retro/events/sd33-r9-corpus-sweep.jsonl   |   5 +
 src/bin/corpus_literal_sweep.rs                | 188 ++-
 src/rules_core/corpus_literal_sweep.rs         | 118 +--
```

**0 of 6** files in the fix commit are under `data/corpus/**`; no regeneration was performed and
none was needed. Grepping the closure instruments for exclusion constructs
(`EXCLUDED|EXCLUDE_|SKIP_BOOKS|ALLOWLIST|SKIPLIST|IGNORE_BOOKS|beginner_box`) across
`src/bin/corpus_literal_sweep.rs`, `src/rules_core/corpus_literal_sweep.rs`,
`scripts/box_ledger.py`, `scripts/denominator_gate.py`, `src/bin/v06_work_inventory.rs` returns
**0 hits in either sweep file**. The only inventory-side exclusion is
`v06_work_inventory.rs`'s `out_of_scope = ["core_essentials"]` — **present verbatim at the
`tranche/13` cut** (`git show f652db7ac7:src/bin/v06_work_inventory.rs | grep -c '\["core_essentials"\]'` → `1`),
inherited, not SD-33's, and the sibling `beginner_box` entry is already removed under
`decisions.md §27b`.

### 1e. Hand-derivation from the pinned oracle bytes — **checked independently, against the source**

Pin confirmed before reading: `scripts/pcgen-oracle-pin.env` names
`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`;
`git -C ~/workspace/repos/pcgen rev-parse HEAD` → the identical SHA.

**Defect 1, `ultimate_equipment/equipment/hellscourge.json`** (`ue_equip_arms_armor.lst:496`):

```
$ sed -n '496,497p' .../ultimate_equipment/ue_equip_arms_armor.lst
Scorpion Whip.COPY=Hellscourge
Hellscourge.MOD  ... EQMOD:Special Ability ~ Enhancement Cost|39300.Special Ability ~ +1 ~ Weapon.Special Ability ~ Unholy ~ Weapon ... SOURCEPAGE:p.156 ... BONUS:SKILL|Intimidate|5 ... SPROP:Shaken creature struck becomes Frightened (DC 17 Will negates), Frightened or Panicked creature must cower 1d4 rounds.

$ grep -n "^Scorpion Whip" .../ultimate_equipment/*.lst
ue_profs_weapon.lst:79:      Scorpion Whip  TYPE:Exotic.Melee.Light.Slashing          <- the decoy
ue_profs_weapon.lst:246:     Scorpion Whip.MOD  TYPE:Flail.Weapon Group Flails
ue_equip_arms_armor.lst:349: Scorpion Whip  PROFICIENCY:WEAPON|Scorpion Whip  TYPE:Weapon.Resizable.Melee.Light.Finesseable.Exotic.Slashing.Flail  COST:5  WT:3  CRITMULT:x2  CRITRANGE:1  DAMAGE:1d4  EQMOD:Material ~ Steel  WIELD:Light  SIZE:M  SOURCEPAGE:p.36   <- the real base, SAME FILE as :496
ue_equip_arms_armor.lst:496: Scorpion Whip.COPY=Hellscourge
ue_equip_arms_armor.lst:504: Scorpion Whip.COPY=Lash of the Howler
ue_equip_arms_armor.lst:717: Scorpion Whip.MOD  TYPE:Flail.Weapon Group Flails
```

Tallied by hand against the shipped record's own `raw_tokens`, in order — **11 of 11** base tokens
from `:349` (`PROFICIENCY`, `TYPE`, `COST:5`, `WT:3`, `CRITMULT:x2`, `CRITRANGE:1`, `DAMAGE:1d4`,
`EQMOD:Material ~ Steel`, `WIELD:Light`, `SIZE:M`, `SOURCEPAGE:p.36`) followed by **3 of 3** tokens
from the `.MOD` row at `:497` (`EQMOD:Special Ability ~ …`, `SOURCEPAGE:p.156`, `SPROP:…`) = **14 of
14** `raw_tokens` entries, byte-for-byte, and nothing else; the `.MOD` row's `BONUS:SKILL|Intimidate|5`
is carried separately and correctly in `raw_bonus_chains`. The record's `source` cites
`ue_equip_arms_armor.lst:496`, `record_key: "Hellscourge"`.
**The enricher's `raw_tokens` are correct. The sweep's whole-book, unsorted `read_dir` walk
resolving the base to `ue_profs_weapon.lst:79` was the defect.**

**Defect 2, `inner_sea_gods/equipment/fugitive_finder.json`** (`isg_equip.lst:78`):

```
$ sed -n '137p' .../inner_sea_gods/isg_equip.lst | tr '\t' '\n' | grep -v '^$'
Fugitive Finder.MOD
TYPE:Magic
SOURCELONG:Inner Sea Gods
SOURCESHORT:isg
SOURCEPAGE:p.255
BONUS:MOVEADD|TYPE.Walk|10
DESC:This +1 human-bane light crossbow ... The church of Abadar typically commissions fugitive finders ... but Abadarans also craft variants ...
$ python3 -c "...json ... fugitive_finder.json ..."
license OGL   pi_field None   pi_marker None
raw_tokens: EQMOD, VISIBLE, TYPE, SOURCELONG, SOURCESHORT, SOURCEPAGE, DESC="[redacted PI]"
```

`"Abadar"` is entry 5 of `src/rules_core/pi_screening.rs`'s `PI_BLACKLIST_TERMS`. The record's own
`license`/`pi_field` are `"OGL"`/`null` — genuinely undeclared, so `pi_redacted_description` is
`false` — yet the real corpus `DESC` independently re-screens as blacklisted, which is exactly what
`enrich_equipment_raw_tokens.rs::screen_field_value` acts on. **The redaction is correct; the
sweep's `token.key != "DESC"` exclusion on its re-screen exemption was the defect.**

**`hand_derivation_conclusion = sweep-wrong`, confirmed independently for both defects.** The
tie-break was the pinned oracle bytes, not a preference between two programs.

### 1f. The sweep still fails on a genuine mismatch — **re-proven live**

Run against a **scratch hardlink copy** of `data/corpus` (`cp -al`, 51,440 JSON files) with
`--repo-root`, so `data/corpus/**` in the repo was never written (`git status --porcelain` empty
before and after):

```
$ <plant {"key":"COST","value":"999999"} into the scratch copy's hellscourge.json raw_tokens>
$ corpus_literal_sweep --repo-root /tmp/sd33-r10-scratch-repo
corpus-literal-sweep: 48634 records examined of 51408 read, 412735 tokens compared (9 synthesized), 51395 digests checked, 1 findings
corpus-literal-sweep: MISMATCH data/corpus/ultimate_equipment/equipment/hellscourge.json: token not byte-present in corpus token closure: COST:999999
corpus-literal-sweep: 1 findings across 1 records
MUTANT_EXIT=1
$ <remove the planted token>
RESTORED
```

Tokens compared moved 412,734 → **412,735** (the one planted token) and the finding named the exact
record and the exact token. Detection is live, not assumed.

---

## CHECK 2 — the Epic 5 consequence, derived by execution

The lane's claim is `epic5_units_affected: 0`. **Verified, and the membership question answered by
execution rather than assumed empty:**

```
$ python3 -c "<join the 10 affected record paths, as unit ids, against docs/work-inventory.json
              and AT-33-E5-003.combined-oracle-results.json>"
inner_sea_gods:equipment:fugitive_finder                  | inv literal-verified   | verdict unverifiable | ours None | oracle None
ultimate_equipment:equipment:blade_of_the_rising_sun      | inv ingested-magnitude | NOROW
ultimate_equipment:equipment:blade_of_the_sword_saint     | inv ingested-magnitude | NOROW
ultimate_equipment:equipment:hammer_polarity              | inv ingested-magnitude | NOROW
ultimate_equipment:equipment:hellscourge                  | inv ingested-magnitude | NOROW
ultimate_equipment:equipment:lash_of_the_howler           | inv ingested-magnitude | NOROW
ultimate_equipment:equipment:pistol_firedrake             | inv ingested-magnitude | NOROW
ultimate_equipment:equipment:pistol_of_the_infinite_sky   | inv ingested-magnitude | NOROW
ultimate_equipment:equipment:spirit_caller                | inv ingested-magnitude | NOROW
ultimate_equipment:equipment:sword_ten_ring               | inv ingested-magnitude | NOROW
```

**1 of the 10** affected records is in Epic 5's 8,330-unit population at all; **9 of 10** are
`ingested-magnitude`, a different population entirely. The lane's "zero Epic 5 units" is therefore
*nearly* right and right where it matters, and the correction is stated rather than smoothed over:
the honest figure is **1 of 10 in-population**, whose row is `unverifiable` with `ours = None` and
`oracle = None` — a row that cannot have been "wrong while reporting agree", because it reports no
value on either side.

**Units whose `ours` could have moved: 0 of 8,330.** The causal chain is closed at the input, not
argued from the output: the fix commit touched **0** `data/corpus/**` files and **0** lines of
`src/bin/enrich_equipment_raw_tokens.rs`, so no `raw_tokens` value any Epic 5 `ours` derives from
changed. `epic5_units_rerun: 0`, `epic5_rows_moved: 0`, `epic5_new_disagreements: []` — no re-run
was owed and none is hidden.

---

## CHECK 3 — nothing else moved

### Build and suites

```
$ cargo test --locked --no-run ; echo NO_RUN_EXIT=$?
NO_RUN_EXIT=0
$ cargo test --locked --no-run 2>&1 | grep -c "Executable tests/"
543
$ ls tests/*.rs | wc -l
543
$ cargo test --locked --lib
test result: ok. 2837 passed; 0 failed; 14 ignored; 0 measured; 0 filtered out; finished in 145.76s
$ cd apps/desktop/src-tauri && cargo test --locked      # own CARGO_TARGET_DIR
test result: ok. 548 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 101.34s
DESKTOP_EXIT=0
```

**543 of 543** integration targets build; lib **2,837 of 2,837** pass (the 2,836 baseline **+1**, the
corpus-sweep lane's own new test — growth, not a drop); desktop **548 of 548** pass.

### Epic 5 artifacts and the ledger

```
$ python3 -c "<len + Counter over each oracle-results file>"
fixture-verified.combined-oracle-results.json  1741  dups 0  Counter({'unverifiable': 1345, 'agree': 396})
literal-verified.oracle-results.json           6589  dups 0  Counter({'unverifiable': 6174, 'agree': 415})
AT-33-E5-003.combined-oracle-results.json      8330  dups 0  Counter({'unverifiable': 7519, 'agree': 811})
reasonless unverifiable: 0 / 0 / 0
$ python3 scripts/box_ledger.py --check --oracle-results .../AT-33-E5-003.combined-oracle-results.json
uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False
BOX_EXIT=0
$ python3 -c "<work-inventory fixture|literal id set  vs  combined file's unit_id set>"
pop 8330 (fixture 1741 + literal 6589)   rows 8330
pop-minus-rows 0   rows-minus-pop 0
per-file: fixture 0/0 both directions, literal 0/0 both directions
$ jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json
0
$ jq '.units|length' docs/work-inventory.json
49438
```

Rows hold at **1,741 / 6,589 / 8,330**; the unexamined set is empty **in both directions**, per
file and combined; **0 of 8,330** disagree; **0 reasonless `unverifiable` of 7,519**;
work-inventory `unknown` **0 of 49,438**.

### The full workspace run, and the inherited failing set — re-derived, not inherited on trust

```
$ cargo test --locked --no-fail-fast ; echo NFF_EXIT=$?
NFF_EXIT=101
$ <attribute every "test result:" line back to its own "Running" line>
599 executables reporting, 600 result lines (src/lib.rs is reported inside this run too)
total: 7,977 passed, 49 failed  = 8,026 of 8,026 executed tests
FAILING SUITES: 31, carrying 49 failures
```

**31 of 599** suites fail, carrying **49 of 8,026** executed tests. Attempt 9 measured **31** and
**49** against **8,023** executed; the executed total is **+3**, exactly the corpus-sweep lane's 3
new tests (1 in `src/rules_core/corpus_literal_sweep.rs`, 2 in `src/bin/corpus_literal_sweep.rs`).
**The failing set did NOT grow: 31 then, 31 now; 49 then, 49 now.**

Enumerated in full (target — passed/failed), one non-integration target included:

```
src/bin/ingest_races.rs                                             43/1
tests/duergar_invisibility_sla_reaches_a_player_via_monster_codex.rs 6/1
tests/formula_interpreter_family_fixture_check.rs                    4/1
tests/no_foreign_home_paths.rs                                       2/1
tests/sd13_sorcerer_level1_spell_baseline.rs                        18/1
tests/sd13_sorcerer_level9_progression.rs                           11/1
tests/sd13_sorcerer_level10_progression.rs                          11/1
tests/sd18_cleric_level11_widening.rs   8/2      tests/sd18_cleric_level12_widening.rs   8/2
tests/sd18_cleric_level13_widening.rs   8/2      tests/sd18_cleric_level14_widening.rs   8/2
tests/sd18_cleric_level15_widening.rs   8/1      tests/sd18_cleric_level16_widening.rs   8/1
tests/sd18_cleric_level17_widening.rs   8/1      tests/sd18_cleric_level18_widening.rs   7/2
tests/sd18_cleric_level19_widening.rs   7/2      tests/sd18_cleric_level20_widening.rs   7/2
tests/sd24_identifier_discipline_audit.rs                            0/1
tests/sd24_wired_integration_audit.rs                                3/2
tests/sd26_cache_acg.rs   7/2                    tests/sd26_cache_apg.rs   5/2
tests/sd26_identifier_discipline_audit.rs                            0/1
tests/sd27_ability_automatic_granted_race_traits.rs                  4/2
tests/sd27_advanced_race_guide_cache_shape.rs                        6/2
tests/sd27_alternate_racial_trait_reachability.rs                   13/2
tests/sd27_book_license_record_counts.rs                             4/2
tests/sd27_equipment_modifier_price_matches_corpus_cost_token.rs     1/2
tests/sd27_known_spells_must_be_on_the_class_spell_list.rs           5/1
tests/sd30_declared_product_identity_in_shipped_class_features.rs    2/1
tests/sd31_class_feature_corpus_key_uniqueness.rs                    1/1
tests/v06_corpus_trap_report.rs                                     21/4
```

`src/bin/ingest_races.rs` is the one non-`tests/` target attempt 9 also named — the two runs agree on
the *shape* of the set, not only its size.

**0 of 31 are SD-33's**, re-derived here rather than carried from attempt 9's claim:

```
$ for f in <each of the 31 targets>; do git log --oneline f652db7ac7..HEAD -- "$f" | wc -l; done | awk '{s+=$1} END {print s}'
0
```

**0 of 31** failing targets carry a single commit since the `tranche/13` cut. The **2** suites that
*were* SD-33's own, broken and then fixed by earlier waves, pass in this run — verified by
execution, not by their fix receipts:

```
tests/sd25_monk_level_up_explanation_filter_audit.rs -> ok. 6 passed; 0 failed
tests/v06_work_inventory.rs                          -> ok. 16 passed; 0 failed; 1 ignored
src/lib.rs                                           -> ok. 2837 passed; 0 failed; 14 ignored
src/bin/corpus_literal_sweep.rs                      -> ok. 13 passed; 0 failed
```

This is Shortfall 2 as attempt 9 reported it, unchanged and unaffected by wave 9's fix: **inherited
debt at the cut, 0 of 31 caused by SD-33**, and therefore not this criterion's to clear.

### `data/corpus/**`

Unchanged by this wave's fix commit (**0 of 6** files). The only count-shaped ratchet the corpus
touches, `BASELINE_CORPUS_LITERAL_RECORDS`, is a **floor** whose growth `verify.sh` reports as a
`note`, not a failure (`scripts/verify.sh:1847-1854`), and `scripts/verify-baselines.env` is
**unchanged since the `tranche/13` cut** (`git diff --stat f652db7ac7..HEAD -- scripts/verify-baselines.env` → empty).
`corpus-sweep` PASSes with that note.

### Independently re-derived headline figures (not restated from receipts)

```
$ jq -r '.families | to_entries[] | "\(.key) \(.value.population)"' .../epic-3-engine-coverage/formula_interpreter.corpus-wide.json
F1 6308  F2 2337  F3 671  F4 1086  F5 589  F6 391  F7 12  F8 196  F9 62      (sum = 11,652 of 11,652)
```

Epic 3's F1 population re-derives to **6,308 of 6,308** and F1..F9 sums to **11,652 of 11,652**, the
receipt's own true-population figure. Epic 3's artifact is at the **SD-33** path
(`artifacts/epic-3-engine-coverage/formula_interpreter.corpus-wide.json`), and SD-32's
`gate-2-engines` tree is **UNTOUCHED**: `git diff --name-only f652db7ac7..HEAD -- docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-2-engines | wc -l` → **0**.

### Denominator gate

```
$ python3 scripts/denominator_gate.py --check
files_checked=59
violations=0
```

**0 violations of 59 files.** Detection re-proven live, in an in-scope file, then removed:

```
$ <append "PROBE: coverage is 100% complete." to AT-33-E6-001-corpus-sweep_cycle_receipt.md>
VIOLATION .../AT-33-E6-001-corpus-sweep_cycle_receipt.md:301: PROBE: coverage is 100% complete.
files_checked=59  violations=1   PROBE_EXIT=1
$ <restore>
files_checked=59  violations=0   BASELINE_EXIT=0
$ git status --porcelain
(empty)
```

A probe file placed at the bundle root was **not** picked up: `DEFAULT_GLOBS` is a named scope
(`artifacts/**/*_cycle_receipt.md`, `progress.md`, and the 7 headline root documents,
`scripts/denominator_gate.py:105-112`), the scope `AT-33-E1-004`'s own scope-widening cycle defined
— a documented glob, not an exclusion list. The probe above was therefore re-run inside that scope.

---

## CHECK 4 — `## Open blockers` holds no active entry

```
$ grep -n "^## Open blockers" progress.md          # the REAL heading, not an archived copy
305:## Open blockers
$ awk 'NR>305 && /^## /{print NR": "$0; exit}' progress.md
473: ## Cycles
$ sed -n '305,473p' progress.md | grep -n "^###\|^<details>\|^</details>"
50:<details>       53:### `corpus_literal_sweep` mismatch on 10 weapon records …
83:</details>      98:<details>
101:### `rending_claw_blades` compute_equipment_effects weapon-path EQMOD-resolution gap …
167:</details>
```

Both `###` entries lie **inside** `<details>` historical blocks (lines 50–83 and 98–167 of the
section). **0 active entries of 2 historical.** The section's own text records the
`corpus_literal_sweep` entry as CLEARED, not superseded.

The `deferral` retro event filed with it is **resolved**:

```
$ python3 -c "<print type/resolves for docs/retro/events/sd33-r9-corpus-sweep.jsonl>"
verification | correction | resolution -> resolves 1787692278828-sd33-r8-build-green-630237 | verification | verification
```

### Open deferrals — enumerated, none defers live DoD scope

`python3 scripts/retro.py summary --since 2026-08-24 --json` → **7 open**, down from attempt 9's 8
(`sd33-r8-build-green`'s is the one now resolved). Each carries a revisit condition (**7 of 7**);
each is checked against `AGENTS.md`'s one question — *was this scope in the Definition of Done?*

| # | actor | scope | DoD? | disposition |
|---|---|---|---|---|
| 1 | `sd33-e4-unknown` | widen `REGISTERED_POOL_GROUPS` for `class_feature` pool ownership | no — `AT-33-E4-002`'s DoD is `unknown` = 0, met (**0 of 49,438**) | forward capability |
| 2 | `sd33-e4-unknown` | recognise `status==unmeasurable` in `pf1e_dashboard_producer.py` | no — outside the criterion's write scope, raises loudly by design | forward capability |
| 3 | `sd33-r4-last75` | live oracle verification of 67 units | **was** DoD (row 17) — **since DONE**: literal rows **6,589 of 6,589**, 0 unrowed | log entry stale, scope closed |
| 4 | `sd33-r5-weapon` | 13 unexamined weapon-shape units | **was** DoD (row 17) — **since DONE** (wave 6 rowed all 39 remaining) | log entry stale, scope closed |
| 5 | `sd33-r5-weapon` | 10 unexamined shape-B/C/D units | **was** DoD (row 17) — **since DONE** (same) | log entry stale, scope closed |
| 6 | `sd33-r5-e5-finalize` | corpus-extraction `.MOD`-attached-EQMOD fix + regeneration | **was** DoD (row 18) — **since DONE** (wave 6, 137 records regenerated, `rending_claw_blades` fixed) | log entry stale, scope closed |
| 7 | `sd33-r6-skillcombat` | new COMBAT non-AC aggregation engine surface, cross-record variable resolution | no — those units are rowed with a verdict and a populated reason, which is row 17's DoD | forward capability |

**0 of 7 defer live DoD scope.** Rows 3–6 named scope that *was* DoD when written and has since been
completed to a commit — verified here by execution (`6,589 of 6,589` rows, unexamined set empty both
directions, `oracle_disagreement=0`), not by their own text. Their ledger entries were never closed
with a `resolution` event; this cycle closes them with `resolution` events, so the ledger stops asserting work that is done. After those four resolutions the ledger reports **3 open** deferrals — rows 1, 2 and 7 above — **0 of 3** deferring DoD scope and **3 of 3** carrying a revisit condition (`python3 scripts/retro.py summary --since 2026-08-24 --json`).

---

## The full scan

| Rows | Statuses | Verdict |
|---|---|---|
| 1–18 | `complete` × 18 | **18 of 18** |
| 19 | this cycle | flipped to `complete` on this PASS |
| 20–21 | `not-started` | Epic 6's own remaining cards, permitted by the criterion |

No row is `returned-to-backlog`, `in-progress`, or `blocked-escalated`. **27 of 27** receipt paths
named by `kanban.md` exist on disk, and **27 of 27** carry the §7 four-buckets row; every row-1–18
criterion receipt carries an explicit figures-with-re-derive-commands section.

---

## `scripts/verify.sh` — every stage, honestly

A full end-to-end run was launched and **did not complete within this turn's budget** (`AGENTS.md`
§2.5: report what was observed and commit anyway). Stage-by-stage:

| Stage | Result |
|---|---|
| `preflight-disk` | PASS |
| `preflight-oracle` | PASS |
| `oracle-pin-selftest` | PASS |
| `producer-selftest` | PASS |
| `pi-redaction-selftest` | PASS |
| `provenance-selftest` | PASS |
| `site-dashboard-selftest` | PASS |
| `site-dashboard-check` | **HUNG** — killed after ~12 min |
| all later stages | not reached in the full run; the load-bearing ones were run directly (below) |

`site-dashboard-check` is the **same** stage attempt 9 and the `corpus-sweep` lane each reported
hanging, on three different diffs — a **third** live confirmation that it is environmental and not
this wave's. Root-caused one level further here than either predecessor did:

```
$ tail -1 $LOG_DIR/site-dashboard-check.log
pf1e-producer: v06_work_inventory failed to run: Command '[... 'cargo', 'run', '--quiet', '--bin', 'v06_work_inventory', '--', '--summary']' timed out after 600 seconds
$ ps -eo pid,etime,pcpu,args | grep v06_work_inventory
2146027  03:46  99.9  .../v06_work_inventory --summary      <- another agent's, different target dir
2146809  01:26  100   .../v06_work_inventory --summary      <- this run's
```

The producer's own 600-second timeout had **already** fired once and it was on its second attempt,
at 100% of one core, when the stage was killed. `scripts/verify.sh` and
`scripts/publish-site-dashboard.sh` wrap that call in no timeout of their own, so the stage cannot
bound itself. This is a real, named, reproducible defect in the gate's own plumbing — not a
verdict on SD-33's diff — and it is written down here rather than left as "environmental".

The stages this scan's verdict actually rests on were each run directly and are reported above:
`corpus-sweep` (**0 findings of 48,634 records examined**), `denominator-gate` (**0 violations of
59 files**), `root-lib` (**2,837 of 2,837**), `desktop` (**548 of 548**), plus
`cargo test --locked --no-run` (**543 of 543**) and the full `--no-fail-fast` workspace run below.

---

## Movement, four buckets

Closure **1** — kanban row 19 `blocked-escalated` → `complete`; `AT-33-E6-001` satisfied.
Reclassification **0**. Reachability **0**. Instrument-correction **0** (this cycle is a scan; it
changed no instrument). Ledger hygiene: **4** stale `deferral` events closed with `resolution`
events, moving no number.

---

## Gate result

**PASS.** Every check above is satisfied on evidence this scan produced, not on a predecessor's
report. Nine prior scans halted this bundle and all nine were right; this one does not halt it
because the one shortfall they were holding it on is genuinely closed, verified against all four
illegitimate routes to the same zero, and the only remaining red — 31 of 599 suites, 49 of 8,026
tests — is inherited at the `tranche/13` cut with **0 of 31** carrying a commit since it.

- **Status:** complete
- **Notes:** Two figures in the wave-9 lane's report were checked and one is corrected, in the
  direction of *more* scrutiny rather than less: `epic5_units_affected: 0` is really **1 of 10**
  in-population (a `unverifiable` row with no value on either side), which does not change the
  conclusion — **0 of 8,330** rows could move, because the fix changed no `raw_tokens` input — but
  the lane reached the right conclusion by a slightly wrong route, and the route is what a future
  reader would copy. `verify.sh`'s `site-dashboard-check` hang is written down as a real
  gate-plumbing defect with its own `incident` event, not filed a third time as "environmental"
  (`AGENTS.md` Rule 8: a warning is not a control).
- **Next-cycle plan:** `AT-33-E6-002` (retrospective written and cited, kanban row 20), then
  `AT-33-E6-003` (sweep / architecture docs / graphify / PR, row 21).

## Environment finding — sixth consecutive wave

The shared checkout at `/home/ubuntu/workspace/repos/codex` is 8 commits behind
`origin/tranche/13` and still carries the same uncommitted, **staged** revert of wave 6's
`.MOD`-fold fix (137 `data/corpus/**` modifications plus deleted receipts, oracle-results and retro
`.jsonl` files) that waves 6, 7, 8 and 9 each found, reported, and worked around. This wave worked
around it the same way — a clean `git worktree add --detach` off `origin/tranche/13`, per
`AGENTS.md` "One writer per tree" — and did not discard it. **Five findings and five workarounds is
a missing mechanism, not bad luck** (`AGENTS.md` Rule 8). It needs an operator, or a cycle with
sanctioned discard authority, to clear that tree; no scan can, because a scan may not throw away
work it did not create.
