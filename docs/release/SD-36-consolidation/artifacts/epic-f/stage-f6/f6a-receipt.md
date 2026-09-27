# F6a receipt — desktop hand-kept class tables → the engine

Stage F6, step F6a (items F6-1 and F6-2a of `polish.json`). Branch `sd36/epic-f6-desktop-polish`,
cut from tranche/16 `0a234a035a`. Out of scope here: F6-2b (Alertness +2), F6-3 (HP Unknown),
F6-4, F6-5, F6-6.

## 1. Inventory — every class-keyed table in `apps/desktop/src` (non-test)

Found with `grep -rn "'class:[a-z_]*'" apps/desktop/src` plus `grep -rnE "Record<(string|ClassId)"`
and a bare-class-name grep over `spellsTabModel` / `skillsModel` / `classPreviewModel` /
`spellsPerDayModel` (these three carry no class-keyed table). Line numbers are tranche/16 HEAD
`0a234a035a`.

| # | Table (file:line at HEAD) | Keys | Kind | Disposition |
|---|---|---|---|---|
| 1 | `MARTIAL_WEAPON_CLASSES` — `apps/desktop/src/characterHub/characterProgression.ts:62` | 5 class ids | rules data (weapon tier) | **deleted**; `classWeaponProficiency` deleted |
| 2 | `CASTER_CLASSES` — `apps/desktop/src/characterHub/characterProgression.ts:240` | 6 class ids | rules data (caster level) | **deleted**; `casterLevel` deleted |
| 3 | `CLASS_SKILLS` — `apps/desktop/src/characterHub/skillsModel.ts:52` | 12 class-skill lists | rules data (class skills) | **deleted**; `hasClassSkillList`, `heldClassesWithoutClassSkillList`, `classSkillListCoverage` deleted |
| 4 | `CLASS_OPTIONS_FALLBACK` — `apps/desktop/src/characterHub/characterHubModel.ts:421` | 31 rows (label, hit die) | command-failure fallback | **kept, unchanged**: installed only when `list_class_creation_roster` fails, always with the visible notice `class roster unavailable: <diagnostic>` (`classRoster.ts:191`) — the allowed typed-fallback shape |
| 5 | `WIZARD_CLASS_ID` — `CharacterSheet.tsx:243` | 1 id | UI routing (which held class a spell pick is attributed to) | not rules data; unchanged |
| 6 | `POOL_REFERENCE_SECTIONS` — `CharacterSheet.tsx:1978` | 2 rows (Rogue Talent, Rage Power) | UI headings for the engine's registered pool groups | not rules data; unchanged |
| 7 | `holdsRogue` — `CharacterSheet.tsx:2323` | 1 slug | UI gate for the Rogue-talent reference section | not rules data; unchanged |

Rules-data class tables: **3 of 3 deleted** (rows 1–3). The single remaining table that states
per-class rules values (row 4) is the command-failure fallback with its visible notice.

## 2. What replaces them

