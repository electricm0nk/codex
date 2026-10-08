---
canonical: true
owner: operator
bundle_id: SD-37
date: 2026-10-02
---

# SD-37 Forward Scope Register

This is the planning entry point for SD-37. It records what SD-37 inherits and what it pushes
forward. Every row falls into one of three classes:

- **Committed**: in SD-37's Definition of Done, so it is a card. A shortfall is a blocker.
- **Structurally implied**: it falls out of committed work.
- **Candidate**: not in the DoD. Each candidate names a revisit condition that is **checked** at
  E7.3 and at the next scoping pass.

Only the operator moves scope between classes (`docs/governance/blocker-closure-doctrine.md`).

## 1. Committed (adopted into SD-37)

| ID | Work | Source | SD-37 cards |
|---|---|---|---|
| ADOPT-1 | Second PCGen-format reader (Starfinder include structure + formula vocabulary) | SD-35 FSR C2.1 (named owner "SD-36"; SD-36 has no C2.1 row: `grep -n 'C2.1' docs/release/SD-36-consolidation/forward-scope-register.md` → no output) | E3.1–E3.MC |
| ADOPT-2 | `rules_tables` → data package | SD-36 FSR FS-3; SD-36 ruling 7; memory `rules-tables-stay-rust-until-starfinder` | E4a.1–E4a.MC |
| ADOPT-3 | `.lst` citation burn-down (SD-36 D6, second half) | SD-36 scope-draft D6 | E4a.3 (D6 status re-derived first) |
| ADOPT-4 | SF oracle in the sparse pin (part of FS-5) | SD-36 FSR FS-5 | E0.1 |

## 2. Structurally implied

| ID | Work | Falls out of |
|---|---|---|
| IMP-1 | Per-system book registries replacing the 19 consts (CUI F-13) | E1.2 |
| IMP-2 | A published rules schema (`schemas/rules/`) | E2.2, E4a.1 |
| IMP-3 | Stub registry 0002 retired for `starfinder-1e` | E4.6 |
| IMP-4 | `verify-baselines.env` re-derived for two systems | E7.2 |
| IMP-5 | Architecture docs gain a Starfinder section | E7.7 |

## 3. Candidates (not in the DoD; not gates)

