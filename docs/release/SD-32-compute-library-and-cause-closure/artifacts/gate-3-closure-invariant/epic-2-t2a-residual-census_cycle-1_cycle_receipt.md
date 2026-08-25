# Cycle 1 — Gate 3 (closure invariant) / Card 11 `epic-2-cause-closure`, lane T2a-residual (measurement only)

- **Card ID:** `epic-2-cause-closure` (shared row; this receipt covers ONLY the T2a-residual
  measurement lane authorised by `decisions.md §13` — the actual mapping work is dispatched from
  this receipt's census output to sibling cycles, per that decision's "measurement... does not
  substitute for the work")
- **Commit SHA:** (this commit, landed on `tranche/12` via the §5 retry protocol)
- **Files touched:**
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-3-closure-invariant/card11-t2a-residual-census-census.md`
    (new — the census memo)
  - `scripts/sd32-t2a-residual-census.py` (new — the committed re-derive script)
  - `docs/retro/events/t2a-residual-census.jsonl` (new — 1 correction, 1 deferral)
  - `docs/retro/events/sd31-transcribe.jsonl` (incidental append from an env-default-actor
    `verify.sh --only preflight-oracle` self-heal check run during this cycle's §2.1 setup, before
    `RETRO_ACTOR` was exported in that shell invocation — a benign side effect of the harness's own
    append-only logging, not a manual edit)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  (`git diff --unified=0 1bb523773d32705d1b7387fd4c494861523f55ba...HEAD -- <files above minus the jsonl logs>`)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff)
- **Acceptance criterion:** `acceptance-and-verification.md` AT-32-E2-001 — cause closure closes by
  class, not by instance. This cycle's scope per the dispatch brief: measurement only for T2a's
  ~2,775-unit residual (re-derived to 2,640), per `decisions.md §13`'s explicit authorisation of a
  measurement precursor. No units closed this cycle — by design, per standing lesson 6
  (`workflow-instruction.md §9`): "a cycle that banks zero units but changes the plan... is a closed
  cycle, judged on its receipt."
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`) —
  fresh worktree, footgun-1 wrong-base landed (stray `site-publish` merge, no `docs/`/`data/`/`scripts/`
  tree); self-healed per §8 via `git reset --hard 8b8e00c0d` + rebase onto `origin/tranche/12`, then
  empty oracle slot self-healed via `scripts/fetch-pcgen-oracle.sh --dest <repo-local pcgen slot>`.
- **Status:** complete (measurement scope only; card 11's shared row is untouched at `in-progress`,
  per the dispatch brief's explicit instruction not to touch its status)

## What this cycle found

1. **Re-derived total: 2,640, not `decisions.md §13`'s tilde-marked "~2,775".** Logged as a
   `scripts/retro.py correction` — see the memo's "Findings" section and
   `docs/retro/events/t2a-residual-census.jsonl`.
2. **547 distinct category labels across 18 books**, heavily long-tailed (266 singletons, 398
   labels with ≤3 records).
3. **Zero of the 547 labels are already covered by `CLASS_FEATURE_POOLS`'s 27 entries** — confirmed
   by set-difference, not assumed. Every group in the structured return carries `registered: false`.
4. **No new consumer-conflict hazard** beyond the one the T2a+T12 cycle already found and fixed
   (`class_feature_pool_catalog.rs`). Audited all remaining `data.class` readers in the codebase.
5. **Feasibility sample (10 groups, TYPE:/PRE token reads) shows the population is NOT uniform**:
   several labels resolve cleanly to an already-dispatched class in one token read (`Ki Power` →
   Monk, `Master of Many Styles` → Monk, `Pack Lord` → Druid, `Adaptation` → Ranger, `Favored Enemy
   Bonus` → Ranger — 239 units, quick wins); several resolve cleanly but only to an
   undispatched-but-real class (`Wild Talent` → Kineticist, `Implement School Focus Power` →
   Occultist — 176 units, these close T2a-residual but land in the T2a∩T12 overlap shape, not fully
   Computed); and at least two need real care: `Domain Power` (172 units, the single largest group)
   is genuinely multi-owner/contextual (shared `DomainLawLVL`-shaped variables across several
   domain-granting classes) and would be mis-mapped by a naive single-class table entry — flagged as
   this census's most important finding; `Demonic Obedience` (42 units) is likely not class-owned at
   all and should be confirmed as "correctly not mapped," not forced into a class.
6. **Mechanism gap:** `POOL_TO_DISPATCHED_CLASS` (tier 2) only targets dispatched classes today;
   mapping a residual label to an undispatched-but-real class (`Wild Talent` → `Kineticist`) needs a
   small, well-precedented extension (parallel table or tier-4 alias teaching), not a new mechanism.
   Named as work-lane scope, not built by this cycle.

## Scope discipline

Read-only on everything except this receipt, the census memo, the re-derive script, and retro logs.
`kanban.md` row 11 left at `in-progress`, untouched. No engine code, corpus data, or pinned count
changed.

- **Discovery forwards:** none requiring a new card — the `Domain Power` multi-owner hazard and the
  `Demonic Obedience` not-class-owned finding are both logged in the census memo as work-lane inputs
  for the sibling T2a-residual work cycles, not new cards.
- **Next-cycle plan:** dispatch work-lane cycles against this census's `books[]` group list (top-20
  labels as individual entries, tail bucketed by book) — quick-win groups first (`Ki Power`, `Master
  of Many Styles`, `Pack Lord`, `Adaptation`, `Favored Enemy Bonus`), `Domain Power` and `Demonic
  Obedience` last, given their extra verification care.

`df -h /`: see final report.
