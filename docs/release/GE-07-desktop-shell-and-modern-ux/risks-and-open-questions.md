# GE-07 Risks and Open Questions

## Resolved by this pass
- The soft blocker on documentary drafting is resolved: Todd explicitly authorized early GE-07 source-STC creation on 2026-06-21.
- The route ambiguity is repaired: GE-07 now has a planning-ready source STC plus an active GE07-E1 stage-specific code-authorizing handoff in `awaiting-todd-merge` state. Downstream GE07-E2 through GE07-E6 remain non-authorizing until their prerequisites become repo truth.
- The same-epic documentary outputs are no longer abstract classes; they now have exact artifact paths.
- The live repo/toolchain posture is now grounded for GE07-E1: `origin/develop` still has no desktop shell scaffold, but a branch-ready scaffold now exists on `ge07-e1-desktop-shell-scaffold` at `48892249d5573927bf23a7e47a6d7d6a742da664`, and the runtime does have Rust and Node toolchains available.
- The smallest additive shell scaffold shape and first runtime-boundary ADR inputs are now recorded as explicit GE07-E1 artifacts rather than being implied in prose.
- The minimum pilot workspace truth burden is now explicit through `artifacts/ge07-e3-ui-truth-verification-receipt-2026-06-22.md`, which captures current selections plus computed and blocked route examples over live pilot data.
- The current explanation/diagnostics truth burden is now explicit through `artifacts/ge07-e4-execution-readiness-closure-2026-06-22.md` plus `artifacts/ge07-e4-explanation-diagnostics-visibility-receipt-2026-06-22.md`, which ground live explanation detail along with blocked-route, validation, and importer diagnostics while still refusing counterfeit shell code authority.
- The current pilot rules/source-package inspection burden is now explicit through `artifacts/ge07-e5-execution-readiness-closure-2026-06-22.md`, which grounds the raw rule identities, lineage carriers, and cross-link obligation back into the active character path without inventing code authority.
- GE07-E6 now has a platform-risk receipt and refreshed constraint ledger grounding Linux/Windows/macOS packaging blockers without claiming ship readiness.

## Hard strategic blocker still in force
### GE-06 has a viability decision, but not a product-truth verdict
GE-07 no longer lacks a GE-06 decision surface. It now inherits an explicit downstream posture of `computed-but-not-oracle-checked`.

That clears the stale planning blocker, but broad product-visible UI implementation remains blocked. Any early GE-07 code move must still stay inside a bounded non-production spike with exact repo paths, write scope, and verification receipts.

## Open questions
### O1. Minimum pilot shell
What is the minimum GE-07 shell that proves product direction without creating a broad UI program inside the pilot?

### O2. Command transport choice
Will the first shell use Tauri commands directly, an internal RPC-like boundary, a service abstraction over the Rust core, or another explicit transport?

### O3. Frontend binding finality
The current preference is TypeScript plus React, but does that remain the final choice once a shell spike or ADR is performed?

### O4. Diagnostics audience split
Which diagnostics must always remain visible to ordinary users, and which may be hidden behind developer-focused detail layers without violating the quality gate?

Current answer from the GE07-E4 closure: blocked-route, validation, and importer diagnostics already have grounded structured payloads and therefore cannot be reduced to a single generic warning surface, but the exact ordinary-user vs. expert-detail layering is still open.

### O5. Explanation presentation depth
What explanation presentation is sufficient for the first pilot: inline, drawer, split view, graph overlay, or a narrower first cut?

Current answer from the GE07-E4 closure: the first shell must at minimum preserve the current upstream explanation detail strings and withheld-explanation behavior on blocked values, but the exact presentation container remains open.

### O5a. Invalid-choice reason ownership
Does the first truthful GE07-E4 coding lane consume an upstream invalid-choice/prerequisite-reason payload from GE-04, or must a separate upstream rules-core slice be completed before shell-side explanation/diagnostic work can be authorized?

### O6. Packaging and signing scope
Which platform packaging/signing obligations are required before the first real implementation slice, and which can remain deferred until after a bounded pilot shell exists?

Current answer from the GE07-E6 spike: keep packaging documentary until a real shell slice exists, then prove one platform at a time with explicit build/signing receipts instead of broad readiness claims.

### O7. Local storage boundary
What local persistence/cache surfaces will the shell need immediately, and what should stay deliberately absent until the core/state model is more mature?

## Risks
### R1. UI-first drift
If GE-07 becomes a large shell/buildout before GE-06 proves the domain path, the project may optimize presentation over truth.

### R2. Semantic duplication in the UI
If frontend code starts recomputing rules behavior or explanation logic, Codex will create two competing authorities and lose explainability integrity.

### R3. Diagnostics suppression
A product-polish instinct may push warnings, unsupported-token notices, or provenance details out of sight, which would violate the migration doctrine.

### R4. Framework overcommitment too early
Declaring final UI stack details before a bounded shell spike or concrete repo scaffold may create rework or false certainty.

### R5. Cross-platform surprise costs
Packaging, webview/runtime behavior, signing, native-dialog behavior, or SQLite/file-path differences may create late surprises if ignored now.

## Forbidden assumptions
- GE-06 has already proven the stack.
- React is irrevocably final without an accepted decision surface.
- the shell may compute authoritative values locally for convenience.
- a screenshot or pretty prototype is equivalent to UI-truth evidence.
- cross-platform packaging is automatically easy because Tauri is selected.

## Recovery path
When GE-07 is revisited for execution posture, the shortest honest path is:
1. read `artifacts/ge07-e1-shell-scaffold-receipt-2026-06-22.md`, `artifacts/ge07-e1-runtime-boundary-adr-input-2026-06-22.md`, `artifacts/ge07-e2-execution-readiness-closure-2026-06-22.md`, `artifacts/ge07-e3-execution-readiness-closure-2026-06-22.md`, `artifacts/ge07-e3-ui-truth-verification-receipt-2026-06-22.md`, `artifacts/ge07-e4-execution-readiness-closure-2026-06-22.md`, `artifacts/ge07-e4-explanation-diagnostics-visibility-receipt-2026-06-22.md`, `artifacts/ge07-e5-execution-readiness-closure-2026-06-22.md`, `artifacts/cross-platform-build-constraint-questions.md`, and `artifacts/ge07-e6-platform-risk-receipt-2026-06-22.md`
2. decide whether GE06-E4-F1 or an equivalent rules-core consumer bridge has actually landed on `origin/develop`
3. decide whether invalid-choice/prerequisite reasons are already grounded upstream or still require a separate GE-04 slice before any truthful GE07-E4 coding lane exists
4. confirm whether the next move is still a bounded non-production scaffold spike or whether the prerequisite shell subtree already exists
5. if the shell subtree exists, choose one platform proof target and build baseline before claiming any broader packaging posture
6. choose one narrow shell slice that does not absorb scaffold creation, boundary adaptation, workspace presentation, explanation/diagnostics projection, rules/source inspection projection, and release engineering in the same packet unless the scope is explicitly restated
7. ground exact repo paths, branch policy, write scope, and verification commands again against the live tree before any execution handoff
