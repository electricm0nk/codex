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
operator's two `usable: yes` lines exist.

**Same-ID check** (`epic-breakdown.md` ↔ this file):

```bash
cd docs/release/SD-41-gear-and-equipment-sets
diff <(awk -F'|' '$2 ~ /^ (C|E)[0-9]/ {gsub(/ /,"",$2); print $2}' kanban.md | sort) \
     <(grep -oE '^\| (C|E)[0-9][0-9.A-Za-z]*' epic-breakdown.md | sed 's/^| //' | sort -u) && echo SAME_IDS
awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'     # 36 (23 + 8 from the seven follow-ups + 3 from the campaign item + E1.11 from the Q7 answer, decisions.md §22–§30, + E2.8 discovered by E1.8)
```

| ID | Title | Tier | Status | Depends on | Receipt / notes |
|---|---|---|---|---|---|
| C0 | Package committed + Cycle 0 receipt + census | lead | complete | — | this commit; branch cut by the planning session 2026-11-12 from `origin/tranche-20` @ `f7f85f3d24` |
| C1 | 0.21.0 on 14 surfaces + notes pointer | haiku | complete | C0 | Phase 1; this commit; receipt `artifacts/cycle_0/C1_cycle_receipt.md` |
| E1.1 | Containers settled into the record via `ir_converter.rs` + `_settled/equipment.json`; `is_container` = Container-typed and no base damage | opus | ready | C1 | Phase 1; blocker cleared 2026-11-12 (`decisions.md §31`: lead grant R1, operator ruling R2); fixture bundle byte-identical; first attempt `370e3346b4`, receipt `artifacts/epic_1/E1.1_cycle_receipt.md` |
| E1.2 | Equipment sets: model, file, projection, Default (root crate) | opus | waiting | E1.1 | Phase 1; `src/saved_character/equipment_sets.rs` (new); the round-trip theorem over seeds + `.pcg` fixtures |
| E1.3 | Container loads (recursive contents vs capacity) | opus | waiting | E1.2 | Phase 1; `src/rules_core/containers.rs` (new) |
| E1.4 | Carried weapons: `in_hand` on the breakdown | opus | waiting | E1.3 | Phase 1; `damage_total.rs` |
| E1.5 | The Unarmed row: always last, real numbers, monk progression, IUS lethal | opus | waiting | E1.4 | Phase 1; `decisions.md §22` |
| E1.6 | `uses_ammunition` + `ammunition_family` settled via `ir_converter.rs`; `ammunition_carried` per weapon | opus | waiting | E1.5 | Phase 1; `decisions.md §23, §31` |
| E1.7 | Encumbrance weighs extra copies + piece quantities; `base_quantity` settled via `ir_converter.rs` | opus | waiting | E1.6 | Phase 1; `decisions.md §25, §28, §30, §31` |
| E1.8 | Price census through the catalog: missing / zero / negative by book + category; converter causes | opus | complete | C1 | Phase 1; side lane `convert`; this commit; receipt `artifacts/epic_1/E1.8_cycle_receipt.md`; 1,763 unpriced of 5,880 Market rows (none 1,448 / zero 311 / negative 4); d20pfsrd list 346–1,763 by what E1.9 evaluates; discovered E2.8 |
| E1.9 | Converter fix + re-convert + `_settled` regen + override-list reader + price gate | opus | waiting | E1.8, E1.1 | Phase 1; lane `convert`; after E1.1 (both edit `ir_converter.rs`, `decisions.md §31` R1); corpus diff limited to price/weight/token/name fields + the settled bundles |
| E1.11 | Every Market item tried: d20pfsrd list in batches of ≤ 150 (lead's `web-fetch` agent per batch); leftover Market keys in `price-unknown.json` each with a `tried` list | opus | waiting | E1.9 | Phase 1; lane `convert`; the effort bar is a closure criterion (`decisions.md §30, §30.1`); P-9; no desktop file |
| E1.10 | Campaign mirror (root crate): `campaigns.json`, atomic folder copy per active campaign, warnings never errors | opus | waiting | E1.7 | Phase 1; `decisions.md §29` |
| E1.MC | E1 merge check + Epic report; Phase-1 run ends | opus | waiting | E1.10, E1.11 | adversarial; partition check `git diff … -- apps` → 0 (less the price test file); corpus field check; writes the resume receipt (`START=E2.1`) |
| E2.1 | Load response: `equipmentSets`, `containerLoads`, `location` + `ordinal`, stacks in pieces (`quantity`, `copies`, `baseQuantity`), custom names; Default file + bundle-quantity migration on first load; old-save test (AC, weight) | opus | waiting | E1.MC, SD-40 merged | Phase 2; first write to `character_hub.rs`; G-1 NEW_KEYS named |
| E2.2 | Commands: move (n pieces; Drop = Not Carried), set CRUD, activate; market → Carried; `set_equipment_active_state` + sell over the sets; TS boundaries | opus | waiting | E2.1 | `main.rs` registration; `CharacterSheet.tsx:3295`; `decisions.md §27, §28` |
| E2.3 | Create item backend: weight / price / container capacity on custom records; `create_custom_item`; custom items weighed + listed; Custom dialog fields | opus | waiting | E2.2 | `decisions.md §25`; old `custom.json` loads |
| E2.4 | Weapons DTO: `inHand`, `unarmed`, `nonlethal`, `usesAmmunition`, `ammunitionCarried`; tab order in hand → carried → Unarmed last; ammo boxes per ammunition weapon | opus | waiting | E2.3 | `decisions.md §9, §22, §23` |
| E2.5 | Market backend: `slot` column; buy by bundle → pieces; sell / discard n pieces; negotiated price (an unpriced row buys through Negotiate, `decisions.md §30.1`); copy names | opus | waiting | E2.4 | `decisions.md §26–§28` |
| E2.6 | Buy Custom backend: `purchase_custom_equipment` + `list_custom_equipment_options` + quote; price and weight from eqmod data | opus | waiting | E2.5 | `decisions.md §24`; Q6 |
| E2.7 | Campaign write-through hooks in every writer + scanning test; `set_character_campaigns`; `mirrorWarnings` | opus | waiting | E2.6 | `decisions.md §29` |
| E2.8 | Market hides PCGen retirement rows (`.FORGET` keys) *(discovered by E1.8)* | sonnet | waiting | E2.7 | Phase 2; `desktop/equipment_catalog.rs` `hidden_from_market` + the two pinned `character_hub.rs` lists; `decisions.md §19` |
| E2.MC | E2 merge check + Epic report | opus | waiting | E2.8 | adversarial; `.pcg` population probe (Default file + AC/weight unchanged) |
| E3.1 | Gear screen: split panel, filter, right tree, Move to menu (quantity on stacks), Drop / Sell / Discard, set picker, capacity warning; replaces EquipmentPanel | opus | waiting | E2.MC | `GearScreen.tsx` + `gearScreenModel.ts` (new); `EquipmentPanel.tsx` deleted; `decisions.md §27, §28` |
| E3.2 | Drag and drop over the same boundary; `dropTarget` pure + tested | opus | waiting | E3.1 | no dependency added (SD-g) |
| E3.4 | Market screen: Item / Price / Weight / Slot table, sortable; description box; Only Show Affordable; Quantity; Negotiate; Buy Custom builder; Create dialog | opus | waiting | E3.2 | `MarketDialog.tsx`, `marketModel.ts` (new), two dialogs (new); `decisions.md §24–§26, §28` |
| E3.5 | Campaign field → membership dialog; "Open <name>" buttons under Skills; mirror warnings shown | opus | waiting | E3.4 | `decisions.md §29`; `CharacterSheet.tsx`, `CharacterHubPage.tsx` |
| E3.3 | ui-smoke: 18 new rows, 6 rewritten; inventory regenerated | opus | waiting | E3.5 | side lane `smoke` (`.worktrees/sd41-smoke`), beside E3.MC and E4.1; lands before E4.MC; the only `spec.json` writer (R24) |
| E3.MC | E3 merge check + Epic report (7 screenshots, looked at) | opus | waiting | E3.5 | adversarial; runs beside E3.3 only on files it does not own |
| E4.1 | Print: Inventory by location (slots → Carried nested → Not Carried; stacks; Create items); Weapons in hand → carried → Unarmed last; ammo boxes per ammunition weapon, no global row | opus | waiting | E3.MC | `gear.rs`, `weapons.typ`; G-2; Looked at; `decisions.md §22, §23, §28` |
| E4.MC | E4 merge check + E4.G's PDFs (4) and screenshots (≥ 12) | opus | waiting | E4.1, E3.3 | adversarial; looks at everything it renders |
| E4.G | OPERATOR GATE: gear screen + printed inventory page, `usable: yes` ×2 | sonnet | waiting | E4.MC | the run ends here until the operator signs; `grep -c '^- \(Gear screen\|Inventory page\) — usable: yes' artifacts/epic_4/E4.G_cycle_receipt.md` → 2 |
| E5.1 | Widest verify + clean build + packaged resources + fresh-install check | sonnet | waiting | E4.G | |
| E5.2 | Final-acceptance scan → retrospective → sweep | opus | waiting | E5.1 | |
| E5.3 | Release notes → architecture truth-up → claims critic | sonnet + opus | waiting | E5.2 | |
| E5.4 | Graphify LAST → PR tranche-21 → develop (final action) | sonnet | waiting | E5.3 | |
