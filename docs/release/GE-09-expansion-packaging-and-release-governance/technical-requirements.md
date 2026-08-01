# GE-09 Technical Requirements

## Problem statement
Codex needs a governed way to grow after the pilot. Without an explicit expansion and release policy, the program risks making broad support claims from narrow proof, expanding by book order instead of evidence, burying known gaps, and promising package compatibility the authoring stack cannot yet sustain.

GE-09 exists to prevent that failure.

## Current-state facts
- GE-09 already exists as a planning-only source STC and documentary output bundle.
- GE-01 currently grounds a pilot-bounded evidence base: 29 conversion-matrix rows and 13 unsupported-token-ledger entries, which is enough for ranked candidate bands but not for a fully granular expansion backlog.
- GE-05 defines the known-gap truth model that GE-09 must reuse rather than fork.
- GE-06 now has an explicit verdict: `computed-but-not-oracle-checked`, not `pilot-viable`.
- GE06-E5-F2 and GE06-E5-F3 explicitly route the next mandatory proof burden to GE-05 parity ownership and reject broad upstream expansion as the next move.
- GE-08 now has a planning-ready source STC, but no code-authorizing handoff or grounded contributor workflow.
- The quality-gate policy already defines an explicit Expansion gate and compatibility claim tiers.

## Requirement set

### 1. Expansion selection governance
1. GE-09 MUST require expansion candidates to be selected by explicit evidence rather than by book order, nostalgia, or feature enthusiasm.
2. Every proposed expansion candidate or candidate band MUST identify:
   - the token families or source-package domains being expanded
   - the affected conversion-matrix rows or explicit missing-row requirement
   - the current compatibility claim tier for the affected scope
   - known-gap posture and blocking risk
   - expected user value or coverage gain
3. Expansion priority MUST weigh at least these factors:
   - coverage impact on the governed migration plan
   - dependency readiness across importer, engine, oracle, and authoring surfaces
   - risk of misleading compatibility claims
   - documentary or implementation effort required to make the claim honest
4. GE-09 MUST forbid broad statements such as "support more Pathfinder" unless the exact scope is named.
5. While GE-06 remains `computed-but-not-oracle-checked`, GE-09 MUST treat "no scope-broadening launch" as the top-ranked outcome unless new parity evidence explicitly changes the claim ceiling.
6. GE-09 MUST distinguish scope-deepening preparation bands from true scope-broadening candidates so stabilization work is not mislabeled as expansion.
7. GE-09 MUST not present a book-by-book or package-by-package ranking beyond what the current pilot-bounded GE-01 evidence actually supports.

### 2. Release milestone model
1. GE-09 MUST define milestone classes that separate internal proof from broader compatibility claims.
2. Every milestone class MUST name the minimum evidence gates it requires across:
   - documentation
   - headless behavior
   - import fidelity
   - rules correctness
   - oracle parity
   - UI truth when product-visible claims are involved
3. A stronger milestone MUST never be claimed from weaker evidence tiers.
4. Release milestones MUST preserve the distinction between:
   - internal proof
   - controlled preview
   - compatibility-scoped package release
   - broader public-facing release posture
5. GE-09 MUST not assign final release authority in this planning pass; it MUST instead require a later explicit authority decision surface.

### 3. Package compatibility and migration policy
1. GE-09 MUST define compatibility language that is scoped by package, token family, and evidence tier.
2. Compatibility language MUST be narrower than the available evidence and MUST never imply full system support when only selected packages or outputs are proven.
3. GE-09 MUST require versioned package and migration policy surfaces before changes are called compatible across releases.
4. The package policy MUST distinguish:
   - authored package compatibility
   - imported legacy-package compatibility
   - intentionally divergent behavior
   - deprecated or downgraded support
5. Any intentionally divergent compatibility rule MUST point to a later decision record rather than being hidden in release notes.
6. Every compatibility statement MUST declare, at minimum:
   - exact package or bounded source scope
   - package class
   - evidence-tier ceiling
   - known-gap posture
   - migration posture
