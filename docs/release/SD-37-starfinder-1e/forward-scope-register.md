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

## 4. Planned capability deferrals

| ID | Capability | Test: was it in the DoD at scoping? | Revisit condition (a command, checked) | Accepted cost |
|---|---|---|---|---|
| **DEF-1** | **Starship sheet** (tier, BP, frame, systems, crew roles) | **No.** The operator asked for Starfinder 1e character sheets, not starships. No oracle data exists: the core PCC's starship add-on is commented out (`_starfinder_core_rulebook.pcc` lines 84–88) and `paizo/core/starship/` is absent (`decisions.md §17`) | `test -d "$PCGEN_REPO_DIR/data/starfinder/paizo/core/starship" \|\| grep -lE '^ABILITY:starship\|^EQUIPMENT:starship' "$PCGEN_REPO_DIR"/data/starfinder/paizo/core/*.pcc` (on 2026-10-02 it prints nothing, exit 1). Run it at E7.3 and at every tranche cut. Any output → operator ruling | No starship sheet. Starship-related gear, spell and ability prose still prints where records exist (SD-f) |

**This deferral tests the blocker-vs-deferral line.** It is a deferral only because starship
scope was never in the Definition of Done. If the operator says it was, then DEF-1 becomes a
blocker, and SD-37 is not done without it.

## 5. No deferrals for SD-37 blockers

No card in `kanban.md` may be moved here by a cycle. A card that cannot close is
`blocked-escalated` and waits for the operator (`workflow-instruction.md §8`).
