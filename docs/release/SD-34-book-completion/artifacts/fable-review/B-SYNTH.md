# B-SYNTH — SD-35 backlog assessment (synthesis of lanes B1–B4)

Judge: B-SYNTH. Date: 2026-08-31. Repo HEAD at assessment: `b4485ae534`; `docs/work-inventory.json` last regenerated at `3aebc28477` (wave-22 shared regeneration), confirmed an ancestor of HEAD.

Every figure below carries its denominator and one of three provenance tags:
**MEASURED** (read off a live artifact or re-derived from one), **EXTRAPOLATED** (a measured rate applied to an unmeasured population), **ASSUMED** (no measurement backs it).

I re-derived the population figures myself, independently of all four lanes, by importing `scripts/completion_atlas.py`'s pure `_bucket_of` and running it over the live `docs/work-inventory.json`. No generator was executed; the only write is this file.

---

## 1. VERDICT

**The method is right. The headline number is dead. The plan artifact everyone is quoting is stale, and SD-34 itself is not closed.**

### 1.1 The 29,283 figure is retired

`docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/forward-plan.json` was generated at `fef202a566`, 107 commits behind HEAD. All four lanes independently found it stale; I make the fifth derivation. They agree to the unit.

| Figure | forward-plan.json | Live (HEAD, MEASURED) | Δ |
|---|---:|---:|---:|
| Total units, 35 non-vehicle books | 42,472 | 42,472 | 0 |
| DONE | 13,189 | 20,103 | +6,914 |
| **Remaining non-DONE** | **29,283** | **22,369** | **−6,914 (−23.6%)** |
| Bucket A | 449 | 449 | 0 |
| Bucket B | 11,299 | 11,299 | 0 |
| Bucket C | 3,981 | 3,981 | 0 |
| Bucket D | 2,587 | 2,587 | 0 |
| Bucket M | 3,977 | 3,635 | −342 |
| Bucket U | 171 | 171 | 0 |
| Bucket V | 6,747 | **175** | **−6,572 (−97.4%)** |
| Bucket X | 53 | 53 | 0 |
| Bucket Z | 19 | 19 | 0 |

All rows MEASURED, denominator 42,472 total units across the 35 non-vehicle books. The drift is confined *exactly* to M and V; every priced bucket is untouched.

**The load-bearing correction:** the "UNPRICED 13,383 units = 45.7% of remaining" claim is wrong by more than 2×. Live unpriced (D+M+V+X+Z) = **6,469 of 22,369 remaining = 28.9%** (MEASURED). Anyone still quoting 45.7% is 16.8 percentage points off on share and 6,914 units off on absolute count.

**The tiers that are still trustworthy:**
- Priced-to-DONE (A+B+U) = **11,919 units of 22,369 remaining** (MEASURED, unchanged) → **1,952.42–6,782.37h** (EXTRAPOLATED from n=2 cycles across 2 books).
- Priced-to-V (C) = **3,981 units of 22,369** (MEASURED, unchanged) → **96.39h** (EXTRAPOLATED from n=1 cycle, one book).

### 1.2 Where the lanes disagree, and who wins

**Disagreement 1 — what the V collapse means.** B3 calls it "the single largest realized ROI event in this program to date… the oracle harness clearing thousands of units corpus-wide in days." B2 calls it a ledger reconciliation of SD-33's *already-computed* verdicts at **zero new oracle runs**, citing `decisions.md §19`, the `artifacts/bucket-v-widen/AT-34-E3-005_bucket_v_widen_cycle_receipt.md` receipt, and the `oracle-agree`/`oracle-unverifiable` → DONE branch in `_bucket_of`. **B2 wins** — I read that branch directly and it is a status remap, not a computation. Both are describing the same value; they differ on *when it was created*. The engineering value came from SD-33's harness build. The 4-day visible drop was bookkeeping catching up. **Do not budget any future work against the apparent rate of "6,572 units in four days." That rate is an artifact of deferred accounting.** This matters: it is precisely the shape of error the repo's own "instrument correction is not closure" rule exists to catch.

