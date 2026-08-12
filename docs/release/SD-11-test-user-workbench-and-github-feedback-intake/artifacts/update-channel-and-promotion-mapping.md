# Update Channel and Promotion Mapping

## Purpose
Define the truthful relationship between operator branch promotion, tester-facing channel semantics, and platform support posture for SD-11.

## Operator truth
The authoritative live promotion flow is branch-based:

```text
develop -> main
```

This is the current operator/control-plane truth. It governs build lineage and promotion decisions. `beta` remains a reserved tester-facing label until a governed candidate promotion surface exists in repo/workflow truth.

## Tester-facing semantic mapping
| Operator backing | Tester-facing semantic | Meaning | Current status |
|---|---|---|---|
| `develop` | `alpha` | fastest-moving tester track; highest churn; acceptable for close/internal testers | live |
| `reserved (no governed candidate branch yet)` | `beta` | reserved candidate track for broader evaluation once a governed promotion surface exists | unavailable today |
| `main` | `stable` | safest supported tester track in this tranche | live |

## UI rules
- the workbench may show `alpha` and `stable` as live tester-facing channel language
- the workbench may show `beta` only as unavailable/reserved until a governed candidate promotion surface exists
- the workbench must not require ordinary testers to reason about raw branch names as the primary update UX
- operator/audit surfaces may retain the underlying branch lineage for provenance and triage
- bug and enhancement issue payloads should include the tester-facing channel/support label and may include the underlying branch lineage in hidden/operator fields when later implementation supports it

## Support-tier posture
| Platform | Support tier | Update truth |
|---|---|---|
| Linux | first-class | the tranche may eventually support the strongest self-update story here |
| macOS | second-class | must be presented honestly as real but less mature |
| Windows | third-class | must remain explicitly bounded; no fake parity claims |

## Updater-surface requirements
Any later implementation must be able to show, at minimum:
- current build/version
- current tester-facing channel/support label
- current platform/support tier
- update-available state when later supported
- update failure/result state when later supported

## Explicit refusals
- do not reduce the product contract to “pull from GitHub and do the needful”
- do not expose raw `develop` or `main` branch names as the primary tester language by default, and do not imply a `beta` backing surface before repo/workflow governance makes it real
- do not treat Windows updater polish as the gating condition for the whole tranche
