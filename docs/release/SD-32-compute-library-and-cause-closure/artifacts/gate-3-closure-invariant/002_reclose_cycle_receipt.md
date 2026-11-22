# Cycle 002-reclose — Gate 1 (`gate-1-shape-closure`) + Gate 3 (`gate-3-closure-invariant`) reclosure

Responds to `decisions.md §14` — both gates were reopened 2026-08-22 (verified twice: card 15's
Opus adversarial verifier, then the orchestrating session, both against the repo-local pinned
oracle `7f818006e371188e5717fd18d74d18a420747fc6`).

- **Card IDs:** `gate-1-shape-closure` (card 5), `gate-3-closure-invariant` (card 9)
- **Commit SHA(s):** see push step below (this receipt is written before commit; will be updated
  if the SHA differs after rebase)
- **Files touched:**
  - `scripts/shape_ledger.py` — `build_ledger()` now returns `join_status_counts`; printed output
    surfaces the matched/no_formula_tokens/no_record split (§14b)
  - `scripts/shape_coverage_standing_gate.py` — new `NO_RECORD_BUDGET_COUNT`/`NO_RECORD_BUDGET_POPULATION`
    constants; `run_gate()` split into `run_gate()` (I/O) + `evaluate_ledger()` (pure invariant
    check); gate now fails when `no_record`'s share of the population exceeds the committed
    baseline share; report/printed output carry the join-status split and budget fields
  - `scripts/tests/test_shape_coverage_standing_gate.py` — deleted the `mock.patch`-based
    `FabricatedUncoveredObjectTest` class entirely; replaced with `NoRecordBudgetInvariantTest`,
    which reaches a red gate through the real `run_gate`/`build_corpus_index`/`build_ledger`/
    `classify_unit` path (real synthetic units at an unreachable real corpus root — no mock,
    no monkeypatch of any function under test); the pile-mismatch regression test now calls the
    new pure `evaluate_ledger()` directly with a hand-built ledger dict (ordinary unit testing of
    an edge case, not a patch of `build_ledger`)
  - `scripts/tests/test_shape_ledger.py` — added a test asserting `join_status_counts` is present
    and reconciles to the population
  - `scripts/family_vocabulary_reconcile.py` — canonical `family-vocabulary.md`/`.json` now carry
    a "§0 Join-status split" section ahead of the family table
  - `scripts/verify.sh` — `run_shape_coverage_standing_gate()` now also parses and reports the
    `no_record` count (`SHAPE_COVERAGE_NO_RECORD` actual)
  - `docs/release/SD-32-compute-library-and-cause-closure/acceptance-and-verification.md` —
    AT-32-G1-001/002 amended, new AT-32-G1-004 (join-status split required on every quoted
    coverage figure); AT-32-G3-001/003 amended (no mock.patch red-proofs; `no_record` budget
    invariant named); Gate 1 and Gate 3 verification-command blocks both updated
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — cards 5 and 9 set
    `in-progress` → `complete` (see below)
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-1-shape-closure/family-vocabulary.{md,json}` — regenerated
  - `docs/retro/events/gates-1-3-reclose.jsonl` — rework event + derived verify.sh runs

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (both against `BASE_BRANCH=$(git merge-base HEAD origin/develop)` and against the pinned `PIN`)
- **Wired-integration audit result:** protocol-defined `BASE_BRANCH` diff (against `origin/develop`)
  flags "placeholder" twice — both hits are **pre-existing, unchanged** doctrine prose in
  AT-32-G1-002/AT-32-G3-002 ("a placeholder family/shape with zero units behind it cannot
  manufacture false coverage"), correct usage of the anti-gaming vocabulary, not a stub/mock
  introduced by this cycle. It shows as "added" only because the whole `docs/release/SD-32-...`
  tree is unmerged relative to `origin/develop` (every SD-32 cycle's dual-audit hits this same
  noise on this file). Isolated to this cycle's actual changes (`git diff --unified=0 2368cc4dd..HEAD`
  — the pinned base):
  `OK_NO_BUNDLE_TAGS` / `OK_NO_TOKENS` — clean.
- **Acceptance criterion:** AT-32-G1-001/002/003/004, AT-32-G3-001/002/003 (verbatim amendments in
  `acceptance-and-verification.md`, this commit)
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`)
- **Status:** complete
- **Notes / judgment calls:**
  1. **Population unchanged.** Re-derived: population 24,914, `join_status_counts` = matched 4,801
     / no_formula_tokens 9,694 / no_record 10,419 — exact match to `decisions.md §14b`'s figures,
     same corpus SHA. No `scripts/retro.py correction` needed (no drift found).
  2. **F0 was not deleted, renamed, or subsumed** (`decisions.md §14c` item 3) — it is unchanged;
     the join-status split is additive.
  3. **Gate 3's invariant is `no_record` share vs. a committed budget**, not `no_record == 0`
     outright — 10,419 cannot close to zero this cycle (book-onboarding backlog, card 11's own
     open T2b/T9 scope). The budget (`NO_RECORD_BUDGET_COUNT=10419`,
     `NO_RECORD_BUDGET_POPULATION=24914`) is set to exactly today's measured baseline, so the real
     full-population run still passes (`no_record` is AT the baseline, not below), while any run
     whose `no_record` share exceeds that baseline — including the orchestrator's 80-unit
     reproduction (100% no_record) — now fails. Integer cross-multiplication
     (`no_record_count * budget_population > budget_count * population`) avoids float rounding at
     the exact-baseline boundary. Future cycles that close real `no_record` units (card 11)
     tighten these two constants downward; nothing in the gate lets them rise.
  4. **`evaluate_ledger()` extracted as a pure function** from `run_gate()` so the sum-the-piles
     regression test could stop mocking `shape_ledger.build_ledger` too — it now calls
     `evaluate_ledger()` directly with a hand-built, deliberately-malformed ledger dict. This
     wasn't explicitly required by the brief (only the AT-32-G3-001 fabrication test was named),
     but the "never prove a gate red by patching the thing under test" rule reads as general, and
     leaving one mock-based test sitting next to the fixed one seemed like exactly the "preserves
     the false assurance" pattern the brief said to avoid.