- **Engine** `src/rules_core/pilot_compute/class_facts_sheet_rules.rs` (new): `class_facts(class_id, level)` →
  - weapon proficiency: the engine's own per-class rule (the one `character_weapon_proficiency` applies):
    the static `CLASS_WEAPON_PROFICIENCIES` row, else the converted-record reader
    `class_weapon_proficiency_view`; Unknown keeps its reason;
  - caster level: every rule the class's own `held_set` holds whose target is
    `CasterLevel(Class(<slug>))` (or its `TakenOnClass` base class's line), evaluated by the one
    evaluator. None held: `NotACaster` when the class declares no `SpellType` (the same caster signal
    `class_spell_levels` reads) or the converter attests the closure complete; otherwise Unknown by
    name. A resolved value below 1 is "no caster level at this level" (Paladin 1–3);
  - class skills: `class_skill_view`, verbatim.
- **Command** `list_class_facts` (`apps/desktop/src-tauri/src/class_facts.rs`, new; registered in
  `main.rs`): read-only, one entry per requested `(classId, level)`:
  `{weaponProficiency {status, source, tiers, named, groups, sets, printed, reason}, casterLevel {status, value, source}, classSkills {status, skills, groups, reason}, hitDie}`.
- **Desktop** `boundary/listClassFacts.ts`, `characterHub/classFactsModel.ts` (new): the sheet asks for
  each held class at its own level and folds the answers — PF1 union for proficiency, one caster level
  per casting class (`Wizard 7 / Cleric 1`, never a sum). `skillsModel.classSkillLookup` does the class-skill
  union with the engine's `ClassSkillView::contains` membership rule. Loading prints `…` / "Loading class
  facts…"; a failed command prints Unknown everywhere plus the notice `class facts unavailable: <cause>`;
  there is no fallback table.
- Weapons tab: `✓ / ✗ / ?` per tier, the named weapons, weapon groups and weapon-set members the classes
  grant ("Also proficient with: Katana, Naginata, Wakizashi, …"), each printed-not-counted grant, and
  the Unknown classes by name.

## 3. RED → GREEN

RED (`f6a-red.log`, a probe of the hand-kept path at `0a234a035a`, `tsx`, exit 1):

```
FAIL Samurai Weapons tab shows Martial ✓ (the reader grants Martial)
FAIL Warrior Weapons tab shows Martial ✓
FAIL Magus Weapons tab shows Martial ✓
FAIL Magus 5 caster level = 5 (desktop: 0)
FAIL class-skill list present for the 50 of 59 roster classes the engine answers (desktop: 12 of 59)
```

GREEN (`f6a-green.log`, the same five checks through the engine path over the served wire, exit 0;
the wire carries levels 1 and 7, so the Magus check is at 7):

```
PASS Samurai / Warrior / Magus Weapons tab shows Martial ✓
PASS Magus 7 caster level = 7
PASS class-skill list present for the 50 of 59 roster classes the engine answers (desktop: 50 of 59)
```

Live app (`apps/desktop/scripts/ui-smoke/run.mjs`, DEV probe, one app, stopped at the end): 3 new
spec rows `sheet-f6a-samurai-weapons`, `sheet-f6a-warrior-weapons`, `sheet-f6a-magus-caster-level` —
**3/3 green** (`artifacts/ui-smoke/f6a/results.json` + screenshots). The first live run
(`artifacts/ui-smoke/f6a/red-first-run/`) was 2/3: the Samurai row read the sheet while the cold
`list_class_facts` call was still in flight (`Caster Level …`, `Martial Weapons (Unknown)`); the tab and
Skills panel now print "Loading class facts…" / "Loading class skills…" while loading, so the harness's
`Loading` wait covers it. Regression on the rows the change touches — `sheet-open-for-tabs`,
`sheet-tab-weapons`, `sheet-action-skill-allocation-dialog`, `sheet-action-add-spell`,
`sheet-action-level-up-dialog` and the five `create-character-*` F4 family rows: **10/10 green**
(`artifacts/ui-smoke/f6a/regression/`). `spec.json` now has 79 rows (76 + 3); the inventory doc was
regenerated (`npm run ui-smoke:doc`).

## 4. Counts (denominator: the 59 roster classes)

Command: `cargo test --locked -j 8 --lib class_facts_sheet_rules -- --test-threads=8 --nocapture`
(`every_roster_class_answer_is_counted`: `class_census::class_creation_roster()`, each class at every
level `1..=max_level`). Full table: `f6a-roster-measure.txt`. Pinned in the test.

| Fact | Answered at every level | Unknown |
|---|---|---|
| Weapon proficiency | **59 of 59** | 0 |
| Caster level | **59 of 59** (37 cast at some level, 22 cast nothing) | 0 |
| Class skills | **50 of 59** | 9 |

The 9 class-skill Unknowns are one mechanism, named on the sheet with the engine's reason: the ACG
classes `arcanist, brawler, hunter, investigator, shaman, skald, slayer, swashbuckler, warpriest`. Their
class line's `Class|<Class>` grant is an unresolved reference in the converted package
(`data/sheet_rules/_defects/unresolved-references.json`), so the reader's walk never reaches the class's
own `<Class> ~ Class Skills` record, which the package does carry (the same remainder
`UNREAD_RECORD_SELECTED_CLASS_SKILLS` documents in `feat_pillar_and_pool_aggregation.rs`, whose oracle
rows cover only Climb/Intimidate/Swim). Closing it is a converter step (resolve the edge), not a desktop
table. The Skills panel prints `Class skills Unknown for <Class> (<reason>)` and applies no +3 for them.

Before: weapon tier from a 5-id table (the engine grants Martial to 24 of the 59 roster classes at some
level; the table marked 5, so 19 printed `✗ Martial Weapons`, Samurai, Warrior and Magus among
them; and it printed `✓ Simple` for every class, Wizard/Druid/Monk included, which grant named weapons,
not the tier), caster level from a 6-id table (31 of the 37 casting roster classes printed `—`), class skills
from 12 hand lists (47 of 59 had none).

The wire the frontend tests read, `f6a-class-facts-wire.json` (59 classes × levels 1 and 7 = 118 rows), is
pinned against the live command by
`class_facts::tests::class_facts_wire_for_every_roster_class_matches_the_committed_artifact`.

## 5. Verification (one pass, after all changes)

| Check | Command | Result |
|---|---|---|
| root lib | `cargo test --locked -j 8 --lib -- --test-threads=8` | 2733 passed, 0 failed, 6 ignored |
| desktop crate | `cargo test --locked -j 8 --manifest-path apps/desktop/src-tauri/Cargo.toml -- --test-threads=8` | 624 passed, 0 failed |
| clippy root / desktop | `cargo clippy --locked --tests -j 8` (each crate) | exit 0, 0 warnings in either log |
| frontend | `npm run typecheck && npm test` (apps/desktop) | clean; 127/127 test files |
| PCGen residue | `python3 scripts/pcgen_residue_gate.py --check --closure` | live_hits=0, PASS |
| ui-smoke | `run.mjs --only …` (§3) | 3/3 new, 10/10 regression |

Frontend tests per changed model file: `classFactsModel.test.ts` (new), `skillsModel.test.ts`
(rewritten over the served wire), `characterProgression.test.ts` (table tests removed).
`data/corpus/**`, `site/**` and `data/sheet_rules/**` untouched.
