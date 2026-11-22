---
title: GE06 Post-F3 Handoff Rack
artifact_type: execution-story-pack
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_route_surface: ../execution-handoff.md
source_merge_receipt: ./ge06-e2-f3-merge-receipt-2026-06-21.md
status: active
review_date: 2026-06-22
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE06 Post-F3 Handoff Rack

## Objective
Rotate GE-06 truthfully after the merged E2-F3 headless receipt path, preserve Todd's manual launch/merge gates, and surface the first honest post-F3 launch pair as live stage-specific handoffs instead of leaving them trapped as prebuild drafts.

## Observed repo anchor
Grounded during promotion on 2026-06-22:

```text
repo: /home/ubuntu/workspace/repos/codex
origin/develop: 6977c86
merge: Merge pull request #13 from electricm0nk/ge06-e2-f3-headless-receipt-path
verification:
  - "$HOME/.cargo/bin/cargo" test --test ge06_pilot_headless_receipt --quiet -> pass
  - "$HOME/.cargo/bin/cargo" test --quiet -> pass
```

## Merged capability floor after F3
GE-06 may still truthfully claim only the following bounded footholds:

```text
selected pilot input contract: represented
base ability modifiers and Fighter class chassis: computed
baseline melee attack bonus / armor class: computed
total Fortitude / Reflex / Will saves: computed
selected deterministic Climb / Intimidate / Swim modifiers: computed
integrated headless receipt path: computed
selected parity-dimension adapter: awaiting-todd-launch
failure classifier and owner mapping: awaiting-todd-launch
oracle parity: not checked
UI truth: not product-visible
```

## E3 activation decision
The E3 pair is now **live and awaiting Todd launch**.

Truthful breakdown:
- `GE06-E3-F1` is now `awaiting-todd-launch` because its live readiness closure and stage-specific execution handoff have been minted from the merged E2-F3 receipt surface.
- `GE06-E3-F2` is now `awaiting-todd-launch` for the same reason: the merged receipt and diagnostics surface were re-read and promoted into a live handoff.
- `GE06-E3-F3` remains blocked because it is a fan-in evidence bundle that depends on outputs from both E3-F1 and E3-F2.

Therefore GE-06 now has **two live code-authorizing handoffs**, but they are not `running` until Todd manually launches them.

## Human-gated automation boundary
This rack is intentionally not a full-autonomy queue.

### Hermes may do
- detect merges and rotate documentary route state
- write merge receipts
- generate readiness closures and live handoffs
- classify packets as awaiting-launch, blocked, running, or awaiting-merge when documentary truth supports that state
- prepare a pile of execution packets for Todd to launch when convenient

### Hermes must not do by default
- launch Claude on Todd's behalf
- merge PRs on Todd's behalf
- pretend a packet is running when Todd has not launched it

## Packet state vocabulary
- `merged` — landed on `origin/develop`
- `ready-to-derive` — enough truth exists to create the next readiness closure / handoff pair now
- `launch-preparable` — documentary work can prepare a packet, but a code-authorizing handoff is not yet honestly grounded
- `awaiting-todd-launch` — a real stage-specific execution handoff exists and Todd may launch it manually
- `running-under-human-invoked-harness` — Todd launched the coding harness
- `awaiting-todd-merge` — code run is complete and waiting on Todd's merge action
- `blocked` — prerequisite evidence or upstream authority is still missing

