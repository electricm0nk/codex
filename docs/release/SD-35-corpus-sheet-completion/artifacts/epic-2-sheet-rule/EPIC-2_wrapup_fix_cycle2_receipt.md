# Cycle EPIC-2-WRAPUP-FIX-2 — Epic 2, Sheet rule / wrap-up correction cycle, round 2

The second correction cycle `workflow-instruction.md §10` step 0 requires: the Epic 2 wrap-up
gate was re-run by an isolated read-only worker and came back **RED again** on one stage. That
worker committed and pushed nothing (`decisions.md §3`'s worker split); its report
`EPIC-2_wrapup_regate2_report.md`, the earlier `EPIC-2_wrapup_regate_report.md`, and three retro
shards are committed by **this** cycle. This cycle runs LOCAL on the shared checkout and does
commit and push.

**The headline of this cycle is a correction to the gate worker's own root cause.** The report
named the wrong mechanism, and the control it asked for would not have observed the failure. §2
below is the disproof; §3 is the mechanism that actually did it.

- **Commit SHA:** `265cd65fa5` (the producer content guard, its three tests, the republished
  feed, the two denominator fixes in the copied report, and the gate worker's hand-off), and
  `<receipt-sha>` carrying this receipt with `kanban.md` / `progress.md`. Cycle start
  `f1f547a41e`.
- **Scope gate:** `SCOPE_GATE: EXEMPT (wrap-up correction cycle)` — `decisions.md §2`'s named
  exemption; a wrap-up fix cycle closes zero units by design. **Not** exempt from the residue
  check, which was run at start and at end and did not move.
- **Files touched:**
  - `scripts/observer/pf1e_dashboard_producer.py` — the content guard on
    `compute_wiring_class_summary()`'s warm cache, `source_document_sha256`, schema 13 → 14
  - `scripts/tests/test_pf1e_dashboard_producer.py` — `ForeignContentCacheIsRejectedTest`
    (3 cases, RED first), and the top-level-key canary updated for the new field
  - `site/dashboard/PF1e-dashboard.json`, `site/dashboard/PF1e-dashboard.json.last-good`,
    `site/dashboard/inventory-pin.json`, `site/status-data.json` — the republished feed
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/EPIC-2_wrapup_regate2_report.md`
    and `EPIC-2_wrapup_regate_report.md` — the gate worker's hand-off, committed here (two
    denominator violations in the first one fixed; §6)
  - `docs/retro/events/at-35-e2-regate.jsonl`, `at-35-e2-regate2.jsonl`,
    `epic2-sheet-rule-regate.jsonl` — the gate worker's shards, committed here
  - `docs/retro/events/epic-2-wrap-up-gate.jsonl` — an orphan shard rescued from a worktree (§7)
  - `docs/retro/events/epic2-sheet-rule-fix2.jsonl` — this cycle's own events
  - `docs/release/SD-35-corpus-sheet-completion/kanban.md`, `progress.md`, this receipt
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** this cycle has no criterion of its own. Its bar is
  `workflow-instruction.md §10` step 0 verbatim: *"The full gate, once, on the isolated worker…
  Any red stage is fixed in a wrap-up correction cycle before the next epic's second cycle
  dispatches; the fix cycle is exempt from the batch floor (`decisions.md §2`'s exemption),
  never from the residue check."*
- **Receipt rows (mechanical):** closed=0, relabeled=0, **rust_lines_changed=0** (this cycle is
  Python and JSON only — `git diff --stat HEAD -- '*.rs'` prints nothing), ratio=n/a (zero
  closures by design, the §2 exemption), builds_recorded=1 (the one full `scripts/verify.sh`),
  pcgen_live_files=253 (unchanged).
- **PCGen residue:** `live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS`
  — the identical line at cycle start and at cycle end; not risen.
- **Oracle parity:** N/A — no `Number` mapping was added and no live path was touched.
- **Movement, four buckets:** closure 0 / relabel 0 / reachability 0 / **instrument-correction:
  one** — the public dashboard feed moved from `by_doneness done 23,650 / in-progress 17,563`
  to `done 46,965 / in-progress 160`, and the instrument that let it be wrong was rebuilt.
- **Refused tokens:** none — this cycle scoped no corpus population, so no refused-token
  deferral is owed. One deferral IS owed and emitted, for the worktree sweep (§7).
- **Discoveries:** the gate worker's stated root cause was wrong (§2), and the real one is a
  class of defect the bundle had not named: **a cache that lives outside the repo, shared by the
  main checkout and every linked worktree, whose freshness test never looks at the source
  document's content.** Emitted as `correction` `1789036432239-epic2-sheet-rule-fix2-cea8a5`.

---

## 1. The red stage, and that it is now green

`site-dashboard-check` — `timeout 2400s scripts/publish-site-dashboard.sh --check` — exit 1,
two lines:

```
site/dashboard/PF1e-dashboard.json input pin matches docs/work-inventory.json (5a0a0787312b5181d41214cb52abcd6e0c250fc409a75675ed6e839b4142e36f)
site/dashboard/PF1e-dashboard.json is STALE -- run ./scripts/publish-site-dashboard.sh
```

The pair matters: **`site-dashboard-pin` PASSED while `site-dashboard-check` FAILED.** The
input was certified; the output did not come from it.

In this cycle's own full run the stage reads:

```
    PASS  site-dashboard-check  (site/dashboard/PF1e-dashboard.json is current)
```

## 2. The gate worker's root cause is wrong — the disproof

The report attributed the stale feed to `publish-site-dashboard.sh`'s publish branch running the
producer **without** `PF1E_DASHBOARD_STRICT_TIMEOUT=1`, so a timing-out state dump falls back to
a stale cache and `write_input_pin` stamps it authoritative in the same run. It asked this cycle
to make the publish branch strict.

That is not the mechanism, and a control aimed at it would observe nothing. Run the producer on
the publish path's exact terms — **no strict flag** — against the same pinned inventory:

```bash
T=$(mktemp -d); cp site/dashboard/PF1e-dashboard.json "$T/PF1e-dashboard.json"; cp -r -p site/dashboard/units "$T/units"
/usr/bin/time -f "%e s" python3 scripts/observer/pf1e_dashboard_producer.py --out "$T/PF1e-dashboard.json"
python3 -c "import json;print(json.load(open('$T/PF1e-dashboard.json'))['work_inventory']['by_doneness'])"
```

It returns the **correct** `{'done': 46965, 'in-progress': 160, 'held': 2313, …}` in **12.67 s**.
`PF1E_DASHBOARD_STRICT_TIMEOUT` changes behaviour on one branch only — a subprocess timeout — and
no timeout occurred, so no stale-dump fallback could have been taken on either the publish run or
the check run. Making the publish branch strict would have changed nothing about this failure,
and would have traded away the blank-site protection `--check`'s own comment explains, for
nothing. **It was not done.** Emitted as `correction`
`1789036432239-epic2-sheet-rule-fix2-cea8a5`, `--verified-by` filled with the command above.

The dispatch brief's warning is the one that applied: *a control aimed at the wrong input is not
a control.*

## 3. What actually did it

`work_inventory.by_doneness` has exactly one source in the whole producer:
`compute_wiring_class_summary()` (`scripts/observer/pf1e_dashboard_producer.py`), read at the
payload's `"work_inventory": work_inventory_panel(load_work_inventory(), compute_wiring_class_summary())`.
That function is cached, and its cache is:

```python
WIRING_CLASS_CACHE = os.environ.get(
    "PF1E_WIRING_CLASS_CACHE", os.path.expanduser("~/swarm-observer/wiring-class-summary.json")
)
```

**Outside the repo.** One file, shared by the main checkout and all 14 linked worktrees, and by
every session on the box. Its warm-cache guard accepted a cache on three predicates:

1. `os.path.getmtime(cache_path) >= src_mtime` — the cache is newer than the document;
2. `cached["schema"] == WIRING_SUMMARY_SCHEMA`;
3. `cached["source_document"] == publishable_document_path(doc_path)`.

Predicate 3 looks like a provenance check and is not. `publishable_document_path()` exists
precisely to normalise an absolute path down to the repo-relative string
`docs/work-inventory.json`, so that no absolute path ever reaches `site/` — which means **every
tree's inventory answers to the same name.** None of the three predicates says anything about
the document's **content**.

So a cache computed in another worktree, or from an earlier revision of the same path, and
written a few minutes later, is newer, same-schema and same-name — and is served verbatim for a
document it never read. That is the observed pair of stage results exactly: the pin certifies
the input, the output came from somewhere else. Corroborating detail from the committed feed
before the repair — its `work_inventory.by_status` read
`{grounded 12551, sheet-complete 25232, text-complete 11655}` while `docs/work-inventory.json`'s
own `totals.by_status` reads
`{grounded 5222, oracle-agree 811, oracle-unverifiable 8491, sheet-complete 23315, text-complete 11599}`,
sum 49,438 either way; two different documents, one feed.

## 4. The control, and the proof it observes the failure

`compute_wiring_class_summary()` now reads the document's bytes once, hashes them, records
`source_document_sha256` on the summary it returns and caches, and **requires that digest to
equal the on-disk digest** before serving a warm cache. A cache written before the guard existed
has no such field, so `None != digest` rejects it — the empty case fails closed rather than
reading as agreement. `WIRING_SUMMARY_SCHEMA` bumped 13 → 14 (the shape changed; the repo's
stated convention, and `WiringSummaryTopLevelKeysCanaryTest` enforces the pairing).

RED first, as `AGENTS.md` rule 1 requires. `scripts/tests/test_pf1e_dashboard_producer.py::ForeignContentCacheIsRejectedTest`,
three cases:

| Case | What it pins |
|---|---|
| `test_cache_computed_from_other_content_at_same_path_is_rejected` | Cold run over document A (1 unit) writes the cache; the same path is then rewritten as document B (7 units) with the cache left newer by mtime, same schema, same normalised `source_document`. The summary must describe B. |
| `test_summary_records_the_sha256_of_the_document_it_read` | The recorded digest is the real `hashlib.sha256` of the bytes — a merely-present or self-consistent field would make the guard always-pass. |
| `test_warm_cache_is_still_served_when_the_content_matches` | An untouched document still hits the cache, so the guard does not silently defeat caching and turn every 5-minute cron tick into a recompute. |

Before the change, `python3 -m unittest scripts/tests/test_pf1e_dashboard_producer.py -k ForeignContentCacheIsRejectedTest`:

```
AssertionError: 1 != 7 : the warm cache was computed from DIFFERENT content at the same path and was served anyway
AssertionError: None != '8d0a642f7e5a27493678e6e55a679f8f38067c561969a4837e89e461a657e8f1'
Ran 3 tests — FAILED (failures=2)
```

After: `Ran 30 tests in 14.333s — OK` for the whole file (27 pre-existing cases plus these 3).

**Prove-it-can-fail, stated in the test's own docstring:** delete the `source_document_sha256`
comparison from the warm-cache guard and re-run — case 1 goes red because the foreign cache is
then served verbatim.

**What this proof does not cover** (`AGENTS.md` rule 7): the *other* out-of-repo cache in the
same directory, `~/swarm-observer/work-inventory-summary.json`, read by `load_work_inventory()`.
It is a different shape — its source is a `cargo run --bin v06_work_inventory --summary`
subprocess, not a file, so there is no input document to hash — and it is guarded by a
`max_age_seconds` age bound instead. It produced `by_status`, which was **consistent** across the
stale and fresh renders here, so it is not implicated in this failure. It is named as unproven,
not as clean.

## 5. The artifact repair

The feed was genuinely wrong, not merely stale: the public dashboard was reporting ~23,300 units
of the corpus as in-progress that are done.

```bash
bash scripts/publish-site-dashboard.sh    # 39.8 s; recomputed, the pre-guard cache being now correctly rejected
```

| `work_inventory.by_doneness` | Committed feed, before | Republished |
|---|---|---|
| `done` | 23,650 | **46,965** |
| `in-progress` | 17,563 | **160** |
| `held` | 2,313 | 2,313 |

`site/dashboard/units/*.json` did not move (the shards were not stale). `inventory-pin.json`
re-stamps the same digest `5a0a0787312b5181d41214cb52abcd6e0c250fc409a75675ed6e839b4142e36f`,
because the input never moved — which was the whole point. `site/status-data.json` moved with
it; `build_public_status.py` reports `30 books, overall 95.0% of 46074 items`.

Checked that the new cache field does not leak into the published payload:
`'source_document_sha256' in json.dumps(feed)` → `False`.

## 6. The second red stage, and why it is not a silencing

`denominator-gate` went red **in this cycle's own run**, `violations=2 of files_checked=260`,
and both violations are in the gate worker's report as delivered — two percentages stated with
no denominator on their own line (a reachability ceiling, and a disk-used share). Fixed by
stating the denominators, re-derived, not by widening the gate. The ceiling is now written the
way `/tmp/codex-verify-5aoIOM/reachability-audit.log` line 3 literally prints it:
`REACHABLE CEILING: 100.00%  (49438 / 49438)`. The disk share is now written the way `df -h /`
prints it, 1.1 T used out of 1.5 T total.
`correction 1789036818878-epic2-sheet-rule-fix2-a59318`.

## 7. Worktree sweep (§10 step 2) — deferred a fifth time, and now an operator action

Both Epic 2 worktrees were **proven** prunable, which the four previous wrap-ups did not manage:

```bash
git rev-list --count origin/tranche/15..a542652c5e   # wf_291be5c8-5f3-14 -> 0
git rev-list --count origin/tranche/15..8cc4ea1516   # wf_291be5c8-5f3-15 -> 0
git show origin/tranche/15:src/bin/formula_interpreter.rs | grep -c pcgen_import   # 3 — their WIP is already landed
```

Their only uncommitted content is the superseded `pcgen_import` codemod, saved to
`/tmp/cargo-sd35-epic2-sheet-rule-fix2/worktree-wip/*.patch`. One thing they held was **not**
superseded and is rescued and committed here: `docs/retro/events/epic-2-wrap-up-gate.jsonl`, a
single `verification` event from 2026-09-08 (`duration_seconds 5712`, FAIL on
`site-dashboard-check`, `reachability-audit-selftest`, `shape-engine-boundary-selftest`) that had
never reached the repo.

The removal itself did not happen: this agent's permission classifier refused both
`git -C <worktree> checkout -- .` and `git clean -fd`, and `git worktree remove` refuses a dirty
worktree. **No `--force` was used or is recommended.** `deferral
1789036867812-epic2-sheet-rule-fix2-33d8c9`. `df -h /`: 1.5 T, 1.1 T used = 72 % of the 1.5 T,
420 G free — no pressure, `preflight-disk` PASS.

## Figures + their re-derive commands (§8)

Each row states its figure and, on the same line, the command that re-derives it.

- Committed feed before repair, `by_doneness done 23,650 / in-progress 17,563` of 49,438 units — `git show f1f547a41e:site/dashboard/PF1e-dashboard.json | python3 -c "import json,sys;print(json.load(sys.stdin)['work_inventory']['by_doneness'])"`
- Republished feed, `by_doneness done 46,965 / in-progress 160 / held 2,313` of 49,438 units — `python3 -c "import json;print(json.load(open('site/dashboard/PF1e-dashboard.json'))['work_inventory']['by_doneness'])"`
- Non-strict producer returns the correct figure in 12.67 s, the §2 disproof — `/usr/bin/time -f "%e s" python3 scripts/observer/pf1e_dashboard_producer.py --out "$T/PF1e-dashboard.json"`
- Republish wall time 39.8 s — `time bash scripts/publish-site-dashboard.sh`
- Producer self-test 30 of 30 cases pass — `python3 -m unittest scripts/tests/test_pf1e_dashboard_producer.py`
- Inventory `totals.by_status` sums to 49,438 of 49,438 units — `python3 -c "import json;t=json.load(open('docs/work-inventory.json'))['totals'];print(sum(t['by_status'].values()), t['units'])"`
- Denominator gate violations 0 of files_checked 261 — `python3 scripts/denominator_gate.py --check`
- Figure provenance violations 0 of figures_examined 338 — `python3 scripts/denominator_gate.py --check-provenance`
- Rust lines changed 0 of 0 files — `git diff --stat f1f547a41e..HEAD -- '*.rs'`
- Gate worker's shards, 5 + 2 + 2 of 9 events — `wc -l docs/retro/events/at-35-e2-regate2.jsonl docs/retro/events/at-35-e2-regate.jsonl docs/retro/events/epic2-sheet-rule-regate.jsonl`
- Disk 1.1 T used of the 1.5 T total, 72 % of 1.5 T — `df -h /`
- Both Epic 2 worktrees carry 0 unmerged commits of 0 — `git rev-list --count origin/tranche/15..a542652c5e` and `git rev-list --count origin/tranche/15..8cc4ea1516`

## 9. Build scope verified

`RETRO_ACTOR=epic2-sheet-rule-fix2 CARGO_TARGET_DIR=/tmp/cargo-sd35-epic2-sheet-rule-fix2 CARGO_INCREMENTAL=0 bash scripts/verify.sh -j 3`
— every stage, no `--only`, run at `f1f547a41e` plus this cycle's working tree.

**49 stages, 48 PASS / 1 FAIL, 3,892 s = 1 h 4 m 52 s** (verify.sh's own derived event
`1789040412704-epic2-sheet-rule-fix2-3c1346`). Driver log
`/tmp/cargo-sd35-epic2-sheet-rule-fix2/verify-full.log`; per-stage logs `/tmp/codex-verify-5aoIOM/`.

**`site-dashboard-check` — the stage this cycle exists to fix — reads
`PASS  site-dashboard-check  (site/dashboard/PF1e-dashboard.json is current)`.**

The one FAIL is `denominator-gate`, and it is a **stale reading of a defect fixed later in the
same run**: that stage runs about twenty minutes in, and it read the gate worker's report as
delivered, before the two denominator fixes of §6 were written. Everything changed after that
point is Markdown under `docs/release/` — no `.rs`, no `.py`, no `site/` file — so the only
stages whose input moved are the two that read Markdown. Both were re-run at the committed tree,
together with the four stages that touch the changed producer and feed:

```
    PASS  denominator-gate  (files_checked=261 violations=0)
    PASS  figure-provenance  (files_checked=191 figures_examined=338 violations=0)
    PASS  site-dashboard-pin  (docs/work-inventory.json matches the pin the feed was published from)
    PASS  producer-selftest  (30 cases passed)
    PASS  site-dashboard-selftest  (13 passed, 0 failed)
    PASS  pcgen-residue-gate  (live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS)
RESULT: PASS
```

(`bash scripts/verify.sh --only denominator-gate --only figure-provenance --only site-dashboard-pin
--only producer-selftest --only site-dashboard-selftest --only pcgen-residue-gate`, derived event
`1789040846387-epic2-sheet-rule-fix2-8b423e`.) **Stated plainly rather than rounded up: this cycle
did not produce one wholly-clean full run.** It produced one full run with a single red stage whose
cause was then removed and re-proved green, and re-running the other 47 unchanged stages for a
Markdown edit is the ~70 minutes `decisions.md §3` exists to refuse.

**Concurrent writer noted, not touched:** at 07:24 another session registered a new worktree at
`.worktrees/ci-trait-choice` (`dc71e0810f`, branch `fix/trait-choice-set-id-roundtrip`), which
shows as an untracked directory in `git status` on this shared checkout. It is not this cycle's
work and nothing here stages or removes it.

- **Sweep population:** N/A — no corpus record changed.
- **Oracle pin:** `7f818006e371188e5717fd18d74d18a420747fc6` (`preflight-oracle` PASS); no figure
  in this receipt came from the pinned corpus.
- **Status:** complete
- **Notes:** the substantive judgment call is §2 — refusing to build the control the gate report
  asked for, because the measurement says it would not observe the failure. The report's *work*
  item (republish, §5) was right and is done; its *mechanism* was not.
- **Next-cycle scope:** Epic 2 wrap-up is closed. The two open items that belong to Epic 7 are
  the worktree sweep (§7) and the `epic-wrapup-gate-red` escalation the gate report raised.
