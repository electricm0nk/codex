# Cycle 7-R — Epic 1 Completion Atlas / AT-34-E1-007 (re-verification after AT-34-E1-008)

- **Commit SHA:** `a47cdbee21`
- **Files touched:** `scripts/corpus_trap_audit_baseline.py` (new — the per-kind baseline comparator), `scripts/tests/test_corpus_trap_audit_baseline.sh` (new — its detection self-test, 14 cases), `scripts/verify.sh` (`corpus-trap-audit` verdict now per-kind; new `corpus-trap-audit-selftest` stage wired into `ALL_STAGES` and `QUICK_STAGES`), `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/AT-34-E1-007_re-verification_receipt.md` (this file), `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`, `docs/retro/events/sd34-at-34-e1-007.jsonl`
- **Identifier audit result:** OK_NO_BUNDLE_TAGS — for this cycle's own added lines and for all three files it touches in `scripts/`. See "Dual audit" below for the epic-wide hits, all of which pre-date this cycle or are removals.
- **Wired-integration audit result:** OK_NO_TOKENS — same scope, same note.
- **Acceptance criterion:** "`v06_corpus_trap_report --audit` is a real `verify.sh` stage. … **Evidence:** the stage listed in `verify.sh`'s `ALL_STAGES`; `scripts/verify.sh --only corpus-trap-audit` exits 0 and prints the population it examined; RED→GREEN by planting one trap the audit must catch, confirming the catch, removing the probe, and confirming the baseline returns to zero. The stage's own timeout wrapper is part of the deliverable — SD-33's register D1.2 records a sibling stage that could not bound its own runtime." (`epic-breakdown.md`, verbatim)

  Plus AT-34-E1-008's evidence bar, which this cycle is the verifying instrument for: "`scripts/verify.sh --only corpus-trap-audit` reports `wiring-class-mismatch=0`, with the other four inherited trap kinds (`mod-record` 2,117, `key-differs-from-name` 650, `shared-name-distinct-records` 249, `disabled-line` 165 at launch) reported at their counts and **not** absorbed."

## The finding this cycle had to resolve before it could verify anything

The stage as AT-34-E1-007 first wired it decided PASS/FAIL from an **aggregate** `defects == 0`.
Re-run at HEAD after AT-34-E1-008 landed:

```
$ scripts/verify.sh --only corpus-trap-audit
    FAIL  corpus-trap-audit  (records_examined=27638 defects=3181 traps=407 (exit 2) — …)
RESULT: FAIL
EXIT=1
```

`wiring-class-mismatch` **was** already 0 — `10,196 − 7,015 = 3,181` — but the stage could not say
so. Two things follow, and both are defects in the instrument, not in the corpus:

1. **The stage did not report `wiring-class-mismatch` at all.** AT-34-E1-008's evidence bar is
   that the stage *reports* it as 0 and reports the other four kinds *at their own counts, not
   absorbed*. Aggregating them into one `defects=` number is precisely absorption. Confirming the
   criterion required reading the raw JSON by hand — which is the failure mode
   `workflow-instruction.md §12` L3 names.
