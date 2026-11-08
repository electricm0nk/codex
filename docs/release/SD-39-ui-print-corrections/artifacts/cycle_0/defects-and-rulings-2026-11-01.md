# Defect notes — new STC release (UI + print corrections)
Started 2026-11-01. Operator dictating. No fixes yet. Grill after list is complete.

## D1 — Printing is unusable (PDF is a jumble of lines)
- Current PDF output: jumble of lines; does not resemble a character sheet; completely unusable.
- This is the 3rd attempt at printing; no progress so far. Operator wants a NEW strategy, not a patch.
- Requirements:
  1. Paper size: default A4; also offer US Letter.
  2. Core sheet template, adaptable per game system:
     - Pathfinder character -> Pathfinder character sheet
     - Starfinder character -> Starfinder character sheet
     - (by implication: Cyberpunk RED -> its own sheet)
  3. Print flow: check paper size, then choose an output option:
     - basic    = minimal set of outputs only
     - spellbook = full details on all spells
     - full     = all math showing how values were derived + character bio + campaign notes
- Open questions for grill: PDF engine choice (HTML->PDF vs native), 2-sided/page-count expectations,
  fonts/licensing, whether "basic" is one page, whether spellbook is standalone or add-on, where campaign notes live today.

## D2 — Main character view layout rearrangement
Current layout (as operator describes it):
- Main pane, row 1: a 2x2 block = Hit Points / Initiative / Encumbrance / Conditions.
- Main pane, row 2: Abilities, with Armor Class / Attack / Speed / Saving Throws stacked in a column inside the Abilities row.
- Main pane somewhere: Character Details.
- Left nav: Class, plus a historical progression section (levels taken, feats, "what next", etc).

Requested target layout:
1. Abilities moves INTO the area where Character Details is today.
2. Character Details moves to the LEFT panel, placed directly BELOW Class.
3. Left-nav historical progression content (levels, feats, what-next, etc) is REMOVED and replaced by a single
   "Show Progression" button. Clicking it opens a flyover (overlay/drawer) menu that can be dismissed.
4. Armor Class / Attack / Speed / Saving Throws move to the SAME ROW as the 2x2 (HP/Init/Encumbrance/Conditions),
   laid out as a 1x4 beside the 2x2. Operator notes the boxes have a lot of empty space, so 2x2 + 1x4 should fit.
- Open questions for grill: exact component/file names for these panels; what the freed Abilities row becomes;
  flyover = modal, side drawer, or popover; keyboard/ESC dismissal; does this apply to all game systems or PF1e only;
  minimum window width behaviour when 2x2 + 1x4 do not fit.

## D3 — Equipment: wrong items in "Add Armor"; replace Add Weapon / Add Armor / Add Gear with one Market
- Bug: "Add Armor" lists many non-armor items (ammunition, rods, weapons, etc). Category filter is wrong or missing.
- Design change: remove the separate "Add Weapon", "Add Armor", "Add Gear" affordances entirely.
- Replace with a single "Market" button. Market handles BUYING and SELLING inventory.
- Open questions for grill: does Market track gold/credits/eddies (currency per system, deduct on buy, credit on sell);
  sell price rule (full, half, custom); category tabs/filters inside Market (weapon/armor/gear/magic);
  does equipping (wear/wield) stay on the inventory panel; free "add without paying" path for GM gifts/starting kit;
  per-system currency (PF1e gp, Starfinder credits, Cyberpunk eddies).

## D4 — Market is missing magical variants of armor and weapons
- Every armor must have its magical counterparts available for purchase: e.g. Leather Armor, +1 Leather Armor,
  +2 Leather Armor, ... (through +5 in PF1e).
- Same for most weapons (+1 Longsword, +2 Longsword, ...).
- Operator saw "a lot of missing options" in the purchasing section generally.
- Open questions for grill: generate variants at runtime from the base item + enhancement table (price = base +
  bonus^2 * 1000 gp armor / 2000 gp weapon + masterwork), vs materialize them in the corpus;
  include special abilities (Flaming, Fortification, etc) or enhancement bonus only in this release;
  special materials (mithral, adamantine) too?; does the +N apply to AC / attack / damage on the sheet automatically;
  Starfinder/Cyberpunk equivalents (Starfinder has no +N enhancement; weapon fusions instead).

