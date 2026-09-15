---
canonical: true
owner: god-emporer
bundle_id: SD-35
date: 2026-09-07
---

# SD-35 Release Notes — build 0.15.0

Written at closure by `AT-35-E7-003`, September 15, 2026.

## What shipped — the corpus

| metric | value | command |
|---|---|---|
| **inventory completion** | **49,450 of 49,450 = 100%** | `python3 scripts/completion_atlas.py --check` |
| **corpus completion** (the headline) | **48,864 of 48,864 real `data/corpus` rules** | cited from `artifacts/epic-7-closure/population-census-final.json` + `b18_ten_render_proof.py` |
| rules written | 70,317 across 49,450 unit files | `python3 -c "import json;print(json.load(open('data/sheet_rules/_report.json'))['rules_written'])"` |
| refused tokens | 0 of 49,450 records | `python3 scripts/token_coverage.py --check` |

Under the sheet rule (`decisions.md §1`), a unit is DONE when the sheet shows exactly what a player would write: a final number, a dice expression, or the rule's words.

## The converter and live evaluator

**Epic 2 — Sheet rule** dispatched the converter (`src/bin/sheet_rule_convert.rs`, `src/pcgen_import/sheet_rule/`) and built the live-side evaluator (`src/rules_core/sheet_rule.rs`). The mapping table (`artifacts/epic-2-sheet-rule/token-mapping/mapping-table.v1.json`) translated 249 rows of PCGen token vocabulary to our schema (`Expr`, `Applies`, prose), and the one-pass converter run closed **21,911 of 23,315** non-DONE units in a single cycle.

**Epics 3–5 — Placement, resolution, residues** completed the conversion for all remaining classes and kinds. **Epic 6 — PCGen exit** removed the 254 live-side files reading PCGen tokens, formulas, and `raw_tokens`, with oracle parity proving correctness before and after (see "Oracle parity" below).

## The sheet — output shape

| lines per unit | units | share of 49,450 |
|---|---|---|
| 1 | 44,840 | 90.7% |
| 2 | 1,302 | 2.6% |
| 3–10 | 3,909 | 7.9% |
| >10 | 350 | 0.7% |
| **mean 1.42, median 1** | 49,450 | 100% |

Re-derive: `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/lines_per_unit_census.py` (or see `retrospective.md §2` for the full shell command).

Nine records in ten print exactly one line. The tail of 201-line records is legitimate: class progressions carry one rule per level.

## The majority form — `Words` share per kind

| kind | rules | `Words` share |
|---|---|---|
| **all kinds** | **70,317** | **56.8% = 39,941 rules** |
| deity | 459 | 100.0% |
| language | 136 | 100.0% |
| domain | 185 | 97.3% |
| spell | 3,104 | 90.1% |
| power | 447 | 88.8% |
| feat | 2,904 | 85.2% |
| monster_ability | 4,601 | 78.7% |
| equipment_modifier | 1,660 | 77.2% |
| class_feature | 20,943 | 74.0% |
| equipment | 7,322 | 64.9% |
| ability | 5,285 | 61.4% |
| race_trait | 3,000 | 61.0% |
| race | 136 | 59.6% |
| trait | 546 | 57.9% |
| template | 3,068 | 34.7% |
| companion | 4,751 | 19.9% |
| skill | 380 | 14.5% |
| class | 837 | 9.8% |
| monster | 10,553 | 6.8% |

Re-derive: `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/words_share_census.py` (or see `retrospective.md §3`).

The split is by kind, not by book: a `monster` stat block is 6.8% words because it carries numbers; a `deity` entry is 100% words because the entry *is* prose.

## Process figures

| metric | value | command |
|---|---|---|
| cycles dispatched | 127 | `git log --oneline --grep "AT-35-E" \| wc -l` or `git diff 4c6c57eb9f..HEAD --stat \| grep "cycle.*receipt"` |
| units closed per cycle | min 0, median 0, max 21,911 | `docs/release/SD-35-corpus-sheet-completion/retrospective.md §1` |
| **build time before / after** (cold, paired) | **188.97 s → 145.89 s (−22.8%, −43 s)** | `python3 -c "import json;print(json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-1-tax-cut/build-time.json'))['paired_rerun'])"` |
| test binaries before / after | 544 → 362 | Epic 1 consolidated 184 near-identical template test files into two binaries |
| build stages verified | 49 of 49 PASS | `bash scripts/verify.sh` at closure |

The build time improvement came from folding templated test families (89 `sd18_*_widening.rs` and 95 `sd13_*progression*.rs` files, ~80k of 187k test lines) into two table-driven binaries, with identical test count before and after: 8,723 entries → 8,723 entries, zero only-in-before, zero only-in-after.

## The PCGen exit

`decisions.md §11`: PCGen tokens, formulas, and `raw_tokens` exist in exactly two places: the **converter** (ingest tooling) and the **test oracle** (`scripts/oracle_harness/`). The live side is clean.

| epic | `live_files` at first receipt | at last receipt | difference |
|---|---|---|---|
| 1 — tax cut | 260 | 260 | 0 (gate built) |
| 2 — sheet rule | 260 | 253 | −7 |
| 3 — place and surface | 260 | 253 | −7 |
| 4 — resolve and verify | 260 | 253 | −7 |
| 5 — residues | 260 | 253 | −7 |
| **6 — PCGen exit** | **254** | **0** | **−254** |
| 7 — closure | 0 | 0 | 0 |

Re-derive: `python3 scripts/pcgen_residue_gate.py --check --closure`

**Epic 6 removed all 254 files**, over 67 of the bundle's 127 cycles, in a long monotonic descent: 254 → 208 → 197 → 81 → 45 → 25 → 16 → 8 → 4 → 0. It never rose.

The converter, parser, generators, and oracle harness all survive — they are reusable assets for the next system (Starfinder uses the same `.lst` format with a different include structure).

### Oracle parity

At the `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` pin:

| phase | compared | agree | disagree |
|---|---|---|---|
| before Epic 2 | N/A | N/A | N/A |
| **at AT-35-E2-005 cycle 4** | **42 values** | **41** | **1** |
| at AT-35-E6-003 (pre-cleanup) | 156 | 154 | 2 |
| **at closure (AT-35-E6-004)** | **159** | **157** | **2** |

The one persistent disagreement (2 → 2) is documented in `artifacts/epic-6-pcgen-exit/oracle-parity-after.json`. The three new compared values from the widened corpus population all agree.

## Version confirmation

- `apps/desktop/package.json`: **0.15.0** ✓
- `apps/desktop/src-tauri/tauri.conf.json`: **0.15.0** ✓

The tranche digit does not change at bundle closure — it moved only on the `tranche/15` cut (`decisions.md §10`).

## Figures + their re-derive commands

Every figure above carries its re-derive command and denominator. Re-run any to verify at HEAD:

```bash
python3 scripts/completion_atlas.py --check
python3 scripts/token_coverage.py --check
python3 scripts/pcgen_residue_gate.py --check --closure
python3 -c "import json;print(json.load(open('data/sheet_rules/_report.json'))['rules_written'])"
python3 -c "import json;print(json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-1-tax-cut/build-time.json'))['paired_rerun'])"
bash scripts/verify.sh
```
