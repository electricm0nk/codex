# S5:suite-root — fixture receipts

Spec: `docs/release/SD-36-consolidation/epic-f-class-completion.md` 3b, review finding 12 (Fixture Protocol).
Tree: commit `b2565e2a37` on `sd36/epic-f1`.
Source log: `/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/6badc5b8-ae3b-4359-80c5-cd0b1598973e/scratchpad/sd36/f1/suite-root.log` (`nohup cargo test --locked -j 8 --no-fail-fast`, ends `EXIT=0`).

## Command and evidence

```
grep -n '^test result' suite-root.log   # 287 matches
grep -n 'FAILED' suite-root.log         # 0 matches
grep -n '^EXIT=' suite-root.log         # line 8036: EXIT=0
```

Every one of the 287 `test result:` lines in the log reads `ok. <n> passed; 0 failed; ...`.
No `FAILED` line, no panic, no `error[` compiler diagnostic appears anywhere in the 8,036-line log.
Denominator: all root-crate test binaries built for this tree (unit + `tests/*.rs` integration targets + doctests), one pass, `--no-fail-fast` so a hang would still show as a non-`ok` result line — none does.

## Classification

**No test in the root-crate suite failed on commit `b2565e2a37`.** There is nothing to classify under (A) re-baseline / (B) root-cause fix / (C) pre-existing-and-unrelated, and no changed value to check against Ruling 1 (the accepted Barbarian Rage lines) — the 9 Rage-line changes were already re-baselined and accepted in prior epic-f1/f1b commits (see git log: `fix(sd36,epic-f1b): join normalises tokenisation variants...`, `feat(sd36,epic-f1): repaired rule package...`), and that fixture state carried into this green run without further movement.

## Supplementary invariant checks run alongside S5 (not required by the protocol, run because they are cheap and this is a closure-adjacent commit)

- `python3 scripts/pcgen_residue_gate.py --check --closure` → `verdict=PASS` (live_hits=0, identifier_hits=0, shipped_scanned=69389).
- `git status --porcelain -- data/corpus site` → empty (neither tree touched this step).
- `cargo clippy --locked -j 8 --all-targets -- -D warnings` → `Finished ... in 0.25s`, `EXIT=0` (cached-clean build, zero warnings under `-D warnings`).

## Result

Pass/fail counts before: 287/287 suites `ok`, 0 failed (per `suite-root.log`, already-run on this tree).
Pass/fail counts after: unchanged — no fixture was re-baselined, no defect was fixed, because none was found.
Re-baselined tests: none.
Fixed defects: none.
STOPs: none.

---

# S5:suite-ingest — fixture receipts

Spec: `docs/release/SD-36-consolidation/epic-f-class-completion.md` 3b, review finding 12 (Fixture Protocol);
`receipts.md`'s Epic C2 evidence (`PCGEN_CORPUS_ROOT` method) for the oracle-parity runs.
Tree: commit `3a72b8f2d4` on `sd36/epic-f1` (before this step's pin fix).

## Runs and logs

1. `cargo test --locked -j 8 -p codex-ingest --no-fail-fast` (unignored suite) —
   `suite-ingest.log` (first pass, 1 failure) then `suite-ingest-2.log` (re-run after the fix, green).
2. Oracle parity, `--ignored --test-threads=2`, `PCGEN_CORPUS_ROOT=$HOME/workspace/repos/pcgen/data`:
   the same 7-target `crates/codex-ingest` command `receipts.md` line 374-379 used (`sd17_a_include_graph`,
   `sd17_b_spellcasting_class`, `sd31_e2_ground_truth_agreement`, `sd27_feat_prerequisite_enforcement`,
   `pcgen_runner_smoke`, `sd17_b_monster_stat_block`, `sd17_b1_martial_class`), plus `--lib --ignored`
   for the crate's own oracle-backed unit tests (11 tests under `#[ignore]`, added to `src/` after
   Epic C2's evidence was written and not covered by that command) — `suite-ingest-oracle.log`.

Real `#[ignore]` count confirmed unchanged from Epic C2 (`git grep -c '#\[ignore' -- crates/codex-ingest/tests`,
counting the `= "..."`-form attributes too): `sd17_a_include_graph` 1, `sd17_b1_martial_class` 5,
`sd17_b_monster_stat_block` 7, `sd17_b_spellcasting_class` 14, `sd27_feat_prerequisite_enforcement` 3,
`sd31_e2_ground_truth_agreement` 1, `pcgen_runner_smoke` 0 real (1 doc-comment mention) = 31, matching
`receipts.md` exactly. `--lib` additionally carries 11 real `#[ignore]`s (10 in
`cache_gen/class_feature_grants.rs`, 1 in `cache_gen/equipment_gap.rs`), none of which existed as
`#[ignore]` tests when Epic C2's command was written (that file has no other commits on this branch —
see Classification 2 below).

