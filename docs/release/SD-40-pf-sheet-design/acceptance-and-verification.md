---
canonical: true
owner: operator
bundle_id: SD-40
slug: pf-sheet-design
canonical_branch: tranche-20
target_version: 0.20.0
date: 2026-11-07
status: planning-ready
---

# SD-40 Acceptance and Verification

Gates G-1…G-8 map the Definition of Done (`scope-draft.md §2`) to commands. Run each cargo filter from
`apps/desktop/src-tauri` (the print module is a `#[cfg(test)]` module of that binary). **A filter that
matches 0 tests is a FAIL, not a pass** (R20). Every closure gate is also a fenced command a reader can
copy without unescaping.

## G-1 PF seed renders unchanged (DoD, `decisions.md §16`) — every card touching `print/**`; re-checked at every `*.MC` and E5.1

Base = the C1 commit, built in its own worktree with its own `CARGO_TARGET_DIR` (R-X). Head = the card's tree.

```bash
cd apps/desktop/src-tauri && mkdir -p <dir>/base <dir>/head
PF_SEED_RENDER_OUT=<dir>/head cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8 --nocapture
diff <dir>/base/sha256.txt <dir>/head/sha256.txt      # pass = no output; any line is a defect (SD-40 changes no DTO the screen reads)
```

## G-2 SF1e and CPR golden output unchanged (DoD 8, R2) — every card touching `print/**` or `resources/print/**`; every `*.MC`

```bash
cd apps/desktop/src-tauri && mkdir -p <dir>/base <dir>/head
PRINT_GOLDEN_OUT=<dir>/head cargo test --locked -j 8 print_golden -- --test-threads=8     # 24 passed
for f in <dir>/head/sf1e-*.pdf <dir>/head/cpr-*.pdf; do
  b=<dir>/base/$(basename "$f"); test -f "$b" || { echo "MISSING_BASE $b"; exit 2; }
  diff <(pdftotext -layout "$b" -) <(pdftotext -layout "$f" -) > /dev/null || echo "TEXT_DIFF $(basename "$f")"
  [ "$(pdfinfo "$b" | awk '/Pages/{print $2}')" = "$(pdfinfo "$f" | awk '/Pages/{print $2}')" ] || echo "PAGES_DIFF $(basename "$f")"
done                                                   # pass = no output (E4.1: the renamed cases compare basic↔none, full↔all; basicBio and spellbook are the named additions)
sha256sum <dir>/base/sf1e-*.pdf <dir>/base/cpr-*.pdf | sed 's#<dir>/base/##' > <dir>/base.sha; sha256sum <dir>/head/sf1e-*.pdf <dir>/head/cpr-*.pdf | sed 's#<dir>/head/##' > <dir>/head.sha
diff <dir>/base.sha <dir>/head.sha                     # a byte delta is allowed ONLY with the frame/dispatcher line named in the receipt and the two checks above empty
git diff <base>..HEAD -- apps/desktop/src-tauri/src/print/layout/sf1e.rs apps/desktop/src-tauri/src/print/layout/cpr.rs | awk 'END{print NR}'   # 0 outside E4.1's named test-literal fence exits
```

## G-3 The model and the vocabulary (DoD 1, 2) — E1.1, E1.MC, E3.2

```bash
cd apps/desktop/src-tauri && cargo test --locked -j 8 print_pf_blocks -- --test-threads=8     # the card's quoted count; includes *_every_pf_block_id_is_in_the_shared_vocabulary, *_every_look_has_its_template, *_a_place_past_the_grid_is_refused
cd apps/desktop && npx tsx src/characterHub/sheetLayout.test.ts && grep -c block_vocabulary src/characterHub/sheetLayout.test.ts   # exit 0; >= 1
python3 -c "import json;d=json.load(open('apps/desktop/src-tauri/src/print/golden/block_vocabulary.json'));print(len(d))"   # the fixture's size, quoted in the receipt
ls apps/desktop/src-tauri/resources/print/looks/*.typ | awk 'END{print NR}'                   # the look count; equals the Look variants (the Rust test)
```

## G-4 Page 1 and the step-1 gate (DoD 3) — E2.3, E2.MC, E2.G

```bash
cd apps/desktop/src-tauri && cargo test --locked -j 8 print_pf_blocks_page_one_holds_the_step_one_blocks_on_both_papers -- --test-threads=8   # 1 passed
ls docs/release/SD-40-pf-sheet-design/artifacts/epic_2/E2.G/*.pdf | awk 'END{print NR}'        # 2 (A4, Letter)
grep -c 'usable: yes' docs/release/SD-40-pf-sheet-design/artifacts/epic_2/E2.G_cycle_receipt.md   # 1
```

## G-5 Presets and the dialog (DoD 5) — E4.1, E4.2, E4.MC

