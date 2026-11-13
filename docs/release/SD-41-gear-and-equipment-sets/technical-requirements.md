---
canonical: true
owner: operator
bundle_id: SD-41
slug: gear-and-equipment-sets
canonical_branch: tranche-21
target_version: 0.21.0
date: 2026-11-12
status: planning-ready
---

# SD-41 Technical Requirements

> **`decisions.md §20` wins where this file differs.**

## 1. Pre-launch prerequisites (each with its check; C1 runs P-1, P-2, P-4, P-7 and pastes the output; the others at the card named)

| ID | Prerequisite | Check | Closes at |
|---|---|---|---|
| P-1 | `tranche-21` exists on origin at `tranche-20`'s tip of 2026-11-12 | `git ls-remote --heads origin tranche-21 \| awk '{print $1}'` = `f7f85f3d24…` or a descendant (`git merge-base --is-ancestor f7f85f3d24 origin/tranche-21`) | C1 |
| P-2 | Working tree clean; `sd40-main` untouched | `git status --porcelain \| wc -l` → 0; `git worktree list` recorded | C1 |
| P-3 | **SD-40 merged and `tranche-21` caught up** (the Phase-2 precondition, R25) | `git merge-base --is-ancestor <SD-40 merge sha> HEAD` in `sd41-main`; `kanban.md` E2.1's `SD-40 merged` token replaced by the SHA | E2.1 (the lead) |
| P-4 | Doctrine present | `test -f docs/governance/no-stub-mvp-doctrine.md && test -f docs/doctrine-external/identifier-discipline.md` | C1 |
| P-5 | Disk ≥ 24 G free before any full sweep; cargo lanes on the box < 3 | `df -h /`; `pgrep -f '[c]argo' \| awk 'END{print NR}'` | every cargo card |
| P-6 | `pdftoppm`, `pdftotext`, `pdfinfo` present; the `run-desktop` skill's screenshot path works with an isolated `XDG_DATA_HOME` | `which pdftoppm pdftotext pdfinfo` → three paths; one screenshot under `artifacts/epic_3/E3.MC/` | E4.1; E3.MC |
| P-7 | Release-notes pointer resolves after C1 | `python3 tools/release/resolve_release_notes.py --repo-root .` exit 0 | C1 |
| P-9 | d20pfsrd.com reachable for E1.9's price list, or the lead's `web-fetch` agent supplies it | `curl -s -o /dev/null -w '%{http_code}' -A 'Mozilla/5.0' https://www.d20pfsrd.com/equipment/` → 200 (a HEAD without a user agent answered 410 on 2026-11-12); else the receipt names the agent that fetched | E1.9 |
| P-8 | The root `--lib` suite and the desktop suite are green on the base | `cargo test --locked -j 8 --lib -- --test-threads=8` 0 FAILED; `cd apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` 0 FAILED (counts quoted) | E1.1 (root baseline); E2.1 (desktop baseline, after the rebase) |

## 2. Normative requirements

| ID | Requirement | Verified by |
|---|---|---|
| NR-1 | No PCGen source, template, token or class structure in `src/`, `crates/`, `apps/desktop/**`, `data/`; the functional reference lives in `technical-design.md §1` only | G-7; `pcgen-residue-gate` |
| NR-2 | No weight, AC or capacity is computed outside Rust; the only new sum is `containers.rs`; TS and Typst format strings the engine gives | G-6's grep; E2.MC, E3.MC, E4.MC read the diffs |
| NR-3 | The saved input format (`equipment=` lines) and `EquipmentSelection` are unchanged; sets live in `equipment_sets.json`; `CURRENT_SAVED_CHARACTER_SCHEMA_VERSION` stays 2 | `git diff <C1 sha>..HEAD -- src/rules_core/character_input.rs \| grep -c 'pub struct EquipmentSelection'` → 0; `grep -n 'SCHEMA_VERSION: u16 = 2' src/saved_character/mod.rs` |
| NR-4 | A copy a set does not name is Carried; a bought copy writes no set entry | E1.2 `equipment_sets_unnamed_copy_is_carried`; E2.2 `move_equipment_purchase_is_carried_everywhere` |
| NR-5 | For a Default set, projection reproduces the saved states copy by copy; AC and weight unchanged on first load of an old save | E1.2 theorem test + the `.pcg` population test; E2.1 old-save test; G-1 |
| NR-6 | AC counts a copy only when its location is a slot or Equipped; weight counts a copy only when Carried, slotted, Equipped, or inside a container that is one of those | E2.2 `move_equipment_shield_into_slot_moves_the_ac_row`, `move_equipment_not_carried_container_drops_contents_weight` |
| NR-7 | Container-ness = the `Container` tag; capacity = the `CONTAINS` token's weight; `Unlimited` never over; `Unstated` shows "no stated capacity"; over-capacity warns and never blocks | E1.1, E1.3 tests; E3.1 model test |
| NR-8 | A cycle, a non-container target, an unknown copy and a full slot are refused with a diagnostic the screen shows word for word | E2.2 tests; E3.1 |
| NR-9 | Sets: unique trimmed names 1–40 chars; the last set cannot be deleted; deleting the active set activates another; activating projects and saves | E1.2, E2.2 tests |
| NR-10 | Weapons: in-hand rows first, carried rows after and marked, Not Carried absent — on the tab and the print; existing in-hand rows byte-identical | E1.4, E2.4, E4.1 tests; G-1 |
| NR-11 | A custom item with a base weighs its base's weight and is a container iff the base is; without a base it is unchanged | E2.3 tests |
| NR-12 | The gear screen's every move is reachable by keyboard through "Move to"; drag calls the same boundary with the same arguments | E3.1, E3.2 tests; E3.MC's eye |
| NR-13 | The 13 named ui-smoke rows pass by name; none deleted; the inventory regenerated | E3.3; G-6 |
| NR-14 | Every screen and print card's receipt has a "Looked at" row naming its images | G-8's `grep -L` |
| NR-15 | The Weapons block always ends with the Unarmed row; its numbers are engine rows built from the corpus record (no literal die in code) | E1.5 tests; `grep -c '"1d3"' src/rules_core/damage_total.rs` → 0 |
| NR-16 | Ammunition boxes appear only under weapons whose record carries the `Projectile` tag; no global ammunition row; tab and print agree | E1.6, E2.4, E4.1 tests; G-9 |
| NR-17 | No Market row has a missing, zero or negative price unless its key is in `price-unknown.json` with a reason; every d20pfsrd price carries its URL | G-10's gate test |
| NR-18 | No TS computes a price: quantities, negotiated totals and Buy Custom quotes come from the backend; `refundPreviewCopper` stays the one mirrored rule | E3.4 model tests; E3.MC's read |
| NR-19 | Stacks change nothing on disk; a stack is `quantity` identical copies the engine already weighs one by one | E2.1 tests; G-1 |
| NR-20 | Every writer of a file under a character root mirrors to every active campaign folder after its write; a bad folder is a warning, never a failed local save; two characters never share a campaign file | E1.10 tests; E2.7's scanning test; G-11 |
| NR-21 | The campaign copy is one-way: no code path reads a character from a campaign folder | `grep -rn 'characters/' src/saved_character/campaign_mirror.rs \| grep -c 'read'` → 0; E2.MC's read |

## 3. Out of scope

DEF-1…DEF-6 and FSR-C2…C5 (`forward-scope-register.md`); any converter change beyond E1.9's price derivation;
any engine change beyond E1.1–E1.7's named functions and fields; any book content; SF1e and CPR gear screens; the Hermes kanban
(retired).