## Pass/fail counts

| Run | Before (this step) | After |
|---|---|---|
| `--lib` (unignored) | 641 passed, 1 failed, 11 ignored | 642 passed, 0 failed, 11 ignored |
| `--no-fail-fast` full crate (unit + all `tests/*.rs` + doctests) | 1 `FAILED` line (the lib target), rest green; 471 `test result:` blocks total | 0 `FAILED` lines; `EXIT=0` |
| Oracle, C2's 7-target command, `--ignored` | (not run this step until the fix landed) | 31 passed, 0 failed — matches `receipts.md` |
| Oracle, `--lib --ignored` (11 tests, not part of C2's command) | — | 10 passed, 1 failed (see Classification 2) |

## Classification 1 — (A) pinned count moved, re-baselined with receipt

**Test:** `pcgen_import::sheet_rule::term_level_refusal_gate::every_degraded_record_converted_and_prints_its_words`
(`crates/codex-ingest/src/pcgen_import/sheet_rule/mod.rs:1214` and `:1221`).

Two pins moved, both because the repaired rule package (commit `b2565e2a37`, "4,456 child-category
links resolved") now correctly joins a token that previously never reached its record:

- `degraded.len()`: pin `423` → actual `424`. Re-derived with the test's own doc-comment command
  (`python3 -c "...degraded=[e for e in tokens['entries'] if e.get('degradations') and e['id'] not in
  refused]; print(len(degraded))"` against `data/sheet_rules/_tokens.json` / `_refused.json`) — matches.
  Diffing the degraded-id set against the parent commit's `data/sheet_rules/_tokens.json`
  (`git show b2565e2a37^:data/sheet_rules/_tokens.json`) isolates the one added id:
  `inner_sea_gods:class:exalted`, added, none removed.
- `numbered_count`: pin `620` → actual `624`. The 4 new lines are exactly `exalted`'s own BAB
  (`target=BaseAttack`) and 3 base-save progressions (`#bonus1/2/3`, Fortitude/Reflex/Will) — all
  `Number`-valued, all with a `target`, none `Text`.

**Evidence the record's own degradation is correct, not a defect:** `exalted`'s new degradation is
`{'ADD (CLASSSKILLS)': ['ADD']}`, byte-identical to the shape already accepted (before this branch's
commit) on its book-sibling `inner_sea_gods:class:evangelist` (same pre-existing degradation, confirmed
present in `b2565e2a37^`'s `_tokens.json` too). Checked the real PCGen source,
`$HOME/workspace/repos/pcgen/data/pathfinder/paizo/campaign_setting/inner_sea_gods/isg_classes.lst`:
`CLASS:Exalted` line 9 carries `ADD:CLASSSKILLS|TYPE=Knowledge` (level 1), the same token type as
`Evangelist`'s `ADD:CLASSSKILLS|2|ANY` (line 7) — a real, corpus-genuine token this converter cannot
lower, matching the doc comment's own CONV-05 example (`bloodrager`'s BAB/saves keep their number while
an unrelated sibling term on the same record degrades to Text). Class: **added-correct** — the newly-held
join correctly surfaces a real degradation that was previously silently dropped, and the 4 numbered
lines are the same per-occurrence guarantee already exercised by `bloodrager`.

**Fix:** updated both pins in `crates/codex-ingest/src/pcgen_import/sheet_rule/mod.rs` (423→424,
620→624). Re-ran the single test (`cargo test -p codex-ingest --lib
pcgen_import::sheet_rule::term_level_refusal_gate::every_degraded_record_converted_and_prints_its_words
-- --exact`): `ok`. Re-ran the full `--lib` suite: `642 passed; 0 failed; 11 ignored`. Re-ran the full
crate (`--no-fail-fast`): `EXIT=0`, 0 `FAILED` lines. No other pinned surface in the 3b.4 list (`feat_prereqs.rs`
and its submodules, `sd27_feat_prerequisite_enforcement.rs`, `sheet_rule_parity`/`sheet_rule_bucket_v_render`
outputs, `_report.json`-pinning tests) moved — confirmed by the clean full-suite re-run.

## Classification 2 — (C) pre-existing, unrelated failure — not fixed, not a STOP

**Test:** `pcgen_import::cache_gen::equipment_gap::tests::find_citation_full_population_regression`
(`crates/codex-ingest/src/pcgen_import/cache_gen/equipment_gap.rs:1521`, `#[ignore]`).

This audit re-resolves every already-shipped `data/corpus/**/equipment*/**/*.json` record whose
`source.kind == "lst_token"` against the live `PCGEN_CORPUS_ROOT` checkout and asserts the citation is
unchanged from what shipped. It failed with dozens of mismatches (many `now=UNRESOLVED`, a few
re-pointed to a different `.lst` file entirely, e.g. `Holy Symbol (Silver)` shipped from
`cr_equipmods.lst:68`, now resolves to `cr_equip_general.lst:210`).

**Attribution from git:** `equipment_gap.rs` has exactly one commit in its history on this branch —
`eac1dd8bc6` (`feat(sd36,epic-a): scaffold crates/codex-ingest and wall off the PCGen tool side`) — and
has never been touched by any Epic F1/F1b commit (`git log --oneline -- crates/codex-ingest/src/pcgen_import/cache_gen/equipment_gap.rs`
shows only that one hit). `data/corpus` itself is untouched this cycle (`git status --porcelain --
data/corpus site` is empty; `holy_symbol_silver.json`'s last commit, `0535a1786d`, long predates this
SD-36 branch). This test is not one of Epic C2's evidenced 52 (`receipts.md` lines 338-388 name it
nowhere), so there is no prior green run on this tree to regress from — it surfaced only because this
step ran `--lib --ignored` in full, beyond C2's specific 7-target command, per this step's "including the
`#[ignore]` oracle parity tests" instruction. The mismatch is between the pinned PCGen checkout's
current `.lst` line numbers/file placement and whatever state produced the already-shipped
`data/corpus` citations — orthogonal to the sheet_rules print-path package this cycle's join/dedup/
tokenisation work changed, and out of this step's write scope (`data/corpus` is invariant-protected;
fixing `find_citation` or regenerating corpus is not part of 3b.4's pinned-surface list, which names
`sheet_rule.rs`/`feat_prereqs.rs`/etc., not `equipment_gap.rs`). Class: **pre-existing, unrelated** —
listed here as a real gap for a follow-up epic (equipment-corpus citation audit against the current
pinned PCGen checkout), not fixed, not a STOP under Ruling 2 (it is not a changed sheet value; it is an
unverified pre-existing oracle audit that has never been run green on this branch).