**Disagreement 2 — the mechanism census.** `forward-scope-register.md` §C1.5 says 136 distinct mechanisms / 35,650 non-DONE units / 29 cover 90%. B3 could not reproduce it and reported 94 mechanisms / 24,475 units / top-25 = 90.2%. My re-derivation (same stated method — `evidence` field, tail after first colon stripped): **94 distinct mechanisms over 24,475 non-DONE units across all 37 books, 24 mechanisms covering 90%** (MEASURED); restricted to the 35-book SD-35 scope, **83 mechanisms over 22,369 units, 22 covering 90%** (MEASURED). B3's shape is right and its count is right to within a rounding of the 90% cutoff. §C1.5's numbers are from a pre-reconciliation snapshot and are retired; its *claim* — a small mechanism set dominates — is confirmed and stronger than stated. Note also: the inventory holds **37 books**, not 38; B3's "all 38 books" label is a miscount that does not affect its arithmetic.

**Disagreement 3 — B4's fast/slow split. This is the weakest load-bearing claim in the whole assessment set, and I am partly overturning it.** B4 measured, correctly and valuably, that core_rulebook's 29 bucket-B cycles split into a fast class (451 units / 89.7% of units cleared, 32.7% of wall time, **14.64 DONE-units/hr**) and a slow class (52 units / 10.3%, 67.3% of wall time, **1.50 DONE-units/hr**) — a 9.8× gap, MEASURED over 503 units / 2,432.3 wall-minutes / one book. That decomposition is sound and reproduces the official blended 5.8 units/hr exactly.

The error is projecting core_rulebook's **89.7% fast / 10.3% slow mix of what was already cleared** onto the 11,299-unit remainder. That is a selection effect: the fast shapes got cleared first. The live corpus-wide bucket-B mechanism census says the remainder's composition is different:

| Bucket-B mechanism, 35 books | Units | % of 11,299 | B4's core_rulebook class |
|---|---:|---:|---|
| `class_feature_owner_matched_by_name_but_record_not_held_by_engine` | 3,235 | 28.6% | SLOW (0.71 units/hr, n=1 cycle) |
| `class_feature_option_pool_record_with_magnitude_not_held_by_engine` | 2,715 | 24.0% | SLOW (7 sub-causes, never re-measured) |
| `class_feature_option_pool_record_not_held_by_engine` | 1,625 | 14.4% | SLOW (same family) |
| `template_content_absent_from_template_table_in_core_essentials` | 977 | 8.6% | FAST shape |
| `feat_key_absent_from_catalog` | 490 | 4.3% | FAST shape |
| `spell_key_absent_from_spell_list` | 391 | 3.5% | FAST shape |
| 41 further mechanisms | 1,866 | 16.5% | mixed |

All MEASURED, denominator 11,299 bucket-B units in the 35 books. **67.0% of the bucket-B remainder (7,575 units) sits in the three `class_feature` families that core_rulebook measured as its slow class** — the inverse of B4's projected 10.3%.

**But there is a second measured cut that points the other way, and it is the more useful one.** Reading the same 11,299 units by `wiring_class`:

| wiring_class | Units | % of 11,299 |
|---|---:|---:|
| display | 6,545 | 57.9% |
| static | 809 | 7.2% |
| derived | 1,155 | 10.2% |
| computed | 2,572 | 22.8% |
| ambiguous | 218 | 1.9% |

MEASURED. And all **3,235** `owner_matched` units carry `magnitude_token_count = 0`, of which **2,992 are `display`**; **1,572 of the 1,625** no-magnitude `option_pool` units are likewise `display`. Under the repo's standing ruling that a zero-magnitude feature with a description shown to the player is DONE, those units clear by **table membership plus description** — the exact shape the SD-34 Epic 2 kind-table loader and core_rulebook's fast cycles (a generic `race_trait_generic/` table fallback: 132 units in 70.0 min = 113 units/hr) already proved.

So the two measured cuts of the same population disagree about how hard the remainder is, by roughly an order of magnitude in projected hours:

