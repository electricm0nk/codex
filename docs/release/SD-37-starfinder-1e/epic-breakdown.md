---
canonical: false
owner: operator
bundle_id: SD-37
date: 2026-10-02
status: planning — acceptance commands below are authored, not yet re-run against a tree that has the change (they cannot be: the change does not exist yet). Each card's first step re-runs its own command on the pre-change tree and records the RED output in its receipt.
---

# SD-37 Epic Breakdown

## §0 Parallel / serial map

This table leads the file because the Workflow script reads it (`workflow-instruction.md §2.4`,
`§3`). **Tier** is the `model` that each `agent()` call must set explicitly (`decisions.md §11`).
**Parallel?** = `yes` only where the file sets are verified disjoint (`workflow-instruction.md
§3`, §4). Every parallel lane that mutates files gets `isolation: 'worktree'` and its own
`CARGO_TARGET_DIR`. There are never more than 3 concurrent cargo lanes.

| Epic/Cycle | Parallel? | Tier | Notes |
|---|---|---|---|
| C0.0 package + unattended receipt | no | (this authoring session) | **complete**. This commit. |
| C0.1 SD-36 loose ends | yes (no cargo; docs/git only) | haiku | Not a gate. Touches only unowned trees (`decisions.md §2`, SD-j). |
| C0.2 package review | no | opus | Fable/Opus review of this package (`decisions.md §11`). Launch waits on it unless the operator waives it. |
| C1 version bump 0.17.0 | no | haiku | One commit, 14 surfaces (CUI F-17). Rebase first if #395 merged. Pushes `tranche/17` to origin. |
| E0.1 oracle sparse paths + fresh-clone proof | yes ∥ E1 | sonnet | `scripts/pcgen-oracle-pin.env`, `scripts/fetch-pcgen-oracle.sh`, `scripts/verify.sh` (preflight-oracle stage only). No cargo. |
| E0.2 licence matrix + SF PI term set | yes ∥ E1 | opus | `docs/governance/license-matrix.md` (SF rows), `docs/governance/ogl-pi-blacklist.md` (SF section), `artifacts/epic_0/`. No cargo. |
| E0.3 SF denominator | after E0.1 | sonnet | New SF inventory file (proposed `docs/work-inventory.starfinder-1e.json`) + sum check. Must fail closed. |
| E0.4 seed hand values from SRD | yes ∥ E1 | opus | `artifacts/epic_0/seed-hand-values.md`. Network needed (SD-c). |
| E1.1–E1.3 partition by game system | no (one batch, one agent) | opus | Shared files: `corpus_loader.rs`, `character_hub.rs`, `authoring_workbench.rs`, 19 BOOKS consts (CUI F-13), converter `closure.rs`/`reprint`/`sheet_rule_convert.rs`. Batch big: E1.1–E1.3 go in one dispatch. |
| E1.4 PF byte-identical gate | after E1.1–E1.3 | opus | Merge check. Renders Aldric + Elowen on both trees. Structural diff with 49,450 unmoved. |
| E1.MC merge check | after E1.4 | opus | Adversarial. Never downgraded. |
| E2.1 additive schema variants | no | opus | `src/rules_core/sheet_rule.rs` (exclusive owner while E2 runs). |
| E2.2 published `schemas/rules/*.schema.json` + check stage | after E2.1 | sonnet | Generated from serde. New `verify.sh` stage. |
| E2.MC | after E2.2 | opus | PF package deserialises unchanged. |
| E3.1 SF `.pcc` include structure (C2.1 adopted) | no | opus | `crates/codex-ingest/src/pcgen_import/{pcc.rs,include_resolver.rs}` + game-mode loader. |
| E3.2 formula-system reader (`MODIFY`/`MODIFYOTHER`/`CHANNEL`/`DATATABLE`) | after E3.1 | opus | `crates/codex-ingest/src/pcgen_import/sheet_rule/**` (formula, convert). 1,954 SF tokens (CUI F-9). |
| E3.3 SF mapping table + oracle rows + planted mutations | after E3.2 | opus | The overloaded-field hazard (`decisions.md §8`). |
| E3.4 Core Rulebook proof generation | after E3.3 | opus | Tune on one book (`decisions.md §4`). |
| E3.5 go wide: 7 books in one batch | after E3.4 | opus | Batch big: all seven books in one dispatch. |
| E3.MC | after E3.5 | opus | Structural diff, `--check`, residue gate, PI, refusals named. |
| E4a.1 data-package format, loader, schema, bundle path, licence/PI stamping | yes ∥ E2–E6 (after E1) | opus | Own worktree + `CARGO_TARGET_DIR`. New loader file; never `corpus_loader.rs`. |
| E4a.2 re-point all 252 importers | after E4a.1 | sonnet | Mechanical, fixed recipe. One dispatch for all 252 (batch big). Owns the 16 desktop importers until merged. |
| E4a.3 `.lst` citation burn-down | after E4a.2 | sonnet | 12,529 lines (CUI F-11). Re-derive SD-36 D6 status first. |
| E4a.4 PF parity + Rust table removal | after E4a.3 | opus | Byte-identical renders + catalogs. Bestiary 1 count before/after. |
| E4a.MC | after E4a.4 | opus | Adversarial. |
| E4.1 generic SF chassis (BAB, saves, HP, Stamina, Resolve, key ability) | yes ∥ E5 (after E3.MC) | opus | Proposed `src/rules_core/pilot_compute/sf_chassis.rs`. No per-class modules. |
| E4.2 EAC/KAC, initiative, skills, ACP | after E4.1 | opus | Proposed `sf_defense.rs`, `sf_skills.rs`. |
| E4.3 themes, point buy, ability increases | after E4.1 | opus | Engine half. Desktop half is E6.2. |
| E4.4 spellcasting 0–6 | after E4.1 | opus | Spells known/per day, DCs. |
| E4.5 credits, bulk, encumbrance | after E4.1 | opus | `src/rules_core/{money.rs,encumbrance.rs}` generalised per system. |
| E4.6 `StarfinderAdapter` + retire stub 0002 for SF | after E4.1–E4.5 | opus | `apps/desktop/src-tauri/src/rule_system_adapter.rs` + new adapter file. |
| E4.MC | after E4.6 | opus | Seeds vs SRD hand values. |
| E5.1 races, themes, class features print | yes ∥ E4 (after E3.MC) | opus | Print path only. No `pilot_compute` edits. |
| E5.2 feats, spells print | after E5.1 | opus | |
| E5.3 equipment, augmentations, upgrades, fusions print + total feeds | after E5.1 | opus | Feeds go to E4.5's totals. E5.3 never re-derives them. |
| E5.4 drone print | after E5.1 | opus | |
| E5.MC | after E5.4 | opus | |
| E6.1 system picker routes `starfinder-1e` to the real adapter | after E4.6, E5.MC | opus | `LandingScreen.tsx`, `characterHubRuntime`, Tauri resources. |
| E6.2 SF creation flow (race → theme → class → point buy) | after E6.1 | opus | |
| E6.3 SF sheet layout (SP/HP/RP, EAC/KAC; no CMB/CMD/touch) | after E6.1 | opus | Engine-single-source. No hand-kept tables. |
| E6.4 SF catalogs read `data/starfinder-1e/sheet_rules` | after E6.1 (and E4a.2 merged for shared files) | opus | |
| E6.5 SF level-up | after E6.2 | opus | |
| E6.6 seeds open in the real app (isolated `XDG_DATA_HOME`) | after E6.3–E6.5 | sonnet | Long run. Sonnet+ wait. |
| E6.MC | after E6.6 | opus | Renders real builds on both trees and opens the seeds. |
| E7.1 SF oracle parity roster + "not covered" list | after E6.MC | opus | PCGen SF game-mode runs. |
| E7.2 widest-scope verify (root + desktop) + baselines | after E7.1, E4a.MC | sonnet | One full pass. Long-run wait. |
| E7.3 final-acceptance scan | after E7.2 | opus | Every card `complete`. Any short = stop, no PR. |
| E7.4 retrospective written + cited | after E7.3 | sonnet | `docs/retro/sd37-retrospective.md`. |
| E7.5 worktree/branch sweep | after E7.4 | haiku | Never `test`/`update-index`. Never a locked tree. |
| E7.6 release notes | after E7.5 | sonnet | Figures re-derived. |
| E7.7 architecture truth-up (+ claims critic) | after E7.6 | sonnet (truth-up) + opus (critic) | |
| E7.8 graphify LAST | after E7.7 | sonnet | Clean tree, HEAD = `origin/tranche/17`. |
| E7.9 PR (final action) | after E7.8 | haiku | Operator merges. |