- **Discovery forwards:** none.
- **Next-cycle plan:** card 11's T2b/T9 book-onboarding work (already open, `decisions.md §13`)
  is what will let a future cycle tighten `NO_RECORD_BUDGET_COUNT` downward as real `no_record`
  units get corpus records.

## Verification transcript

### Re-derive the join-status split (decisions.md §14b) — unchanged population/SHA
```
$ python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus --output /tmp/ledger.json
population (not-done units considered): 24914
unclassified: 0
join-status split (decisions.md §14b -- this is the honest coverage figure, not the family rollup below on its own):
  matched               4801  (19.3%)  -- rests on a real corpus record
  no_formula_tokens     9694  (38.9%)  -- record found, carries no DEFINE/BONUS
  no_record            10419  (41.8%)  -- join found no corpus record at all

family rollup:
  F0     20113  No formula content (no DEFINE/BONUS token found for this unit)
  F1      1790  Flat-constant magnitude (bare literal)
  F10        3  Level-threshold step-count (summed >= indicators)
  F2      1490  Per-level scaling (<Class>LVL bare or arithmetic)
  F3       303  Ability-modifier-derived (STR/DEX/CON/INT/WIS/CHA)
  F4       570  Named-counter/pool variable (plain identifier reference)
  F5       361  Clamped/capped per-level scaling (min/max/floor/ceil around a level expr)
  F6       211  classlevel(...)-derived
  F7         5  Conditional-step (if/boolean toggle)
  F8        41  Other named-variable expression (residual)
  F9        27  Skill-rank-derived (skillinfo/TOTALRANK)
```

### AT-32-G3-001 red-proof — orchestrator's own reproduction, BEFORE vs. AFTER

BEFORE (base `2368cc4dd`, unmodified `shape_coverage_standing_gate.py`):
```
(0, {'population': 80, 'unclassified_count': 0, 'piles_reconcile': True, 'families': {'F0': 80}})
```
`exit 0, PASS` — the defect `decisions.md §14a` records.

