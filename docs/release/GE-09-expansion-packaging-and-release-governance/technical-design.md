# GE-09 Technical Design

## Design purpose
This document describes the planning architecture for GE-09. It does not design a packaging toolchain or release automation system. It designs the governance surfaces that future packaging, release, and expansion work must obey.

## Core design posture
GE-09 is a control-plane epic.

Its job is to turn upstream evidence into narrower downstream permissions:
- GE-01 says what migration coverage exists.
- GE-05 says what known-gap and parity truth exists.
- GE-06 says what the pilot can honestly claim today.
- GE-08 says what authoring and package-editing posture is real.
- GE-09 decides what those truths allow the program to claim, release, expand, or invite contributors to change.

## Current operating truth
The decisive current fact is not that GE-09 lacks upstream inputs.
It now has them.

The decisive fact is that those inputs narrow the answer:
- GE-06 explicitly says the pilot is `computed-but-not-oracle-checked`, not `pilot-viable`.
- GE06-E5-F2 / E5-F3 explicitly say the next mandatory proof burden is GE-05 parity closure, not broad scope expansion.
- GE-08 now exists as a planning-ready source STC, which is enough to constrain contribution posture but not enough to grant contributor workflow or implementation authority.

Therefore GE-09's first truthful output is a ranked planning policy that can say "not yet" with evidence instead of pretending every queued idea is equally eligible.

## Proposed governance surfaces

### 1. Expansion selection surface
A governed selection policy decides which scope may move next and which must wait.

Inputs:
- conversion-matrix coverage
- unsupported-token ledger posture
- known-gap posture
- parity evidence
- pilot claim ceiling
- authoring readiness posture
- operator value/risk judgment

Outputs:
- current hold or go posture
- candidate ranking bands
- blocked candidates with reasons
- review triggers for re-ranking

### 2. Release milestone surface
A milestone model prevents the program from treating every successful test as a release.

The model should separate at least:
- internal proof
- controlled preview
- compatibility-scoped package release
- broader public-facing release posture

Each milestone is a policy shell over evidence, not a marketing label.

### 3. Compatibility language surface
Compatibility must be spoken in exact scope.

Design rule:
- the compatibility language ceiling is derived from the strongest evidence tier actually reached for the named package or token family
- the policy must support downgrade paths when regressions or new gaps appear

### 4. Coverage dashboard surface
The dashboard is the operator-facing mirror of expansion truth.

It should make four things legible at once:
- what is covered
- what is blocked
- what is partially true but still risky to claim
- what should be expanded next only after the gate changes

### 5. Contribution-intake surface
Contribution intake is deliberately downstream of authoring truth.

GE-09 should consume the GE-08 source STC as the current authoring boundary, but leave actual contributor workflow provisional until GE-08 produces a narrower accepted readiness posture and any necessary doctrine decisions exist.

### 6. Packaging and distribution pipeline surface
Packaging is not a discovery mechanism.
It is the downstream conveyor that turns already-bounded truth into a distributable artifact class without widening the claim.

The pipeline must stay ordered like this:
1. GE09-E2 fixes the compatibility sentence for the exact named scope: package class, evidence-tier ceiling, known-gap posture, version surface, and migration/downgrade stance.
2. GE09-E3 fixes the milestone class, receipt bundle, refusal conditions, and authority posture for the same exact scope.
3. GE07-E6 contributes the cross-platform packaging-risk ledger: target platforms, installer/signing/checksum questions, and explicit blocker classes. This is prerequisite evidence about what later packaging work must solve; it is not proof that Linux, Windows, or macOS packaging already works.
4. GE08-E5 defines the first truthful product-visible editor/workbench slice. If the thing being distributed includes authored-package editing or workbench behavior, the later handoff must cite that slice plus a narrower GE-08 readiness or execution closure. Planning-ready GE-08 posture or documentary GE08-E5 alone cannot be retold as runtime proof.
5. Only after those surfaces agree may a later GE09-E6 successor mint a stage-specific implementation-readiness or operations handoff that names operators, runtime surfaces, verification receipts, and non-goals.

This creates four distinct downstream distribution classes:
- internal proof artifact distribution
- controlled desktop pilot preview
- compatibility-scoped package preview
- release-candidate or supported-release distribution

Each class inherits the same upstream order, but the later classes demand stronger receipts. The packaging artifact changes; the truth contract may not.

### 7. Decision-record surface
Certain questions should not be solved inside release notes or acceptance prose.

When the program reaches those questions, GE-09 should route them to doctrine decisions, especially:
- release authority
- intentional divergence with compatibility consequences
- package signing or trust-network policy
- public distribution posture
- legal/licensing constraints that change claim scope

## Evidence flow
The intended decision flow is:

1. upstream artifacts generate evidence or blockers
2. GE-09 classifies what claim ceiling that evidence allows
3. GE-09 either holds scope, reranks candidate bands, or narrows an allowed next move
4. milestone and compatibility policy surfaces decide whether a release or expansion step is even eligible
5. only then may a bounded downstream implementation or operations handoff exist

This order matters. If implementation or distribution comes first, the truth surface collapses.

