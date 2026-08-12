# GitHub Artifact Publication and Promotion Contract

## Purpose
Define the authoritative GitHub-backed publication surface for SD-12 so later implementation can publish bounded tester builds, map them to channels, and preserve branch-lineage truth without inventing a second release system.

## Authoritative surfaces
- GitHub repository release/prerelease units are the canonical publication objects for bounded tester builds in this tranche
- release assets attached to those units are the authoritative artifact retrieval surface
- machine-readable channel/update metadata may be attached to or generated from those GitHub-backed publication units, but it must remain traceable back to them

## Required release unit contents
Every official tester-channel publication unit must include:
1. one or more platform artifacts allowed by the platform/support matrix
2. a checksum artifact covering every distributed asset
3. a provenance/build-receipt artifact linking the assets to a concrete source revision and publication event
4. a machine-readable update-manifest payload or a manifest reference that is resolvable from the GitHub-backed release unit
5. human-readable release notes stating the tester-facing channel, platform scope, and any manual-only or unsupported update posture

## Publication states
| Publication state | Backing operator truth | Tester-facing meaning | GitHub posture | Current status |
|---|---|---|---|---|
| `alpha` | `develop` | fastest-moving internal/close-tester track | prerelease or equivalent bounded GitHub publication state | live |
| `beta` | `reserved (no governed candidate branch yet)` | reserved candidate label for broader tester evaluation once a governed promotion surface exists | candidate/prerelease posture unavailable until a real promotion surface exists | unavailable today |
| `stable` | `main` | safest supported tester track in this tranche | non-prerelease release or equivalent stable publication state | live |

## Publication rules
- official tester-channel publication must originate from governed promotion points, not feature branches
- the publication record must preserve the originating branch/promotion truth even if the tester-facing surface prefers `alpha`, `beta`, or `stable`
- if a build is republished, superseded, or withdrawn, the publication surface must retain that state rather than forcing operators to infer it from missing files
- private or gated GitHub distribution is acceptable for the bounded tester program only if the resulting friction and failure modes are reflected honestly in the product/update surface

## Naming and identity requirements
The exact filename scheme remains open, but every official asset name or attached metadata must encode enough identity to determine:
- product/build identity
- version
- platform
- channel or release class
- provenance handle sufficient to connect back to source revision and publication record

## Promotion obligations
- promoting from `develop` to `main` must update publication truth deliberately; if a future candidate stage such as `beta` is added, its backing promotion surface must become real in repo/workflow governance before the publication contract claims it
- if a later workflow reuses the same source revision across multiple promotion states, the publication record must still distinguish which channel/support state each release unit represents
- if a later workflow rebuilds at promotion time, provenance must record that the asset set changed even when the semantic version remains related

## Explicit refusals
- do not treat a raw Git tag, branch, or ad hoc uploaded file as the whole publication contract
- do not let feature-branch artifacts masquerade as official tester channels
- do not bury withdrawn/superseded state in operator folklore
- do not claim “GitHub-backed updater” unless the release unit also satisfies the manifest and integrity contracts in this packet
