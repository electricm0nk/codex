---
title: SD-43 — Market, Inventory and the Abilities Tab — Release Package
status: planning — front half (README, scope-draft, decisions, epic-breakdown, technical-requirements, technical-design) authored 2026-11-22; the back half (workflow-instruction, kanban, progress, acceptance-and-verification, risks-and-open-questions, content-unit-inventory, forward-scope-register, artifacts/, references/) is authored next. Not planning-ready until the back half lands, the placeholder gate passes over all files, and the open questions in decisions.md §30 carry their defaults into the kanban.
bundle_id: SD-43
slug: market-inventory-and-abilities
scope: docs/release/SD-43-market-inventory-and-abilities
artifact_type: release-index
canonical_branch: tranche-43
kanban_board: local-file ./kanban.md
target_version: 0.43.0
canonical_source: docs/release/SD-43-market-inventory-and-abilities (this folder)
date: 2026-11-22
---

# SD-43 — Market, Inventory and the Abilities Tab — Release Package

> ## OPERATING METHOD — REQUIRED FOR THIS BUNDLE
>
> **This bundle runs as a `Workflow`-tool script invoked from a live session. It is NOT run with `/loop /batch`
> and it is NOT a one-shot task.** `workflow-instruction.md §2.4` (back half) says how to write the script.
> **Unattended after launch** except at **one operator wait: E4.G**, the operator's approval of the proposed
> item names (intake item 20 ruling, `decisions.md §22`). Every other open question has a default already taken
> (`decisions.md §30`). **One operator precondition:** the CI secret for the report API key (Q-13.2).
> `scope-draft.md` is the *what*; `workflow-instruction.md` is the *how*.

This folder is the canonical surface for SD-43. A harness needs only this folder, the intake it was authored
from (`../SD-43-intake/`), and the in-repo doc tree.

## 0. Preamble

The operator used SD-42's alpha and raised 24 intake items (0–23, `../SD-43-intake/intake.md`, opened
2026-11-18, last edited 2026-11-22). The operator's order of importance: **first** the Market and inventory
management (items 5–12: one-column gear list with correct container contents, double-click to equip, whole-box
drop targets, a standard Market button, a Sell window, item stats in the Market, filter columns with no 200-row
cap, sell values and container sales); **then** the Abilities tab (items 22, 22a, 22b: PCGen's seven sub-tabs —
Feats, Racial Abilities, Traits, Archetype, Background, Class Features, Misc — each showing what can be selected,
what is selected, the picks granted per level, enforcing grants and prerequisites in the engine as feats are
enforced today, and printed); **then** the rest (campaign folder relocation, rest-and-recovery text, House
Rules, custom item exchange, report API with an Anonymous selector, Update menu layout, the Developer tab, a
fixed sheet layout, the campaign button, placeholder item names). SD-43 delivers every item as a working first
version; SD-44 is the operator's feedback round on it, not a home for unfinished SD-43 scope
(`decisions.md §3`, ruling 22b-1).

## 1. Bundle snapshot

