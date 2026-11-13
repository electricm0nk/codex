---
canonical: true
owner: operator
bundle_id: SD-41
slug: gear-and-equipment-sets
canonical_branch: tranche-21
target_version: 0.21.0
date: 2026-11-12
status: planning-ready
---

# SD-41 Acceptance and Verification

Gates G-1…G-8 map the Definition of Done (`scope-draft.md §2`) to commands. Root-crate filters run from
the repo root; desktop filters from `apps/desktop/src-tauri` (a binary crate: every test is a
`#[cfg(test)]` module or a `tests/*.rs` integration test). **A filter that matches 0 tests is a FAIL, not
a pass** (R20). Every closure gate is also a fenced command a reader can copy without unescaping.

## G-1 PF seed renders: no existing value moves (DoD 2, 5; `decisions.md §6, §8`) — every Phase-2 card touching the load response; E1.4; re-checked at every `*.MC` and E5.1

Base = the C1 commit (Phase 1) or the catch-up-rebase commit (Phase 2), built in its own worktree with its
own `CARGO_TARGET_DIR` (R-X). Head = the card's tree. The load response **gains** the named keys and
nothing else: the check drops exactly those, then requires the rest byte-equal.

```bash
cd apps/desktop/src-tauri && mkdir -p <dir>/base <dir>/head
PF_SEED_RENDER_OUT=<dir>/head cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8 --nocapture
python3 - <dir>/base <dir>/head <<'PY'                 # pass = prints SAME_EXCEPT_NAMED for each seed
import json, sys, pathlib
NEW_KEYS = {'equipmentSets', 'containerLoads', 'location', 'ordinal', 'quantity', 'copies', 'baseQuantity', 'inHand', 'unarmed', 'nonlethal', 'usesAmmunition', 'ammunitionCarried', 'slot', 'mirrorWarnings', 'campaigns'}   # E2.1, E2.4, E2.5, E2.7 (the card names any other)
NEW_ROWS = ('combat.weapon_attack_bonus.unarmedstrike', 'combat.weapon_damage_total.unarmedstrike', 'combat.weapon_damage_bonus.unarmedstrike', 'combat.weapon_threat_range_low.unarmedstrike')   # E1.5 / E2.4: the Unarmed row every seed gains; carried-weapon rows exist only for copies no seed holds (E1.4 states the population)
def strip(node):
    if isinstance(node, dict):
        return {k: strip(v) for k, v in node.items() if k not in NEW_KEYS}
    if isinstance(node, list):
        return [strip(v) for v in node if not (isinstance(v, dict) and str(v.get('id', '')).startswith(NEW_ROWS))]
    return node
base, head = map(pathlib.Path, sys.argv[1:3])
seeds = sorted(p.name for p in base.glob('*.json'))
if not seeds: sys.exit('NO_BASE_SEEDS')
for name in seeds:
    same = strip(json.load(open(base / name))) == strip(json.load(open(head / name)))
    print(name, 'SAME_EXCEPT_NAMED' if same else 'VALUE_MOVED')
PY
```

A `VALUE_MOVED` line is a defect; a new key or row id a card adds must be added to `NEW_KEYS` / `NEW_ROWS`
in the card's receipt with its reason. **The old-save test** (E2.1, `tests/old_save_loads_unchanged.rs`)
is G-1's second leg: AC and weight literals recorded from the base.

## G-2 SF1e and CPR golden output unchanged — E4.1, E4.MC, E5.1

```bash
cd apps/desktop/src-tauri && mkdir -p <dir>/base <dir>/head
PRINT_GOLDEN_OUT=<dir>/head cargo test --locked -j 8 print_golden -- --test-threads=8
for f in <dir>/head/sf1e-*.pdf <dir>/head/cpr-*.pdf; do
  b=<dir>/base/$(basename "$f"); test -f "$b" || { echo "MISSING_BASE $b"; exit 2; }
  diff <(pdftotext -layout "$b" -) <(pdftotext -layout "$f" -) > /dev/null || echo "TEXT_DIFF $(basename "$f")"
  [ "$(pdfinfo "$b" | awk '/Pages/{print $2}')" = "$(pdfinfo "$f" | awk '/Pages/{print $2}')" ] || echo "PAGES_DIFF $(basename "$f")"
done
echo G2_DONE                                             # pass = only G2_DONE printed
```