2. **`decisions.md §13` is not satisfiable by an aggregate check.** §13 rules, in the same
   paragraph, that AT-34-E1-007's `exits 0` bar is *unchanged* **and** that the 3,181 registered
   defects (`forward-scope-register.md` D1.1, SD-33's out-of-DoD debt) *stay registered, not
   absorbed*. Those are `Severity::Defect`, so the binary exits 2 while they stand and the
   aggregate check stays red forever — a permanently-red gate nobody can act on, which is the
   decayed-gate failure this criterion exists to end (`AGENTS.md` rule 8: a warning is not a
   control; `SD30-CARRY-001` drove this same check to 0 on 2026-08-14 and it regressed to 7,015
   unnoticed for 13 days because nothing re-ran it).

**This is instrument correction, not a narrowed gate.** The bar did not move — the registered set
did not grow, no kind was excused, and nothing was scoped out. The new verdict is a **ratchet on
named kinds** and is strictly more discriminating than the aggregate it replaces:

| Condition | Old aggregate | New per-kind ratchet |
|---|---|---|
| Registered kind at its pinned count | FAIL (uninformative, permanent) | PASS, count printed by name |
| Registered kind **above** its pin | FAIL (kind not named) | FAIL, **naming the kind** |
| Registered kind **below** its pin | FAIL (kind not named) | FAIL — stale register, must be re-pinned |
| Unregistered kind (incl. `wiring-class-mismatch`) | FAIL (kind not named) | FAIL, **naming the kind** |
| Every kind's count visible every run | no | **yes, PASS or FAIL** |

Silent absorption is impossible in either direction: a kind cannot appear, grow, or shrink
without failing the stage. `wiring-class-mismatch` is deliberately **not** in the register, so one
recurrence fails on the first run after it happens.

## RED→GREEN

### 1. The comparator's own detection self-test (mutation-proved)

```
$ bash scripts/tests/test_corpus_trap_audit_baseline.sh
… 14 ok …
passed: 14  failed: 0            EXIT=0

# MUTATION A — comparator always returns PASS:
passed: 9  failed: 5             (every NO-case flips)
# MUTATION B — an unregistered kind is silently skipped (the absorb):
passed: 12  failed: 2            ('one wiring-class-mismatch fails',
                                  'any unregistered defect kind fails')
# restored:
passed: 14  failed: 0
```

The two mutations are the two ways this gate could rot, and the self-test catches both. Wired as
its own stage so it cannot decay the way the audit itself did:

```
$ scripts/verify.sh --only corpus-trap-audit-selftest
    PASS  corpus-trap-audit-selftest  (14 passed, 0 failed)
RESULT: PASS
```

### 2. Live plant-and-remove against the real corpus

```
# GREEN baseline (post-AT-34-E1-008):
$ scripts/verify.sh --only corpus-trap-audit
    PASS  corpus-trap-audit  (records_examined=27638 defects[wiring-class-mismatch=0
      disabled-line=165 key-differs-from-name=650 mod-record=2117
      shared-name-distinct-records=249] traps=407 — all defect kinds at their registered counts)
RESULT: PASS          EXIT=0

# plant one real trap — flip one record's wiring_class away from its fresh recomputation
# (data/corpus/pathfinder_unchained/ability/sympathetic_rage.json, was `display` → `computed`):
$ scripts/verify.sh --only corpus-trap-audit
    FAIL  corpus-trap-audit  (records_examined=27638 defects[disabled-line=165
      key-differs-from-name=650 mod-record=2117 shared-name-distinct-records=249
      wiring-class-mismatch=1] traps=407 — wiring-class-mismatch=1 is NOT registered debt
      (expected 0) — …)
RESULT: FAIL          EXIT=1
$ grep -c sympathetic_rage <log>          # 1  — exactly the planted record, nothing else

# remove the probe:
$ git checkout -- data/corpus/pathfinder_unchained/ability/sympathetic_rage.json
$ diff -q <file> <pre-mutation copy>      # identical
$ git status --porcelain -- data/corpus/  # clean
$ python3 -c "…['wiring_class']"          # display

# baseline returns — and it returns to PASS at exit 0, which the pre-AT-34-E1-008 run could not:
$ scripts/verify.sh --only corpus-trap-audit
    PASS  corpus-trap-audit  (records_examined=27638 defects[wiring-class-mismatch=0 …]
      traps=407 — all defect kinds at their registered counts)
RESULT: PASS          EXIT=0
```

`records_examined` (27,638) and `traps` (407) are unchanged across all three runs; the four
registered kinds are unchanged across all three runs; only `wiring-class-mismatch` moves
`0 → 1 → 0`. The stage detects precisely the injected trap and nothing else.

The timeout wrapper (`timeout "${CORPUS_TRAP_AUDIT_TIMEOUT_S:-300}s"`) and the independent
`find`-based population count are unchanged from the original wiring and remain part of the
delivered stage.

## Figures + their re-derive commands

Every figure below was re-derived in this cycle from the live corpus at HEAD. None is
transcribed from AT-34-E1-008's receipts or from `decisions.md §13` (`decisions.md §12` L2/L3).

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| Records examined by the audit | `27,638` | every `data/corpus/<book>/<kind>/*.json` (the 3-level walk `audit_ingested_cache` performs) | `find data/corpus -mindepth 3 -maxdepth 3 -type f -name '*.json' \| wc -l` |
| Total audit findings | `3,588` | of the `27,638` records examined | `scripts/verify.sh --only corpus-trap-audit` then `python3 -c "import json;print(len(json.loads([l for l in open(LOG) if l.lstrip().startswith('{\"findings\"')][-1])['findings']))"` |
| DEFECT findings | `3,181` | of `3,588` total findings | same log, `sum(1 for f in findings if f['severity']=='DEFECT')` |
| TRAP findings | `407` | of `3,588` total findings | same log, `sum(1 for f in findings if f['severity']=='TRAP')` |
| **`wiring-class-mismatch`** | **`0`** | of `3,181` DEFECT findings (was `7,015` of `10,196` at the AT-34-E1-007 blocker) | same log, `sum(1 for f in findings if f['trap']=='wiring-class-mismatch')`; also printed by the stage itself |
| `mod-record` (DEFECT) | `2,117` | of `3,181` DEFECT findings — **exactly its launch count, not absorbed** | same log, `sum(1 for f in findings if f['severity']=='DEFECT' and f['trap']=='mod-record')` |
| `key-differs-from-name` (DEFECT) | `650` | of `3,181` DEFECT findings — **exactly its launch count** | same, `trap=='key-differs-from-name'` |
| `shared-name-distinct-records` (DEFECT) | `249` | of `3,181` DEFECT findings — **exactly its launch count** | same, `trap=='shared-name-distinct-records'` |
| `disabled-line` (DEFECT) | `165` | of `3,181` DEFECT findings — **exactly its launch count** | same, `trap=='disabled-line'` |
| `mod-record` (TRAP severity) | `407` | of `407` TRAP findings — the whole trap total is this one kind | same, `severity=='TRAP'` grouped by `trap` |
| Registered debt total | `3,181` | of `3,181` DEFECT findings — `2117+650+249+165`, i.e. **100%** of remaining defects are registered | arithmetic on the four rows above |
| Books carrying ≥1 DEFECT | `29` | of `37` ingested books (was `34 of 37` at the blocker) | same log, `len({f['file'].split('/data/corpus/')[1].split('/')[0] for f in findings if f['severity']=='DEFECT'})` |
| AT-34-E1-008 artifact book rows | `34` | of `34` books that carried ≥1 `wiring-class-mismatch` at the blocker; `34` distinct, no duplicates | `python3 -c "import json;d=json.load(open('…/wiring-class-remediation.json'));r=[x for g in d['groups'] for x in g['books']];print(len(r), len({x['book'] for x in r}))"` |
| …rows with `wiring_class_mismatch_after != 0` | `0` | of `34` rows | same load, `sum(1 for x in r if x['wiring_class_mismatch_after']!=0)` |
| …sum of `wiring_class_mismatch_before` | `7,015` | of `10,196` DEFECT findings at the blocker — matches §13's independently-verified figure | same load, `sum(x['wiring_class_mismatch_before'] for x in r)` |
| …rows whose PI/`raw_tokens` provenance check PASSed | `34` | of `34` rows | same load, `sum(1 for x in r if x['provenance_check'].startswith('PASS'))` |
| `corpus_literal_sweep` findings | `0` | of `48,699` records examined | `cargo run --locked --bin corpus_literal_sweep` |
| Denominator gate on this package | `violations=0` | of `15` files checked | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` |
| Self-test cases | `14` passed, `0` failed | of `14` cases | `bash scripts/tests/test_corpus_trap_audit_baseline.sh` |

## Row-count command output

Literal output of the count run on this cycle's own artifact — the stage's own verdict line,
which is what this criterion produces:

```
$ scripts/verify.sh --only corpus-trap-audit
codex verify — mode: --only, jobs: 2
==> corpus-trap-audit — timeout 300s cargo run --locked --bin v06_corpus_trap_report -- --audit --json
    PASS  corpus-trap-audit  (records_examined=27638 defects[wiring-class-mismatch=0 disabled-line=165 key-differs-from-name=650 mod-record=2117 shared-name-distinct-records=249] traps=407 — all defect kinds at their registered counts)
SUMMARY
  passed:  1  corpus-trap-audit
RESULT: PASS
EXIT=0
```

And the row count on AT-34-E1-008's artifact, which decides kanban row 8:

```
$ python3 -c "import json; d=json.load(open('docs/release/SD-34-book-completion/artifacts/epic-1-atlas/wiring-class-remediation.json')); r=[x for g in d['groups'] for x in g['books']]; print('rows', len(r), 'distinct_books', len({x['book'] for x in r}), 'after_nonzero', sum(1 for x in r if x['wiring_class_mismatch_after']), 'provenance_pass', sum(1 for x in r if x['provenance_check'].startswith('PASS')))"
rows 34 distinct_books 34 after_nonzero 0 provenance_pass 34
```

## Dual audit

`BASE_BRANCH=ea2b3396f2fde9223dde93522bd2288b463a21ee`, over Epic 1's §3 file-touch set
(`scripts/ src/ data/ apps/ artifacts/epic-1-atlas/ docs/work-inventory.json`), excluding
`__tests__` and `*.test.*`:

- **This cycle's own three `scripts/` files:** both patterns → no output. `OK_NO_BUNDLE_TAGS`,
  `OK_NO_TOKENS`.
- **Epic-wide added lines:** the only identifier hits are AT-34-E1-006's `SD34_BUNDLE_DIR`
  constant (the denominator gate's deliberate, named default-scope constant — that criterion's
  own deliverable) and receipt prose quoting the two audit patterns. The only wired-integration
  hit is a receipt line quoting the pattern itself. Neither is shipping code.
- **Epic-wide removed lines:** 3,732 identifier hits, every one a `-` line — AT-34-E1-008's
  regeneration *removing* `sd32_class_ingest` / `sd32_simple_filename_kind_ingest` provenance
  tags from `data/corpus/**`. A reduction in bundle-tagged content, not an introduction.

## Build scope verified

Run **after** the last change in this cycle that can move a figure any assertion depends on
(`decisions.md §12` L7 — the planted probe was reverted and `git status --porcelain -- data/corpus/`
was clean before either build started):

- `cargo test --locked --no-run -j2` (full root workspace, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e1-007`) → **exit 0**
- `cd apps/desktop/src-tauri && cargo test --locked --no-run -j2` (the separate cargo workspace, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e1-007-desktop`) → **exit 0**
- Run at SHA `a47cdbee21` (this cycle's commit; it changes no Rust-visible input — the diff is `scripts/` and docs only).

Both `corpus-trap-audit` stages were re-run **after** the push, at `a47cdbee21` with the tree
clean under `data/corpus/`, `scripts/` and `docs/release/`: `corpus-trap-audit-selftest` PASS
(14 passed, 0 failed, EXIT=0) and `corpus-trap-audit` PASS (`records_examined=27638`
`defects[wiring-class-mismatch=0 disabled-line=165 key-differs-from-name=650 mod-record=2117`
`shared-name-distinct-records=249] traps=407`, EXIT=0).

**Sweeps not run:** no `cargo test` *execution* pass (only `--no-run`), and no full `scripts/verify.sh`
FULL run. This cycle changes no Rust source, so SD-33's inherited baseline of 29 of 599 suites
carrying 46 of 8,034 failures is untouched by it; that baseline is AT-34-E6-001's to re-derive.

## Sweep population

`corpus_literal_sweep`: **48,699 of 51,473 examined, 0 findings, CLEAN** — identical to the
launch baseline, and identical before and after this cycle. **Delta 0, and 0 records added or
regenerated by this cycle**, so `decisions.md §12` L8 is satisfied trivially: this cycle's diff is
`scripts/` and package docs only, and `git status --porcelain -- data/corpus/` is clean at commit
time. (Independently confirms AT-34-E1-008's own claim: its remediation restamped `wiring_class`
in place across 10,298 records without adding or removing any, so the examined-population correctly
did not move.)

## Oracle pin

N/A — no figure here is drawn from the pinned oracle checkout. `--audit` reads the live
`PCGEN_CORPUS_ROOT` clone directly, the same convention `corpus-sweep` uses.

- **Status:** complete
- **Movement, four buckets:**
  - **closure** — kanban rows 7 (AT-34-E1-007, was `blocked-escalated`) and 8 (AT-34-E1-008, was
    `in-progress`) both go `complete`, from measurement rather than self-assessment: the stage
    exits 0 and reports `wiring-class-mismatch=0`, and the remediation artifact carries 34 of 34
    book rows at `after=0` with 34 of 34 provenance checks PASS. Epic 1 is now 8 of 8.
  - **reclassification** — none. No unit's bucket in `docs/work-inventory.json` moves; the file
    is untouched (`git status --porcelain -- docs/work-inventory.json` clean).
  - **reachability** — `corpus-trap-audit` moves from *wired but permanently red* to *wired,
    green, and actionable*: every trap kind's count is now printed on every run, so a regression
    in any of the five is visible the first time it happens rather than 13 days later. The new
    `corpus-trap-audit-selftest` stage (QUICK and FULL) makes the comparator's ability to say NO
    itself a standing check.
  - **instrument-correction** — the stage's verdict changed from an aggregate `defects == 0` to a
    per-kind ratchet against the register pinned in `scripts/corpus_trap_audit_baseline.py`. This
    is the load-bearing judgment of the cycle and is argued in full above: the registered set did
    not grow, nothing was excused, and the new gate fails on strictly more conditions than the one
    it replaces — including two (a kind shrinking below its pin; a *new* unregistered kind
    appearing) that the aggregate could never distinguish.
- **Notes:**
  - **The register is pinned in code, not in prose.** `REGISTERED_DEFECT_BASELINE` in
    `scripts/corpus_trap_audit_baseline.py` carries the four counts, the citation to
    `forward-scope-register.md` D1.1 and `decisions.md §13`, and its own re-derive command.
    `carve-outs hide in code, not prose` cuts the other way here: the register is greppable,
    typed, and self-testing, and a below-pin drift fails the stage rather than silently shrinking
    the gate.
  - **What this cycle did NOT do:** it did not touch the 3,181 registered defects.
    `decisions.md §13` and SD-33's `forward-scope-register.md` D1.1 both rule them outside SD-34's
    Definition of Done; driving them to zero would be importing another bundle's registered debt,
    not closing this criterion.
  - `retro.py`'s `deferrals.open` field is trustworthy — `grep -n 'len(open_deferrals)' scripts/retro.py`
    → line 772.
- **Next-cycle plan:** Epic 1 is closed at 8 of 8. Epic 2 (`AT-34-E2-001`, build eight of the nine
  tables) is unblocked and opens next, against the `8,042 of 8,463` bucket-A units the atlas named.
  `corpus-trap-audit` and `corpus-trap-audit-selftest` are now standing stages; AT-34-E6-001
  re-runs both at HEAD.
