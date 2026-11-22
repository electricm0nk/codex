# SD-36 Epic F · Stage F6e receipt — second starter seed: Elowen Ashgrave, Human Wizard 5, Fireball prepared

**Operator ruling 2026-09-27, option 3:** keep "Aldric Ironhand" (Human Fighter 3) and add a second seed — a 5th-level Wizard, female, whose 3rd-level spell is Fireball.

## What was built

`apps/desktop/src-tauri/src/character_hub.rs`: `seed_default_character_if_needed(app)` now resolves the app-data dir and calls `seed_default_characters_at(app_data_dir, app_version)`, which walks `starter_seeds()`. Each seed is a `CreateCharacterRequest` run through **`compose_character_input`** (the composer `create_character` uses — the same create path, no hand-written character file), plus `pf1_adapter::apply_record_and_prepare_spell_selection` for extra spells (the mutation the Spells tab's first-spell path runs). A seed saves only when `build_pilot_headless_receipt` is `Computed`.

| field | Aldric (unchanged) | Elowen (new) |
|---|---|---|
| `character_id` | `00000000-0000-0000-0000-000000000001` | `00000000-0000-0000-0000-000000000002` |
| `display_label` | Aldric Ironhand | Elowen Ashgrave (fictional) |
| `race_id` / `class_id` / `level` | `race:human` / `class:fighter` / 3 | `race:human` / `class:wizard` / 5 |
| stored scores Str/Dex/Con/Int/Wis/Cha | 17/13/14/14/12/8 | **8/14/13/16/12/10** |
| `ability_bonus_target` | strength | **intelligence** (Int 16 + 2 = **18**) |
| `saved_at` | `2026-01-01T00:00:00.000Z` | `2026-01-01T00:00:00.000Z` (same convention) |
| extra spells | — | `Fireball` recorded (Known) + prepared, `class:wizard` |
| marker | `.default_character_seeded` | `.default_character_seeded_2` |

Elowen's spellbook = the canonical Wizard seeds (`class_seeds::canonical_seeds_for("wizard", 5)`: Evocation specialist, opposed Necromancy/Transmutation, `Light` Known + Prepared) **plus Fireball** Known + Prepared.

**Seeding rules.** Each seed has its own marker, so an install that already carries Aldric's marker gains Elowen on its next launch; a marker keeps a deleted seed deleted; a seed whose character directory already exists is never written (its marker is written instead) — a player's edits survive even a lost marker.

## Fireball's id — resolved, not guessed

The engine's spell id is the CRB spell-list key (`SpellSelection.spell_id`, resolved by `class_spell_levels::class_spell_level` and `resolve_prepared_spell_school`; the Add Spell picker sends `entry.key` from `list_spells`). For Fireball that key is **`"Fireball"`** (`src/rules_core/rules_tables/crb/spell_list.rs:527`, Evocation, level 3; `crb/wizard_spell_list.rs:266` `("Fireball", 3)`). It is the label of the converted record **`core_rulebook:spell:fireball`** (`data/sheet_rules/core_rulebook/spell/fireball.json`, provenance `cr_spells.lst:233`), whose `granted_by` states `ClassSpellList { id: "wizard", spell_level: 3 }`. `the_fireball_seed_id_is_the_converted_crb_record` pins all three facts (list key, class level 3, converted record label + wizard level 3).

## RED → GREEN

- RED (before the implementation): `cargo test --locked -j 8 --manifest-path apps/desktop/src-tauri/Cargo.toml starter_seed_tests --no-run` → 24 errors: `cannot find function seed_default_characters_at` ×8, `FIREBALL_SPELL_ID` ×5, `SECOND_SEED_CHARACTER_ID` ×5, `SECOND_SEED_MARKER` ×4, `FIREBALL_CONVERTED_RECORD_ID` ×1, `WIZARD_CLASS_ID_FOR_SEED` ×1. Frontend RED: `buildCharacterHubListSurface.test.ts` → `ENOENT … f6e-starter-seed-list-wire.json`.
- GREEN — `character_hub::starter_seed_tests` (7): `the_second_seed_is_a_level_5_wizard_with_fireball_prepared` (temp app-data dir → seed → load: Wizard 5, scores 8/14/13/16/12/10 + `ability:intelligence`, Fireball Known and Prepared, canonical seeds kept, the only prepared spell at Wizard level 3 is Fireball, receipt `Computed` with 0 claim-blocking diagnostics, `load_saved_character_at_root` returns a snapshot); `an_existing_install_gains_the_second_seed_without_touching_the_first` (pre-F6e install with a player-edited Aldric: Aldric byte-identical, Elowen added); `a_seed_whose_id_already_exists_is_never_overwritten` (both markers lost, both characters edited: both byte-identical); `a_deleted_seed_does_not_come_back`; `a_fresh_install_seeds_both_characters_and_both_markers`; `the_fireball_seed_id_is_the_converted_crb_record`; `the_starter_seed_list_wire_matches_the_committed_artifact` (pins `f6e-starter-seed-list-wire.json`; `CODEX_WRITE_F6E_WIRE=1` rewrites it).
- Frontend: `verifiesAFreshInstallListsBothStarterSeeds` in `buildCharacterHubListSurface.test.ts` reads that wire (`src/testSupport/starterSeedListWire.ts`) — rows `Aldric Ironhand | Human | class:fighter:3` and `Elowen Ashgrave | Human | class:wizard:5`, printed `Fighter 3` / `Wizard 5` (the Load screen's `formatHeldClasses`).

## Hand-worked Elowen 5 vs the sheet

Ability modifiers (CRB Table 1-3): Str 8 → −1, Dex 14 → +2, Con 13 → +1, Int 18 → +4, Wis 12 → +1, Cha 10 → +0. Wizard progression: ½ BAB, poor Fort/Ref, good Will, d6, 2 + Int skill ranks (CRB Wizard class table — the brief cites Table 3-16, p.79; the corpus carries no page numbers, so the page is the brief's, not re-verified here).

| total | hand-worked | engine / real app | source of the app value |
|---|---|---|---|
| BAB | ⌊5/2⌋ = **+2** | +2 | `class_chassis.base_attack_bonus` |
| base saves F/R/W | ⌊5/3⌋=1 / 1 / 2+⌊5/2⌋=4 → **1/1/4** | 1/1/4 | `baseSaves` |
| total saves F/R/W | 1+1=**2** / 1+2=**3** / 4+1=**5** | +2/+3/+5 | `totalSaves`; app screenshot |
| HP | 1st level max d6 + Con: 6+1 = 7; levels 2–5 at the sheet's average rule ⌊6/2⌋+1 = 4, +1 Con each: 4 × 5 = 20 → **27** | **27** | `characterProgression.ts maxHitPoints`; ui-smoke row expects `HIT POINTS\n27` (green) |
| skill points / level | 2 + 4 (Int) + 1 (Human) = **7** | 7 | Progression rail, ui-smoke expects `Skill points: 7` (green) |
| caster level | **5** | 5 | `class_chassis.wizard.caster_level` |
| spells/day 0/1/2/3 | base 4/3/2/1; Int +4 bonus 0/1/1/1 (Table 1-3); Evocation specialist +1 per level ≥ 1 → **4/5/4/3** | 4/5/4/3 | `class_spell.wizard.total_spells_per_day.*` |
| spell DC (3rd) | 10 + 3 + 4 = **17** | 17 | `class_chassis.wizard.spell_save_dc.spell_level_3` |
| Fireball | recorded + prepared in a 3rd-level slot (3rd-level slots: 3; 1 used) | `class_spell.wizard.spellbook_contents` "Light, Fireball"; `daily_preparation` "Light, Fireball"; Spells tab "Fireball · CRB · Evocation · Wizard level 3" (Known and Prepared rows) | screenshot |

The five requested totals (BAB, base saves, save totals, HP, Fireball in a 3rd-level slot) match.

**Loadout-derived values — the create path's fixed loadout, not a seed choice.** `compose_character_input` gives EVERY created character (every class) the same fixed loadout: feats Power Attack + Dodge (Human bonus-feat slot) + Weapon Focus (longsword); longsword + chain shirt equipped; 1 rank each in Climb/Intimidate/Swim. Elowen, built through that path as required, carries it. Hand-worked against it:

| total | hand-worked (PF1) | engine | verdict |
|---|---|---|---|
| AC / touch / flat-footed | 10 + 4 (chain shirt) + 2 Dex + 1 Dodge = 17 / 13 / 14 | 17 / 13 / 14 | match |
| melee (longsword) | +2 BAB −1 Str +1 WF −4 nonproficient weapon = −2; PF1 also applies a non-proficient armor's check penalty to attack rolls (−2) → −4 | −2 | **engine omits armor non-proficiency** — `src/rules_core/pilot_compute/class_slayer.rs:378` states no armor-nonproficiency-penalty mechanic exists |
| Climb | 1 rank −1 Str + worse of armor ACP (−2) and Medium-load ACP (−3; 29 lb carried vs Str 8 light max 26 lb) = −3 | −2 | **engine folds the armor ACP only**: `pilot_compute_corpus.rs:725-727` reads `effects.armor_check_penalty_total`, never `encumbrance.load_armor_check_penalty` (served only to the AC/Encumbrance panels) |
| Power Attack | prerequisite Str 13 — Elowen has 8 | listed as a feat | **prerequisite not enforced on the create path's fixed feat** |

## Real app (one launch per run, isolated data root per F6d)

`RUN_DESKTOP_AGENT=f6e node scripts/ui-smoke/run.mjs --only load-seed-list-both,load-seed-wizard-fireball --out <scratch>/f6e-out` → **2/2 green (M = 2 rows)**, evidence `docs/release/SD-36-consolidation/artifacts/ui-smoke/f6/seed/` (`results.json`, `run.log`, both screenshots). The isolated store held exactly the two seeds (`…0001`, `…0002`), 0 created / 0 deleted / 0 leftover, root removed at exit. Real store unchanged: `f6e-real-before.txt` = `f6e-real-after.txt` (1 character dir; sha256 of every path+size+mtime under `~/.local/share/io.electricm0nk.codex` `38446d8e…dbdda2` both times). No app process left running.

- `load-seed-list-both`: Load screen shows `Aldric Ironhand / Human Fighter 3` and `Elowen Ashgrave / Human Wizard 5`.
- `load-seed-wizard-fireball` (LAST spec row on purpose — it ends scrolled down to show the Fireball rows): Elowen's sheet, `HIT POINTS 27`, `Skill points: 7`, Spells tab `Fireball · CRB · Evocation · Wizard level 3` (Known and Prepared), spells per day Level 3 = 3.

Iterations: run 1 green but its screenshot showed HP `Unknown` and skill points `Unknown` (the class roster was still in flight — the sheet opened straight from Load, the first `ensureClassRosterLoaded` call of the session) and Fireball below the fold; the row gained an 8 s wait, a scroll and the HP / skill-point expectations; runs 2–3 green, run 3 is the receipt.

## Verification

- `cargo test --locked -j 8 --manifest-path apps/desktop/src-tauri/Cargo.toml -- --test-threads=8` → **633 passed, 0 failed** (626 before + 7 new; `f6e/f6e-desktop-test.log`).
- `cd apps/desktop && npm test` → **131/131 test files passed** (`f6e/f6e-npm-test.log`); `npm run typecheck` → exit 0 (`f6e/f6e-typecheck.log`).
- `npm run ui-smoke:doc` regenerated `docs/testing/ui-smoke-inventory.md` (83 rows, 18 screens).

## Remainder, by mechanism (owned by this batch; not closed in F6e)

1. **Ability scores print as 10 + 2 × modifier.** `CharacterSheet.tsx:470 scoreFromModifier` — every odd score prints one low: Elowen CON 13 prints **12**; Aldric STR 19 → 18, DEX 13 → 12. The served `abilityScores` (`effective_ability_scores_dto`) folds only the Human +2 (`apply_human_ability_bonus`), so it is not a drop-in fix for other races; the fix needs the engine's effective score per race.
2. **HP / skill points print `Unknown` while the class roster is loading** (the sheet opened straight from Load): a loading state printed as the Unknown verdict.
3. **Create-path fixed loadout** (Power Attack below its Str 13 prerequisite, chain shirt + longsword on a Wizard, Climb/Intimidate/Swim ranks) applies to every created character.
4. **Armor non-proficiency penalty to attack** and **Medium-load armor check penalty to skills** are not computed (files above).
5. `snapshot.spellbook` is absent for Wizard 5, so the Spells tab says "Save DCs and slot totals are not computed for this build yet" above the computed spells-per-day rows it then prints.
