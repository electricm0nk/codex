# Desktop sheet print-path map (F1b stage-4, read-only)

READ-ONLY investigation. Nothing in the worktree was edited; `data/sheet_rules` was
temporarily swapped to the F1 `<scratch>/dump-after` dump for item 4's population sweep and
restored with `git checkout -- data/sheet_rules && git clean -fd data/sheet_rules` (`git status
--short` confirmed empty before the swap and empty again after — see §4). `python3
scripts/pcgen_residue_gate.py --check --closure` → `verdict=PASS` after restore.

Per this step's instructions, this file is written to scratch only; copying it to
`docs/release/SD-36-consolidation/artifacts/epic-f/stage4/` is the NEXT step, not this one.

---

## 1. Every path that prints class features / racial traits / feats / proficiencies / "rules
   and features" lines

`CharacterSheet.tsx` is 4,536 lines. Every render path that shows rule text or a rule-derived
magnitude on the sheet, front-to-back:

| # | Sheet section | Frontend component / model | Boundary call | Tauri command / backend field | Backend source of the TEXT |
|---|---|---|---|---|---|
| 1 | **Class Features** (in the "Actions" tab, `ActionsTab`) | `CharacterSheet.tsx:2278-2467` (the tab body) + `classFeaturesModel.ts` `buildClassFeatureSurface` | `loadSavedCharacterDetail.ts` (`explanations` field, `:61`) | `LoadSavedCharacterResponse.explanations` (`character_hub.rs:592`) | **(a) bespoke `pilot_compute` explanations** — every `class_feature.*` / `class_chassis.*` id the per-class Rust modules (`class_barbarian.rs`, `class_slayer.rs`, `combat.rs`, ...) emit. `detail` is rendered verbatim (`classFeaturesModel.ts:80-81`). |
| 1b | **Class Features → corpus description** (same rows, sub-line) | `classFeaturesModel.ts` `findCorpusDescription` (`:211-223`), rendered `CharacterSheet.tsx:2383-2394` | `loadClassFeatureDescriptions.ts` + `loadClassFeatureFeatBridgeDescriptions.ts` (`CharacterSheet.tsx:2270-2274`) | two Tauri commands backed by `class_feature_descriptions.rs` and `class_feature_feat_bridge.rs` | **(b) the class-feature description catalog** — `data/corpus/<book>/class_feature/<class_slug>/<feature_slug>.json`, read by `class_feature_descriptions.rs` (doc comment `:1-60`), joined onto a row an EXPL id already created via `matchesCorpusFeature` (`classFeaturesModel.ts:192-198`: `id.includes('.{classSlug}.') && id.endsWith(featureSlug)`). |
| 1c | **Class Features → "Not computed"** notices | `classFeaturesModel.ts:305-328` `noticeHasSheetRule`, rendered `CharacterSheet.tsx:2404-2436` | same as #1 (`explanations`) + `sheetLines` | same as #1 + #4 | An `.unsupported` EXPL is suppressed from "Not computed" when a sheet_rule already prints the same facet — the frontend's OWN copy of the naive `<class>_<tail>`/`<tail>` join spec 3b.2 calls broken (`classFeaturesModel.ts:319-320` mirrors `sheet_rule.rs:1962-1971` exactly, segment for segment). |
| 1d | **Class Features → browsable "unmatched" descriptions** (T4 fix) | `classFeaturesModel.ts:271-290` `unmatchedClassFeatureDescriptions` | same as #1b | same as #1b | (b) again — every corpus description for a held class (or a held-feat-granted synthetic pool, `class_feature_feat_bridge.rs`) that #1/#1b did NOT already attach to a grounded EXPL row. Rendered where? Not directly traced in this pass — `ClassFeatureDescriptionReferenceSection` (`CharacterSheet.tsx:2068`, item #1e). |
| 1e | **Class Features → pool-member reference lists** ("Available Rogue Talents", "Available Rage Powers") | `ClassFeaturePoolReferenceSection` (`CharacterSheet.tsx:1987-2062`) | `loadClassFeaturePoolOptions.ts` | `class_feature_pool_picker.rs` | (b)-adjacent — a menu of every real pool member a class's corpus rows declare, independent of whether the character selected it. Data-driven by `POOL_REFERENCE_SECTIONS` (`CharacterSheet.tsx:1952-1971`), currently Rogue Talent + Rage Power only. |
| 2 | **Racial Traits** section | `RacialTraitsSection` (`CharacterSheet.tsx:1843-1942`) + `racialTraitsModel.ts` `buildRacialTraitsSurface` | (inline in `loadSavedCharacterDetail` response, `resolvedRacialTraits` field) | `LoadSavedCharacterResponse.resolved_racial_traits` (`character_hub.rs:639`) | **(d) `race_trait_picker::build_race_selection_for_feats`** (`RaceCorpus::resolve` + `render_trait_description`) — a SEPARATE resolver from `sheet_rule`. `text` is corpus prose with the character's own feats already resolved into it (`racialTraitsModel.ts:1-35`). |
| 3 | **"Rules and features"** section (in the Actions tab, below Class Features) | `RulesAndFeaturesSection` (`CharacterSheet.tsx:2178-2232`), mounted at `:2438` | `loadSavedCharacterDetail.ts` (`sheetLines` field, `:122`) | `LoadSavedCharacterResponse.sheet_lines` (`character_hub.rs:667`) via `sheet_lines_for` (`character_hub.rs:742-755`) | **(c) `render_sheet`** over `held_set` from `HeldSeed::from_character` (`sheet_rule.rs:1299-1331`) + `with_sheet_rules` (`class_shared_core.rs:40-52`). This is the converted `data/sheet_rules/` package — every held, `print:true` rule's line, one per class/race_trait/feat/trait/equipment/spell/skill/ability/companion/domain kind. |
| 4 | **Feats tab** (`FeatsTab`, `CharacterSheet.tsx:2497+`) | `featsTabModel.ts` `resolveSelectedFeatEntries` | `loadFeats`/`listFeats.ts` (unfiltered catalog, loaded once) | `feat_catalog.rs` | The catalog's OWN static description text (690 feats, 5 books) — **completely independent of `sheet_rule`**. Resolves each of the character's persisted `chosen.selected_feats` raw strings to `{name, description}`; a feat that resolves to nothing falls back to the raw string. |
| 5 | **Traits section** (below Feats) | `TraitsSection` (`CharacterSheet.tsx:2640+`) | `loadCharacterTraits.ts` | backend trait catalog | Static trait catalog text, same shape as #4, not traced further in this pass (out of the 4 named categories — traits are neither class features, race traits, feats nor proficiencies). |
| 6 | **Weapons tab — proficiency checklist** (Simple/Martial/Exotic ✓/✗) | `WeaponsTab` (`CharacterSheet.tsx:836-870`), fed by `classWeaponProficiency` | **none — no boundary call** | **none — no backend call at all** | **(e) a hand-authored frontend table**, `characterProgression.ts:66-92`: a hardcoded `MARTIAL_WEAPON_CLASSES` `Set` of 5 class ids, matched by exact `classId` string. Zero join to `sheet_rule` or to any corpus record. |

