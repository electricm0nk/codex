# Stage F6 merge-readiness receipt (desktop polish batch)

Branch `sd36/epic-f6-desktop-polish` at `8f3b15a8d2` (F6a-F6e), against tranche/16 `0a234a035a`.
Check run 2026-09-27. Nothing in the tree was edited. The one app this check launched was stopped at
the end. Evidence is in `merge-readiness/`. Scratch paths inside `ui-smoke-full-results.json` point
at tmpfs; the three screenshots this receipt cites are copied beside it.

**Verdict: NOT merge-ready.** There are 3 blockers (B1-B3). Because of them, this receipt is left
uncommitted.

## Blockers

### B1. Bloodrager's Caster Level box disagrees with the engine at levels 1-3

- **What the desktop shows.** The Caster Level box prints **1** for a Bloodrager 1 (and 2 and 3 for
  Bloodrager 2 and 3). The source is `list_class_facts` (`f6a-class-facts-wire.json`:
  `class:bloodrager` level 1 `{status: caster, value: 1, source: advanced_class_guide:class:bloodrager#bonus4}`).
  It is folded by `classFactsModel.summarizeCasterLevel` (`merge-readiness/probe12.jsonl`).
- **What the engine says.** The headless receipt states `class_chassis.bloodrager.caster_level value=0`
  at levels 1 and 3: "the rule does not open until Bloodrager level 4 ... a correct absence (0)"
  (`merge-readiness/bloodrager-engine-caster-level.txt`).
- **What the source says.** The converted rule's own gate is `applies: ClassLevel(bloodrager) >= 4`.
  This comes from `acg_classes.lst:44`, `BONUS:CASTERLEVEL|Bloodrager|Caster_Level_Bloodrager|PRECLASS:1,Bloodrager=4`.
