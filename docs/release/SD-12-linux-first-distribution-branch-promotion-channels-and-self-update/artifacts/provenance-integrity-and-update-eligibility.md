# Provenance, Integrity, and Update Eligibility

## Purpose
Define the minimum build identity, integrity material, and platform trust thresholds required before a distributed tester build may claim automatic update eligibility.

## Minimum build identity
Every distributed tester build must be linkable to:
- product/build label
- semantic or bounded version string
- source revision / commit or equivalent provenance handle
- operator promotion state / release unit
- tester-facing channel label
- platform/support tier
- publication timestamp

## Required publication companions
Every official tester build must publish:
- a checksum artifact covering each distributed asset
- a provenance/build-receipt artifact that links assets to a source revision and publication event
- a manifest entry or equivalent machine-readable update record pointing at the correct assets

## Update eligibility gate
A platform/build may claim `automatic` update eligibility only when all of the following are true:
1. the release unit exists and is in a valid publication state
2. the manifest points to the correct asset for the current platform/channel
3. checksum and provenance material are available and checkable
4. the platform-specific trust threshold for that platform is explicitly satisfied
5. rollback/withdrawal behavior is defined for failure or supersedence

If any of the above are false, the build must be classified as `manual-only`, `blocked`, `withdrawn`, or `unsupported` instead of `automatic`.

## Platform trust thresholds
| Platform | Minimum SD-12 truth |
|---|---|
| Linux | automatic-update claims require checksum + provenance + manifest + chosen Linux artifact path + explicit recovery posture; any stronger signing rule may be added later but these minimums are non-optional |
| macOS | no automatic-update claim until the macOS trust strategy (for example signing/notarization or an explicit alternative) is fixed and satisfied |
| Windows | no automatic-update claim in this tranche unless a later explicit slice establishes the Windows trust posture and recovery path |

## Proof obligations for later execution
Any implementation slice claiming automatic update must prove:
- which trust threshold it satisfies
- where the checksum/provenance materials are published
- how a client verifies or consumes them
- how withdrawn or bad builds are prevented from appearing as normal eligible updates

## Explicit refusals
- do not equate “file can be downloaded” with trustworthy update eligibility
- do not allow one platform’s trust posture to authorize another’s
- do not publish official tester builds with no provenance handle
- do not classify a build as `automatic` when the recovery path is undefined