AFTER (this cycle, same reproduction command, unmodified):
```
$ python3 -c "
import sys; sys.path.insert(0,'scripts')
import shape_coverage_standing_gate as G
u=[{'id':f'b:{k}:{i}','kind':k,'book':'b','status':'not-started','wiring_class':'static','source_file':'totally_fake_file.lst','source_line':i} for k in ('ability','skill','template','deity','power','domain','language','kit') for i in range(1,11)]
print(G.run_gate({'units':u}, corpus_root='/nonexistent'))"
(1, {'population': 80, 'unclassified_count': 0, 'family_total': 80, 'piles_reconcile': True,
'families': {'F0': 80}, 'join_status_counts': {'no_record': 80}, 'no_record_count': 80,
'no_record_budget_count': 10419, 'no_record_budget_population': 24914,
'no_record_budget_exceeded': True, 'corpus_sha': '7f818006e371188e5717fd18d74d18a420747fc6'})
```
Exit status **1** (`FAIL`) — `no_record_budget_exceeded: True` — never patches `build_ledger` or
any other code under test; the 80 units run through the real `classify_unit`/`build_corpus_index`
join against a real (unreachable) `corpus_root`.

### Real full-population run — still passes (baseline share, not below it)
```
$ python3 scripts/shape_coverage_standing_gate.py --inventory docs/work-inventory.json --corpus-root data/corpus
population (not-done units considered): 24914
unclassified: 0
piles reconcile: True (24914 families-total == 24914 population)
join-status split (decisions.md §14b): matched=4801 no_formula_tokens=9694 no_record=10419
no_record budget: 10419/24914 vs. baseline 10419/24914 -- exceeded: False
corpus SHA: 7f818006e371188e5717fd18d74d18a420747fc6
```
exit 0.

### Closed-on-empty proofs (both gates)
```
$ echo '{}' | python3 scripts/shape_coverage_standing_gate.py 2>&1 | grep -q "no coverage" && echo GATE_G3_FAILS_CLOSED_ON_EMPTY_OK
GATE_G3_FAILS_CLOSED_ON_EMPTY_OK

$ python3 scripts/shape_ledger.py --inventory /dev/null 2>&1 | grep -q "no coverage" && echo GATE_G1_FAILS_CLOSED_ON_EMPTY_OK
GATE_G1_FAILS_CLOSED_ON_EMPTY_OK
```

### `scripts/verify.sh` stages
```
$ scripts/verify.sh --only shape-coverage-standing-gate
PASS  shape-coverage-standing-gate  (population=24914 unclassified=0 no_record=10419 corpus_sha=7f818006e371188e5717fd18d74d18a420747fc6)

$ scripts/verify.sh --only shape-coverage-standing-gate-selftest
PASS  shape-coverage-standing-gate-selftest  (12 cases passed)
```

### Self-tests (direct)
```
$ python3 -m unittest scripts.tests.test_shape_coverage_standing_gate -v
Ran 12 tests in 0.265s — OK

$ python3 -m unittest scripts.tests.test_shape_ledger -v
Ran 29 tests in 0.382s — OK

$ python3 -m unittest scripts.tests.test_family_vocabulary_reconcile -v
Ran 8 tests in 0.009s — OK
```

### Broad suites
```
$ cargo test --locked --lib
test result: ok. 2388 passed; 0 failed; 13 ignored; 0 measured; 0 filtered out

$ cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml
test result: ok. 516 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Dual-audit gate
```
BASE_BRANCH=$(git merge-base HEAD origin/develop)   # = 1bb523773d32705d1b7387fd4c494861523f55ba
# whole docs/release/SD-32-.../*.md tree reads as "added" against develop (bundle unmerged) —
# grep hits pre-existing "placeholder family/shape" doctrine prose, not new stub/mock code
OK_NO_BUNDLE_TAGS
# (STUB/MOCK/todo/fixme/hack): clean; "placeholder" hits are pre-existing AT-32-G1-002/G3-002 text

# Isolated to this cycle's own changes (git diff --unified=0 2368cc4dd..HEAD, the pinned base):
OK_NO_BUNDLE_TAGS
OK_NO_TOKENS
```
