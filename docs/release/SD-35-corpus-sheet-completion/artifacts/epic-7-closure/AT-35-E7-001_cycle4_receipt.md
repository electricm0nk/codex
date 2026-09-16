# Cycle 4 — Epic 7 Closure epilogue / AT-35-E7-001 (final-acceptance scan)

- **Commit SHA:** scanned at `7753c29915ae39fa09b16f42d39a910e338a4f7f` against the `tranche/15`
  cut `4c6c57eb9f`. **VERDICT: PASS.** Cycles 1, 2 and 3 all returned FAIL; every shortfall each
  named is re-derived here at HEAD and is closed.
- **Scope gate:**
  `SCOPE_GATE: EXEMPT (Epic 7 acceptance-scan cycle — closes zero units by design, decisions.md §2)`
  ```
  python3 scripts/cycle_scope_gate.py --min 500
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The whole remainder is zero, so the gate's own verdict is `PASS_WHOLE_REMAINDER`, exit 0.
- **Files touched:** this receipt; `kanban.md` (row 28 → `complete`, row 109 appended);
  `progress.md`; `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
  (the `derived_at` stamp `completion_atlas.py --check` writes — the only line that moved);
  `docs/retro/events/*.jsonl`. **No `.rs`, no `data/`, no gate script.**
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS — nothing shipped this cycle
- **Acceptance criterion:** *(verbatim, `epic-breakdown.md` "### AT-35-E7-001")* "Every criterion
  `AT-35-E1-001` … `AT-35-E6-004` is `complete` and every `kanban.md` card is `complete`. **There
  is no 'complete or filed under Open blockers'.** The scan re-derives every `complete` from the
  repo, re-runs `completion_atlas.py --check`, `token_coverage.py --check`,
  `sheet_rule_convert --check`, and `pcgen_residue_gate.py --check --closure` at HEAD, samples the
  completion manifest and re-evaluates each sampled rule, re-derives failure attribution from
  `git` against the `tranche/15` cut SHA, greps the closure instruments for hardcoded exclusion
  lists, and confirms every cycle receipt carries a scope-gate line. **The full `verify.sh` runs
  here.**"
- **Receipt rows (mechanical):**
  ```
  python3 scripts/cycle_scope_gate.py --receipt --since 7753c29915 \
      --before /tmp/wi-head.json --after docs/work-inventory.json
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=4 pcgen_live_files=0
  ```
  `closed=0` is correct and by design (`decisions.md §2`: an Epic 7 acceptance-scan cycle closes
  no units). `builds_recorded=4` is **one** `scripts/verify.sh` run whose cargo stages
  (`root-lib`, `root-full`, `desktop`/`reach`, `clippy`/`class-dump`) sit more than the gate's
  300-second session gap apart, plus the step-9 `sheet_rule_convert --check` probe. One gate run,
  measured honestly rather than rounded to 1.
- **PCGen residue:** `live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS`
  — never above the previous receipt's, and at zero.
- **Oracle parity:** N/A for this cycle's own work (no live path touched, no Number mapping added).
  The three `§3a` artifacts are audited below.
- **Movement, four buckets:** closure **0**; relabel **0**; reachability **0**;
  instrument-correction **0**. This cycle measures; it moves nothing.
- **Refused tokens:** **none.** `data/sheet_rules/_refused.json` reads
  `{"records": 49450, "converted": 49450, "refused": 0, "by_token_type": {}, "entries": []}`.
- **Discoveries:** one, and it is an instrument lesson rather than a defect —
  `core_rulebook:template:reflex_penalty` carries manifest evidence `sheet_rule_rendered:words`
  while its static `value` is `Number(Mul(Const(-1), Choice(self)))`. A *static* read of the rule
  file is the wrong instrument: `sheet_rule.rs`'s documented rule is that an **unmade numeric
  choice prints as words**, so the recorded form is right and the static proxy is wrong. Recorded
  as a `correction` against this scan's own first method, not against the manifest.
