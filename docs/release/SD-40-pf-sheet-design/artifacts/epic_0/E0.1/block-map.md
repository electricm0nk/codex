# E0.1 block map — Codex print blocks ↔ app UI ids ↔ PCGen std sections

A picture to aim at, never a source of numbers (decisions.md §6, §18). Codex's engine is the oracle for
every value; nothing here is read by any Rust or TS test.

## Sources (each re-derivable)

| Population | Count | Command (from the repo root; `$PCGEN_REPO_DIR` per `scripts/pcgen-oracle-pin.env`) |
|---|---|---|
| Codex block builders | **16** | `grep -h '^pub fn ' apps/desktop/src-tauri/src/print/blocks/*.rs \| awk 'END{print NR}'` |
| `SheetBoxId` members | **13** | `awk '/export type SheetBoxId/,/;/' apps/desktop/src/characterHub/sheetLayout.ts \| grep -c "^  \| '"` |
| `SquareBoxId` members | **4** | `grep -n 'export type SquareBoxId' apps/desktop/src/characterHub/sheetLayout.ts` (line 100: hitPoints, initiative, encumbrance, conditions) |
| `SHEET_TABS` entries | **8** | `grep -n 'export const SHEET_TABS' apps/desktop/src/characterHub/CharacterSheet.tsx` (Weapons, Defense, Gear, Spells, Pets, Feats, Actions, Custom) |
| PCGen std `match="character"` names, page 1 body up to `protection` | **29** (as grepped; one, `basics/bab`, sits inside an XML comment) | `awk '/template match="character"/{f=1} f' "$PCGEN_REPO_DIR/outputsheets/d20/fantasy/pdf/fantasy_master_common_blocks.xslt" \| grep -oE 'call-template name="[^"]+"\|apply-templates select="[^"]+"' \| sed -E 's/.*="([^"]+)"/\1/' \| awk '{print} /^protection$/{exit}' \| awk 'END{print NR}'` |

The rendered picture: `aldric-pcgen-std.pdf` (3 pages; PNGs `aldric-pcgen-std-{1,2,3}.png`) and
`aldric-pcgen-simple.pdf` (3 pages; its page 1 differs from std only in the weapon box: one To Hit / Dam
row instead of the 1H-P/1H-O/2H/2W grid). PCGen page 1 = the first `fo:page-sequence` (xslt lines 16–100
of the `match="character"` body); page 2 = the second (equipment onward); page 3 = the bio page.

## The map

PCGen page column: **1** = rendered on PCGen std page 1 for Aldric; **2** = page 2; **3** = bio page;
**—** = PCGen std has no section (or it renders empty for Aldric). Codex page 1 today: identity, combat,
abilities, skills (`print/layout/pf1e.rs` header comment and `layout()` order).

