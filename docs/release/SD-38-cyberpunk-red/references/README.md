---
canonical: true
owner: operator
bundle_id: SD-38
slug: cyberpunk-red
canonical_branch: tranche-18
target_version: 0.18.0
date: 2026-10-13
artifact_type: reference-index
status: draft — authored under operator-away defaults; operator rulings on licence and scope are on record
---

# SD-38 References

## Conduct

- `../../../../AGENTS.md`: Non-Negotiable Rules (TDD, no fake completion, scope, rules 7–9), Blocker
  Discipline, Concurrency and Measurement.
- `../../../../CLAUDE.md`: activation surface.

## Templates (authored from, not copied)

- `../../template/template.md`: the chassis.
- `../../../governance/workflow-instruction-template.md`: the per-cycle procedure; its §11 closure
  order is defective and corrected in `../workflow-instruction.md §11`.
- `../../../../.claude/skills/stc-authoring/SKILL.md`: the repo-local authoring/audit skill.

## Doctrine

- `../../../governance/blocker-closure-doctrine.md`
- `../../../governance/deferral-revisit-doctrine.md` (DEF-1…DEF-5)
- `../../../governance/no-stub-mvp-doctrine.md` §"Per-cycle audit"
- `../../../governance/wired-integration-stubs-registry.md` (entry 0002, retired for `cyberpunk-red` at E0.4)
- `../../../governance/license-matrix.md` (Cyberpunk RED section, E0.2)
- `../../../doctrine-external/identifier-discipline.md`

## Retrospectives (read before dispatching)

- `../../../retro/sd37-retrospective.md`: "Changes for the next bundle" (line 273) → rules R8–R13.
- `../../../retro/sd36-retrospective.md`: lessons 14–20 → rules R1–R7 (R1 retired for SD-38).
- `../../../retro/sd31-retrospective.md`: the shape E7.6 follows.
- SD-38's own retrospective is written and cited here by E7.6 (not yet written).

## Sibling bundles

- `../../SD-37-starfinder-1e/`: the shape precedent for every file here; `decisions.md §1` (path
  deviation), `§2` (14 version surfaces, bump commit `9d03a76996`), `§12` (unattended protocol);
  E1.4 PF render-hash harness; E4.6 adapter retirement of stub 0002 for Starfinder.

## Source (outside the repo; never copied into git)

- `/home/ubuntu/workspace/inbox/cyberpunk-red/cpr-corebook-v122.pdf`, `cpr-corebook-v122.txt`,
  `pages/pNNN.txt` (pNNN = printed page + 1).
- The seven inventories: `../artifacts/cycle_0/inventory/`.

## Memory notes this package encodes

`paper-sheet-generator-not-video-game`, `sheet-generator-print-the-rule-not-simulate`,
`batch-big-not-small-per-dispatch`, `one-pass-verify-dont-repeat-a-clean-gate`,
`clean-tree-means-unfiltered-status`, `closure-criteria-must-not-permit-open-cards`,
`graphify-runs-against-final-repo-state`, `graphify-force-update-replaces-semantic-graph`,
`workflow-agent-model-must-be-explicit`, `operator-away-autonomous-mode`,
`sheet-rule-package-path-is-baked-at-compile-time`, `proxmox-host-stops-vm-on-guest-oom`,
`peer-session-may-grab-the-shared-checkout`.
