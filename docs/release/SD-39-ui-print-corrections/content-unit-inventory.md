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
| F-3 | Cards in this bundle | **51** | `awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/' docs/release/SD-39-ui-print-corrections/kanban.md \| awk 'END{print NR}'` |
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
| F-12 | Converter sites that drop `SLOTS` | **2** (`sheet_rule/convert.rs:1936`, `:2557`) | `grep -n '"SLOTS"' crates/codex-ingest/src/pcgen_import/sheet_rule/convert.rs \| awk 'END{print NR}'` |
| F-13 | Equipment records that will carry a `slot` | **(E1.0)** of **(E1.0)** equipment records | `python3 -c "import json,glob;fs=glob.glob('data/sheet_rules/*/equipment/*.json');print(sum(1 for f in fs if json.load(open(f)).get('slot') not in (None,'none')),'of',len(fs))"` |
| F-14 | PCGen equipment records with a `SLOTS:` token (the source) | **(E1.0)** | `grep -rl 'SLOTS:' "$PCGEN_CORPUS_ROOT" --include=*.lst \| xargs grep -h 'SLOTS:' \| awk 'END{print NR}'` (resolve `$PCGEN_CORPUS_ROOT` via `scripts/pcgen-oracle-pin.env`) |
| F-15 | Feats with `MULT:YES` in the pinned source (stackable) | **(E1.0)** | `grep -rh 'MULT:YES' "$PCGEN_CORPUS_ROOT" --include=*feats*.lst \| awk 'END{print NR}'` |
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
