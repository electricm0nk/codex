# Distribution Platform Support and Channel Matrix

## Purpose
Define the truthful relationship between operator branch promotion, tester-facing channel semantics, platform support tiers, and allowed delivery/update posture for SD-12.

## Operator truth
The authoritative live promotion flow is branch-based:

```text
develop -> main
```

This is the current operator/control-plane truth. It governs promotion authority and release lineage. `beta` remains a reserved tester-facing label until a governed candidate promotion surface exists in repo/workflow truth.

## Tester-facing semantic mapping
| Operator backing | Tester-facing channel | Meaning | Current status |
|---|---|---|---|
| `develop` | `alpha` | fastest-moving tester track; highest churn; acceptable for close/internal testers | live |
| `reserved (no governed candidate branch yet)` | `beta` | reserved candidate track for broader evaluation once a governed promotion surface exists | unavailable today |
| `main` | `stable` | safest supported tester track in this tranche | live |

## Platform distribution and support matrix
| Platform | Support tier | Minimum delivery posture | Automatic update posture | Rollback / recovery posture | Notes |
|---|---|---|---|---|---|
| Linux | first-class | must receive the first bounded tester artifact set, including one install-oriented artifact path, one recovery/manual path, and release metadata | may claim automatic update once manifest, checksum/provenance, and Linux trust thresholds are satisfied | must support explicit withdrawal, supersedence, and recovery guidance | first platform that must receive the full bounded SD-12 story |
| macOS | second-class | must receive an explicit real posture: manual download-only or bounded package path; it may not be omitted silently | automatic update is blocked until the macOS trust threshold is defined and satisfied explicitly | withdrawal/recovery state must still be visible even if updates remain manual | second-class but real means named support, not parity theater |
| Windows | third-class | may remain manual-only, highly bounded, or explicitly unavailable for this tranche, but the truth must be stated plainly | no automatic update claim is allowed in this tranche unless a later slice explicitly authorizes it | if artifacts exist at all, they must still participate in withdrawal/recovery truth | third-class containment is a real product statement, not a temporary silence |

## Channel eligibility rules
- only governed promotion states may publish official tester-channel artifacts
- feature branches may produce ad hoc developer artifacts but must not be represented as `alpha`, `beta`, or `stable`
- channel eligibility must be derivable from the release control plane, not from UI copy alone
- the UI may show tester-facing channel words, but operator/audit surfaces must retain the underlying branch lineage

## Build identity requirements
Every distributed tester build must carry, at minimum:
- product/build label
- version string
- commit or provenance handle
- operator source branch / promotion lineage reference
- tester-facing channel label
- platform label and support tier
- update eligibility state

## Explicit refusals
- do not expose raw `develop` or `main` branch names as the primary tester-facing UX, and do not claim a `beta` backing branch until it exists in repo/workflow truth
- do not treat a downloadable file with no provenance or manifest as a finished tester-distribution surface
- do not let macOS or Windows parity ambitions block Linux-first execution
- do not let Windows become “maybe supported” by implication; it must remain explicitly bounded