- If the class_feature families behave as core_rulebook's slow class did: 7,575 units at 1.50/hr + 3,724 at 14.64/hr ≈ **5,304h** (EXTRAPOLATED) — near the top of forward-plan.json's 1,952–6,782h range.
- If they are display-shaped placement work: 7,354 display+static units at the fast 14.64/hr + 3,727 compute units at a slower rate ≈ **600–1,000h** (EXTRAPOLATED).
- B4's own two-regime figure, **~1,468h** (EXTRAPOLATED), sits between them but rests on the composition assumption I just showed is wrong.

**That spread is the honest current state of knowledge, and closing it is the single highest-value measurement in SD-35.** It is card 1's job (§4). Do not let any SD-35 plan quote a bucket-B hour figure as settled.

### 1.3 So: is the pipeline addressing the backlog properly?

Yes on method, with three corrections that must land before SD-35 launches.

**What is working.** The bucket taxonomy is a real instrument, not prose. Every unit is classified by live code (`_bucket_of`), the classifier is honest about what it does not know (five buckets carry a null rate and the plan file says so), and the mechanism census makes the remainder closable by name rather than by "the rest." Rate ledgers with per-cycle wall time exist for buckets A, B, C and U. That is a materially better instrument than SD-31 had.

**What must be fixed first.**
1. **Regenerate the plan artifacts.** `forward-plan.json` (`build_forward_plan.py`) and `ordered-plan.json` (`build_ordered_plan.py`) are stale; `ordered-plan.json` is stale on two independent axes (generated one commit set *before* forward-plan.json and already disagreeing with it at 7/35 books, net 81 units, before the 6,914-unit drift). `capability-register.json` is stale on its V-related field. No SD-35 scoping document may cite them un-regenerated.
2. **Stop quoting a to-V rate as a to-DONE rate.** Bucket C's measured rate is 41.3 units/hr *net-reclassified* and **0.0 units/hr to DONE** (MEASURED, n=1). The 96.39h price for 3,981 units buys a bucket move, not closure. That distinction is exactly the "closure vs relabel" trap this repo has already been bitten by.
3. **SD-34 is not closed.** kanban.md shows **20 of 36 rows complete** (MEASURED, counted directly; the file's own prose header still says "27 rows covering 28 criteria" and is stale text). Scoping SD-35 off an unfinished bundle is fine as a proposal, but SD-35 cannot launch on figures whose producing bundle still has 16 open rows.

---

## 2. ENGINE ANSWER

**Yes — engines can expedite SD-35, and they are the only thing that can. But the track record says engines pay off only under a specific discipline, and roughly a fifth of the remainder is provably outside their reach.**

### 2.1 What the track record actually says

- **SD-31 generic passes FAILED** (`docs/retro/sd31-retrospective.md`). The failure mode was a pass defined by *scope* ("run something generic over the corpus") with no named mechanism, no measured target population, and no oracle to say whether the output was right.
- **SD-32 `formula_interpreter` + `bonus_stack_reader` SUCCEEDED** (`docs/retro/sd32-*`) — but slowly at the tail: ~6.5h initial build (2 cycles) + ~2.4h generalization (4 cycles) + **12+ further cycles / ~2 days of widening** for a *narrower* 10-family scope before coverage rose meaningfully. Coverage reached 4,798 of 11,652 formula-bearing units (41%, MEASURED).
- **SD-33 oracle harness SUCCEEDED** — the most expensive one-time build of the three (10 dispatch waves / ~2 days plus fold remediation), and it is what made the 6,572-unit V disposition legitimate rather than a relabel. Its value was verification capacity, realized later.
- **SD-34 generic kind-table loader SUCCEEDED and is the cheapest win on record** (`artifacts/epic-2-tables/table-build-rate.json`): **all 7 new kinds together** for 1,359s + 779s of cycle wall time (0.59h total, MEASURED), sweeping corpus-wide bucket A from 8,463 → 449.

The dividing line is not "generic vs specific." It is: **named mechanism → measured population → small build → oracle verification → corpus-wide run.** SD-31 skipped the first two and the fourth. Everything that succeeded did all four.

### 2.2 Engine-addressable fraction of the 22,369-unit remainder

All MEASURED against the live inventory; denominator 22,369 remaining non-DONE units, 35 books.

| Class of work | Units | % of 22,369 | Engine story |
|---|---:|---:|---|
| **Placement / display work** — B display+static (7,354) + D `pending_wiring_class_review` display+static (1,563) | **8,917** | **39.9%** | Directly engine-addressable. Same shape as the proven Epic 2 kind-table loader. Largest lever in the backlog. |
| **Compute work** — B computed+derived (3,727) + M computed+derived (3,428) | **7,155** | **32.0%** | Addressable by the *existing* SD-32 engines, but SD-32's own widening tail says this is cycles-of-work, not a multiplier. |
| **Not engine-shaped** — bucket C, one mechanism (`no_explanation_id_and_no_diagnostic_names_this_feature`), 100% of C | **3,981** | **17.8%** | Confirmed per-feature hand-wiring: 39 distinct hand-written `explanation_id` call sites in `src/rules_core` (MEASURED, B3). No engine touches it. |
| **Capability builds / residue** — X (53), V (175), Z (19), U (171), remaining ambiguous/other B and D | **2,316** | **10.3%** | Bespoke or one-time capability construction; not per-unit engine work. |

**Headline: ~72% of the remainder (16,072 of 22,369) is engine-shaped; ~18% (3,981) is provably not; ~10% is bespoke residue.** The 17.8% that is not engine-shaped is also the single largest mechanism in the entire corpus. SD-35 costing must carry bucket C as its own hand-wiring line item and must never fold it into "mechanism leverage."

### 2.3 The honest caveat

"Engine-addressable" is a statement about *shape*, not about *proven throughput*. The only place a shape-to-rate link is MEASURED is bucket A (0.59h for 7 kinds → 8,014 units cleared corpus-wide) and core_rulebook's fast bucket-B cycles (14.64 DONE-units/hr, n=14 cycles, one book). Everything else in the 72% is EXTRAPOLATED across a shape analogy. That is why §4's spike is card 1 and not card 9.

---

## 3. PROPOSED SD-35 SHAPE

Five engines, ordered by measured-units-covered ÷ calibrated build cost. **Per-mechanism corpus-wide cycles throughout — no per-book lanes.** Per-book lanes are what produced SD-31's zero yield and they re-pay the fixed cost 35 times.

Every engine carries the same gate, copied verbatim in structure from the SD-32/SD-33 success pattern:

> **GATE (measure → build → oracle-verify → corpus run).**
> **G1 Measure:** re-derive the exact target population from live `docs/work-inventory.json` by mechanism (not by book), record the count and the denominator in the card before writing code.
> **G2 Build:** one mechanism, timeboxed. Record wall-clock in a `step-cost-ledger.json` entry — a cycle with no recorded wall time does not count as measured.
> **G3 Oracle-verify:** run the SD-33 harness against a sample of ≥30 affected units (or the whole population if smaller). **Go requires ≥90% `agree` or `unverifiable-with-named-reason`, and zero `disagree` that the build caused.**
> **G4 Corpus run:** single corpus-wide pass. **Success is measured as units moving `→ DONE`, never as units moving between non-DONE buckets.** Report the bucket-diff, closures and relabels separately.
> **No-go:** if G3 fails, the engine stops at G3 and the card reports the failure with its measured population intact. It does not proceed to G4 "to see."

### Engine 1 — Kind-table loader extension: `power` and `companion` kinds

- **Mechanisms served:** `power_content_has_no_engine_table`, `companion_content_has_no_engine_table`.
- **Units covered:** **449 of 22,369 remaining (2.0%)** — 421 `power` (all in `ultimate_psionics`) + 28 `companion`. This is 100% of bucket A. MEASURED.
- **Cost:** **~0.1–0.6h** (EXTRAPOLATED from `table-build-rate.json`: 0.59h measured wall time for 7 kinds together plus fail-closed proofs, on 208 already-built shared lines). B3 priced it at ~0.038h; I widen that upward because B3's figure is a per-kind marginal that ignores fail-closed proof time, which the ledger bills to the cycle.
- **Why first:** highest ROI on record, lowest risk, and it retires a whole bucket. It is also the cheapest possible rehearsal of the gate.

### Engine 2 — Display-class record placement (the backlog's biggest lever)

- **Mechanisms served:** `class_feature_owner_matched_by_name_but_record_not_held_by_engine` (3,235), `class_feature_option_pool_record_not_held_by_engine` (1,625), `template_content_absent_from_template_table_in_core_essentials` (977), `feat_key_absent_from_catalog` (490), `spell_key_absent_from_spell_list` (391).
- **Units covered:** target population **6,718 of 22,369 (30.0%)**; of these, **6,545 of 11,299 bucket-B units are `wiring_class = display`** and all 3,235 `owner_matched` units carry `magnitude_token_count = 0`. MEASURED.
- **Cost:** **UNMEASURED for this shape at this scale.** Calibration bracket (EXTRAPOLATED): core_rulebook's generic `race_trait_generic/` table fallback cleared 132 units in 70.0 min (113 units/hr) and its "Shape 7" book-wide grant cleared 72 in 45.2 min (95.6/hr); Epic 2's loader cleared 8,014 corpus-wide units for 0.59h of build. Against those, a 6,718-unit placement engine plausibly costs **10–60h of build-and-widen**. Against core_rulebook's *slow* owner-matched rate (0.71 units/hr) the same population costs **~4,500h**. The gap between those two brackets is the whole uncertainty in this proposal.
- **Extra gate condition (mandatory):** this engine is the closest thing in the plan to an SD-31 generic pass. It does not start until §4's card-1 spike returns a measured owner-matched display rate at n ≥ 100 units. If that rate lands below ~5 DONE-units/hr, this engine is a no-go in its current form and the family must be re-decomposed by sub-cause first.

### Engine 3 — `wiring_class` review classifier

- **Mechanisms served:** the six `*_table_holds_zero_magnitude_record_pending_wiring_class_review` families — `template_content` (594), `deity_content` (408), `race_trait_generic` (157), `ability_content` (101), `language_content` (81), `domain_content` (80), plus smaller.
- **Units covered:** **1,446 of 2,587 bucket-D units (55.9%) = 6.5% of the 22,369 remainder.** MEASURED.
- **Cost:** **UNMEASURED.** ASSUMED cheap by structural analogy to the kind-table loader (a labeling pass over records the engine already holds, not new compute). B3 ranked this second on the same reasoning and was equally explicit that it is unpriced.
- **Extra gate condition:** this is the highest relabel risk in the plan — a "wiring class review" can move 1,446 units from D to display-DONE or from D to M and look identical in a bucket total. **G4 must report `→ DONE` counts only.** A pass that produces bucket churn and zero closures is a failed pass, reported as such.

### Engine 4 — Widen `formula_interpreter` / `bonus_stack_reader` / `race_trait_formula_binding` against bucket M

- **Mechanisms served:** `ability_content_table_holds_record_magnitude_not_yet_computed` (1,236), `spell_list_entry_with_resolved_level` (520), `in_catalog_with_corpus_magnitude_but_no_observed_consumer` (471), `race_trait_generic_table_holds_record_magnitude_not_yet_computed` (437), plus 11 smaller.
- **Units covered:** **3,428 of 3,635 bucket-M units are computed-or-derived wiring class (94.3%) = 15.3% of the 22,369 remainder.** MEASURED.
- **Cost:** **UNMEASURED for this slice**; `forward-plan.json .measured_rates.M` is null. EXTRAPOLATED from SD-32's own history: ~9h to build and generalize, then **12+ cycles / ~2 days of widening for a narrower 10-family scope**. Expect worse here — this is 15 families across 35 books. Realistic bracket **40–150h**, low confidence.
- **Extra gate condition:** run G1–G4 on the single largest family (`ability_content`, 1,236 units) alone before committing to the other 14. B3's own gap note is that nobody has verified record-by-record whether these magnitudes are actually formula- or bonus-shaped, i.e. reachable by these engines at all. G3 on family 1 answers that.

### Engine 5 — Oracle probe surfaces for the bucket-V remainder

- **Mechanisms served:** `race_trait_generic_table_holds_record_magnitude_not_yet_computed` in V (142), plus 5 small families (33).
- **Units covered:** **175 of 22,369 (0.8%)** — the genuine post-reconciliation residue, set-verified in `artifacts/bucket-v-widen/AT-34-E3-005_bucket_v_widen_cycle_receipt.md`. MEASURED.
- **Cost:** **UNMEASURED.** The one attempt (core_rulebook's Ranger Favored-Enemy `measure-ranger-fe.pcg`/`.ftl`) was never timed. Requires a new `.pcg`/`.ftl` oracle round-trip per sub-cause against `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.
- **Why last among the engines:** smallest population by an order of magnitude. Its real value is that it is the cleanest possible test of oracle-round-trip cost, which is an input to every other card's G3.

### Explicitly not an engine — bucket C hand-wiring

- **3,981 of 22,369 (17.8%)**, one mechanism, 39 existing hand-written call sites. MEASURED. Carry as a separate judgment-work line. Its only measured rate (41.3 units/hr) is **net-to-V and 0.0 to DONE** — the 96.39h figure buys reclassification, not closure. Any SD-35 plan that shows bucket C as "priced" without that qualifier is misreporting.

### Also not an engine — bucket X capability builds

11 named capabilities in `capability-register.json`, all `built_by_sd34: false`; **53 of 170 corpus-wide X units fall in the 35-book scope (0.2% of the remainder)**. No capability carries an hours estimate anywhere. Handle via §4's spike, not as an engine card.

---

## 4. NO-RATE PLAN — SD-35 card 1

**Card 1: timeboxed rate-measurement spike. Nothing else in SD-35 starts until it reports.**

All five unpriced buckets (D, M, V, X, Z) have **zero measured rate — 5 of 5** (MEASURED; both vehicle books' `step-cost-ledger.json` list D under `buckets_not_yet_cleared`, and real M-clearing cycles ran without recording wall time). The two "illustrative analog" figures in circulation — D at 446–1,552h borrowed from bucket B's rate, M at a ~90h floor borrowed from bucket U's rate — are EXTRAPOLATED-ANALOG, explicitly low-confidence, and must not be summed with each other or with anything else.

**Total timebox: 40 hours. Seven measurements. Every cycle records wall-clock or does not count.**

| # | Bucket / question | Sample | n | Timebox | Success = |
|---|---|---|---:|---:|---|
| 1a | **B composition — the decisive one.** `class_feature_owner_matched_by_name_but_record_not_held_by_engine`, display/zero-magnitude slice | one corpus-wide cycle, records drawn across ≥5 books | ≥100 units | 8h | a DONE-units/hr rate with stated n; settles the 600h-vs-5,300h spread in §1.2 |
| 1b | B composition, second shape: `class_feature_option_pool_record_with_magnitude_not_held_by_engine` (2,715 units, computed/derived) | one corpus-wide cycle | ≥50 units | 6h | a DONE-units/hr rate; also tests whether the 7 sub-causes batch |
| 2 | **D**, sub-family 1: `class_feature_of_unmodelled_corpus_class` (926 units) | one corpus-wide cycle | ≥30 units | 6h | first D rate ever recorded |
| 3 | **D**, sub-family 2: `*_pending_wiring_class_review` (1,446 units) | one classifier cycle | ≥50 units | 4h | rate **and** a `→ DONE` vs relabel split |
| 4 | **M**: `ability_content_table_holds_record_magnitude_not_yet_computed` (1,236 units) | one corpus-wide cycle | ≥50 units | 6h | first M rate; measured to DONE, not to bucket move |
| 5 | **V**: `race_trait` residue (142 units) | one timed oracle round-trip | ≥20 units | 4h | first timed oracle-probe cost — an input to every G3 |
| 6 | **X**: build exactly one capability end to end — `companion_mount_advancement_table` (9 units, the smallest of the 11) | 1 capability | n=1 | 4h | first capability build-time figure in the program |
| 7 | **Z**: `beginner_box` bootstrap probe (19 units, one book, no compiled rule set) | scoping probe only | n=1 book | 2h | a defensible 4h-vs-40h answer, or a stated reason none is obtainable |

**Exit criterion:** every bucket has either a rate with an explicit n and denominator, or a written statement of why no rate is obtainable and what would make one obtainable. A spike that returns "we ran out of time" on any row is reported as an open row, not omitted. No blended cross-bucket total is produced — the rates differ by more than an order of magnitude and averaging them is how the 45.7% figure got its authority in the first place.

**Also in card 1 (mechanical, ~1h):** regenerate `forward-plan.json`, `ordered-plan.json` and `capability-register.json` at HEAD and re-state every SD-35 figure against them.

---

## 5. RISKS — where this assessment is thin

1. **The bucket-B hour figure is unresolved by an order of magnitude.** §1.2 gives 600–1,000h, ~1,468h and ~5,304h from three defensible readings of the same MEASURED population. Nothing in this document settles it. It is the largest single uncertainty in SD-35 and it is 11,299 units — 50.5% of the remainder.
2. **Every rate in the program has n ≤ 2 books.** Bucket B: n=2 cycles across 2 books (1.667–5.8 units/hr) driving a 1,952–6,782h range — a 3.5× spread from two data points. Bucket C: n=1. Bucket U: n=2 cycles, one book. Bucket A: n=7 tables, one book, flagged as an estimate by its own source. B4's fast/slow decomposition: n=29 cycles but **one book**.
3. **B4's composition assumption is measurably wrong and its hour figure inherits that.** I confirmed the decomposition arithmetic and overturned the projection. B4 flagged this itself ("0 of 34 remaining books has a mechanism-level bucket-B breakdown", ASSUMED) — the flag was correct and the census now contradicts the assumption.
4. **The per-book setup tax is unmeasured and there is evidence it is real.** `ultimate_campaign`'s single bucket-B cycle *reused an already-built* generic mechanism and still ran at 1.667 units/hr — the slow end of the range (MEASURED, n=1). Bucket A's own note separates a mechanism's build cost from its per-book wiring cost. Every "corpus-wide cycle" price in §3 assumes that tax is small. Nothing measures it.
5. **All five derivations share one instrument.** B1, B2, B3, B4 and I all read `docs/work-inventory.json` through `scripts/completion_atlas.py::_bucket_of`. Our agreement to the unit proves we ran the same code, not that the code is right. A classifier defect would fool all five identically — the precedent is on record, where a self-consistent instrument chain agreed a book had zero races. **Nobody has audited a sample of the 6,572 `oracle-agree`/`oracle-unverifiable` dispositions that produced the V collapse.** If a material share of them are wrong, the remainder is understated and the biggest good news in this document evaporates. That audit belongs in card 1.
6. **Denominator hazard.** Three populations are in play and they are routinely confused: **22,369** (non-DONE, 35 SD-35 books), **24,475** (non-DONE, all 37 books), **49,438** (all units including DONE). Two of the four lanes mixed 35-book and all-book denominators inside a single table. Every SD-35 figure states which.
7. **The V-collapse framing risk.** If SD-35 planning absorbs B3's "thousands of units in days" reading rather than B2's reconciliation reading, it will price future oracle work at roughly 40× its real throughput. This is the most consequential misreading available from the input material.
8. **This assessment scopes off an unfinished bundle.** SD-34 stands at 20 of 36 kanban rows complete. Sixteen open rows can still move populations, rates, or both.
9. **`oracle_probe_surface_for_no_table_kinds` (2,062 units, `capability-register.json`) may conflict with `decisions.md §19`'s disposition rule.** B3 raised it and could not resolve whether that population is open work. Unresolved here too. If it is open, the V line in §1.1 is wrong.
10. **Nothing here was built or run.** Read-only throughout; the only write is this file. Every cost in §3 is a calibration bracket, not a quote. This is a proposal.
