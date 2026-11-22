---
title: GE06 Post-F2d Handoff Rack
artifact_type: execution-story-pack
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_route_surface: ../execution-handoff.md
source_merge_receipt: ./ge06-e2-f2d-merge-receipt-2026-06-21.md
status: draft
review_date: 2026-06-21
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE06 Post-F2d Handoff Rack

## Objective
Convert GE-06 from a manually nudged single-step chain into a queued packet rack that preserves the two human gates Todd wants to keep:

1. Todd launches Claude manually.
2. Todd merges PRs manually.

Everything else should be documentary/orchestration work.

## Observed repo anchor
Grounded on 2026-06-21 after `git fetch origin --prune` and detached verification of `origin/develop`:

```text
repo: /home/ubuntu/workspace/repos/codex
origin/develop: 2deb11b
merge: Merge pull request #12 from electricm0nk/ge06-e2-f2d-selected-skill-modifiers
verification: "$HOME/.cargo/bin/cargo" test --quiet -> pass
```

## Merged capability floor after F2d
GE-06 may now truthfully claim only the following bounded footholds:

```text
selected pilot input contract: represented
base ability modifiers and Fighter class chassis: computed
baseline melee attack bonus / armor class: computed
total Fortitude / Reflex / Will saves: computed
selected deterministic Climb / Intimidate / Swim modifiers: computed
integrated headless command / receipt path: documentary packet derived and awaiting Todd launch
oracle parity: not checked
UI truth: not product-visible
```

## Human-gated automation boundary
This rack is intentionally not a full-autonomy queue.

### Hermes may do
- detect merges and rotate documentary route state
- write merge receipts
- generate readiness closures, draft handoffs, and queue metadata
- classify packets as draft, launch-preparable, blocked, or awaiting-merge
- prepare a pile of execution packets for Todd to launch when convenient

### Hermes must not do by default
- launch Claude on Todd's behalf
- merge PRs on Todd's behalf
- pretend a packet is running when Todd has not launched it

## Packet state vocabulary
- `merged` — landed on `origin/develop`
- `ready-to-derive` — enough truth exists to create the next readiness closure / handoff pair now
- `launch-preparable` — documentary work can prepare a packet, but a code-authorizing handoff is not yet honestly grounded
- `blocked` — prerequisite evidence or upstream authority is still missing
- `awaiting-todd-launch` — a real stage-specific execution handoff exists and Todd may launch it manually
- `awaiting-todd-merge` — code run is complete and waiting on Todd's merge action