## D5 — Equipping mechanic (body slots, worn vs carried) is entirely missing
- Rule-set import missed the "mechanical" equipping layer. In PF1e most magic items occupy a BODY SLOT:
  helmet -> head; ring -> hands (operator's words; PF1e canonical = ring slot, max 2); belt, body, chest, eyes,
  feet, hands, head, headband, neck, shoulders, wrist, ring x2, armor, shield, wielded weapons.
- Reference implementation: PCGen source (SLOTS / equipment location data in the .lst / EquipSlot definitions).
  PCGen is converter input + oracle only (operator ruling 2026-09-07) -> slot data must be converted into Codex's own schema.
- Needed UI: in the Gear section, an "Equipment" button/panel to manage what is CARRIED vs what is WORN/WIELDED.
- Needed engine behaviour: worn armor affects Armor Class; carried (not worn) armor does NOT.
  By extension: worn magic items apply their bonuses; carried ones do not; slot conflicts (two helmets) are refused or flagged.
- Operator: "That entire mechanic is not present in the current code set."
- Open questions for grill: enforce slot limits hard (block) or soft (warn); weapons: wielded main/off hand, two-handed;
  shields; weight always counts whether worn or carried (encumbrance); Starfinder slots (armor upgrades, worn items)
  and Cyberpunk (cyberware install locations) in this release or PF1e only; does the printed sheet show worn/carried state.

## D6 — Feats: no budget enforcement, no duplicate guard
- Bug A: can keep adding feats with zero feat slots remaining. Feat budget (level-based + racial bonus + class bonus
  feats) must gate the Add action. Disable/hide Add when remaining == 0.
- Bug B: the same feat can be added repeatedly. Must refuse duplicates unless the feat is explicitly stackable
  (PF1e MULT:YES / "can be taken multiple times" e.g. Toughness is NOT, Skill Focus / Weapon Focus ARE but with a different target each time).
- Open questions for grill: do prerequisites get enforced too (BAB, ability score, other feats) or just budget+dupes;
  bonus-feat pools (Fighter bonus feats = combat feats only) tracked separately; what happens to already-saved
  characters that exceed the budget (flag, don't delete); same bugs likely exist for skills ranks / spells known / traits.

## D7 — Feat picker: "Hide unavailable" checkbox (prereqs not met)
- Add a checkbox to the Add Feat picker: "Hide feats whose prerequisites are not met."
- Implies prerequisite EVALUATION must exist/work (BAB, ability scores, prior feats, class level, race, skill ranks).
- Open questions for grill: default checked or unchecked; persist the choice; when unchecked, show unmet feats
  greyed out with the missing prereq named?; can you still ADD an unmet feat (GM override) or is it blocked;
  same checkbox wanted on spells / class features / prestige class pickers.

## D8 — Left column title: "Progression" -> "Character Details"
- Rename the left-column panel heading from "Progression" to "Character Details".
- Ties to D2: Character Details content moves into this left panel (below Class); the old progression content
  collapses into the "Show Progression" button. So the rename is the natural result of D2.

## D9 — Show Progression flyover must show WHAT was chosen at each level
- The flyover (from D2) must list, per level taken, the actual selections made, not just "1 feat granted":
  - e.g. Wizard 1: feat = Scribe Scroll (bonus), feat = Spell Focus (Evocation); spells learned = Magic Missile, Shield, ...;
    skill ranks spent = Spellcraft +1, Knowledge (arcana) +1; class abilities gained = Arcane Bond (familiar), Cantrips;
    ability score increase at 4/8/12/... = which stat.
- Covers: feats, spells learned, skills (ranks per level), abilities/class features, ability-score increases, HP rolled, etc.
- Implies the character model must RECORD the level at which each choice was made (per-level ledger), not just the
  flat current totals. Check whether the save format already keys choices by level.
- Open questions for grill: read-only history, or can you edit/undo a past-level choice from the flyover;
  what about choices made before this release (legacy saves without level tags -> show "unknown level"?);
  show "unspent" slots per level too (e.g. "1 feat unchosen at level 3").

## D10 — "Add Custom" dialog: top-row selection styling bug
- In the Add Custom dialog, the top row (category/tab chips) has display quirks:
  - click item A -> it highlights (correct);
  - click item B -> item A does NOT revert to its original style; it turns a LIGHT colour with WHITE text (unreadable).
- Likely cause: stale :focus / :focus-visible or "previously-selected" class not cleared; selected vs focused styles
  collide; or light-theme token used for the de-selected state.
- Fix: deselected items return exactly to their resting style; only one selected item at a time; focus ring distinct from selection.
- Open questions for grill: which "top row" exactly (type chips: weapon/armor/gear/feat/spell?); light or dark theme
  when observed; does the same chip component appear elsewhere (Market filters, feat picker) -> fix the shared component once.

## D11 — Custom equipment: real mechanical stats + slot assignment (both optional)
- Today: custom equipment "stats" are plain text only. The engine cannot use them.
- Needed: structured, OPTIONAL fields on a custom item:
  - bonuses to actual sheet values: attack (to-hit), damage, Armor Class (and bonus TYPE: enhancement/deflection/
    natural/armor/shield/dodge so stacking rules apply), saves, ability scores, skills, speed, initiative, HP, etc.
  - a body SLOT (head, hands, ring, neck, ...) so the item can be equipped in the right place (ties to D5).
- Both are voluntary: a custom item with no stats and no slot stays a plain text item, as today.
- Worn custom item with stats -> bonuses apply (D5 rule); carried -> no effect.
- Open questions for grill: allowed bonus targets = fixed dropdown list (recommended) vs free formula;
  weight/cost fields; custom WEAPON vs custom ARMOR vs custom wondrous item as sub-types with their own fields
  (damage dice, crit range, armor bonus, max dex, ACP); do custom items print on the sheet with their bonuses shown in "full" mode;
  should Market (D3) also list the character's custom items for sale.

## D12 — Character import must support PCGen character files (.pcg)
- Add a PCGen character import: PCGen saves characters as .pcg (line-based key:value text; also may export .xml/.html via
  output sheets). Import .pcg into a Codex character.
- Operator ruling 2026-09-07 tension: "nothing left of PCGen in LIVE code"; converter/oracle live on the tool side
  (crates/codex-ingest, desktop may dev-dep only). An in-app import that parses .pcg at RUNTIME needs a ruling:
  (a) allow a .pcg reader in the live app as a converter-at-the-boundary (parse -> Codex schema, no PCGen tokens persist), or
  (b) ship a CLI/tool-side converter (.pcg -> Codex character JSON) that the app then imports as its own format.
- Mapping needed: race, classes+levels, ability scores, feats (with targets), skills ranks, spells known/prepared,
  equipment (incl. equipped location -> D5 slots), languages, traits, bio/description, money, HP, alignment, deity.
- Open questions for grill: .pcg only or also PCGen XML output; which PCGen version(s) (6.x); fail loud on unknown
  items vs import as custom items; import creates NEW character only, or can merge; test corpus = the 29 CRB oracle
  characters already in the repo (they have .pcg files?).

# GRILL RULINGS (2026-11-01)
- R1 Print engine: Rust generates the PDF from a per-system template (Typst-as-crate proposed). Not browser print, not PDF forms.
  Current path to retire: apps/desktop/src/characterHub/printLayout.ts + window.print() in CharacterSheet.tsx:2854.
- R2 Template shape: shared page frame (header/footer/fonts/margins) + shared BLOCKS (ability table, skills, weapons,
  spells, ...); each system has a short layout file choosing/arranging blocks. PF1e, SF1e, Cyberpunk RED each get one.
- R3 Paper size: app Setting, default A4, options A4 | US Letter; print dialog shows current and allows one-off override.
  Layout blocks must flow for both sizes (no fixed-inch positioning).
- R4 Print modes: stacking checkboxes. Sheet always; + Spellbook (full spell text) + Show the math (derivations)
  + Bio & campaign notes. basic = none ticked; full = all ticked.
- R5 Output: TWO buttons. "Save PDF" (save-as dialog, default <character>-<system>.pdf, then open in system viewer)
  and "Print" (same PDF generated to a temp file, handed to the OS print path). One PDF generator, two sinks.
- R6 Print verification: golden text-extraction tests (values present, block order, page count) on seed characters
  for every system x paper size x mode combination that matters; PLUS one operator eyeball sign-off per system on a
  real printed sheet before the bundle closes (a closure gate, not optional). No pixel snapshots.
- R7 Progression flyover: left side drawer over the sheet; close via X, click-outside, Escape.
- R8 Level ledger: stamp character level on every new feat / spell / skill rank / ability increase selection.
  One-time migration for legacy saves: assign by slot order (feat slots in level order, saved order); rows marked
  "assigned on upgrade" in the flyover. Code facts: prereqs ARE enforced on add_feat_selection (SD-27); NO budget
  check, NO duplicate guard; saved selections carry saved_at but no level.
- R9 Over-budget feats on legacy saves: keep all; red "over budget by N" badge on Feats tab + flyover; block new
  adds until under budget. Never auto-delete. Bonus-feat pools (Fighter combat feats etc) count separately.
- R10 Hide-unmet checkbox: default TICKED; unticked shows unmet feats greyed with the missing prereq text, Add disabled.
  No GM-override path this release.
- R10b AMENDED: the Custom tab gains an "Overrides" group of per-character toggles that allow use WITHOUT qualification:
  (1) feats without prerequisites, (2) weapons without proficiency, (3) spells outside class list / level / ability min,
  (4) armor without proficiency, (5) equipment ignoring slot/limits. Candidate extras proposed by Claude:
  (6) ignore feat budget, (7) ignore alignment restrictions (class/deity/spell), (8) ignore encumbrance.
  Any item added under an override is TAGGED (badge in UI + marked on the printed sheet in "full" mode) so the sheet never lies silently.
- R11 Market money: keep the 3 existing price modes (cashless / buy 100% sell 50% / character build) and make Market
  honour them; refuse unaffordable buys in paid modes; currency label per system (gp / credits / eddies).
- R12 Slots: hard block ("head slot is taken by X"); item stays carried; slot-override toggle (R10b #5) allows + tags.
  Code fact: converter parses SLOTS (crates/codex-ingest/src/pcgen_import/lst_parser/equipment.rs:87) but DROPS it
  in sheet_rule/convert.rs:1936/2557. Slot = add a `slot` field to the Codex equipment schema + one converter re-run.
  Body-slot list itself comes from PCGen's equipment location data (EQSLOTS) -> convert into data/equipment_types.json or similar.
- R13 .pcg import: tool-side converter in crates/codex-ingest (.pcg -> Codex character JSON), shipped as a sidecar
  binary in the app bundle; the live app's Import offers "Codex file | PCGen file" and shells out for the latter.
  Live code never parses .pcg. Unknown items -> imported as custom items + listed in an import report, not dropped.
- R14 Magic variants: runtime generation. Market shows base item + "+1..+5" picker; price = masterwork + bonus^2 x 1000 gp
  (armor/shield) or x 2000 gp (weapon); enhancement bonus type feeds AC / attack / damage when worn/wielded.
  Special abilities (Flaming...) and special materials (mithral...) deferred to a later bundle (capability deferral, not blocker).
- R15 System scope: PF1e gets everything. SF1e + CPR get: new print engine + their layout file, main-view layout
  rearrangement (where the panels exist), Market shell (buy/sell with their currency), Add Custom chip fix.
  PF1e-only this bundle: body slots, magic variants, feat budget/dup/prereq-hide, progression ledger + flyover,
  custom-item stats/slots, .pcg import. SF/CPR slot rules = later bundle (capability deferral).
- R16 Epic order (operator chose UI first, print LAST):
  A feat budget / duplicate guard / hide-unmet checkbox + Add Custom chip styling fix
  B main-view layout rearrangement + left panel rename + Show Progression drawer shell
  C Market (replaces Add Weapon/Armor/Gear) + price modes + magic variant picker
  D body slots (schema + converter re-run) + worn/carried engine rule + Equipment panel + custom-item stats/slots
  E level ledger + legacy migration + flyover content
  F overrides group on Custom tab (tagging on UI + sheet)
  G .pcg tool-side converter + Import button
  H print engine (Rust/Typst, templates PF1e/SF1e/CPR, A4/Letter setting, stacking modes, Save PDF + Print, golden tests)
  I closure (operator eyeball sign-off on printed sheets per system is a gate here)
- Claude's layout calls (unchallenged): 1x4 wraps under the 2x2 below a breakpoint; "next level grants" text moves into the drawer.
