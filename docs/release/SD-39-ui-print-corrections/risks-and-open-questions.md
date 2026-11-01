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
| R-4 | Converter re-run (E1.0) changes unrelated sheet-rule output | low | high | `sheet_rule_convert --check` both systems; a diff of `data/sheet_rules/` limited to the three new fields (`git diff --stat` + a jq check that only `slot`/`stackable`/`enhanceable` keys changed) | yes if the diff is wider: stop, report |
| R-5 | Default-to-carried on add (E4) changes every PF seed's AC | certain for seeds with armor | high | E4.2 sets seeds' existing armor `EquippedActive` in the migration (legacy items keep their current state = worn); only **new** adds default to carried; receipt lists every seed delta (`decisions.md §18`) | yes |
| R-6 | Ledger migration mis-assigns legacy feats (wrong level) | medium | low | rows marked `assigned_on_upgrade`; the drawer says so; no sheet number depends on `taken_at_level` | n/a (by design) |
| R-7 | Shared-file collisions: `CharacterSheet.tsx`, `character_hub.rs`, `main.rs`, `spec.json` touched by E1–E8 | high if parallel | high | serial main lane for every card that touches them; the writer order in `epic-breakdown.md §0`; `git status` before every write; SD-38 retro change 6 → R19 | no — one writer |
| R-8 | Peer session's worktree (`fix-perform-install`) collides with the sweep | low | high | SD-f; E9.7 removes only `sd39-*` | no |
| R-9 | `.pcg` format variance (PCGen 6.0x vs 6.08 keys) | medium | medium | fixtures from the pinned oracle; unknown keys → custom items + report (SD-m) | yes |
| R-10 | Budget pools wrong for a class (bonus feats) → false refusals | medium | medium | pools from the class catalog, not a hand table (R2 of SD-36); `ignore_feat_budget` override is the escape hatch; the badge shows the pool math | yes |
| R-11 | Market replaces the Add buttons before Market is complete → a tab with no way to add items | low | high | E3.3 removes the buttons in the same card that lands the dialog; E3.MC checks every tab has an add path | yes |
| R-12 | Print "Show the math" has gaps where no explanations row exists | certain for some values | low | SD-a prints "(no derivation recorded)"; E8.5 counts them per seed and the receipt lists them (candidates for a later bundle) | n/a |
| R-13 | Clippy found only after commit (SD-38 retro change 3) | high | low | R16: `cargo clippy --locked --tests` before the first local commit of any `.rs` card | yes |

## 2. Open questions → defaults taken

| Question | Default | Where |
|---|---|---|
| Package path `docs/release` vs `docs/stc` | `docs/release` | `decisions.md §1` |
| Which Typst version | latest stable at E8.1; recorded | `decisions.md §4` |
| Print sink on Linux without a print handler | open the PDF in the viewer; the receipt says so | `technical-design.md §1` |
| Default state for newly bought items | carried, with an "equip now" checkbox in Market | `technical-design.md §2` |
| Bonus-type vocabulary for custom bonuses | the engine's existing stacking vocabulary; no new type | `decisions.md §13` |
| Ring slot count | 2 | `decisions.md §9` |
| Override extras (3) kept? | yes unless the operator strikes them before launch | `decisions.md §10` |

## 3. Non-self-healable conditions (card → `blocked-escalated`)

A gate PR not merged; two live lanes on one file; RED→GREEN not preserved; a stub, mock or "Would …"
string in shipping code; a converter diff wider than the three fields (R-4); the fresh-install check
impossible on this box (SD-l); `typst` unbuildable (SD-h); the operator's E9.4 verdict "not usable"
(becomes discovered E8 cards, then E9.4 re-runs).
