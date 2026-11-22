# Cycle 3 — Epic 2 / Card 11 `epic-2-cause-closure` — T8 warm-cache-invalidation fix

- **Card ID:** 11 (`epic-2-cause-closure`)
- **Commit SHA:** (recorded after push — see report)
- **Files touched:**
  - `scripts/observer/pf1e_dashboard_producer.py`
  - `scripts/tests/test_pf1e_dashboard_producer.py`
  - `docs/retro/events/epic-2-t8-cache-fix.jsonl` (new, `scripts/retro.py correction`)
  - `docs/retro/events/sd31-transcribe.jsonl` (auto-appended by `verify.sh`'s own
    transcribe hook when `producer-selftest` was run this cycle — not a manual edit)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS` — the only `\btodo\b` hits in the
  scoped diff are two pre-existing `docs/release/SD-31-corpus-closure-grind/todo/defects.md`
  path citations carried in from cycle 2's own T8 comment block (unified diff includes
  surrounding context since `--unified=0` still shows the hunk header's context for a
  block this cycle's own new comment sits inside); same accepted directory-name-reference
  shape the cycle-2 receipt already logged for the identical citation.
- **Acceptance criterion:** `decisions.md §11` conditions 1–4, re-verified against the
  cycle-2 (`e3f3559dd`) T8 fix, which satisfied conditions 1/3/4 but had NOT actually
  satisfied condition 2 in the one place that matters — the real, default, warm
  `WIRING_CLASS_CACHE` path — because cycle 2's own condition-2 proof ran `main()`
  against a scratch `--out` path with a cold cache, never exercising the warm-cache
  short-circuit branch at all.

## Reproduction (before this cycle's fix)

```bash
python3 -c "
import sys,os,json; sys.path.insert(0,'scripts/observer')
import pf1e_dashboard_producer as p
c=p.WIRING_CLASS_CACHE
print('cache:', c, 'exists:', os.path.exists(c))
if os.path.exists(c):
    d=json.load(open(c)); print('cached schema:', d.get('schema'), '| has field:', 'classifier_reclassified_units' in d)
print('schema const:', p.WIRING_SUMMARY_SCHEMA)
s=p.compute_wiring_class_summary()
print('has field:', 'classifier_reclassified_units' in s)
print('computed:', s['corpus_wide'].get('computed'), 'display:', s['corpus_wide'].get('display'))
"
```

Output (run against the unmodified tip, before this cycle touched anything):

```
cache: /home/ubuntu/swarm-observer/wiring-class-summary.json exists: True
cached schema: 12 | has field: False
schema const: 12
has field: False
computed: 9464 display: 14285
```

Confirms the defect exactly as reported: the cache carries pre-T8 schema `12`, the T8
field is absent, and the served `corpus_wide` figures are the pre-fix values, on a warm
(pre-existing) cache at the real default path.

## Root cause

`compute_wiring_class_summary()` added `classifier_reclassified_units` to its return
dict (cycle 2, commit `e3f3559dd`) but the warm-cache validity check
(`cached.get("schema") == WIRING_SUMMARY_SCHEMA`) still compared against `12` — the
constant was never bumped. A cache written by the pre-fix producer therefore has
`schema: 12`, is newer than `docs/work-inventory.json`, and passes the equality check
unchanged, so the reclassification loop never runs against it.

## The fix

`WIRING_SUMMARY_SCHEMA` bumped `12 -> 13` at
`scripts/observer/pf1e_dashboard_producer.py:3565` (now with a comment naming this
exact incident, alongside the pre-existing 11->12 comment documenting the prior
occurrence of the identical hazard shape). This is the minimal correct fix — the
constant exists for exactly this, per the task brief and the pre-existing 11->12
precedent in the same file.

## RED → GREEN

New test `StaleSchemaCacheIsRejectedTest` (`scripts/tests/test_pf1e_dashboard_producer.py`):
writes a cache shaped exactly like a pre-T8 producer would have written it — `schema: 12`
(hardcoded in the test as the historical pre-fix value, independent of the live
constant, so the test keeps proving the historical defect even after the constant
moves again), `available: True`, `source_document` matching the doc under test (so the
pre-existing P0.2 `source_document` guard is not itself what causes rejection — this
test isolates the schema/field gap specifically), no `classifier_reclassified_units`
field, and pre-reclassification `corpus_wide`/`doneness` counts — given a newer mtime
than the source doc, then asserts the cache is rejected: the reclassification fires,
`classifier_reclassified_units` is present, and `corpus_wide`/`doneness` reflect the
corrected tally.

This differs in shape from the pre-existing `ClassifierReclassifiedUnitsTest`
(cycle 2): every test in that class calls `compute_wiring_class_summary()` against a
`cache_path` that has never been written (cold cache every time), so it never exercises
the cache-HIT branch at all — it is structurally incapable of catching a cache
invalidation defect, which is exactly why it passed while the defect was live.

RED confirmed, un-bumped constant (`WIRING_SUMMARY_SCHEMA = 12`):
```bash
python3 -m unittest scripts.tests.test_pf1e_dashboard_producer.StaleSchemaCacheIsRejectedTest -v
```
```
FAIL: test_pre_t8_schema_cache_is_rejected_and_reclassification_fires
AssertionError: unexpectedly None : classifier_reclassified_units missing -- a pre-T8-schema
warm cache was served instead of being recomputed
```

GREEN after the one-line bump to `13`:
```bash
python3 -m unittest scripts.tests.test_pf1e_dashboard_producer -v
# Ran 19 tests ... OK
bash scripts/verify.sh --only producer-selftest
# PASS  producer-selftest  (19 cases passed)
```

## Generalizing the hazard (condition-3-equivalent judgment call, brief item 3)

Added `WiringSummaryTopLevelKeysCanaryTest`: pins the exact top-level key set
`compute_wiring_class_summary()` returns for a real (non-degenerate) summary. Any key
added, removed, or renamed on the `result` dict fails this test immediately, forcing a
conscious decision about whether `WIRING_SUMMARY_SCHEMA` needs to move — this is the
second recorded instance of this exact hazard shape (the file's own 11->12 comment
documents the first), so a test that makes the *next* one loud is warranted.

**Judgment call, stated explicitly:** this is a CI trip-wire, not a fully mechanical
derivation. I considered deriving `WIRING_SUMMARY_SCHEMA` automatically from a hash of
the result dict's key set (so a human literally cannot forget), but rejected it: the
warm-cache-hit branch's entire purpose is to avoid rebuilding `result`, so there is no
fresh key set available to hash against at validation time without defeating the cache.
The only way to make the *validation* itself self-deriving would be a second
hand-maintained parallel structure (e.g. a hardcoded expected-key frozenset consulted at
runtime) — which is the same "a human must remember to update this" hazard in a
different shape, not a removal of it. The canary test is a real, CI-enforced mechanism
(it fails the build, not just a warning), but it is enforced at test time, not
prevented at the type level. I judge this in scope and satisfies the file-shape
generalization the brief asked to "consider"; a stronger runtime-mechanical version was
considered and explicitly not built, for the reason above.

## Real before/after figures, real default warm cache path (brief item 4)

Command (run once against the pre-fix producer at tip, once against the fix — see
retro correction below for the pre-fix run's actual output):

```bash
python3 -c "
import sys,os,json; sys.path.insert(0,'scripts/observer')
import pf1e_dashboard_producer as p
s=p.compute_wiring_class_summary()
print(s['corpus_wide']['computed'], s['corpus_wide']['display'],
      s['doneness']['done'], s['doneness']['held'])
