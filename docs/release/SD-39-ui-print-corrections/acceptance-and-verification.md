---
canonical: true
owner: operator
bundle_id: SD-39
slug: ui-print-corrections
canonical_branch: tranche-19
target_version: 0.19.0
date: 2026-11-01
status: planning-ready
---

# SD-39 Acceptance and Verification

Gates G-1…G-12 map the Definition of Done (`scope-draft.md §2`) to commands. Run each filter in the cargo workspace that holds the test. **A filter that matches 0 tests is a FAIL, not a pass** (R20). Every closure gate is
also a fenced command somewhere a reader can copy without unescaping.

## G-1 Seed deltas (DoD 9) — every card; re-checked at every `*.MC` and E9.3

Base = the C1 commit, built in its own worktree with its own `CARGO_TARGET_DIR` (R-X). Head = the card's tree.

```bash
# PF: Aldric (Fighter 3), Elowen (Wizard 5)
cd apps/desktop/src-tauri && PF_SEED_RENDER_OUT=<dir> cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8 --nocapture
diff <base>/sha256.txt <head>/sha256.txt            # a difference is allowed ONLY with a field-by-field list in the receipt
python3 - <<'PY'   # list every changed JSON path between base and head renders
import json,sys
# the card binds BASE/HEAD json paths; prints "path: old -> new" lines; the receipt pastes them all
PY
# SF: four seeds' printed lines; CPR: five seeds (`ls <dir>`; an empty dir is a FAIL)
cd apps/desktop/src-tauri && mkdir -p <dir> && SF_SEED_LINES_OUT=<dir> cargo test --locked -j 8 --bins sf_seed_printed_features_equal_the_features_the_converted_records_grant -- --test-threads=8
cd apps/desktop/src-tauri && mkdir -p <dir> && CPR_SHEET_LINES_OUT=<dir> cargo test --locked -j 8 --bins every_rule_line_prints_with_its_resolved_numbers -- --test-threads=8
diff -r <base> <head>                                 # pass = no output, except E3.5 and E8.3 (listed)
cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --check
cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --system starfinder-1e --check
```

`mkdir -p <dir>` is in the command because the writer tests panic on a missing directory (SD-38 retro §2);
E8.3 builds the in-test control if the incident fires once (R14).

## G-2 Data unchanged unless a proof failed (E1.0)

```bash
git diff --name-only <c1-sha>..HEAD -- data/sheet_rules | awk 'END{print NR}'     # 0, unless E1.0's receipt names the failed proof
```

## G-3 Feat guards (DoD 3) — E1.1–E1.3, E1.MC

```bash
cargo test --locked -j 8 feat_budget -- --test-threads=8
cargo test --locked -j 8 feat_duplicate -- --test-threads=8
cd apps/desktop && npm run typecheck && npm test        # picker filter test included
```

## G-4 Layout (DoD 2) — E2.1–E2.4, E2.MC

```bash
cd apps/desktop && npm test                             # sheetLayout.test.ts region map, drawer reducer
grep -c 'Character Details' apps/desktop/src/characterHub/CharacterSheet.tsx   # ≥ 1 (heading)
RUN_DESKTOP_AGENT=sd39-e2.4 npm run ui-smoke -- --only <E2.4 ids>
```

## G-5 Market (DoD 4) — E3.1–E3.6, E3.MC

```bash
for t in market_category_filter enhancement_price_table market_buy_sell market_attach_price enhancement_rows; do cargo test --locked -j 8 $t -- --test-threads=8; done
grep -c 'Add Weapon\|Add Armor\|Add Gear' apps/desktop/src/characterHub/{Character,Starfinder,Cyberpunk}CharacterSheet.tsx   # 0 0 0
```

The CRB price table the test encodes (5 armor/shield + 5 weapon rows: +1 1,000/2,000 … +5 25,000/50,000
gp, plus masterwork 150/300) is quoted in the test with its page cite.

## G-6 Slots and custom items (DoD 5) — E4.1–E4.7, E4.MC

