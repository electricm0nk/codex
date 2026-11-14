---
canonical: true
bundle_id: SD-41
slug: gear-and-equipment-sets
canonical_branch: tranche-21
target_version: 0.21.0
date: 2026-11-12
board: local-file ./kanban.md
status: planning-ready
---

# SD-41 Kanban

One row per card; one criterion per card (`epic-breakdown.md`). The dispatched agent that closes a card
edits that card's row in its own closing commit. A row moves to `complete` only together with a receipt
and its commit SHA.

**Status vocabulary:** `ready` · `waiting` · `in-progress` · `complete` · `partial` (blocks closure) ·
`blocked-escalated` (operator ruling needed; blocks closure).

**Column order is load-bearing:** E5.2's scan reads ID as awk field `$2` and Status as `$5`.
**`Depends on` is load-bearing:** the script dispatches a card only when every dependency is `complete`.
`SD-40 merged` on E2.1 is a dependency the **lead** clears by hand (replacing it with the merge SHA after
the catch-up rebase, `decisions.md §4`, SD-m). Row order is dispatch order. **Closure rule:** no PR while
any row other than E5.4 is anything but `complete`. **One operator gate:** E4.G stays `partial` until the
operator's three `usable: yes` lines exist.

**Same-ID check** (`epic-breakdown.md` ↔ this file):

```bash
cd docs/release/SD-41-gear-and-equipment-sets
diff <(awk -F'|' '$2 ~ /^ (C|E)[0-9]/ {gsub(/ /,"",$2); print $2}' kanban.md | sort) \
     <(grep -oE '^\| (C|E)[0-9][0-9.A-Za-z]*' epic-breakdown.md | sed 's/^| //' | sort -u) && echo SAME_IDS
awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'     # 46 (E2.11 discovered by E1.11; was 45: E1.15 and E2.10 discovered by E1.9; was 43: E1.14 from the Q11 answer, decisions.md §35; was 42: E4.2–E4.4 from the page-1 request, decisions.md §34; was 39 (E1.13 and E2.9 discovered by E1.6; was 37: E1.12 from the Backpack ruling, decisions.md §32; was 36: 23 + 8 from the seven follow-ups + 3 from the campaign item + E1.11 from the Q7 answer, decisions.md §22–§30, + E2.8 discovered by E1.8)
```