"
```

| Figure | Before (pre-fix, warm cache, reproduced this cycle) | After (post-fix, warm cache, this cycle) | Δ |
|---|---:|---:|---:|
| `corpus_wide.display` | 14,285 | 14,273 | −12 |
| `corpus_wide.computed` | 9,464 | 9,476 | +12 |
| `doneness.done` | 13,458 | 13,470 | +12 |
| `doneness.held` | 1,230 | 1,218 | −12 |

**These match the cycle-2 receipt's claimed figures exactly.** The T8 fix's own
computation was correct; it simply never reached the dashboard because of the schema
gap. No figure-correction retro entry was needed (brief item 4's "if the real numbers
differ, that is a finding" did not fire — they do not differ). A different correction
was logged instead: `docs/retro/events/epic-2-t8-cache-fix.jsonl`
(`1787447916117-epic-2-t8-cache-fix-d7261c`), correcting the cycle-2 receipt's condition-2
claim ("proof it reaches the consumed JSON") — the proof ran against a scratch `--out`
path with a cold cache, not the real warm cache, so it did not actually establish what
it claimed for the live pipeline.

## Pinned-count sweep (brief item 5)

```bash
grep -rln '13458\|13470\|14285\|14273\|9464\|9476\|1230\|1218' tests/ src/ scripts/ apps/
```
5 hits: `src/rules_core/rules_tables/{bestiary,bestiary_2,bestiary_4}/monster_data.rs`
(`source_line:` fields / `.lst` comment line references) and
`apps/desktop/src-tauri/Cargo.lock` — all coincidental digit substrings, none a
dashboard-doneness assertion. Also checked `docs/release/SD-32-compute-library-and-cause-closure/`:
3 hits — `gate-0-census-closure/diff.json` (`"monster": 1218`, unrelated census count),
`artifacts/.../epic-2-cause-closure_cycle-2_t8_cycle_receipt.md` and `progress.md`
(both correctly quoting the cycle-2 receipt's figures, which this cycle confirms are
accurate). Nothing needed a fix.

## Dashboard staleness (brief item 6 — not touched)

`site/dashboard/PF1e-dashboard.json` was NOT regenerated this cycle, per the brief's
explicit instruction. This cycle's schema bump makes the *next* regeneration of that
file pick up the reclassification (which it previously would not have, warm-cache or
not, since the committed producer now emits schema 13) — it does not itself change
`site/dashboard/PF1e-dashboard.json`'s content or staleness state. The file was already
`STALE` per cycle 2's own `publish-site-dashboard.sh --check` finding (unrelated corpus
drift, `proven_units` 9047→9057 and book-status changes); this cycle neither worsens nor
resolves that pre-existing staleness.

- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`PCGEN_ORACLE_SHA`) — not
  load-bearing for this fix (same as cycle 2; T8's population lives in
  `docs/work-inventory.json`), recorded per the receipt schema's standing field.
- **Status:** complete
- **Discovery forwards:** none.
- **Next-cycle plan:** this closes the last remaining gap on T8 under `decisions.md §11`
  condition 2 (proved against the REAL warm cache, not a scratch path). Card 11's other
  content (T2a/T2b/T9/T4/T12) is unchanged and untouched by this cycle. Per
  `workflow-instruction.md §6` step 8, `kanban.md` row 11 stays `in-progress` — a
  consolidation cycle owns marking it `complete`.
