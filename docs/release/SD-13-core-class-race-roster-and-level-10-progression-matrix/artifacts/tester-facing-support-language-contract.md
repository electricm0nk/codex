# Tester-Facing Support Language Contract

## Objective
Define the only approved tester-facing language for SD-13 breadth status so SD-11 and later consumers do not improvise counterfeit confidence.

## Approved wording by state

### `supported`
Allowed patterns:
- “Supported in the current bounded PF1 Core Rulebook roster slice for the named level band.”
- “This roster path is currently supported within the bounded SD-13 breadth tranche.”

### `partial`
Allowed patterns:
- “Partially supported in the current bounded roster slice; some progression or semantic obligations remain explicitly limited.”
- “This roster path is available with visible limitations; see the listed support-state details.”

### `lossy`
Allowed patterns:
- “Available only with lossy support in the current bounded roster slice; important semantics are simplified or approximated.”
- “This roster path can be exercised only under explicitly reduced fidelity; see the listed semantic loss.”

### `blocked`
Allowed patterns:
- “Blocked by known missing semantics in the current bounded roster slice.”
- “Not currently supported for truthful use at the claimed level band; the blocking gap is known and recorded.”

### `unverified`
Allowed patterns:
- “Included in the bounded roadmap scope, but not yet verified for this support level.”
- “This roster path remains in scope but has not yet been verified for the claimed progression band.”

## Prohibited wording
- “Core supported” when any relevant row remains `partial`, `lossy`, `blocked`, or `unverified`
- “Supported” with no visible qualifier when the row is not `supported`
- “Works” or “ready” with no state linkage
- “Parity” unless the GE-05 evidence surface for that row actually exists
- UI-only optimism such as “should work” or “available now” as a substitute for matrix truth

## Consumer rule
- SD-11 may present these labels, but SD-13 remains the authority that defines them.
- If a tester-facing surface cannot carry the necessary nuance, it must degrade toward caution rather than overstatement.
- Issue/report surfaces should preserve the row state and any gap or evidence references when available.
