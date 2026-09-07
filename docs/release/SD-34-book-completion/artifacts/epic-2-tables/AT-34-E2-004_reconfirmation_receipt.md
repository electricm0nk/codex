# Cycle 12-R — Epic 2 (Build eight of the nine tables) / AT-34-E2-004 (reconfirmation at HEAD)

- **Commit SHA:** `2fa209e25f6a7bb8bd68f5a83a9d43e12b8d9d3f` (HEAD at cycle start — no production
  code change was needed; this cycle's own commit carries only this receipt + `progress.md` +
  `kanban.md` + a retro event)
- **Files touched:** `docs/release/SD-34-book-completion/artifacts/epic-2-tables/AT-34-E2-004_reconfirmation_receipt.md`
  (new, this file), `docs/release/SD-34-book-completion/progress.md`,
  `docs/release/SD-34-book-completion/kanban.md`, `docs/retro/events/sd34-at-34-e2-004.jsonl`
  (appended — a `note` event; the file already carries the original cycle's `correction` event)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "**Evidence:** `python3
  scripts/completion_atlas.py --book core_rulebook --check` and `--book ultimate_campaign
  --check` each report bucket A at zero, with movement stated in four buckets (closure /
  reclassification / reachability / instrument-correction)."

## Why this cycle exists

`AT-34-E2-004` was already `complete` on `kanban.md` row 12, recorded at commit `0dd52ccb65` and
committed 2026-08-27 (`artifacts/epic-2-tables/AT-34-E2-004_cycle_receipt.md`). Every sibling
criterion in this epic (`AT-34-E2-001`, `AT-34-E2-002`, `AT-34-E2-003`) has since been
re-dispatched and reconfirmed at HEAD after 97 commits of Epic 3 work landed against
`src/rules_core/`, `src/bin/v06_work_inventory.rs`, and `docs/work-inventory.json` — the exact
files this criterion's evidence is measured against. `AT-34-E2-004` was the one sibling still
carrying only its original, pre-Epic-3 receipt. `decisions.md §12` L2/L19 (never carry your own
number forward — re-derive it) applies here identically: this cycle re-runs both check commands
at HEAD rather than re-quoting the 2026-08-27 receipt, and reports whether bucket A drifted.

## Re-derivation at HEAD

**1. `core_rulebook`, at HEAD:**

```
$ python3 scripts/completion_atlas.py --book core_rulebook --check
book=core_rulebook population=6701 unclassified=0 overlap=0
  DONE: 1438
  A: 0
  B: 534
  C: 414
  D: 382
  M: 1048
  V: 2751
  U: 18
  X: 116
  Z: 0
```

Bucket A: **0**, unchanged from the original cycle's `0` (`6,701` unchanged as the book's
population; `unclassified=0 overlap=0` both hold). Every other bucket moved — this is Epic 3's
work (`AT-34-E3-001`/`E3-003` etc.) landing on the same book, not a regression of this
criterion. Since the original cycle: `DONE 1165→1438` (`+273`), `B 1035→534` (`−501`),
`C 370→414` (`+44`), `D 412→382` (`−30`), `M 921→1048` (`+127`), `V 2734→2751` (`+17`),
`U 58→18` (`−40`), `X 6→116` (`+110`), `Z 0→0`. None of that movement is this cycle's — it is
Epic 3 and the operator rulings (`decisions.md §17`/`§18`) reclassifying `core_rulebook` units
across other buckets. Bucket A itself never moved off zero at any point in that history.

**2. `ultimate_campaign`, at HEAD:**

```
$ python3 scripts/completion_atlas.py --book ultimate_campaign --check
book=ultimate_campaign population=265 unclassified=0 overlap=0
  DONE: 127
  A: 0
  B: 5
  C: 0
  D: 4
  M: 88
  V: 18
  U: 21
  X: 2
  Z: 0
```

Bucket A: **0**, unchanged from the original cycle. Every other bucket is **byte-identical**
to the original cycle's after-figures too (`DONE=127 A=0 B=5 C=0 D=4 M=88 V=18 U=21 X=2 Z=0`) —
`ultimate_campaign` has not been touched by any cycle since `AT-34-E2-004`'s original run; Epic 3
scopes `core_rulebook` only (`epic-breakdown.md` Epic 3 header) and Epic 4 (Ultimate Campaign to
zero) has not yet been dispatched (`workflow-instruction.md §2.4`'s sequential ordering — Epic 4
follows Epic 3).

**3. Corpus-wide check (context, not the criterion's own bar, but confirms no regression on the
same bucket definition the two book checks use):**

```
$ python3 scripts/completion_atlas.py --check
population=49438 buckets=10 unclassified=0 overlap=0
  DONE: 14584
  A: 449
  B: 11964
  C: 4395
  D: 3053
  M: 5076
  V: 9516
  U: 211
  X: 171
  Z: 19
done_evidence_violations=0
missing_clearing_mechanisms=0
stale_derived_at=False
citation_failures=0
```

Exit 0. Corpus-wide bucket A is `449` — **identical to the original cycle's own recorded
after-figure** (`8463 → 449`). The remaining 449 units are entirely `power` (421, deferred to
Epic 5's pricing per `epic-breakdown.md` Epic 2 header) and `companion` (28, the
`ultimate_psionics`/other-book units the original receipt's figures row already named as the
remainder — `python3 scripts/missing_engine_tables.py --check` still reports the same
`power=421, companion=28` split). Neither figure has moved since the original cycle; this
criterion's two named books are not among the books still holding bucket-A units.

**4. Dual-audit gate, re-run on Epic 2's declared file-touch set
(`src/bin/v06_work_inventory.rs`, `src/rules_core/`,
`docs/release/SD-34-book-completion/artifacts/epic-2-tables/`) against the current merge-base
with `origin/develop`:**

```
$ BASE_BRANCH=$(git merge-base HEAD origin/develop)   # ea2b3396f2
$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/bin/v06_work_inventory.rs src/rules_core/ \
    docs/release/SD-34-book-completion/artifacts/epic-2-tables/ ':!**/__tests__/**' ':!**/*.test.*' \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
OK_NO_BUNDLE_TAGS

$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/bin/v06_work_inventory.rs src/rules_core/ \
    docs/release/SD-34-book-completion/artifacts/epic-2-tables/ ':!**/__tests__/**' ':!**/*.test.*' \
  | grep -cnE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'
24
```

24 matches, every one the word `placeholder`, reviewed line-by-line: all are PCGen's own
CHOOSE-menu "no-selection" domain term (`AT-34-E3-001`'s documented vacuous-placeholder
sub-cause, e.g. `src/bin/v06_work_inventory.rs:5205`: `"PCGen's own CHOOSE-menu \"no selection\"
placeholder row for the Barbarian class..."`), plus this receipt schema's own literal grep
pattern text and the sibling `AT-34-E2-003` reconfirmation receipt's prose quoting the same
pattern — none is a stub token in shipping code. Same self-healable false-positive category the
sibling `AT-34-E2-001`/`002`/`003` reconfirmations already found and disposed of.

## Row-count command output

```
$ python3 scripts/completion_atlas.py --book core_rulebook --check | grep '^  A:'
  A: 0
$ python3 scripts/completion_atlas.py --book ultimate_campaign --check | grep '^  A:'
  A: 0
```

Bucket A reads `0` for both named books, directly off the count — not a self-assessment,
unchanged from the original cycle. (As the original receipt noted: `--book --check`'s own exit
code is `1`, because that mode's exit condition is "every non-DONE bucket is zero," a
full-book-closure bar this bundle assigns to `AT-34-E3-005`/Epic 4, not to this criterion.
`acceptance-and-verification.md` row 28 names the evidence as "bucket A at 0," which both printed
counts satisfy exactly.)

## Build scope verified

- `cargo test --locked --no-run` (workspace root, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e2-004`):
  exit 0, all 600 test executables built (matching the count both the original `AT-34-E2-004`
  cycle and the sibling `AT-34-E2-003` reconfirmation recorded), zero `error` lines.
- `apps/desktop/src-tauri` (separate cargo workspace), `cargo test --locked --no-run`
  (`CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e2-004-desktop`): exit 0, one pre-existing `dead_code`
  warning (`SavedCharacterMutationOpDescriptor::op`, unrelated field, same warning every sibling
  reconfirmation this epic recorded), zero errors.
- `cargo test --locked --lib rules_core::rules_tables::simple_kind_tables` (the seven built
  tables' own correctness suite, live proxy for "the tables still hold what the atlas depends
  on"): 13 passed, 0 failed, 0 ignored — identical to the sibling reconfirmations.
- Run at HEAD `2fa209e25f6a7bb8bd68f5a83a9d43e12b8d9d3f`. No production code changed by this
  cycle; this run reconfirms the tree still builds clean and the atlas's own inputs are
  unregressed, not new behavior.

## Sweep population

`corpus_literal_sweep`: **48708 examined**, `0` findings, CLEAN (`3138 tokens exempted under
decisions.md §24 redaction across 1058 codex_generated_name records`). This is **not** this
cycle's movement — `git status --porcelain data/corpus` is empty both before and after (this
cycle touched zero corpus records); the population grew from the original cycle's `48699` to
`48708` (`+9`) entirely from Epic 3's later, unrelated corpus regenerations landed between the
original `AT-34-E2-004` cycle and HEAD — the same `+9` the sibling `AT-34-E2-001` reconfirmation
already recorded and attributed. `decisions.md §12` L8's rule (examined-population must grow by
exactly the record delta) does not apply to this cycle since this cycle adds zero records.

## Oracle pin

N/A — no figure in this receipt was derived from the pinned PCGen oracle corpus; bucket V units
remain proxy-verified only, unaffected by this reconfirmation.

## Denominator gate

```
$ python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'
files_checked=15
violations=0
```

## Box ledger (inherited, read-only, informational — not this criterion's evidence bar)

```
$ python3 scripts/box_ledger.py --check
WARNING: group 'not-ingested' states count=26002 but live recomputation finds 0 (THE-BOX.md needs re-deriving)
WARNING: group 'unmeasurable' states count=321 but live recomputation finds 211 (THE-BOX.md needs re-deriving)
uncovered=19861 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False
EXIT=1
```

Non-zero exit, **pre-existing and growing drift**, not introduced by this cycle: the original
`AT-34-E2-004` receipt already recorded `uncovered=21504` at its own HEAD as "pre-existing drift
from `THE-BOX.md` not being re-derived this bundle, not introduced by this cycle" — `box_ledger`
is inherited **read-only** from SD-33 (`decisions.md §2`: "nothing in SD-34 writes to it") and
`THE-BOX.md` re-derivation is out of this criterion's file-touch set (`workflow-instruction.md
§3` row 2: `src/bin/v06_work_inventory.rs`, `src/rules_core/`, `artifacts/epic-2-tables/` —
`THE-BOX.md` is not named). The `uncovered` count moving from `21504` to `19861` between the
original cycle and this one is Epic 3's classification work continuing to move units the stale
box does not yet track — a decrease, not new damage. Flagged here for visibility; not a blocker
against `AT-34-E2-004`'s own evidence bar (bucket A at 0 for the two named books), which the
independent `completion_atlas.py` checks above satisfy directly.

- **Status:** complete
- **Movement, four buckets:** instrument-correction (this cycle re-verifies the existing
  measurement against HEAD; it moves no unit across any bucket and adds no new figures — it
  confirms the original cycle's bucket-A-zero result has not silently drifted across 97
  intervening commits)
- **Notes:**
  - No drift on the criterion itself: bucket A is `0` for both `core_rulebook` and
    `ultimate_campaign` at HEAD, exactly as the original cycle established. `ultimate_campaign`'s
    entire per-bucket row is byte-identical to the original cycle's after-figures, confirming
    Epic 3's `core_rulebook`-only scope has not touched it. `core_rulebook`'s non-A buckets moved
    substantially (Epic 3's own closure work plus the `decisions.md §17`/`§18` operator rulings)
    but bucket A never left zero across that history.
  - `docs/retro/events/sd31-transcribe.jsonl` observed modified in this shared checkout at cycle
    start (another lane's file, per `workflow-instruction.md`'s standing note) — left untouched,
    confirmed via `git status --porcelain` immediately before every git write this cycle.
  - `box_ledger.py --check`'s non-zero exit is flagged above as inherited drift outside this
    criterion's evidence bar and file-touch set, not self-healed or narrowed around — reported
    plainly rather than silenced, per `decisions.md §12` L15 (a vacuous pass is not a pass; the
    inverse also holds — a pre-existing, out-of-scope fail is not this criterion's failure).
- **Next-cycle plan:** none required by this reconfirmation. Epic 2 (`AT-34-E2-001..004`)
  remains fully `complete` on `kanban.md`, all four criteria now reconfirmed at HEAD with no
  drift found. Epic 3 continues as the active epic; `box_ledger.py`'s growing `uncovered` count
  is named here for whichever cycle re-derives `THE-BOX.md` or closes Epic 6's final-acceptance
  scan, so it is not rediscovered from scratch.
