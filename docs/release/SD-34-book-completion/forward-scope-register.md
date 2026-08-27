---
canonical: true
owner: god-emporer
bundle_id: SD-34
date: 2026-08-26
---

# SD-34 Forward-Scope Register

Successor work depending on this package's output. **No unowned tidiness entries** — every
row names a home.

**This register is not a parking lot.** Scope that is in SD-34's Definition of Done at launch
cannot appear here; that would be the laundering
`../../governance/blocker-closure-doctrine.md` removes. A row here is either (a) work that
only becomes possible *because* SD-34 shipped, or (b) a question SD-34's posture deliberately
does not answer.

**The test:** was this scope in the Definition of Done at launch? If yes, it is a blocker,
not a register row.

## C1.x — Owned by an SD-34 successor epic

| ID | Item | Owner |
|---|---|---|
| C1.1 | The **one engine table SD-34 costs but does not build** — `power` (421 units, every one inside `ultimate_psionics`). Costed by AT-34-E5-003 from Epic 2's measured build rate across the eight it did build. A scope ruling with a stated reason (`decisions.md §7`), not a deferral. **The plan must also state what `ultimate_psionics` still needs after the table exists** — that book has all eight non-DONE buckets occupied, so the table alone does not close it. | A successor SD-N; the cleanest available opening move. |
| C1.2 | **The 35 books SD-34 prices but does not complete** — each with its per-bucket population, clearing mechanism, and projected cost in `artifacts/epic-5-forward-plan/forward-plan.json`, ordered cheapest-first with single-bucket books flagged by name. **This row is a costed queue, not a deferral** — producing it is SD-34's primary deliverable (`scope-draft.md §7` S3). | A successor SD-N per book or book-group, dispatchable straight from the plan. |
| C1.3 | **Every capability AT-34-E5-002's register names that SD-34 did not build.** Per capability: what it is, which buckets and books it unblocks, and its population. This is the operator's question — *"if we need to build something to process the remaining work after the shape engine runs, sd34 must tell us that"* — answered in machine-readable form. | A successor SD-N, scoped directly from the register. |

| C1.4 | **SD-35 — the remaining 35 books.** Operator intent, 2026-08-27: *"sd-35 will be the rest of the books unless we find a problem."* SD-34's two-book scope is a **deliberate proof**, not a sample of convenience — it validates the completion theory before committing to a ~50,000-unit run (`technical-requirements.md` N7's measure-before-a-population-run rule, applied at bundle scale). SD-35 inherits the priced plan from AT-34-E5-001 and the measured rates from AT-34-E2-003 / E3-004 / E4-003. **Precondition:** SD-34 closes without finding a problem that invalidates the theory; AT-34-E3-006's `atlas-defects.md` is the instrument that would surface one. | SD-35. |
| C1.5 | **The mechanism-leverage finding, measured 2026-08-27 — SD-35's dispatch shape.** The corpus's **35,650 of 49,438** non-DONE units resolve to **136 distinct mechanisms** (evidence families, `:`-suffix normalised); **29 of 136 cover 90%** of the population, and the largest span **17-28 of 37 books each**. Re-derive: group non-DONE units in `docs/work-inventory.json` by `evidence` with the `:<specific>` tail stripped. **Demonstrated in this bundle, not projected:** Epic 2 built 8 tables in ~4 cycles and bucket A fell **8,463 -> 449 of 49,438 corpus-wide**; AT-34-E1-008 restamped **7,015 of 10,196** stale records across 34 books in 4 cycles; **1,649 units reached DONE across 20 books, of which only 53 are `core_rulebook`** — 89% of the movement landed outside the two vehicle books. **Consequence for SD-35:** cost is per *cycle*, not per unit, and a cycle scoped to one book's slice of a shared mechanism discards most of what it already paid for. Scope SD-35's cycles to the **mechanism, corpus-wide**; book completion is the by-product. This is `../../governance/`-adjacent doctrine already recorded as *generic pass, not per-object lanes* — SD-34 supplies its first measured proof. | SD-35's `workflow-instruction.md §3`. |

## C2.x — Future SD-N ownership

| ID | Item | Owner |
|---|---|---|
| C2.1 | **The second PCGen-format reader** (Starfinder — already in the pinned checkout at `data/starfinder`, same `.lst` format, different `.pcc` include structure). Inherited unchanged through SD-32 and SD-33. **Explicitly out of SD-34's scope** (`scope-draft.md §9`): there is no case for a second system while the first has 37,173 of 49,438 units open. | A future SD-N. |
| C2.2 | **Traveller, Cyberpunk Red, World of Darkness, Solarus Arcanum.** Inherited from SD-33 C2.2. Each needs its own answer to "what is the oracle", a question now visibly separate from "what is the reader". | A future SD-N per system. |
| C2.3 | **The form-interpreter PMMG build** ("Edge of the Sea" tranche). Inherited from SD-33 C2.3; `scripts/verify.sh` still carries the warning every cycle. | A future SD-N. |

## C3.x — Research-grade forward scope

