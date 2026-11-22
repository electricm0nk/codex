# Brief 1 — cycle receipt (create-character fixes, F1–F7 + F3b)

Branch `ui-brief1` (worktree `.worktrees/brief1`), base `tranche-17-ui` @ 5f020b221b, merged with
`tranche-17-ui` twice (8c4fad3170, ddd46db0d8). Not pushed.

**Status: complete**, except the items under "Uncovered" and "Not done". F1 was reproduced and fixed.
F2–F7 and F3b are done and verified.

## Commits (`git log --oneline 5f020b221b..ui-brief1 --no-merges`, Brief 1 code commits only)

| sha | subject |
|---|---|
| e6f67b45c7 | fix(brief1-F4): read class_chassis spells-per-day rows |
| 04d87eaa22 | feat(brief1-F3/F5): wizard spellbook size, draft known-spell rows, drawbacks + trait count check |
| 21a41f3bc7 | feat(brief1-F6): one Options/Selected pair per spell level, shared pools |
| 253031fb4e | feat(brief1-F2/F3/F5): known-spell quotas, drawback trait limits, racial trait counts |
| 2d935014b7 | test(brief1-F1): forbid stray 'H2' on the Create screen smoke row |
| ca4b4c3017 | docs(brief1): cycle receipt + retro events (first pass) |
| bacfd9c43a | feat(brief1-F3): drawbacks in the trait picker, Meticulous -2 on untrained skill totals |
| 7a7228e611 | feat(brief1-F2): ManageBox manageLabel; 'Swap racial traits' |
| 5bf3dbb420 | docs(brief1): receipt second pass |
| 1dcaa4a84d | test(brief1-F1): run.mjs headingExact check |
| 15d3bd5acd | feat(brief1-F7): Hunter, Skald, Investigator known quotas (+ formula-book UI) |
| 6d9cdc8194 | fix(brief1-F1): switch off community-theme heading markers |
| 3085ec9c4e | feat(brief1-F3b): sheet skill rows add the engine's trait/drawback modifier |
| 01cc6a1085 | feat(brief1-F7): Witch, Arcanist, Alchemist book-size quotas |

## Scope corrections

- The brief named `src/rules_core/class_sorcerer_wizard.rs` / `class_wizard_prepared_spellbook.rs`;
  they live in `src/rules_core/pilot_compute/`. Lead confirmed.