7. The package policy MUST define separate version surfaces for:
   - authored package-file contract (`schema_version`)
   - authored content revision (`package_version`)
   - stable record identity continuity
8. GE-09 MUST treat version labels as governance markers, not automatic runtime guarantees; if a migration surface or verified loader/translator path is absent, the claim MUST stay `migration-required`, `migration-undefined`, or equivalent rather than silently implying seamless upgrade.
9. Downgrade behavior MUST distinguish claim-tier downgrade from artifact/version rollback and MUST require immediate visible lowering of the claim ceiling when regressions or new known gaps invalidate a stronger posture.
10. While GE-08 remains planning-ready without narrower execution proof, authored-package migration, downgrade, and exchange guarantees MUST remain provisional rather than being narrated as already proven behavior.

### 4. Coverage dashboard requirements
1. GE-09 MUST define a governed dashboard schema that can track expansion truth over time.
2. The dashboard MUST support, at minimum, these fields:
   - token family or content-domain ID
   - source package or book scope
   - current evidence tier ceiling
   - owning spec domain or subsystem
   - known-gap count and severity posture
   - latest verification artifact
   - next review trigger
   - compatibility-language ceiling
3. The dashboard MUST make unverified or downgraded areas visible instead of allowing silence.
4. The dashboard MUST be usable for deciding both where to expand next and what not to claim yet.

### 5. Known-gap and regression governance
1. GE-09 MUST consume the GE-05 known-gap policy instead of inventing a second truth system.
2. Every release or expansion claim MUST identify whether blocking or accepted known gaps exist for the scoped package or token family.
3. Regression findings MUST be treated as first-class release inputs, not post-hoc cleanup.
4. GE-09 MUST require downgrade or block behavior when regression or gap posture invalidates a stronger compatibility claim tier.
5. Known-gap acceptance MUST remain narrow and MUST never broaden the public claim beyond the proven scope.

### 6. Contribution intake governance
1. GE-09 MUST consume the GE-08 source STC as the current authoring-truth boundary rather than behaving as if GE-08 were still absent.
2. Contribution intake MUST remain provisional until GE-08 produces a narrower accepted authoring/execution posture and any required doctrine decisions about contribution or distribution authority exist.
3. Any future package contribution policy MUST require an inspectable deterministic source bundle rather than screenshots, prose, or opaque runtime state.
4. The minimum reviewable submission bundle MUST include, at minimum:
   - `manifest.yaml` with `schema_version`, `package_id`, `package_version`, dependency posture, and `validation_state`
   - package-local stable IDs
   - machine-readable provenance
   - machine-readable diagnostics
   - bounded package scope
   - compatibility-language constraints
   - named review ownership
5. GE-09 MUST consume GE-08's validation-state posture honestly: packages in `draft`, `invalid`, or `deferred` state may be preserved for diagnostic or local work-in-progress purposes, but they MUST NOT be accepted as proof-bearing package intake.
6. GE-09 MUST forbid treating arbitrary scripting or opaque plugins as the default contribution path.
7. Contribution intake MUST distinguish internal operator-authored packages, trusted collaborator packages, and any future broader community intake.
8. Until a narrower accepted intake posture exists, only bounded operator-owned proof-package review may be considered conditionally admissible; trusted-collaborator and broad-community package intake remain closed.

### 7. Cross-platform packaging milestones
1. GE-09 MUST distinguish internal pilot distribution from broader cross-platform packaging claims.
2. Any packaging milestone MUST be tied to the exact target surface being distributed: headless proof artifact, desktop pilot build, package bundle, or broader release.
3. Cross-platform packaging MUST remain blocked from strong promises until the underlying behavior and compatibility claims are already evidence-grounded.
4. Packaging convenience MUST not outrun release truth.

