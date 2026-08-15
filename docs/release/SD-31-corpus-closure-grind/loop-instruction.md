---
canonical: true
owner: god-emporer
status: planning-ready (SD-32 absorbed, epics re-sequenced, operator ruling 2026-08-15)
date: 2026-08-15
---

# SD-31 Loop Instruction

**`SD-30-class-feature-archetype-bundle/loop-instruction.md` governs this package's cycles, with the
overrides below.** Do not fork a second copy of that file's cycle procedure (branch-state check,
disk-budget step, verify.sh discipline, stop-vs-press-on rules, retrospective-event emission,
unattended-mode authorization) — read it directly each cycle. This file states only what differs.

## Overrides

1. **Card source:** this package's own `kanban.md`, not SD-30's — `epic-0-reachability-audit` through
   `epic-9-closure`, in that claim-priority order. Capability (Epics 1-2) precedes the lanes consuming
   it; see `decisions.md §2` for why, and do not re-order by convenience.
2. **PI-gate citation is a cycle-0 precondition, every cycle in Epics 5/6/7:** before claiming a
   card that touches a specific book, read `SD-30-class-feature-archetype-bundle/kanban.md` and confirm
   `epic-3-pi-gate`'s state for that book (cite the SD-30 `progress.md` receipt showing SD30-E3-F2/F3
   `COMPLETE` for it). A cycle that skips this check is out of protocol (`acceptance-and-verification.md
   AT-31-003`).
3. **Progress receipts land in this package's own `progress.md`**, not SD-30's — even though the cycle
   consumes an SD-30 gate, the receipt for work done under this package's epics is recorded here.
4. **Concurrency/hardware numbers:** `SD-30 decisions.md §47`'s 8-core capture is **stale** — SD-30's
   own pre-launch cycle re-measured the box on 2026-08-14 at **24 cores / 167 GiB / 968 GB at 19 %
   used**, computed a full-gate cap of **8**, and wrote the new figures into `SD-30
   loop-instruction.md`'s "Concurrency and resource budget" section. Read that section, not `§47`, and
   re-derive before any wave per the standing rule — this box has now moved twice in one week.
5. **The two internal capability gates are hard, and they are per-batch, not per-epic** (`decisions.md
   §2`, `kanban.md` "The two gates that exist because of the merge"). Before claiming `epic-6-ingest-lanes`
   F3/F4, read the named race-batch list Epic 1-F3 maintains in `kanban.md` and confirm the batch covers
   the races the target book's rows reference — "Epic 1 is in flight" is not an open gate. Before
   claiming Epic 3-F4 or Epic 5-F3, confirm `epic-2-verdict-paths` is `COMPLETE`. Re-derive the workable
   pool after every chassis batch; the 553-unit `race_trait` figure is a function of Epic 1's output,
   not a constant.
6. **Deferral is not available to a cycle.** "Named a successor for the remainder" is struck from this
   package (`decisions.md §2` item 5). A unit leaves the 100 % denominator only via an operator-signed
   Structural Exclusion Register entry (`decisions.md §3`, `AT-31-100`). A cycle may propose; only the
   operator grants. Cost is never an exclusion reason.
7. **Epic 0's audit runs at every epic closure**, and its output goes in the receipt (`decisions.md §4`).
8. **The PCGen oracle pin is checked first, every cycle.** The first command after the branch-state
   check (SD-30 loop-instruction.md's cycle-0 step) is `scripts/verify.sh --only preflight-oracle`
   (or, equivalently, `scripts/fetch-pcgen-oracle.sh --check`). Bootstrap with
   `scripts/fetch-pcgen-oracle.sh` if it fails. Quote the pin SHA
   (`scripts/pcgen-oracle-pin.env`'s `PCGEN_ORACLE_SHA`) in every cycle's re-derive receipt — a
   figure re-derived against an unstated oracle commit is not re-derived.
9. **A quoted dashboard figure must name its source when `status_sources_agree` is false** (added
   2026-08-15, launch-readiness remediation Step 5, drift D13). The live dashboard JSON's
   `work_inventory.status_sources_agree` field (`scripts/observer/pf1e_dashboard_producer.py`,
   `_cross_tab_status_margin` / `work_inventory_panel()`) is the arithmetic skew test between the
   dashboard producer's two work-inventory sources — a fresh `v06_work_inventory --summary` run vs.
   the committed full document's cross-tab. They routinely carry different `generated_at` stamps
   without the figures actually disagreeing; when `status_sources_agree` is `false`, they really are
   different corpus snapshots, and citing a number from the JSON without saying which of the two
   sources it came from (`by_status` vs. `by_doneness`/`cross_tab`, and each one's own
   `generated_at`/`doneness_source_generated_at`) risks a figure that silently drifted between the two
   reads. Any cycle receipt quoting a live-dashboard figure checks this field first and, if `false`,
   names the specific source field and its own stamp — not just "the dashboard says N."

## What is not overridden

Everything else in SD-30's `loop-instruction.md` — the per-cycle procedure shape, the disk reclamation
step, the "generated, never hand-maintained" figure discipline, Decision §22's Workflow-tool dispatch
mode, Decision §24's stop-vs-press-on rules, Decision §48's Opus-high orchestrator mandate — applies to
this package's cycles unchanged.