- Granted 2026-10-11 (brief commits 3581fd8c74, b57ddf49f4, c23ec36b74, f348bf707b, 02d7e08a1c,
  fe1980d2de, be6011ab8d): `trait_picker.rs`, `skill_allocation.rs` (F3); `ManageBox.tsx` (F2;
  no ui-smoke row clicks the Create screen's Manage buttons: `grep -n '"Manage"'
  apps/desktop/scripts/ui-smoke/spec.json` → no match); `run.mjs` (F1); `CharacterSheet.tsx`
  skill rows, `skillsModel.ts`, `pf1_adapter.rs` (F3b; `pf1_adapter.rs` turned out not to need a
  change: the mutation response carries no explanations and the sheet re-reads the detail);
  `communityTheme.ts` (+ new `communityTheme.test.ts`), `uiProbe.ts` (F1); the four F7 class files
  and `class_occult_and_psionic.rs` (one Alchemist call).
- `tests/sd13_wizard_level1_prepared_spell_baseline.rs` and `uiSmokeProbe.test.ts` were edited as
  the brief's "+ tests".

## F1 — "H2" before the title (fixed)

- Cause: the app's only built-in community theme, Ayu Mirage (`BUILTIN_FEATURED_THEMES`,
  `apps/desktop/src/settings/communityTheme.ts`), loads github
  `bcdavasconcelos/Obsidian-Ayu_Mirage` `master/obsidian.css`, whose lines ~1759–1792 carry
  unscoped `h1:before { content: "H1 " }` … `h6:before`, Obsidian note-heading markers.
- Reproduced (before the fix): Settings → Appearance → Ayu Mirage, then the Create screen shows
  "H2 Create a character". Screenshot `/home/ubuntu/cargo-target/brief1/f1b/create-ayu-mirage.png`.
  Probe at the same moment: `headings ['Create a character']` (generated text is invisible to
  textContent/innerText, so `headingExact` and `forbid: H2` both pass with the marker on screen).
- Fix: the bridge stylesheet injected with every community theme sets `content: none !important`
  on `h1::before`…`h6::before` under `body.theme-dark` / `body.theme-light`.
- Guard: `uiProbe.ts` reports `headingMarkers` (computed `::before` content of h1–h3, via the
  tested `headingMarkerText`); `run.mjs` `noHeadingMarkers: true` fails a row that draws any;
  `create-character-render` uses it plus `headingExact: "Create a character"` and `forbid: ["H2"]`.
- Confirmed after the fix, theme on: probe `headings ['Create a character'] markers []`; screenshot
  `/home/ubuntu/cargo-target/brief1/f1c/create-ayu-mirage-fixed.png` shows the plain title.
- The operator's built (release) app was not checked; the fix is in shipped CSS, not dev-only code.

## F3b — sheet skill rows (twin fix)

- Ruling A asked the rows to show engine totals. The engine total (`allocate_skill_ranks`) has no
  feat fold and grounds class skills for Fighter/Wizard/Rogue only, so printing it verbatim would
  drop feat bonuses and class-skill bonuses the panel prints today. Smallest correct change: the
  engine serves the part the panel lacked, each skill's trait/drawback modifier
  (`skill_allocation::sheet_skill_trait_modifiers`), as `skill.trait_modifier.<skill>` rows on the
  sheet's explanations; the panel adds it to each row (`traitSkillModifierFor`).
- Live (Dwarf Fighter 1, Acrobat + Meticulous, Dex 14, Int 10, Cha 6): Acrobatics +1 (+2 Dex +1
  Acrobat −2), Appraise −2, Bluff −4, Perception +0, Climb +5 (1 rank, no −2). Before this change
  the rows had neither the +1 nor the −2.

## F7 — known quotas for six classes

| class | rows | source |
|---|---|---|
| Hunter | `class_chassis.hunter.spontaneous.spells_known.spell_level_N` = KNOWN, minus 1 at levels 1+ | pinned PCGen oracle `acg_classes.lst:144-163`; `:143` "The KNOWN are 1 higher than in the book for levels 1+ because all the Summon Nature's Ally spells are automatically known in addition" |
| Skald | `class_chassis.skald.spontaneous.spells_known.spell_level_N` | `acg_classes.lst:301-320` (new `skald_spells_known_table`) |
| Investigator | `class_spell.acg.investigator.formula_book_size.extract_level_K_and_higher` | `data/corpus/advanced_class_guide/class_feature/investigator/alchemy.json` |
| Alchemist | `class_spell.apg.alchemist.formula_book_size.*` | `data/corpus/advanced_players_guide/class_feature/alchemist/formulae.json` |
| Witch | `class_spell.apg.witch.spellbook_size.*` | wizard rule, flagged (see Uncovered) |
| Arcanist | `class_spell.acg.arcanist.spellbook_size.*` | wizard rule, flagged (see Uncovered) |

Formula book: `max(0, 2 + Int) + (level − 1)`, nested by creatable extract level. Spellbook
(wizard/witch/arcanist): every cantrip, `max(0, 3 + Int) + 2 × (level − 1)`, nested by castable
level. Hunter and Skald no longer show the "engine does not state how many spells … knows" line.

Oddities transcribed as the oracle has them: Hunter level 1 `KNOWN:4,3` (`acg_classes.lst:144`;
quota 4 orisons, 2 first-level after the SNA adjustment); Skald level 19 `KNOWN:6,6,5,6,5,5,4`
(`acg_classes.lst:319`; the 2nd-level 5 is below level 18's 6).

## Tests added (all passes)

- Rust engine: `wizard_spellbook_size_tests` (5), `trait_effects::drawback_tests` (5),
  `skill_allocation::meticulous_drawback_tests` (2), `sheet_skill_trait_modifier_tests` (3),
  `formula_book_size_tests` (4), `hunter_spells_known_quota_tests` (2),
  `skald_spells_known_quota_tests` (2), `witch_arcanist_spellbook_size_tests` (2).
- Rust desktop: sorcerer known, wizard spellbook, trait count, picker drawbacks,
  `the_draft_spell_options_state_known_quotas_for_hunter_skald_and_investigator`,
  `the_saved_sheet_carries_each_skills_trait_and_drawback_modifier`.
- TS: `spellsPerDayModel`, `spellDialogModel` (sorcerer, wizard, cleric, witch gap → now picker,
  hunter/skald/alchemist/investigator, witch/arcanist, reload key), `transferListModel`,
  `TransferListBody`, `manageItems`, `creationRacialTraitsPreview`, `ManageBox`, `skillsModel`
  (`verifiesTheTraitModifierRowIsAddedToTheSkill`), `communityTheme.test.ts`, `uiSmokeProbe`
  (`headingMarkerText`).

## Red evidence

- Pass 1/2: see logs `red-desktop.log`, `red-f3-root.log`, `red-f3-picker.log`, `root-mutation.log`
  (wizard spellbook proved by mutation: `2 passed; 3 failed`).
- Pass 3: `red-f7.log` (hunter/skald/investigator draft: `2 passed; 5 failed`, rows absent);
  `red-f7-desktop.log` (investigator known row filtered out); `red-f7b.log` (witch, arcanist,
  alchemist: `0 passed; 3 failed`); `red-f3b-desktop.log` (`left: None right: Some(-1)`); TS reds:
  `communityTheme.test.ts` assertion failed before the rule; `uiSmokeProbe`, `skillsModel` failed
  on the missing export; `spellDialogModel` `alchemist: … got {}` and `Witch level 0 (all cantrips),
  got Witch level 0`. `sheet_skill_trait_modifiers` and `formula_book_size_rows` unit tests were
  written with their function (the behaviour tests above were red first).

## Verification (final pass, after all edits)

| command | result |
|---|---|
| `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` (repo root, log `root-full-3.log`) | 299 suites ok, 0 failed; 6551 passed, 0 failed |
| `cargo test --locked -j 8 --no-fail-fast -p codex-desktop -- --test-threads=8` (log `desktop-full-4.log`) | `Running unittests src/main.rs`: 793 passed, 2 failed (+ 1 other suite ok) |
| rerun of the 2 failing tests (log `desktop-rerun-4.log`) | `5 passed; 0 failed` (filter `the_draft_spell_options the_saved_sheet_carries`) |
| `npm run typecheck` / `npm test` | exit 0 / `156/156 test files passed.` |
| `npm run ui-smoke:doc` | `Wrote docs/testing/ui-smoke-inventory.md (104 rows, 18 screens).` |

The 2 desktop failures, attributed: (1) `the_draft_spell_options_state_a_wizards_spellbook_size…`:
my F7 commit 15d3bd5acd changed the known filter to `contains("_book_size.")`, which misses
`spellbook_size.` — a real regression (wizard draft quotas empty), fixed to `contains("book_size.")`
in 3085ec9c4e; (2) `the_saved_sheet_carries…`: the test assumed Swim untrained, but create gives a
Fighter 1 rank each in Climb/Intimidate/Swim; the test now checks Appraise (−2) and Climb (none).
Desktop total after the fix: 794 + 2 = 796 passed, 0 failed (derived from the two logs; the full
desktop suite was not rerun). Counts: `awk '/^test result/{p+=$4; fl+=$6} END{print p, fl}' <log>`;
suites: `awk '/^test result: ok/{ok++} /^test result: FAILED/{f++} END{print ok, f}' <log>`; logs in
`/home/ubuntu/cargo-target/brief1/logs/`.

Earlier passes (superseded): pass 1 root 6535/0, desktop 784/0; pass 2 root 6538/0, desktop
785/0 (after one picker-guard fix).

Live app (Xvfb, `RUN_DESKTOP_AGENT=brief1`, `CODEX_REPO_ROOT=<worktree>`; without it the app reads
the corpus from the custom target dir and shows "No race could be read"): level-5 Wizard spells
dialog, Traits dialog with drawbacks, a saved Fighter with 3 traits + Meticulous, the F1 theme
before/after, and the F3b skill rows (above). The `create-character-render` smoke row stays red on
a cold launch only for "'Loading' still present after the row's wait budget" (the class catalog
loads slower than the row waits); its `headingExact` passed on every run.

## Decisions

- Drawbacks: 17 (`grep -rli "Drawback ~" data/corpus/ultimate_campaign | wc -l`); id
  `drawback:<slug>`; 16 printed text; Meticulous −2 computed on 0-rank skill totals.
- Count rule: traits ≤ 2, or ≤ 3 with exactly one drawback; drawbacks ≤ 1; enforced in the dialog
  and in `create_character_at_root`.
- No picker (read-only per-day text): cleric, druid, paladin, ranger, warpriest, shaman,
  antipaladin; any other caster with per-day rows but no known rows keeps the "not counted yet" line.
- F6: grouped pairs only with `pairPerGroup` (spells).
- F2: `N innate · M replaced by alternates`; "Swap racial traits".

## Four-check Wired Integration audit (`tranche-17-ui...HEAD`)

```
1: OK_NO_TOKENS
2: OK_NO_NOOP_HANDLERS
3: OK_NO_MOCK_LEAKS
4: OK_NO_WOULD_STRINGS
```

## Uncovered (what the proofs do not cover)

- **Witch and Arcanist book size use the wizard rule**, not a class-specific line (ruling A). Greps
  that found no class line: `grep -rln "0-level witch spells\|witch spells plus\|0-level wizard spells plus\|all 0-level wizard" data/corpus ~/workspace/repos/pcgen/data/pathfinder`
  → only `core_rulebook/class_feature/wizard/spellbooks.json` and `cr_abilities_class.lst`;
  `grep -rlE "three 1st-level|plus three|0-level witch spells|spellbook containing" data/corpus/advanced_players_guide data/corpus/advanced_class_guide`
  → only `lore_mystery/arcane_archivist.json`. The Witch familiar record says only "The familiar
  stores all the spells that you know".
- Hunter level 1 and Skald level 19 rows as above (oracle values, not checked against print).
- The existing Hunter known-spell cap check (`unmet_hunter_known_spell_conditions`) still treats
  KNOWN as the chosen cap (one more than the dialog allows); not changed.
- Skill rows: Craft/Perform/Profession family rows get no trait modifier (their rows name no
  subtype); trained-only skills at 0 ranks get none (the engine reports no total).
- Multiclass casters; opposed-school cantrip exclusion; Int too low to cast a level.
- F1: one dev build on Xvfb; the release build was not inspected.
- Bloodrager/Bard known rows parse through the same regex; only Sorcerer, Hunter and Skald are
  tested end to end.

## Not done

- (Closed by the lead 2026-10-12.) Full desktop suite rerun at 6966da45a0: `cd apps/desktop/src-tauri && CARGO_TARGET_DIR=/home/ubuntu/cargo-target/brief1 cargo test --no-fail-fast -- --test-threads=8` → 795 passed + 1 passed, 0 failed, 2 ignored (2 `test result` lines, summed with awk).
