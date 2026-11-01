---
canonical: true
owner: operator
bundle_id: SD-39
slug: ui-print-corrections
canonical_branch: tranche-19
target_version: 0.19.0
date: 2026-11-01
status: planning-ready
---

# SD-39 Forward Scope Register

Classes: **Committed** (in the DoD; a shortfall is a blocker), **Structurally implied** (falls out of
committed work), **Candidate** (not in the DoD; a revisit condition is *checked* at E9.5). Only the
operator moves scope between classes.

## 1. Committed

| ID | Work | Source | SD-39 cards |
|---|---|---|---|
| COM-1 | Rust PDF generation, per-system templates, A4/Letter, stacking modes, Save PDF + Print | D1; R1–R6 | E8.1–E8.5, E9.4 |
| COM-2 | Main-view layout moves, Character Details title, Show Progression drawer | D2, D8; R7 | E2.1–E2.3 |
| COM-3 | Feat budget, duplicate guard, hide-unmet checkbox | D6, D7; R9, R10 | E1.1–E1.3 |
| COM-4 | Market replaces the three Add buttons; category filter fixed; magic +1…+5; price modes | D3, D4; R11, R14 | E3.1–E3.5 |
| COM-5 | Body slots, worn vs carried, Equipment control, custom-item stats/slots | D5, D11; R12 | E1.0, E4.1–E4.4 |
| COM-6 | Level ledger + migration + drawer content | D9; R8 | E5.1–E5.3 |
| COM-7 | Overrides group with tagging | R10b | E6.1, E6.2 |
| COM-8 | `.pcg` import via sidecar | D12; R13 | E7.1, E7.2 |
| COM-9 | Add Custom chip styling | D10 | E1.4 |

## 2. Structurally implied

| ID | Work | Falls out of |
|---|---|---|
| IMP-1 | Slot table and `slot_of` over existing tags (no converter run unless the proof fails) | COM-3, COM-4, COM-5 (E1.0) |
| IMP-2 | `ledger.json` beside `custom.json` + first-load build | COM-6 (E5.1, E5.2) |
| IMP-3 | `sheetLayout.ts` registers the left rail and right column | COM-2 (E2.1) |
| IMP-4 | Packaged resources: fonts + sidecar in `tauri.conf.json` and `packaged_resources.rs` | COM-1, COM-8 (E9.1) |
| IMP-5 | ui-smoke rows for every new control; inventory regenerated | every UI card (R12 of SD-38) |
| IMP-6 | Architecture docs gain Print, Inventory/Slots and Import sections | E9.9 |
| IMP-7 | PF render-hash baseline re-recorded with explained deltas | `decisions.md §18` |

## 3. Candidates (not in the DoD)

| ID | Work | Source | Why not in SD-39's DoD | Revisit condition (checked at E9.5) |
|---|---|---|---|---|
| FSR-C1 | Writer tests for `SF_SEED_LINES_OUT` / `CPR_SHEET_LINES_OUT` create their out dir | `docs/retro/sd38-retrospective.md` change 1 | not a user-facing correction; **but** R14 below makes E8.3 build it if the incident fires once | `grep -rl 'sf-seed-lines-out-dir-missing' docs/retro/events \| awk 'END{print NR}'` rises during SD-39 |
| FSR-C2 | Full `npm run ui-smoke` as a gate; stale rows rewritten | SD-38 retro "did not work" (a) | ≈50 stale PF rows predate this bundle | E2.4's `npm run ui-smoke` report shows any red row SD-39 did not touch |
| FSR-C3 | Create's draft preview shows "29 lb carried" with no equipment | `../tranche-17-ui/brief-2-layout.md:57` | not dictated; E4.2 reads the source while wiring weight and records the cause | E4.2's receipt names the cause and whether draft and saved sheet differ |
| FSR-C4 | Level-up gating findings | tranche-17-ui open findings | not dictated | the operator names them |
| FSR-C5 | Release-build eyeball (tranche-17-ui) | same | superseded by E9.4's eyeball for print; UI eyeball not dictated | the operator asks |
| FSR-C6 | Graph re-index decision, licence sign-offs, FSR-C2 (SD-38 sample) | `../SD-38-cyberpunk-red/README.md §6` | operator decisions outside this bundle | the operator rules |
| FSR-C7 | Remove `.worktrees/ci-opt-level` once PR #401 merged | memory `pr-gate-root-cargo-test-skips-crates` | #401 is merged (`5ef2f17b67`); the tree is not in `git worktree list` on 2026-11-01, so nothing to do | `git worktree list \| grep -c ci-opt-level` → 0 |

## 4. Planned capability deferrals

The test (`docs/governance/deferral-revisit-doctrine.md`): was this in the DoD when scoped? No, by
operator ruling in the grill.

| ID | Capability | Operator ruling | Revisit condition (a command, checked at E9.5) | Accepted cost |
|---|---|---|---|---|
| DEF-1 | Magic weapon/armor special abilities (Flaming, Fortification …) and special materials (mithral …) | R14: "later" | the operator asks: `grep -rli 'special abilit' docs/release/SD-*/scope-draft.md \| grep -v SD-39 \| awk 'END{print NR}'` ≥ 1 | Market offers +N only |
| DEF-2 | Starfinder armor-upgrade slots; Cyberpunk install locations | R15 | same grep for 'armor upgrade' / 'install location' | SF/CPR inventory stays flat |
| DEF-3 | PCGen XML output import | R13 option 3 declined | same grep for 'pcgen xml' | `.pcg` only |
| DEF-4 | Pixel-snapshot print tests | R6 option 2 declined | same grep for 'pixel snapshot' | text + eyeball |
| DEF-5 | GM-override path beyond the Overrides toggles | R10 option 3 → became R10b toggles | n/a (superseded) | — |
| DEF-6 | Editing a past level's choice from the drawer | author call in `decisions.md §11` | the operator asks (same grep for 'edit past level') | drawer is read-only |

If the operator says any of these was meant to be in the DoD, it becomes a blocker.

## 5. No deferrals for SD-39 blockers

No card in `kanban.md` may be moved here by a cycle. A card that cannot close is `blocked-escalated`.