## Queue summary
| Packet | Kind | State now | Depends on | Parallel-safe notes | Planned artifact identities | Todd action |
|---|---|---|---|---|---|---|
| GE06-E2-F3 — End-to-end command and receipt path | coding lane | awaiting-todd-launch | F2d merged | No parallel launch partner yet; this is the active proving slice for downstream parity/UI consumers | `ge06-e2-f3-execution-readiness-closure-2026-06-21.md`, `ge06-e2-f3-execution-handoff-2026-06-21.md`, `ge06-e2-f3-merge-receipt-YYYY-MM-DD.md` | launch Claude on the packet when ready |
| GE06-E3-F1 — Selected parity-dimension adapter | coding or documentary bridge, depending on exact evidence shape | launch-preparable (prebuild drafted) | GE06-E2-F3 receipt/evidence | likely parallel-safe with E3-F2 once E2-F3 has emitted a stable evidence shape and write scopes are disjoint | `ge06-e3-f1-prebuild-readiness-closure-2026-06-21.md`, `ge06-e3-f1-prebuild-handoff-2026-06-21.md`, future live triplet `ge06-e3-f1-execution-readiness-closure-YYYY-MM-DD.md` / `ge06-e3-f1-execution-handoff-YYYY-MM-DD.md` / `ge06-e3-f1-merge-receipt-YYYY-MM-DD.md` | none yet |
| GE06-E3-F2 — Failure classifier and owner mapping | coding lane | launch-preparable (prebuild drafted) | GE06-E2-F3 receipt/evidence | likely parallel-safe with E3-F1 after E2-F3 if write scopes can be kept separate (oracle-validation adapter vs rules-core classifier) | `ge06-e3-f2-prebuild-readiness-closure-2026-06-21.md`, `ge06-e3-f2-prebuild-handoff-2026-06-21.md`, future live triplet `ge06-e3-f2-execution-readiness-closure-YYYY-MM-DD.md` / `ge06-e3-f2-execution-handoff-YYYY-MM-DD.md` / `ge06-e3-f2-merge-receipt-YYYY-MM-DD.md` | none yet |
| GE06-E3-F3 — Viability evidence bundle | documentary or coding support lane | blocked | E3-F1 and E3-F2 | fan-in packet; should not launch before both upstream evidence surfaces exist | `ge06-e3-f3-...` triplet | none yet |
| GE06-E4-F1 — Pilot view-model contract from real outputs | coding lane | blocked | E2-F3 plus explicit UI-authority posture | no parallel launch until the exact UI authority posture is grounded; GE-07 is still only a spec domain | `ge06-e4-f1-...` triplet | none yet |
| GE06-E4-F2 — Explanation and diagnostic inspection surface | coding lane | blocked | E4-F1 | may later run in parallel with E4-F3 if E4-F1 establishes a stable view-model boundary | `ge06-e4-f2-...` triplet | none yet |
| GE06-E4-F3 — One exportable summary boundary | coding or documentary boundary lane | blocked | E4-F1 | may later run in parallel with E4-F2 if export scope and inspection UI stay disjoint | `ge06-e4-f3-...` triplet | none yet |
| GE06-E5-F1 — Pilot viability review artifact | documentary review lane | blocked | E3-F3 and E4 evidence posture | fan-in review artifact; not a parallel first mover | `ge06-e5-f1-...` review artifact | none yet |
| GE06-E5-F2 — Narrow-vs-expand decision trigger | documentary decision lane | blocked | E5-F1 | serial after viability review | `ge06-e5-f2-...` decision artifact | none yet |
| GE06-E5-F3 — Upstream delta/no-change review | documentary governance lane | blocked | E5-F1 and E5-F2 | may partially draft in advance, but truthful completion waits for the decision outputs | `ge06-e5-f3-...` review artifact | none yet |

## What should happen next automatically
### Immediate documentary move
E2-F3 has now been derived.

Derived artifacts:
1. `ge06-e2-f3-execution-readiness-closure-2026-06-21.md`
2. `ge06-e2-f3-execution-handoff-2026-06-21.md`
3. route-surface rotation to `awaiting-todd-launch`

### Immediate human move
Todd may now launch Claude on E2-F3 whenever convenient.

### After Todd launches Claude on E2-F3
The chain should wait in a human-gated run state rather than demanding repeated nudges.

Target state transition:

```text
no-active-handoff
-> awaiting-todd-launch
-> running-under-human-invoked-harness
-> awaiting-todd-merge
-> merged
-> no-active-handoff (with next rack rotation)
```

### After Todd merges E2-F3
Hermes should automatically:
1. write the merge receipt
2. rotate the route surface
3. advance the rack
4. re-evaluate whether E3-F1 and E3-F2 can be prepared together as the first genuine parallel pair

## First expected parallelization point
The most likely first real parallel pair remains:

```text
GE06-E3-F1 — Selected parity-dimension adapter
GE06-E3-F2 — Failure classifier and owner mapping
```

But only after E2-F3 emits a stable integrated receipt/evidence shape.

That gate is now prebuilt explicitly rather than left in conversation memory: the E3-F1 and E3-F2 draft closures/handoffs exist, but they carry `code_authority: false` until a post-merge documentary pass promotes them.

Why not earlier:
- E2-F3 is the packet that converts the current merged compute footholds into one integrated headless evidence surface
- without that surface, E3 lanes would be fabricating their inputs or inventing a contract for later consumers

## Non-negotiable truth rules
- Do not mark a packet `awaiting-todd-launch` until the actual stage-specific execution handoff exists.
- Do not mark a packet `running` until Todd actually launches Claude.
- Do not mark a packet `merged` until repo state proves it.
- Do not manufacture parallelism by launching two lanes against the same unstable evidence shape.
- Keep every code-producing packet bounded to its own stage-specific artifact identity; never retarget a prior handoff file in place.

## Completion rule
This rack is useful only if it prevents chair-driving.

That means it must:
- show Todd exactly what is merged
- show exactly what packet family is next
- show which packets are blocked vs preparable
- preserve the Todd launch and Todd merge gates
- identify the earliest honest parallelization point
- let future documentary runs advance the queue without requiring Todd to type `next` after every merge
