# SD-36 Epic F4c: the Create picker and Level Up read the class roster from the engine (receipt)

Scope: the frontend half of F4 (`epic-f-class-completion.md` §6; acceptance F4.2 in
`epic-breakdown.md`), plus two fields on F4b's wire DTOs that the frontend needed. No `data/**`
change. Branch `sd36/epic-f4-f5`, worktree `/home/ubuntu/workspace/worktrees/codex-epic-f4`, on
top of F4b (`29d69473a7`).

Logs in this directory: `f4c-red.log`, `f4c-green.log`, `f4c-verify.log`. Wire artifacts:
`f4c-class-roster-wire.json`, `f4c-level-up-fighter6-wire.json`.

## 1. What changed

**New `apps/desktop/src/characterHub/classCatalog.ts`.** Holds the class catalog state and
imports no values: `source` (`loading | roster | fallback`), `options` (the Create picker),
`known` (label, hit die and skill ranks for every census id, offered or withheld), and `notice`.
`findClassOption` and `knownClass` are the lookups every model now uses. While the catalog is
`loading`, nothing is offered and no lookup falls back to a compiled-in list.

**New `apps/desktop/src/characterHub/classRoster.ts`** (+ `classRoster.test.ts`, self-executing).

- Wire types that mirror `character_hub.rs` verbatim.
- `classOptionsFromRoster`: one option per served class, in served order.
- `groupClassOptionsByFamily`: contiguous family runs, used for the `<optgroup>`s.
- `installClassRoster` / `installClassRosterFailure` / `ensureClassRosterLoaded`: load once,
  then install the roster or the fallback. The fallback always carries its notice. An empty
  roster counts as a failure.
- `useClassCatalog`: a React subscription.
- `loadLevelUpClassOptions`, `levelUpChoiceGroups`, `describeEntryRequirement`: the three Level
  Up groups and the printed requirement lines.
- `fallbackLevelUpResponse`: covers `list_level_up_class_options` failing. It offers advance and
  add-base from the catalog, with the failure printed, no prestige class, and the 20-level cap.

**`characterHubModel.ts:409`.** `CLASS_OPTIONS` is now `CLASS_OPTIONS_FALLBACK: readonly
ClassOption[]`, the same 31 rows unchanged. The only code that reads it is `fallbackCatalogState`,
which runs only when the roster command fails. The form then prints `class roster unavailable:
<diagnostic> — offering the built-in list of 31 classes instead.` (`role="alert"`).
`getLevelOptionsForClass` / `clampLevelForClass` / `canTakeAnotherLevelIn` read the catalog.
`ClassOption.hitDie` is now `number | null`, and the type gains `family`, `familyLabel` and `book`.

**`CreateCharacterForm.tsx`.** Waits for both rosters. The class `<select>` renders the catalog's
options grouped by family, so option count = roster length: 59 on a healthy checkout. The DEV
probe's `selectsNonEmpty: ["character-race", "character-class"]` (`spec.json:342`) covers it. The
fallback notice is printed above the select. The HP box prints `Unknown` when no hit die is known.

**`LevelUpDialog.tsx`.**

- Reads `list_level_up_class_options` for the saved character.
- The select has three option groups: "Advance a class you have", "Add a base class", "Add a
  prestige class".
- A selected prestige class prints each entry requirement in the rule's words with its note
  (`— met` / `— unmet` / `— situational: <condition>`). Accept stays enabled (§9.2).
- At the level cap, the dialog says so and offers nothing.
- Command diagnostics are printed.
- The hit die and skill points show `Unknown` when not stated. They no longer fall back to
  `?? 8`.

**Model consumers.**

- **`characterProgression.ts`:** skill ranks, hit die and labels come from `knownClass`. The
  `CLASS_SKILL_POINTS` table is deleted. `classHitDie`, `classSkillPointsBase`,
  `totalSkillPoints` and `maxHitPoints` return `null` rather than an assumed d8 or 2.
- **`skillsModel.ts`:** `totalSkillPointsAvailable` can be `null`. The new
  `classSkillListCoverage` and `heldClassesWithoutClassSkillList` feed a Skills-panel note that
  names each held class with no class-skill list.
- **`spellsTabModel.ts`:** the source-class label comes from `knownClass`, which covers prestige
  classes too.