| ID | Work | Source | Why not in SD-37's DoD | Revisit condition (checked at E7.3) |
|---|---|---|---|---|
| FSR-C1 | 8 open SD-34 fable-review P1 correctness rows | SD-36 FS-2 (count quoted, not re-derived; its link names the wrong dir, the real one is `docs/release/SD-34-book-completion`) | PF correctness; the operator asked for Starfinder; FS-2's "should land before Starfinder" is a soft gate, overridden by default (`decisions.md §10`) | At E7.3, re-derive the open-P1 count from `docs/release/SD-34-book-completion/` and report it. Any P1 whose fix path touches a file SD-37 changed is re-opened for an operator ruling |
| FSR-C2 | FS-15: 6 of 74 prestige mixes Blocked on oracle save-formula precedence | SD-36 FS-15 | PF content | `cargo run --locked -j 8 --bin class_census -- --json <p>` at E7.2: if `prestige_mix_computed` ≠ 68, report it |
| FSR-C3 | FS-27 (4 Unchained classes refused in a mix), FS-28 (33 catalog `Chosen` summaries print a choice id) | SD-36 FS-27/28 | PF content | E7.2 census + catalog dump. Report the counts. Re-open if E1/E4a changed either count |
| FSR-C4 | FS-7: thread the source book through `HeldSeed`/`ChosenCharacterState` (PF) | SD-36 FS-7 | Cross-system collisions are handled by system namespacing (SD-h); the PF in-system problem is unchanged | At E7.3: if any SF `(kind, slug)` collides within SF itself (E0.3's inventory reports it), re-open |
| FSR-C5 | FS-9: PF oracle-parity roster widening | SD-36 FS-9 | PF | E7.1 builds the SF roster. If the SF runner change makes PF widening a one-line roster edit, re-open |
| FSR-C6 | Half-Orc/Half-Elf free +2; armour check penalty on Dex skills (PF) | SD-36 Epic F next-week list (memory `sd36-epic-f-class-completion`) | PF | If E4.2's ACP implementation is generic over systems, re-open the PF ACP item at E7.3 |
| FSR-C7 | FS-6: widen `pcgen_residue_gate.py` `TOKEN_SYNTAX_PATTERNS` to the full token-head vocabulary | SD-36 FS-6 | Partly touched by E3 (SF token heads) | At E7.3: if E3 added SF heads to the gate, compare against the full vocabulary and report the remainder |
| FSR-C8 | Move the package to `docs/stc/` or amend the overlay | `decisions.md §1` | Repo convention vs overlay | When the operator rules on §1 |
| FSR-C9 | **Fix `docs/governance/workflow-instruction-template.md §11` closure order** (release notes before graphify; graphify last; PR last) | `decisions.md §13`; memory `graphify-runs-against-final-repo-state` | This package's write scope excludes `docs/governance/` | Next bundle's scoping pass, or a governance cycle the operator grants. This bundle's instance is already corrected |
| FSR-C10 | Mark the SD-36 `PF1e-dashboard.json` `usage` block as frozen/stale in its README | `decisions.md §12.3` | Not SD-37 scope | When the next quota-rule author reads it |

### 3.1 E7.3 revisit results (2026-10-07, on `476487c54d`)

Each row above was checked by `bash artifacts/epic_7/E7.3_logs/fsr_checks.sh`, run from the repo
root (output in `artifacts/epic_7/E7.3_logs/fsr_checks.txt`; receipt
`artifacts/epic_7/E7.3_cycle_receipt.md`).

| ID | Result | Condition met? |
|---|---|---|
| FSR-C1 | **0** of the 8 SD-34 P1s are open. 6 were FIXED by SD-36 Epic E, and 2 (R9-02, R11-02) became moot when SD-36 Epic B deleted their files. The "8 open" figure was stale (retro correction). | No |
| FSR-C2 | `prestige_mix_computed=68`, in E7.2 `verify-full.out` and again in E7.3's own re-run (`E7.3_logs/class-census.log`) | No |
| FSR-C3 | FS-27: the 4 Unchained refusals are pinned by `level_up_options_name_the_engines_mix_refusal_before_accept`, which is `ok` in E7.2's desktop log. FS-28: not re-counted, because SD-36 recorded no command for its 33. Its inputs are unchanged since `20bf84a3b2`: `data/sheet_rules` has 0 diff lines, and `fact_words`/`weapon_words` are untouched. | No |
| FSR-C4 | **31** Starfinder `(kind, slug)` pairs are held in 2 SF books. The inventory and `data/starfinder-1e/sheet_rules` agree (31 = 31). **1** of them has a different `value` in each book: `equipment:needler_rifle` is core `Dice 1d6` and COM `Text`. SF builds seed by book-qualified rule id (`sf_defense.rs` `HeldSeed { rule_ids }`), not by bare slug, so the FS-7 mechanism is not on the SF seed path. But `SheetRulePackage::find` prefers only `core_rulebook:` and otherwise takes the alphabetical minimum, so a bare-slug `find` on an SF package picks `character_operations_manual` over `core`. | **Yes.** A decision waiting for the operator. Not a gate: §3 candidates are not in the DoD. |
| FSR-C5 | E7.1 added 22 new files under `scripts/oracle_harness/` (all `A`) and changed no PF runner file | No |
| FSR-C6 | SF ACP is `armor_check_penalty(package, build: &SfBuild)`, which is Starfinder-only | No |
| FSR-C7 | `scripts/pcgen_residue_gate.py` is unchanged since `20bf84a3b2` (0 diff lines) | No |
| FSR-C8–C10 | These are revisited at an operator ruling, at the next scoping pass, or by the next quota-rule author. None is checkable at E7.3. | Not checkable at E7.3 |
| DEF-1 | The `decisions.md §17` command prints no stdout and exits 0, with the oracle at the pin | No |

## 4. Planned capability deferrals

| ID | Capability | Test: was it in the DoD at scoping? | Revisit condition (a command, checked) | Accepted cost |
|---|---|---|---|---|
| **DEF-1** | **Starship sheet** (tier, BP, frame, systems, crew roles) | **No.** The operator asked for Starfinder 1e character sheets, not starships. No oracle data exists: the core PCC's starship add-on is commented out (`_starfinder_core_rulebook.pcc` lines 84–88) and `paizo/core/starship/` is absent (`decisions.md §17`) | The fenced command in `decisions.md §17` (resolves the oracle via `fetch-pcgen-oracle.sh --check`, exits 2 if it cannot check, prints one line per starship directory or uncommented `ABILITY/EQUIPMENT/RACE/KIT:…starship` include in any SF PCC). C0.2 ran it on 2026-10-02: no output, exit 0. Run it at E7.3 and at every tranche cut. Any output → operator ruling; exit 2 → the check failed, E7.3 is short. (C0.2 replaced the authoring `test -d … \|\| grep …` form, which printed nothing both when the directory existed and when `$PCGEN_REPO_DIR` was unset.) | No starship sheet. Starship-related gear, spell and ability prose still prints where records exist (SD-f) |

**This deferral tests the blocker-vs-deferral line.** It is a deferral only because starship
scope was never in the Definition of Done. If the operator says it was, then DEF-1 becomes a
blocker, and SD-37 is not done without it.

## 5. No deferrals for SD-37 blockers

No card in `kanban.md` may be moved here by a cycle. A card that cannot close is
`blocked-escalated` and waits for the operator (`workflow-instruction.md §8`).

## Added at closure (2026-10-08)

| ID | Class | Item | Revisit condition |
|---|---|---|---|
| FSR-C10 | candidate | graphify's node-count guard refuses every `cluster-only` write with net −1 (SD-36 2026-09-27, SD-37 E7.8); name the node the loader drops and make the guard a control, not a stop; then re-index SD-37 content with a semantic `--update` | the next bundle's E0: `grep -c 'graphify-node-count-guard' docs/retro/events/*.jsonl` ≥ 2 |
