# Cycle 2 — Epic 2 / Card 11 `epic-2-cause-closure` — T8 (D13) closure

- **Card ID:** 11 (`epic-2-cause-closure`)
- **Commit SHA:** (recorded after commit, see push step)
- **Files touched:**
  - `scripts/observer/pf1e_dashboard_producer.py`
  - `scripts/tests/test_pf1e_dashboard_producer.py`
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion:** `AT-32-E2-001` — "Cause closure closes by class, not by
  instance... T8/T7 (16 units together) close opportunistically" — plus
  `decisions.md §11`'s four conditions on this specific write-scope grant.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
  `PCGEN_ORACLE_SHA`) — not itself load-bearing for this fix (T8's population lives in
  `docs/work-inventory.json`, not the PCGen oracle directly), recorded per the receipt
  schema's standing field.
- **Status:** complete
- **Notes:**

  **Scope resolution.** No sibling `epic-2-t7-t8` PROPOSED-diff receipt existed in
  `artifacts/gate-3-closure-invariant/` at cycle start (`ls` of that directory showed only
  `epic-2-cause-closure_cycle-1_cycle_receipt.md`, `001_cycle_receipt.md`, and one `.run.json`);
  this cycle re-derived T8's fix independently rather than verifying a hypothesis.

  **What T8/D13 actually is.** `epic-breakdown.md`'s Epic 2 table: "T8 | Status stamp never
  re-examined once written | 12 | The producer's own doc comment names the missing check."
  The literal defect is `docs/release/SD-31-corpus-closure-grind/todo/defects.md` D13:

  > `wiring_class`-vs-`status` classifier blind spot: 12 CRB flag-shaped `class_feature` units
  > are `display`+`grounded` and never re-examined once stamped `held`. Evasion, Improved
  > Evasion, Timeless Body, Woodland Stride, Quarry, Improved Uncanny Dodge and siblings are
  > text-only flags whose full token closure the classifier's own doc comment names as missing,
  > not built.

  The named doc comment is `pf1e_dashboard_producer.py`'s `_doneness_verdict_uncapped()`
  `display` branch (line ~3953 pre-fix), which already discusses this EXACT shape at length
  (the `bloodrager_indomitable_will` worked example: "computed-shaped content misclassified as
  display") and already concludes the correct, conservative verdict is `held` — NOT `done`,
  NOT `in-progress` — "because the instrument that would actually resolve this is a wiring-class
  classifier that checks the full token closure GE-01 defines, which does not exist yet."

  **Investigation finding, stated explicitly:** `doneness_verdict('display', 'grounded',
  'class_feature')` already returns `held` today, pre-fix — verified directly:
  ```bash
  python3 -c "
  import sys; sys.path.insert(0, 'scripts/observer')
  import pf1e_dashboard_producer as p
  print(p.doneness_verdict('display', 'grounded', 'class_feature'))
  "
  # -> held
  ```
  D13's real defect is therefore NOT a wrong verdict (the verdict is already correct and
  deliberately conservative). It is that this population had **no named, standing home
  anywhere the dashboard publishes** — it sat inside the generic `held` bucket
  indistinguishable from every other `held` unit, so nothing flagged it for the classifier
  work that would resolve it, and it went "never re-examined once stamped held" (D13's own
  title). The fix closes that: a new, generically-derived (not hardcoded-by-id) field names
  and counts this exact population every producer run.

  **Population re-derived independently** (kind=='class_feature' AND wiring_class=='display'
  AND status=='grounded', `EXCLUDED_BOOKS` = `{beginner_box}` dropped):
  ```bash
  python3 -c "
  import json
  d = json.load(open('docs/work-inventory.json'))
  units = d.get('units') or []
  c = 0; names = []
  for u in units:
      if (u.get('kind') == 'class_feature' and (u.get('wiring_class') or 'ambiguous') == 'display'
              and (u.get('status') or 'unknown') == 'grounded'):
          c += 1; names.append((u.get('book'), u.get('name'), u.get('id')))
  print(c)
  for n in names: print(n)
  "
  ```
  Result: **12**, all `core_rulebook`, names exactly matching D13's list (Evasion x3,
  Improved Evasion x2, Timeless Body x2, Woodland Stride x2, Quarry x1, Improved Uncanny
  Dodge x2). Matches D13's stated population exactly — confirms the defect row, not a
  contradiction of it.

  **RED → GREEN.** Added `ClassifierReexaminationQueueTest` to
  `scripts/tests/test_pf1e_dashboard_producer.py` (3 cases: correct filtering by kind +
  wiring_class + status + excluded-book, the empty-case-is-a-real-zero anti-gaming check per
  Decision 1a, and the field surviving `work_inventory_panel()`'s threading). RED confirmed
  for the intended reason (`AssertionError: unexpectedly None` — the field did not exist yet):
  ```
  python3 -m unittest scripts.tests.test_pf1e_dashboard_producer.ClassifierReexaminationQueueTest -v
  # 3 failures, all "classifier_reexamination_queue missing from the cache" /
  # "did not reach work_inventory_panel()"
  ```
  GREEN after implementation — full suite:
  ```bash
  python3 -m unittest scripts.tests.test_pf1e_dashboard_producer -v
  # Ran 16 tests ... OK
  bash scripts/verify.sh --only producer-selftest
  # RESULT: PASS (16 cases)
  ```

  **The fix (bounded to `pf1e_dashboard_producer.py`, condition 4).**
  `compute_wiring_class_summary()`'s per-unit loop now also collects unit ids matching the
  predicate above into a new `classifier_reexamination_queue` field on its result dict
  (`{"predicate": ..., "count": N, "units": [...]}`), always present (count 0 is a real
  "checked, none found", never an absent field — Decision 1a's fail-closed doctrine).
  `work_inventory_panel()` threads it straight through (no further book-exclusion needed —
  the loop already drops `EXCLUDED_BOOKS` inline), with an explicit zero-count fallback shape
  for an older, pre-this-field cache. `doneness_verdict()` itself is UNCHANGED — the `held`
  verdict for this cell was already correct; this cycle does not touch it.

  **Proof it reaches the consumed JSON (Decision §11 condition 2).** Confirmed
  `site/dashboard/PF1e-dashboard.json` is the file the static viewer actually fetches
  (`publish-site-dashboard.sh`'s own docstring: "the viewer fetches PF1e-dashboard.json as a
  RELATIVE url... the data file must sit beside the page that serves it") — this is the
  producer → JSON → static-viewer pipeline's terminal artifact, distinct from the
  `~/swarm-observer/PF1e-dashboard.json` a background cron writes (`DEFAULT_OUT`'s own
  comment). Ran the real end-to-end entrypoint (`main()`, not just the internal function)
  against this checkout's `docs/work-inventory.json` to a scratch `--out` path:
  ```bash
  PF1E_WORK_INVENTORY_DOC="$(pwd)/docs/work-inventory.json" \
  python3 scripts/observer/pf1e_dashboard_producer.py --out /tmp/scratch/PF1e-dashboard.json
  python3 -c "
  import json
  d = json.load(open('/tmp/scratch/PF1e-dashboard.json'))
  print(d['work_inventory']['classifier_reexamination_queue'])
  "
  # {'predicate': \"kind=='class_feature' and wiring_class=='display' and status=='grounded', EXCLUDED_BOOKS dropped\", 'count': 12, 'units': [...12 ids...]}
  ```
  Confirms the field reaches `work_inventory.classifier_reexamination_queue` in the exact
  document shape `site/dashboard/PF1e-dashboard.json` uses (same top-level key,
  `["generated_at", ..., "work_inventory", ...]`, verified against the currently-committed
  file's own key list).

  **Committed `site/dashboard/PF1e-dashboard.json` was NOT regenerated this cycle — logged as
  a deferral, not silently skipped.** `bash scripts/publish-site-dashboard.sh --check`, run
  against the UNMODIFIED (pre-fix) `git show HEAD:scripts/observer/pf1e_dashboard_producer.py`,
  already reports `STALE` before this cycle touched anything — the committed copy has drifted
  from the current `docs/work-inventory.json` for reasons unrelated to T8 (`proven_units`
  9047→9057 and several book-status/reconciliation changes appear identically with the
  original producer). Regenerating and committing the full file here would (a) exceed
  `decisions.md §11` condition 4's bounded T8 scope by folding in unrelated corpus drift, and
  (b) risk a large, noisy collision with the six sibling lanes pushing to this same branch.
  Logged: `scripts/retro.py deferral` (actor `epic-2-t8`, `docs/retro/events/epic-2-t8.jsonl`),
  naming the next scheduled site publish as the revisit condition.

  **Figures re-derived, greps run (condition 3).** This fix is purely additive — no existing
  count (`corpus_wide`, `by_status`, `doneness`, `by_kind`, etc.) changes value; only a new
  field is added. Confirmed no other file pins an expectation this could break:
  ```bash
  grep -rln 'classifier_reexamination_queue' tests/ src/ scripts/ apps/   # only the two files this cycle touches
  grep -rn 'wiring_class.*status.*blind\|blind spot' scripts/ src/ apps/ tests/ --include=*.py --include=*.rs
  # no hit references the 12-unit T8 population or asserts a count this change moves
  ```
  Full `scripts/tests/test_pf1e_dashboard_producer.py` suite (16 cases, including the
  pre-existing `DonenessVerdictGridTest` grid and PI-redaction tests) stays green, confirming
  no adjacent assertion regressed.

- **Discovery forwards:** none requiring a new card. The pre-existing `site/dashboard/PF1e-dashboard.json`
  staleness (unrelated to T8) is filed as a `scripts/retro.py deferral`, not a `## DISCOVERED`
  entry — it needs a routine site-publish run, not new bundle scope.
- **Next-cycle plan:** T8 is closed; this removes the last non-`complete` condition on card 11
  named in `decisions.md §11` condition 4 (T8 was the only card-11 item blocked purely on
  write-scope). Card 11's remaining content (T2a/T2b/T9/T4/T12/T7) is unchanged by this cycle
  and stays exactly as cycle 1 scoped/deferred it — this cycle does not re-open or re-attempt
  any of them. Per `workflow-instruction.md §6` step 8, this cycle leaves `kanban.md` row 11 at
  `in-progress` (not `complete` — a consolidation cycle owns that) and appends this receipt to
  `progress.md`.