```bash
cd apps/desktop/src-tauri && cargo test --locked -j 8 print_pf_blocks_each_preset_is_a_list -- --test-threads=8    # 1 passed
grep -c 'PRESETS' apps/desktop/src-tauri/src/print/layout/pf1e.rs                                # >= 1
grep -rn 'PRINT_OPTION_BOXES\|spellbook: bool\|math: bool' apps/desktop/src apps/desktop/src-tauri/src/print | awk 'END{print NR}'   # 0 after E4.2
cd apps/desktop && npx tsx src/characterHub/PrintDialog.test.ts && grep -c 'print-preset-' src/characterHub/PrintDialog.test.ts   # exit 0; >= 4
```

## G-6 Density (DoD 6) — E4.1, E4.MC, E5.1

```bash
python3 -c "import json;d=json.load(open('apps/desktop/src-tauri/src/print/golden/print_golden.json'));bad=[k for k,v in d.items() if 'pages_max' not in v or v['pages']>v['pages_max']];print(len(d),bad)"   # 24 []
python3 -c "import json;d=json.load(open('apps/desktop/src-tauri/src/print/golden/print_golden.json'));print({k:(v['pages'],v['pages_max']) for k,v in d.items() if k.startswith('pf1e')})"   # pasted in the receipt; pf1e-*-basic pages <= 2 (Elowen is the golden PF character: <= 3), per decisions.md §10
cd apps/desktop/src-tauri && cargo test --locked -j 8 print_golden -- --test-threads=8        # 24 passed (asserts pages <= pages_max)
```

Aldric's own ceilings (Basic ≤ 2 on both papers) are asserted by `print_proof`'s page-count check
(E4.1 adds it: `PRINT_PROOF_OUT` set → it also asserts each PDF's page count ≤ the `pages_max` of the
matching golden case family) and read by hand from `pdfinfo` in E4.MC's receipt.

## G-7 No PCGen in live code (R3) — E0.1, every `*.MC`, E5.1

```bash
bash scripts/verify.sh --only pcgen-residue-gate
git diff --name-only <base>..HEAD -- apps src crates data resources scripts | awk 'END{print NR}'   # E0.1: 0
grep -rniE 'xsl|pcgen' apps/desktop/src-tauri/resources/print/ | awk 'END{print NR}'          # 0
```

## G-8 Closure (DoD 10) — E5.1–E5.5

```bash
cd docs/release/SD-40-pf-sheet-design
test -s kanban.md || { echo NO_KANBAN; exit 2; }
awk -F'|' '$2 ~ /^ (C|E)[0-9]/ { n++ } END { if (n != 23) print "ROW_COUNT " n }' kanban.md
awk -F'|' '$2 ~ /^ (C|E)[0-9]/ && $2 !~ /^ E5\.[3-5] / && $5 !~ /^ complete *$/ { print $2 "|" $5 }' kanban.md
grep -c 'usable: yes' artifacts/epic_2/E2.G_cycle_receipt.md        # 1
grep -c 'usable: yes' artifacts/epic_5/E5.2_cycle_receipt.md        # 5
grep -c 'SD-40 INTERIM NOTES' release-notes.md                        # 0 at E5.4+
python3 ../../../tools/release/resolve_release_notes.py --repo-root ../../..
grep -L 'Looked at' artifacts/epic_[1-4]/*_cycle_receipt.md | grep -v 'E1.1_\|E1.2_\|E2.G_\|E4.1_\|E4.2_'   # prints nothing (NR-11)
```

Plus the template §6 step 1a items: `cargo test --test packaged_resources` (desktop),
`cargo test --test no_baked_manifest_dir_in_production` (root), `python3 -m pytest tools/release -q`.

## Artifact map

| Gate | Receipt(s) | Evidence path |
|---|---|---|
| G-1, G-2 | every `print/**` card and MC | `artifacts/epic_<n>/<card>_cycle_receipt.md` "Seed deltas", "Golden deltas" |
| G-3 | E1.1, E1.MC, E3.2 | `artifacts/epic_1/`, `print/golden/block_vocabulary.json` |
| G-4 | E2.3, E2.MC, E2.G | `artifacts/epic_2/E2.G/` (the two PDFs, their PNGs, the signed receipt) |
| G-5, G-6 | E4.1, E4.2, E4.MC | `artifacts/epic_4/`, `print/golden/print_golden.json` |
| G-7 | E0.1, every MC | `artifacts/epic_0/E0.1/` (the reference set, never shipped) |
| R7 "Looked at" | every look/layout card and MC | `artifacts/epic_<n>/<card>/*.png` |
| G-8 | E5.1–E5.5 | `artifacts/epic_5/`, `artifacts/epic_5/E5.2/` (10 PDFs, signed receipt), `receipts.md`, `docs/retro/sd40-retrospective.md` |

## Does not cover

Pixel positions (DEF-1); SF1e and CPR looks (DEF-4); macOS and Windows installers; the Update → Check
path (unchanged since SD-39 E9.2); any engine value.