- **Mechanism.** `class_facts_sheet_rules::class_caster_level_in` (lines 145-166) collects every held
  `CasterLevel(Class)` rule and calls `evaluate`, which returns the value and never tests
  `rule.applies`. The rule is held at level 1 because it is a sibling of the class principal
  (`held_set`'s `add` pulls in siblings without gating them).
- **Population.** All 52 converted class `CasterLevel` rules were scanned. Among the roster classes,
  only two carry a class-level opening gate: bloodrager (`>= 4`) and ranger (`>= 4`). The ranger's value (level - 3) is below 1
  anyway, so only the bloodrager prints a wrong number. The alignment gates (druid, hunter, paladin,
  antipaladin) are ignored by the same code path. They do not change a number on the census posture.
- **Cross-check.** Desktop vs engine over 59 roster classes at levels 1 and 7 (118 rows): 37 agree,
  80 have no engine chassis line to compare against, and **1 contradicts** (bloodrager 1)
  (`merge-readiness/caster-level-desktop-vs-engine-59.txt`).
- **Fix.** Gate each value with `evaluate_applies(&rule.applies, ...)`. A closed gate means "no caster
  level yet". Then re-pin the wire and add a bloodrager 1/3/4 row to
  `every_roster_class_answer_is_counted`.

### B2. The ui-smoke row `sheet-tab-actions` is red (reproduced on 2 runs)

- **Failure.** `forbidden string present: 'failure'`, in the full run (`ui-smoke-full.log:56`) and again
  on an isolated rerun (`ui-smoke-rerun.log`). Screenshot: `sheet-tab-actions.FAILED.png`.
- **Mechanism.** The Smoke Test PC (Elf Wizard 1) wears the create path's fixed Chain Shirt. The
  Actions tab's Rules and features section prints the Chain Shirt line's StatBlock
  `Arcane spell failure: 20%` (`data/sheet_rules/core_rulebook/equipment/chain_shirt.json`, unchanged
  since SD-35). That is correct PF1 text, so this is a false positive of the harness's global
  `failure` forbid. F6c already gave `sheet-f6c-shaman-no-domain-line` the same
  `allowGlobalForbid: ["failure"]` exemption.
- **Not caused by F6.** The row was last green on the 2026-09-18 full run. No full run has happened
  since, and F6 did not touch this path.
- **Fix.** Add `allowGlobalForbid: ["failure"]` to the row, with that reason in its notes, then rerun it.

### B3. The ui-smoke row `intelligent-item-catalog-open-and-search` is red (reproduced on 2 runs), because the app serves 0 components

- **Failure.** `marker not found on screen: 'Speech'`. The screen reads "— 0 components", "All books (0)"
  (`intelligent-item-catalog-open-and-search.FAILED.png`).
- **Mechanism.** In `intelligent_item_catalog::load_record_rows`, the component names come from
  `data/corpus/<book>/equipment/equipmods/*.json`, `data.key`. `codex_repo_root()` resolves to the
  app's resource dir first. That dir carries the generated corpus bundle, and
  `scripts/gen-corpus-bundle.mjs` `transform('equipment')` writes every equipment file as `{}`: "only
  the file's on-disk PRESENCE matters". So the catalog gets 0 of 246 Intelligent Item records
  (core_rulebook 161 + mythic_adventures 85). This holds in the dev app, and in any packaged build
  with the same resource layout.
- **Not caused by F6.** The bundler landed on 2026-09-20 (`b1e3b2fa4a`). The last full ui-smoke green
  was 2026-09-18. F6 changed none of `intelligent_item_catalog.rs`, `authoring_workbench.rs`,
  `gen-corpus-bundle.mjs` or `tauri.conf.json`.
- **Fix.** Pick one mechanism: widen the bundle to the fields this catalog reads (with a residue
  audit), or read the components from the converted package or `_settled` bundle that already ships.

## Checks that passed

| Check | Command / evidence | Result (denominator) |
|---|---|---|
| Seed characters Computed | desktop `starter_seed_tests` (7): `a_fresh_install_seeds_both_characters_and_both_markers` saves only on `Computed`; `the_second_seed_is_a_level_5_wizard_with_fireball_prepared` asserts 0 claim-blocking | 2 of 2 seeds Computed |
| Elowen, Fireball prepared in a 3rd-level slot, real app, isolated root | full run, row `load-seed-wizard-fireball` green; `load-seed-wizard-fireball.png`: Spells tab "Fireball · CRB · Evocation · Wizard level 3", Known and **Prepared**; Level 3 slots 3 | green |
| Real store untouched | `~/.local/share/io.electricm0nk.codex` checked before, after the full run, and after the rerun: 1 character dir, 15,380 entries, sha256 of path+size+mtime `38446d8e…dbdda2` all three times (`real-before.txt`, `real-after.txt`, `real-after2.txt`); isolated roots removed | unchanged |
| Weapons / Caster Level / Skills vs engine, 15 classes | `probe12.ts` over the served wire (pinned to the live command by `class_facts_wire_for_every_roster_class_matches_the_committed_artifact`, passing): samurai, warrior, magus, kineticist, shaman, monk, unchained_rogue, cleric, sorcerer, psion, expert, commoner, bloodrager, ranger, paladin at L1 and L7 | Weapons 15/15 agree with the engine's proficiency reader and PF1 (Samurai/Warrior/Magus Martial ✓, Monk/Psion named only, Commoner one weapon of choice); Skills 15/15 = `class_skill_view` (Shaman Unknown by name, the F6a 9-class ACG remainder); Caster Level 14/15, **bloodrager wrong (B1)** |
| No hand-kept class table | `grep` over `apps/desktop/src`: `MARTIAL_WEAPON_CLASSES`, `CASTER_CLASSES`, `CLASS_SKILLS` survive only in comments | 0 rules tables; `CLASS_OPTIONS_FALLBACK` (31 rows) is used only when the roster command fails, with the visible notice, and its dice equal the engine's |
| No offered class with HP Unknown | `f4c-class-roster-wire.json` (pinned by `list_class_roster_wire_carries_hit_die_and_skill_ranks_for_every_census_class`, passing) | 0 of 59 `hitPointsDie` null; `hitDie == hitPointsDie` 59 of 59; equal to `list_class_facts.hitDie` 59 of 59 |
| Printed hit-die line vs computed HP | the roster die, the fold die and `print_hit_die_lines` all read `hit_die_source`; Monk 4 / Rogue 3 = 31 + 21 = 52 checked by hand | agree |
| Printed lines vs tranche/16, 12 builds | `class_census --sheet-dump <b> --with-sheet-rules`, release builds of both trees (`builds.txt`, `render12-t16-vs-head.diff`) | 12 of 12 Computed on both. 3 deltas, all by cited mechanism: shaman 5 `Shaman (domains) +1` and `Life (Spirit)` withheld (F6c §3, ACG p.35); monk 4 + rogue 3 `multiclass.hit_points 52` added and `class_chassis.hit_points.unknown` removed (F6b hit-die source, CRB p.56 d8). 0 uncited |
| No raw id in a printed requirement | `f4c-level-up-fighter6-wire.json`, every string under `addPrestige` | 0 requirement lines carry an id (the 6 id-shaped strings are the blocker `message`, shown only as a hover title) |
| Level Up: accepted then refused | `level_up_options_name_the_engines_mix_refusal_before_accept`: Fighter 6, 133 options each taken for real | computed 123; refused-with-blocker 10 (disabled, not acceptable); refused by anything else **0** |
| Census | `cargo run --locked -j 8 --release --bin class_census -- --json` (`census-mr.json`) vs `stage-f4-f5/census-f5.json` | ids 137, computed 63, prestige mix 68 of 74, mix panel 185 of 185, roster 59. Diff: 12 prestige message texts (F6c label rewrite) + `generated_at` only. `gen_class_status_table.py --check --json census-mr.json` OK |
| Desktop crate | `cargo test --locked -j 8 --manifest-path apps/desktop/src-tauri/Cargo.toml -- --test-threads=8` | 633 passed, 0 failed |
| ui-smoke full spec | `RUN_DESKTOP_AGENT=f6-mr node scripts/ui-smoke/run.mjs` | **77 of 83 green**, 2 red (B2, B3), 1 blocked (`campaign-manager-list`, a command-channel stall; green on rerun), 3 manual; 37 created / 37 deleted / 0 leftover |

## Polish (not merge-blocking)

1. **Ability scores print one low for odd scores** (`CharacterSheet.tsx:470 scoreFromModifier`). Elowen
   Con 13 prints 12; Aldric Str 19 prints 18. This is a wrong printed line, present on tranche/16 too
   (line 456 there), and is F6e remainder 1. Print the engine's effective score.
2. **Weapons tab "Also proficient with" lists PCGen pseudo-weapons** as weapons: `Flurry of Blows`
   (Monk), and `Spells (Ray)` / `Spells (Touch)` / `Splash Weapon` / `Unarmed Strike` / `Grapple`
   (every NPC class, samurai, magus and others). Only some classes list them. Filter the non-weapon
   entries by one rule.
3. **Expert class skills print the Path-A canonical picks** (the first 10 skills alphabetically) as
   plain class skills. CRB p.450 says "any ten, player's choice". This is the recorded F3c2 ruling,
   but the panel should name them as default picks.
4. **Magus class skills omit Knowledge (religion)**, following the oracle (`um_abilities_class.lst:68`).
   The Ultimate Magic text should be checked for a possible FS-23-style oracle defect.
5. **Spell text prints a raw formula**: Fireball reads "(min(10,CASTERLEVEL))d6" and "If requires
   Fireball from mythic spell (no record in the corpus)". This is pre-existing spell-catalog prose.
6. **Samurai class-skill list carries `samurai_mount`**, which is not a skill. The panel ignores it.
7. **Prestige requirement skill terms print lowercased slugs** ("knowledge nobility ranks at least 3")
   rather than skill labels.
8. **Elowen loads with 29 unallocated skill points** (only the create path's fixed Climb, Intimidate
   and Swim ranks are placed).
9. F6c remainders stand: the 4 Unchained base classes are refused in a mix (disabled, named), the 6
   FS-15 prestige classes are refused, and 33 catalog `Chosen` summaries still print choice ids.

## Blockers fixed (commit "fix(sd36,epic-f6): merge-readiness blockers")

The blocker list is `merge-readiness-blockers.json`. Evidence for the fixes is in `merge-readiness/fix/`.

| Blocker | Root cause fixed | RED → GREEN | Result |
|---|---|---|---|
| B1 Bloodrager Caster Level | `class_caster_level_in` now gates each held `CasterLevel` rule with `sheet_rule::sibling_line_gate`, the same line-gate decision `render_sheet` makes before it prints a `#` sibling (it is now a public function, and `render_sheet` calls it). A closed gate means NotACaster, "no caster level yet". A leaf over a fact the class query does not carry (paladin/druid/hunter/antipaladin alignment) is undecided, not failed. Raw `evaluate_applies` would have made those 4 classes non-casters (37 → 33 casters, seen and rejected). | `bloodrager_casts_nothing_before_four_then_at_class_level`: RED `fix/b1-red.log`, GREEN `fix/b1-green.log` (7 of 7) | Bloodrager 1 and 3 NotACaster; 4 → 4; 7 → 7. Roster counts unchanged (59 / 50 / 59, 37 casters). `every_roster_class_answer_is_counted` pins the first casting level for bloodrager, paladin, ranger and antipaladin (4) and for druid and hunter (1). Wire `f6a-class-facts-wire.json` re-pinned: 118 rows, 1 status change (bloodrager L1 caster 1 → notACaster). 3 other rows change only their reason text. Frontend `classFactsModel.test.ts` adds Bloodrager 1 `—` and 7 `7`. |
| B2 `sheet-tab-actions` | `allowGlobalForbid: ["failure"]`, with the Chain Shirt reason in the row's notes | real app, isolated root: `fix/ui-smoke-2rows.log` | PASS |
| B3 Intelligent Item catalog 0 components | `scripts/gen-corpus-bundle.mjs` bundles `equipment/equipmods/` records as `{data:{key,name,cost_gp}, source:{path,line}}`, exactly the fields `load_record_rows_in` reads. Other equipment files stay `{}`. The catalog now takes a corpus root (`build_catalog_in`). | `corpus_bundle_parity_test` now asserts that the catalog read off the bundle equals the one read off `data/corpus/`. RED `fix/b3-red.log`: raw 152 served, bundle 0. GREEN: equal. | Residue audit `fix/b3-equipmods-bundle-residue-audit.txt`: 1,494 of 1,494 equipmods records carry the raw fields and the sanitizer changed none; `pcgen_residue_gate.py --check --closure` gives live 0/0 PASS (70,045 shipped files scanned). Real app row PASS. |

Suites after the fix: desktop crate `cargo test` 633 passed, 0 failed (`fix/desktop-suite.log`). Root `cargo test --workspace`: every binary ok, including lib 2,745 passed and 6 ignored (`fix/root-suite.log`). Desktop `npm test`: 131 of 131 files. ui-smoke `--only sheet-tab-actions,intelligent-item-catalog-open-and-search`: 2 of 2 green, and the isolated root was removed (`fix/ui-smoke-2rows-results.json`, screenshots beside it). No full 76-row ui-smoke run was made after the fix.