| Codex block builder | File | Codex page 1 today | App UI id(s) (`SheetBoxId` / `SquareBoxId` / tab) | PCGen std section(s), in `match="character"` call order | PCGen page | SD-40 page 1 (R5) |
|---|---|---|---|---|---|---|
| `identity` | blocks/core.rs | yes | `portrait`, `levelXp`, `classPanel`, `characterDetails` | `basics` (name, player, deity, region, alignment, class, race, size, height/weight, vision, level, XP, age, gender, eyes, hair) | 1 | yes (header) |
| `abilities` | blocks/core.rs | yes | `abilities` | `abilities` | 1 | yes |
| `combat` (today one block; E2.1 splits it, decisions.md §8) | blocks/core.rs + layout/pf1e.rs `combat` | yes | `squareBlock` → `hitPoints`, `initiative`; `combatStrip` → `armorClass`, `attack`, `speed`, `saves` | `.` mode `hp_table` (HP, wounds, subdual, DR, speed), `armor_class`, `initiative`, `saving_throws`, `resistances` (miss chance, ASF, ACP, SR, energy resist — printed beside initiative), `attack` (conditional), `attack` (ranged_melee: MELEE / RANGED / CMB rows carrying base attack bonus, plus the CMB/CMD maneuver grid). `basics/bab` is commented out in the XSLT: BAB has no box of its own, it is a column of the attack rows | 1 | yes |
| `skills` | blocks/skills.rs | yes | `skills` | `skills` (page-1 mode), `skillinfo` (the ✓/✗ legend) | 1 | yes |
| `weapons` | blocks/gear.rs | no | tab `Weapons` | `weapons/martialarts`, `weapons/unarmed`, `weapons/naturalattack`, `weapons/spiritweaponmelee`, `weapons/spiritweaponranged`, `weapons` (per-weapon box: hand, type, size, crit, reach, To Hit / Dam grid) | 1 | yes (R5; SD-b moves it to page 2 first on a spill) |
| `armor` | blocks/gear.rs | no | tab `Defense` | `protection` (armor / shield: type, AC, max Dex, check, spell failure) | 1 | no (R5 names five blocks; armor sits after weapons on PCGen page 1) |
| `inventory` | blocks/gear.rs | no | tab `Gear`; `SquareBoxId` `encumbrance` (load level) | `encumbrance` (page 1, the load word only), `equipment`, `weight_allowance`, `money` | 1 (encumbrance) / 2 | no |
| `feats` | blocks/feats.rs | no | tab `Feats` | `special_abilities`, `traits`, `drawbacks` and the Feats list (rendered on page 2 as "Feats") | 2 | no |
| `class_features` | blocks/feats.rs | no | tab `Feats` (nearest; the map pairs by content, not by a code link) | `class_features` and the per-feature boxes (`class_features/rage`, `…/channel_energy`, `…/layonhands`, …), `special_attacks`, `special_qualities`, `racial_traits` ("Special Qualities" on Aldric's page 2) | 1 (feature boxes) / 2 | no |
| `actions` | blocks/feats.rs | no | tab `Actions` | — (PCGen std has no actions section; nearest is the conditional modifiers under `saving_throws`) | — | no |
| `spells` | blocks/spells.rs | no | tab `Spells` | `spellcasteroutputs` / the std sheet's spell pages (empty for Aldric) | — (Aldric) | no |
| `spellbook` | blocks/spells.rs | no | tab `Spells` | separate sheet `csheet_fantasy_spellbook_3x5.xsl`: one 3×5 card per spell (`elowen-pcgen-spellbook-3x5.pdf`) | separate | no |
| `custom` | blocks/extras.rs | no | tab `Custom`; nearest square: `conditions` | `conditions`, `misc/magics` (the nearest PCGen has: temporary effects and magic notes) | 2 | no |
| `pets` | blocks/extras.rs | no | tab `Pets` | `misc/companions` ("Other Companions"), `animal_tricks` | 2 | no |
| `math` | blocks/math.rs | no | none (the screen shows explanations on hover; `showProgression` is the nearest rail box) | — (PCGen prints the breakdown inline: the `= 10 + 4 + …` AC row, the save and attack columns) | — | no |
| `bio_notes` | blocks/bio_notes.rs | no | `characterDetails` (bio fields) | the bio page (name, race, age, gender, vision, alignment, hand, height, weight, eyes, hair, description) and `languages` | 3 (languages on 2) | no |

## What is page 1 (the answer the card asks for)

- **PCGen std page 1 for Aldric**, top to bottom as rendered (`aldric-pcgen-std-1.png`): `basics` header
  strip; left column `abilities`, then `saving_throws` (+ conditional modifiers), `attack` (MELEE /
  RANGED / CMB with BAB as a column) and the CMB/CMD grid, `weapons/unarmed`, the Longsword `weapons`
  box, `protection`; centre top `.` hp_table (HP, wounds, subdual, DR, speed), `armor_class` (one row with
  the `=` breakdown), `initiative` + `resistances`, `encumbrance`; right column `skills` + `skillinfo`.
  13 rendered sections: basics, abilities, hp_table, armor_class, initiative, resistances, encumbrance,
  skills, skillinfo, saving_throws, attack, weapons, protection (a reader counts them in the PNG).
- **Codex page 1 today**: `identity`, `combat`, `abilities`, `skills`.
- **SD-40 step-1 page 1 (R5)**: `identity` header + `abilities`, the split `combat` (`hitPoints`,
  `initiative`, `armorClass`, `attack`, `saves`, `speed`), `skills`, `weapons`. PCGen also puts
  `protection` (Codex `armor`) and the `encumbrance` word on page 1; R5 does not name them, so they are
  not step-1 scope (a later step may place them — picture only, no ruling implied).

## Does not cover

PCGen's numbers (Codex's engine is the oracle; e.g. PCGen's HP 28 comes from the hand-entered
per-level rolls 10/6/6 in `aldric.pcg`, not from Codex — irrelevant to the picture); colour
variants; PCGen's companion / notes sheets; the std sheet's spell pages for a caster (Aldric has none;
Elowen was rendered on the 3×5 spellbook sheet only).