## Queue summary
| Packet | Kind | State now | Depends on | Parallel-safe notes | Live artifact identities | Todd action |
|---|---|---|---|---|---|---|
| GE06-E3-F1 — Selected parity-dimension adapter | coding lane | awaiting-todd-launch | merged GE06-E2-F3 receipt | parallel-safe with E3-F2 because the write scope remains in `src/oracle_validation/**` | `ge06-e3-f1-execution-readiness-closure-2026-06-22.md` / `ge06-e3-f1-execution-handoff-2026-06-22.md` | manually launch in Claude when ready |
| GE06-E3-F2 — Failure classifier and owner mapping | coding lane | awaiting-todd-launch | merged GE06-E2-F3 receipt + diagnostics | parallel-safe with E3-F1 because the write scope remains in `src/rules_core/**` | `ge06-e3-f2-execution-readiness-closure-2026-06-22.md` / `ge06-e3-f2-execution-handoff-2026-06-22.md` | manually launch in Claude when ready |
| GE06-E3-F3 — Viability evidence bundle | documentary or coding support lane | blocked | E3-F1 and E3-F2 outputs | fan-in packet; should not launch before both upstream evidence surfaces exist | future `ge06-e3-f3-...` triplet | none yet |
| GE06-E4-F1 — Pilot view-model contract from real outputs | coding lane with pre-viability spike option | bounded pre-viability spike posture; no live handoff yet | E2-F3 plus accepted GE-07 source STC; live handoff still waits for explicit spike authorization or accepted GE-06 viability posture | stale blocker is cleared: GE-07 now has a source STC; the remaining gate is route grounding (exact repo paths, toolchain, and spike receipts), not missing planning authority | `ge06-e4-f1-launch-posture-2026-06-22.md`, then future `ge06-e4-f1-execution-readiness-closure-YYYY-MM-DD.md` / `ge06-e4-f1-execution-handoff-YYYY-MM-DD.md` / `ge06-e4-f1-merge-receipt-YYYY-MM-DD.md` | authorize bounded spike if UI exploration should start before GE-06 viability; otherwise hold for viability verdict |
| GE06-E4-F2 — Explanation and diagnostic inspection surface | coding lane | blocked | E4-F1 | may later run in parallel with E4-F3 if E4-F1 establishes a stable view-model boundary | future `ge06-e4-f2-...` triplet | none yet |
| GE06-E4-F3 — One exportable summary boundary | coding or documentary boundary lane | blocked | E4-F1 | may later run in parallel with E4-F2 if export scope and inspection UI stay disjoint | future `ge06-e4-f3-...` triplet | none yet |
| GE06-E5-F1 — Pilot viability review artifact | documentary review lane | blocked | E3-F3 and E4 evidence posture | fan-in review artifact; not a parallel first mover | future `ge06-e5-f1-...` review artifact | none yet |
| GE06-E5-F2 — Narrow-vs-expand decision trigger | documentary decision lane | blocked | E5-F1 | serial after viability review | future `ge06-e5-f2-...` decision artifact | none yet |
| GE06-E5-F3 — Upstream delta/no-change review | documentary governance lane | blocked | E5-F1 and E5-F2 | may partially draft in advance, but truthful completion waits for the decision outputs | future `ge06-e5-f3-...` review artifact | none yet |

## What should happen next automatically
### Immediate documentary move
E3 promotion is now complete.

Derived artifacts from this rotation:
1. `ge06-e3-f1-execution-readiness-closure-2026-06-22.md`
2. `ge06-e3-f1-execution-handoff-2026-06-22.md`
3. `ge06-e3-f2-execution-readiness-closure-2026-06-22.md`
4. `ge06-e3-f2-execution-handoff-2026-06-22.md`
5. route-surface promotion from `no-active-handoff` to `awaiting-todd-launch`

### Immediate human move
Todd may now manually launch either or both live E3 handoffs.

### Next documentary promotion target
The next truthful packet remains:

```text
GE06-E3-F3 — Viability evidence bundle
```

It must stay blocked until both E3-F1 and E3-F2 produce outputs.

## First expected parallelization point
The first real parallel pair is now explicit, live, and evidence-backed:

```text
GE06-E3-F1 — Selected parity-dimension adapter
GE06-E3-F2 — Failure classifier and owner mapping
```

They are not running yet, but they are now honest launch gates rather than documentary forecasts.

## Non-negotiable truth rules
- Do not mark a packet `running-under-human-invoked-harness` until Todd actually launches it.
- Do not mark a packet `awaiting-todd-merge` until a real coding run returns evidence.
- Do not mark a packet `merged` until repo state proves it.
- Do not manufacture fan-in readiness for E3-F3 before both upstream outputs exist.
- Keep every code-producing packet bounded to its own stage-specific artifact identity; never retarget a prior handoff file in place.

## Completion rule
This rack is useful only if it prevents chair-driving while preserving truth.

That means it must:
- show Todd exactly what is merged
- show exactly which handoffs are now live
- show which packets are blocked vs awaiting launch
- preserve the Todd launch and Todd merge gates
- identify the first honest parallelization pair
- refuse to call E3-F3 ready before the two upstream evidence lanes finish