**E8 (starship)** is **not a card**. It is a planned capability deferral (`decisions.md §17`, FSR
DEF-1).

**Card count:** there are 55 cards (`kanban.md` row check). This map has 53 rows, because
E1.1–E1.3 are one dispatch. The criterion tables below carry all 55 IDs. `kanban.md` holds the
`diff … && echo SAME_IDS` command that proves the two sets are identical. Re-run it after any
edit.

### Cross-bundle section

| Relationship | Bundle | What crosses | Disposition |
|---|---|---|---|
| Predecessor | SD-36 (consolidation, PR #393 merged 2026-09-29) | crate wall (`crates/codex-ingest`), structural-diff protocol (`docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py`), census, seeds Aldric/Elowen, ui-smoke isolated root, lessons 14–20 | Reused. Lessons are rules R1–R7. |
| Predecessor | SD-36 FS-3 / D6 / ruling 7 | `rules_tables` → data package; `.lst` burn-down | Adopted as E4a (`decisions.md §19`). D6 status re-derived at E4a.3. |
| Predecessor | SD-35 C2.1 (second PCGen-format reader) | converter, parser, generators, oracle harness, pin | Adopted as E3 (`decisions.md §10`). |
| Predecessor | SD-35 §11 (converter kept for Starfinder) | tool side intact | Reused. No deletions. |
| Predecessor | SD-34 P1s (FS-2), SD-36 FS-15/27/28, Epic F next-week list | PF correctness | Candidates only (FSR). Not gates. |
| Successor (any) | starship sheet | no oracle data | FSR DEF-1, with a checked revisit condition. |
| Successor (any) | Traveller, Cyberpunk Red, etc. (SD-35 C2.2) | E1's system partition is the seam they reuse | Not in SD-37. |
| Shared infra | PR #395 (`fix/finalize-no-msi-glob`) | `publish-tester-release.yml` | Does not gate. C1 rebases if it lands first. |
| Template | `workflow-instruction-template.md §11` closure order | release notes after graphify | Fixed in this bundle's instance. Template fix is FSR-C9. |

---

## Sizing

The sizes below are **estimates**, unmeasured until each epic returns, and the receipt records the
real cost. The only measured reference point is SD-36 Epic F: ≈10.7 M subagent tokens for 10
batches (quoted). E3 and E4a are expected to be the largest. The quota stop rule
(`decisions.md §12.3`) binds whatever the estimate says.

---

## Epic C — Cycle 0 and Cycle 1

| ID | Criterion | Acceptance command (must be able to fail) |
|---|---|---|
| C0.0 | Package authored. Operator-away receipt in `progress.md`. | `ls docs/release/SD-37-starfinder-1e/{README,decisions,workflow-instruction,progress,kanban}.md`; `grep -c 'Cycle 0' docs/release/SD-37-starfinder-1e/progress.md` ≥ 1 |
| C0.1 | SD-36 loose ends dispositioned. Each item is either done or logged as owned by a live session. | Receipt lists `git worktree list` before/after; `git branch -r \| grep -c 'origin/sd36/'` before (7) and after; `git ls-remote --heads origin test update-index \| wc -l` = 2 after |
| C0.2 | Package review by Opus. Findings fixed in this package or escalated. | Receipt lists each finding with file:line and disposition; `grep -rn '<[a-z_-]*>' docs/release/SD-37-starfinder-1e/*.md` output is reviewed against `workflow-instruction.md §9` |
| C1 | `0.17.0` across exactly the 14 surfaces, in one commit. | `git show --name-only --format= <sha> \| awk 'NF' \| wc -l` → 14; `git show <sha> -- Cargo.toml \| wc -l` → 0; `npm test -- buildVersionTriple` in `apps/desktop` green; `git ls-remote --heads origin tranche/17` prints a SHA |

## Epic E0 — Oracle and licence admission

| ID | Criterion | Acceptance command | Does not cover |
|---|---|---|---|
| E0.1 | The pin's sparse paths include `data/starfinder` and `system/gameModes/Starfinder`. `fetch-pcgen-oracle.sh` fails when `data/starfinder/paizo/core` is missing. `verify.sh preflight-oracle` checks SF. | In a **fresh** directory outside the repo's clone: `scripts/fetch-pcgen-oracle.sh --dest <fresh> --quiet && test -f <fresh>/data/starfinder/paizo/core/_starfinder_core_rulebook.pcc && test -d <fresh>/system/gameModes/Starfinder`; then remove the SF cone and confirm the probe exits non-zero; `bash scripts/verify.sh --only preflight-oracle` PASS (`--only` is the script's documented single-stage selector, `scripts/verify.sh:44`) | CI runners (proved at E7.2 by the CI run) |
| E0.2 | `license-matrix.md` has one row per SF book (10), with licence, PI posture, `operator_sign_off`, and include/exclude. SF PI term set recorded. | `grep -c 'starfinder' docs/governance/license-matrix.md` ≥ 10; a test (proposed `tests/sf_license_registry.rs`) asserts the SF registry lists exactly the 8 in-scope books and that SSRGG/LPJ are absent | Legal correctness of Paizo's terms (operator signs) |
| E0.3 | SF work inventory: one unit per sheet-reachable record across the 8 in-scope books, summed by a fail-closed command. Excluded rows (214, CUI F-6) reported by name. | Sum-check command exits non-zero on any mismatch; receipt shows the unit count with two independent implementations | Records PCGen does not carry |
| E0.4 | Hand values for the 4 SF seeds (BAB, Fort/Ref/Will, HP, Stamina, Resolve, EAC, KAC, every skill total, spells known/per day for Mystic/Technomancer). Every value has a `source:` SRD URL + section. Opus-reviewed. | `awk -F'|' 'NR>2 && $0 !~ /https?:\/\//' artifacts/epic_0/seed-hand-values.md` prints no value rows | Any build outside the 4 seeds |

## Epic E1 — Game-system partition (no PF behaviour change)

| ID | Criterion | Acceptance command | Does not cover |
|---|---|---|---|
| E1.1 | A `GameSystem` id keys the package roots. `live_sheet_rules()` and the desktop `sheet_rule_package()` resolve the root **at runtime** per system (not only through the compile-time `CARGO_MANIFEST_DIR` baked at `corpus_loader.rs:360`). PF root = current paths (`decisions.md §7`). | `cargo test --locked -j 8 --lib game_system_root -- --test-threads=8` (new test: each id resolves to exactly one root; the PF root is unchanged; an unknown id is an error, not a fallback) | Packaged-app paths on tester machines (E6.1) |
| E1.2 | All 19 BOOKS consts (CUI F-13) are replaced by per-system registries, or wrapped by one. | `grep -rnE 'const [A-Z_]*BOOKS[A-Z_]*\s*:' src apps/desktop/src-tauri/src crates/codex-ingest/src --include='*.rs' \| awk 'END{print NR}'` → the receipt states the new count, and every remaining const is reached only through the registry (named per file) | — |
| E1.3 | The converter takes a system parameter: `closure.rs` `BOOKS_RELATIVE`, `reprint::VARIANT_LINE_BOOKS`, and the output dir. | `cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --check` exit 0 for PF (unchanged), plus the new `--system pathfinder-1e` spelling exit 0 | SF output (E3) |
| E1.4 | PF renders are **byte-identical** before/after for Aldric (Fighter 3) and Elowen (Wizard 5). Structural diff: 49,450 records unmoved, 0 rule deltas. Residue gate PASS. | `sha256sum` of each seed's rendered sheet on both trees (built per tree, sheet-rule path swapped at the baked location, memory `sheet-rule-package-path-is-baked-at-compile-time`), hashes equal; `structural_diff.py` `verdict=PASS`; `python3 scripts/pcgen_residue_gate.py --check --closure` exit 0 | Characters other than the 2 seeds (the structural diff covers the package; the hash covers the render of 2 builds) |
| E1.MC | Adversarial merge check | Opus agent re-runs E1.4 on the merged tree and plants one mutation (a PF book dropped from the registry), which must flip the hash | — |

## Epic E2 — Additive schema extension

| ID | Criterion | Acceptance command | Does not cover |
|---|---|---|---|
| E2.1 | `BonusTarget::{Eac,Kac,Stamina,Resolve}`, `Expr::KeyAbilityMod`, per-system `STACKING_TYPES`, an SF `SpellKind`, and `CharacterFacts` theme/key ability. All additive. | The PF package round-trips unchanged: `cargo test --locked -j 8 --lib sheet_rule -- --test-threads=8` plus a new test that deserialises every `data/sheet_rules/**.json` and re-serialises byte-equal | SF semantics (E3/E4) |
| E2.2 | `schemas/rules/sheet_rule.schema.json` generated from the serde types, with a `verify.sh` stage `rules-schema-check` that fails on drift | `bash scripts/verify.sh --list \| grep -c rules-schema-check` → 1; regenerating and diffing gives an empty diff; a hand-edit to the schema turns the stage red | Corpus record schema (E4a.1 publishes its own) |
| E2.MC | Adversarial merge check | Opus re-runs E2.1/E2.2 on the merged tree | — |

## Epic E3 — Starfinder converter (SD-35 C2.1 adopted)

| ID | Criterion | Acceptance command | Does not cover |
|---|---|---|---|
| E3.1 | SF `.pcc` include structure parsed: campaign-loaded `STAT:`/`SIZE:`/`SAVE:`/`ALIGNMENT:`/`VARIABLE:`/`DATATABLE:`/`DYNAMIC:`/`GLOBALMODIFIER:` files, plus `system/gameModes/Starfinder` (10 `.lst`, CUI F-18). | A test parses all 12 `.pcc` (CUI F-2) and resolves every include. The receipt lists the resolved file count = 131 `.lst` (CUI F-1) minus the excluded books' files, by name | Semantics of the files (E3.2) |
| E3.2 | Formula-system reader for `MODIFY`/`MODIFYOTHER`/`CHANNEL`/`DATATABLE`. Every one of the 1,954 SF `MODIFY*` tokens (CUI F-9) is mapped or named-refused. | `token_coverage.py` (SF mode) reports mapped + refused = 1,954, with refusals listed by name; two implementations agree on the token count | Correctness of each mapping (E3.3 oracle rows) |
| E3.3 | Separate SF mapping table. One row per overloaded field (`HP\|CURRENTMAX`+`HD`, `HP\|ALTHP`, `COMBAT\|AC` EAC/KAC, `FACT:KeyAbilityScore`, Resolve), each citing the SRD rule and an oracle observation. Planted mutations: swapping ALTHP↔CURRENTMAX, or dropping the `HD` term, must turn the seed fixtures red. | The mutation script's receipt shows each planted mutation → FAIL, and restored → PASS | Overloads not yet discovered: the receipt lists every SF field whose PF mapping was **reused** unchanged, so a reviewer can challenge each one |
| E3.4 | Core Rulebook proof: `data/starfinder-1e/sheet_rules` + corpus generated for `paizo/core`. Refused = 0 or every refusal named. PI-screened. `--check` exit 0. | `sheet_rule_convert --system starfinder-1e --check` exit 0; `_report.json` records = E0.3's CRB unit count; residue gate PASS | Other books |
| E3.5 | Go wide: Armory, COM, Pact Worlds, Near Space, AA1–3 in one batch. Same gates. | Same commands, all 8 books; `_report.json` records = E0.3's in-scope total | — |
| E3.MC | Adversarial merge check | Structural diff on the SF package (pinned delta classes; PF package unmoved, 49,450); 4 planted mutations FAIL; PI audit on SF records | — |

## Epic E4 — SF chassis compute (sheet totals only, no per-class modules)

| ID | Criterion | Acceptance command | Does not cover |
|---|---|---|---|
| E4.1 | Generic SF chassis reader over converted data: BAB, Fort/Ref/Will, HP (race + class/level), Stamina ((class SP + Con)/level, per §8 resolution), Resolve (SRD formula), key ability. **No `class_*.rs` for SF.** | Seed fixtures (E0.4) green for all 4 SF seeds; `ls src/rules_core/pilot_compute \| grep -ciE 'soldier\|mystic\|envoy\|technomancer\|operative\|mechanic\|solarian'` → 0 | Classes outside the seeds: E7.1's parity roster covers all 11 at level 1 |
| E4.2 | EAC/KAC (10 + armour bonus + Dex capped by max Dex), initiative, skill totals (ranks + ability + class-skill bonus + ACP). | Seed fixtures green, including every Envoy 3 skill total | Feats/abilities not in the seeds |
| E4.3 | Themes (+1 ability, theme class skill), SF point buy, ability increases at 5/10/15/20. | Seed fixtures green; a test for an ability increase at level 5 (Mystic 5, Technomancer 5) | Levels > 5 |
| E4.4 | Spells known / per day for levels 0–6 and save DCs, for Mystic and Technomancer. | Seed fixtures green (Mystic 5, Technomancer 5) | Witchwarper/Precog/other COM casters (parity roster level 1 only) |
| E4.5 | Credits and bulk totals, encumbrance thresholds. | Seed loadout fixtures green | — |
| E4.6 | `StarfinderAdapter: RuleSystemAdapter`. Stub registry entry 0002 retired for `starfinder`. | `grep -n 'starfinder' apps/desktop/src-tauri/src/stub_adapter.rs` → no SF routing; registry row updated; `cargo test --locked -j 8 -- --test-threads=8` in `apps/desktop/src-tauri` green | — |
| E4.MC | Adversarial merge check | Opus renders the 4 SF seeds and compares each total to E0.4. 0 mismatches. Planted mutation (Con dropped from Stamina) → red | — |

## Epic E4a — `rules_tables` → data package (parallel track)

| ID | Criterion | Acceptance command | Does not cover |
|---|---|---|---|
| E4a.1 | Data-package format (JSON per table, `schemas/rules/rules_tables.schema.json`), loader (new file, not `corpus_loader.rs`), bundle path, licence/PI stamping. | Loader round-trip test over every table; PI sweep passes on the package; `tauri-resources-tracked` PASS | — |
| E4a.2 | All 252 importers (CUI F-12) re-pointed in one dispatch. | `for r in src crates apps/desktop/src-tauri tests; do grep -rlE 'rules_tables::' $r --include='*.rs' \| awk '!/src\/rules_core\/rules_tables\//' \| awk 'END{print NR}'; done` → 0 0 0 0 (or the receipt names each remaining file and why), cross-checked by the Python walker | — |
| E4a.3 | `.lst` citation burn-down. First, SD-36 D6's status is re-derived. | Receipt shows the D6 status; the F-11 command on the package path → the receipt states the before (12,529) and after counts and the predicate; residue gate PASS | — |
| E4a.4 | PF parity: Aldric/Elowen byte-identical; catalog outputs byte-identical; Bestiary 1 monster count equal before/after; Rust tables removed. | Hash set equal; `find src/rules_core/rules_tables -name '*.rs' \| wc -l` → 0, or the receipt names each file kept and why | Catalog entries not rendered by the catalog dump |
| E4a.MC | Adversarial merge check | Opus re-runs E4a.4 on merged `tranche/17` | — |

## Epic E5 — SF print-path content

| ID | Criterion | Acceptance command | Does not cover |
|---|---|---|---|
| E5.1 | Races, themes, class features print with every resolvable term resolved. | For each seed, the sheet's printed feature list = the set of features the converted records grant at that level (test); 0 raw PCGen tokens (`pcgen_residue_gate.py --check --closure`) | — |
| E5.2 | Feats and spells print. Spell prose formulas as words. | Same residue gate; seed spell lists = E0.4's | — |
| E5.3 | Equipment, augmentations, upgrade slots, fusions print. Numeric feeds reach E4's totals. | Seed loadouts render; the EAC/KAC fixtures stay green with armour equipped | — |
| E5.4 | Drone prints for a Mechanic build. | A Mechanic 1 render prints the drone block (parity roster build) | Drone mods past level 1 |
| E5.MC | Adversarial merge check | Opus | — |

## Epic E6 — Desktop SF surfaces

| ID | Criterion | Acceptance command | Does not cover |
|---|---|---|---|
| E6.1 | Landing `starfinder-1e` routes to `StarfinderAdapter`. SF data root bundled. | Frontend test on routing; `tauri-resources-tracked` PASS with the SF root; app build on a clean checkout | — |
| E6.2 | Creation flow: race → theme → class → point buy, with the SF cost table. | Frontend tests + ui-smoke creation row green | — |
| E6.3 | SF sheet layout: SP/HP/RP, EAC/KAC; no CMB/CMD/Touch/Flat-Footed on SF sheets. **No hand-kept tables**: the engine is the single source. | A test greps the SF sheet component's render for `CMB\|CMD\|Touch\|Flat-Footed` → 0; a test that every SF number on the sheet comes from an engine explanation row | — |
| E6.4 | SF catalogs read `data/starfinder-1e/sheet_rules`. | Catalog test lists SF records; no SF catalog imports `rules_tables` | — |
| E6.5 | SF level-up | ui-smoke level-up row green on a seed | — |
| E6.6 | All 4 SF seeds and both PF seeds open in the real app under an isolated `XDG_DATA_HOME`. The real store is untouched. | ui-smoke receipt: 6 of 6 open; real store entry count and sha256 equal before/after | — |
| E6.MC | Adversarial merge check | Opus renders real builds on both trees and opens the seeds | — |

## Epic E7 — Verification and closure

| ID | Criterion | Acceptance command | Does not cover |
|---|---|---|---|
| E7.1 | SF oracle parity: PCGen SF game-mode runs for the 4 seeds + each of the 11 classes at level 1. The receipt carries an explicit **"what the oracle does not contain"** list. | Parity harness (SF mode) shows 0 unexplained mismatches; the not-covered list is present | Anything on the not-covered list |
| E7.2 | Widest-scope verify, run once: root workspace and `apps/desktop/src-tauri` `cargo test`; full `verify.sh`; `verify-baselines.env` re-derived; CI green on `tranche/17`. | `bash scripts/verify.sh` PASS (every stage); each `test result: FAILED` line attributed to its `Running` line (there must be none); `gh run list --branch tranche/17 --limit 3` success | — |
| E7.3 | Final-acceptance scan: every card `complete`. FSR revisit conditions checked (incl. DEF-1). | `awk -F'|' '$2 ~ /^ (C\|E)[0-9]/ && $5 !~ /complete/' kanban.md` prints nothing (kanban column 4 = Status, so awk field `$5`; E7.3 itself and E7.4–E7.9 are exempt only while they are the card running, and the scan names them). **If anything prints, stop: no retrospective, no sweep, no PR.** | — |
| E7.4 | Retrospective written from `retro.py summary`, cited from `references/README.md` | `grep -c 'sd37-retrospective' docs/release/SD-37-starfinder-1e/references/README.md` ≥ 1 | — |
| E7.5 | Worktree/branch sweep for this bundle (found vs removed counts) | Receipt; `git ls-remote --heads origin test update-index \| wc -l` = 2 | — |
| E7.6 | Release notes with re-derived figures | Each figure in `release-notes.md` has a command | — |
| E7.7 | Architecture truth-up + claims critic | Truth-up receipt in `receipts.md`; critic 0 blockers | — |
| E7.8 | Graphify LAST over the final tree | Receipt: `git status --porcelain \| wc -l` → 0, `git rev-parse HEAD` = `git rev-parse origin/tranche/17`, indexed SHA recorded; on the node-count guard (exit 1), file the receipt and stop. Never force. | — |
| E7.9 | PR `tranche/17 → develop` opened as the final action | `gh pr view --json state` OPEN. The operator merges. | — |