- **`classPreviewModel.ts`:** the preview takes the picked roster option (id + label). An
  "Unavailable" message names both.
- **`characterHubRuntime.ts`:** the outcome context reads `findClassOption`.
- **`CharacterSheet.tsx` and `SkillAllocationDialog.tsx`:** subscribe to the catalog. HP, skill
  points and the allocation budget print `Unknown` when null (an Unknown budget allows no spend),
  and the Skills panel prints the class-skill note.

**Backend (`apps/desktop/src-tauri/src/character_hub.rs`), wire additions only:**

- `ClassCreationEntryDto` gains `skillRanksPerLevel` (from
  `skill_ranks_per_level_from_package`) and `hitPointsDie`.
- `WithheldClassDto` gains `hitDie`, `hitPointsDie` and `skillRanksPerLevel`, so a character
  holding a prestige class still gets its label and figures.
- `hitPointsDie` is the chassis record's hit die: the one the engine's hit-point fold reads
  (`class_chassis_sheet_rules::records`, loaded once). It is `None` when the class has no chassis
  record.
- Roster membership is unchanged: 59 offered and 78 withheld, as in F4b.

## 2. One rule for hit points, and the defect it keeps off the sheet

The printed `Hit die` row (`hitDie`, the roster rule's input) and the chassis die (`hitPointsDie`)
agree on all 54 of the 59 offered classes that have a chassis record. The Rust test asserts
equality on each. The other **5 of 59** have no chassis record:

- `class:monk`
- `class:unchained_barbarian`
- `class:unchained_monk`
- `class:unchained_rogue`
- `class:unchained_summoner`

The engine's fold reports their hit points Unknown (`class_chassis.hit_points.unknown`), and every
HP figure in the frontend now does the same.

This matters for Monk. Its printed row is the **FS-23** oracle defect (`HD:10`; CRB p.56 says d8).
Wiring the frontend to the printed row would have turned the correct d8 HP into a wrong d10 HP.
FS-23 says that fix must land before anything makes Monk HP computable. The RED in
`f4c-red.log` (`class:monk hit die: the one the HP fold reads: expected null, got 10`) is that
wiring, caught. The four Unchained classes lose a correct HP figure to Unknown under the same
rule, because each is a class-selection class whose die came from its base class's line. Section 5
names this remainder.

`CLASS_OPTIONS_FALLBACK` keeps Monk at d8 (the operator ruling of 2026-07-29). The fallback drift
guard compares label on 31 of 31 rows. It compares hit die on the 26 of 31 where the fold states
one; the other 5 are the 5 above.

## 3. RED → GREEN

Every RED in `f4c-red.log` is the new test run against the pre-F4c behaviour: the HEAD module, or
the hardcoded 31 installed as the catalog.

| test file | RED | GREEN |
|---|---|---|
| Rust `list_class_roster_wire_carries_hit_die_and_skill_ranks_for_every_census_class` | compile: `no field skill_ranks_per_level` / `hit_die` | ok (in 621/621) |
| `classRoster.test.ts` | module absent; then `class:monk hit die … expected null, got 10` | pass |
| `characterHubModel.test.ts` | `class:adept level option count: expected 20, got 1` (the hardcoded 31 does not offer the census roster) | pass |
| `characterProgression.test.ts` | `Arcanist: 2 + Int (the record), not the old table's 3: expected 2, got 3` | pass |
| `skillsModel.test.ts` | `class:alchemist is named, not silently all-cross-class: expected class:alchemist, got ` | pass |
| `spellsTabModel.test.ts` | `expected Known · Psychic Warrior, got Known · Psychic_warrior` | pass |
| `classPreviewModel.test.ts` | `a catalogued class at a catalogued level yields a row: expected Row, got Unavailable` | pass |

The frontend tests read the real wire, not a sample. `testSupport/classRosterWire.ts` reads the
two JSON artifacts. The Rust test above serialises the live `build_class_creation_roster()` and
`build_level_up_class_options(Fighter 6)` and fails on any byte difference, unless it is run with
`CODEX_WRITE_F4C_WIRE=1`. In the full desktop run it compared the artifacts and did not rewrite
them.

## 4. Counts (denominator, command)

| figure | value | command / source |
|---|---|---|
| Create picker options | **59** (= roster length) of 137 census ids | `classRoster.test.ts` over `f4c-class-roster-wire.json` |
| newly offered vs the hardcoded list | **28** (59 − 31), e.g. samurai, magus, warrior, kineticist, gunslinger | same |
| family groups | 7: CRB 11, APG 6, ACG 10, Unchained 4, UC 3, Untabled 20, CRB NPC/Ex 5 | same |
| classes known to the catalog | **137** (59 offered + 74 prestige + 4 ex_state) | same |
| Level Up, Human Fighter 6 | advance 1 (Fighter → 7), add base 58, add prestige **74 of 74** | `f4c-level-up-fighter6-wire.json` |
| Arcane Archer requirement notes | 4 lines: unmet, unmet, unmet, met (BAB ≥ 6); still selectable | same |
| deleted skill-ranks table vs the records | wrong for **33 of 59** roster classes (13 of the hardcoded 31, e.g. Arcanist 3 vs 2, Inquisitor 2 vs 6) | wire `skillRanksPerLevel` vs the deleted `CLASS_SKILL_POINTS` + its default 2 |
| offered classes with skill ranks stated | 59 of 59 | Rust wire test |
| withheld prestige classes with a hit die | 74 of 74 (chassis die, 74 of 74) | Rust wire test / wire JSON |
| offered classes whose HP prints Unknown | **5 of 59** (§2) | Rust wire test |
| roster classes with a class-skill list | **12 of 59** (47 named on the Skills panel) | `skillsModel.test.ts` |

## 5. Verification

| gate | command | result |
|---|---|---|
| F4.2 acceptance | `cd apps/desktop && npm test -- classRoster characterHubModel characterProgression skillsModel` | 126/126 files passed. `scripts/run-tests.mjs` ignores its arguments, so this is the full suite. |
| typecheck | `cd apps/desktop && npm run typecheck` | exit 0 |
| full frontend | `cd apps/desktop && npm test` | 126/126 files passed (was 125 + `classRoster.test.ts`) |
| desktop clippy | `cargo clippy --locked --tests -j 8 --manifest-path apps/desktop/src-tauri/Cargo.toml -- -D warnings` | exit 0 |
| full desktop | `cargo test --locked -j 8 --no-fail-fast --manifest-path apps/desktop/src-tauri/Cargo.toml -- --test-threads=8` | 621 passed / 0 failed (F4b's 620 + the wire test) |

The root crate is untouched, so the root suite was not re-run.

## 6. Carried forward (named by mechanism, not done here)

- **F4.4 ui-smoke rows.** `create-character-{samurai,magus,warrior,kineticist,inquisitor-generic}`
  and `level-up-fighter6-into-arcane-archer` are not yet in `spec.json`. No app was launched in
  this step. The Level Up accept path for a prestige class (`level_up_character` with a prestige
  class id) was not driven.
- **Architecture head counts (F5).** These still describe the hardcoded 31 and are F5's
  "five sites together" edit:
  - `docs/architecture/desktop-app.md:412`
  - `docs/architecture/status.md:67`, `:191`, `:205` ("32 of 63 … not in the Create picker")
- **Class skills: 47 of 59 roster classes have no list.** `skillsModel.CLASS_SKILLS` is a 12-row
  hand table. These classes are now named on the Skills panel instead of being silently scored
  all-cross-class. The fix is to read the converted records' `CSKILL` grants
  (`*_class_skills` class features in `data/sheet_rules`), not to widen the table.
- **HP Unknown for 5 of 59 roster classes (§2).** Two things close it: FS-23's book-cited Monk
  override (a converter step), and the frontend reading HP from the engine's own sheet rather
  than recomputing it from a die.
- **Two frontend class tables remain, outside this step's list:** `CASTER_CLASSES` (6 ids,
  `casterLevel`) and `MARTIAL_WEAPON_CLASSES` (5 ids, `classWeaponProficiency`) in
  `characterProgression.ts`.
- **List screens do not subscribe to the catalog.** `LoadCharacterScreen` and the campaign sheets
  call `formatHeldClasses` without subscribing. They show served labels once the catalog is loaded
  (it loads on the first Create form or sheet mount) and the id-derived label before that.
