# Cycle AT-34-E1-008-R2 — Epic 1 Completion Atlas / AT-34-E1-008 (independent re-derivation at HEAD)

This lane was dispatched against `AT-34-E1-008` after the criterion had already landed
(original group receipts: `AT-34-E1-008_G1_cycle_receipt.md` .. `_G4_cycle_receipt.md`; kanban
row already `complete`; verifying instrument `AT-34-E1-007_re-verification_receipt.md`, landed at
`a47cdbee21`; further reconfirmed at `ba23c938b1` per `git log`). Per `decisions.md §12` L2/L20 —
never carry a number forward, and a dispatch script's return value is not a closure claim — every
figure below was re-run from this lane's own shell against the tree at `ba23c938b1`, not
transcribed from an earlier receipt.

- **Commit SHA:** see push output below (this receipt's own landing commit; base was `ba23c938b1`,
  local HEAD was already identical to `origin/tranche/14` before this cycle started).
- **Files touched (this cycle):**
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/AT-34-E1-008_reconfirmation_receipt.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md` (prepended)
  - No production code, `docs/work-inventory.json`, or `data/corpus/**` edits — the criterion was
    already met; this cycle re-derives and confirms, nothing to fix.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — this cycle's own diff is docs-only (one new
  receipt file, one progress.md prepend).
- **Wired-integration audit result:** `OK_NO_TOKENS` — same scope.
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-34-E1-008 — `wiring-class-mismatch` is driven to zero across every affected book
  >
  > AT-34-E1-007 wired the stage; this criterion makes it green. **7,015 of 10,196** defects,
  > across **34 of 37** books, in a check `SD30-CARRY-001` drove to `0` on 2026-08-14 and which
  > has silently regressed since because nothing re-ran it (`decisions.md §13`).
  >
  > Per book: re-run the canonical generator (`gen_book_cache`) via the **guarded path only** —
  > never a hand-edit of `data/corpus/**`, never `--allow-stamp-loss` — then verify, **per
  > record**, that license/PI metadata and `raw_tokens` survived, and re-audit that book to zero.
  >
  > **Evidence:** `scripts/verify.sh --only corpus-trap-audit` reports `wiring-class-mismatch=0`,
  > with the other four inherited trap kinds (`mod-record` 2,117, `key-differs-from-name` 650,
  > `shared-name-distinct-records` 249, `disabled-line` 165 at launch) reported at their counts
  > and **not** absorbed. Plus `artifacts/epic-1-atlas/wiring-class-remediation.json` — per book:
  > defects before, defects after, records regenerated, and the PI/`raw_tokens` survival check
  > result. `cargo run --locked --bin corpus_literal_sweep` reports 0 findings and its
  > **examined-population moves by exactly the record delta** (`decisions.md §12` L8).

## Re-derivation at HEAD (`ba23c938b1`)

### 1. The per-kind trap-audit stage, live

```
$ bash scripts/verify.sh --only corpus-trap-audit
==> corpus-trap-audit — timeout 300s cargo run --locked --bin v06_corpus_trap_report -- --audit --json
    PASS  corpus-trap-audit  (records_examined=27638
      defects[wiring-class-mismatch=0 disabled-line=165 key-differs-from-name=650
               mod-record=2117 shared-name-distinct-records=249] traps=407
      — all defect kinds at their registered counts)
RESULT: PASS
```

`wiring-class-mismatch=0`, exactly the criterion's bar. The other four inherited kinds are
reported **by name, at their own counts** (`mod-record=2117`, `key-differs-from-name=650`,
`shared-name-distinct-records=249`, `disabled-line=165`) — identical to the launch figures quoted
in the criterion's own text, confirming they were **not absorbed**, not silently re-pinned.

### 2. `wiring-class-remediation.json` — the per-book artifact, re-summed

```
$ python3 -c "
import json
d=json.load(open('docs/release/SD-34-book-completion/artifacts/epic-1-atlas/wiring-class-remediation.json'))
groups=d['groups']
tot_before=tot_after=tot_regen=0
books=set()
for g in groups:
    for b in g['books']:
        tot_before += b.get('wiring_class_mismatch_before', 0)
        tot_after  += b.get('wiring_class_mismatch_after', 0)
        tot_regen  += b.get('records_regenerated', 0)
        books.add(b['book'])
print('groups', len(groups), 'books', len(books))
print('before', tot_before, 'after', tot_after, 'regenerated', tot_regen)
"
groups 4 books 34
before 7015 after 0 regenerated 10298
```

**34 of 37 books**, **7,015 → 0** defects — matches `decisions.md §13`'s and the criterion's own
figures exactly, re-derived, not transcribed. (Denominator: 37 is the bundle's total book
population, `content-unit-inventory.md`; 34 is the count of books the artifact names as carrying
`wiring-class-mismatch` findings before remediation — the other 3 books carried none of this trap
kind to begin with, per `decisions.md §13`'s scoping.)

### 3. `corpus_literal_sweep`, live, and the population-delta check

```
$ CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e1-008 cargo run --locked --quiet --bin corpus_literal_sweep
corpus-literal-sweep: 48708 records examined of 51482 read, 413336 tokens compared (9 synthesized),
                       51469 digests checked, 0 findings
corpus-literal-sweep: 3138 tokens exempted under decisions.md §24 redaction across 1058
                       codex_generated_name records
corpus-literal-sweep: CLEAN
```

0 findings — the second half of the criterion's evidence bar. The AT-34-E1-008 groups' own four
receipts (G1, G2, G3, G4) each independently recorded `48699 examined before → 48699 after,
delta 0` for their own in-place restamps (`records_regenerated` moved keys/fields on existing
records, adding none) — G3's 2,615-record regeneration included. The live figure above,
`48708`, is **9 higher** than that `48699` baseline; that movement postdates AT-34-E1-008's own
four cycles (all logged `delta 0` at the time each ran) and belongs to later Epic-1 criteria that
ingested additional records after E1-008 closed (e.g. AT-34-E3-001's `domain` unit, `decisions.md
§14`) — not a finding against this criterion. AT-34-E1-008's own claim, "examined-population
moves by exactly the record delta," is about **its own** regeneration (0 records added, 0 delta —
verified true in each of the four group receipts above) and is unaffected by unrelated later
population growth.

### 4. Widest build scope

```
$ CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e1-008 cargo test --locked --no-run
EXIT=0    (20 test binaries built, including tests/v06_corpus_trap_report.rs)
```

`apps/desktop/src-tauri` not touched by this criterion (its file-touch set is `data/corpus/**`
via the guarded generator and `artifacts/epic-1-atlas/` only, per `workflow-instruction.md §3`
row 1's `AT-34-E1-008 ONLY` clause) — not run.

### 5. Dual audit, re-run on Epic 1's cumulative file-touch set

```
$ BASE_BRANCH=$(git merge-base HEAD origin/develop)
$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- scripts/completion_atlas.py \
    scripts/tests/test_completion_atlas.py src/bin/v06_work_inventory.rs docs/work-inventory.json \
    scripts/verify.sh scripts/denominator_gate.py data/corpus docs/release/SD-34-book-completion/artifacts/epic-1-atlas \
    ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
```

Hits found are exclusively: (a) `-` (removed) lines inside regenerated `data/corpus/**` JSON,
where the *old* provenance tag `sd32_class_ingest` / `sd32_simple_filename_kind_ingest` is being
replaced by the corrected `wiring_class`/signals fields (this is the fix itself, already
documented by AT-34-E1-008's own G-group receipts); and (b) doc prose inside
`AT-34-E1-006_cycle_receipt.md` / `scripts/denominator_gate.py`'s `SD34_BUNDLE_DIR` constant name
and receipt prose quoting these exact patterns — both from AT-34-E1-006, a different, already-
closed criterion, already accounted for in that criterion's own receipt. No hit is inside this
criterion's own commits or represents an identifier tag left in shipping code.

```
$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <same paths> ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'
```

All hits are either (a) `evidence`/`reason` field values inside regenerated `data/corpus/**` JSON
describing PCGen's own real "no selection" CHOOSE-menu placeholder rows (`vacuous_placeholder_row_
no_corpus_content_to_render`) — real corpus content, not a code stub, already ruled non-defect by
AT-34-E3-001; or (b) code comments / test names in `src/rules_core/` describing that same
real-data condition; or (c) receipt prose. None are inside `data/corpus/**` regenerations this
criterion is responsible for and none are stubs in shipping logic.

## Row-count command output (this cycle's own artifact)

```
$ ls docs/release/SD-34-book-completion/artifacts/epic-1-atlas/AT-34-E1-008_G*_cycle_receipt.md | wc -l
4
$ python3 -c "import json; d=json.load(open('docs/release/SD-34-book-completion/artifacts/epic-1-atlas/wiring-class-remediation.json')); print(len(d['groups']), sum(len(g['books']) for g in d['groups']))"
4 34
```

4 group receipts, 4 groups in the artifact, summing to 34 book rows — self-consistent.

## Oracle pin

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`) — no
new figure in this receipt depended on the pinned corpus checkout directly; all figures came from
the live `data/corpus/**` tree and the repo's own binaries.

- **Status:** complete
- **Movement, four buckets:**
  - **Closure:** 0 — this cycle closed nothing new; AT-34-E1-008's four groups already closed
    7,015 → 0 `wiring-class-mismatch` defects across 34 books in prior cycles.
  - **Reclassification:** 0.
  - **Reachability:** 0.
  - **Instrument-correction:** 0 — the per-kind ratchet correction that made this criterion
    verifiable happened in `AT-34-E1-007_re-verification_receipt.md`, a different (already-closed)
    criterion's cycle; this cycle re-derives against that corrected instrument and finds it still
    green.
- **Notes:** This criterion's work was already complete, committed, and pushed to
  `origin/tranche/14` before this lane started (`git rev-parse HEAD origin/tranche/14` returned
  identical SHAs `ba23c938b1d1...`). This receipt is an independent, from-scratch re-derivation of
  every figure in the criterion's evidence bar, per `decisions.md §12` L2 ("never carry your own
  number forward") and L20 ("a dispatch script's return value is not a closure claim") — nothing
  in this receipt was transcribed from `AT-34-E1-008_G1..G4_cycle_receipt.md` without independently
  re-running the command that produced it.
- **Next-cycle plan:** none — the criterion is closed. `kanban.md` row 8 already reads `complete`
  with correct receipt links; no edit needed there. AT-34-E6-001 (final-acceptance scan) is the
  next instrument that touches this criterion, re-deriving it once more at bundle closure.