```bash
for t in slot_of slot_conflict worn_vs_carried custom_item_bonuses worn_item_applies; do cargo test --locked -j 8 $t -- --test-threads=8; done
```

## G-7 Overrides (DoD 7) — E6.1, E6.2, E6.MC

```bash
cargo test --locked -j 8 overrides_ -- --test-threads=8     # 16 cases
```

## G-8 Ledger (DoD 6) — E5.1–E5.3, E5.MC

```bash
for t in ledger_stamp ledger_migration progression_ledger; do cargo test --locked -j 8 $t -- --test-threads=8; done
```

## G-9 Print engine present and the old path gone (DoD 1) — E8.1, E8.4

```bash
grep -rn 'window.print\|printLayout' apps/desktop/src | awk 'END{print NR}'   # 0
grep -n 'fn compute\|calc(' apps/desktop/src-tauri/src/print/ | awk 'END{print NR}'   # 0 (NR-2)
cd apps/desktop/src-tauri && cargo test --locked -j 8 print_smoke -- --test-threads=8
```

## G-10 Print golden suite (DoD 1) — E8.5, E8.MC

```bash
cd apps/desktop/src-tauri && cargo test --locked -j 8 --test print_golden -- --test-threads=8
# 24 passed: {pf1e,sf1e,cpr} × {A4,Letter} × {none,spellbook,math,all}
# each: text present, headings in layout order, page count asserted, Typst diagnostics 0, two runs byte-equal
```

## G-11 Import (DoD 8) — E7.1, E7.2

```bash
cargo test --locked -p codex-ingest -j 8 pcg_import_fixtures -- --test-threads=8
bash scripts/verify.sh --only pcgen-residue-gate --only crate-wall
grep -rlE 'pcg([^e]|$)' src apps/desktop/src apps/desktop/src-tauri/src      # only the sidecar caller and its boundary; read by hand in the receipt
```

## G-12 Closure (DoD 10) — E9.1–E9.11

```bash
cd docs/release/SD-39-ui-print-corrections
test -s kanban.md || { echo NO_KANBAN; exit 2; }
awk -F'|' '$2 ~ /^ (C|E)[0-9]/ { n++ } END { if (n != 62) print "ROW_COUNT " n }' kanban.md
awk -F'|' '$2 ~ /^ (C|E)[0-9]/ && $2 !~ /^ E9\.([5-9]|1[01]) / && $5 !~ /^ complete *$/ { print $2 "|" $5 }' kanban.md
grep -c 'usable: yes' artifacts/epic_9/E9.4_cycle_receipt.md        # 3
grep -c 'SD-39 INTERIM NOTES' release-notes.md                        # 0 at E9.8+
python3 ../../../tools/release/resolve_release_notes.py --repo-root ../../..
```

Plus the template §6 step 1a items: `cargo test --test packaged_resources` (desktop),
`cargo test --test no_baked_manifest_dir_in_production` (root), `python3 -m pytest tools/release -q`,
and E9.2's observed fresh-install values.

## Artifact map

| Gate | Receipt(s) | Evidence path |
|---|---|---|
| G-1 | every card | `artifacts/epic_<n>/<card>_cycle_receipt.md` "Seed deltas" |
| G-2 | E1.0 | `artifacts/epic_1/E1.0_cycle_receipt.md` + `E1.0/key-diff.txt` |
| G-3…G-8 | the named cards + MC | per-epic receipts |
| G-9, G-10 | E8.1, E8.4, E8.5, E8.MC | `artifacts/epic_8/`, the 24 PDFs kept under `artifacts/epic_8/E8.5/` (git-ignored if > 5 MiB total; hashes committed) |
| G-11 | E7.1, E7.2 | `artifacts/epic_7/` + `E7.1/import-report-<fixture>.md` |
| G-12 | E9.1–E9.11 | `artifacts/epic_9/`, `receipts.md`, `docs/retro/sd39-retrospective.md` |

## Does not cover

Pixel layout of the PDF beyond the operator's three sheets (E9.4); SF/CPR slot rules; special
abilities; any book content.
