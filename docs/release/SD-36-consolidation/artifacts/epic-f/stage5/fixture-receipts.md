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