PF golden page counts may move at E4.1 and are listed before → after in the receipt (no ceiling).

## G-3 The partition held (DoD, `decisions.md §4`) — E1.MC

```bash
git diff --name-only <C1 sha>..HEAD -- apps | grep -v '^apps/desktop/src-tauri/tests/market_price' | awk 'END{print NR}'   # 0
git diff --name-only <C1 sha>..HEAD -- src crates | awk 'END{print NR}'   # > 0
git diff --name-only <C1 sha>..HEAD -- data | grep -v '/equipment/' | grep -v '_settled/equipment.json' | grep -v price_overrides | awk 'END{print NR}'   # 0 (E1.1/E1.6/E1.7/E1.9 touched only equipment records, the settled bundles and the override lists)
sha256sum apps/desktop/src-tauri/resources/corpus_fixtures/_settled/equipment.json   # equal to the base's unless E1.9 names a wrong fixture price
```

## G-4 Locations, projection and the Default set (DoD 1, 2, 4, 5) — E1.2, E2.1, E2.2, E2.MC

```bash
cargo test --locked -j 8 --lib equipment_sets -- --test-threads=8                                   # ≥ 8 passed
cd crates/codex-ingest && cargo test --locked -j 8 --test equipment_sets_pcg_population -- --test-threads=8   # 1 passed; receipt quotes "N of N fixtures round-trip"
cd apps/desktop/src-tauri && cargo test --locked -j 8 --test old_save_loads_unchanged -- --test-threads=8     # 1 passed
cd apps/desktop/src-tauri && cargo test --locked -j 8 move_equipment -- --test-threads=8                      # ≥ 6 passed
```

## G-5 Containers and created items (DoD 3, 8, 17) — E1.1, E1.3, E1.7, E2.3, E3.1

```bash
cargo test --locked -j 8 --lib container_ -- --test-threads=8        # 4 passed; counts 232 files / 265 rows, 210 / 88 / 73 / 49 pinned (decisions.md §31 R3)
cd crates/codex-ingest && cargo test --locked -j 8 container_of -- --test-threads=8   # 2 passed
cargo test --locked -j 8 --lib container_capacity_backpack -- --test-threads=8   # 1 passed (60 lb, decisions.md §32)
cd apps/desktop/src-tauri && cargo test --locked -j 8 custom_overrides_ignore_container_capacity -- --test-threads=8   # 1 passed
cargo test --locked -j 8 --lib containers_ -- --test-threads=8       # 4 passed
cargo test --locked -j 8 --lib extra_weighed -- --test-threads=8     # 2 passed
cd apps/desktop/src-tauri && cargo test --locked -j 8 create_custom_item -- --test-threads=8   # 4 passed
cd apps/desktop && npx tsx src/characterHub/gearScreenModel.test.ts   # the over-capacity and nesting cases among ≥ 8
```

## G-6 The gear screen and the smoke rows (DoD 9, 10) — E3.1–E3.3, E3.MC

```bash
cd apps/desktop && npm run typecheck && npm test && npm run ui-smoke:doc && git status --porcelain ../../docs/testing/ui-smoke-inventory.md   # empty
test ! -f apps/desktop/src/characterHub/EquipmentPanel.tsx && echo PANEL_GONE
grep -nE '\b(reduce\(|\+= *[a-z]*[wW]eight|lbs *[-+*/])' apps/desktop/src/characterHub/GearScreen.tsx apps/desktop/src/characterHub/gearScreenModel.ts | awk 'END{print NR}'   # 0 (NR-2)
python3 -c "import json,sys;r=json.load(open(sys.argv[1]));print(sum(x['status']=='green' for x in r),[x['id'] for x in r if x['status']!='green'])" docs/release/SD-41-gear-and-equipment-sets/artifacts/epic_3/E3.3/results.json   # 25 []
cd apps/desktop && npx tsx src/characterHub/marketModel.test.ts && npx tsx src/characterHub/weaponsTabModel.test.ts   # E3.4, E2.4
```

