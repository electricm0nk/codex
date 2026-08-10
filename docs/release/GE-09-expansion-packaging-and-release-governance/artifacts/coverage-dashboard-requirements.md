---
title: GE-09 Coverage Dashboard Requirements
stc_id: STC-CODEX-GE-09
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-09-expansion-packaging-and-release-governance/artifacts
source_stc: ../README.md
related:
  - ../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv
  - ../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/unsupported-token-ledger.csv
  - ../../GE-05-oracle-validation-and-parity-harness/artifacts/known-gap-policy.md
  - ../../../doctrine/quality-gate-policy.md
---

# GE-09 Coverage Dashboard Requirements

## Purpose
Define the governed field contract for any future GE-09 dashboard, CSV, or evidence ledger so package, token-family, evidence-tier, compatibility-ceiling, and known-gap posture remain legible without turning reporting into a substitute for proof.

## Evidence basis for this planning pass
The field contract is grounded in the current evidence surfaces, not in hypothetical future automation:
- GE-01 conversion matrix currently provides 29 pilot-bounded rows of token-family and construct coverage truth.
- GE-01 unsupported-token ledger currently provides 13 tracked unresolved or deferred entries with explicit severities and owners.
- GE-05 defines the authoritative known-gap classes and required ledger/reporting behavior.
- The quality-gate policy defines the evidence-gate stack and compatibility claim tiers (`Observed`, `Parsed`, `Converted`, `Computed`, `Oracle-checked`, `Product-visible`).
- GE-06 currently caps the integrated pilot claim ceiling at `computed-but-not-oracle-checked`.
- GE-08 is planning-ready but does not yet authorize stronger authored-package or contribution claims.

## Governing design rules
- The dashboard is an evidence index, not a release decision engine and not a substitute for the underlying artifacts.
- Unknown, blocked, deferred, or downgraded states must be visible as first-class values rather than inferred from missing rows.
- Every row must link to concrete evidence artifacts or doctrine surfaces; status-only rows are invalid.
- The dashboard must support both expansion selection and claim refusal by making ceilings, blockers, and missing proof visible.
- Aggregated summaries may exist, but the canonical data model must remain inspectable at row level.

## Required row scopes
A future implementation may render different views, but the underlying ledger MUST support at least these row scopes:

| Row scope | Purpose | Minimum granularity rule |
|---|---|---|
| `token-family` | Track migration or rules coverage for a distinct construct family. | One row per governed token family or construct cluster that can carry its own evidence ceiling. |
| `package-scope` | Track a source package, book, or bounded content domain. | One row per package or adjacent-domain band that could be named in a compatibility or expansion claim. |
| `candidate-band` | Track ranked expansion or scope-deepening candidate bands. | One row per GE-09 candidate band when the band spans multiple token families or packages. |
| `known-gap-cluster` | Surface grouped unresolved debt that caps claims across multiple rows. | One row per reusable blocker cluster when the same unresolved mechanics or parity debt affects multiple scopes. |

A single UI may collapse these views, but the stored model must preserve which scope class each row represents.

## Required fields
Every dashboard row MUST include the following fields.

### 1. Identity and scope fields

| Field | Required meaning | Notes |
|---|---|---|
| `row_id` | Stable unique identifier for the row. | Must survive sorting or UI changes. |
| `row_scope` | One of `token-family`, `package-scope`, `candidate-band`, `known-gap-cluster`. | Required for downstream interpretation. |
| `scope_key` | Canonical short key for the governed scope. | Example shapes: `fighter-progression-formulas`, `core-rulebook-adjacent-domain-cluster`, `human-race-trait-composition`. |
| `scope_label` | Human-readable scope name. | Must be explicit enough to appear in a report or review packet. |
| `source_package_or_book` | The exact package, book, or domain source being discussed. | Multi-source rows must name the aggregation rule. |
| `token_family_or_domain` | The token family, mechanics cluster, or content domain represented. | Must not collapse unrelated mechanics into a vague bucket. |
| `in_pilot_scope` | Whether the row is inside the current GE-06 pilot boundary. | Boolean or equivalent explicit classification. |
| `scope_classification` | One of `pilot-core`, `pilot-adjacent`, `scope-deepening`, `scope-broadening`, `authoring-linked`, `future-release-only`. | Prevents stabilization work from being mislabeled as expansion. |

