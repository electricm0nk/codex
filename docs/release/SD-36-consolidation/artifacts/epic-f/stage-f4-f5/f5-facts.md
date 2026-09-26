# SD-36 Epic F5a fact sheet (measured 2026-09-26, tranche/16 HEAD e70a8745ed)

Every figure below: value, denominator, command. Nothing from prose or memory.

## Class census
Command: `cargo run --locked -j 8 --bin class_census -- --json <path>` (debug build, ~33 min wall, single-threaded);
artifact `docs/release/SD-36-consolidation/artifacts/epic-f/stage-f4-f5/census-f5.json` (generated_at 2026-09-26T21:55:17Z).
- ids = 137 (distinct class ids, all engine registries merged)
- non_prestige_swept = 63 of 137; computed = 63 of 63; blocked = 0 of 63
- prestige_swept = 74 of 137; prestige_alone_blocked = 74 of 74; prestige_mix_computed = 68 of 74; prestige_mix_unknown = 0
- the 6 Blocked prestige mixes: evangelist, exalted, mammoth_rider, pure_legion_enforcer, sentinel, ulfen_guard -- all `multiclass.save_shape.unrecognized` (FS-15)
- mix_panel_swept = 185; mix_panel_computed = 185 of 185; mix_panel_blocked = 0
- roster_offered = 59 (roster_reason: offered 59, ex_state 4 of 63 non-prestige: ex_antipaladin, ex_barbarian, ex_inquisitor, ex_paladin; 0 not_computed, 0 hit_die_absent)
- family partition (non-prestige 63): CRB 11, APG 6, ACG 10, Unchained 4, UC 3, untabled exotic 20, CRB NPC/Ex 7, generic-only 2
Epic F baseline (census 2026-09-20 @ 424e93e93c): 42 of 135.

## Generated table
`python3 scripts/gen_class_status_table.py --check --json <census-f5.json>` -> exit 0, "OK class-coverage table matches the census (ids=137 computed=63 prestige_swept=74 mix_panel_computed=185 of 185)".

## Desktop roster
Roster count 59 == census roster_offered=59 (F4.1 `cargo test --locked list_class_creation_roster`, desktop crate, f4c-green.log). Hardcoded picker before: 31 (`CLASS_OPTIONS`, now `CLASS_OPTIONS_FALLBACK`).
Tauri commands registered: 77 (python strip-comments count over `generate_handler![...]`, main.rs); 75 at 424e93e93c (+ list_class_creation_roster, list_level_up_class_options).