## Current ranking constraint
The current GE-01 evidence base is still pilot-bounded.
That means GE-09 may truthfully rank:
- scope-deepening preparation bands
- inventoried adjacent-domain broadening bands

It may not truthfully rank a detailed future package backlog beyond those grounded bands.

## Reconciliation path
GE-09 is no longer waiting for GE-06 or GE-08 to exist.
It is waiting for stronger downstream truth.

The next reconciliation pass should:
1. read the latest GE-05 parity artifacts
2. confirm whether GE-06's propagated claim ceiling changed
3. read the latest GE07-E6 platform-risk receipt and any later platform-specific build/install/signing receipts for the target distribution surface
4. read GE08-E5 plus any narrower GE-08 readiness closures or doctrine decisions that affect contribution posture or product-visible authoring/workbench claims
5. rerank candidate bands and claim ceilings before deriving any downstream handoff

## Anti-design rules
- Do not design a release bot before the authority model exists.
- Do not treat packaging research or a built artifact as proof that distribution is now honest.
- Do not design public package workflow before authoring posture exists.
- Do not let package-format convenience redefine compatibility truth.
- Do not let dashboard optimism outrun the underlying evidence.
- Do not let "expansion" become a synonym for "do more books."
- Do not turn a pilot-bounded matrix into a counterfeit global roadmap.

## Success shape
GE-09 succeeds when future sessions can answer:
- Why is broadening scope blocked or allowed right now?
- Which candidate band is ranked next, and why?
- What exact evidence tier allows this claim?
- What known gaps still constrain the package?
- What milestone class is honestly reachable now?
- Which upstream gate is still blocking packaging or distribution for this exact surface?
- What decisions remain blocked until parity, contributor posture, or release authority is stronger?

If those answers are visible, GE-09 is doing its job.


## Design Addendum — 2026-08-02 — per-class coverage evidence and the wiring-class audit

**Decision.** GE-09 reports coverage per `wiring_class`, with class-appropriate evidence, and owns the audit that keeps the axis from being used to inflate the headline number. The field contract, the per-class definition of `proven`, the aggregation rules, and audit checks A1–A6 are specified in `artifacts/coverage-dashboard-requirements.md` §"Unit wiring-class reporting". The class vocabulary itself is GE-01's, at `../GE-01-legacy-corpus-and-conversion-matrix/artifacts/wiring-class-determination.md`, and is cited rather than restated.

**Why GE-09 rather than the generator.** Determining a unit's class is a corpus question (GE-01). Deciding what evidence that class must produce before a coverage claim may cite it is a claim-ceiling question, which is this bundle's subject. Letting the generator decide both would put the definition of `proven` inside the tool that reports it.

**The rule this addendum most needs to survive.** A single aggregate coverage percentage may not be published alone; the `computed`-class figure must appear beside it. Measured: of `core_rulebook`'s 4,743 held units, 777 (16.4%) genuinely require bespoke wiring, 3,856 are reachable by three mechanical checks, and 110 must first be disambiguated. An aggregate that mixes them lets the cheap majority bury the expensive minority and read as near-complete while every hard record is untouched. This is the existing `Minimum truth rules for aggregated views` prohibition — no summary may imply a green state the evidence does not support — restated for a new axis.

**The gaming risk, named.** Reclassifying a `computed` unit as `derived` or `static` dodges the wiring bar and moves the number without doing work. It requires editing only the determinator and it looks like a refinement. The classes form a strict lattice collapsed highest-bar-wins, so a downgrade can only happen by deleting a signal; A1–A6 make signal deletion observable and blocking, and require every unit to carry a determinator version and a source-row digest so that "the corpus changed" is always distinguishable from "the rules changed".

**Cross-bundle finding, recorded not actioned.** SD-28's `epic-14-harness` is specified as widening the observation harness for the ~4,050 `ingested-magnitude` units. Under this taxonomy that bucket is 63.3% `static`, 22.2% `derived`, 12.6% `computed`, 1.5% `ambiguous` — four workstreams with four different acceptance tests, not one harness. The finding is written up in `artifacts/coverage-dashboard-requirements.md` §"Cross-reference: impact on SD-28's completion epics" as a recommendation to that package's owner. GE-09 does not modify SD-28.

**Audit correction, recorded because the mistake is instructive.** The first draft of this addendum's check A5 specified a *symmetric* agreement floor between `display` and the generator's `magnitude_token_count == 0` rule, at the then-measured 99.1%. That was wrong. The two components shared a blind spot — `.MOD` rows and `BENEFIT:` prose both carry magnitudes neither could see — so their agreement measured a shared assumption, not correctness. Real agreement is 89.8%, and the divergence is the fix. A symmetric floor would have made the *corrected* determinator fail the audit. A5 is now one-directional: no unit with `magnitude_token_count > 0` may ever be `display` (0 violations across 9,828 held units). **A cross-check between two components built on the same assumption will always look like confirmation.**

**Evidence.** Every figure re-derived 2026-08-02 from `docs/work-inventory.json` (`generated_at 2026-08-02T04:02:12Z`); commands recorded inline in the artifact and in GE-01's determination artifact.
