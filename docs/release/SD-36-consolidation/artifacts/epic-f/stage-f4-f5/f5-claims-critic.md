# F5a claims critic (2026-09-26, tranche/16 HEAD a857be8959)

Scope: every capability/count claim in README.md, docs/architecture/*.md (the 8 files F5a touched, plus
headers of the rest), release-notes.md, epic-breakdown.md Epic F tables, forward-scope-register.md.
Fact sheet: `stage-f4-f5/f5-facts.md` (identical to the scratch facts.md). Figures below re-derived by
the critic unless marked "(from log)".

Verdict: **NOT merge-ready: 4 blockers.** This file stays uncommitted until they are fixed.

## Claims table

| # | Claim (where) | Verdict | Instrument figure / command |
|---|---|---|---|
| 1 | 137 ids; 63 of 63 non-prestige Computed; 74 of 74 prestige Blocked alone; 68 of 74 in carrier mix; mix panel 185 of 185; roster 59 (README:11, status.md Posture + table, rules-engine §3d, rules-data-tables, release-notes, epic-breakdown) | CONFIRMED | `census-f5.json`: ids 137, computed 63, non_prestige_swept 63, prestige_alone_blocked 74, prestige_mix_computed 68, mix_panel_computed 185 of 185, roster_offered 59 |
| 2 | 6 Blocked prestige mixes are all `multiclass.save_shape.unrecognized` (FS-15) | CONFIRMED | census-f5 prestige[].mixes: evangelist, exalted, mammoth_rider, pure_legion_enforcer, sentinel, ulfen_guard |
| 3 | Family partition 11/6/10/4/3/20/7/2 = 63; roster_reason offered 59 + ex_state 4 | CONFIRMED | Counter over census-f5 `classes[].family` / `roster_reason` |
| 4 | status.md generated table matches the census | CONFIRMED | `python3 scripts/gen_class_status_table.py --check --json .../census-f5.json` exit 0 |
| 5 | status.md retired-figure grep prints nothing | CONFIRMED (0 hits), but the grep is too narrow: it misses desktop-app.md:4 (see B1) | grep at status.md:81 |
| 6 | 77 Tauri commands, incl. `list_class_creation_roster`, `list_level_up_class_options` | CONFIRMED | comment-stripped split of `generate_handler![...]`, main.rs: 77 |
| 7 | 76 ui-smoke rows; final 66 green + 3 manual of 69; F4 7 of 7; regression 4 of 4; spec.json 79,319 bytes, run.mjs 41,698 bytes | CONFIRMED | spec.json rows 76; results.json counts; `wc -c` |
| 8 | Level-up offers every census prestige class, never blocked | CONFIRMED | `list_level_up_class_options_offers_prestige_with_printed_requirements` (character_hub.rs:10485) |
| 9 | `CLASS_OPTIONS_FALLBACK` (31 rows) is used only when the roster command fails, and the failure is printed | CONFIRMED | characterHubModel.ts:421; classRoster.ts:22; CreateCharacterForm.tsx:282 |
| 10 | 37 books; records 49,450, converted 49,450, refused 0; rules 73,363; var_tables 6,211; degraded 424 | CONFIRMED | `_report.json`; `ls -d data/sheet_rules/*/ \| grep -v '/_' \| wc -l` = 37 |
| 11 | unresolved 11,925 -> 6,252 (A 0, B 63, D 2,646, E 2,764, F 779) | CONFIRMED | `len(unresolved-references.json)` = 6,252; unres2.py |
| 12 | grant-by-type 24; undefined-variables 89; undeclared-in-pinned-tree 776; ambiguous-parent 19; skill-ranks-unresolved 7; subclass-token-unconverted 117; pool-member-unconverted 124 | CONFIRMED | len of each `_defects/*.json` |
| 13 | closure_complete 136 of 189 class principals | CONFIRMED | release-notes command, re-run: 136 189 |
| 14 | src/tests line counts 349,508 / 85,906 / 134,255 / 39,323 | CONFIRMED | `find <dir> -name '*.rs' \| xargs cat \| wc -l` |
| 15 | public status 100% of 49,450 (frozen) | CONFIRMED | site/status-data.json overall: done 49,450 of 49,450 |
| 16 | verify PASS 51/0; root-lib 2,727; root-full 6,398/293; ingest 1,764/166; desktop 621; census stage figures; stale-floor notices | CONFIRMED (from log) | `stage-f4-f5/verify-f4-1.log` |
| 17 | F4.3: 1 definition of `canonical_seeds_for`, 2 importing call sites | CONFIRMED | `git grep` counts: class_seeds.rs 1; pf1_adapter.rs:53, v06_class_state_dump.rs:60 |
| 18 | Cited converter modules and functions exist (`pool_link.rs::link_pool_choices`, `sheet_rule::link_path_a_picks`, `subclass.rs`, `natural_attack.rs`, `tests/common/mod.rs::assert_multiclass_status_parity`, verify.sh `class-census` stage) | CONFIRMED | file/grep |
| 19 | FS-23: "No sheet total reads [the Monk d10] today" | CONFIRMED | roster `hitPointsDie` for class:monk is null (f4c-receipt §2); `hit_die_from_package("monk")` = 10 but only `hitDie` (printed row) carries it |
| 20 | desktop-app.md:4 "the create-flow class picker offers all 31 classes the engine computes end to end, with a precisely-named 11-class UI-surface gap" | **FALSE** (B1) | roster offers 59 (census `roster_offered=59`; desktop-app.md:414 itself says 59) |
| 21 | README/status present the 59-class Create/level-up/sheet flow as a real end-to-end product; the named limitations are only Ex-*, prestige-alone, FS-15, multiclass joins | **FALSE by omission** (B2) | f4c-receipt §5 names open desktop remainders absent from every closure doc: HP Unknown on 5 of 59 roster classes (monk, unchained_barbarian, unchained_monk, unchained_rogue, unchained_summoner; `maxHitPoints` reads roster `hitPointsDie` = null); `skillsModel.CLASS_SKILLS` is a 12-row hand table, 47 of 59 roster classes have no class-skill list; `CASTER_CLASSES` (6 ids) and `MARTIAL_WEAPON_CLASSES` (5 ids) remain hand tables in characterProgression.ts:240/:62 |
| 22 | epic-breakdown.md:107 Epic F baseline evidence `artifacts/epic-f/docs-truth/class-census.md` | **BROKEN PATH** (B3) | not in the tree and in no commit (`git log --all -- <path>` empty); also cited at decisions.md:292, technical-design.md:186. The F0 baseline census is `artifacts/epic-f/census-f0.json` |
| 23 | release-notes.md §"Architecture-docs rewrite (D1)": "entirely uncommitted (HEAD is still b22ea9e113)", getting-started.md/glossary.md "untracked (`??`)", "none of it yet committed", "do not edit docs/architecture/** further from this bundle"; Known follow-ups #2 "(Uncommitted as of this docs pass ...)" | **FALSE** (B4) | `git ls-files` lists both docs and `tests/support/paths.rs`; committed in 4369b61a1d; F5a itself edited 8 docs/architecture files |
| 24 | desktop-app.md:420 `CLASS_OPTIONS_FALLBACK` at `characterHubModel.ts:409` | FALSE (line drift, polish) | declaration is at :421; :409 is inside its doc comment |
| 25 | FS-14 row body "203 of 7,119 rows"; FS-13 row "remaining ~793" | STALE in row body (polish) | the closure summary above the table states 204 of 6,252 and F 779 correctly |
| 26 | status.md:202 roster 59 cited to `census-merge-readiness.json` | CONFIRMED (values equal: 59/137/63/68), polish: cite census-f5.json like the rest | |
| 27 | Mermaid blocks parse | UNVERIFIABLE | F5a changed no Mermaid line (`git show HEAD` diff); no mermaid parser on this box; fence/diagram-type lint clean |
| 28 | release-notes.md carries the sections `check_release_manifest.py` requires (Summary, User-Visible Changes, ...) | FALSE, pre-existing, not currently bound by any manifest (polish before publish) | `REQUIRED_NOTES_SECTIONS`; no `## Summary` header |

## Blockers

- **B1** desktop-app.md:4: stale present-tense headline (31 classes, 11-class gap). Rewrite the
  earlier-pass sentence in past tense or drop the parenthetical; widen status.md's retired-figure grep
  with `all 3[1] classes|1[1]-class UI-surface gap`.
- **B2** open F4 desktop remainders (claim 21) missing from status.md "Known gaps and stubs /
  Desktop app", README "Known limitations", release-notes "Named remainder", and the forward-scope
  register. Add each with its denominator and mechanism (FS rows: HP from the engine sheet or FS-23;
  class skills from converted `CSKILL`; the two frontend class tables).
- **B3** repoint `artifacts/epic-f/docs-truth/class-census.md` (epic-breakdown.md:107, decisions.md:292,
  technical-design.md:186) to `artifacts/epic-f/census-f0.json`.
- **B4** release-notes D1 section and Known follow-up #2: mark as the C2-time record (past tense) or
  correct to "committed in 4369b61a1d".

## Resolution (round 1, 2026-09-26)

All four blockers fixed in "docs(sd36,epic-f5): claims-critic fixes round 1":

- **B1** desktop-app.md:4 earlier-pass parenthetical put in the past tense (the 31-row picker was
  replaced by the engine roster in F4). status.md's retired-figure grep gains
  `all 3[1] classes|1[1]-class UI-surface gap`; re-run over README.md + docs/architecture/*.md: 0 hits.
- **B2** the three F4c desktop remainders added, each with denominator and mechanism, to status.md
  "Known gaps and stubs > Desktop app" (3 rows), README "Known limitations", release-notes "Named
  remainder", desktop-app.md §Character flow, and forward-scope FS-24 (HP Unknown 5 of 59), FS-25
  (class skills 47 of 59; 12-key `CLASS_SKILLS` re-derived vs census-f5 `roster_reason=offered` 59),
  FS-26 (`CASTER_CLASSES` 6 ids, `MARTIAL_WEAPON_CLASSES` 5 ids, all 11 ids in the roster).
- **B3** epic-breakdown.md, decisions.md, technical-design.md (and progress.md row 11) repointed to
  `artifacts/epic-f/census-f0.json` (computed 42, ids 135, generated 2026-09-21, committed 258301b9bf);
  the prose now says the 2026-09-20 `424e93e93c` measurement's own report was never committed.
- **B4** release-notes D1 section rewritten as landed (`4369b61a1d`, 15 files, +4,239/-2,576 per
  `git show --stat`), with the C2-time record quoted in the past tense; follow-up #2 now names the
  landing commits. Correction to the critic's evidence: `tests/support/paths.rs` was added in
  `45ef7e2327`, not `4369b61a1d` (`git log --diff-filter=A`).
- Polish also applied: #24 (`CLASS_OPTIONS_FALLBACK` at `characterHubModel.ts:421`); the C2-baseline
  table header no longer says "current HEAD ... uncommitted".
- `python3 scripts/gen_class_status_table.py --check --json .../census-f5.json` exit 0 after the edits.