## Supplementary invariant checks

- `python3 scripts/pcgen_residue_gate.py --check --closure` → `verdict=PASS` (live_hits=0,
  identifier_hits=0, shipped_scanned=69389).
- `git status --porcelain -- data/corpus site` → empty both before and after this step.
- `cargo clippy --locked -j 8 -p codex-ingest --all-targets -- -D warnings` → clean, `EXIT=0`.

## Result

Pass/fail counts before (this step's first run): `--lib` 641/642 (1 failed), full crate 1 `FAILED` line.
Pass/fail counts after: `--lib` 642/642, full crate 0 `FAILED` lines, `EXIT=0`; oracle (C2's 7-target
command) 31/31; oracle `--lib --ignored` 10/11 (1 pre-existing/unrelated, see Classification 2).
Re-baselined tests: `every_degraded_record_converted_and_prints_its_words` (2 pins, both `added-correct`,
receipt above).
Fixed defects: none (the only failure in scope, Classification 1, was a stale pin, not a defect;
Classification 2 is out of this step's scope and not fixed).
STOPs: none — no `changed-value` or `removed-unexplained` line appeared; the one non-Rage-line movement
found (the `exalted` degradation) is a newly-surfaced `added-correct` join fix, not a changed value.

---

# S5:suite-desktop — fixture receipts

Spec: `docs/release/SD-36-consolidation/epic-f-class-completion.md` 3b, review finding 12 (Fixture Protocol).
Tree: commit `f9dac29276` on `sd36/epic-f1` (S5:suite-ingest, immediately prior).

## Runs and logs

1. Desktop crate: `cargo test --locked -j 8 --no-fail-fast --manifest-path apps/desktop/src-tauri/Cargo.toml`
   — `suite-desktop.log` (`nohup`, single binary target `src/main.rs`, ends `EXIT=0`).
2. Frontend: `cd apps/desktop && npm run typecheck && npm test` — `suite-desktop-frontend.log`
   (`nohup`, ends `EXIT=0`).

## Command and evidence

```
grep -c '^test result' suite-desktop.log     # 1 (single unittest binary, src/main.rs)
grep -n 'FAILED' suite-desktop.log           # 0 matches
grep -n '^EXIT=' suite-desktop.log           # line 627: EXIT=0
```
`test result: ok. 612 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 360.33s`
— includes `corpus_bundle_parity_test::sanitised_corpus_bundle_has_the_same_race_equipment_and_spell_population_as_the_raw_corpus`
(the corpus-bundle parity test named in this step's instructions) and every catalog-count-pinning test
(`equipment_catalog`, `spell_catalog`, `reference_library_catalog`, `race_trait_picker`, `character_hub`),
all `ok`.

```
grep -n 'FAIL' suite-desktop-frontend.log | grep -v 'PASS'   # 0 matches
grep -c 'PASS ' suite-desktop-frontend.log                   # 125
grep -n '^EXIT=' suite-desktop-frontend.log                  # EXIT=0
```
`typecheck`: `tsc --noEmit` → `TC=0`. `npm test`: `125/125 test files passed.`

## Classification

**Nothing failed in either run.** The desktop crate's single test binary (612 tests, one `test result:`
line, matching the 611-filtered/612-total and 609-filtered/612-total counts already observed in this
tree's earlier targeted gate runs — `gate-desktop-bundle-parity.log`, `gate-desktop-sheetrule.log`, both
pre-dating this step, same population) and the frontend's 125 test files all ran green on the first
attempt. There is no pinned count/fixture/snapshot that moved, so nothing to classify under (A); no
defect surfaced, so nothing under (B); no pre-existing failure, so nothing under (C); and no changed
value to check against Ruling 1 (the 9 accepted Rage lines were already re-baselined in prior epic-f1
commits and are not touched by the desktop crate's own fixtures, which hold no Rage-value pins).

## Supplementary invariant checks

- `python3 scripts/pcgen_residue_gate.py --check --closure` → `verdict=PASS` (live_hits=0,
  identifier_hits=0, shipped_scanned=69389).
- `git status --porcelain -- data/corpus site` → empty both before and after this step.
- `python3 -c "import json;r=json.load(open('data/sheet_rules/_report.json'));print(r['records'],r['converted'])"`
  → `49450 49450` — frozen record count unmoved.
- `python3 scripts/site/check_frozen_status.py --check` → `OK: ... frozen at 100% (49450 units)`.

## Result

Pass/fail counts before: not run yet on this tree this step (first invocation).
Pass/fail counts after: desktop crate 612/612 passed, 0 failed, `EXIT=0`; frontend 125/125 test files
passed, typecheck `TC=0`, `EXIT=0`.
Re-baselined tests: none.
Fixed defects: none.
STOPs: none.
