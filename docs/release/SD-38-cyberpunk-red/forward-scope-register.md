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

# SD-38 Forward Scope Register

The planning entry point for SD-38: what it inherits and what it pushes forward. Classes:

- **Committed** — in SD-38's Definition of Done, so it is a card. A shortfall is a blocker.
- **Structurally implied** — falls out of committed work.
- **Candidate** — not in the DoD; each names a revisit condition **checked** at E7.5.

Only the operator moves scope between classes (`docs/governance/blocker-closure-doctrine.md`).

## 1. Committed

| ID | Work | Source | SD-38 cards |
|---|---|---|---|
| COM-1 | Cyberpunk RED as a third game system: seam, data, engine, create, sheet, improve, catalogs | operator scope ruling "full book" (2026-10-13) | E0–E7 |
| COM-2 | Whole-text licence posture with a recorded deviation | operator licence ruling (2026-10-13) | E0.2 |
| COM-3 | GM tables as print-only reference pages, incl. the three with no inventory | brief DEFAULT (`decisions.md §5`) | E1.8, E6.4 |

## 2. Structurally implied

| ID | Work | Falls out of |
|---|---|---|
| IMP-1 | `GameSystem` widened to three systems; `sheet_rules_relative` optional | E0.1 |
| IMP-2 | Stub registry 0002 retired for `cyberpunk-red` | E0.4 |
| IMP-3 | Restore-starters generalised beyond Pathfinder requests | E7.1 |
| IMP-4 | `verify-baselines.env` re-derived for three systems | E7.4 |
| IMP-5 | Architecture docs gain a Cyberpunk RED section | E7.9 |
| IMP-6 | A licence-matrix section for a non-OGL book | E0.2 |

## 3. Candidates (not in the DoD; not gates)

| ID | Work | Source | Why not in SD-38's DoD | Revisit condition (checked at E7.5) |
|---|---|---|---|---|
| FSR-C1 | Reword CPR record `text` fields (keep names and numbers) if the operator later narrows the licence posture | operator: "we can back stuff out or reword it later" | the ruling is "do the whole thing for now" | `python3 -c "import json;print(json.load(open('data/cyberpunk-red/corpus/LICENSE.json'))['operator_sign_off']['signed_off'])"` prints `False` (the operator revoked sign-off) |
| FSR-C2 | Raise E1.V's sample above 10% | R-2 | the brief fixed 10% | the E1.V receipt reports any `disagree` rate above 5% in a kind |
| FSR-C3 | SD-37 candidates carried forward unchanged (graphify node-count guard, template §11 closure order, FS-7 bare-slug `find`) | `docs/release/SD-37-starfinder-1e/forward-scope-register.md` §3 and "Added at closure" | not in SD-38's scope | E7.10's graphify run hits the node-count guard again (`grep -c 'node-count' docs/release/SD-38-cyberpunk-red/receipts.md` ≥ 1) |
| FSR-C4 | The other placeholder rule sets (`solarus-arcanum`, `traveller`, `world-of-darkness`) | `LandingScreen.tsx` `RULE_SETS` | the operator named Cyberpunk RED only | the operator names another system |

## 4. Planned capability deferrals

The test (`docs/governance/deferral-revisit-doctrine.md`): was this scope in the Definition of Done
when the work was scoped? For each row the brief's "Not in DoD" list says no.

| ID | Capability | Revisit condition (a command, checked at E7.5) | Accepted cost |
|---|---|---|---|
| DEF-1 | NET Architecture **generator** | the operator asks: `grep -rli 'architecture generator' docs/release/SD-*/scope-draft.md \| grep -v SD-38 \| awk 'END{print NR}'` ≥ 1 | GMs roll from the printed tables |
| DEF-2 | In-play dice engine (creation-time table rolls are in scope) | the operator asks for in-play rolls (same grep for 'dice engine') | the sheet prints dice |
| DEF-3 | NET-run simulation | the operator asks (same grep for 'net-run simulation') | NET combat stays printed |
| DEF-4 | Vehicle-combat simulation | the operator asks (same grep for 'vehicle combat') | vehicle rules stay printed |
| DEF-5 | NPC / Mook generator | the operator asks (same grep for 'mook generator') | stat blocks are print-only pages |

If the operator says any of these was meant to be in the DoD, it becomes a blocker and SD-38 is not
done without it.

## 5. No deferrals for SD-38 blockers

No card in `kanban.md` may be moved here by a cycle. A card that cannot close is `blocked-escalated`
and waits for the operator (`workflow-instruction.md §8`).