## Sheet-rule package (data/sheet_rules/_report.json)
- records 49,450; converted 49,450; refused 0 (unchanged since the cut 50572eebad)
- rules_written: 70,317 at cut (50572eebad) -> 71,862 at Epic F start (424e93e93c) -> 73,363 now
  (the brief's "71,869" is the pre-F1c figure, receipts.md:686, not the Epic F start)
- var_tables 5,309 (424e93e93c) -> 6,211; degraded_records 423 -> 424
- book directories: 37 (`ls -d data/sheet_rules/*/ | grep -v '/_' | wc -l`)

## _defects (len of each JSON list; before = `git show 424e93e93c:<path>`)
| file | Epic F start | now |
|---|---|---|
| unresolved-references | 11,925 | 6,252 |
| grant-by-type | 613 | 24 |
| undefined-variables | 739 | 89 |
| undeclared-in-pinned-tree (informational, new F3b2b) | absent | 776 |
| ambiguous-parent-category-target (new F1) | absent | 19 |
| skill-ranks-unresolved (new F3b2) | absent | 7 (eidolon, 5 bestiary creature-type classes, companion; 0 census classes) |
| subclass-token-unconverted (new F3c3) | absent | 117 |
| subclass-superseded-reprint (new F3c3) | absent | 7 |
| pool-member-unconverted (new) | absent | 124 |
| pool-option-unconverted / -id-collision / pool-pick-collision / row-declared-category-shared | absent | 2 / 1 / 1 / 1 |
| undeclared-contributed-variables | 58 | 60 |
| grants-to-unconverted-targets | 709 | 709 |
| others (choice-marker 127, double-percent 3, editorial 163, glued 5, inline-formula 110, literal-in-prose 55, spaced-percentile 1) | same | same |

Unresolved references by mechanism (`python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/unres2.py`, artifact stage-f4-f5/f5-unres2.txt):
| mech | Epic F start (11,925) | now (6,252) |
|---|---|---|
| A child category, target converted | 4,456 | 0 |
| B child category, target not a converted unit | 63 | 63 |
| C child category, target not under parent | 0 | 0 |
| D plain category, target converted, resolver misses | 3,033 | 2,646 |
| E plain category, target not ingested | 3,565 | 2,764 |
| F target found nowhere | 808 | 779 |
Sum now 63+2,646+2,764+779 = 6,252. Proficiency-related rows now: F 17, D 7.
Reader-remainder mechanisms (reader-remainder.md, `cargo test --locked -j 8 --lib every_census_class_has_a_known_proficiency_answer`): G 3 classes (G-U diabolist, G-O exalted, G-K rivethun_emissary; all prestige), H 0 (undefined-variables 89 remain package-wide, 648 moved to undeclared-in-pinned-tree), N 0 classes (FS-20 closed; 6 gate references to a helper remain MissingRule).

## Verify (last full run)
`bash scripts/verify.sh --full` at merge c2b7e03dd5, log artifacts/epic-f/stage-f4-f5/verify-f4-1.log: RESULT PASS, 51 stages passed, 0 failed.
- root-lib 2,727 passed; root-full 6,398 across 293 suites; ingest-full 1,764 across 166 suites; desktop 621; frontend 126/126 files; clippy root 0 / desktop 0 / ingest 0
- sheet-rules-check records=49450 converted=49450 refused=0 rules=73363 var_tables=6211
- class-census stage ids=137 computed=63 prestige_alone_blocked=74 mix_panel_computed=185 prestige_mix_computed=68
- stale-floor notices (baselines lower than measured, F5.3): ROOT_LIB 2587<2727, ROOT_FULL 6203<6398, ROOT_BIN 285<293, INGEST_FULL 1679<1764, INGEST_BIN 157<166, DESKTOP 612<621, FRONTEND_FILES 125<126

## verify-baselines.env (effective values, `set -a; source scripts/verify-baselines.env`)
CENSUS_IDS 135 (floor; measured 137), CENSUS_COMPUTED 63, CENSUS_MIX_COMPUTED 185, CENSUS_PRESTIGE_ALONE_BLOCKED 74, CENSUS_PRESTIGE_MIX_COMPUTED 68, COMPUTED_CLASSES 31 (v06_class_state_dump), CLIPPY 0/0/0, CORPUS_LITERAL_RECORDS 48706, DESKTOP 612, FRONTEND_FILES 125, INGEST 1679/157, ROOT 2587/6203/285.

## ui-smoke
- spec rows: 76 (`apps/desktop/scripts/ui-smoke/spec.json` .rows; 69 + 7 F4 rows); inventory header "76 rows across 18 screens (3 manual)"
- final/results.json (2026-09-18): 69 rows, 66 green, 3 manual
- f4/results.json (2026-09-26): 7 of 7 green; f4/regression/results.json 4 of 4 green (red runs run1 4/7, run2 5/7, run3 6/7 kept)

## Lines (release notes)
| figure | cut 50572eebad (release-notes) | Epic F start 424e93e93c | now e70a8745ed | command |
|---|---|---|---|---|
| src root | 465,469 | 337,790 | 349,508 | `find src -name '*.rs' \| xargs cat \| wc -l` |
| src codex-ingest | 0 | 81,185 | 85,906 | same over crates/codex-ingest/src |
| tests root | 182,070 | 132,264 | 134,255 | same over tests |
| tests codex-ingest | 0 | 37,163 | 39,323 | same over crates/codex-ingest/tests |
| pilot_compute/*.rs files | 1 | (42 per C1) | 45 | `ls src/rules_core/pilot_compute/*.rs \| wc -l` |
| largest pilot_compute file | 88,828 (mod.rs) | 6,160 | 6,049 (class_paladin_ranger.rs) | `ls -S ... \| head -1 \| xargs wc -l` |
| public status | 95.0% of 37,892 | 100% of 49,450 | 100% of 49,450 | site/status-data.json overall |

## Retro log
`python3 scripts/retro.py summary --since 2026-09-21`: 47 events (27 corrections, 12 verification, 6 notes, 1 deferral, 1 rework); 8 corrections had propagated when caught; 12 verify runs, 2 with a failing stage (token-coverage); 89 commits. Artifact stage-f4-f5/f5-retro-summary-since-2026-09-21.txt.
