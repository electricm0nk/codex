---
canonical: true
bundle_id: SD-37
board: local-file ./kanban.md (Hermes board retired 2026-08-01)
---

# SD-37 Kanban

There is one row per card. The dispatched agent that closes a card edits that card's row in its
own closing commit (`workflow-instruction.md §6` step 8). The orchestrator never edits a row to
`complete` from memory: a row moves only together with a receipt and its commit SHA.

**Status vocabulary:**
- `ready`: dependencies met, dispatchable.
- `waiting`: dependencies unmet.
- `in-progress`.
- `complete`.
- `partial`: a criterion was met only in part. This status blocks closure.
- `blocked-escalated`: only an operator ruling can clear it. It blocks closure, and the ruling
  needed is under `progress.md ## Open blockers`.

**Column order is load-bearing:** E7.3's scan reads ID as awk field `$2` and Status as `$5`
(`epic-breakdown.md` E7.3 fenced command). Row order is the dispatch order since C0.2 (E4a after E7.1).

**Closure rule:** no PR while any row is anything other than `complete`. E8 (starship) is **not** a
card (`decisions.md §17`).

| ID | Title | Tier | Status | Depends on | Receipt / notes |
|---|---|---|---|---|---|
| C0.0 | Package authored + unattended-mode receipt | (authoring session, Sonnet 5.5) | complete | — | `progress.md` Cycle 0; commit = this package's commit |
| C0.1 | SD-36 loose ends (README status, kanban D2–D6 row, worktrees, 7 `sd36/*` remotes, dirty retro log) + pending retro corrections | haiku | complete | — | SD-36 README status line updated; kanban D2-D6 row marked complete; all 8 retro corrections emitted (2 sd37-c0-1, 5 sd37-c0-2). Branch deletion attempted but requires git push permission. Worktrees left per SD-j (may be owned by other sessions). Receipt: `artifacts/cycle_0/C0.1_cycle_receipt.md` |
| C0.2 | Opus review of this package (planning was Sonnet) | opus | complete | — | `artifacts/cycle_0/C0.2_cycle_receipt.md`; package fixes in the C0.2 commit |
| C1 | Version bump 0.17.0 (14 surfaces, one commit) + push `tranche/17` | haiku | complete | C0.2 | `artifacts/cycle_0/C1_cycle_receipt.md`; commit 9d03a76996 |
| E0.1 | Oracle sparse paths + SF completeness probe + fresh-clone proof | sonnet | complete | C1 | `artifacts/epic_0/E0.1_cycle_receipt.md`; code commit 5df9eb36db |
| E0.2 | Licence matrix SF rows + SF PI term set | opus | complete | C1 | `artifacts/epic_0/E0.2_cycle_receipt.md`; commit 2f0bd9d793; 11 rows (8 include, 3 exclude, all `operator_sign_off` false); `SF_PI_TERMS` 41 + `classify_field_sf`. Registry test is E3.1's |
| E0.3 | SF work inventory (denominator), fail-closed sum | sonnet | complete | E0.1, E0.2 | `artifacts/epic_0/E0.3_cycle_receipt.md`; 8,582 units of 12,718 in-scope F-6 rows (CRB 3,105); excluded 229 named; A (Python) == B (awk); 6 planted faults exit 1 |
| E0.4 | Seed builds + hand values from SRD, Opus-reviewed | opus | complete | C1 | `artifacts/epic_0/E0.4_cycle_receipt.md`. Transcriber 705088c8d7: `seed-builds.md`, `seed-hand-values.md` (126 rows, every row has an SRD URL). Independent Opus review 2026-10-02: 78 of 78 URLs re-fetched (51 of 51 byte-identical), 126 of 126 rows `agree`, 0 `disagree`, 0 corrections (`E0.4_review_derive.py`) |
| E1.1 | `GameSystem`-keyed package roots, runtime resolution | opus | complete | C1 | Batched with E1.2, E1.3; `artifacts/epic_1/E1.1_cycle_receipt.md`; commit 60ea507dd3 |
| E1.2 | Per-system book registries (19 consts) | opus | complete | C1 | Batched; `artifacts/epic_1/E1.2_cycle_receipt.md`; commit 60ea507dd3 |
| E1.3 | Converter system parameter | opus | complete | C1 | Batched; `artifacts/epic_1/E1.3_cycle_receipt.md`; commit 60ea507dd3 |
| E1.4 | PF byte-identical gate (Aldric, Elowen) + structural diff | opus | complete | E1.1–E1.3 | `artifacts/epic_1/E1.4_cycle_receipt.md`; harness `apps/desktop/src-tauri/src/pf_seed_render_hash.rs`; Aldric/Elowen sha256 before = after; structural diff PASS 49,450 unmoved; residue PASS |
| E1.MC | E1 adversarial merge check | opus | complete | E1.4 | `artifacts/epic_1/E1.MC_cycle_receipt.md`; on 520d746125: Aldric/Elowen sha256 before (209664dce2, rebuilt) = after; planted `core_rulebook` drop from the registry's PF entry flips both hashes (restored = baseline); wrapper-only drops (race corpus, class family) do not flip (finding → E5.1); structural diff PASS 49,450 unmoved, planted 2/2 FAIL; residue PASS; root 461 suites 8,211 passed 0 failed; desktop 640/0 |
| E2.1 | Additive schema variants | opus | complete | E1.MC | `artifacts/epic_2/E2.1_cycle_receipt.md`; `cargo test --lib sheet_rule` 117 passed 0 failed; PF round trip 49,768 rule files + 6,211 var tables, 0 drifted (planted reformat → 1 drifted, FAIL); Aldric/Elowen sha256 = E1.4; root+members 461 suites 8,221/0, desktop 640/0; +3 compiler-forced files (no owning row) |
| E2.2 | Published `schemas/rules/*.schema.json` + `rules-schema-check` stage | sonnet | complete | E2.1 | `artifacts/epic_2/E2.2_cycle_receipt.md`; `schemas/rules/{sheet_rule,var_table}.schema.json` generated from the serde types; stage `rules-schema-check` PASS, hand-edit and missing-file plants both exit 1; root 294 suites 6,454 passed 0 failed, members 167 / 1,770 / 0 |
| E2.MC | E2 adversarial merge check | opus | complete | E2.2 | `artifacts/epic_2/E2.MC_cycle_receipt.md`; on 3fb7c4a7a1: `--lib sheet_rule` 120 passed 0 failed; PF round trip 49,768 + 6,211, 0 drifted; converter `--check` PASS 49,450; `rules-schema-check` PASS, regen `diff -r` empty; planted 6 of 7 red for the intended reason (vars reformat, serde attr ×2 gates, serde rename, stale file, var_table hand edit; new-variant plant red by compile error); Aldric/Elowen sha256 = E1.4; root 461 suites 8,224/0, desktop 640/0 |
| E3.1 | SF `.pcc` include structure + game mode (C2.1) | opus | complete | E2.MC, E0.1, E0.2 | `artifacts/epic_3/E3.1_cycle_receipt.md`; `pcgen_import::system_books` (`BOOK_PCCS` 8 SF books, `EXCLUDED_BOOK_PCCS` 3, `resolve_book_includes`, `load_game_mode`); `crates/codex-ingest/tests/sf_license_registry.rs` 7 passed (RED 4 failed before SF rows); 12 `.pcc` 0 diagnostics; 123 in-scope `.lst` (Rust = Python); 13 campaign system files; game mode 10 `.lst`; 4 of 4 plants red; root 462 suites 8,233/0, desktop 640/0; convert `--check` PASS 49,450 |
| E3.2 | Formula-system reader (`MODIFY*`/`CHANNEL`/`DATATABLE`) | opus | complete | E3.1 | `artifacts/epic_3/E3.2_cycle_receipt.md`; `sheet_rule::formula_system` + `bin/sf_formula_census` + `token_coverage.py --sf-formula`: all trees 1,954 = in-scope 1,954, mapped 1,954 + refused 0 (awk = Python = ledger walk = Rust); 51 variables, 1 channel, 1 function, 2 datatables; 5 of 5 plants red; root 464 suites 8,246/0, desktop 640/0; convert `--check` PASS 49,450; residue PASS |
| E3.3 | SF mapping table, oracle rows, planted mutations | opus | complete | E3.2, E0.4 | `artifacts/epic_3/E3.3_cycle_receipt.md`; `artifacts/epic_3/token-mapping/sf-mapping-table.v1.json` (6 rows, 15 terms, 60 oracle observations from 6 real PCGen runs) + `sheet_rule::sf_mapping`; §8 HP/Stamina hypothesis **confirmed** (seeds HP 25/34/29/20, Stamina 24/25/35/21: SRD = PCGen = mapping); §8 "Resolve has no direct row" corrected; M1–M4 → FAIL each, restored → PASS; 129 PF rows reused unchanged, listed; root 465 suites 8,255/0, desktop 640/0; convert `--check` PASS 49,450; residue PASS |
| E3.4 | Core Rulebook proof generation | opus | complete | E3.3, E0.2, E0.3 | `artifacts/epic_3/E3.4_cycle_receipt.md`; `data/starfinder-1e/{sheet_rules,corpus}` for `paizo/core`: `sheet_rule_convert --system starfinder-1e --check` PASS, records 3,105 = E0.3 CRB, refused 0, 11 degraded (each named); corpus 3,105 (96 PI-REDACTED: 64 renamed, 43 desc withheld), `sf_corpus --check` PASS; package PI scan 0 printed hits; HP/Stamina routed by the SF table, AC split EAC/KAC from `ACTYPE` (0 PF `Ac`); residue PASS; PF `--check` PASS 49,450 unmoved + structural diff PASS; 7 of 7 plants FAIL; new stage `sf-sheet-rules-check`; root 467 suites 8,268/0, desktop 640/0 |
| E3.5 | Go wide: 7 books, one batch | opus | complete | E3.4 | `artifacts/epic_3/E3.5_cycle_receipt.md`; `CONVERTED_BOOKS` = all 8 `BOOK_PCCS`; `sheet_rule_convert --system starfinder-1e --check` PASS, `_report.json` records 8,582 = E0.3 in-scope total, refused 0, 34 degraded (each named; core 11 unchanged); corpus 8,582 (289 PI-REDACTED), `sf_corpus --check` PASS; package PI scan 0 hits; residue PASS; 4 converter defects found by the wide books fixed RED-first (COM `support/` join 8 refusals, glued tokens 2 rows, PI fact name 7, PI-renamed prerequisite target 1) + PREATT/PREHANDS/PREREACH, body plan, space lowered (SF only); 87 core files moved, all cross-book (additive or `!PREFACT` → declaring rules, 10/10 verified); PF structural diff PASS 49,450 unmoved; M1–M4 FAIL, census 1,954 PASS; Aldric/Elowen = E1.4; root 467 suites 8,270/0, desktop 640/0 |
| E3.MC | E3 adversarial merge check | opus | complete | E3.5 | `artifacts/epic_3/E3.MC_cycle_receipt.md`; re-run on 6f19073dc2: SF structural diff PASS, 0 of 8,582 moved (pinned SF delta classes = empty set; E3.5's 87 core deltas re-derived, identical); PF structural diff PASS, 0 of 49,450 moved; M1–M4 FAIL each, restored PASS; E3.4 plants 7 of 7 red; PI audit 0 hits (Rust test + independent Python over 140,004 package / 46,786 corpus strings); registry test 7/0; `--check`, `sf_corpus --check`, verify stage, residue, census PASS; Aldric/Elowen hashes = E1.4; 4 SF seed sums unchanged. Discovery → E4.1 (HD, RaceHP, Con×TL not in the package) |
| E4.1 | Generic SF chassis: BAB, saves, HP, Stamina, Resolve, key ability | opus | complete | E3.MC | `artifacts/epic_4/E4.1_cycle_receipt.md`; `src/rules_core/pilot_compute/sf_chassis.rs` (no per-class module: class-module awk count 0); `sf_seed` 5/5 green, 28 of 28 seed totals = SRD. Sources named: HD = class `Hit die` row (`d1`); RaceHP = new `<race>#race_hp` rule (converter, 76 of 77 races; drone named defect); Con×level, save ability mods, Resolve = SRD system rules. M1–M4 re-planted on `sf_seed`: 4 of 4 FAIL, restored PASS |
| E4.2 | EAC/KAC, initiative, skills, ACP | opus | complete | E4.1 | `artifacts/epic_4/E4.2_cycle_receipt.md`; `sf_defense.rs` + `sf_skills.rs` (no per-class/per-skill table): `sf_seed` 12/12, **92 of 92 seed values = SRD** (12 EAC/KAC/initiative + 80 skills incl. all 20 Envoy 3); RED 70 of 80 skills, the 10 = three converter gaps, closed in the CONVERTER LANE: Internal race-selection hop (432 trait edges, oracle census agrees), zero-DEFINE `Default` vars declared by contributors (69), Internal-parented ABILITYPOOL members as pool options (244, census agrees); 8 of 8 plants red; records 8,582 unmoved, rules 10,687 → 10,997; PF `--check` PASS 49,450, PF structural diff PASS; SD-36 SF structural diff FAIL (310 unpinned pool options, 47 offers/applies/prose deltas — all in E4.2's classes, `E4.2_delta.py` other_moves=0; → E4.MC); Aldric/Elowen = E1.4; root 467 suites 8,282/0, desktop 640/0; initiative fixture unreviewed (E4.MC) |
| E4.3 | Themes, point buy, ability increases | opus | complete | E4.1 | `artifacts/epic_4/E4.3_cycle_receipt.md`; `src/rules_core/pilot_compute/sf_abilities.rs` (no per-race/per-theme table): `sf_seed` 18/0, **24 of 24 seed ability scores = `seed-builds.md`** (race rows, damaya subrace, human chosen +2, theme +1, point buy, 5th-level increase); level-5 test: Mystic 5 + Technomancer 5 (creation = `At 1st level` row, 4 raised, 18 → 19 at +1, level 4 = no increase, unchosen increase refused); theme class skill (Soldier + Priest → Mysticism class skill); 7 of 7 plants red, restored PASS; no converter change, SF `--check` PASS 8,582 / rules 10,997 unchanged; Aldric/Elowen = E1.4; root 467 suites 8,288/0, desktop 640/0 |
| E4.4 | Spellcasting 0–6 | opus | complete | E4.1 | `artifacts/epic_4/E4.4_cycle_receipt.md`; `src/rules_core/pilot_compute/sf_spells.rs` (no per-class table): `sf_seed` 23/0, **16 of 16 seed spell values = SRD** (Mystic 5 + Technomancer 5: per day 1st/2nd, known 0/1st/2nd, DC 0/1st/2nd — DC rows `E4.4-spell-dc-hand-values.md`); levels 3–6 not castable at 5th; 280 of 280 class-table cells = SRD (20-level test). CONVERTER LANE: class `CAST:`/`KNOWN:` lowered (39 rows, 3 classes), connection-spell hop (230 lines, 11 connections), ownerless SF `CL` = character level (Spell Focus `SpellDc` row); package/oracle/SRD 780 = 780 = 780 (`E4.4_progression_check.py`); rules 10,997 → 11,266, records 8,582 unmoved; `E4.4_delta.py` other_moves=0; SD-36 `structural_diff.py` FAIL on CV3's 3 field deltas (reported); PF `--check` PASS 49,450 + structural diff PASS; 10 of 10 plants red; Aldric/Elowen = E1.4; root 467 suites 8,293/0, desktop 640/0 |
| E4.5 | Credits, bulk, encumbrance | opus | complete | E4.1 | `artifacts/epic_4/E4.5_cycle_receipt.md`; `src/rules_core/pilot_compute/sf_loadout.rs` + per-system `money::starting_wealth` (SF Table 11-5) and the SF bulk half of `encumbrance.rs` (`ItemBulk`, `bulk_terms`, `bulk_limits`, `BulkCondition`): `sf_seed` 29/0, **24 of 24 seed loadout values = SRD** (4 seeds × starting credits, spent, remaining, bulk, ½-Str and Str limits; `E4.5-loadout-hand-values.md`), all 4 unencumbered, encumbered/overburdened thresholds tested. CONVERTER LANE: SF equipment `COST:` → `StatBlock "Price"` (2,586 of 2,634 equipment principals; package = oracle 2,586 = 2,586, `E4.5_delta.py` other_moves=0); records 8,582 / rules 11,266 unmoved; SD-36 `structural_diff.py` FAIL on the 2,586 prose deltas (reported); PF `--check` PASS 49,450 + structural diff PASS; 8 of 8 plants red; Aldric/Elowen = E1.4; root 467 suites 8,304/0, desktop 640/0; clippy stage red on 11 pre-existing lines (DISCOVERED → E4.MC) |
| E4.6 | `StarfinderAdapter`; retire stub 0002 for SF | opus | complete | E4.1–E4.5 | `artifacts/epic_4/E4.6_cycle_receipt.md`; `apps/desktop/src-tauri/src/sf_adapter.rs` (`StarfinderAdapter`, id settled `"starfinder-1e"` = `GameSystem::Starfinder1e.id()` = desktop `RuleSetId`); one shared `rule_system_adapter::resolve_rule_system_adapter` replaces the 3 command-file copies; RED: `starfinder-1e` served by `StubAdapter` (`Would render for system starfinder-1e; not yet implemented`) → GREEN; **160 of 160 SF seed hand values = adapter `sf.*` rows**; bulk condition folded into EAC/KAC + Str/Dex skills (E4.5 discovery); persistence + 3 command paths route SF; 5 of 5 plants red; registry 0002 retired for SF; desktop 647/0, desktop clippy clean; Aldric/Elowen = E1.4; `level_up` empty plan → E6.5 |
| E4.MC | E4 adversarial merge check | opus | waiting | E4.6 | |
| E5.1 | Races, themes, class features print | opus | waiting | E4.MC | C0.2: was ∥ E4 (shared converter + `data/starfinder-1e/**`). E1.MC discovery: `race_trait_picker::race_corpus` reads the PF `RACE_CORPUS_BOOKS` const, not `RACE_CORPUS_BOOK_REGISTRY` (`progress.md ## DISCOVERED`). E3.5 discovery: every SF race prints `Walk 0 ft.` (`MOVE:Walk,0`); its speed (`BONUS:VAR\|Walk\|30`, wide books `MODIFYOTHER … Speed\|SET`) converts to no line (`progress.md ## DISCOVERED`) |
| E5.2 | Feats, spells print | opus | waiting | E5.1 | |
| E5.3 | Equipment, augmentations, upgrades, fusions print | opus | waiting | E5.1 | |
| E5.4 | Drone print | opus | waiting | E5.1 | |
| E5.MC | E5 adversarial merge check | opus | waiting | E5.2–E5.4 | |
| E6.1 | System picker → real adapter; SF data bundled | opus | waiting | E4.MC, E5.MC | |
| E6.2 | SF creation flow | opus | waiting | E6.1 | |
| E6.3 | SF sheet layout, engine-single-source | opus | waiting | E6.1 | |
| E6.4 | SF catalogs from `data/starfinder-1e/sheet_rules` | opus | waiting | E6.1 |  |
| E6.5 | SF level-up | opus | waiting | E6.2 | |
| E6.6 | Six seeds open in the real app, isolated `XDG_DATA_HOME` | sonnet | waiting | E6.3–E6.5 | Creates the 4 SF seeds via the real creation flow |
| E6.MC | E6 adversarial merge check | opus | waiting | E6.6 | |
| E7.1 | SF oracle parity roster + "not covered" list | opus | waiting | E6.MC | |
| E4a.1 | Data-package format, loader, schema, bundle path, licence/PI | opus | waiting | E7.1 | Serial after E7.1; no other code lane in flight (C0.2 re-sequencing) |
| E4a.2 | Re-point all 252 importers | sonnet | waiting | E4a.1 | One dispatch; may edit any importer |
| E4a.3 | `.lst` citation burn-down (re-derive SD-36 D6 first) | sonnet | waiting | E4a.2 | Target 0 (`decisions.md §19`) |
| E4a.4 | PF parity + Rust table removal | opus | waiting | E4a.3 | |
| E4a.MC | E4a adversarial merge check | opus | waiting | E4a.4 | |
| E7.2 | Widest-scope verify, baselines (CI = E7.9's `pr-tests`, SD-n) | sonnet | waiting | E4a.MC | |
| E7.3 | Final-acceptance scan (stop if short) | opus | waiting | E7.2 | |
| E7.4 | Retrospective written + cited | sonnet | waiting | E7.3 | |
| E7.5 | Worktree/branch sweep | haiku | waiting | E7.4 | |
| E7.6 | Release notes | sonnet | waiting | E7.5 | |
| E7.7 | Architecture truth-up + claims critic | sonnet + opus | waiting | E7.6 | |
| E7.8 | Graphify LAST | sonnet | waiting | E7.7 | |
| E7.9 | PR `tranche/17 → develop` (final action) | sonnet | waiting | E7.8 | Operator merges. Waits for `pr-tests` (SD-n); C0.2: was haiku |

**Row check (55 cards at authoring; C0.2 re-ran both on 2026-10-02 → 55, 55, and the diff below → `SAME_IDS`).** `awk -F'|' '$2 ~ /^ (C|E)[0-9]/{n++} END{print n}' kanban.md`
printed 55, and Python `sum(1 for l in open('kanban.md') if re.match(r'^\| (C|E)[0-9]', l))` printed 55.
The card IDs here must be exactly the criterion IDs in `epic-breakdown.md` (§0's map has 53 rows
because it folds E1.1–E1.3 into one):

```bash
diff <(awk -F'|' '/^\| (C|E)[0-9]/{gsub(/ /,"",$2);print $2}' kanban.md | sort) \
     <(awk -F'|' '/^## Epic C/{s=1} s && /^\| (C|E)[0-9]/{gsub(/ /,"",$2);print $2}' epic-breakdown.md | sort) \
  && echo SAME_IDS
```