### 2. Evidence and claim fields

| Field | Required meaning | Notes |
|---|---|---|
| `evidence_tier_ceiling` | Highest quality-gate claim tier actually proven for this row. | Must reuse doctrine tiers, not invent new ones. |
| `claim_ceiling_phrase` | Human-readable compatibility ceiling for the row. | Example: `computed but not oracle-checked for pilot-bounded Fighter progression`. |
| `gate_posture` | Current posture across documentation, import, rules, oracle, and UI truth gates. | May be structured or summarized, but must preserve failing gates. |
| `latest_verification_artifact` | Most recent artifact proving the current ceiling. | Must be a path or immutable reference, not free text alone. |
| `evidence_date` | Date of the latest verification artifact. | Required for drift review. |
| `verification_basis_class` | One of `matrix-row`, `unsupported-ledger`, `parity-artifact`, `decision-record`, `manual-review`, `multi-source`. | Makes evidence provenance legible. |
| `compatibility_language_ceiling` | Strongest allowed external wording for this row. | Must become narrower when evidence is weaker. |
| `downgrade_required` | Whether the row is currently under downgrade pressure relative to prior claims. | Must not require the reader to infer this from comments. |

### 3. Known-gap and regression fields

| Field | Required meaning | Notes |
|---|---|---|
| `known_gap_count` | Count of known gaps affecting the row. | Zero must be explicit, not implied. |
| `highest_gap_severity` | Highest active severity posture affecting the row. | Must align with GE-01/GE-05 governance surfaces. |
| `blocking_gap_classes` | Gap classes from GE-05 that currently block a stronger claim. | Use exact GE-05 class names where applicable. |
| `accepted_gap_classes` | Gap classes visible but currently tolerated within the named claim boundary. | Cannot silently broaden the claim. |
| `regression_state` | One of `none-known`, `suspected`, `confirmed`, `downgraded`, `blocked`. | Makes regression posture first-class. |
| `regression_artifact` | Evidence reference for the newest regression finding, if any. | Empty only when `regression_state` is `none-known`. |
| `block_condition_summary` | Compact statement of what must remain false before a stronger claim is allowed. | Example: `blocked until oracle parity exists for scoped outputs`. |

### 4. Ownership and action fields

| Field | Required meaning | Notes |
|---|---|---|
| `owning_surface` | Primary owning GE, subsystem, or doctrine surface. | Must identify who can actually change the posture. |
| `downstream_owner` | The next work lane or authority surface expected to act. | Can name a future GE slice rather than a person. |
| `next_honest_move` | The smallest truthful next action for the row. | Example: `maintain hold`, `expand matrix coverage`, `run parity comparison`, `mint narrower authoring policy`. |
| `review_trigger` | Named event that forces re-review. | A row without this is governance theater. |
| `review_status` | One of `current`, `needs-review`, `blocked`, `deferred`, `superseded`. | Must remain explicit even in aggregate views. |
| `last_reviewed_at` | Date the row was last materially reviewed. | Required for drift detection. |

## Required derived classifications
A future implementation MUST support at least the following derived classifications so humans can filter without rewriting doctrine:
- by evidence tier ceiling
- by compatibility-language ceiling
- by scope classification (`pilot-core`, `pilot-adjacent`, `scope-deepening`, `scope-broadening`, `authoring-linked`, `future-release-only`)
- by owning surface (GE-01, GE-05, GE-06, GE-08, GE-09, doctrine)
- by regression state
- by block posture (`unblocked`, `known-gap-limited`, `downgrade-required`, `blocked`)
- by candidate rank or hold status for expansion review

## Review-trigger contract
Each row MUST name at least one event-driven review trigger. A future dashboard implementation must support these trigger classes at minimum:

| Trigger class | Required effect |
|---|---|
| New GE-05 parity artifact changes a gap class, evidence tier, or comparison outcome. | Recompute the row's evidence ceiling and compatibility-language ceiling. |
| GE-06 propagated posture changes. | Re-evaluate whether scope-deepening or scope-broadening remains blocked. |
| GE-01 matrix coverage expands or unsupported-ledger posture changes. | Re-rank candidate bands and refresh package/token-family counts. |
| A regression artifact lands for a previously claimed scope. | Force downgrade-or-block review before the prior claim survives. |
| A doctrine decision records intentional divergence or authority change. | Update claim wording, gap posture, and downstream obligations. |
| GE-08 readiness or contribution posture changes. | Re-evaluate authored-package and contribution-linked rows. |
| Calendar drift threshold passes with no event trigger. | Require stale-row review; this is fallback only, never the primary governance model. |

