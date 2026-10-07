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

> **Reading a command out of a table cell (C0.2, binding).** Inside a Markdown table cell, `\|` is
> the escape for `|`. Before running any command copied from a table cell in this package, replace
> every `\|` with `|`. Left in place, `\|` silently changes the command: in a regex it becomes a
> literal pipe (C0.2 measured E7.3's closure scan printing **0** rows over 54 open cards, and the
> E4.1 class-module grep printing 0 over a matching file); between commands it becomes an argument.
> Every command that gates closure is also given in a fenced block (E7.3 below,
> `workflow-instruction.md §11`), where no escaping applies. Paths in acceptance commands are
> repo-root-relative unless the command `cd`s first; a check whose input file is missing must
> exit non-zero, never print nothing.

| Epic/Cycle | Parallel? | Tier | Notes |
|---|---|---|---|
| C0.0 package + unattended receipt | no | (this authoring session) | **complete**. This commit. |
| C0.1 SD-36 loose ends | yes (no cargo; docs/git only) | haiku | Not a gate. Touches only unowned trees (`decisions.md §2`, SD-j). |
| C0.2 package review | no | opus | Fable/Opus review of this package (`decisions.md §11`). **complete** before launch (`artifacts/cycle_0/C0.2_cycle_receipt.md`); not re-dispatched. |
| C1 version bump 0.17.0 | after C0.2 (∥ C0.1) | haiku | One commit, 14 surfaces (CUI F-17). Rebase first if #395 merged. Pushes `tranche/17` to origin. |
| E0.1 oracle sparse paths + fresh-clone proof | yes ∥ E1 | sonnet | `scripts/pcgen-oracle-pin.env`, `scripts/fetch-pcgen-oracle.sh`, `scripts/verify.sh` (preflight-oracle stage only). No cargo. |
| E0.2 licence matrix + SF PI term set | yes ∥ E1 | opus | `docs/governance/license-matrix.md` (SF rows), `docs/governance/ogl-pi-blacklist.md` (SF section), `src/rules_core/pi_screening.rs` (SF term set), `artifacts/epic_0/`. **Uses cargo** (the term-set test), so it counts as one of the ≤ 3 cargo lanes (C0.2 correction: authoring said "No cargo"). |
| E0.3 SF denominator | after E0.1, E0.2 | sonnet | New SF inventory file (proposed `docs/work-inventory.starfinder-1e.json`) + sum check. Must fail closed. |
| E0.4 seed builds + hand values from SRD | yes ∥ E1 | opus | `artifacts/epic_0/seed-builds.md` first, then `artifacts/epic_0/seed-hand-values.md`. Network needed (SD-c). The step runs two agents: the transcriber, then an independent Opus reviewer that re-fetches every cited URL (both `opus`). |
| E1.1–E1.3 partition by game system | no (one batch, one agent) | opus | Shared files: `corpus_loader.rs`, `character_hub.rs`, `authoring_workbench.rs`, 19 BOOKS consts (CUI F-13), converter `closure.rs`/`reprint`/`sheet_rule_convert.rs`. Batch big: E1.1–E1.3 go in one dispatch. |
| E1.4 PF byte-identical gate | after E1.1–E1.3 | opus | Merge check. Renders Aldric + Elowen on both trees. Structural diff with 49,450 unmoved. |
| E1.MC merge check | after E1.4 | opus | Adversarial. Never downgraded. |
| E2.1 additive schema variants | no (after E1.MC) | opus | `src/rules_core/sheet_rule.rs` (exclusive owner while E2 runs). |
| E2.2 published `schemas/rules/*.schema.json` + check stage | after E2.1 | sonnet | Generated from serde. New `verify.sh` stage. |
| E2.MC | after E2.2 | opus | PF package deserialises unchanged. |
| E3.1 SF `.pcc` include structure (C2.1 adopted) | no (after E2.MC, E0.1, E0.2) | opus | `crates/codex-ingest/src/pcgen_import/{pcc.rs,include_resolver.rs}` + game-mode loader. Also writes the SF-registry licence test moved here from E0.2 (the registry only exists after E1.2). |
| E3.2 formula-system reader (`MODIFY`/`MODIFYOTHER`/`CHANNEL`/`DATATABLE`) | after E3.1 | opus | `crates/codex-ingest/src/pcgen_import/sheet_rule/**` (formula, convert). 1,954 SF tokens (CUI F-9). |
| E3.3 SF mapping table + oracle rows + planted mutations | after E3.2, E0.4 | opus | The overloaded-field hazard (`decisions.md §8`). |
| E3.4 Core Rulebook proof generation | after E3.3, E0.2, E0.3 | opus | Tune on one book (`decisions.md §4`). |
| E3.5 go wide: 7 books in one batch | after E3.4 | opus | Batch big: all seven books in one dispatch. |
| E3.MC | after E3.5 | opus | Structural diff, `--check`, residue gate, PI, refusals named. |
| E4.1 generic SF chassis (BAB, saves, HP, Stamina, Resolve, key ability) | no (after E3.MC) | opus | Proposed `src/rules_core/pilot_compute/sf_chassis.rs`. No per-class modules. |
| E4.2 EAC/KAC, initiative, skills, ACP | after E4.1 | opus | Proposed `sf_defense.rs`, `sf_skills.rs`. |
| E4.3 themes, point buy, ability increases | after E4.1 | opus | Engine half. Desktop half is E6.2. |
| E4.4 spellcasting 0–6 | after E4.1 | opus | Spells known/per day, DCs. |
| E4.5 credits, bulk, encumbrance | after E4.1 | opus | `src/rules_core/{money.rs,encumbrance.rs}` generalised per system. |
| E4.6 `StarfinderAdapter` + retire stub 0002 for SF | after E4.1–E4.5 | opus | `apps/desktop/src-tauri/src/rule_system_adapter.rs` + new adapter file. |
| E4.MC | after E4.6 | opus | Seeds vs SRD hand values; PF hash pair re-run (E4.5 generalises PF `money.rs`/`encumbrance.rs`). |
| E5.1 races, themes, class features print | after E4.MC (C0.2: was ∥ E4; they shared `crates/codex-ingest/**` and `data/starfinder-1e/**`, and E5 needs E4's render path and totals) | opus | Print path only. No `pilot_compute` edits. |
| E5.2 feats, spells print | after E5.1 | opus | |
| E5.3 equipment, augmentations, upgrades, fusions print + total feeds | after E5.1 | opus | Feeds go to E4.5's totals. E5.3 never re-derives them. |
| E5.4 drone print | after E5.1 | opus | |
| E5.MC | after E5.2–E5.4 | opus | |
| E6.1 system picker routes `starfinder-1e` to the real adapter | after E4.MC, E5.MC | opus | `LandingScreen.tsx`, `characterHubRuntime`, Tauri resources. |
| E6.2 SF creation flow (race → theme → class → point buy) | after E6.1 | opus | |
| E6.3 SF sheet layout (SP/HP/RP, EAC/KAC; no CMB/CMD/touch) | after E6.1 | opus | Engine-single-source. No hand-kept tables. |
| E6.4 SF catalogs read `data/starfinder-1e/sheet_rules` | after E6.1 | opus | E4a now runs after E7.1, so E6 owns the desktop catalog files outright. |
| E6.5 SF level-up | after E6.2 | opus | |
| E6.5a SF feats, spells known and gear chosen through the real app (DISCOVERED by E6.5, added 2026-10-06) | after E6.5 | opus | Every affordance writes through the engine; no hand-kept lists (R2). |
| E6.6 seeds open in the real app (isolated `XDG_DATA_HOME`) | after E6.3–E6.5a | sonnet | Long run. Sonnet+ wait. Creates the 4 SF seeds through the real creation flow from `artifacts/epic_0/seed-builds.md`. |
| E6.MC | after E6.6 | opus | Renders real builds on both trees and opens the seeds; PF hash pair re-run. |
| E7.1 SF oracle parity roster + "not covered" list | after E6.MC | opus | PCGen SF game-mode runs. |
| E4a.1 data-package format, loader, schema, bundle path, licence/PI stamping | after E7.1 (C0.2 re-sequencing; was ∥ E2–E6) | opus | Serial: no other code lane in flight. New loader file. |
| E4a.2 re-point all 252 importers | after E4a.1 | sonnet | Mechanical, fixed recipe. One dispatch for all 252 (batch big). May edit any importer, because nothing else runs. |
| E4a.3 `.lst` citation burn-down | after E4a.2 | sonnet | 12,529 lines (CUI F-11). Re-derive SD-36 D6 status first. Target 0 (`decisions.md §19`). |
| E4a.4 PF parity + Rust table removal | after E4a.3 | opus | Byte-identical renders + catalogs. Bestiary 1 count before/after. |
| E4a.4a Six table generators write the data package (DISCOVERED by E4a.4, added 2026-10-07) | after E4a.4 | opus | `gen_feat_gap_tables`, `gen_equipment_gap_tables`, `ingest_class_spell_levels_arg`, `ingest_spells`, `scripts/transcribe_{monster,companion}_tables.py`. Each re-run proves byte-identity against the pinned oracle. |
| E4a.MC | after E4a.4a | opus | Adversarial. |
| E7.2 widest-scope verify (root + desktop) + baselines | after E4a.MC | sonnet | One full pass. Long-run wait. |
| E7.3 final-acceptance scan | after E7.2 | opus | Every card `complete`. Any short = stop, no PR. |
| E7.4 retrospective written + cited | after E7.3 | sonnet | `docs/retro/sd37-retrospective.md`. |
| E7.5 worktree/branch sweep | after E7.4 | haiku | Never `test`/`update-index`. Never a locked tree. |
| E7.6 release notes | after E7.5 | sonnet | Figures re-derived. |
| E7.7 architecture truth-up (+ claims critic) | after E7.6 | sonnet (truth-up) + opus (critic) | |
| E7.8 graphify LAST | after E7.7 | sonnet | Clean tree, HEAD = `origin/tranche/17`. |
| E7.9 PR (final action) + wait for `pr-tests` | after E7.8 | sonnet (C0.2: was haiku; the `pr-tests` wait is long, R6) | Operator merges. |

**E8 (starship)** is **not a card**. It is a planned capability deferral (`decisions.md §17`, FSR
DEF-1).

**Card count:** there are 57 cards (`kanban.md` row check; 55 at authoring, E6.5a added 2026-10-06 and E4a.4a 2026-10-07 by discovery). This map has 55 rows, because
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
| E0.1 | The pin's sparse paths include `data/starfinder` and `system/gameModes/Starfinder`. `fetch-pcgen-oracle.sh` fails when `data/starfinder/paizo/core` is missing. `verify.sh preflight-oracle` checks SF. | In a **fresh** directory outside the repo's clone: `scripts/fetch-pcgen-oracle.sh --dest <fresh> --quiet && test -f <fresh>/data/starfinder/paizo/core/_starfinder_core_rulebook.pcc && test -d <fresh>/system/gameModes/Starfinder`; then remove the SF cone and confirm the probe exits non-zero; `bash scripts/verify.sh --only preflight-oracle` PASS (`--only` is the script's documented single-stage selector, `scripts/verify.sh:44`) | CI runners (no CI triggers on `tranche/17`; the closure PR's `pr-tests` run, awaited by E7.9, is the CI evidence — SD-n) |
| E0.2 | `license-matrix.md` has one row per SF PCC that carries data: the 10 book dirs **plus** `paizo/core/_society` (11), with licence, PI posture, `operator_sign_off`, and include/exclude. SF PI term set recorded in `ogl-pi-blacklist.md` and `pi_screening.rs`. (The SF-registry test moved to E3.1, because the registry is created by E1.2, which runs in parallel with E0.) | `for b in paizo/core paizo/core/_society paizo/armory paizo/character_operations_manual paizo/pact_worlds paizo/near_space paizo/alien_archive paizo/alien_archive_2 paizo/alien_archive_3 paizo/starfinder_society_rules lpj_design/infinite_space; do grep -qF "starfinder/$b" docs/governance/license-matrix.md \|\| echo "MISSING $b"; done` prints nothing (RED today: 11 lines); a `pi_screening` test classifies one SF PI term from the new set as PI | Legal correctness of Paizo's terms (operator signs) |
| E0.3 | SF work inventory: one unit per sheet-reachable record across the 8 in-scope books, summed by a fail-closed command. Excluded rows (229 = SSRGG 177 + LPJ 37 + core `_society` 15, CUI F-6) reported by name. | Sum-check command exits non-zero on any mismatch (a planted off-by-one in the inventory must make it exit non-zero, shown in the receipt); receipt shows the unit count with two independent implementations | Records PCGen does not carry |
| E0.4 | (1) **Seed builds** `artifacts/epic_0/seed-builds.md`: every field listed in `content-unit-inventory.md §4` for each of the 4 SF seeds, meeting the `decisions.md §9` constraints. (2) **Hand values** `artifacts/epic_0/seed-hand-values.md`, one table row per value, shaped `\| SF-<seed> \| <field> \| <value> \| <SRD URL + section> \|`, for BAB, Fort/Ref/Will, HP, Stamina, Resolve, EAC, KAC, every skill total, and spells known/per day for Mystic/Technomancer. (3) An independent Opus reviewer re-fetches every URL and records agree/disagree per row. | From the repo root: `f=docs/release/SD-37-starfinder-1e/artifacts/epic_0/seed-hand-values.md; test -s "$f" \|\| exit 1; awk -F'\|' '/^\| *SF-/ && $0 !~ /https?:\/\//' "$f"` prints nothing; `awk -F'\|' '/^\| *SF-/{n[$2]++} END{for(s in n) print s, n[s]}' "$f"` lists all 4 seeds, each with ≥ 9 rows; the reviewer's table has 0 `disagree` rows | Any build outside the 4 seeds; the SRD's own errata |

## Epic E1 — Game-system partition (no PF behaviour change)

| ID | Criterion | Acceptance command | Does not cover |
|---|---|---|---|
| E1.1 | A `GameSystem` id keys the package roots. `live_sheet_rules()` and the desktop `sheet_rule_package()` resolve the root **at runtime** per system (not only through the compile-time `CARGO_MANIFEST_DIR` baked at `corpus_loader.rs:360`). PF root = current paths (`decisions.md §7`). | `cargo test --locked -j 8 --lib game_system_root -- --test-threads=8` (new test: each id resolves to exactly one root; the PF root is unchanged; an unknown id is an error, not a fallback) | Packaged-app paths on tester machines (E6.1) |
| E1.2 | All 19 BOOKS consts (CUI F-13) are replaced by per-system registries, or wrapped by one. | `grep -rnE 'const [A-Z_]*BOOKS[A-Z_]*\s*:' src apps/desktop/src-tauri/src crates/codex-ingest/src --include='*.rs' \| awk 'END{print NR}'` → the receipt states the new count (19 before, CUI F-13); then `for f in $(grep -rlE 'const [A-Z_]*BOOKS[A-Z_]*\s*:' src apps/desktop/src-tauri/src crates/codex-ingest/src --include='*.rs'); do grep -q 'GameSystem' "$f" \|\| echo "UNREGISTERED $f"; done` prints nothing (RED today: every file prints). Two of the 19 are inside `rules_tables/` (`companion_chassis.rs`, `monster_chassis.rs`); E4a later moves them with the tables | — |
| E1.3 | The converter takes a system parameter: `closure.rs` `BOOKS_RELATIVE`, `reprint::VARIANT_LINE_BOOKS`, and the output dir. | `cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --check` exit 0 for PF (unchanged), plus the new `--system pathfinder-1e` spelling exit 0 | SF output (E3) |
| E1.4 | PF renders are **byte-identical** before/after for Aldric (Fighter 3) and Elowen (Wizard 5). Structural diff: 49,450 records unmoved, 0 rule deltas. Residue gate PASS. **E1.4 owns the render-hash harness:** it writes one test or bin (proposed `tests/pf_seed_render_hash.rs`) that renders a starter seed from `apps/desktop/src-tauri/src/character_hub.rs` through the same sheet-surface function the desktop command returns, and prints the sheet JSON; it names the entry point in its receipt. E4.MC, E6.MC, E4a.4 and E7.2 reuse exactly that harness (G-1). For the "before" tree, the harness file is copied into a worktree at the base SHA. | `sha256sum` of each seed's rendered sheet on both trees (built per tree, sheet-rule path swapped at the baked location, memory `sheet-rule-package-path-is-baked-at-compile-time`), hashes equal; `structural_diff.py` `verdict=PASS`; `python3 scripts/pcgen_residue_gate.py --check --closure` exit 0 | Characters other than the 2 seeds (the structural diff covers the package; the hash covers the render of 2 builds) |
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
| E3.1 | SF `.pcc` include structure parsed: campaign-loaded `STAT:`/`SIZE:`/`SAVE:`/`ALIGNMENT:`/`VARIABLE:`/`DATATABLE:`/`DYNAMIC:`/`GLOBALMODIFIER:` files, plus `system/gameModes/Starfinder` (10 `.lst`, CUI F-18). Also: a test (proposed `tests/sf_license_registry.rs`, moved here from E0.2) asserts the SF book registry lists exactly the 8 in-scope books and that SSRGG, LPJ and `paizo/core/_society` are absent and named in the exclusion record. | A test parses all 12 `.pcc` (CUI F-2) and resolves every include. The receipt lists the resolved `.lst` count = 131 (CUI F-1) minus the excluded PCCs' files (SSRGG 5, LPJ 2, core `_society` 1 → 123), by name; the registry test is RED before the registry gains SF rows | Semantics of the files (E3.2) |
| E3.2 | Formula-system reader for `MODIFY`/`MODIFYOTHER`/`CHANNEL`/`DATATABLE`. Every one of the 1,954 SF `MODIFY*` tokens (CUI F-9, all 10 PCC data trees; E3.2 also reports the in-scope subset) is mapped or named-refused. | `scripts/token_coverage.py` has no SF mode today (its flags are `--check/--inventory/--package/--table/--out`); **E3.2 adds it** and names the flag. It reports mapped + refused = the token count, with refusals listed by name; two implementations agree on the token count (CUI F-9's awk and Python) | Correctness of each mapping (E3.3 oracle rows) |
| E3.3 | Separate SF mapping table. One row per overloaded field (`HP\|CURRENTMAX` + `HD` + `RaceHP` → Hit Points; `HP\|ALTHP` + `CON*TL` + Toughness → Stamina — the hypothesis in `decisions.md §8`, which E3.3 confirms or overturns; `COMBAT\|AC` EAC/KAC; `FACT:KeyAbilityScore`, including the choice forms `Str or Dex` and `INT or WIS`; Resolve), each citing the SRD rule (E0.4) and an oracle observation. Planted mutations M1–M4 (`decisions.md §8`) must each turn at least one seed fixture red. | The mutation script's receipt shows M1, M2, M3, M4 → FAIL each, and restored → PASS | Overloads not yet discovered: the receipt lists every SF field whose PF mapping was **reused** unchanged, so a reviewer can challenge each one |
| E3.4 | Core Rulebook proof: `data/starfinder-1e/sheet_rules` + corpus generated for `paizo/core`. Refused = 0 or every refusal named. PI-screened. `--check` exit 0. | `sheet_rule_convert --system starfinder-1e --check` exit 0; `_report.json` records = E0.3's CRB unit count; residue gate PASS | Other books |
| E3.5 | Go wide: Armory, COM, Pact Worlds, Near Space, AA1–3 in one batch. Same gates. | Same commands, all 8 books; `_report.json` records = E0.3's in-scope total | — |
| E3.MC | Adversarial merge check | Structural diff on the SF package (`structural_diff.py <scratch> --baseline data/starfinder-1e/sheet_rules`; pinned delta classes) and on PF (`--baseline data/sheet_rules`, 49,450 unmoved); mutations M1–M4 FAIL; PI audit on SF records; E3.1's registry test green | — |

## Epic E4 — SF chassis compute (sheet totals only, no per-class modules)

| ID | Criterion | Acceptance command | Does not cover |
|---|---|---|---|
| E4.1 | Generic SF chassis reader over converted data: BAB, Fort/Ref/Will, HP (race HP + class HP × level, per E3.3's mapping), Stamina ((class SP + Con mod) × level, per E3.3's mapping), Resolve (SRD formula), key ability. **No `class_*.rs` for SF.** | Seed fixtures (E0.4) green for all 4 SF seeds (`cargo test --locked -j 8 --lib sf_seed -- --test-threads=8`, test created by E4.1); `ls src/rules_core/pilot_compute \| awk 'tolower($0) ~ /soldier\|mystic\|envoy\|technomancer\|operative\|mechanic\|solarian\|biohacker\|vanguard\|witchwarper/' \| awk 'END{print NR}'` → 0 (unescape `\|` first) | Classes outside the seeds: E7.1's parity roster covers the 10 player classes at level 1 (+ Drone via Mechanic 1) |
| E4.2 | EAC/KAC (10 + armour bonus + Dex capped by max Dex), initiative, skill totals (ranks + ability + class-skill bonus + ACP). | Seed fixtures green, including every Envoy 3 skill total | Feats/abilities not in the seeds |
| E4.3 | Themes (+1 ability, theme class skill), SF point buy, ability increases at 5/10/15/20. | Seed fixtures green; a test for an ability increase at level 5 (Mystic 5, Technomancer 5) | Levels > 5 |
| E4.4 | Spells known / per day for levels 0–6 and save DCs, for Mystic and Technomancer. | Seed fixtures green (Mystic 5, Technomancer 5) | Witchwarper and other non-seed casters (parity roster level 1 only); casters from books outside the pinned oracle |
| E4.5 | Credits and bulk totals, encumbrance thresholds. | Seed loadout fixtures green | — |
| E4.6 | `StarfinderAdapter: RuleSystemAdapter`. Stub registry entry 0002 retired for `starfinder`. | A new test asserts the adapter resolved for `starfinder-1e` (the desktop `RuleSetId`) is `StarfinderAdapter` and that its chassis call returns no `Would …` string — RED on the pre-change tree, where `StubAdapter` serves it (`stub_adapter.rs:189–242`; note the stub's id is `"starfinder"`, the desktop's is `'starfinder-1e'`: E4.6 settles the id and names it); registry entry 0002 updated; `cargo test --locked -j 8 -- --test-threads=8` in `apps/desktop/src-tauri` green. (C0.2: the authoring check `grep -n 'starfinder' stub_adapter.rs` could not fail cleanly — the file's own tests name `"starfinder"`.) | — |
| E4.MC | Adversarial merge check | Opus renders the 4 SF seeds and compares each total to E0.4. 0 mismatches. Planted mutation (Con dropped from Stamina) → red. PF hash pair (E1.4 harness) equal to E1.4's after-hashes, because E4.5 edits PF `money.rs`/`encumbrance.rs` | — |

## Epic E4a — `rules_tables` → data package (serial, after E7.1 — C0.2 re-sequencing)

E4a runs with no other code-writing lane in flight (`decisions.md §3`, §19). Its four cards may edit
any importer. A file that must stay behind is **not** a pass: it makes the card
`blocked-escalated` naming the file and the ruling needed (C0.2 removed the authoring "or the
receipt names each file kept and why" escape hatches from E4a.2/E4a.4, which let a card close with
the criterion unmet).

| ID | Criterion | Acceptance command | Does not cover |
|---|---|---|---|
| E4a.1 | Data-package format (JSON per table, `schemas/rules/rules_tables.schema.json`, generated by the same generator E2.2 built so `rules-schema-check` covers it), loader (new file), bundle path (`tauri.conf.json` resources), licence/PI stamping. | Loader round-trip test over every table; PI sweep passes on the package; `bash scripts/verify.sh --only tauri-resources-tracked --only rules-schema-check` PASS | — |
| E4a.2 | All 252 importers (CUI F-12) re-pointed in one dispatch. | `for r in src crates apps/desktop/src-tauri tests; do grep -rlE 'rules_tables::' $r --include='*.rs' \| awk '!/src\/rules_core\/rules_tables\//' \| awk 'END{print NR}'; done` → `0 0 0 0` (today `72 92 16 72`), cross-checked by a Python `os.walk` over the same roots skipping `target`/`node_modules` | — |
| E4a.3 | `.lst` citation burn-down. First, SD-36 D6's status is re-derived. Target 0 (`decisions.md §19`). | Receipt shows the D6 status; `test ! -e src/rules_core/rules_tables \|\| grep -rcF '.lst' src/rules_core/rules_tables \| awk -F: '{s+=$2} END{print s+0}'` → 0 or no output (12,529 before, CUI F-11) and the same predicate over every bundled data-package file → 0, each cross-checked in Python. Out of D6 scope and only reported: the 1,098 `.lst` lines elsewhere in `src/rules_core` (`grep -rcF '.lst' src/rules_core --exclude-dir=rules_tables \| awk -F: '{s+=$2} END{print s+0}'` → 1098 on 2026-10-02, Python agrees); residue gate PASS. (`pcgen_residue_gate.py` does not count `.lst` citations — it passes today — so it is not the burn-down check.) | — |
| E4a.4 | PF parity: Aldric/Elowen byte-identical; catalog outputs byte-identical; Bestiary 1 monster count equal before/after; Rust tables removed. | Hash set equal (E1.4 harness); `test ! -e src/rules_core/rules_tables \|\| find src/rules_core/rules_tables -name '*.rs' \| awk 'END{exit NR>0}'` exits 0 (no `.rs` left) | Catalog entries not rendered by the catalog dump |
| E4a.4a | The six generators that wrote compiled `.rs` into the removed `src/rules_core/rules_tables` now write `data/rules_tables/<id>.json`. A re-run of each against the pinned oracle leaves the package byte-identical. | `grep -rnE '"src/rules_core/rules_tables/|f"src/rules_core/rules_tables/' crates/codex-ingest/src/bin scripts --include='*.rs' --include='*.py' \| awk -F: '{print $1}' \| sort -u \| awk '!/scripts\/tests\//' \| awk 'END{print NR}'` → 0 (was 6); each generator re-run → `git status --porcelain data/rules_tables` empty; golden digest test green; PF hash pair equal to E1.4 | Generators outside the six |
| E4a.MC | Adversarial merge check | Opus re-runs E4a.2–E4a.4a's commands on `origin/tranche/17` and plants one mutation (one table JSON row edited) that must flip a PF hash or the round-trip test | — |

## Epic E5 — SF print-path content

| ID | Criterion | Acceptance command | Does not cover |
|---|---|---|---|
| E5.1 | Races, themes, class features print with every resolvable term resolved. | For each seed, the sheet's printed feature list = the set of features the converted records grant at that level (test); 0 raw PCGen tokens (`pcgen_residue_gate.py --check --closure`) | — |
| E5.2 | Feats and spells print. Spell prose formulas as words. | Same residue gate; seed spell lists = E0.4's | — |
| E5.3 | Equipment, augmentations, upgrade slots, fusions print. Numeric feeds reach E4's totals. | Seed loadouts render; the EAC/KAC fixtures stay green with armour equipped | — |
| E5.4 | Drone prints for a Mechanic build. | A Mechanic 1 render prints the drone block (parity roster build) | Drone mods past level 1 |
| E5.MC | Adversarial merge check | Opus re-runs E5.1–E5.4's commands on `origin/tranche/17`; `python3 scripts/pcgen_residue_gate.py --check --closure` exit 0; E4.MC's SF seed fixtures still green; plants one mutation (one granted feature dropped from a seed's converted records) that must fail E5.1's test. (C0.2: the authoring command cell read only "Opus", which cannot fail.) | — |

## Epic E6 — Desktop SF surfaces

| ID | Criterion | Acceptance command | Does not cover |
|---|---|---|---|
| E6.1 | Landing `starfinder-1e` routes to `StarfinderAdapter`. SF data root bundled. | Frontend test on routing; `tauri-resources-tracked` PASS with the SF root; app build on a clean checkout | — |
| E6.2 | Creation flow: race → theme → class → point buy, with the SF cost table. | Frontend tests + ui-smoke creation row green | — |
| E6.3 | SF sheet layout: SP/HP/RP, EAC/KAC; no CMB/CMD/Touch/Flat-Footed on SF sheets. **No hand-kept tables**: the engine is the single source. | A test greps the SF sheet component's render for `CMB\|CMD\|Touch\|Flat-Footed` → 0; a test that every SF number on the sheet comes from an engine explanation row | — |
| E6.4 | SF catalogs read `data/starfinder-1e/sheet_rules`. | Catalog test lists SF records; no SF catalog imports `rules_tables` | — |
| E6.5 | SF level-up | ui-smoke level-up row green on a seed | — |
| E6.5a | Feats, spells known and gear (armour, weapons, carried items, applied upgrades) can be chosen for a Starfinder character in the real app, at creation and on the sheet, from the engine's pools and catalogs. The level-up dialog's owed picks can be made. | Frontend tests on the three affordances; a ui-smoke row that builds one SF seed's full loadout from `artifacts/epic_0/seed-builds.md` and shows its EAC/KAC/credits equal to E0.4; `grep -rln 'appendToCharacter\|addFeatSelection\|addSpellSelection' apps/desktop/src --include=*.tsx` now lists a Starfinder surface | Seeds other than the one in the smoke row (E6.6 covers all 4) |
| E6.6 | The 4 SF seeds are created through the real creation flow from `artifacts/epic_0/seed-builds.md` (ui-smoke rows added to `spec.json`), then all 4 SF seeds and both PF seeds open in the real app under an isolated `XDG_DATA_HOME`. The real store is untouched. | ui-smoke receipt: 6 of 6 open; each SF seed's sheet totals equal E0.4's hand values; real store entry count and sha256 equal before/after | — |
| E6.MC | Adversarial merge check | Opus renders real builds on both trees and opens the seeds; PF hash pair (E1.4 harness) equal; E6.3's no-hand-kept-table test green | — |

## Epic E7 — Verification and closure

| ID | Criterion | Acceptance command | Does not cover |
|---|---|---|---|
| E7.1 | SF oracle parity: PCGen SF game-mode runs for the 4 seeds + each of the 10 player classes at level 1, plus the Drone through a Mechanic 1 build (CUI F-7: the 11th `CLASS:` is `Drone`, `TYPE:Monster`). The receipt carries an explicit **"what the oracle does not contain"** list. | Parity harness (SF mode) shows 0 unexplained mismatches; the not-covered list is present | Anything on the not-covered list |
| E7.2 | Widest-scope verify, run once: root workspace and `apps/desktop/src-tauri` `cargo test`; full `verify.sh`; `verify-baselines.env` re-derived; PF hash pair re-run. (No CI runs on `tranche/17` pushes — SD-n; CI evidence is E7.9's.) | `bash scripts/verify.sh -j 8` PASS (every stage); each `test result: FAILED` line attributed to its `Running` line (there must be none); root and desktop `cargo test` logs copied to `artifacts/epic_7/` | Windows/macOS packaged builds; CI (E7.9) |
| E7.3 | Final-acceptance scan: every card except the closure chain E7.3–E7.9 is `complete`. FSR revisit conditions checked (incl. DEF-1, the fenced command in `decisions.md §17`). | The fenced command below prints nothing. **If anything prints, stop: no retrospective, no sweep, no PR.** | — |
| E7.4 | Retrospective written from `retro.py summary`, cited from `references/README.md` | `test -s docs/retro/sd37-retrospective.md && grep -c 'retro/sd37-retrospective.md' docs/release/SD-37-starfinder-1e/references/README.md` ≥ 1 (C0.2: the authoring check matched the pre-written forward reference and passed before E7.4 ran; that reference was removed) | — |
| E7.5 | Worktree/branch sweep for this bundle (found vs removed counts) | Receipt; `git ls-remote --heads origin test update-index \| wc -l` = 2 | — |
| E7.6 | Release notes with re-derived figures | Each figure in `release-notes.md` has a command | — |
| E7.7 | Architecture truth-up + claims critic | Truth-up receipt in `receipts.md`; critic 0 blockers | — |
| E7.8 | Graphify LAST over the final tree | Receipt: `git status --porcelain \| wc -l` → 0, `git rev-parse HEAD` = `git rev-parse origin/tranche/17`, indexed SHA recorded; on the node-count guard (exit 1), file the receipt and stop. Never force. | — |
| E7.9 | PR `tranche/17 → develop` opened as the final action, then the PR's `pr-tests` run awaited inside the turn (SD-n). E7.9 commits its own `kanban.md`/`progress.md` rows (status `complete`) in one docs-only commit immediately **before** `gh pr create`; if the PR cannot be opened or `pr-tests` is red, it pushes one follow-up commit setting itself `blocked-escalated` with the failing job named. | Before `gh pr create`: the E7.3 scan below with `E7\.[3-9]` narrowed to `E7\.9` prints nothing. After: `gh pr view --json state` → OPEN; `gh pr checks <n> --watch` exit 0. The operator merges. | — |

**E7.3's closure scan** (run from the package directory; `$2` = ID, `$5` = Status, per
`kanban.md`'s column order):

```bash
cd docs/release/SD-37-starfinder-1e
test -s kanban.md || { echo NO_KANBAN; exit 2; }
awk -F'|' '$2 ~ /^ (C|E)[0-9]/ { n++ } END { if (n != 57) print "ROW_COUNT " n }' kanban.md
awk -F'|' '$2 ~ /^ (C|E)[0-9]/ && $2 !~ /^ E7\.[3-9] / && $5 !~ /^ complete *$/ { print $2 "|" $5 }' kanban.md
```

Pass = no output. C0.2 measured the authoring form (`(C\|E)` inside the regex) printing 0 rows
over 54 open cards on 2026-10-02 — it could never fail. The E7.3–E7.9 exemption is by ID, not by
"the card running", and the 57-row check (55 + E6.5a + E4a.4a) guards against a dropped row.