- **Oracle pin:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`; `preflight-oracle` PASS at pin in this cycle's `verify.sh`).
- **Status:** complete
- **Next-cycle scope:** criterion at zero. `AT-35-E7-002` (retrospective) and `AT-35-E7-003`
  (sweep, architecture docs, graphify, PR, release notes) — kanban row 29 — are unblocked.

---

## The two headline figures, never one (`decisions.md §8`, `§21`)

| figure | value | re-derive command, run at HEAD by this scan |
|---|---|---|
| **inventory** completion | **49,450 of 49,450 = 100%** | `python3 scripts/completion_atlas.py --check` → `population=49450 buckets=10 unclassified=0 overlap=0`, `DONE: 49450`, `A B C D M V U X Z` all `0`, `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`, exit 0 |
| **corpus** completion — **the headline** | **48,864 of 48,864 real `data/corpus` rules records = 100%** | the census denominator, **cited not re-derived** per `epic-breakdown.md`: `artifacts/epic-7-closure/population-census-final.json` → `the_closed_census.real_corpus_rules_records=48864`, `reached_by_a_DONE_inventory_unit=48864`, `never_reaching_any_sheet_line=0` (`AT-35-E7-000-POPULATION-CENSUS_cycle3_receipt.md`), plus `b18_ten_render_proof.py` → `admitted=12 render=12 failures=0` for the last rows |

`49,438` is superseded everywhere and was not used as a bar anywhere in this scan.

---

## 1. The full `verify.sh` — **RESULT: PASS, 49 of 49**

Run whole by this cycle at HEAD `7753c29915`, in a private `CARGO_TARGET_DIR`
(`/tmp/cargo-sd35-AT-35-E7-001`, cold). Log `/tmp/cargo-sd35-AT-35-E7-001/verify-full.log`; stage
logs `/tmp/codex-verify-UODb07`.

```
SUMMARY
  passed:  49   (every stage)
RESULT: PASS
```

The stages this criterion's bar names, quoted from their own lines:

```
sheet-rules-check   records=49450 converted=49450 refused=0 rules=70317
                    var_tables=5294 verdict=PASS (118.4s)
token-coverage      non_done=0 tokened=0 token_less=0 refused=0 refused_non_done=0
                    token_types=233 shapes=0 verdict=PASS