| ID | Title | Tier | Status | Depends on | Receipt / notes |
|---|---|---|---|---|---|
| C0 | Package committed + Cycle 0 receipt + census | lead | complete | — | this commit; branch cut by the planning session 2026-11-12 from `origin/tranche-20` @ `f7f85f3d24` |
| C1 | 0.21.0 on 14 surfaces + notes pointer | haiku | complete | C0 | Phase 1; this commit; receipt `artifacts/cycle_0/C1_cycle_receipt.md` |
| E1.1 | Containers settled into the record via `ir_converter.rs` + `_settled/equipment.json`; `is_container` = Container-typed and no base damage | opus | complete | C1 | Phase 1; landed 2026-11-12 (130 containers settled, fixture sha unchanged; predicate ¬DAMAGE ∧ ¬Weapon, `decisions.md §20`); blocker cleared 2026-11-12 (`decisions.md §31`: lead grant R1, operator ruling R2); fixture bundle byte-identical; first attempt `370e3346b4`, receipt `artifacts/epic_1/E1.1_cycle_receipt.md` |
| E1.2 | Equipment sets: model, file, projection, Default (root crate) | opus | complete | E1.1 | Phase 1; landed 2026-11-12; theorem: 114 root fixture inputs + 63 of 63 converted `.pcg` fixtures round-trip (starter seeds moved to E2.1, `decisions.md §20`); receipt `artifacts/epic_1/E1.2_cycle_receipt.md` |
| E1.3 | Container loads (recursive contents vs capacity) | opus | complete | E1.2 | Phase 1; landed 2026-11-13; `src/rules_core/containers.rs` (new), 4 tests; backpack-in-pouch figure corrected to 10.5 (`decisions.md §20`); receipt `artifacts/epic_1/E1.3_cycle_receipt.md` |
| E1.4 | Carried weapons: `in_hand` on the breakdown; additive entry point `resolve_weapon_damage_breakdown_with_carried`; old function and all existing tests unchanged | opus | complete | E1.3 | Phase 1; `damage_total.rs` only; lead option (c), `decisions.md §33`; code `0a725d14bf`; receipt `artifacts/epic_1/E1.4_cycle_receipt.md` (declined once: `862ca3fe86`) |
| E1.5 | The Unarmed row: always last, real numbers, monk progression, IUS lethal | opus | complete | E1.4 | Phase 1; `decisions.md §22`; additive per §33 (c): last row of `resolve_weapon_damage_breakdown_with_carried` + `pilot_compute::unarmed_strike_combat_rows`; Aldric `1d3+4` (`1d3+3` corrected, §20); code `80b2ce8757`; receipt `artifacts/epic_1/E1.5_cycle_receipt.md` |
| E1.6 | `uses_ammunition` + `ammunition_family` settled via `ir_converter.rs`; `ammunition_carried` per weapon | opus | complete | E1.5 | Phase 1; `decisions.md §23, §31`; also settles `ammunition_kinds` + `base_quantity` (`decisions.md §20`); 87 `Projectile` / 228 `Ammunition` / 30 `BASEQTY` corpus records; desktop fixture restored byte-identical, its regen carded as E2.9; receipt `artifacts/epic_1/E1.6_cycle_receipt.md` |
| E1.7 | Encumbrance weighs extra copies + piece quantities; `base_quantity` settled via `ir_converter.rs` | opus | complete | E1.6 | Phase 1; `decisions.md §25, §28, §30, §31`; `base_quantity` was settled by E1.6, E1.7 adds its test; 19 `compute_encumbrance` call expressions (not 41, `decisions.md §20`); receipt `artifacts/epic_1/E1.7_cycle_receipt.md` |
| E1.8 | Price census through the catalog: missing / zero / negative by book + category; converter causes | opus | complete | C1 | Phase 1; side lane `convert`; this commit; receipt `artifacts/epic_1/E1.8_cycle_receipt.md`; 1,763 unpriced of 5,880 Market rows (none 1,448 / zero 311 / negative 4); d20pfsrd list 346–1,763 by what E1.9 evaluates; discovered E2.8 |
| E1.9 | Converter fix + re-convert + `_settled` regen + override-list reader + price gate | opus | complete | E1.8, E1.1 | Phase 1; lane `convert`; this commit; receipt `artifacts/epic_1/E1.9_cycle_receipt.md`; Market rows unpriced 1,763 → 397 (`market_price_census`); 1,433 corpus records + 1,386 Market table rows priced (`field_diff_check.py`); `price-unknown.json` 397 / `d20pfsrd-prices.json` 0; discovered E1.15, E2.10 |
| E1.11 | Every Market item tried: d20pfsrd list in batches of ≤ 150 (lead's `web-fetch` agent per batch); leftover Market keys in `price-unknown.json` each with a `tried` list | opus | complete | E1.9 | Phase 1; lane `convert`; this commit; receipt `artifacts/epic_1/E1.11_cycle_receipt.md`; d20pfsrd answered a browser user agent, so the card fetched the pages itself (no lead `web-fetch`); 397 rows in 3 batches: 41 cited (`d20pfsrd-prices.json`), 356 `price-unknown.json` each with `tried`; settler fence exit (cited list skipped `.COPY=` rows whose base is outside the oracle, `decisions.md §20`); SD-27 absent-price pin takes the cited price (fence exit); discovered E2.11 |
| E1.12 | Backpack 60 lb: cited capacity override list read by `ir_converter.rs`; `_settled` regenerated; RED test settled capacity == 60 lb | opus | complete | E1.1, E1.9 | Phase 1; lane `convert`; discovered 2026-11-12 (`decisions.md §32`); landed 2026-11-13 in the main lane (this commit); receipt `artifacts/epic_1/E1.12_cycle_receipt.md`; CRB Backpack settled `Pounds(60.0)`, 1 record / 1 field moved; fixture sha `74bb85a4…` unchanged; live-code check `live_backpack_hits.py` → 0 (`decisions.md §20`) |
| E1.13 | Inherited-copy bundles keep their `BASEQTY`: CRB `Arrows (20)`, `Bolts (Crossbow/10)`, `Bolts (Crossbow/Repeating/5)`, `Bullets (Sling/10)`, `Darts (Blowgun/10)` settle `base_quantity` *(discovered by E1.6)* | opus | waiting | E1.12 | Phase 1; lane `convert`; `decisions.md §19, §20, §28, §30`; re-ingest of `lst_inherited_copy` records (their `COPY=` line's own tokens) |
| E1.14 | Climb speed on the race layer: `DeclaredMoveModes.climb_ft` from the race-trait `MOVE` data; Vanara constant → data; Vanara 20 ft, Human none; `climb_speed` → 3 passed (`decisions.md §36`) | opus | complete | E1.3 | Phase 1; `src/` only; discovered 2026-11-13 (`decisions.md §35` Q11); the desktop `speed.climb` push is E2.4's; lands before E1.MC; landed 2026-11-13 (this commit): `climb_ft` settled (4 of 919 race-trait records), Vanara 20 / Human none / Tree Stranger none, constant removed; 2026-11-14: Skinwalker ruling (a) — the bestial-form climb prints as Change Shape text, test dropped, acceptance 4 → 3 passed (`decisions.md §36`); complete; receipt `artifacts/epic_1/E1.14_cycle_receipt.md` |
| E1.15 | The census test passes the PCGen residue gate: `tests/market_price_census.rs` reads `raw_tokens` ×2, `raw_bonus_chains` ×1, `%CHOICE` ×2 (E1.8's cause classifier), 5 live hits under `apps/desktop` *(discovered by E1.9)* | opus | waiting | E1.9 | Phase 1; lane `convert`; `bash scripts/verify.sh --only pcgen-residue-gate -j 8` PASS; before E1.MC; `decisions.md §19` |
| E1.10 | Campaign mirror (root crate): `campaigns.json`, atomic folder copy per active campaign, warnings never errors | opus | complete | E1.7 | Phase 1; `decisions.md §29`; `save` mirrors and discards, `save_with_warnings` returns them; desktop callers switch in E2.7 (`decisions.md §20`); receipt `artifacts/epic_1/E1.10_cycle_receipt.md` |
| E1.MC | E1 merge check + Epic report; Phase-1 run ends | opus | waiting | E1.10, E1.11, E1.12, E1.13, E1.14, E1.15 | adversarial; partition check `git diff … -- apps` → 0 (less the price test file); corpus field check; writes the resume receipt (`START=E2.1`) |
| E2.1 | Load response: `equipmentSets`, `containerLoads`, `location` + `ordinal`, stacks in pieces (`quantity`, `copies`, `baseQuantity`), custom names; Default file + bundle-quantity migration on first load; old-save test (AC, weight) | opus | waiting | E1.MC, SD-40 merged | Phase 2; first write to `character_hub.rs`; G-1 NEW_KEYS named |
| E2.2 | Commands: move (n pieces; Drop = Not Carried), set CRUD, activate; market → Carried; `set_equipment_active_state` + sell over the sets; TS boundaries | opus | waiting | E2.1 | `main.rs` registration; `CharacterSheet.tsx:3295`; `decisions.md §27, §28` |
| E2.3 | Create item backend: weight / price / container capacity on custom records; `create_custom_item`; custom items weighed + listed; Custom dialog fields; "Ignore container capacity limits" override (`decisions.md §32`) | opus | waiting | E2.2 | `decisions.md §25`; old `custom.json` loads |
| E2.9 | Desktop fixture `_settled/equipment.json` regenerated: 41 `Projectile` weapons gain `uses_ammunition` / `ammunition_family` *(discovered by E1.6)* | sonnet | waiting | E2.3 | Phase 2; one generated file; before E2.4; `decisions.md §19, §20, §31` |
| E2.10 | Re-pin `every_offered_modifier_row_charges_the_price_the_picker_displayed` (`character_hub.rs:14624`): the display-vs-charge divergence set is `[("UE", "Masterwork Tool")]` now that E1.9 priced CRB `Holy Symbol (Silver)` / `(Wooden)` *(discovered by E1.9)* | sonnet | waiting | E2.3 | Phase 2; one test edit; the desktop suite is red on that one test until it lands (E1.9 receipt) |
| E2.11 | The Market offers `+1`…`+5` only on what can take one: a priced named magic weapon or armor (E1.11 priced `Holy Avenger` 120,630 gp; the Market now offers `+1 Holy Avenger` 122,930 gp) offers none; `enhancement_price_table_offers_nothing_on_what_cannot_take_a_bonus` green *(discovered by E1.11)* | opus | waiting | E2.3 | Phase 2; `equipment_catalog.rs::market_variants` read "no price" as "already magic"; the desktop suite is red on that one test until it lands (E1.11 receipt); before E2.5 |
| E2.4 | Weapons DTO: `inHand`, `unarmed`, `nonlethal`, `usesAmmunition`, `ammunitionCarried`; the `speed.climb` row push (E1.14); callers `character_hub.rs:2879` + `rule_system_adapter.rs:435` → the carried entry point; stowed-sword test updated; carried rows marked; tab order in hand → carried → Unarmed last; ammo boxes per ammunition weapon | opus | waiting | E2.9 | `decisions.md §9, §22, §23, §33`; also pushes `pilot_compute::unarmed_strike_combat_rows` into the load response and names `combat.weapon_*.unarmedstrike` in G-1 `NEW_ROWS` (E1.5, `decisions.md §20`) |
| E2.5 | Market backend: `slot` column; buy by bundle → pieces; sell / discard n pieces; negotiated price (an unpriced row buys through Negotiate, `decisions.md §30.1`); copy names | opus | waiting | E2.4 | `decisions.md §26–§28` |
| E2.6 | Buy Custom backend: `purchase_custom_equipment` + `list_custom_equipment_options` + quote; price and weight from eqmod data | opus | waiting | E2.5 | `decisions.md §24`; Q6 |
| E2.7 | Campaign write-through hooks in every writer + scanning test; `set_character_campaigns`; `mirrorWarnings` | opus | waiting | E2.6 | `decisions.md §29` |
| E2.8 | Market hides PCGen retirement rows (`.FORGET` keys) *(discovered by E1.8)* | sonnet | waiting | E2.7 | Phase 2; `desktop/equipment_catalog.rs` `hidden_from_market` + the two pinned `character_hub.rs` lists; `decisions.md §19` |
| E2.MC | E2 merge check + Epic report | opus | waiting | E2.8 | adversarial; `.pcg` population probe (Default file + AC/weight unchanged) |
| E3.1 | Gear screen: split panel, filter, right tree, Move to menu (quantity on stacks), Drop / Sell / Discard, set picker, capacity warning; replaces EquipmentPanel | opus | waiting | E2.MC | `GearScreen.tsx` + `gearScreenModel.ts` (new); `EquipmentPanel.tsx` deleted; `decisions.md §27, §28` |
| E3.2 | Drag and drop over the same boundary; `dropTarget` pure + tested | opus | waiting | E3.1 | no dependency added (SD-g) |
| E3.4 | Market screen: Item / Price / Weight / Slot table, sortable; description box; Only Show Affordable; Quantity; Negotiate; Buy Custom builder; Create dialog | opus | waiting | E3.2 | `MarketDialog.tsx`, `marketModel.ts` (new), two dialogs (new); `decisions.md §24–§26, §28` |
| E3.5 | Campaign field → membership dialog; "Open <name>" buttons under Skills; mirror warnings shown | opus | waiting | E3.4 | `decisions.md §29`; `CharacterSheet.tsx`, `CharacterHubPage.tsx` |
| E3.3 | ui-smoke: 19 new rows, 6 rewritten; inventory regenerated | opus | waiting | E3.5 | side lane `smoke` (`.worktrees/sd41-smoke`), beside E3.MC and E4.1; lands before E4.MC; the only `spec.json` writer (R24) |
| E3.MC | E3 merge check + Epic report (7 screenshots, looked at) | opus | waiting | E3.5 | adversarial; runs beside E3.3 only on files it does not own |
| E4.1 | Print: Inventory by location (slots → Carried nested → Not Carried; stacks; Create items); Weapons in hand → carried → Unarmed last; ammo boxes per ammunition weapon, no global row | opus | waiting | E3.MC | `gear.rs`, `weapons.typ`; G-2; Looked at; `decisions.md §22, §23, §28` |
| E4.2 | Print page 1: section titles as filled boxes (one shared style); Conditional Bonuses ↔ Resistances swapped; Conditional Bonuses an empty box | opus | waiting | E4.1 | Phase 2; `decisions.md §34` (1)–(3); goldens; Looked at |
| E4.3 | Print page 1: Skills table with shaded alternate rows; Weapons without blank rows; Speed 2×2 with Climb (\"—\" without an engine row) | opus | waiting | E4.2 | Phase 2; `decisions.md §34` (4)–(6); Q11 |
| E4.4 | Print page 1: the stored portrait image, else the system logo fetched into git with its source cited; never the word | opus | waiting | E4.3 | Phase 2; `decisions.md §34` (7), `§35` Q10; `LEAD_WEB_FETCH` on a failed fetch |
| E4.MC | E4 merge check + E4.G's PDFs (4) and screenshots (≥ 12) | opus | waiting | E4.4, E3.3 | adversarial; looks at everything it renders |
| E4.G | OPERATOR GATE: gear screen + printed inventory page + page 1, `usable: yes` ×3 | sonnet | waiting | E4.MC | the run ends here until the operator signs; `grep -c '^- \(Gear screen\|Inventory page\|Page 1 ([^)]*)\) — usable: yes' artifacts/epic_4/E4.G_cycle_receipt.md` → 3 |
| E5.1 | Widest verify + clean build + packaged resources + fresh-install check | sonnet | waiting | E4.G | |
| E5.2 | Final-acceptance scan → retrospective → sweep | opus | waiting | E5.1 | |
| E5.3 | Release notes → architecture truth-up → claims critic | sonnet + opus | waiting | E5.2 | |
| E5.4 | Graphify LAST → PR tranche-21 → develop (final action) | sonnet | waiting | E5.3 | |
