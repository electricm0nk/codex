# DoD-8 — on-screen verification, SD31-E6-F10-004

`RUN_DESKTOP_AGENT=sd31equipclass4` — worktree already had `node_modules` (verified before
launch; `frontend-install` also confirmed it present during the gate). Driven directly via
`apps/desktop/.claude/skills/run-desktop/driver.sh`, never via `verify-on-screen.sh` (that
harness has no `class_feature`/equipment-family coverage for this shape and a known
`race_trait` coordinate bug — same standing exception this program's own prior equip-class
cycles record) and never concurrently with `scripts/verify.sh` (launched only after the full
gate's own process had fully exited, confirmed via `ps`/`cwd` before starting).

## Sequence

1. `driver.sh launch` → `Ready. DISPLAY=:74 WINDOW_ID=2097155` (cold build, ~3m11s —
   `/tmp/run-desktop-driver-sd31equipclass4.tauri-dev.log`).
2. `screenshot 00-hub.png` — hub screen confirmed live.
3. `click 578 929` ("Browse Equipment Catalog" link, coordinate re-used from
   `SD31-E6-F5-004`'s own recorded precedent) → `screenshot 01-catalog.png`.
4. `click 970 364` (search box) → `type "Read Languages"` → `screenshot 02-search.png`.
5. `driver.sh stop`.

## What renders, and why it proves this cycle's own work

**`01-catalog.png`** — the Equipment Catalog's own header states **"7817 items across 25
ingested books"** and names every one of them, including `BOTD2`, `HA`, `ISC`, `ISG`, `ISI`,
`ISR`, `ISWG`, `MC`, `MYTHIC`, `OA`, `UC`, `UE`, `UI`, `UM`, `UPSI`, `UW`. The per-book chip row
shows, live and machine-rendered (not hand-typed): **`ISG (125)`, `MYTHIC (252)`, `ISC (65)`,
`ISI (34)`, `BOTD2 (5)`** — byte-identical to this cycle's own `gen_equipment_gap_tables`
stdout and to `docs/release/SD-31-corpus-closure-grind/artifacts/OPEN-ISSUES.md` row 206's
figures, proving the same total (`7817`) the gate's own `equipment_resolver.rs`/
`equipment_catalog.rs` pinned-count tests assert is genuinely what a player's running app
serves, not merely what a test fixture claims.

**`02-search.png`** — searching `"Read Languages"` returns **`Legendary Intelligent Item /
Read Languages`, book `MYTHIC`, category `Equipment Mods`, `1000 gp`**, with its real
description rendering: *"This item can read script in any language regardless of its known
languages."* This is a real record from `mythic_adventures` — the book that had **zero
content of any kind** in `data/corpus/` before this cycle (`git status --porcelain
data/corpus/mythic_adventures` showed the whole directory untracked at cycle start; its
`LICENSE.json` was created fresh, not restated) — now rendering its real cost and real corpus
description on the exact screen a player uses to browse equipment. The cost (`1000 gp`) and
mythic_adventures/equipment/equipmods/legendary_item_intelligent_item_communication_
read_language.json`'s own shipped `cost_gp`/`description` fields (source citation:
`ma_equipmods.lst:113`, record key `Legendary Item ~ Intelligent Item ~ Communication /
Read Language`) — re-verified by direct read before writing this line, not assumed from the
similarly-named sibling record.

Neither screenshot is staged or synthetic data — both are the real, running desktop app
reading the real compiled `equipment_catalog_rows()` chain this cycle's own
`gen_equipment_gap_tables.rs`/`gen_cache_equipment_gap` changes produced.