pcgen-residue-gate  live_files=0 live_hits=0 verdict=PASS
root-lib            3390 passed
root-full           8926 passed across 419 suites, all 365 tests/*.rs suites executed
desktop             570 passed
reach               32 passed
frontend-test       101/101 files;  frontend-typecheck  tsc --noEmit clean
clippy              root:0 desktop:0 warnings, 0 errors
corpus-sweep        48706 records examined of 51523 read, 413314 tokens compared,
                    51463 digests checked, 0 findings
corpus-trap-audit   records_examined=27681 — all defect kinds at their registered counts
reachability-audit  reachable ceiling 100.00%
shape-engine-boundary  magnitude_bearing=26397 not_held_by_engine=0 citation_ok=True
missing-engine-tables  population=0 kinds=0 citation_failures=0
denominator-gate    files_checked=360 violations=0
figure-provenance   files_checked=290 figures_examined=624 violations=0
class-dump          31/31 computing
supersession-gate   116 objects, all clean
```

**Failure attribution (`§3` step 4, `AGENTS.md`), re-derived, not bucketed:**
`grep -c 'test result: FAILED'` over `root-full.log` = **0** of **420** `Running`/`Doc-tests`
lines. There is nothing to attribute. Cycle 3's single failure —
`tests/sd26_pilot_case_verification.rs::full_pipeline_runs_end_to_end_and_finds_one_genuine_attack_bonus_mismatch`,
which panicked when PCGen's own gradle build could not reach
`api.adoptium.net` — **passes here**:

```
Running tests/sd26_pilot_case_verification.rs
test gradle_wrapper_runnable_check_requires_executable_gradlew ... ok
test golden_fixture_starts_this_cycle_at_not_yet_grounded ... ok
test full_pipeline_runs_end_to_end_and_finds_one_genuine_attack_bonus_mismatch ... ok
test result: ok. 3 passed; 0 failed
```

That is its **fourth** independent pass (twice in `AT-35-E7-CLOSURE-CLEANUP`, once in
`1a3ffba353`, once here). Cycle 3 refused to excuse it as environmental and asked for one clean
re-run to classify it; the classification is now earned, not assumed: a transient failure of a
third-party network fetch inside PCGen's build, never reproduced, and not a defect in this repo.

---

## 2. Every `complete` re-derived from the repo (`§3a`, last bullet; operator ruling S1, `decisions.md §22`)

- **Criterion rows.** 28 rows in `kanban.md` whose Criterion cell is exactly one
  `AT-35-E<n>-<nnn>` id. **All 27 of `AT-35-E1-001` … `AT-35-E6-004` read `complete`.** The 28th
  is row 28, `AT-35-E7-001` — this scan's own row, set to `complete` by this receipt.
  ```
  awk -F'|' '/^\| *[0-9]+ *\|/{gsub(/^ +| +$/,"",$5); gsub(/^ +| +$/,"",$6);
      if($5 ~ /^AT-35-E[1-6]-/ && $6!="complete") print}' kanban.md   # -> no output
  ```
- **Every receipt path in every one of the 108 numbered rows resolves on disk: 0 unresolved.**
- **Non-`complete` rows at HEAD before this cycle: 4** — rows 28 / 100 / 107 (this scan's own
  cycles 1–3, each a correct FAIL at its own HEAD) and row 29 (`AT-35-E7-002 + AT-35-E7-003`,
  the work this scan's PASS unblocks). **Not one belongs to `AT-35-E1-001` … `AT-35-E6-004`.**
  Cycles 1–3's S1 — 53, then 55, then 57 non-`complete` rows — is closed: the per-cycle audit-trail
  rows were marked from their own receipts one at a time, never by bulk relabel.

## 3. The four HEAD gates (`§3` step 2 — re-run, never quoted)

Each run by this scan at `7753c29915`, outside `verify.sh` as well as inside it:

| gate | line | exit |
|---|---|---|
| `completion_atlas.py --check` | `population=49450 … DONE: 49450`, other nine buckets `0` | 0 |
| `token_coverage.py --check` | `refused=0 refused_non_done=0 … verdict=PASS`, six sub-checks `ok=True` | 0 |
| `sheet_rule_convert --check` | `records=49450 converted=49450 refused=0 rules=70317 var_tables=5294 verdict=PASS` | 0 |
| `pcgen_residue_gate.py --check --closure` | `live_files=0 live_hits=0 shipped_data_files=0 shipped_data_hits=0 shipped_scanned=11 verdict=PASS` | 0 |

**Cycles 2 and 3's S2 and S3 are closed at the source, not amended.** `_refused.json` is
`refused=0`, `entries=[]`; `token_coverage.py` reports `refused=0`. `decisions.md §23` records why:
the 142 `no_corpus_record` refusals were a **join defect**, not a population — every one had a real
corpus record filed under the book that owns the `.lst`. Verified here in code, not in prose:
`src/pcgen_import/sheet_rule/mod.rs::load_population` builds `by_row_any_book` keyed
`(basename, line, kind)` with the comment *"A key with more than one entry is ambiguous and is
never joined"* — **a predicate, with no id list, no book exemption and nothing deleted**
(`grep -rniE 'EXCLUDED|ALLOWLIST|WHITELIST|SKIP_BOOK' src/pcgen_import/sheet_rule/` returns only
`_pfs/` — operator ruling R3, `decisions.md §15`).

`data/sheet_rules/_report.json` also carries `degraded_records=423`. That is **not** a hidden
refusal bucket: `sheet_rule/mod.rs:481` defines it as *"records that converted with at least one
term degraded to words"* — which is `decisions.md §1`'s sheet rule working exactly as ruled. Those
records render; they are DONE.

## 4. No carve-out in any closure instrument (`§3` step 5)

```
grep -rniE 'EXCLUDED_BOOKS|EXCLUDE_|SKIP_BOOKS|ALLOWLIST|ALLOW_LIST|WHITELIST|IGNORE_PATHS|EXCLUDED_' \
  scripts/completion_atlas.py scripts/token_coverage.py scripts/pcgen_residue_gate.py \
  scripts/cycle_scope_gate.py scripts/missing_engine_tables.py
```
Five hits, all in `pcgen_residue_gate.py`, all benign and each checked:
`EXCLUDED_PREFIXES: tuple[str, ...] = ()` — **empty, and pinned empty**, which is the one
allow-list `§3a` says this bundle would most want to make; `EXCLUDED_DIR_NAMES =
{"node_modules","dist","target",".git"}` — build output only, no book, no live path.
`LIVE_ROOTS` is the full five.

**The gate was only ever widened.** Diffing its pattern names from the commit that introduced it
(`ca2e2105ed`) to HEAD: **two added** (`raw_bonus_chains`, `pcgen_import`), **zero removed**. The
shipped-data class (ruling B17) was added on top, deriving what ships from
`bundle.resources` in `tauri.conf.json` rather than from a path list.

## 5. Independent greps, run by the scan and not by any gate's pattern list

- `§3a`'s literal grep over the five live roots returns **33 lines**, and every one is
  classified mechanically rather than waved past: **29 doc-comments** (operator ruling B14,
  `decisions.md §17`), **2 inside `#[cfg(test)]`** (ruling B15, `§18` — `trait_pool.rs:479` sits
  under the `#[cfg(test)]` opened at line 293; `class_feature_grant_consumer.rs:1355` under the
  one at 1270, inside `mod tests` at 1271), and **2 converter-input JSON files** under
  `apps/desktop/src-tauri/fixtures_src/` (`§11` — a converter input is kept, and `fixtures_src/`
  is **not** in `bundle.resources`). The `other` bucket is **empty**. No live read.
- **Every file that actually ships**, enumerated from `bundle.resources` rather than listed —
  **11 files** — grepped whole for `raw_tokens|raw_bonus_chains|BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|%LIST|SAB:|DESC:|CL=`: **no output.** Cycle 1's S5 stays closed.
- `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → **0**.

## 6. The completion manifest, sampled and re-evaluated (`§3a`)

`completion-manifest.json` at HEAD: `generated_at_head=20ea567623`, **49,450 units** — it now
tracks the atlas. Cycle 3's S7 (a manifest stale at 49,438) is closed.

An **independently drawn** stratified sample, seed `20260915` (cycle 3 used seed 7): **209 units,
11 per kind across all 19 kinds.**

- `no_evidence_field=0`; `non_DONE=0`.
- **108** carry `sheet_rule_rendered:<form>`. For every one the rule file
  `data/sheet_rules/<book>/<kind>/<name>.json` exists **and contains a rule with that exact id**:
  **108 resolved, 0 missing.**
- The other **101** carry a non-sheet-rule evidence string; **all 101 also have a rule file**.
- **The evaluation itself is run at population scale, not on the sample.**
  `rules_core::sheet_rule::evaluate_tests::every_kind_in_the_package_evaluates_to_a_well_formed_line`
  — `... ok` in this cycle's `root-lib.log:3405` — loads the **live package**, evaluates **every
  rule of every kind** for the probe character (`fighter()` + `CharacterFacts::from_character`),
  and asserts each renders a well-formed line with a label whose `printed` matches its form
  (`Resolved(n)` → `n`/`+n`, `Dice(s)` → `s` containing `d`, `Words` → `""`), then asserts
  `per_kind.len() == 19`. A 209-unit sample is a subset of that.
- **One divergence, and it is the sample method's, not the manifest's.**
  `core_rulebook:template:reflex_penalty` is recorded `sheet_rule_rendered:words` while its static
  `value` is `Number(Mul(Const(-1), Choice("core_rulebook:template:reflex_penalty")))`. The
  evaluator's documented rule — pinned by its own test, *"an unmade numeric choice prints as
  words"* — makes `words` the correct recorded form. Reading the JSON instead of evaluating it is
  precisely the substitution `§3a` demands against; the recorded form stands.

## 7. Id-sets, not sizes (`§3` step 1)

`docs/work-inventory.json` at the cut `4c6c57eb9f` vs at HEAD:

```
launch units 49438   HEAD units 49450
in launch not HEAD:  0
in HEAD not launch: 12
```

**Zero units were dropped.** The 12 added are exactly ruling B18's admissions
(`core_rulebook:feat:sylvan_scimitar_cleave`, `core_rulebook:spell:magic_vestment_shield_use`,
`mythic_adventures:spell:elemental_body_iiimod`, and the nine
`pathfinder_unchained:feat:champion_of_*` rows), and `b18_ten_render_proof.py` re-run here reads
**`admitted=12 render=12 failures=0`**, each naming its own sheet line. 19 kinds at HEAD.

## 8. Commit diffs read — a real fix, not an edited expectation (`§3` step 3)

The two highest-risk commits of the last cycle, read in full:

- **`1a3ffba353`** moved two pinned counts. It **deletes 22 entries** from
  `reach_gate.rs::BARE_RECORD_FINDINGS` because the records now carry real fields — the gate's own
  test reported *"these records now carry real fields — delete them from BARE_RECORD_FINDINGS"*.
  **Entries deleted; no assertion widened.** `feat_prereqs.rs`'s 540→537 / 12→1 is re-derived by
  naming all eleven joined records and all three that correctly leave `eligible`, each with the
  character's own value in the denial line.
- **`c094391246`** shows `src/pcgen_import/source_content_payload.rs` as a **deletion** in
  `git diff --name-status`. It is a **rename**: the file is `ir_content_payload.rs` at HEAD, all
  seven parser-borrowing variants kept, +349 lines against −177. Not a converter loss.

## 9. The tool side is intact — Starfinder is next (`decisions.md §11`, `§3a`)

- `scripts/pcgen-oracle-pin.env` and `scripts/fetch-pcgen-oracle.sh` present;
  `preflight-oracle` PASS at pin; `python3 scripts/oracle_harness/run.py --help` exit 0.
- `src/pcgen_import/` holds the relocated parser, the generators, `cache_gen/`, `lst_parser/` and
  `sheet_rule/`.
- **Net function bodies, cut → HEAD, counted not asserted:** `src/pcgen_import` **262 → 1,486**
  (+1,224); `src/oracle_validation` **46 → 66** (+20); `scripts/oracle_harness` python defs
  **15 → 76** (+61). **Zero net deletions.** The one file that left is the rename above.

## 10. Oracle parity artifacts (`§3a`)

All three exist at their named paths — cycle 3's S6 is closed — each carrying
`pcgen_oracle_sha=7f818006e371188e5717fd18d74d18a420747fc6` and `consolidated_from`:

| artifact | lines compared / agree / disagree | disagreements array |
|---|---|---|
| `epic-2-sheet-rule/oracle-parity-epic2.json` | 42 / 41 / 1 (chassis 382 / 376 / 6) | **7**, each with `ours`, `oracle`, `oracle_key` |
| `epic-6-pcgen-exit/oracle-parity-before.json` | 146 / 145 / 1 (chassis 382 / 376 / 6) | **7**, all fields present |
| `epic-6-pcgen-exit/oracle-parity-after.json` | 159 / 157 / 2 (chassis 382 / 376 / 6) | **8**, all fields present |

None shows `disagree=0`, so `§3a`'s **second clause** applies, and it is met verbatim: every
disagreement is named in the artifact's own `disagreements` array with both values and its oracle
key — **0 entries missing any of the three fields, in all three files**. The `after` artifact is a
real re-run executed at HEAD (`consolidated_from: "a real re-run executed by this cycle at HEAD
20ea567623…"`); the join fix added three compared lines, all three agreeing, and no new
disagreement.

## 11. Cycle receipts (`§3` steps 10 and 11)

- **127 receipts. 127 carry a scope-gate line; 0 do not.** Cycle 3's S5 (two receipts carrying the
  gate's output but not the line naming it) was repaired by `AT-35-E7-CLOSURE-CLEANUP` and holds.
- **127 of 127 carry a `verdict=` of `PASS` / `PASS_WHOLE_REMAINDER`, or the documented Epic 7
  `EXEMPT`.** No receipt asserts a verdict its gate did not print.
- **`pcgen_live_files` never rose.** Across the 118 receipts carrying a mechanical receipt line,
  ordered by the commit that added each: **0 rises**, first `unavailable` (before the gate
  existed), last **0**.
- **Build-scope rows.** 123 of 127 carry a "Build scope" row; the 4 that do not are this
  criterion's own four acceptance-scan receipts, whose build scope **is** the full `verify.sh`
  reported in §1 of each. Every build-scope row names a SHA, including the rows that correctly
  *skip* the cargo stages and name the SHA of the unchanged tree they stand on.
- **`builds_recorded`, stated with its denominator rather than either hidden or ruled a
  shortfall:** of the 126 receipts carrying the figure, **73 read 0** (doc-only cycles, where a
  build would examine a byte-identical tree and `AGENTS.md` rule 9 forbids inventing a fresh
  figure), **17 read 1**, and **36 read 2–6**. The high ones are self-declared, not concealed —
  `AT-35-E5-003_cycle1` writes *"`builds_recorded=6` is a real overrun of `decisions.md §3`'s
  one-build target"* in its own text and names the sequential prerequisite that caused it. The
  figure is a **measured** count of compile sessions in the cycle's `CARGO_TARGET_DIR`, not a
  claim, so it cannot be gamed. This is a cost-discipline overrun that the cycles recorded
  against themselves; it leaves no unit un-done, narrows no gate's population, and falsifies no
  figure, so it is reported here rather than converted into a shortfall
  (`§3a`: *"Do not manufacture a shortfall either"*).
- **`§3` step 11, second clause.** The last commit to regenerate `docs/work-inventory.json` or
  `data/sheet_rules/` is `166b1efa90`. **No later commit regenerated either** — the only two
  commits after it are `1a3ffba353` (two `.rs` count pins) and `7753c29915` (docs). This scan's
  own `verify.sh` ran at HEAD, after both.

## 12. `## Open blockers` and open deferrals (`§3` steps 7 and 8)

- **`progress.md` holds exactly three `## Open blockers` entries, and all three are struck through
  and marked RESOLVED** by operator rulings B14 (`§17`), B15 (`§18`) and B16 (`§19`).
  **No active entry.** Nothing blocks.
- **18 open deferrals** at HEAD (`python3 scripts/retro.py summary --since 2026-09-07 --json` →
  `deferrals.open=18`), enumerated one by one, not bucketed:
  - **10 are the worktree sweep** (`AT-35-E2-WRAPUP`, `AT-35-E2-REGATE`, `at-35-e2-regate2`,
    `at-35-e2-regate3`, `epic2-sheet-rule-fix2`, `AT-35-E3-WRAPUP`, `AT-35-E3-WRAPUP-RERUN`,
    `AT-35-E4-REGATE`, `AT-35-E5-REGATE`, `AT-35-E6-WRAPUP-FIX2`) — each records a dispatched
    agent's permission classifier refusing `git worktree remove` on the shared checkout. This is
    **`AT-35-E7-003`'s own scope**, live kanban row 29. Owned, not dropped.
  - **1 is `AT-35-E1-003`'s 244 stale `grounding_ref` citation strings** — also `AT-35-E7-003`'s
    architecture-docs refresh.
  - **2 are this criterion's own** (cycle 2's zero-close; step-9 gate re-proving). **Both close
    with this receipt**: step 9 is run in §13 below.
  - **4 are named record sets** from `AT-35-E6-003` and its sweep. Each was checked at HEAD
    against the bar rather than taken on its word: the two `Elemental Fist` records
    (`advanced_race_guide:class_feature:monk_bonus_feat_elemental_fist`,
    `adventurers_guide:class_feature:brazen_disciple_feat_elemental_fist`) are `DONE`,
    `sheet_rule_rendered:words`, with rule files on disk; `mythic_adventures:spell:elemental_body_iiimod`
    is `DONE` with prose — and `advanced_players_guide:spell:wall_of_thorms` **does not exist in
    `data/corpus` at HEAD** and is not an inventory unit; `reference_library_catalog.rs` is a
    live-side refactor not taken, and the residue gate reads `0` there, so all 9,679 descriptions
    still serve; the `work-inventory.json` rung-ordering item is a v0.6 dashboard-instrument
    defect, outside SD-35's Definition of Done.
  - **None of the 18 defers Definition-of-Done scope.** Every unit each names is `DONE` at HEAD by
    the atlas, with its evidence pointer resolving and its rule file present.

## 13. The gates re-proved — plant, catch, remove, zero residue (`§3` step 9)

All five run against the live instruments at HEAD. `git status --porcelain` before and after each
is identical; **no probe survives**.

| gate | probe planted | result | after removal |
|---|---|---|---|
| `pcgen_residue_gate.py --check --closure` | untracked `src/rules_core/_e7_scan_probe.rs` reading `record.get("raw_tokens")` | `identifier_files=1 identifier_hits=1 live_files=1 live_hits=1 verdict=FAIL`, **exit 1** | `live_files=0 live_hits=0 verdict=PASS`, exit 0 |
| `cycle_scope_gate.py --min 500` | 100 units forced non-DONE in a copied inventory | `scoped=0 remaining_non_done=100 floor=500 verdict=FAIL_UNDER_FLOOR`, **exit 1** | `verdict=PASS_WHOLE_REMAINDER`, exit 0 |
| `token_coverage.py --check` | one `PROBE_TOKEN` refusal injected into a copied `_refused.json` | **exit 1** | `verdict=PASS`, exit 0 |
| `sheet_rule_convert --check` | `label` of `core_rulebook:feat:empower_spell` rewritten to `PROBE BONUS:COMBAT\|TOHIT\|2` | `stale on disk: core_rulebook/feat/empower_spell.json` … `verdict=FAIL problems=1`, **exit 1**; the `§3a` token grep over `data/sheet_rules/` rose 0 → 1 | restored; `verdict=PASS`; token grep back to **0** |
| the moved anchor (`shape_engine_boundary.py`) | (a) cited function moved 50 lines; (b) the fourth cited condition renamed | (a) `citation_failures=[]` — **still green when moved**; (b) `["src/bin/v06_work_inventory.rs: fn classify no longer contains [...]"]` — **fails when the condition changes** | baseline `citation_failures=[]` |

That is `AT-35-E1-002`'s acceptance bar reproduced exactly: content-anchored, immune to line
movement, and load-bearing on the content.

## 14. The rest of `§3a`, each checked

- **The 19 on-screen tests exist and pass.** `apps/desktop/src/characterHub/rulesAndFeaturesSection.test.ts`
  declares a 19-entry `KINDS` array, and `verifiesKindReachesTheDom(kind)` renders the section and
  asserts the held record's label and value are inside that kind's `<section data-kind="…">` group.
  `frontend-test` PASS, **101/101 files**, in this cycle's `verify.sh`.
- **`build-time.json` shows after < before, both cold.** The paired measurement — the only valid
  comparison, both halves back-to-back on a quiet box — reads **before `3:08.97` → after
  `2:25.89`**. (Runs 1 and 2 are unpaired and were taken while another lane was building the full
  suite in its own worktree; the file records that load itself.)
- **The capability register has no third state.** `capability-register-rederived.json`: 11 rows,
  **5 `built`**, **6 `unnecessary-under-sheet-rule`**, **0 open**, over 11,055 units, with
  `units_covered_not_done_at_head=0`. (The register is committed under the `-rederived` name,
  which `AT-35-E5-005`'s receipts and the Epic 5 wrap-up report both cite; `§1`'s artifact column
  names it `capability-register-closed.json`. The substance `§3a` asks for — **no row in a third
  state** — is present and re-derivable by the script the file names.)

---

## Verdict

**PASS.** Every criterion `AT-35-E1-001` … `AT-35-E6-004` is `complete`, re-derived from the repo
and not from a label. Every one of `§3`'s eleven steps and every `§3a` bullet is green at HEAD
`7753c29915`, with the full `scripts/verify.sh` **49 of 49** run inside this cycle. The eight
shortfalls cycle 3 named, and the five cycle 1 and four cycle 2 named before it, are each closed
at their source — none by an amendment to the bar.

`AT-35-E7-002` (retrospective, written **and cited from `references/README.md` in the same
cycle**) and `AT-35-E7-003` (sweep with counts, architecture docs, graphify, PR, release notes)
are now unblocked. **The operator merges `tranche/15` → `develop`.**
