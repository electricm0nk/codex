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
awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'     # 23
```

| ID | Title | Tier | Status | Depends on | Receipt / notes |
|---|---|---|---|---|---|
| C0 | Package committed + Cycle 0 receipt + census | lead | complete | — | this commit; branch cut by the planning session 2026-11-12 from `origin/tranche-20` @ `f7f85f3d24` |
| C1 | 0.21.0 on 14 surfaces + notes pointer | haiku | ready | C0 | Phase 1 |
| E1.1 | Containers in the record: `is_container`, `ContainerCapacity` from `CONTAINS` | opus | waiting | C1 | Phase 1; root crate only |
| E1.2 | Equipment sets: model, file, projection, Default (root crate) | opus | waiting | E1.1 | Phase 1; `src/saved_character/equipment_sets.rs` (new); the round-trip theorem over seeds + `.pcg` fixtures |
| E1.3 | Container loads (recursive contents vs capacity) | opus | waiting | E1.2 | Phase 1; `src/rules_core/containers.rs` (new) |
| E1.4 | Carried weapons: `in_hand` on the breakdown | opus | waiting | E1.3 | Phase 1; `damage_total.rs` |
| E1.MC | E1 merge check + Epic report; Phase-1 run ends | opus | waiting | E1.4 | adversarial; partition check `git diff … -- apps` → 0; writes the resume receipt (`START=E2.1`) |
| E2.1 | Load response: `equipmentSets`, `containerLoads`, `location` + `ordinal`; Default file on first load; old-save test | opus | waiting | E1.MC, SD-40 merged | Phase 2; first write to `character_hub.rs`; G-1 NEW_KEYS named |
| E2.2 | Commands: move, set CRUD, activate; market → Carried; `set_equipment_active_state` + sell over the sets; TS boundaries | opus | waiting | E2.1 | `main.rs` registration; `CharacterSheet.tsx:3295` |
| E2.3 | Custom items: `base_item_id`; weight + container-ness from the base; Custom dialog picker | opus | waiting | E2.2 | `character_custom.rs`; old `custom.json` loads |
| E2.4 | Weapons DTO `inHand`; Weapons tab in hand → carried; Not Carried off | opus | waiting | E2.3 | |
| E2.MC | E2 merge check + Epic report | opus | waiting | E2.4 | adversarial; `.pcg` population probe (Default file + AC/weight unchanged) |
| E3.1 | Gear screen: split panel, filter, right tree, Move to menu, set picker, capacity warning; replaces EquipmentPanel | opus | waiting | E2.MC | `GearScreen.tsx` + `gearScreenModel.ts` (new); `EquipmentPanel.tsx` deleted |
| E3.2 | Drag and drop over the same boundary; `dropTarget` pure + tested | opus | waiting | E3.1 | no dependency added (SD-g) |
| E3.3 | ui-smoke: 7 new rows, 6 rewritten; inventory regenerated | opus | waiting | E3.2 | side lane `smoke` (`.worktrees/sd41-smoke`), beside E4.1; lands before E4.MC; the only `spec.json` writer (R24) |
| E3.MC | E3 merge check + Epic report (4 screenshots, looked at) | opus | waiting | E3.2 | adversarial; runs beside E3.3 only on files it does not own |
| E4.1 | Print: Inventory by location (slots → Carried nested → Not Carried); Weapons in hand → carried | opus | waiting | E3.MC | `gear.rs`; G-2; Looked at |
| E4.MC | E4 merge check + E4.G's PDFs (4) and screenshots (≥ 9) | opus | waiting | E4.1, E3.3 | adversarial; looks at everything it renders |
| E4.G | OPERATOR GATE: gear screen + printed inventory page, `usable: yes` ×2 | sonnet | waiting | E4.MC | the run ends here until the operator signs; `grep -c '^- \(Gear screen\|Inventory page\) — usable: yes' artifacts/epic_4/E4.G_cycle_receipt.md` → 2 |
| E5.1 | Widest verify + clean build + packaged resources + fresh-install check | sonnet | waiting | E4.G | |
| E5.2 | Final-acceptance scan → retrospective → sweep | opus | waiting | E5.1 | |
| E5.3 | Release notes → architecture truth-up → claims critic | sonnet + opus | waiting | E5.2 | |
| E5.4 | Graphify LAST → PR tranche-21 → develop (final action) | sonnet | waiting | E5.3 | |