## Minimum truth rules for aggregated views
If a future UI shows summary cards, charts, or color bands, those summaries MUST preserve these truths:
- the number of rows at each evidence ceiling
- which rows are blocked by known gaps or regressions
- which rows are merely `Computed` and not `Oracle-checked`
- which rows fall outside pilot scope and are therefore not eligible for broad compatibility language
- which rows represent scope-deepening stabilization versus true scope broadening

Any summary that can turn `computed-but-not-oracle-checked` into an implied green state is invalid.

## Implementation guardrails for future builders
- CSV, SQLite, Markdown ledger, and UI dashboard implementations are all acceptable as long as they preserve the required fields and doctrine.
- Missing evidence links, missing review triggers, or implied zero-gap rows must fail validation.
- Automated rollups may summarize but must never delete row-level provenance.
- GE-09 reporting surfaces must consume GE-05 known-gap truth and GE-01 evidence truth; they may not fork either model.

## Unit wiring-class reporting (added 2026-08-02)

GE-01 classifies every imported rule record on a second axis, `wiring_class`, orthogonal to the work-inventory `status` axis. The class vocabulary — `display`, `static`, `derived`, `computed`, plus `ambiguous` for determination failure — and its mechanical determination from the PCGen record are defined once, at `../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/wiring-class-determination.md`. GE-02 carries the field on the canonical object; GE-04 owns the evaluator the `derived` class needs. **This section defines only what GE-09 owns: what proven means per class, how the aggregate is computed, and the audit that stops the axis being used to inflate coverage.** It does not restate the class definitions.

### Why this section exists
One axis was doing two jobs. `grounded` means *a real computed magnitude was OBSERVED reaching a consumer*, a test only a bespoke-wired magnitude can pass. Every record whose magnitude is a plain function of caster level therefore stalls in `ingested-magnitude` and reads as unfinished even when the engine is entirely right about it. Re-derived 2026-08-02 from `docs/work-inventory.json` (`generated_at 2026-08-02T04:02:12Z`): 4,050 units corpus-wide sit there, 3,062 of them in `core_rulebook`, which reports 912 of 5,716 units proven (16.0%) while holding 3,981 (69.6%). Reporting the same evidence bar for all four kinds of record is the defect; reporting a *lower* bar for all four would be the over-claim. Per-class evidence is the only honest fix.

### Required fields, added to every unit-scoped row