Legend, matching the task's (a)-(e): **(a)** bespoke `pilot_compute` explanations (item 1); **(b)**
the class-feature description catalog (items 1b/1d, and structurally the same shape for feats,
item 4's `feat_catalog.rs`, and for races, item 2's `race_trait_picker`); **(c)** `render_sheet`
sheet_lines via `with_sheet_rules` (item 3, `character_hub.rs:723-755`); **(d)** a rules_tables /
other-resolver Rust path distinct from `sheet_rule` (item 2's `race_trait_picker`); **(e)**
anything else — found exactly once, item 6's hardcoded weapon-proficiency table.

---

## 2. Class feature / race trait / feat / proficiency traces — TODAY vs AFTER the link repair

### 2a. Class feature: Barbarian 5, Uncanny Dodge

Barbarian gets Uncanny Dodge at class level 2, so a Barbarian 5 already has it.

**TODAY (before F1's link repair, current tracked `data/sheet_rules`):**

- **Path 1 (Class Features).** The bespoke Rust module grounds
  `class_feature.barbarian.uncanny_dodge` (confirmed in the stage-3 receipt's own duplicate
  listing, `docs/release/SD-36-consolidation/artifacts/epic-f/stage3/blast-radius-receipt.md:124-127`)
  — `detail` reads e.g. `"Barbarian Uncanny Dodge granted at barbarian level 7 (PF1 Core
  Rulebook, 2nd-level barbarian class feature, part of the...)"`. Shown as a `ClassFeatureRow`
  in the Class Features section, value a bare number (likely `0`, a filler — the module carries
  no real magnitude for this facet).
- **Path 3 (Rules and features).** `core_rulebook:class_feature:barbarian_uncanny_dodge`
  (`data/sheet_rules/core_rulebook/class_feature/barbarian_uncanny_dodge.json`, `print: true`,
  `value: Text`, label `"Uncanny Dodge"`) is **already held today**, because `HeldSeed`'s naive
  tail-walk (`sheet_rule.rs:1960-1976`) tries candidate `{class}_{tail}` = `barbarian_uncanny_dodge`
  for tail `uncanny_dodge`, and that candidate is an EXACT rule-slug hit — one of the "three
  pairs the naive spec gets right by accident" the epic doc names (§3b.2), just not one it
  enumerates by this exact name. So this line already prints today, alongside path 1 — **a
  pre-existing duplicate the stage-3 receipt's own delta-only methodology does not measure**
  (it only classifies NEWLY added lines, and this one was not new).

**Join key today:** the bespoke facet id's tail (`uncanny_dodge`) happens to equal the
converted rule's own trailing segment once `{class}_` is prepended — an id-slug coincidence, not
a citation-based join (no shared `source.path`/`source.line`, no shared display name check).

**AFTER F1's link repair (post-swap dump, verified against `<scratch>/l20`/`sheets/after`):** a
SECOND, previously-unheld converted rule becomes held and printed:
`core_rulebook:class_feature:barbarian_uncanny_dodge_tracker`
(`data/sheet_rules/core_rulebook/class_feature/barbarian_uncanny_dodge_tracker.json`, `print:
true`, label `"Barbarian ~ Uncanny Dodge Tracker"`, `closure_rows` citing
`cr_abilities_class.lst:432,437-439` — a DIFFERENT oracle line than the base rule's `:429`).
This is exactly the stage-3 receipt's own SUSPECTED-duplicate #1
(`blast-radius-receipt.md:124-127`, label-similarity score 0.852). It is a genuinely distinct
converted record (different `closure_rows`, not a `#` sibling of the base rule), so whether it
is a real duplicate or a legitimate second paragraph of Uncanny Dodge rules text is undetermined
by this pass — the receipt itself marks it SUSPECTED, pending R2/R3 (3b.2), which do not exist
on this branch (`git grep -c rule_for_explanation` → 0).

**Join keys available, both pairs:** rule id (exact match possible for path-1↔path-3-base
today; NO shared id for path-3-base↔path-3-tracker — they are two independent oracle rows for
the same PCGen `.lst` feature block); corpus source citation (`closure_rows` — different lines,
so citation-based dedup would correctly treat them as distinct); display label (`"Uncanny
Dodge"` vs `"Barbarian ~ Uncanny Dodge Tracker"` vs the bespoke EXPL's own free-text `detail` —
this is the ONLY join stage-3's classifier could use, since R2 does not exist yet).

### 2b. Race trait: Elf, Keen Senses

`data/sheet_rules/core_rulebook/race_trait/elf_keen_senses.json`: `print: true`, `value:
Number`, `target: Skill(perception)`, `bonus_type: Racial` — a real sheet-total-feeding number
(+2 Perception), not prose-only.

- **Path 2 (Racial Traits section).** `race_trait_picker::render_trait_description` renders the
  full sentence ("Elves receive a +2 racial bonus on Perception skill checks.") as prose,
  already held TODAY — race trait keys come straight from the race resolver
  (`resolve_racial_traits_for_character`, `character_hub.rs:748-749`), not from any F1 edge.
- **Path 3 (Rules and features).** The SAME rule, `core_rulebook:race_trait:elf_keen_senses`,
  is held today via `seed.race_traits` (`character_hub.rs:748-750` feeds `extra_race_traits`
  into `with_sheet_rules`) and prints as a NUMBER line (`+2`, target Perception) — this is the
  R3 "text always prints + number, when a bespoke value agrees" shape, except **no bespoke path
  computes a Perception number for this trait at all**: `skillsModel.ts` has no racial-bonus
  logic (confirmed: no `racial`/`perception` hits in that file). So the Skills panel's own
  Perception total never includes this +2 — the number is only ever visible as this separate
  Rules-and-features line, which the player must notice and add by hand. This is a wiring GAP
  (an unintegrated total), not a duplicate — the inverse failure mode from 2a/2c.

**Join key:** the resolver's own trait `key` string (`"Elf ~ Keen Senses"`) → `slug(key)` →
`package.find("race_trait", slug)` (`sheet_rule.rs:1955-1959`) — a direct, always-correct slug
match, same shape as the feat join (2d below), not the fuzzy class-feature walk.

**AFTER F1's link repair:** per 3b.1's own table, race_trait edges (349 of them) are Keen
Senses–shaped sub-abilities newly reachable once their PARENT trait is held — e.g. a
language-grant or weapon-familiarity sub-rule `granted_by` this trait. Elf Keen Senses itself
carries no such sub-grant in the sample checked here; the general case is "pure gain" per 3b.1
("Low: ... adds the granted sub-rule text ... unless a sub-rule has a numeric value the race
bundle also prints").

### 2c. Feat: Power Attack

`core_rulebook:feat:power_attack.json`: `print: true`, `value: Text` (no numeric target — the
real -X/+Y attack/damage tradeoff numbers are computed elsewhere, in the bespoke combat/attack
pipeline, not sourced from this record or from path 4 below).

- **Path 4 (Feats tab).** `feat_catalog.rs`'s own static description
  (`find("Power Attack").source == "Crb"`, `feat_catalog.rs:1030`) is shown for EVERY character
  that has `"Power Attack"` / `"feat:power_attack"` in `chosen.selected_feats`, via
  `resolveSelectedFeatEntries` — **unconditionally**. `featsTabModel.ts` has **no reference to
  `sheetLines` anywhere** (confirmed by grep) — unlike `classFeaturesModel.ts`'s
  `noticeHasSheetRule`, there is no suppression logic for feats at all.
- **Path 3 (Rules and features).** `package.find("feat", slug)` (`sheet_rule.rs:1948-1954`) is
  a DIRECT exact-slug lookup off `chosen.selected_feats` — always correct when a converted
  `feat:` rule with that slug exists, no fuzzy join involved. Prints the converted DESC/BENEFIT
  prose as a `"feat"`-kind line, unconditionally, for every held feat with `print: true`.

**This is a systematic, unconditional duplicate, not an occasional one**: any selected feat
that (a) has a converted `data/sheet_rules/**/feat/*.json` record and (b) that record's `print`
is `true` prints its description TWICE — once in the Feats tab (catalog text), once in Rules
and features (converted text) — on every load, for every character, with no join failure
involved (the feat join is exact-slug, always correct) and no dedup mechanism on either side.
This is unaffected by F1/F1b (feats are not part of the A-target edge set; 3b.1's table lists
only 6+1 feat/equipment edges, and those are downstream grants FROM an already-held feat, not
the feat's own held-ness). **Not measured or gated by any part of 3b's acceptance criteria**,
since 3b.2's R1-R4 scope is `class_feature` facets only.

**Join key:** exact rule-id slug (`id_slug(selected_feat)` → `package.find("feat", slug)`) —
the strongest join in this whole map; the duplicate exists despite a correct join, because
nothing on either print path checks the other.

### 2d. Proficiency line: Fighter and Wizard, Weapon and Armor Proficiency

`data/sheet_rules/core_rulebook/class_feature/weapon_and_armor_proficiency_fighter.json`
(**id** `core_rulebook:class_feature:weapon_and_armor_proficiency_fighter` — note the reversed
word order vs the naive join's expected `fighter_weapon_and_armor_proficiency`, confirmed by
listing the directory) and `.../wizard_weapon_and_armor_proficiency.json` (**id**
`core_rulebook:class_feature:wizard_weapon_and_armor_proficiency` — this one DOES match the
naive join's `{class}_{tail}` candidate). **Both have `"print": false`.**

- **Path 1 (Class Features).** `combat.rs:170-260` (`explain_base_class_weapon_and_armor_
  proficiency`, `ground_class_weapon_and_armor_proficiency`) grounds
  `class_feature.wizard.weapon_and_armor_proficiency` / `class_feature.cleric...` / etc. — shown
  as a `ClassFeatureRow`.
- **Path 3 (Rules and features).** **Never prints for either class, today or after F1**,
  because `render_sheet` filters on `r.print` at the RULE level (`sheet_rule.rs:2116`)
  independent of how (or whether) the rule got held — `print: false` wins regardless of join
  correctness. So spec 3b.2's own worked example (`class_feature.fighter.weapon_and_armor_
  proficiency` failing to join to `fighter_weapon_and_armor_proficiency`) is, for THIS specific
  record, moot for the printed sheet: even a corrected join (R2) would attach a rule that never
  renders a line. (R2 still matters for anything ELSE the join's correctness gates — e.g.
  `noticeHasSheetRule`'s "Not computed" suppression, item 1c above, which does not check `print`
  and would incorrectly suppress a real "not computed" notice against a rule that can never
  visibly resolve the claim.)
- **Path 6 (Weapons tab checklist).** `classWeaponProficiency(classId)`
  (`characterProgression.ts:92`) — a THIRD, wholly independent path: a hardcoded `Set` of 5
  class ids checked by exact string match, with **no join to any corpus record or rule id at
  all**. Fighter is in the martial set (hardcoded `true`); Wizard is not (hardcoded `false`,
  the function's own default). Neither value can ever disagree with path 1's prose (no shared
  computation), and neither can ever be VERIFIED against it either — they are two independently
  hand/engine-maintained claims about the same fact with zero cross-check.

**Join keys:** path 1 uses the bespoke `class_feature.<class>.weapon_and_armor_proficiency`
facet id, joined ad hoc per class inside `combat.rs`'s own Rust code (not through `sheet_rule`
at all). Path 3 (moot here) would use the same class_feature slug join as 2a. Path 6 uses no
join whatsoever — a class-id string membership test.

---

## 3. How the desktop decides which sheet_lines to show

Backend (`sheet_rule.rs`):

- `render_sheet` (`:2109-2131`) calls `held_set` (`:1925-2107`), then filters:
  1. drops any id in `held.removed` (a `Waives`/`Revokes` grant that fired, R4's scope) —
     `:2114`;
  2. resolves each remaining id to its `SheetRule` via `package.rule(id)` — an id with no
     backing rule record (should not happen for a held id, defensive) is dropped — `:2115`;
  3. **filters on `r.print`** (`:2116`) — the record's own author-set boolean, e.g. `false` for
     both weapon/armor proficiency records in 2d above;
  4. for a SIBLING rule (`r.id.contains('#')`, e.g. a `#bonusN` fact), additionally evaluates
     that sibling's own `applies` gate through the `Evaluator` (`:2123-2125`) — a sibling
     inherits `held`-ness from its principal (`held_set`'s `add` closure, `:1930-1932`) but must
     still pass its own conditional to actually PRINT (this is how `barbarian_rage#bonus2`
     "Rage (Str)" only shows while the situational `applies` condition holds).
  5. sorts by `(kind, label, id)` (`:2129`) — this IS the section grouping key downstream.
- No "family" or section concept exists in the backend beyond `kind` (the middle segment of the
  rule id: `class_feature`, `race_trait`, `feat`, `class`, `ability`, `companion`, `domain`,
  `equipment`, `spell`, `skill`, `monster_ability`, ...) — confirmed: `data/sheet_rules/` has no
  top-level "proficiency" kind; weapon/armor proficiency records live under `class_feature`.

Frontend (`CharacterSheet.tsx`):

- `groupSheetLinesByKind` (`:2165-2176`) does a single linear pass, grouping ADJACENT lines of
  the same `kind` — correct only because the backend already sorted by `kind` first (`:2129`);
  the frontend applies **no filtering of its own** — no print-flag check (there is none to
  check; the DTO carries no `print` field, only already-filtered lines), no family/section
  logic, no de-duplication against `explanations` or against itself. `RulesAndFeaturesSection`
  (`:2178-2232`) renders every line it is handed, unconditionally, grouped and labelled
  (`sheetLineKindLabel`, `:2155-2162`) but never dropped.
- The ONLY frontend-side suppression logic anywhere in this whole map is `noticeHasSheetRule`
  (`classFeaturesModel.ts:305-328`, item 1c) — and it only ever SUPPRESSES an `.unsupported`
  "Not computed" NOTICE, never a grounded `ClassFeatureRow`, never a Feats-tab entry, never a
  Racial-Traits-section row. Every other cross-path duplicate found in §2 is unmitigated by
  design as of this checkout.

---

## 4. Population-level duplicate estimate, F1b's actual (post-repair) print surface

**Instrument.** `class_census --sheet-dump <class>:20 --with-sheet-rules`
(`/home/ubuntu/workspace/worktrees/codex-epic-f1-target/debug/class_census`, already built) run
against `data/sheet_rules` temporarily swapped to `<scratch>/dump-after` (F1's post-link-repair
converter output), one run per base class, `-P 8`. **Population: 61 base classes** — every
`class_id` in `docs/release/SD-36-consolidation/artifacts/epic-f/census-f0.json`'s `classes`
array (F0's own base-class census; verified `len == 61`), each at character level 20. Output:
`<scratch>/l20/<class>.txt` (`EXPL|`/`DIAG|`/`HELD|`/`LINE|` text format,
`class_census.rs:601-672`). Swap restored via `git checkout -- data/sheet_rules && git clean -fd
data/sheet_rules`; `git status --short` confirmed empty both before the swap and after the
restore; `_report.json` confirmed `71862` (pre-swap) → `71863` (post-swap) → `71862` (restored).

**Method (documented limitation, same class of proxy stage-3 used, since R2/`rule_for_
explanation` does not exist on this branch — script:
`<scratch>/desktop-print-paths-population-count.py`):** for every `kind=class_feature` LINE row
in each build, normalise its `label` (lowercase, strip non-alphanumerics) and test whether that
normalised string appears as a substring of (A) the normalised, concatenated `detail` text of
every EXPL row in the SAME build (path 1's bespoke explanations — a population-scale version of
what stage-3's duplicate check compared one label against), or (B) the filename (normalised) of
any `data/corpus/<book>/class_feature/<class_slug>/*.json` description file for that class
(path 1b/1d's corpus catalog). Labels shorter than 4 normalised characters are skipped (too
generic — `"AC"`, `"CL"` would match almost anything). **This is a coarse, label-text proxy, not
a rule-id join — every count below is an upper-bound SUSPECT figure**, exactly as stage-3's own
`blast-radius-receipt.md:32-43` documents for its narrower, delta-only version of the same
check.

**Population totals (61 builds, level 20, `class_feature`-kind LINE rows only):**

| Denominator | Count |
|---|---|
| Total `class_feature`-kind LINE rows across all 61 builds | **1,021** |
| Total LINE rows of any kind across all 61 builds | 1,956 |
| ...whose normalised label also appears in that build's own EXPL `detail` text (path 1 vs path 3, class-feature duplicate shape) | **519 of 1,021 (50.8%)** |
| ...whose normalised label matches a corpus class_feature description filename for that class (path 1b/1d vs path 3) | **733 of 1,021 (71.8%)** |
| ...matching EITHER | **810 of 1,021 (79.3%)** |

**Caveat on the catalog figure (733/1,021):** `data/corpus/<book>/class_feature/<class_slug>/`
holds EVERY corpus description for that class, not only the ones actually reachable on screen
(`unmatchedClassFeatureDescriptions`, item 1d, only shows a description when no grounded EXPL id
already claims it). A catalog-name hit therefore only proves "a corpus description with this
name exists for this class" — not that it is actually rendered twice on THIS character's sheet.
The EXPL-detail figure (519/1,021) is the tighter, more direct proxy for an actual on-screen
duplicate, since both sides (path 1's `detail`, path 3's `label`) are things the SAME loaded
character's sheet actually shows.

**Per-class table** (class, class_feature LINE rows, hits vs EXPL detail, hits vs corpus
catalog, hits vs either):

```
class                   cf_lines  hit_detail  hit_catalog  hit_either
adept                          2           0            0           0
aegis                         30          10           26          26
alchemist                     20           2           19          19
antipaladin                   17          12           14          17
arcanist                      11           2           10          10
aristocrat                     2           0            0           0
barbarian                     27          10           24          24
bard                          27          20           11          22
bloodrager                    21          11           18          18
brawler                       18          10           17          17
cavalier                      17           7           16          16
cleric                          9           4            6           6
commoner                       0           0            0           0
cryptic                       24          13           17          18
dread                         17          10           11          12
druid                         21          14           19          19
ex_barbarian                  26           1            0           1
ex_paladin                    23           1            0           1
expert                          1           0            0           0
fighter                       24           4           16          16
gunslinger                    29           6           27          27
hunter                        38           9           35          35
inquisitor                    24           4           20          20
investigator                  15           5           13          13
kineticist                    29          18           19          19
magus                         20          11           20          20
marksman                      13           6            5           6
medium                        20          16           17          17
mesmerist                     18          15           15          15
monk                          37          25           36          36
ninja                         13           4            9          11
occultist                     17          17           17          17
oracle                          2           1            1           2
paladin                       23          13           16          16
psion                           3           1            0           1
psychic                       12          11           12          12
psychic_warrior                21           7           16          17
ranger                        25          14           19          19
rogue                         17           8           14          14
samurai                       18           3           14          14
shaman                         10           2            8           8
shifter                       20          18           20          20
skald                         22          15           18          18
slayer                        16          10           11          11
sorcerer                       10           7            7           8
soulknife                       8           7            7           7
spiritualist                  22          22           22          22
summoner                      14          13           14          14
swashbuckler                  27           7           15          15
tactician                     16          12           14          14
unchained_barbarian            12           8            0           8
unchained_monk                 22          19            0          19
unchained_rogue                14           8            0           8
unchained_summoner              16          15            0          15
vigilante                      13          11           13          13
vitalist                       16          11           10          11
warpriest                       0           0            0           0
warrior                          2           0            0           0
wilder                         15           6           11          12
witch                            2           0            2           2
wizard                          13           3           12          12
```

Full raw JSON (per-class breakdown + sample matched pairs for spot-checking):
`<scratch>/population-duplicate-estimate.json`.

**Notable pattern:** every one of the four Pathfinder Unchained classes (`unchained_barbarian`,
`unchained_monk`, `unchained_rogue`, `unchained_summoner`) has `hit_catalog = 0` — their
`class_feature` corpus description directories under `data/corpus/pathfinder_unchained/
class_feature/<class_slug>/` are either absent or use a naming convention this proxy's directory
lookup does not match (not investigated further in this read-only pass; a real follow-up
question for whoever builds R2/R3, since it means the (b) corpus-catalog source may be
structurally under-populated or differently keyed for that whole book).

**Classes with `cf_lines = 0`** (`commoner`, `warpriest`) and near-zero (`adept`, `aristocrat`,
`expert`, `warrior`, `oracle`, `psion` at 1-3): the CRB NPC classes and several untabled classes
ground almost no bespoke class features at all today, so the converted path is close to the
ONLY source of class-feature text for them — the opposite risk from the duplicate cases: for
these, F1's link-repair is a close-to-pure population gain (fewest existing lines to collide
with), while classes like `monk` (37 lines, 36 hit either) and `hunter` (38 lines, 35 hit
either) carry the densest pre-existing bespoke coverage and the highest population-level
duplicate-shape risk.

---

## Summary

| Path pair | Join key(s) available | Today | After F1 link repair |
|---|---|---|---|
| Class Features (bespoke EXPL) ↔ Rules-and-features (converted LINE) | rule id (only when the naive `{class}_{tail}`/`{tail}` walk happens to hit exactly — `sheet_rule.rs:1960-1976`); corpus citation (`closure_rows`, never compared); display label (the only thing stage-3's own classifier could use) | **Already duplicates in the "lucky exact match" cases** (e.g. Barbarian Uncanny Dodge base rule, 2a) — a pre-existing gap the stage-3 delta receipt does not measure | +8 SUSPECTED new duplicates across 70 sampled builds (`blast-radius-receipt.md`); population proxy over all 61 base classes @ L20: **519/1,021 (50.8%)** of class_feature LINE rows text-match an EXPL detail in the same build |
| Class Features description catalog (1b/1d) ↔ Rules-and-features | `(class_slug, feature_slug)` exact join for 1b/1d itself; label-text proxy vs LINE (this pass) | Present wherever a corpus description exists and is unmatched/matched | 733/1,021 (71.8%) label-match, but this OVER-counts (catalog holds every description, not just reachable ones — see caveat above) |
| Feats tab (static catalog) ↔ Rules-and-features | Exact slug join on BOTH sides (`feat_catalog.rs` lookup; `package.find("feat", slug)`) — the join is correct, the duplicate is structural | **Unconditional duplicate for every held feat with a `print:true` converted record** — no suppression exists (`featsTabModel.ts` never reads `sheetLines`) | Unaffected by F1 (feats are not an A-target kind) |
| Racial Traits (resolver prose) ↔ Rules-and-features | Trait `key` → `slug(key)` → exact rule-id match | Already both print today (e.g. Keen Senses); the NUMBER (+2 Perception) is orphaned — no bespoke skill computation consumes it | 349 new race_trait edges (sub-abilities of already-held parents) — "pure gain" per 3b.1 unless a sub-rule's number collides with something already printed |
| Weapons tab proficiency checklist ↔ everything else | **None** — hardcoded class-id `Set` membership, zero corpus/rule join | Independent, unverifiable-against-the-engine claim | Unaffected by F1 — this path does not read `sheet_rule` at all |

Files read in this pass (none edited): `apps/desktop/src/characterHub/CharacterSheet.tsx`,
`classFeaturesModel.ts`, `racialTraitsModel.ts`, `featsTabModel.ts`, `characterProgression.ts`;
`apps/desktop/src-tauri/src/character_hub.rs`, `class_feature_descriptions.rs`,
`class_feature_feat_bridge.rs`, `class_feature_pool_picker.rs`, `feat_catalog.rs`,
`rule_system_adapter.rs`; `src/rules_core/sheet_rule.rs`,
`src/rules_core/pilot_compute/class_shared_core.rs`, `src/rules_core/class_census.rs`,
`src/bin/class_census.rs`; `docs/release/SD-36-consolidation/epic-f-class-completion.md` §3b;
`docs/release/SD-36-consolidation/artifacts/epic-f/stage3/{blast-radius-receipt.md,
structural-diff-receipt.md,render_after_sheets.sh}`;
`docs/release/SD-36-consolidation/artifacts/epic-f/census-f0.json`; assorted
`data/sheet_rules/**/*.json` and `data/corpus/**/class_feature/**/*.json` records.
