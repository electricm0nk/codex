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

# SD-39 Risks and Open Questions

## 1. Ranked risks

| # | Risk | Likelihood | Impact | Control | Self-healable? |
|---|---|---|---|---|---|
| R-1 | **Fourth print attempt also unusable** because tests pass on text and nobody looks at paper | medium | high | E9.4 operator eyeball is a hard gate; E8.5's page-count and overflow-zero asserts; the layout file never names inches | no — E9.4 waits |
| R-2 | `typst` crate build time or CI failure (large dependency tree) | medium | medium | P-4 at E8.1 start; `CARGO_PROFILE_TEST_OPT_LEVEL` already set in CI (PR #401); SD-h escalates with the alternative named | no |
| R-3 | Fonts missing from the packaged app → PDF falls back or fails on a clean install | medium | high | fonts in `bundle.resources` + `packaged_resources.rs` assertion (E9.1); E9.2 fresh-install prints a sheet | yes (add the resource) |
| R-4 | E1.0 finds `repeatable` or tags wrong, and a converter run is needed | low | high | `sheet_rule_convert --check` both systems; the run is limited to the failed field | yes if bounded: stop, report if wider |
| R-5 | Wear-if-free on buy surprises a build | low | low | the message names the state; saved items keep their state, so no seed AC changes | yes |
| R-6 | Ledger migration mis-assigns legacy feats (wrong level) | medium | low | rows marked `assigned_on_upgrade`; the drawer says so; no sheet number depends on `taken_at_level` | n/a (by design) |
| R-7 | Shared-file collisions: `CharacterSheet.tsx`, `character_hub.rs`, `main.rs`, `spec.json` touched by E1–E8 | high if parallel | high | serial main lane for every card that touches them; the writer order in `epic-breakdown.md §0`; `git status` before every write; SD-38 retro change 6 → R19 | no — one writer |
| R-8 | A session started later collides with the sweep | low | high | SD-f; E9.7 removes only `sd39-*` | no |
| R-9 | `.pcg` format variance (PCGen 6.0x vs 6.08 keys) | medium | medium | fixtures from the pinned oracle; unknown keys → custom items + report (SD-m) | yes |
| R-10 | Budget pools wrong for a class (bonus feats) → false refusals | medium | medium | pools from the class catalog, not a hand table (R2 of SD-36); `ignore_feat_budget` override is the escape hatch; the badge shows the pool math | yes |
| R-11 | Market replaces the Add buttons before Market is complete → a tab with no way to add items | low | high | E3.3 removes the buttons in the same card that lands the dialog; E3.MC checks every tab has an add path | yes |
| R-12 | Print "Show the math" has gaps where no explanations row exists | certain for some values | low | SD-a prints "(no derivation recorded)"; E8.5 counts them per seed and the receipt lists them (candidates for a later bundle) | n/a |
| R-14 | Sidecar missing from one platform's bundle | medium | medium | `tauri.sidecar.json` + all three publish jobs (E7.2); Windows and macOS are only proved by CI at publish after the merge, **not by this run** | no — stated in the notes |
| R-15 | An imported character holds Weapon Focus and the `power_attack` row its PCGen file does not, because Create gives every character the same. E7.4 took the added Longsword away (`pcg_import` adds no gear) | certain, by design of the engine today | medium | each is an `added` line in the import report (E7.1); widening the baseline is outside this bundle's scope | no |
| R-13 | Clippy found only after commit (SD-38 retro change 3) | high | low | R16: `cargo clippy --locked --tests` before the first local commit of any `.rs` card | yes |

## 2. Open questions → defaults taken

| Question | Default | Where |
|---|---|---|
| Package path `docs/release` vs `docs/stc` | `docs/release` | `decisions.md §1` |
| Which Typst version | latest stable at E8.1; recorded | `decisions.md §4` |
| Bonus-type vocabulary for custom bonuses | the engine's existing stacking vocabulary; no new type | `decisions.md §13` |
| Ring slot count | 2 | `decisions.md §9` |
| Override extras (3) kept? | yes unless the operator strikes them before launch | `decisions.md §10` |

## 3. Non-self-healable conditions (card → `blocked-escalated`)

A gate PR not merged; two live lanes on one file; RED→GREEN not preserved; a stub, mock or "Would …"
string in shipping code; a converter diff wider than the three fields (R-4); the fresh-install check
impossible on this box (SD-l); `typst` unbuildable (SD-h); the operator's E9.4 verdict "not usable"
(becomes discovered E8 cards, then E9.4 re-runs).
