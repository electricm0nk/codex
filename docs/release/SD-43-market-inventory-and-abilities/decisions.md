---
canonical: true
owner: operator
bundle_id: SD-43
slug: market-inventory-and-abilities
canonical_branch: tranche-43
target_version: 0.43.0
date: 2026-11-22
status: planning — front half
---

# SD-43 Decisions

Each entry is **OPERATOR** (a ruling, quoted with its date from `../SD-43-intake/intake.md`), **DEFAULT** (the
author's call; every one is listed in §30 with the command or person that settles it) or **STANDING** (a rule
carried from earlier bundles or memory). `§31` (corrections to the intake from repo evidence) wins where this
file and the intake disagree on a fact; an operator ruling always wins over a DEFAULT.

## §1 — Package location (DEFAULT)

`docs/release/SD-43-market-inventory-and-abilities/`, the house path every sibling uses
(`ls -d docs/release/SD-4*` lists `SD-40-…` to `SD-43-intake`). `docs/release/SD-43-intake/` stays untouched as
the cited source.

## §2 — Branch, board, version (OPERATOR item 23, 2026-11-22; STANDING on the one-commit rule)

- **Branch** `tranche-43`, cut from `origin/develop` @ `fb94a85049` (`git merge-base HEAD origin/develop` →
  `fb94a85049c18fd22b776a28c51eb527ec0ddef3`). Renamed from `tranche-23` (operator: *"yes, SD-43; rename and
  bump"*, 2026-11-22). Not on origin yet (`git branch -r | grep -c tranche-43` → 0); the first push is C1's.
- **Board** `./kanban.md` (local file, as SD-42).
- **Version** `0.43.0`: the minor equals the SD number from now on (operator: *"jump our minor version to
  0.43… it's getting hard to remember to translate the numbers"*). Landed in `147a3f71eb` on 14 files
  (`git show --name-only --format= 147a3f71eb | awk 'NF' | awk 'END{print NR}'` → 14: 3 version files, the
  `codex-desktop` `Cargo.lock` entry, the publish stamp, 9 test files). The Cargo.lock check the intake left
  open is settled: `awk '/^name = "codex-desktop"/{f=1} f&&/^version/{print;exit}' apps/desktop/src-tauri/Cargo.lock`
  → `version = "0.43.0"`.
- **Not yet done: `docs/release/current-release.json`** still reads `{"tranche": 22, "notes_path":
  "docs/release/SD-42-…/release-notes.md"}`, and `python3 tools/release/resolve_release_notes.py --repo-root .`
  fails today (exit 2: *"docs/release/current-release.json is for tranche 22 but apps/desktop/package.json is
  0.43.0"*). `check-release-manifest.yml`, `release-tooling-tests.yml` and `publish-tester-release.yml` call it
  (`grep -ln resolve_release_notes .github/workflows/*.yml`), so the first PR would go red. **C1** writes an
  interim `release-notes.md` (sentinel `SD-43 INTERIM NOTES`) and points the pointer at it with
  `"tranche": 43`, in one commit. The pointer's `tranche` field now carries the minor (43), which is what
  `resolve_release_notes.py:47` compares against `package.json`.
- Published build: `0.43.<run>` (`publish-tester-release.yml:129`).
- The updater treats `0.43.x` as newer than `0.22.x`: `apps/desktop/src/update/eligibility.ts:61`
  `compareVersions` splits on `.` and compares `Number(...)` per segment (read, not run). The signed manifest
  carries the version as a string; no prefix rule was found (`grep -n "0\.2" schemas/update/update-manifest.schema.json`
  is the check E7.1 runs).

## §3 — Operator rulings on record (verbatim from `../SD-43-intake/intake.md`)

| Ref | Ruling (operator; date) | Carried by |
|---|---|---|
| 0 | Cleanup order after SD-42: collapse worktrees, update the site, MOVE the site to `electricm0nk/codex-web`, update graphify "no `--force`, no `graphify update`" (2026-11-18) | §4; C2 |
| 1-obs | "a location change carries the existing content to the new folder and creates the standard skeleton there" (2026-11-18) | §7; E6.1 |
| 2-a | Rest text per race and class; "plain text. No calculations"; player override "in the custom rules menu"; placement "the printout, under the bio page" (2026-11-18) | §8; E2.4 |
| 2-b | "also show a copy of the same rest and recovery text in the app, on the character screen's Biography tab, under the biography text. Same text, same order as the printed sheet… not a second copy of the rules" (2026-11-22) | §8; E2.5 |
| 3 | House Rules: a new top-level menu; per character; new characters all off; game-system specific, none for Cyberpunk RED; six rules: slot limit off, HP at level-up (seven methods), ignore feat prerequisites, ignore weapon proficiency, ignore armour proficiency, Ambidextrous (2026-11-19) | §11; E6.2–E6.4 |
| 4 | Export to local file or campaign folder; import asks the source; same name = edit, not a second copy (2026-11-19) | §21; E6.5 |
| 5-a | "Use ONE column, a list view. The action buttons… sit on the SAME LINE as the item name" (2026-11-21) | §5; E1.2 |
| 5-b | Row order: non-containers A–Z, then containers A–Z with nesting, repeated at every depth; indent kept; light boxes kept, nested boxes inside the parent's (2026-11-21) | §5; E1.2 |
| 5-c | Not Carried: its own section; items in it "do NOT count toward encumbrance" (2026-11-21) | §5; E1.2 |
| 5-d | Naming: "the same name on both panels. The sack with 2 items is just 'Sack', not 'Sack #1'. A renamed copy shows its custom name on both panels" (2026-11-21) | §5; E1.1 |
| 5-e | "a container that holds items is NEVER part of a stack… No '#N' numbering… A stack is only identical EMPTY copies, shown as 'Name xN'" (2026-11-21) | §5; E1.1 |
| 6 | Several slots → FIRST FREE slot, else rule 2 on the first `WORN_SLOTS` slot it fits; full slot → SWAP, old item to Carried; stack → ONE per double-click; Enter on a focused row = double-click (2026-11-21) | §6; E1.4 |
| 7 | "the whole container box is a drop target. Dropping on top of an item already inside the container puts the dragged item into that container" (2026-11-21) | §6; E1.3 |
| 8 | "do NOT add an entry to the tab list (`SHEET_TABS` stays as it is)"; Market button top-left on every panel, twice the size, accent colour; remove Print from the panels ("Print belongs under the menu"); remove the pricing control from Gear (2026-11-21) | §9; E1.7 |
| 9 | Sell → small Market sell window: Market Rate / Build / Negotiate, default amount shown, typed value for Negotiate, Sell / Cancel; "Cashless is left out of the sell window"; "The window opens on Market Rate"; name "Market Rate", "Normal" must not appear; these rulings apply ONLY to the sell window, `PRICE_MODE_OPTIONS` is not edited (2026-11-21) | §10; E1.6 |
| 10 | "show the stats in that box. Same for Armor and Gear. Where an item has stats, list them" (2026-11-21) | §13; E1.9 |
| 11 | "A filter column, not chips"; "Remove the 200 cap… windowed list"; Weapons filter on kind AND damage type, a two-type weapon under both, Ammunition a filter value; an `Other` value holds every row that fits no other value, every row reachable through `All` (2026-11-21) | §14; E0.2, E1.10, E1.11 |
| 12 | Inventory panel shows each item's sell value under the Market's pricing option, between the description and Negotiate; selling a container moves its DIRECT contents to Carried, a nested container keeps its own (2026-11-21) | §10, §12; E1.5, E1.8 |
| 13 | `POST https://campaign-codex.org/api/report`, header `x-codex-key`, body `{type,title,body,version}`, `title`/`body` non-empty, `body` under 8 KB, `version` from the running build; handle 201/403/400/413/no network/timeout/other; success says "sent for review", never "issue created"; key never in the repo (2026-11-21) | §23; E5.1 |
| 14 | Channel selector moves into the top block beside `Channel:`; text blocks get more room (2026-11-22) | §25; E5.3 |
| 15 | Keep both paths; GitHub direct stays preferred; an **Anonymous** selector on the bug and enhancement screens picks the API; a **Handle** field, optional, "in the report `body` as the first line. No API change"; GitHub unavailable → a message, "No silent fallback" (2026-11-22) | §23; E5.2 |
| 16 | "option 2. Slim the screen. Keep: build, platform, backend version + commit, update status, and the Feedback evidence panel. Remove: Feature check, Data source, Backend feature check, Diagnostics, Blocked calculations, Explanations, Sources, fallback notice, Notes. The loader stays" (2026-11-22) | §25; E5.4 |
| 17 | Top bar fixed; "the left rail… and the right column… also stay fixed. Only the central pane scrolls. The page itself does not scroll"; "same behavior on the Starfinder and Cyberpunk sheets" (2026-11-22) | §24; E2.1, E2.2 |
| 18 | The **Open campaign** button on the character screen fills its space; "The Campaign Manager screen keeps its banner, but smaller"; "A character in several campaigns gets one full-width button per campaign, stacked" (2026-11-22) | §24; E2.3 |
| 19 | Corpus Ingest Diagnostic moves to the Developer tab; built in the same cycle as 16 (2026-11-22) | §25; E5.4 |
| 20-a | "I propose names from the book text; the operator approves the list before it ships" (2026-11-22, read from the one-word reply "1"; the intake asks to confirm) | §22; E4.1, E4.G |
| 20-b | "the 'no placeholder in a display name' test covers all item kinds, one test" (2026-11-22) | §22; E4.1 |
| 21 | "the layout must fit the real window height, not an assumed one. The button shows in full"; do 17 and 21 in one cycle (2026-11-22) | §24; E2.1 |
| 22-a | "It must be in both the UI and the printout"; a new **Abilities** tab next to **Feats** (working name); a real picker per pool saved on the character; core favored class bonus first, race alternates second; generic, registry-driven, not one lane per class (2026-11-22) | §15–§20; E3 |
| 22b-1 | "all seven sub-tabs in SD-43" as a first working version; "Plan the work so SD-43 delivers all seven (what can be selected, what is selected, saved on the character, on the printout), and SD-44 is a feedback round, not unfinished scope. Where the engine does not yet hold a sub-tab's data, SD-43 still ships the tab with that gap shown on screen, and the gap is listed in the plan. A gap is never hidden and never faked. No stub shipped as done." (2026-11-22) | §18; E3 |
| 22b-2 | "the Class Features selections are missing from our UI and must be added. The UI must track what can be selected and what is selected, for each sub-tab"; pools sit under Class Features (2026-11-22) | §15; E3.4 |
| 22b-3 | "grants and qualification are enforced, as feats are today": track what is granted at each level and from which source; refuse a pick not granted; refuse a pick not qualified for, and say why; show remaining picks per pool; "the engine owns the budget and the prerequisite check, the UI renders them. The UI never re-derives a grant or a prerequisite"; "Engine gaps are SD-43 work, not a reason to ship an unchecked picker" (2026-11-22) | §16; E3 |
| 23 | Minor = SD number; branch renamed `tranche-43`; version `0.43.0` (2026-11-22) | §2; C1 |

## §4 — Item 0, post-SD-42 cleanup: what is left (DEFAULT on the card)

Verified 2026-11-22: `git worktree list` prints one line (the main checkout); `ls site` → no such directory;
`f4c8d4a3cc` records the move to codex-web; codex-web's newest commits are 2026-11-21
(`gh api 'repos/electricm0nk/codex-web/commits?per_page=3' --jq '.[].sha[0:10]'` → `161c245581`, `f81c42ff54`,
`c218b09f68`). Not evidenced: step 4, graphify after SD-42 — the newest dated graph directory is
`graphify-out/2026-11-17` (`ls graphify-out`). **C2** runs the incremental graphify the operator named (no
`--force`, no `graphify update`; memory "graphify --force replaces the semantic graph": a node-count guard exit
1 means file the receipt and stop). Closure still runs graphify last (§26).

## §5 — Gear list and containers (OPERATOR 5-a…5-e; DEFAULT on the order of work)

- **Reproduce first (E0.4).** Brother Tomas Vane is a shipped starter seed (`grep -rli "tomas vane"
  apps/desktop/src-tauri/src/character_hub.rs apps/desktop/src/testSupport/starterSeedListWire.ts` → both), so
  the reproduction runs on a fresh seed in an isolated app-data root, not on the operator's data. E0.4 records
  per symptom: the steps, the stored `equipment_sets.json` rows, and whether it is a display bug or a data bug.
- **Read from the code (not run):** `gearScreenModel.ts:324–329` `copyTitle` appends `#N` when two container
  rows share a name; `:312` and `:342–343` bind each container row's move target and table to
  `row.held.copies[0]` (*"identical empty containers are interchangeable (§27.5)"*). A stack row of N sacks is
  therefore one container target bound to its first copy — consistent with the operator's hypothesis. E1.1
  proves or refutes it with a failing test before changing it.
- **E1.1 (data and naming) lands before E1.2 (layout).** The intake warns the one-column change may not fix the
  data; the model fix is tested on its own first.
- **Sort tie-break (DEFAULT):** natural order — digit runs compare as numbers, case-insensitive
  (`Intl.Collator(undefined, {numeric: true, sensitivity: 'base'})`, a platform feature; no new dependency).
- **Into a stacked empty container (DEFAULT, intake's reading):** the copy splits off the stack first, then
  holds the item. **A container emptied again (DEFAULT):** stays a separate row; no re-merge.
- **Not Carried:** E1.2 asserts the screen's carried weight equals the engine's
  `encumbrance.totalCarriedWeightLbs` (`CharacterSheet.tsx:1632` passes it) with a Not Carried item present.
- Context to read before changing rows: SD-42 `decisions.md §27.3` (rename), `§27.4` (detail panel), `§27.5`
  (stacks), SD-41 `decisions.md §38` (container tables).

## §6 — Double-click / Enter and the whole-box drop target (OPERATOR 6, 7)

- **Double-click** reuses the existing Move-to write (`GearScreen.tsx` `onMove`, `:73` region); no second write
  path. No handler exists today (`grep -n -i -E "onDoubleClick|dblclick" apps/desktop/src/characterHub/GearScreen.tsx`
  → nothing). Slot choice lives in `gearScreenModel.ts` as a pure function with tests (first free slot; else
  first `WORN_SLOTS` slot it fits, swap). A slot-less item: nothing moves and the row's status line says why
  (DEFAULT). The handler ignores events whose target is inside a `button`. Enter on a focused row calls the same
  function. Tie-break "first slot in `WORN_SLOTS` order" is the intake's wording of the operator's "swap"
  (DEFAULT, §30 Q-6.1).
- **Drop target:** today `dropZone(...)` is attached only to a container's own row (`GearScreen.tsx:397`,
  `:414`); item rows inside carry none, so a drop bubbles to the section zone (`:543`). E1.3 makes the whole box
  (header, empty space, every child row) target the NEAREST container; `stopPropagation` (`:366`, `:372`) stays;
  `dropTarget` (`gearScreenModel.ts:363`) keeps refusing a container into itself or its contents, proved by a
  test on the widened area. Lands on E1.2's layout (intake: "Land this fix on the new layout").

## §7 — Campaign folder relocation (OPERATOR 1-obs; DEFAULT on the three open questions)

Code to start from: `apps/desktop/src/campaign/campaignFolderModel.ts`, `campaignMigrationModel.ts`,
`EditCampaignScreen.tsx`; backend `apps/desktop/src-tauri/src/campaign_folder.rs` (`require_absolute` `:152`,
`open_campaign_folder_at` `:219`). DEFAULTS (§30 Q-1.1–Q-1.3): **copy, verify, switch** — every file copied, each
file's bytes compared, then the known-folders entry points at the new path; **the old folder is kept** and the
screen says where it is (no data destroyed by a path change); a **non-empty target is refused** unless it holds a
`campaign.json` with the same campaign id (then it is opened as is, nothing copied — the synced-copy case). The
skeleton is written by the same function SD-42's create path uses, `create_campaign_folder_at`
(`campaign_folder.rs:200`; `grep -n "fn create" apps/desktop/src-tauri/src/campaign_folder.rs`); E6.1 reads it
before reuse.

## §8 — Rest and recovery text (OPERATOR 2-a, 2-b; DEFAULT on the source)

- **Source (DEFAULT, §30 Q-2.1).** The corpus has no per-race/per-class rest record: a phrase search finds 23
  records across kinds (`grep -rlis "hours of rest\|night's rest\|8 hours of sleep\|eight hours of rest" data/corpus --include=*.json | awk -F/ '{print $4}' | sort | uniq -c`
  → class_feature 14, equipment 2, ability 2, spell 1, race_trait_generic 1, monster_ability 1, feat 1,
  companion 1), and the Core Rulebook's rest rules (natural healing, spell preparation) are chapter prose, not
  records. E0.5 extends this census (every class's spell-preparation and resource-refresh rule, every race with
  a rest-related trait) and E2.4 builds one committed table — one row per class/race rule, each row's text taken
  from the book with its book and page cited, no number computed. The operator reviews it in SD-44's feedback
  round; E2.4's receipt lists every row's citation.
- **One source, two surfaces:** the print block (new `print/blocks/rest.rs`, placed after `bio_notes.rs`'s
  block on the bio page) and the Biography tab (`CharacterSheet.tsx` `FreeTextTab` at `:620`, rendered at
  `:4532`) read the same engine command. The Biography text area is one field saved on blur; the rest text
  renders **outside** that field, read-only, and is never written into `bio.biography` (E2.5 test).
- **Override:** "in the custom rules menu" — the per-character Custom record (`custom.json`, §11) gains one
  optional free-text field; when set it replaces the table text on both surfaces (DEFAULT §30 Q-2.2: whole-text
  replace, not per-line).

## §9 — Market button, Print and the pricing control (OPERATOR 8; DEFAULT on Weapons and SF/CPR)

- Today (`grep -n "onOpenMarket\|PriceModeControl\|props.onPrint" apps/desktop/src/characterHub/CharacterSheet.tsx`):
  Weapons `PriceModeControl` `:849`, Market `:851`, Print `:854`; Defense Market `:1396`; Gear `PriceModeControl`
  `:1615`, Market `:1617`. All use `addItemButtonStyle` (shared with Add Spell and others).
- **One `MarketButton` component** used by all three panels; its size and colour apply to it only.
- **Print already has a menu route:** `CharacterSheet.tsx:4153` `{ label: 'Print', onSelect: handlePrint }` in
  the sheet's ☰ menu (§31 C-3). Removing the panel Print button leaves a way in; E1.7 asserts the menu entry
  still opens `PrintDialog`.
- **Weapons pricing control (DEFAULT, §30 Q-8.1):** removed too, so pricing lives only in the Market (the
  intake's pick).
- **Starfinder and Cyberpunk RED:** their Market and Print are buttons in the sheet's own bar
  (`StarfinderCharacterSheet.tsx:82`, `:85`; `CyberpunkCharacterSheet.tsx:168`, `:171`), and neither imports
  `AppMenu` (`grep -n AppMenu` on both → nothing). DEFAULT (§30 Q-8.2): they use the shared `MarketButton`
  top-left; their Print moves into a ☰ menu only if E2.2 gives them one (the shared layout of item 17);
  otherwise Print stays where it is, since removing it would leave no way in (no-stub rule).
- The sizes ("twice today's") are stated as numbers in E1.7's receipt.

## §10 — Sell window and sell values (OPERATOR 9, 12-ask-1)

- Mapping (intake, confirmed by reading `priceMode.ts:5` `PriceMode = 'cashless' | 'standard' | 'characterBuild'`):
  Market Rate = `standard` (50 %), Build = `characterBuild` (100 %), Negotiate = the typed gp total the Sell
  write already accepts (`onSell(row, quantity, negotiatedGp)`).
- The window's choice is its own; it never writes the session `priceMode` (`CharacterSheet.tsx:2890`). It opens
  on Market Rate. The intake's consequence reading (§30 Q-9.1, DEFAULT yes): the window returns money per the
  option chosen even when the Market is set to Cashless.
- **One value function, two screens:** the sell window's default amount and the Market inventory's value
  column both call `sellNegotiateGp` (`marketModel.ts:115`), which mirrors `refund_copper`
  (`character_hub.rs:4098`). A row with no catalog price shows `—` and Sell still removes it. Cashless in the
  Market inventory column: `—` (DEFAULT §30 Q-12.2). Format: `formatMarketPrice` (`itemPickerFilter.ts`).
- "the pricing above" is removed (`GearScreen.tsx:31`, `:188`, `:205`; `:572`'s doc comment).

## §11 — House Rules (OPERATOR 3; DEFAULT on storage and the open questions)

- **Where custom options live today (the intake's "to verify"):** per character, in `custom.json` beside the
  character (`character_custom.rs:1–2`, `CUSTOM_FILE_NAME` `:40`), struct `Overrides` (`:131`) with nine flags.
  The install-wide `apps/desktop/src/settings/` store holds UI preferences in localStorage (theme, print paper,
  feat picker's hide-unmet) and no override (`grep -rn "Overrides\|ignore_slot_limits" apps/desktop/src/settings`
  → nothing). The per-character rule holds.
- **Four of the six rules already exist as flags:** slot limit off = `ignore_slot_limits`; ignore feat
  prerequisites = `feats_without_prerequisites`; ignore weapon proficiency = `weapons_without_proficiency`;
  ignore armour proficiency = `armor_without_proficiency`. New: **HP at level-up method** and **Ambidextrous**.
  DEFAULT: the House Rules menu is a new top-level ☰ entry that edits the same `Overrides` (no second store);
  the Custom dialog stops showing the four flags so one setting has one home (§30 Q-3.4).
- New characters: every flag off (already the `Default` derive). Game-system filter: CPR shows none; Starfinder's
  list is settled by E0.5 (which rules exist in SF: SF hit points are fixed per level, so the HP methods likely do
  not apply — to verify).
- **HP methods (DEFAULTS, §30 Q-3.1–Q-3.3):** level 1 stays maximum (the rules give it; the house rule applies
  from level 2); Average rounds down, "Average, round up" rounds up; Percentage rounds down with a floor of 1; a
  rolled or entered result is stored once at level-up in the hit-point record and never re-rolled when the sheet
  is regenerated. "Random" rolls are the one place SD-43 rolls a die; the value feeds the HP total, so it is a
  sheet number, not a simulation.
- **Ambidextrous:** removes the off-hand penalty in the engine's two-weapon attack rows; the UI only reads.

## §12 — Selling a container moves its contents (OPERATOR 12-ask-2; DEFAULT on the open questions)

- Not found today: no code moves a sold container's contents (`sell_held_pieces_at_root` `character_hub.rs:5635`,
  `discard_equipment_copy_at_root` `:5754`; read, not run). E1.5 writes the RED test first: sell a Backpack
  holding a Sack holding a Bedroll, plus loose items → Backpack gone and paid; loose items and the Sack in
  Carried; Bedroll still in the Sack; carried weight correct. Acceptance also covers: empty container, nested,
  inside a Bag of Holding, worn (belt pouch on Belt) and its encumbrance after the sale.
- One backend for both screens (Market inventory and Gear Sell call the same command).
- DEFAULTS (§30 Q-12.1, Q-12.3): Drop and Discard of a container are unchanged in SD-43 (item 12 says they are
  not part of it); contents of a sold container that were Not Carried stay Not Carried.

## §13 — Stats in the Market's description box (OPERATOR 10; DEFAULT on order and missing values)

- Today `MarketEntryDto` (`apps/desktop/src/boundary/listMarket.ts`) carries no stat field and the box prints
  `selected.description ?? selected.detail` (`MarketDialog.tsx` ~305).
- The engine states, the UI formats (twin rule, `../SD-27-future-state-book-content-ingestion/decisions.md
  §29.1`). E1.9 first reads whole corpus records per category (weapons, armor, shields, gear, magic) and decides
  reuse of `describe_held_equipment_at_root` (`character_hub.rs:7126`, a HELD copy) versus a new
  `describe_market_entry` command over a catalog row; preference: one renderer shared with `GearDetail.tsx`.
  A new command means `main.rs` registration, a boundary file, and the DTO — all in FILES YOU OWN.
- Stats first, then prose; a missing stat reads `—` (DEFAULTS §30 Q-10.1, Q-10.2). `+N` variants follow the
  chosen `+N`. The box scrolls at the dialog's smallest size. Print untouched.
- Proof width (intake, kept): acceptance samples every category and every `TYPE` shape and counts the `—` per
  stat, so a silent gap shows as a number.

## §14 — Market filters, the 200 cap, the counts (OPERATOR 11)

- **Counts first (E0.2).** A committed Rust test walks `built_catalog()` (`equipment_catalog.rs:834`) and prints
  per Market list the count shown and, for every dropped row, its reason: duplicate key (`market_from_catalog`
  `:807`), `hidden_from_market` (`:736`: `.FORGET`, `(Base)` template with no price, `Codex-Named Unit`), category
  `Equipmods` (`market_category_of` `:709`), no tags, book not ingested. Acceptance: shown + dropped-with-reason =
  source total, per list; the sum equals `built_catalog().entries.len()`; a row dropped with no reason is a
  defect. It must reproduce the operator's screen numbers (Magic 3,680; Gear 1,624; Weapons 480 — read off the
  screen by the operator, not derived by us) and give Armor and Shields exact values. If it cannot reproduce
  them, the test is wrong, not the screen.
- The intake's estimate source `data/equipment_types.json` (10,674 names, `python3 -c "import
  json;print(len(json.load(open('data/equipment_types.json'))['types']))"`) is a different set from the Market's
  source; the gap between the two is explained by E0.2, not assumed to be a loss.
- **Filter values** live in one model file with tests (`marketFilters.ts`, new), from each row's `types`
  (already on `EquipmentCatalogEntryDto`); Gear reuses `equipmentCategories.ts`. Weapons: Kind (Melee, Ranged,
  Thrown, Ammunition), Damage (Bludgeoning, Piercing, Slashing), Skill (Simple, Martial, Exotic), Hands (Light,
  One-handed, Two-handed); Armor: weight class, Suit/Bracer; Shields: Light, Heavy, Tower, Buckler; Magic: Kind and
  Slot. These values are the intake's proposal; E1.10 confirms each against the tag counts and records them.
  Per list and per column, value counts plus `Other` account for every row (a two-tag row counts twice; the
  rule is stated in the test).
- **Cap removal:** `MARKET_ROW_LIMIT = 200` (`itemPickerFilter.ts:362`) and the notice (`MarketDialog.tsx:248`)
  go; a windowed list draws only on-screen rows. Keyboard selection, the description box, sort, Only Show
  Affordable and the `+N` choice keep working; time to first paint on the largest list is measured on this
  machine and stated as such.

## §15 — The Abilities tab: placement and frame (OPERATOR 22-a, 22b-1, 22b-2; DEFAULT on the tab list)

- The frame is PCGen's seven sub-tabs as the operator sees them: Feats, Racial Abilities, Traits, Archetype,
  Background, Class Features, Misc (22b). Pools (ninja tricks, rogue talents, rage powers…) sit under Class
  Features. The category table in 22a is background, not the frame.
- **One tab, not two (DEFAULT, §30 Q-22.3).** Ruling 22-a said "a new Abilities tab next to Feats (working
  name)"; ruling 22b makes Feats the first of the seven sub-tabs. Two feat pickers would drift, so: the `Feats`
  entry in `SHEET_TABS` (`CharacterSheet.tsx:579`) becomes `Abilities`; its first sub-tab is today's `FeatsTab`
  (`:2343`) unchanged in behaviour; the Traits section (`TraitsSection` `:2516`, today inside `FeatsTab`) moves to
  the Traits sub-tab; `RacialTraitsSection` (`:1689`) and the two pool reference lists move to their sub-tabs.
  Ruling 8's "`SHEET_TABS` stays as it is" was about not adding a Market tab; this is a rename of an existing
  entry, flagged for the operator anyway.
- What each sub-tab has today (read from code, not run): Feats — picker, `feats.budget.*` badge, overrides;
  Traits — add/remove (`TraitsSection`), no budget; Racial Abilities — a read-only list on the sheet, alternates
  pickable only at creation (`raceCatalog/alternateTraitPickerModel.ts`, `CreateCharacterForm.tsx`); Archetype —
  no UI (`grep -rli archetype apps/desktop/src --include=*.ts --include=*.tsx` → nothing) but the engine records a
  selection as a `SelectedChoice` with `ARCHETYPE_CHOICE_ID` (`src/rules_core/archetype_resolver.rs`); Background
  — none found; Class Features — read-only reference lists for two pools (`POOL_REFERENCE_SECTIONS` `:1798`) and
  unmatched class feature descriptions (`ClassFeatureDescriptionReferenceSection` `:1914`), nothing recorded;
  Misc — unknown. E0.1 settles Background and Misc against PCGen's own UI strings at the pin.

## §16 — The engine owns grants and prerequisites (OPERATOR 22b-3; DEFAULT on the module shape)

- **The model to copy (read):** `src/rules_core/feat_budget.rs` — pools from held rules, `FeatPool {id,
  granted, spent, …}`, `explanation_rows()` emitting `feats.budget.<pool>.{granted,spent,remaining}` (`:139`),
  `refusal()` returning the reason a pick is refused (`:159`); the desktop calls `feat_budget_for`
  (`character_hub.rs:951`) and ships rows via `feat_budget_rows` (`:1019`); the UI reads them in
  `readFeatBudget` (`characterProgression.ts:228`) into `FeatBudgetView` (`:215`) and `FeatsTab` follows
  `canAdd`/`addBlockedReason`.
- **SD-43 builds the same shape once for the other sub-tabs:** a rules_core module emitting
  `abilities.budget.<subtab>.<pool>.{granted,spent,remaining}` rows with each pool's grant sources per level,
  and a `refusal(subtab, pool, record)` that returns the budget reason or the unmet prerequisites by name. One
  generic Tauri command family (list options with `qualified` + reason, add, remove), registered in `main.rs`.
  The UI never computes a grant or a prerequisite.
- **What exists today for each (grep, 2026-11-22):** no budget outside feats
  (`grep -rln "traits\.budget\|trait_budget\|pool_budget" src apps/desktop/src-tauri/src --include=*.rs` →
  nothing); feat prerequisites in `src/rules_core/feat_prereqs/` (`converted_gate.rs`, `general.rs`, …); pool
  members carry `PRE*` tokens in their raw tokens (`class_feature_pool_catalog.rs:128` mentions `PREABILITY`)
  but no evaluator for them is known — E0.1 measures it. These are engine gaps, so they are cards (E3.2–E3.7).
- **Pool sizes per level:** how a class grants "a rogue talent at 2nd level and every 2 levels" in our engine is
  not yet known; E0.1 answers with a command before E3.4 is sized.

## §17 — Overrides on every sub-tab (DEFAULT, the intake's pick)

The GM's allowance uses the existing mechanism: `Overrides` in `custom.json` (§11) and the
`AdmittedRecordDto`/`OverrideTag` flags (`CharacterSheet.tsx:2351`, `:2438`). Two new flags,
`abilities_without_prerequisites` and `ignore_ability_budget`, mirror `feats_without_prerequisites` and
`ignore_feat_budget`; an admitted pick shows its flag. (§30 Q-22.4.)

## §18 — Engine gaps versus data gaps (OPERATOR 22b-1, 22b-3)

- **Engine gap:** records exist but the engine has no budget, prerequisite check or saved field → a card in E3,
  cleared before closure. Never an on-screen notice at closure.
- **Data gap:** the corpus holds nothing to select for a sub-tab or pool (for example if Background has no
  Pathfinder records) → the sub-tab ships with a sentence naming what is missing, and the gap is listed in this
  package (E0.1 writes the list into `artifacts/epic_0/`; the back half's `forward-scope-register.md` cites it).
  E3.MC counts the notices the built app renders and compares with E0.1's list.

## §19 — Favored class bonus (OPERATOR 22-a: core first, race alternates second)

No native UI today (`grep -rli "favored" apps/desktop/src --include=*.ts --include=*.tsx` → only
`CharacterSheet.tsx`, a comment about a Cleric's favored weapon at `:3858`). The PCGen import carries it as
custom grants (`crates/codex-ingest/src/pcg_import/map.rs`, `favored_hit_points`, `favored_skill_ranks`). SD-43:
the player picks the favored class and, per level, +1 HP or +1 skill rank (core), and the race's alternate
options (second); the HP and skill totals follow in the engine (they feed sheet totals, so they are computed;
an alternate option whose effect is not a sheet total is printed as text). DEFAULT placement: a section of the
Class Features sub-tab (§30 Q-22.7). An imported character's existing grants are not double-counted (E3.8
test).

## §20 — The Abilities printout (OPERATOR 22-a)

The chosen abilities per sub-tab and the favored class bonus print as text under the same headings, after the
feats block (`apps/desktop/src-tauri/src/print/blocks/feats.rs`; registry `print_pf_blocks.rs`). Rule text only;
no number beyond those already on the sheet.

## §21 — Custom items export and import (OPERATOR 4; DEFAULT on the open questions)

Custom equipment lives in each character's `custom.json` `equipment` list (`character_custom.rs:99`
`CustomRecordDto`). DEFAULTS (§30 Q-4.1–Q-4.3): one file holds many items (a JSON array with the game system
named); the campaign folder location is `Custom Items/` at the campaign root, added to the skeleton; an import
whose game system differs from the character's is refused with a message. Same name → the import edits that
item (operator).

## §22 — Item names (OPERATOR 20-a, 20-b)

- Scan re-derived 2026-11-22 (equipment only, `data.name`, `re.search(r'[\[\]%]|\bNAME\b', name)` over
  `glob('data/corpus/*/equipment/*.json')`): **3,805** records, **111** hits, **18** distinct name strings; by book
  Advanced Player's Guide 60, Ultimate Psionics 50, Ultimate Combat 1 — matches the intake. Widened to every
  kind (`glob('data/corpus/*/*/**/*.json', recursive=True)`, field `data.name`, kind = 4th path part): **51,474**
  records, **201** hits (equipment 111, class_feature 72, ability 16, skill 1, feat 1). Not covered by either scan:
  `label`/`OUTPUTNAME` fields, `_settled` and `rules_tables` copies, the desktop bundle copy
  (`apps/desktop/src-tauri/resources/corpus_bundle/`), other placeholder shapes. E0.3 covers those.
- Fix shape: the display name is derived in ONE place every screen reads (corpus files are generated; a hand
  edit would be undone). One test fails on any display name of any item kind containing `[`, `]` or `NAME`
  (20-b).
- **Operator wait E4.G (20-a):** E4.1 proposes every replacement name from the book text, in a list in
  `artifacts/epic_4/`; the operator approves it; E4.2 ships the approved list. The intake asks to confirm the
  reading of the one-word reply "1" (§30 Q-20.1); E4.G's question carries that confirmation.

## §23 — Reports through the codex-web API; the Anonymous selector (OPERATOR 13, 15; DEFAULT on the key)

- Today: `apps/desktop/src/feedback/` (`browserHandoff.ts`, `submissionState.ts`, `controlledDefectPayload.ts`)
  and the composers under `apps/desktop/src/testerWorkbench/feedback/{bug,enhancement,evidence}`.
- **Key (DEFAULT, §30 Q-13.2):** a GitHub Actions secret (name proposed: `CODEX_REPORT_KEY`) stamped into the
  release build at build time, the same pattern as the `UPDATE_SIGNING_KEY` secret; the operator creates the
  secret (the key lives at the operator's private path, never in the repo, logs or receipts). A build without
  the key (dev builds, a PR build) disables Send under Anonymous with a stated reason — never a fake success.
  Risk recorded (intake): a key in a shipped app can be extracted; codex-web's rate limits are the real
  protection.
- **Where the request is made:** from Rust (a Tauri command), so the key never enters the web view's JS
  (DEFAULT). The desktop crate already depends on an HTTP client: `ureq` 2 with TLS
  (`grep -n "reqwest\|ureq" apps/desktop/src-tauri/Cargo.toml` → `22:ureq = …`); no new dependency.
- 8 KB limit counted in bytes before sending. Diagnostics (DEFAULT §30 Q-13.3): build version, OS, install kind,
  channel — appended to `body` within the limit. Handle is `body`'s first line (operator).
- Success text: "sent for review". Never "issue created". GitHub unavailable with Anonymous off → a message
  telling the user to turn Anonymous on (operator: no silent fallback).

## §24 — Sheet layout, Show Progression, the campaign button (OPERATOR 17, 18, 21)

- **Layout (E2.1 PF, E2.2 SF/CPR):** the sheet wrapper today escapes page padding with
  `marginLeft: 'calc(50% - 50vw)', marginTop: '-3rem', width: '100vw'` (`CharacterSheet.tsx:4553`) and the bar
  is a plain `div` with `borderBottom: '2px solid var(--color-accent)'` (`:4558`). The new layout is one shared
  piece (a grid sized to `100dvh`: bar row, then three columns; only the centre scrolls; each side column
  scrolls inside itself when taller than the window — DEFAULT §30 Q-17.1) used by all three sheets. Opaque bar
  background; dialogs and the progression drawer stay above it; the App's fixed ☰ does not overlap.
- **Show Progression** (`progressionButtonRef`, `:2756`, `:4370`) sits inside the fixed column with the bottom
  padding it needs; reproduced at a window ~40 px shorter than full screen and at the app's smallest size.
  `tauri.conf.json:17–18` sets 1920 × 1200 (read); no fixed pixel height in the new layout.
- **Campaign button (E2.3):** `CharacterSheet.tsx:4425` calls `props.onOpenCampaign(membership.campaignId)` — a
  UUID — while the handler (`CharacterHubPage.tsx` `onLoad={(campaignDir) => …}` region `:198–214`) and
  `open_campaign_folder` (`campaign_folder.rs:546`, `require_absolute` `:152`) need an absolute folder path.
  Fix: pass the membership's folder path (or resolve id → path in one place); RED test clicks the button with a
  real membership row. Why tests missed it: E2.3's receipt states which test exercised the handler and with
  what argument. One full-width `ActionBanner` per campaign, stacked; the Manager's `OpenCampaignBanner`
  (`CampaignManagerScreen.tsx:52`) stays, smaller.

## §25 — Update menu, Developer tab, and the signed-manifest brief (OPERATOR 14, 16, 19; brief placement)

- **Update menu (E5.3):** `apps/desktop/src/update/ChannelSelector.tsx`, `CheckPanel.tsx`, `installedPanel.tsx`,
  `lastCheckPanel.tsx`, `Ui.tsx`. Test case: a long release note at the narrowest window wraps at word
  boundaries.
- **Developer tab (E5.4, one card for 16 + 19):** body inline in `App.tsx` (~893–1045; Notes at `:1033`);
  `SettingsModal.tsx:12` tab id `developer`. Keep and remove per ruling 16. The `surface` loader stays (the bug
  and enhancement composers read it). Corpus Ingest Diagnostic (`CorpusIngestDiagnosticPanel.tsx`) embeds as a
  section without its Close (DEFAULT §30 Q-19.1); the landing entry (`LandingScreen.tsx:274`, `:376–386`) and the
  `corpusIngestDiagnostic` mode (`CharacterHubPage.tsx:52`, `:136`, `:181`) are removed with their tests. The
  backend note `"Real GE-08 authoring workbench snapshot…"` (`authoring_workbench.rs:480`) goes if no other
  reader remains (grep in the card).
- **Signed update manifest brief:** the dispatch brief for this package called it "a build inside SD-43, no
  version bump". The brief itself says the opposite (operator correction 2026-11-21: *"NOT inside SD-43, NOT a
  new SD, and NOT a new tranche"*) and it is merged (`git merge-base --is-ancestor 2225c0111c HEAD` → true;
  `apps/desktop/src-tauri/src/update/signature.rs` exists). SD-43 has no card for it (§31 C-5). E7.1 re-checks
  that the publish still signs on `0.43.<run>`.

## §26 — Closure order and merge (STANDING; skill "Merge and watch", operator 2026-11-20; memory)

All cards `complete` (a filed blocker does not satisfy the scan) → retrospective
`docs/retro/sd43-market-inventory-and-abilities-retrospective.md` written from `scripts/retro.py summary` and
cited from `references/README.md` → full worktree/branch sweep with counts → release notes (final) and
`current-release.json` check → architecture truth-up → **graphify last** (no `--force`) → **PR
`tranche-43 → develop` last**. The lead merges when the PR's checks are green and watches the `develop` publish
(`publish-tester-release.yml`) to green; a red publish is a blocker. No operator gate before `develop`; the
operator gates `develop → test → main`.

## §27 — Paper sheet; print the rule (STANDING, memory 2026-09-07)

Only values that feed a sheet total are computed (favored class HP and skill ranks, House Rules HP, Ambidextrous
attack rows, sell values that are money). Every chosen ability, rest rule and pool member prints as text. No
simulation engines, no consumer-delta proofs.

## §28 — PCGen is a reference and a test oracle, never live code (STANDING)

The pinned oracle (`scripts/pcgen-oracle-pin.env`, `PCGEN_ORACLE_SHA=7f818006e3…`) is read through
`$PCGEN_REPO_DIR` / `$PCGEN_CORPUS_ROOT` (bootstrapped by `scripts/fetch-pcgen-oracle.sh`), never by a literal
path, and nothing is copied from it.

## §29 — Concurrency, tiers, discipline (STANDING, `AGENTS.md`)

One writer per tree; every agent its own worktree and its own `CARGO_TARGET_DIR` outside `/tmp`, deleted at the
end; no `git stash`; at most 3 concurrent cargo lanes (memory: the Proxmox host stops the VM on guest OOM); TDD
(failing test first, quoted in the receipt); every figure with its command; every dispatched `agent()` sets
`model` explicitly — Opus 5.5 default, Sonnet 5.5 for mechanical cards, Haiku 4.5 for housekeeping (user's
global tiering, 2026-09-29); retro events via `scripts/retro.py` as they happen.

## §30 — Open questions, each with its default and what settles it

The run takes the default and logs it; the operator may overrule any before or during the run. "Settled by" is a
command when the repo can settle it, else the operator.

| ID | Question (source) | Default taken | Settled by |
|---|---|---|---|
| Q-1.1 | Move or copy on relocation? (item 1) | copy, verify bytes, switch; old folder kept | operator |
| Q-1.2 | New folder already holds content? (item 1) | refuse unless its `campaign.json` has the same campaign id; then open it, copy nothing | operator |
| Q-1.3 | Keep or remove the old folder? (item 1) | keep; the screen names it | operator |
| Q-2.1 | Source of the rest text (item 2) | a committed table, one cited row per class/race rule, text from the book; E0.5 census first | E0.5 census command (`grep -rlis … data/corpus` widened per class) + operator review in SD-44 |
| Q-2.2 | Override shape (item 2) | one free-text field in `custom.json` replacing the whole text | operator |
| Q-3.1 | House-rule HP at level 1? (item 3) | no; level 1 stays maximum | operator |
| Q-3.2 | Rounding for Average and Percentage (item 3) | down; "round up" is its own option; Percentage floor 1 | operator |
| Q-3.3 | Rolled result kept or re-rolled on regeneration? (item 3) | kept, stored at level-up | operator |
| Q-3.4 | The four existing flags: also in the Custom dialog? | no; House Rules is their one home | operator |
| Q-3.5 | Which House Rules show for Starfinder (item 3) | those whose rule exists in SF, listed by E0.5 | E0.5 (`grep` of the SF engine for slot limits, proficiency, two-weapon, HP-per-level) |
| Q-4.1 | One file per item or many? (item 4) | one file, many items | operator |
| Q-4.2 | Where in the campaign folder? (item 4) | `Custom Items/` at the root, added to the skeleton | operator |
| Q-4.3 | Cross-system import refused? (item 4) | yes, with a message | operator |
| Q-5.1 | Item into a stacked empty container (item 5) | the copy splits off first | E1.1 test |
| Q-5.2 | Emptied container re-merges? (item 5) | no | operator |
| Q-6.1 | No free slot: first `WORN_SLOTS` slot it fits? (item 6) | yes | operator |
| Q-8.1 | Weapons pricing control removed too? (item 8 (b)) | yes | operator |
| Q-8.2 | SF/CPR Print button placement (item 8) | moves into a ☰ menu only if E2.2's shared layout gives them one; else stays | E2.2 (`grep -n AppMenu` on both sheets after E2.2) |
| Q-9.1 | Sell window returns money when the Market is Cashless? (item 9) | yes; its choice is its own | operator |
| Q-10.1 | Stats order (item 10) | stats first, then prose | operator |
| Q-10.2 | Missing stat hidden or `—`? (item 10) | `—` | operator |
| Q-11.1 | App counts for Weapons, Armor, Shields (item 11) | the operator gave Weapons 480; Armor and Shields "under 200"; E0.2's test produces exact values | E0.2 test output |
| Q-12.1 | Drop / Discard of a container move its contents too? (item 12) | unchanged in SD-43 (item 12 excludes them) — the intake's pick was yes for Drop; if the operator says yes it becomes a card | operator |
| Q-12.2 | Cashless value in the Market inventory (item 12) | `—` | operator |
| Q-12.3 | Not Carried contents of a sold container (item 12) | stay Not Carried | operator |
| Q-13.2 | Key into release builds; dev builds (item 13) | CI secret `CODEX_REPORT_KEY` stamped at build; dev builds disable Send with a reason. **Precondition: the operator creates the secret** before E5.1 can be proved in a release build | operator |
| Q-13.3 | Diagnostics attached? (item 13) | version, OS, install kind, channel | operator |
| Q-17.1 | Side columns scroll on their own when taller than the window? (item 17) | yes | operator |
| Q-19.1 | Embed the diagnostic or open it from a button? (item 19) | embed as a section | operator |
| Q-20.1 | Confirm the reading of "1" (item 20) | operator proposes-and-approves as written | operator, at E4.G |
| Q-22.1 | "SD23"/"SD24" read as SD-43/SD-44 (item 22) | yes — item 23 and ruling 22b-1 settle it | settled |
| Q-22.2 | PCGen sub-tab → data category mapping; what Background and Misc hold | E0.1 reads PCGen's UI strings at the pin; the operator's screen is the authority; a screenshot in `../SD-43-intake/` would settle it (optional, item 22b (2)) | E0.1 (`grep -rn "Racial Abilities\|Background\|Misc" "$PCGEN_REPO_DIR/code/src/java/pcgen/gui2" --include=*.java --include=*.properties`) |
| Q-22.3 | One Abilities tab replacing Feats, or a tab beside it? | one tab, Feats as sub-tab 1 | operator |
| Q-22.4 | Overrides on the six new sub-tabs? (22b) | yes, two new flags | operator |
| Q-22.5 | Retraining and replace-a-pick: excluded as the strawman said? (22) | excluded (scope-draft §5); advanced/master tricks still selectable as members | operator |
| Q-22.6 | Abilities tab on SF and CPR sheets? | no; Pathfinder only | operator |
| Q-22.7 | Where the favored class section sits | Class Features sub-tab | operator |

## §31 — Corrections to the intake from repo evidence (2026-11-22, tree `e451225461`)

| ID | Intake says | Repo shows | Command |
|---|---|---|---|
| C-1 | Item 22: "Only two pools are registered: Rogue Talent and Rage Power" | The backend catalog no longer filters by that list: `REGISTERED_POOL_GROUPS` is "kept only as documentation… no longer consulted by `load_pool_catalog`'s filter" and `is_registered_pool_group` accepts any `" ~ "` key. The two-pool limit is the UI's `POOL_REFERENCE_SECTIONS` table. Population: 1,913 distinct group names, 16,348 group-qualified records, 7,328 text-only (catalog-servable) | `sed -n 178,211p src/rules_core/class_feature_pool_catalog.rs`; `grep -n POOL_REFERENCE_SECTIONS apps/desktop/src/characterHub/CharacterSheet.tsx`; `python3 scripts/census_class_feature_pool_population.py` |
| C-2 | Item 22: "Ninja Trick … the corpus holds it (`data/corpus/ultimate_campaign/ability/ninja_trick.json`)" | That file is the retraining record (`key` "Retraining ~ Class Feature ~ Ninja Trick"). The pool is `data/corpus/ultimate_combat/class_feature/ninja_trick/`; 45 records carry group "Ninja Trick" (Rogue Talent 130, Rage Power 170, Discovery 129, Magus Arcana 57, Arcanist Exploit 46, Slayer Talent 46, Mercy 15 by the same loop) | `python3 -c "import json;print(json.load(open('data/corpus/ultimate_campaign/ability/ninja_trick.json'))['data']['key'])"`; the group loop: `for f in glob('data/corpus/*/class_feature/**/*.json', recursive=True)`, count `key.split(' ~ ')[0]` where `' ~ ' in key` |
| C-3 | Item 8: "no menu entry that opens the print dialog for a character" | The Pathfinder sheet's ☰ menu has Print: `{ label: 'Print', onSelect: handlePrint }` | `grep -n "label: 'Print'" apps/desktop/src/characterHub/CharacterSheet.tsx` → `4153` |
| C-4 | Item 23: "SD-43 is built on `tranche-23`"; Cargo.lock "could not run" | Branch is `tranche-43`; the lock reads `0.43.0` | `git branch --show-current`; the `awk` in §2 |
| C-5 | Dispatch brief: the signed-manifest work is "a build inside SD-43" | The brief places itself outside SD-43 and is merged | §25 |
| C-6 | Item 3: "to verify where existing custom options are stored" | Per character, `custom.json` `Overrides`; four of the six house rules already exist as flags | §11 |
| C-7 | Item 20: "18 distinct patterns" | The loop's 18 is distinct name strings; same figure | §22 |
| C-8 | Item 22a: `ABILITYCATEGORY` count 2,616 lines / 2,430 names | Not re-derived here: `$PCGEN_REPO_DIR` is unset in this shell and the rule forbids the literal path; E0.1 re-runs it through `scripts/fetch-pcgen-oracle.sh --print-env` | E0.1 |
