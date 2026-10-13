---
canonical: true
owner: operator
bundle_id: SD-38
slug: cyberpunk-red
canonical_branch: tranche-18
target_version: 0.18.0
date: 2026-10-13
status: draft — authored under operator-away defaults; operator rulings on licence and scope are on record
---

# SD-38 Release Notes — Cyberpunk RED

SD-38 INTERIM NOTES. Work on `tranche-18` is in progress. Card E7.8 replaces this whole body with the
final notes. E7.8 and E7.11 refuse to close while the sentinel at the start of this paragraph is in the
file (`grep -c 'SD-38 INTERIM NOTES' docs/release/SD-38-cyberpunk-red/release-notes.md` → 0).

## Summary (interim)

Cyberpunk RED (corebook v1.22) is being added as a third game system beside Pathfinder 1e and
Starfinder 1e. Pathfinder and Starfinder characters are meant to stay byte-identical (gate G-1).
No Cyberpunk RED feature in this build is described as finished until E7.8 writes these notes.

## What E7.8 writes here

A summary; user-visible changes under the Cyberpunk Red chip (creation by three methods, the sheet,
improvement, catalogs, GM reference pages, three starter characters); the licence posture
(`decisions.md §4`); what stays the same (Pathfinder and Starfinder byte-identical, G-1); and a
"Figures" section where every number is `R-n` with the command that produced it and a second
implementation, in the shape of `docs/release/SD-37-starfinder-1e/release-notes.md`.

Why this body is not the resolver's closure marker: `docs/release/current-release.json` must name this
file from C1 on, because the resolver requires the pointer's tranche to equal `apps/desktop/package.json`
(`tools/release/resolve_release_notes.py:45–51`), and C1 sets `0.18.0`. A body containing the resolver's
markers would keep the CI step `release-tooling-tests.yml:49` red from C1 to E7.8 (`decisions.md §2`).