| Field | Value |
|---|---|
| Bundle ID | SD-43 |
| Slug | `market-inventory-and-abilities` |
| Canonical branch | `tranche-43`, cut from `origin/develop` @ `fb94a85049` (`git merge-base HEAD origin/develop` → `fb94a85049c18fd22b776a28c51eb527ec0ddef3`); renamed from `tranche-23` on 2026-11-22 (intake item 23); not yet on origin (`git branch -r \| grep -c tranche-43` → 0 on 2026-11-22) |
| Kanban board | local file `./kanban.md` (back half) |
| Epics / cards | C, E0–E7 / **56 cards** including one operator wait, E4.G (`epic-breakdown.md §0`; re-derive with `awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/' epic-breakdown.md \| awk 'END{print NR}'` → 56) |
| Target version | `0.43.0`, already landed in `147a3f71eb` (14 files: `git show --name-only --format= 147a3f71eb \| awk 'NF' \| awk 'END{print NR}'` → 14). `docs/release/current-release.json` still names tranche 22 — C1 moves it (`decisions.md §2`) |
| Published build | `0.43.<run>` (`.github/workflows/publish-tester-release.yml:129` `VERSION="0.43.${GITHUB_RUN_NUMBER}"`) |
| Predecessor | SD-42 (`../SD-42-pcgen-import-and-campaign-folder/`), merged as PR #413 |
| Successor | SD-44, the operator's feedback round on SD-43 (intake 22b-1) |
| Dispatch mechanism | `Workflow` tool, invoked from a live session (`workflow-instruction.md §2`) |
| Cadence | N/A |
| Closure gate | every card `complete` → retrospective written and cited → worktree/branch sweep → release notes → architecture truth-up → **graphify last** → **PR `tranche-43 → develop` last**; the lead merges when green and watches the develop publish to green (`decisions.md §26`) |
| Source | `../SD-43-intake/intake.md` (items 0–23, 22a, 22b) and the repo at `e451225461` |

## 2. Files in this folder

| File | Job | Author |
|---|---|---|
| `README.md` | this index | front half |
| `scope-draft.md` | every intake item with its disposition, the Definition of Done, non-goals, out of scope | front half |
| `decisions.md` | §1 location, §2 branch/board/version, §3 rulings verbatim, §4–§25 per-area decisions, §26 closure order, §27–§29 standing rules, §30 open questions with defaults, §31 corrections to the intake | front half |
| `epic-breakdown.md` | §0 parallel/serial map with tiers; per-card criteria, FILES YOU OWN, failing test first, acceptance command, does-not-cover | front half |
| `technical-requirements.md` | prerequisites P-n and normative requirements NR-n | front half |
| `technical-design.md` | today's code with file:line; the Abilities tab design; Market/inventory design; the sheet layout | front half |
| `workflow-instruction.md` | the *how*: prefix, per-cycle procedure, receipt schema, closure | back half |
| `kanban.md` | one row per card | back half |
| `progress.md` | live log; Cycle 0 receipt; open blockers | back half |
| `acceptance-and-verification.md` | gates; ruling → card → acceptance traceability | back half |
| `risks-and-open-questions.md` | ranked risks with controls; mirrors `decisions.md §30` | back half |
| `content-unit-inventory.md` | every figure with its command | back half |
| `forward-scope-register.md` | what SD-44 inherits (feedback only) and the real exclusions | back half |
| `artifacts/`, `artifacts/README.md` | per-epic receipts and census output | back half; populated per cycle |
| `references/`, `references/README.md` | doctrine, skills, sibling bundles, retrospectives | back half |

`receipts.md` and `release-notes.md` are created by cards (C1 writes the interim notes; E7.3/E7.4 the closure
receipts).

## 3. In-repo cross-references

- Conduct: `../../../AGENTS.md`, `../../../CLAUDE.md`.
- Doctrine: `../../governance/no-stub-mvp-doctrine.md` (the four-check per-cycle audit),
  `../../governance/blocker-closure-doctrine.md`, `../../governance/deferral-revisit-doctrine.md`,
  `../../governance/wired-integration-stubs-registry.md`, `../../doctrine-external/identifier-discipline.md`.
- Templates: `../template/template.md`, `../../governance/workflow-instruction-template.md`.
- Architecture: `../../architecture/` (truthed-up at E7.3).
- The twin rule: `../SD-27-future-state-book-content-ingestion/decisions.md §29.1`.

## 4. Relationship to other release folders