| Field | Required meaning | Notes |
|---|---|---|
| `wiring_class` | One of `display`, `static`, `derived`, `computed`, `ambiguous`. | GE-01 is the authority. Never inferred from `status`. |
| `wiring_class_signals` | Full signal set from the source record, e.g. `["derived:bonus","computed:pre_guard"]`. | Mandatory. The audit below is impossible without it. |
| `wiring_class_determinator_version` | Version of the determination ruleset that produced the class. | A class value with no ruleset version is unauditable. |
| `source_row_digest` | Digest of the unit's whole **token closure** — its base row plus every `.MOD` row targeting it — not of the base row alone. | The discriminator between "the content changed" and "the rules changed". A digest over the base row alone would miss a `.MOD` row edit, which is where 8,234 corpus magnitudes live. |
| `upstream_implementation_marker` | Whether the legacy record carries an upstream not-implemented admission (PCGen's `[Not Implemented]` prefix in `DESC:`). | Reported beside `wiring_class`; MUST NOT feed it or `proven` in either direction. See the conflation rule below. |

### What `proven` means, per class

| `wiring_class` | proven when | evidence artifact |
|---|---|---|
| `display` | the engine holds the record and its description renders on the surface the player reads | render assertion naming the surface |
| `static` | the stored value is equal to the corpus literal **and** that value reaches a consumer or a rendered field | literal-equality check plus the naming of the consumer/field |
| `derived` | the GE-04 scalar-derived evaluator returns the correct value **at sampled inputs**, with `dependencies` populated | evaluation fixture, sampled at ≥3 inputs including any formula cap boundary |
| `computed` | a real consumer observes a delta — **today's `grounded` bar, unchanged and not weakened** | the existing grounded evidence |
| `ambiguous` | **never.** An `ambiguous` unit is not provable while ambiguous. | — |

Three rules bind these:

- **`static` requires a consumer or a rendered field, not merely a stored value.** Without that clause `static` becomes "the number is in a table somewhere", which is precisely the over-claim `ingested-magnitude` was minted to prevent. 3,050 held units are `static` — the second-largest class — so a weak bar here moves the headline number more than any other single change.
- **`derived` requires sampling, not one evaluation.** A formula correct at level 1 and wrong at level 11 is the failure this class exists to catch, and `min(10,CASTERLEVEL)`-shaped caps make the cap boundary a required sample.
- **The `computed` bar does not move.** No unit reaches proven by being called `derived` when its magnitude is guarded, temporary, or choice-driven.

### Aggregation

- `proven_units` = the sum of units meeting their own class's bar. Nothing else counts. `ambiguous` units count toward the denominator and never toward the numerator.
- **A single aggregate coverage percentage MUST NOT be published alone.** Every aggregate is published as a vector — proven and total per class — and any headline figure MUST appear adjacent to the `computed`-class figure computed on its own. Rationale, measured: of `core_rulebook`'s 4,743 held units only 777 (16.4%) are `computed`; another 3,856 are reachable by three mechanical checks and 110 must first be disambiguated. An aggregate that mixes them lets 3,856 cheap units bury 777 expensive ones and read as near-complete while every hard record is untouched. The existing rule that *any summary that can turn `computed-but-not-oracle-checked` into an implied green state is invalid* extends verbatim to this axis.
- Class distribution MUST be reported per book, not only corpus-wide. Books differ sharply: `core_rulebook` is 47.7% `static` (2,264 of 4,743) while `advanced_class_guide` is 57.7% `display` (1,458 of 2,527).
- The `ambiguous` count MUST be shown on every view that shows the other four. It is a work item — an unresolved construct in exactly the sense this bundle already governs — not a rounding error.

Reproduce the distribution:

```
$ python3 docs/release/GE-01-legacy-corpus-and-conversion-matrix/artifacts/wiring-class-determination.py HELD
inventory docs/work-inventory.json generated_at 2026-08-02T04:02:12Z
scope HELD  n=9828
  display      3599   36.6%
  static       3050   31.0%
  computed     1695   17.2%
  derived      1224   12.5%
  ambiguous     260    2.6%
  dual-signal (derived AND computed) 470
  carrying upstream '[Not Implemented]' marker 0 (reported, never classifying)
per book:
   book                         held  display  static  derived  computed ambiguous
   core_rulebook                4743      920    2264      672       777       110
   advanced_class_guide         2527     1458     283      211       518        57
   advanced_players_guide       2466     1220     500      301       398        47
   bestiary                       46        1       3       40         2         0
   core_essentials                46        0       0        0         0        46
```

### Transition rule
Introducing this axis MUST NOT move a single unit into `proven` on the day it lands. The classes describe *remaining work*; the evaluators still have to be built and run. Two consequences:

- Until GE-04's evaluator exists, every `derived` unit is unproven regardless of how obviously correct its formula looks.
- The reverse correction is due immediately, and it is larger than it first appeared. **243 of the 2,390 units currently counted `text-complete` — 10.2%, and therefore counted `proven` — carry a magnitude the `magnitude_token_count == 0` test cannot see**: 110 `ambiguous` (a magnitude stated only in prose, largely on a `.MOD BENEFIT:` row), 74 `static` and 27 `computed` (a magnitude on a `.MOD` row, which produces no unit of its own), 32 `derived` (the level-scaling `RANGE:Close` keyword, or a parenthesised `CASTERLEVEL` expression). These leave `proven` when the axis lands; 133 return on their own class's evidence, and the 110 `ambiguous` cannot return until the underlying records carry a machine-readable magnitude. A taxonomy that only ever moves units *into* proven is not a measurement.

### Anti-gaming audit

The gaming risk is specific and worth naming plainly: **reclassifying a `computed` unit as `derived`, `static`, or `display` dodges the wiring bar and moves the headline number without doing any work.** It is the cheapest available way to fake progress under this design, it requires editing only the determinator, and it looks like a refinement. The audit below is mandatory, not advisory, and it is a release gate.

The audit rests on one structural property: the classes form a strict lattice `display < static < derived < computed`, collapsed highest-bar-wins, so **a unit can only be downgraded by deleting a signal.** Signal deletion is observable.

| # | Check | Trigger | Required effect |
|---|---|---|---|
| A1 | **Downgrade ledger.** Diff `wiring_class` per unit against the previous run. Any move down the lattice is a finding. | every regeneration | Finding must be resolved by exactly one of: (a) `source_row_digest` changed — the corpus row itself changed, evidence attached; or (b) `wiring_class_determinator_version` changed — then it is a **definition change**, not an observation, and requires recorded approval before the run's numbers may be published. An unexplained downgrade blocks the run. |
| A2 | **Signal-deletion detector.** Diff `wiring_class_signals` per unit. A signal present in run *N* and absent in run *N+1* with an unchanged `source_row_digest` is a determinator regression. | every regeneration | Blocking. This catches the downgrade mechanism itself, including on dual-class units where `wiring_class` did not visibly move. |
| A3 | **`computed` ratchet, per book.** The count of `computed` units in a book may not decrease. | every regeneration | A decrease is blocking unless A1 justified every constituent unit individually. Aggregate justification is not accepted. |
| A4 | **Determinator-diff review.** Any change to the determination ruleset is reviewed line by line and every hunk classified as *added observation* or *changed definition*. | any determinator change | A changed definition requires explicit recorded approval. This mirrors the review already required of the work-inventory generator, and applies for the same reason: the component that measures is the component a dishonest run edits. |
| A5 | **One-directional token-count invariant: no unit with `magnitude_token_count > 0` may ever be classified `display`.** Verified at 0 violations across all 9,828 held units. | every regeneration | Any violation is blocking. **This check is deliberately one-directional.** An earlier draft specified a symmetric agreement floor against the generator's `magnitude_token_count == 0` rule — 99.1% at the time. That was wrong: the two components shared a blind spot (`.MOD` rows and `BENEFIT:` prose), so their agreement measured a shared assumption rather than correctness. Real agreement is 89.8%, and the 10.2% divergence is the fix, not a regression. A floor on symmetric agreement would have made the correct determinator fail the audit. |
| A7 | **Closure integrity.** `source_row_digest` must cover the unit's whole token closure, and the determinator's `.MOD` base-name resolution must match the work-inventory generator's. | any change to either component | Blocking on divergence. If the two resolve `.MOD` targets differently they will disagree about which rows govern a unit, and every downstream diff becomes noise. |
| A6 | **`ambiguous` may not be drained silently.** A unit leaving `ambiguous` requires either a changed corpus row or an approved determinator change, per A1. | every regeneration | `ambiguous` is the honest bucket; quietly emptying it into `display` or `static` is a downgrade by another name. 260 held units sit there today, 110 of them currently counted `proven`. |

Two supporting requirements make the audit runnable rather than aspirational:

- **Determination must be deterministic and versioned.** Given the same corpus row and the same determinator version, the class and signal set are identical. Without that, every diff in A1/A2 is noise and the audit degrades to a review of opinions.
- **`source_row_digest` is mandatory, and it digests the token closure.** It is the only thing that distinguishes "the corpus changed" from "the rules changed", and that distinction is the entire audit. Digesting the base row alone would leave `.MOD` edits invisible — which is where 8,234 corpus magnitudes live.

### Upstream completeness is a separate claim and MUST NOT be conflated

PCGen's stock data marks some records as not mechanically implemented upstream by prefixing `DESC:` with `[Not Implemented]` — every one of `ultimate_campaign`'s 23 story feats carries it. Reporting MUST keep that distinct from our own coverage:

- `upstream_implementation_marker` is reported beside `wiring_class` and **never feeds it, `proven`, or any coverage figure**, in either direction. *Accursed* is marked `[Not Implemented]` upstream and still carries a fully specified benefit formula (`BENEFIT:You gain spell resistance equal to 5 + your character level`), so the marker predicts nothing about our evidence bar.
- A unit MUST NOT be reported as done on the strength of a `[Not Implemented]` description alone. Rendering upstream's admission of incompleteness is not rendering the record's benefit.
- A unit may legitimately be complete on our side — we render the accurate benefit text, it is not a stub — while upstream considers it unimplemented. A view that merges the two will read an upstream gap as our own, or our completeness as upstream's. Both errors are silent.

### Cross-reference: impact on SD-28's completion epics (recommendation only)

`docs/release/SD-28-ultimate-book-content-ingestion/` is owned by another actor. This is a finding for that owner, recorded here because GE-09 owns the reporting surface it affects. **Nothing in SD-28 is modified by this artifact.**

- **`epic-14-harness` is under-specified as written.** SD-28 `decisions.md §32` correctly identifies Epic 14 (observation-harness widening) as a gating prerequisite for roughly 4,050 `ingested-magnitude` units and a hard dependency of Epics 23, 25 and 28, on the grounds that `classify()`'s `Kind::Spell` and `Kind::Equipment` arms have no probe at all. That diagnosis is right. The prescription — *widen the observation harness* — fits only part of the work. Under this taxonomy those 4,050 units decompose as: **2,565 `static` (63.3%), 898 `derived` (22.2%), 511 `computed` (12.6%), 60 `ambiguous`, 16 `display`** (`python3 docs/release/GE-01-legacy-corpus-and-conversion-matrix/artifacts/wiring-class-determination.py ingested-magnitude`). Only the 511 `computed` units need a widened observation harness. The 898 `derived` units need a **formula evaluator** — GE-04's scalar-derived magnitude evaluator, a different component with a different acceptance test — and the 2,565 `static` units need neither: they need a literal-equality check against the corpus row plus a named consumer. The 60 `ambiguous` units need a corpus fix before any of the three can touch them. **Four workstreams, not one harness.** Scoping them all as "harness widening" makes Epic 14 look like one task and will under-cost it by roughly eight to one.
- **This taxonomy strengthens §32's anti-gaming rule rather than relaxing it.** §32 forbids reaching a target by *reclassifying units, relaxing the classifier, broadening what counts as text-complete, weakening or skipping a gate, or editing the work-inventory generator to report more favourably*, and names Epic 14 as the sharpest gaming risk in the set precisely because widening the harness is what a dishonest run would do. `wiring_class` adds a second surface with the same risk profile, and A1–A6 above are its Epic-30-equivalent. `wiring_class` is **not** a licence to move a unit out of `ingested-magnitude`: 12.6% of that bucket is `computed` and stays on the observed-delta bar, and no unit becomes proven until its own class's evidence exists.
- **§27's display-value discriminator is the prose form of this axis.** SD-28 `decisions.md §27` already rules that a record whose value derives from data the engine holds is display-value work, not engine work, and requires a cycle to name *which input the engine does not have* before deferring. `wiring_class` applies that same test from the token shape instead of per-record human judgement, which is what makes it auditable at 44,191 units. The two should be reconciled by SD-28's owner, not by parallel drift.
- **One correction the owner should carry, and it is not small.** §32 states `proven = grounded + text-complete`. **243 of the 2,390 `text-complete` units (10.2%) carry a magnitude the `magnitude_token_count == 0` test cannot see** (detail and breakdown in the Transition rule above). Any 100%-proven target computed on today's `text-complete` set inherits that over-claim, and it compounds: `text-complete` is the cheapest path to `proven`, so a bundle chasing a 100% target has the strongest possible incentive to route units into it.
- **`.MOD`-carried magnitudes are a corpus-wide exposure, not an `ultimate_campaign` quirk.** The case that surfaced this — every `ultimate_campaign` unit reporting `magnitude_token_count: 0` while `Accursed`'s `.MOD BENEFIT:` row states *"spell resistance equal to 5 + your character level"* — is one instance of a pattern spanning **8,234 `.MOD` rows carrying a magnitude token**, touching 1,895 of the 9,828 held units. Any per-book completion epic that reads `magnitude_token_count` to decide what work remains will under-count that book's real magnitude surface. Credit to the `epic-13-calibration` actor, who found it in the field; verified independently here.
- **`[Not Implemented]` must not be read as our own status.** All 23 `ultimate_campaign` records carry PCGen's upstream `[Not Implemented]` marker. It is an upstream-completeness claim and is specified above as a separately-reported field that never feeds `wiring_class` or `proven`. A completion epic that treats the marker as a blocker will defer work that is genuinely doable; one that treats a `[Not Implemented]` description as sufficient to render will ship a stub.

## Completion rule
This requirement artifact is complete for the planning pass when a future builder can implement a governed dashboard or evidence ledger without inventing the row classes, field meanings, compatibility ceilings, or review triggers that control GE-09 truth.