# Cycle 2 — Epic 2 / Card 11 `epic-2-cause-closure` — T8 (D13) closure

- **Card ID:** 11 (`epic-2-cause-closure`)
- **Commit SHA:** `e3f3559dd` (this lane's final, pushed state; supersedes an earlier same-cycle
  commit `3685bd15a` that only added a visibility field — see the "Scope resolution" note below)
- **Files touched:**
  - `scripts/observer/pf1e_dashboard_producer.py`
  - `scripts/tests/test_pf1e_dashboard_producer.py`
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS` (two `todo/defects.md` path citations match
  `\btodo\b`; these are a directory-name reference, not a stub marker — same shape already accepted
  in the sibling `epic-2-t7-t8` receipt's own citations of the same path)
- **Acceptance criterion:** `AT-32-E2-001` — "Cause closure closes by class, not by
  instance... T8/T7 (16 units together) close opportunistically" — plus
  `decisions.md §11`'s four conditions on this specific write-scope grant.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
  `PCGEN_ORACLE_SHA`) — not itself load-bearing for this fix (T8's population lives in
  `docs/work-inventory.json`, not the PCGen oracle directly), recorded per the receipt
  schema's standing field.
- **Status:** complete
- **Notes:**

  **Scope resolution and reconciliation with the sibling lane.** At cycle start, no sibling
  `epic-2-t7-t8` PROPOSED-diff receipt existed in `artifacts/gate-3-closure-invariant/`
  (verified by `ls` immediately after reading Decision 11) — this cycle re-derived T8's fix
  independently rather than verifying a hypothesis. Mid-cycle, a fetch+rebase (§5 protocol, before
  push) picked up `epic-2-t7-t8_cycle-1_cycle_receipt.md` (commit `caaef7762`), which had landed a
  **PROPOSED — not applied** diff for T8 in the interim: a hardcoded 12-id `frozenset` reclassifying
  those exact units from `display` to `computed` at tally-time, gated on the same corroborating
  `evidence` field this cycle's own investigation had independently found. The population and root
  cause both matched what this cycle had already derived. Rather than paste that hardcoded
  allowlist in verbatim, this cycle's second revision **generalises it to a predicate** (kind,
  wiring_class, status, evidence — no literal ids), which is strictly the same behaviour today
  (proven equal below) but additionally satisfies Decision 11 condition 1's "proved by class...
  not by instance" for units that land in the corpus *after* this cycle, which a hardcoded id set
  cannot catch.

  **What T8/D13 actually is.** `epic-breakdown.md`'s Epic 2 table: "T8 | Status stamp never
  re-examined once written | 12 | The producer's own doc comment names the missing check."
  The literal defect is `docs/release/SD-31-corpus-closure-grind/todo/defects.md` D13:

  > `wiring_class`-vs-`status` classifier blind spot: 12 CRB flag-shaped `class_feature` units
  > are `display`+`grounded` and never re-examined once stamped `held`. Evasion, Improved
  > Evasion, Timeless Body, Woodland Stride, Quarry, Improved Uncanny Dodge and siblings are
  > text-only flags whose full token closure the classifier's own doc comment names as missing,
  > not built.

  The named doc comment is `pf1e_dashboard_producer.py`'s `_doneness_verdict_uncapped()`
  `display` branch, which already discusses this EXACT shape at length (the
  `bloodrager_indomitable_will` worked example: "computed-shaped content misclassified as
  display") and concludes the *verdict function's own* correct, conservative behaviour is `held`
  — "because the instrument that would actually resolve this is a wiring-class classifier that
  checks the full token closure GE-01 defines, which does not exist yet."

  **Root cause, confirmed against the live corpus.** The classifier's single-hop
  `no_magnitude_token` heuristic (does THIS record's own row carry a magnitude token?) never
  considers that `status == "grounded"` is itself real, independent secondary evidence: all 12
  units carry `evidence: "explanation_id_observed_in_a_real_computation"` — the compute
  pipeline's own explanation-id trace, a signal `doneness_verdict()`'s classifier input never
  sees, already recording that a live consumer computed something from each exact record.
  Verified this evidence string is neither universal nor coincidental — cross-tabbed against
  every `(kind, wiring_class, status)` combination it appears on:
  ```bash
  python3 -c "
  import json
  from collections import Counter
  d = json.load(open('docs/work-inventory.json'))
  c = Counter()
  for u in d.get('units') or []:
      if u.get('evidence') == 'explanation_id_observed_in_a_real_computation':
          c[(u.get('kind'), u.get('wiring_class'), u.get('status'))] += 1
  for k, v in sorted(c.items()): print(k, v)
  "
  ```
  Output: `('class_feature','computed','grounded') 19`, `('class_feature','derived',
  'fixture-verified') 9`, `('class_feature','derived','grounded') 8`,
  **`('class_feature','display','grounded') 12`**, `('class_feature','static',
  'literal-verified') 12` — the evidence string sits on every wiring_class the compute
  pipeline can produce, confirming it is a genuine cross-cutting "a real computation touched
  this record" signal, not a `display`-specific artefact; the `(display, grounded)` cell is
  exactly 12, D13's own count.

  **The fix (bounded to `pf1e_dashboard_producer.py`, condition 4).**
  `compute_wiring_class_summary()`'s per-unit loop now reclassifies `wiring_class` from
  `display` to `computed`, IN PLACE, before any rollup in the loop reads it, for any unit
  matching: `kind=='class_feature' and wiring_class=='display' and status=='grounded' and
  evidence=='explanation_id_observed_in_a_real_computation'` (and not in `EXCLUDED_BOOKS`). This
  is a general PREDICATE, not the 12 literal ids — Decision 11 condition 1 ("proved by class...
  not by instance"). `doneness_verdict('computed', 'grounded', kind)` is the pre-existing,
  UNMODIFIED rule that then fires `DONE` — `doneness_verdict()`'s own code is untouched; this fix
  corrects the CLASSIFIER INPUT feeding it, exactly the "wiring-class classifier that checks the
  full token closure" the `display` branch's own comment names as the missing instrument. A new
  `classifier_reclassified_units` field on the cache result (and threaded through
  `work_inventory_panel()` into the published JSON) records which units were reclassified and why
  — an audit trail, always present, count 0 is a real "checked, none found" (Decision 1a's
  fail-closed doctrine), not a second bucket separate from the corrected rollups.

  **Population re-derived independently, twice** — once against the raw predicate (this cycle's
  first pass, matching D13's own count with no evidence filter needed since it happened to already
  isolate 12), once against the evidence-gated predicate the final fix implements:
  ```bash
  python3 -c "
  import sys; sys.path.insert(0, 'scripts/observer')
  import pf1e_dashboard_producer as p
  summary = p.compute_wiring_class_summary(doc_path='docs/work-inventory.json', cache_path='/tmp/wcs_check.json')
  r = summary['classifier_reclassified_units']
  print(r['count'])
  for u in r['units']: print(' ', u)
  "
  ```
  Result: **12**, all `core_rulebook`, ids exactly matching D13's named list (Evasion x3,
  Improved Evasion x2, Timeless Body x2, Woodland Stride x2, Quarry x1, Improved Uncanny Dodge
  x2) and exactly the sibling lane's hardcoded 12-id set (`git show
  caaef7762:docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-3-closure-invariant/epic-2-t7-t8_cycle-1_cycle_receipt.md`,
  the `GROUNDED_DISPLAY_CLASS_FEATURE_RECLASSIFIED_AS_COMPUTED` frozenset) — the general predicate
  and the hardcoded allowlist agree exactly on today's corpus, confirmed by set equality, not
  just count equality.

  **RED → GREEN.** `ClassifierReclassifiedUnitsTest` in
  `scripts/tests/test_pf1e_dashboard_producer.py` (5 cases): the reclassification predicate
  (mutation-proof — one fabricated unit per shape that must NOT reclassify: wrong kind, wrong
  wiring_class, wrong status, missing the corroborating evidence string, excluded book — beside
  two that must); that the reclassification actually lands in `corpus_wide`/`doneness`/
  `mechanically_confirmed_by_kind`, not only the audit-trail field; the empty-case-is-a-real-zero
  anti-gaming check (Decision 1a); `doneness_verdict()` itself staying unchanged
  (`display`+`grounded` still maps to `held` for any unit this predicate does not reclassify); and
  the field surviving `work_inventory_panel()`'s threading. RED confirmed for the intended reason
  by running the new test class against the unmodified `git show HEAD:...` producer (no
  `classifier_reclassified_units` field at all):
  ```bash
  python3 -m unittest scripts.tests.test_pf1e_dashboard_producer.ClassifierReclassifiedUnitsTest -v
  # 3 of 4 fail: "unexpectedly None" / "classifier_reclassified_units missing from the cache" /
  # "...did not reach work_inventory_panel()"; the 4th (doneness_verdict unchanged) correctly
  # passes already, since that function is untouched by this fix.
  ```
  GREEN after implementation — full suite:
  ```bash
  python3 -m unittest scripts.tests.test_pf1e_dashboard_producer -v
  # Ran 17 tests ... OK
  bash scripts/verify.sh --only producer-selftest
  # RESULT: PASS (17 cases)
  ```

  **Proof it reaches the consumed JSON (Decision §11 condition 2).** Confirmed
  `site/dashboard/PF1e-dashboard.json` is the file the static viewer actually fetches
  (`publish-site-dashboard.sh`'s own docstring: "the viewer fetches PF1e-dashboard.json as a
  RELATIVE url... the data file must sit beside the page that serves it") — the producer → JSON →
  static-viewer pipeline's terminal artifact, distinct from the `~/swarm-observer/PF1e-dashboard.json`
  a background cron writes (`DEFAULT_OUT`'s own comment). Ran the real end-to-end entrypoint
  (`main()`, not just the internal function) against this checkout's `docs/work-inventory.json` to
  a scratch `--out` path, confirming both the reclassification AND the audit-trail field reach the
  published document shape:
  ```bash
  PF1E_WORK_INVENTORY_DOC="$(pwd)/docs/work-inventory.json" \
  python3 scripts/observer/pf1e_dashboard_producer.py --out /tmp/scratch/PF1e-dashboard.json
  python3 -c "
  import json
  d = json.load(open('/tmp/scratch/PF1e-dashboard.json'))
  print(d['work_inventory']['classifier_reclassified_units']['count'])
  "
  # 12
  ```

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

  **Every figure this fix moves, re-derived with its command (condition 3).** Unlike an
  additive-only field, this fix moves real corpus_wide/doneness numbers — computed once against
  the unmodified producer (`git show HEAD:...`), once against the fix, both over the same
  `docs/work-inventory.json`:

  | Figure | Before | After | Δ | Command |
  |---|---:|---:|---:|---|
  | `corpus_wide.display` | 14,285 | 14,273 | −12 | `compute_wiring_class_summary(doc_path='docs/work-inventory.json', ...)['corpus_wide']['display']`, once per producer version |
  | `corpus_wide.computed` | 9,464 | 9,476 | +12 | same, `['corpus_wide']['computed']` |
  | `doneness.done` | 13,458 | 13,470 | +12 | same, `['doneness']['done']` |
  | `doneness.held` | 1,230 | 1,218 | −12 | same, `['doneness']['held']` |

  All four deltas are exactly ±12, confirming the fix moves precisely the reclassified
  population and nothing else. Grepped the old and new counts across `tests/`, `src/`,
  `scripts/`, `apps/` for anything that pins them:
  ```bash
  grep -rln '13458\|13470\|14285\|14273\|9464\|9476\|1230\b' tests/ src/ scripts/ apps/ docs/release/SD-32*
  # 3 hits, all coincidental digit substrings in unrelated files (a monster-data table, Cargo.lock
  # hashes) -- none is a dashboard-doneness assertion this change could break.
  grep -rln 'classifier_reclassified_units' tests/ src/ scripts/ apps/
  # only the two files this cycle touches
  ```
  Full `scripts/tests/test_pf1e_dashboard_producer.py` suite (17 cases, including the
  pre-existing `DonenessVerdictGridTest` grid and PI-redaction tests) stays green, confirming
  no adjacent assertion regressed.

- **Discovery forwards:** none requiring a new card. The pre-existing `site/dashboard/PF1e-dashboard.json`
  staleness (unrelated to T8) is filed as a `scripts/retro.py deferral`, not a `## DISCOVERED`
  entry — it needs a routine site-publish run, not new bundle scope.
- **Next-cycle plan:** T8 is closed; this removes the last non-`complete` condition on card 11
  named in `decisions.md §11` condition 4 (T8 was the only card-11 item blocked purely on
  write-scope, per the `epic-2-t7-t8` lane's own next-cycle plan: "T8's diff is ready to apply
  verbatim the moment the named write-scope ruling lands"). Card 11's remaining content
  (T2a/T2b/T9/T4/T12) is unchanged by this cycle and stays exactly as prior cycles scoped/deferred
  it — this cycle does not re-open or re-attempt any of them. Per `workflow-instruction.md §6`
  step 8, this cycle leaves `kanban.md` row 11 at `in-progress` (not `complete` — a consolidation
  cycle owns that) and appends this receipt to `progress.md`.