- **SD-42** (`../SD-42-pcgen-import-and-campaign-folder/`): predecessor, merged (PR #413). SD-43 changes screens
  SD-42 built: the gear screen's container tables and per-copy names (`decisions.md §27.3–§27.5` there), the
  campaign folder (`§11` there), the open-campaign banner (`§26.2` there).
- **SD-41** (`../SD-41-gear-and-equipment-sets/`): stacks, `CopyFacts`, equipment sets, the Market's price modes.
- **SD-43 intake** (`../SD-43-intake/`): the cited source. Untouched by this package.
- **Signed update manifest** (`../SD-43-intake/signed-update-manifest-brief.md`): **not SD-43 work.** The brief
  places itself outside SD-43 (its header, operator correction 2026-11-21) and it is already merged
  (`git merge-base --is-ancestor 2225c0111c HEAD && echo in` → `in`; PR #419). SD-43 carries no card for it;
  `decisions.md §25` records this and the one thing SD-43 reuses from it (the CI-secret pattern for item 13).

## 5. Build version target

`major` 0; minor **43** = the SD number (intake item 23 ruling, 2026-11-22; replaces "minor = tranche
number"); `build` = the GitHub run number, stamped at publish. Repo files read `0.43.0`
(`awk '/^name = "codex-desktop"/{f=1} f&&/^version/{print;exit}' apps/desktop/src-tauri/Cargo.lock` →
`version = "0.43.0"`).

## 6. Author's calls not covered by an operator ruling (review before launch)

| # | Call | Default taken | Where |
|---|---|---|---|
| 1 | One Abilities tab or a tab beside Feats | `Feats` in `SHEET_TABS` becomes `Abilities`, holding seven sub-tabs with Feats first; no second feat picker | `decisions.md §15`, §30 Q-22.3 |
| 2 | How a sub-tab's grants and prerequisites are modeled | one engine module per the `feat_budget.rs` shape: `abilities.budget.<subtab>.<pool>.{granted,spent,remaining}` rows plus a `refusal` string; the UI reads rows only | `decisions.md §16`, `technical-design.md §3` |
| 3 | Overrides on the six new sub-tabs | the existing `Overrides` mechanism in `custom.json`, two new flags | `decisions.md §17` |
| 4 | Where the House Rules live | the same per-character `custom.json` `Overrides`, four of six flags already exist | `decisions.md §11` |
| 5 | The rest-and-recovery text source | a committed, cited rules-text table, since the corpus has no per-class rest record | `decisions.md §8`, §30 Q-2.1 |
| 6 | The report API key | a GitHub Actions secret stamped at build time; dev builds disable Send with a stated reason | `decisions.md §23` |
| 7 | Campaign folder relocation | copy, verify, switch; the old folder is kept; a non-empty target is refused unless it is the same campaign | `decisions.md §7` |

## 7. Initial-package construction

Authored on 2026-11-22 on `tranche-43` @ `e451225461` in the shared checkout (one writer; `git status --porcelain`
empty before the first write) from `../SD-43-intake/intake.md`. Every path and line number in these files was
checked with `grep -n`/`sed -n`/`wc -l` on that commit; every figure carries its command. Code was read, not
run, except the census commands quoted in `decisions.md §31` and `technical-design.md`. The front half was
authored by a dispatched agent; the back half follows.

## 8. Placeholders in this package (the gate: `grep -rn '<[a-z_-]*>' docs/release/SD-43-market-inventory-and-abilities/*.md`)

| Token | Kind | Resolves at |
|---|---|---|
| `<run>` | the GitHub run number in `0.43.<run>` | each publish (`publish-tester-release.yml:129`) |
| `<pool>`, `<subtab>` | schema placeholders in explanation-row ids (`feats.budget.<pool>.granted`, `abilities.budget.<subtab>.<pool>.granted`) | never; they name a family of rows |
| `<lane>` | worktree/target-dir name in `epic-breakdown.md §0` | the lane column of the same table |

Any other hit is a defect.

## 9. Cross-reference

- `../template/template.md`, `../../governance/workflow-instruction-template.md` — the two templates.
- `../../../.claude/skills/stc-authoring/SKILL.md` — the repo-local authoring skill.
