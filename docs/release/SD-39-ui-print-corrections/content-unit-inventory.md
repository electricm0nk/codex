---
canonical: true
owner: operator
bundle_id: SD-39
slug: ui-print-corrections
canonical_branch: tranche-19
target_version: 0.19.0
date: 2026-11-01
status: planning-ready — every figure carries the command that produced it; measured on develop @ 5ef2f17b67 (local) on 2026-11-01 unless stated
---

# SD-39 Content Unit Inventory

A corrections bundle counts controls, records and surfaces rather than book content. Every figure has
its command and its denominator. Figures marked **(E1.0)** are measured by that card and written back
here in its commit.

## §1 Version and package surfaces

| F | Figure | Value | Command |
|---|---|---|---|
| F-1 | Files carrying `0.18.` that C1 bumps to `0.19.` | **14** | `grep -rln '0\.18\.' --include=*.json --include=*.toml --include=*.lock --include=*.ts --include=*.yml --include=*.py apps .github tools docs/architecture docs/release/current-release.json \| grep -v node_modules \| grep -v target \| awk 'END{print NR}'` |
| F-2 | C1 commit file count (F-1 + `current-release.json`) | **15** | `git show --name-only --format= <c1-sha> \| awk 'NF' \| wc -l` |
| F-3 | Cards in this bundle | **54** | `awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/' docs/release/SD-39-ui-print-corrections/kanban.md \| awk 'END{print NR}'` |
| F-4 | Chassis files present | **16** (13 required + `technical-design.md`, `technical-requirements.md`, `kanban.md`) | `find docs/release/SD-39-ui-print-corrections -maxdepth 2 -name '*.md' \| awk 'END{print NR}'` |

## §2 Surfaces the defects name (the before state)

| F | Figure | Value | Command |
|---|---|---|---|
| F-5 | Lines in `CharacterSheet.tsx` | **4739** | `wc -l < apps/desktop/src/characterHub/CharacterSheet.tsx` |
| F-6 | `window.print()` call sites in the desktop frontend | **1** (`CharacterSheet.tsx:2854`) | `grep -rn 'window.print' apps/desktop/src --include=*.tsx --include=*.ts \| grep -v test \| awk 'END{print NR}'` |
| F-7 | PDF/print crates in either cargo lockfile | **0** | `grep -c '^name = "\(typst\|printpdf\|genpdf\|lopdf\|pdf-extract\)"' Cargo.lock apps/desktop/src-tauri/Cargo.lock` |
| F-8 | Font files shipped | **0** | `find apps/desktop -name '*.ttf' -o -name '*.otf' \| grep -v node_modules \| awk 'END{print NR}'` |
| F-9 | Add-picker kinds in `buildItemPickerConfig` | **7** (`weapon, armor, gear, spell, feat, featTarget, modifier`) | `grep -n "kind: 'weapon'" apps/desktop/src/characterHub/CharacterSheet.tsx` then read the union at line 269 |
| F-10 | Equipment commands that take an `active_state` | **2** (`AddEquipmentSelectionRequest`, `EquipmentSelectionImportDto`) | `grep -c 'active_state: ActiveStateDto' apps/desktop/src-tauri/src/character_hub.rs` |
| F-11 | Rust files whose `ActiveState` match arms the worn/carried rule must prove | **5** (`character_input.rs`, `pilot_compute_corpus.rs`, `encumbrance.rs`, `contract.rs`, `damage_total.rs`) | `grep -rln 'ActiveState::' src/rules_core/*.rs \| awk 'END{print NR}'` |
| F-12 | Slot-location data in `data/sheet_rules/*/equipment/*.json` | **7,386** records; **2,906** with a slot tag; **1,617** with a `Slot:` line; **27** differ: 12 `wrists` spelling, 7 `does not take up slot`, 8 `Slot: none` on a weapon tag (the first-written 19 left out the 8) | `cargo test --locked -j 8 --lib slot_of -- --test-threads=8 --nocapture` |
| F-13 | Records `slot_of` resolves to a body slot | **2,838** of 7,386 rows; **2,130** of 6,223 principal rows (the unit a character holds); predicate: `slot_of` is not `none` | `cargo test --locked -j 8 --lib slot_of -- --test-threads=8 --nocapture`; a second implementation in Python over the same files agrees |
| F-14 | PCGen `SLOTS:` in Core Rulebook lists = a count of hands or feet used, not a location, and not weapon handedness: the 2s are boots, gloves and bracers | values 2 (30), 1 (3), 0 (3) | `awk 'BEGIN{FS="\t"} {for(i=1;i<=NF;i++) if($i ~ /^SLOTS:/) c[$i]++} END{for(k in c) print k,c[k]}' $PCGEN_CORPUS_ROOT/pathfinder/paizo/roleplaying_game/core_rulebook/*.lst` |
| F-15 | `repeatable: true` feats vs PCGen `MULT:YES` | **2,927** agree, **0** disagree, of 2,927 PF feat rows (207 repeatable); the other way, 172 of 174 `CATEGORY:FEAT` + `MULT:YES` rows in shipped books sit on a repeatable record and 2 are on no feat record | `cargo test --locked -j 8 --lib repeatable_equals -- --test-threads=8 --nocapture` |
| F-16 | `.pcg` fixtures available | **5** in `scripts/oracle_harness/eqm-fixtures/` + **(E7.1)** under `$PCGEN_REPO_DIR` | `ls scripts/oracle_harness/eqm-fixtures/*.pcg \| awk 'END{print NR}'`; `find "$PCGEN_REPO_DIR" -name '*.pcg' \| awk 'END{print NR}'` |
| F-17 | Feat-add commands that enforce prerequisites today | **1** (`add_feat_selection_enforcing_prerequisites_at_root`) | `grep -c 'enforcing_prerequisites_at_root' apps/desktop/src-tauri/src/character_hub.rs` (≥ 1) |
| F-18 | Feat-budget or duplicate guards today | **0** | `grep -rn 'feat_budget\|feats_remaining\|duplicate feat' apps/desktop/src-tauri/src src/rules_core \| awk 'END{print NR}'` |

## §3 Seeds and gates

| F | Figure | Value | Command |
|---|---|---|---|
| F-19 | PF seeds in the render-hash harness | **2** (Aldric Fighter 3, Elowen Wizard 5) | `grep -c 'Aldric\|Elowen' apps/desktop/src-tauri/src/pf_seed_render_hash.rs` (≥ 2) |
| F-20 | SF seed-line files; CPR seeds | **4**; **3** | `../SD-38-cyberpunk-red/acceptance-and-verification.md` G-1; `grep -c 'StarterSeed' apps/desktop/src-tauri/src/character_hub.rs` |
| F-21 | `verify.sh` stages (full) | **54** | `bash scripts/verify.sh --list \| awk 'NR>1 && $2=="yes"' \| awk 'END{print NR}'` |
| F-22 | ui-smoke rows | **(E2.4)** before / after | `python3 -c "import json;print(len(json.load(open('apps/desktop/scripts/ui-smoke/spec.json'))['rows']))"` |

## §4 Print blocks (the design's denominator for G-10)

| F | Figure | Value |
|---|---|---|
| F-23 | Block types | **15** (`technical-design.md §1`: Identity, Abilities, Combat, Skills, Weapons, Armor, Inventory, Feats, ClassFeatures, Spells, Custom, Pets, Actions, Math, BioNotes) |
| F-24 | Layout files | **3** (pf1e, sf1e, cpr) |
| F-25 | Golden combinations in G-10 | **3 systems × 2 papers × 4 option sets (none, spellbook, math, all) = 24** PDFs per run |