### 8. Packaging and distribution pipeline governance
1. GE-09 MUST treat packaging and distribution as a downstream pipeline that consumes already-bounded truth; it MUST NOT use packaging work to discover or inflate compatibility, release, or authoring claims.
2. Any future packaging/distribution handoff MUST cite, in one place, all of the following prerequisite surfaces for the same exact scope:
   - GE09-E2 package compatibility and migration posture
   - GE09-E3 release milestone and authority posture
   - GE07-E6 cross-platform packaging-risk evidence
   - GE08-E5 product-visible editor/workbench posture when the distributed surface includes authored-package editing or workbench behavior
3. The pipeline MUST distinguish at least these downstream classes:
   - internal proof artifact distribution
   - controlled desktop pilot preview
   - compatibility-scoped package preview
   - release-candidate or supported-release distribution
4. Every packaging/distribution class MUST name:
   - exact target surface being distributed
   - exact package or workflow scope
   - target platform set
   - operator or authority surface
   - installation path
   - rollback or uninstall posture
   - verification receipt bundle
5. GE07-E6 evidence MUST be treated as a blocker ledger and target-platform-risk input, not as proof that desktop packaging works on Linux, Windows, or macOS today.
6. If the distributed thing includes authored-package editing, workbench UX, or product-visible authoring claims, the claim ceiling MUST remain bounded by GE08-E5's first truthful workbench slice plus a narrower GE-08 readiness or execution closure; planning-only GE-08 posture by itself is insufficient.
7. GE-09 MUST refuse any packaging/distribution handoff that lacks an exact scope sentence, exact claim tier, exact migration/downgrade posture, or exact authority class for the thing being shipped.
8. GE-09 MUST route any later implementation or operations successor through a stage-specific readiness closure that names operators, runtime surfaces, verification receipts, and non-goals before a repo, CI, installer, deployment, or publication lane is called honest.

### 9. Dependency posture and current ceilings
1. GE-09 MUST preserve GE-06's explicit verdict exactly: `computed-but-not-oracle-checked`, not `pilot-viable`.
2. GE-09 MUST preserve that GE-08 is planning-ready but still lacks an active code-authorizing handoff or settled contributor workflow.
3. Requirements that depend on stronger parity evidence, product-visible proof, or narrower authoring execution posture MUST be written as provisional or blocked rather than fabricated as settled design.
4. The STC MUST provide a future reconciliation path once GE-05 parity, GE-08 readiness, or doctrine decisions change the posture.

### 10. Review cadence and reranking
1. GE-09 MUST define event-driven rerank triggers for expansion priority rather than relying only on calendar churn.
2. At minimum, rerank triggers MUST include:
   - a GE-05 parity artifact that changes the gap class or evidence ceiling
   - a GE-06 propagated verdict change
   - closure of a high-leverage GE-02 / GE-03 / GE-04 mechanics cluster
   - a GE-08 readiness or doctrine change that alters authoring or contribution posture
   - a GE07-E6 successor receipt that changes platform packaging, install, signing, or rollback feasibility for a named target surface
3. GE-09 MAY define a slower fallback calendar review cadence, but that cadence MUST exist only to catch drift when no event trigger fires.

## Non-goals
- shipping release automation from this STC
- defining final package-signing mechanics
- standing up a public package registry or marketplace
- declaring final plugin ABI or untrusted extension model
- broadening compatibility language beyond the evidence tiers already defined by doctrine

## Required generated documentary outputs
The following documents are required outputs of GE-09 itself:
- `artifacts/expansion-scope-selection-policy.md`
- `artifacts/release-milestone-model.md`
- `artifacts/coverage-dashboard-requirements.md`
- `artifacts/package-compatibility-and-migration-policy.md`
- `artifacts/known-gap-and-regression-governance.md`
- `artifacts/contribution-intake-policy.md`

## Traceability notes
- GE-01 governs expansion truth inputs.
- GE-05 governs known-gap truth inputs and the parity gate that currently caps broadening.
- GE-06 governs the current pilot claim ceiling and now explicitly says that ceiling is `computed-but-not-oracle-checked`.
- GE-08 governs how authored packages and contribution flows can honestly work, but only at a planning-ready level today.
- GE-09 governs how those truths become bounded expansion and release policy without inflation.