## G-9 Weapons: order, Unarmed, ammunition (DoD 7, 14, 15) — E1.5, E1.6, E2.4, E4.1

```bash
cargo test --locked -j 8 --lib unarmed_row -- --test-threads=8                       # 4 passed
cargo test --locked -j 8 --lib uses_ammunition -- --test-threads=8                   # 3 passed
cd apps/desktop/src-tauri && cargo test --locked -j 8 weapons_in_hand_first -- --test-threads=8   # 3 passed
grep -c '"Ammunition"' apps/desktop/src-tauri/resources/print/looks/weapons.typ      # 0 (no global row)
```

## G-11 Campaign write-through (DoD 21) — E1.10, E2.7, E3.5

```bash
cargo test --locked -j 8 --lib campaign_mirror -- --test-threads=8                                   # 5 passed (two alike names → two folders; byte-identical; inactive writes nothing; missing dir warns, local intact)
cd apps/desktop/src-tauri && cargo test --locked -j 8 --test every_character_root_writer_mirrors -- --test-threads=8   # 1 passed (every writer calls after_write)
cd apps/desktop/src-tauri && cargo test --locked -j 8 campaign_write_through -- --test-threads=8     # 3 passed
cd apps/desktop && npx tsx src/characterHub/campaignMembershipModel.test.ts                          # ≥ 4 cases
```

## G-10 Prices and the Market (DoD 16, 18–21) — E1.8, E1.9, E1.11, E2.5, E2.6, E3.4

```bash
(cd apps/desktop/src-tauri && cargo test --locked -j 8 --test market_price_census market_price -- --test-threads=8)   # 3 passed (census, the listed gate, market_price_unknown_rows_name_every_source_tried at E1.11+)
python3 -I -c "import json;print(len(json.load(open('data/price_overrides/d20pfsrd-prices.json'))),len(json.load(open('data/price_overrides/price-unknown.json'))))"   # both figures quoted with E1.8's denominators
ls docs/release/SD-41-gear-and-equipment-sets/artifacts/epic_1/E1.11/batch-*.json | awk 'END{print NR}'   # ⌈unpriced Market rows / 150⌉
cd apps/desktop/src-tauri && cargo test --locked -j 8 market_backend -- --test-threads=8   # 5 passed
cd apps/desktop/src-tauri && cargo test --locked -j 8 catalog_slot -- --test-threads=8     # 1 passed
cd apps/desktop/src-tauri && cargo test --locked -j 8 buy_custom -- --test-threads=8       # 4 passed
```

## G-7 No PCGen in live code (`decisions.md §17`) — every `*.MC`, E5.1

```bash
git diff --name-only <base>..HEAD -- apps src crates data resources scripts | xargs -r grep -lis 'pcgen' | awk 'END{print NR}'   # 0 (a doc-comment citing the pinned oracle's docs is allowed only in this package)
bash scripts/verify.sh --only pcgen-residue-gate -j 8   # PASS
```

## G-8 Closure (DoD 13) — E5.1–E5.4

