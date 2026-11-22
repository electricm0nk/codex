---
title: GE06-E1-F1 Handoff Readiness Closure
artifact_type: handoff-readiness-closure
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE06-E1-F1 — Grounded character selection ledger
workflow_route: research
readiness: research-ready
handoff_created: true
created_handoff: ../research-handoff.md
superseded_by: ./ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md
review_date: 2026-06-20
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE06-E1-F1 Handoff Readiness Closure

## Supersession notice
This closure is preserved as the 2026-06-20 routing decision that authorized non-code discovery. Its unresolved input questions were closed by `ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md`.

## Verdict
GE06-E1-F1 is the correct first bounded GE-06 slice, and it is now **research-ready**.

The derived non-code handoff is:

```text
../research-handoff.md
```

This closure records why that handoff is justified. The closure itself remains non-code-authorizing; code authority still does not exist anywhere in GE-06.

## Selected bounded slice

```text
GE06-E1-F1 — Grounded character selection ledger
```

Source in `epic-breakdown.md`:

- objective: finalize the exact integrated pilot case and reconcile charter facts, token-family gates, and canonical-object obligations into one bounded fixture contract
- acceptance:
  - the exact race/class/level/ability-score/feat/skill/equipment selection set is explicit
  - unresolved selections are either closed or promoted into explicit blockers
  - no selection silently expands the charter boundary

## Why this is the first slice
The GE-06 source STC explicitly forbids a broad implementation prompt and says the next truthful move is a bounded readiness closure rather than an `execution-handoff.md`.

`README.md` states that the most likely first readiness target is a **non-code closure on the exact integrated pilot character fixture**. GE06-E1-F1 is that closure target.

GE06-E1-F1 is narrower than the spec domain because it does not attempt to:

- implement importer/runtime integration in `/home/ubuntu/workspace/repos/codex`
- prove headless compute or explanation behavior
- prove oracle-comparison output
- choose a GE-06 integration branch or worktree
- define the final product-visible UI slice
- broaden the pilot charter under the label of fixture cleanup

It instead answers the first dependency question that would make every later GE-06 slice counterfeit if left vague: **what exact pilot character input set are we actually claiming to prove?**

## Grounded facts recovered by tools

| Fact | Result |
|---|---|
| Workspace date from runtime | `2026-06-20` |
| Target workspace | `/home/ubuntu/workspace` |
| GE-06 source-STC next-stage rule | no `execution-handoff.md`; first likely target is a non-code closure on the exact integrated pilot character fixture |
| Codex repo branch observed | `ge04-e1-f1-character-input-record-shape` |
| Local untracked residue in Codex repo | `AGENTS.md`, `CLAUDE.md`, `Cargo.lock`, `target/` |
| Pilot charter grounded seed facts | case `pf1-crb-human-fighter-level1`; Human; Fighter 1; ability scores `16/14/14/10/12/8`; named feat seed `power_attack` |
| Human race source surfaces grounded | `core_rulebook.pcc` -> `_race.pcc` -> `human_races.lst`, `human_abilities_race.lst`, `human_abilities_globalvar.lst` |
| Fighter source surfaces grounded | `cr_classes.lst#Fighter`; `cr_abilities_class.lst#Weapon and Armor Proficiency ~ Fighter`; `cr_abilities_class.lst#Class Skills ~ Fighter`; `cr_abilities_class.lst#Fighter ~ Bonus Feats`; `cr_abilitycategories.lst#Fighter Bonus Feat` |
| Named feat surface grounded | `cr_feats.lst` line 134 `Power Attack` |
| Representative equipment surfaces grounded | `cr_equip_arms_armor.lst` lines 40/53 `Chain Shirt`, lines 165/223 `Longsword`, and `cr_profs_weapon.lst` line 57 `Longsword` |
| Current GE-06 unresolved debt | exact skill allocation, exact equipment loadout, additional feat/choice entitlement closure, export-summary boundary |

## Gate table

| Gate | Status | Resolution |
|---|---|---|
| Source STC exists | pass | GE-06 source-STC bundle exists under `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/`. |
| Next-stage rule selects a non-code first move | pass | `README.md` explicitly says the next truthful move is a bounded readiness closure and names non-code fixture closure as a likely first target. |
| Downstream slice selected | pass | GE06-E1-F1 is selected as the first candidate handoff slice. |
| Slice is narrower than the spec domain | pass | The slice closes fixture inputs only; it excludes headless implementation, parity runtime work, UI work, and integration-branch decisions. |
| Required source universe is grounded | pass | GE-06, pilot charter, GE-01 artifacts, GE-04 fixture requirements, and exact PCGen source files for Human/Fighter/Power Attack/Chain Shirt/Longsword are identified. |
| Output artifact path is explicit | pass | The derived handoff grants exactly one receipt path under `artifacts/`. |
| Code-producing route blocked | pass | `code_authority: false`; no repo write scope, branch policy, implementation commands, or verification commands are granted. |
| GE-07 UI dependency still quarantined | pass | GE-07 exists only as a spec domain, so product-visible UI scope remains blocked from this handoff. |
| Unresolved questions remain visible | pass | Skill allocation, equipment loadout, additional feat/choice debt, and export-summary boundary remain explicit until grounded or promoted to blockers. |
| Non-goals explicit | pass | The derived handoff forbids implementation code, parity claims, UI proof, and silent charter expansion. |

## Granted research surface
The derived handoff grants only this output path:

```text
/home/ubuntu/workspace/programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-grounded-character-selection-ledger-2026-06-20.md
```

Everything else remains out of scope for the downstream worker.

## Verification evidence from this pass
This readiness pass grounded the route using real tool evidence from:

```text
date +%F
git -C /home/ubuntu/workspace/repos/codex branch --show-current
git -C /home/ubuntu/workspace/repos/codex status --short --branch
```

and repository/document discovery confirming:

- the GE-06 source-STC next-stage rule
- the GE06-E1-F1 epic seed and acceptance text
- the current unresolved fixture debt in `artifacts/pilot-character-fixture-requirements.md`
- the current open questions in `risks-and-open-questions.md`
- exact Human/Fighter/Power Attack/Chain Shirt/Longsword source surfaces in the PCGen corpus and GE-01 artifacts

## Result
A code-authorizing handoff still does **not** exist for GE-06.

A bounded non-code handoff now exists at:

```text
../research-handoff.md
```

Future work on GE06-E1-F1 must use that handoff and remain documentary/discovery-only.

## Stop condition for the downstream run
Stop the downstream research run if any of the following become true:

1. the exact selection set cannot be grounded from the allowed source universe without inventing facts,
2. multiple plausible loadouts or choice resolutions remain and no authority surface chooses between them,
3. the work would need to modify `/home/ubuntu/workspace/repos/codex` or `/home/ubuntu/workspace/repos/pcgen`,
4. the work would need to patch GE-06 control documents instead of writing only the granted receipt,
5. closing the fixture would require silent pilot-scope expansion rather than an explicit charter/ADR decision.

## Closure status
GE06-E1-F1 is selected, grounded, and **research-ready**. The derived `research-handoff.md` is now the correct execution surface for this non-code closure.