| ID | Item | Owner |
|---|---|---|
| C3.1 | **How to verify a system whose rules exist only as prose.** Inherited from SD-33 C3.1. A prose-sourced system has no oracle at all — the extraction *is* the corpus and the extraction is the thing most likely to be wrong. SD-34 sharpens it: "done" is defined against a pinned oracle, so a system without one has no definition of done either. | A research spike. |
| C3.2 | **Whether oracle agreement is the right definition of correct.** Inherited from SD-33 C3.2. PCGen is an implementation, not the rulebook; where it diverges from print, the harness reports `agree` on a shared error. **SD-34 raises the stakes**: a book declared "done forever" on oracle agreement inherits every place the oracle is wrong. Quantifying that rate is now more load-bearing than it was. | A research spike, scopeable once SD-34 has a body of book-level agreement data. |
| C3.3 | **Errata.** The operator's own definition names it: *"done. forever. or at least until paizo posts errata requiring updates."* SD-34 has no mechanism to detect that a banked book's source has changed upstream. A book declared done stays declared done regardless. | A future SD-N. **Named because the definition names it**, not because SD-34 deferred it. |

## E1.x — Branches ruled OUT of SD-33's 2026-08-26 fold — do not re-litigate

Mirrored from `../SD-33-computed-value-verification/forward-scope-register.md §E1`. SD-33 wrote:
*"Do not re-fold any of the three. A branch's file count is not its value."* AT-34-E6-003's sweep
**deletes these on that ruling and does not re-diagnose them** (`decisions.md §12` L6).

| ID | Branch | Ruling | Why |
|---|---|---|---|
| E1.1 | `worktree-wf_a45ece26-3fc-1` | **Superseded** | 1,612 grant files — the largest file count of any branch swept. Its `class` field holds feature-group names where HEAD's curated records hold real class names, and it lacks `granted_via_archetype`, which `class_feature_grant_consumer.rs` defaults to `true` when absent. Folding it would silently mis-mark all 1,612 records. |
| E1.2 | `worktree-wf_13156488-c9b-1` | **Superseded** | Wave 20 work, superseded by SD-32's `untabled_base_class_feature_roster`. |
| E1.3 | `worktree-wf_c1156061-e3f-5` | **Superseded** | Wave 30 — 27 lines of notes on a closed bundle. No records, no code. |

Any **other** branch the sweep finds carrying unmerged records is diagnosed schema-against-HEAD
before it is folded or removed, and the diagnosis goes in the sweep receipt.

## Carried forward from SD-33

| ID | Item | Owner |
|---|---|---|
| C1.8 (closed here) | Wire `v06_corpus_trap_report -- --audit` into `scripts/verify.sh` as a real stage. Inherited unclosed through SD-31, SD-32, and SD-33. | **AT-34-E1-007** (kanban row 7). In SD-34's Definition of Done — not a carry. |
| SD-33 inherited test debt | **29 of 599** workspace suites carrying **46 of 8,034** failures, proven pre-existing at the `tranche/13` cut, registered by SD-33's closure (its register D1.1; the 31 / 49 of 8,026 figure quoted through attempt 10 was corrected by the operator's fold — two targets fixed outright, denominator +8). SD-34 re-derives the set at the `tranche/14` cut as its own baseline (`technical-requirements.md §3`) but does not own fixing it. | A future SD-N, or a dedicated cleanup cycle. |
| SD-33 open deferral 1 (`1787633115006-sd33-e4-unknown-136912`) | Widen `REGISTERED_POOL_GROUPS` in `src/rules_core/class_feature_pool_catalog.rs` to resolve more of the 3,052 `class_feature_option_pool_record_with_magnitude_not_held_by_engine` units to a real class owner; 1,128 distinct unmatched group prefixes need per-group corpus research. Those units sit in SD-34's bucket `B`. **Revisit condition:** a cycle with corpus-research time budgeted for pool ownership — in SD-34 that is AT-34-E3-001 for the Core Rulebook slice; the rest is priced by AT-34-E5-001. | AT-34-E3-001 (core slice); Epic 5's plan (remainder). Not SD-33 DoD scope. |
| SD-33 open deferral 2 (`1787633121875-sd33-e4-unknown-58d073`) | Add recognition of `status == unmeasurable` to `scripts/observer/pf1e_dashboard_producer.py`'s `_doneness_verdict_uncapped()`. **Revisit condition:** whichever cycle next touches that file — AT-34-E1-005's rename touches every consumer of the status field, so that is the cycle. | AT-34-E1-005. |
| SD-33 open deferral 3 (`1787667636036-sd33-r6-skillcombat-3dee2d`) | COMBAT non-AC subtoken aggregation (INITIATIVE / TOHIT / TOHIT.Ranged / SAVE, 6 units), cross-record class-feature variable resolution (`flurry_of_fists`, `flurry_of_strikes`, 2 units), and the Special-Quality eqmod live-oracle attachment gap (2 units). Each a larger-than-one-cycle `src/rules_core/` or `scripts/oracle_harness/` surface. **Revisit condition:** a cycle with dedicated budget for the new `ResolvedEquipmentEffect` fields / cross-record lookup / EQMOD attachment. Those units are in bucket `V` or `U`; the Core Rulebook ones are AT-34-E3-003's. | AT-34-E3-003 (core slice); AT-34-E5-002's capability register (the rest). |
| `site-dashboard-check` hang | `publish-site-dashboard.sh --check` invokes `v06_work_inventory --summary` with no timeout wrapper; observed spinning at full CPU past a 600s producer timeout and killed after ~12 minutes, three times on three different diffs during SD-33's closure. Filed as an incident, not an environmental note. | A future SD-N, or whichever cycle next touches `scripts/verify.sh`'s stage list. |