```bash
cd docs/release/SD-41-gear-and-equipment-sets
test -s kanban.md || { echo NO_KANBAN; exit 2; }
awk -F'|' '$2 ~ /^ (C|E)[0-9]/ { n++ } END { if (n != 37) print "ROW_COUNT " n }' kanban.md
awk -F'|' '$2 ~ /^ (C|E)[0-9]/ && $2 !~ /^ E5\.[2-4] / && $5 !~ /^ complete *$/ { print $2 "|" $5 }' kanban.md
grep -c '^- \(Gear screen\|Inventory page\) — usable: yes' artifacts/epic_4/E4.G_cycle_receipt.md   # 2 (anchored; the receipt's own instructions must not match)
(cd ../../../apps/desktop/src-tauri && cargo test --locked -j 8 --test market_price_census market_price -- --test-threads=8) 2>&1 | grep -c 'market_price_unknown_rows_name_every_source_tried .* ok'   # 1 (Q7 effort bar, decisions.md §30.1)
python3 -I -c "import json;u=json.load(open('../../../data/price_overrides/price-unknown.json'));print(len(u),sum(1 for r in u if r.get('tried')))"   # equal figures; the count the operator saw at E4.G
grep -c 'SD-41 INTERIM NOTES' release-notes.md                        # 0 at E5.3+
python3 ../../../tools/release/resolve_release_notes.py --repo-root ../../..
grep -L 'Looked at' artifacts/epic_[34]/*_cycle_receipt.md | grep -v 'E3.1_\|E3.2_\|E3.3_\|E4.G_'   # prints nothing (E3.MC, E4.1, E4.MC looked)
grep -L 'closure check' artifacts/epic_*/*_cycle_receipt.md                 # prints nothing (R15)
```

Plus the template §6 step 1a items: `cargo test --test packaged_resources` (desktop),
`cargo test --test no_baked_manifest_dir_in_production` (root), `python3 -m pytest tools/release -q`, and the
fresh-install check recorded in E5.1's receipt.

## Artifact map

| Gate | Receipt(s) | Evidence path |
|---|---|---|
| G-1 | every Phase-2 load-response card and MC; E1.4 | `artifacts/epic_<n>/<card>_cycle_receipt.md` "Seed deltas"; `apps/desktop/src-tauri/tests/old_save_loads_unchanged.rs` |
| G-2 | E4.1, E4.MC, E5.1 | the receipt's "Golden deltas" |
| G-3 | E1.MC | the receipt's partition line |
| G-4 | E1.2, E2.1, E2.2, E2.MC | `artifacts/epic_1/`, `artifacts/epic_2/`; the `.pcg` population line |
| G-5 | E1.1, E1.3, E2.3, E3.1 | `artifacts/epic_1/`, `artifacts/epic_2/E2.3_cycle_receipt.md` |
| G-6 | E3.1–E3.5, E3.MC | `artifacts/epic_3/E3.3/results.json`, `artifacts/epic_3/E3.MC/*.png` |
| G-9 | E1.5, E1.6, E2.4, E4.1 | receipts; `artifacts/epic_4/E4.1/*.png` |
| G-10 | E1.8, E1.9, E1.11, E2.5, E2.6, E3.4 | `artifacts/epic_1/E1.8/market-prices.json`, `artifacts/epic_1/E1.11/batch-*.json`, `data/price_overrides/*.json`, receipts |
| G-11 | E1.10, E2.7, E3.5 | receipts; `artifacts/epic_3/E3.3/results.json` (the three campaign rows) |
| G-7 | every MC, E5.1 | receipts |
| E4.G | E4.MC, E4.G | `artifacts/epic_4/E4.G/` (4 PDFs, ≥ 12 PNGs, the signed receipt) |
| G-8 | E5.1–E5.4 | `artifacts/epic_5/`, `receipts.md`, `docs/retro/sd41-retrospective.md` |

## Does not cover

Pixel positions; drag in ui-smoke (no drag op; E4.G's eye); Starfinder and Cyberpunk RED gear and markets
(unchanged); macOS and Windows installers; weight-reduction containers (DEF-1); eqmod formulas the converter
does not evaluate (listed by count, Q6); splitting a bundle (Q8); any rules value beyond the container sums,
the unarmed row and the ammunition count (`decisions.md §16`).
