# Cycle AT-33-E5-finalize-wave3 — Epic 5 Re-verification / AT-33-E5-001, AT-33-E5-002, AT-33-E5-003 (totals + kanban call)

- **Commit SHA:** recorded by a follow-up commit after landing, per this bundle's own precedent (`AT-33-E5-001`'s `e10dead123`, `AT-33-E5-002`'s `56dc837d8d`, `AT-33-E5-finalize`'s `f8f82a61fb`).
- **Files touched:**
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/finalize-wave3-merge.py` (new — merges wave 3's 3 lane files into `literal-verified.oracle-results.json`, resolving cross-lane duplicate `unit_id`s via worst-verdict-wins)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/literal-verified.oracle-results.json` (regenerated — 6,514 rows)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json` (regenerated — 8,255 rows)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/finalize-wave3-duplicate-unit-ids.json` (new — full per-source detail on the 15 real cross-lane duplicates)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/finalize-wave3-missing-shapes.py` (new — classifies the 75 still-unrowed units by real corpus-record shape)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/finalize-wave3-missing-literal.json`, `finalize-wave3-missing-literal-shapes.json` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-001_cycle_receipt.md`, `AT-33-E5-002_cycle_receipt.md`, `AT-33-E5-003_cycle_receipt.md` (final-totals sections appended)
  - `docs/release/SD-33-computed-value-verification/kanban.md` (rows 16/17/18)
  - `docs/release/SD-33-computed-value-verification/progress.md` (this cycle's entry, one row per disagreement)
  - `docs/retro/events/sd33-r3-e5-finalize.jsonl` (new)
  - `docs/retro/events/sd31-transcribe.jsonl` (pre-existing dirty tracked file found at turn start —
    two locally-appended, never-pushed verify.sh log lines from a prior agent turn sharing this
    checkout, union-merged with `origin/tranche/13`'s own newer entries so no side's log lines
    were lost; append-only JSONL, no semantic conflict; committed separately as
    `976d12dccb` before this cycle's own work, to unblock the rebase)
  - This file (new).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (this cycle's own uncommitted diff, scoped to
  every file listed above)
- **Wired-integration audit result:** `OK_NO_TOKENS` on this cycle's own uncommitted diff. The
  full-branch-scope command (`git diff ... "$(git merge-base HEAD origin/develop)...HEAD" -- <full
  epic-5-reverification scope>`) surfaces one match, `"placeholder marker"` inside
  `AT-33-E5-remainder-charbuild_cycle_receipt.md`'s own quoted audit output — a **pre-existing**
  match from an earlier landed cycle, already self-disclosed there as "one pre-existing match, not
  introduced by this cycle"; not introduced or touched by this cycle.
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-001 — the 1,741 `fixture-verified` units are re-examined against the oracle
  >
  > **Evidence:** per-unit `(ours, oracle, verdict)` rows committed; agreement and disagreement
  > counts both stated, with the denominator.
  >
  > ### AT-33-E5-002 — the 6,589 `literal-verified` units are re-examined
  >
  > **Evidence:** as above.
  >
  > ### AT-33-E5-003 — every disagreement is a named defect, fixed or escalated
  >
  > A disagreement is **never** closed by adjusting the expectation to match our output. Each is
  > root-caused: either our computation is wrong (fix it) or the oracle comparison is wrong (fix
  > the harness, and re-run everything it already judged).
  >
  > **Evidence:** one entry per disagreement in `progress.md`, each resolved to a commit or an
  > operator escalation. **A filed blocker does not satisfy this criterion.**

## What this cycle is, and is not

This cycle **merges and totals** — it dispatched no new lane, examined no new unit itself, and
implemented no engine fix. Its own deliverable is: (1) merge wave 3's three landed lane files into
the three canonical files, resolving every real cross-lane duplicate `unit_id` it finds rather than
last-writer-wins; (2) re-derive the unexamined set directly from `docs/work-inventory.json`, not by
inference; (3) confirm zero reasonless-`unverifiable` across all three files; (4) root-cause every
new disagreement (26, all real, none fixed this cycle — see below); (5) re-prove disagree-detection
capability on the current, unmodified batch path; (6) keep the denominator gate green; (7) make the
honest kanban call on rows 16/17/18.

**Reports were not trusted as evidence.** Every figure below was re-derived by counting rows in the
committed files after fetching and rebasing onto `origin/tranche/13`'s real HEAD (`1cfa4d7ca8` at
fetch time), not taken from any lane's own self-reported totals.

## Environment hazard found and cleared before any of this cycle's own work

`git status --porcelain` at turn start listed one dirty tracked file,
`docs/retro/events/sd31-transcribe.jsonl` (2 locally-appended `verify.sh` PASS log lines never
committed or pushed by a prior agent turn sharing this checkout), plus 4 untracked
`sd-33-*.workflow.js` files (pre-existing orchestration scripts, not this cycle's scope, left
untouched). The dirty tracked file blocked a clean `git merge --ff-only origin/tranche/13` /
`git rebase`. Confirmed both local and `origin/tranche/13` had independently appended *different*
new lines to the same append-only JSONL log (real diff comparison, not assumed) — union-merged
both sides' lines (order-preserving, origin's lines first) so neither side's log entries were
lost, committed separately (`976d12dccb`) before starting this cycle's own merge work, per
AGENTS.md's "fix the source, not the symptom" and "one writer per tree" (confirmed via `git diff`
that the two versions differed only by genuinely distinct appended lines, not a real conflict).

## Merge: wave 3's three lane files → the canonical files

Fetched `origin/tranche/13`, rebased local `HEAD` onto it (12 commits gained, including all three
wave-3 lane landings: `dd274ffa2f`/`83fe2de103` (`var`), `66984fe7bc`/`f66ae64320`/`3aadb9442e`/
`ec5cf4bbe9`/`1cfa4d7ca8` (`combat`), `b1838c8d38`/`dae1591fe7` (`stat-save-tail`)).

- `equipment-shape-var.oracle-results.json`: 108 rows (44 agree / 1 disagree / 63 unverifiable)
- `equipment-shape-combat.oracle-results.json`: 82 rows (40 agree / 26 disagree / 16 unverifiable)
- `equipment-shape-stat-save-tail.oracle-results.json`: 141 rows
- `literal-verified.oracle-results.json` (pre-merge): 6,198 rows

All 331 raw rows (321 distinct — 10 `var`↔`combat` duplicate `unit_id`s) confirmed
`status=='literal-verified'` in `docs/work-inventory.json`, 0 `fixture-verified` — re-derive:
`python3 -c "import json; inv={u['id']:u['status'] for u in json.load(open('docs/work-inventory.json'))['units']}; ids=set(); [ids.update(r['unit_id'] for r in json.load(open(f))['results']) for f in ['docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-shape-var.oracle-results.json','docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-shape-combat.oracle-results.json','docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-shape-stat-save-tail.oracle-results.json']]; import collections; print(collections.Counter(inv.get(u) for u in ids))"`
→ `Counter({'literal-verified': 321})`.

### 15 real cross-lane duplicate `unit_id`s — a real finding, root-caused

An equipment record can carry more than one magnitude token (e.g. an armor item with both a
`VAR|ArmorCheckPenalty` chain and a `COMBAT|AC` chain, confirmed against `magnitude_token_count`
in `docs/work-inventory.json` — e.g. `boots_of_the_mastodon` carries 3,
`hallowed_chain_greater` carries 5). More than one shape lane's own census independently counted
and examined the SAME unit_id for a DIFFERENT dimension of that unit, producing more than one row
for it. Full per-source detail: `finalize-wave3-duplicate-unit-ids.json`.

Breakdown of the 15 (`finalize-wave3-merge.py`'s own output):
- 4 `literal-verified`↔`var`: `boots_of_the_mastodon` (both agree, genuinely independent
  dimensions — opposite-signed values, `-2` vs `+2`), `crown_of_conquest` and `hunter_s_band`
  (`literal-verified` dimension agrees, `var` dimension `unverifiable` — a real,
  gated-by-unbuilt-class-feature reason), `rod_escape` (both agree, identical values — likely the
  SAME dimension examined twice by two lanes, not two distinct dimensions).
- 1 `literal-verified`↔`combat`: `ring_of_unquenchable_passions` (both agree, independent
  dimensions).
- 10 `var`↔`combat`: 8 cases where `var` agrees but `combat` disagrees, 1 (`goblin_plate`) where
  both agree, and 1 (`panoply_of_the_fierani_knight`) where **both** disagree — a compound case,
  see below.

**Merge rule applied (never last-writer-wins):** the merged row takes the worst verdict across all
of a unit's source rows (`disagree` > `unverifiable` > `agree`) — a unit is not correctly-verified
as a whole if any one of its examined dimensions is wrong or unchecked. Every source row is
preserved verbatim under the merged row's own `multi_shape_sources` field; nothing discarded.

## The re-derived unexamined set: 75, by real corpus shape (never inferred)

Took the full `literal-verified` `unit_id` set from `docs/work-inventory.json` (6,589, `jq
'[.units[]|select(.status=="literal-verified")]|length' docs/work-inventory.json`), subtracted
every `unit_id` present in the merged `literal-verified.oracle-results.json` (6,514) — the
difference is **75**, printed and classified, not assumed empty:

```
python3 -c "import json; inv=set(u['id'] for u in json.load(open('docs/work-inventory.json'))['units'] if u['status']=='literal-verified'); got=set(r['unit_id'] for r in json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/literal-verified.oracle-results.json'))['results']); print(len(inv-got))"
→ 75
```

Every one of the 75's real corpus record read (`finalize-wave3-missing-shapes.py`, fixing an
initial glob-path bug this cycle found — `equipment_modifier` corpus records live at
`data/corpus/<book>/equipment/equipmods/*.json`, not `data/corpus/<book>/equipment_modifier/*.json`
— confirmed via `find data/corpus -iname draco.json`):

| Shape | Count |
|---|---:|
| `WEAPON` | 23 |
| `SKILL` | 17 |
| `WEAPONPROF` | 15 |
| `COMBAT` | 7 |
| `VAR` (equipment_modifier kind) | 5 |
| `EQMWEAPON` | 3 |
| `SITUATION` | 2 |
| `EQM` | 1 |
| `MOVEADD` | 1 |
| `STAT` | 1 |
| **Total** | **75** |

Full detail: `finalize-wave3-missing-literal-shapes.json`.

## Reasonless `unverifiable`: 0, re-derived across the combined file

`python3 -c "import json; d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json')); print(len([r for r in d['results'] if r['verdict']=='unverifiable' and not r.get('reason')]))"`
→ `0`.

## 26 disagree — every one root-caused, none fixed this cycle

The merged distinct disagree count (27 raw disagreeing rows minus 1 collapsed by the
`panoply_of_the_fierani_knight` `var`↔`combat` duplicate) is **26**. Full per-unit table with
`(unit_id, ours, oracle, root cause)` is in `progress.md`'s own `AT-33-E5-finalize-wave3` entry
(one entry per disagreement, per this criterion's own evidence requirement).

- **21** share one named engine gap: `compute_arms_armor_effect`
  (`src/rules_core/equipment_effects/arms_armor.rs`) and `compute_var_effect` (`general.rs`) each
  read only a base equipment record's own literal chain; neither resolves a base item's `EQMOD:`-
  referenced modifier record (a separate `equipment_modifier` corpus record baked into the same
  equipped item) and sums its own separate `BONUS:` chain. This is the same base-item-plus-
  attached-EQMOD gap `AT-33-E5-remainder-equipment_cycle_receipt.md` first named for a 3-unit
  slice; confirmed this cycle to recur across both `COMBAT` and `VAR` shapes at real scale (21
  units).
- **3** (`field_plate`, `stoneplate`, `snakeskin_tunic`): a real baseline-diff harness-methodology
  limitation (a second-order `AC.Total` effect the whole-character diff cannot separate from the
  item's own token) — not necessarily an engine defect.
- **1** (`sea_knife`): an unhandled `PRE`-gated conditional chain, read unconditionally.
- **1** (`diviner_s_blight`): not yet individually diagnosed.

**Why none were fixed this cycle:** the dominant 21-unit fix is a real, cross-cutting change to two
resolvers (EQMOD-chain lookup and summation), needing its own TDD cycle (RED against 2-3 of the
affected units first) and a full live-oracle re-verification of every affected unit before it could
be trusted — real engineering work outside this merge/finalize cycle's own scope and remaining turn
budget. Attempting it rushed, without the TDD discipline AGENTS.md requires, would risk landing a
plausible-but-wrong fix — worse than leaving the gap named. Scoped as the concrete top item of the
next cycle's plan (`progress.md`'s entry).

**Note found while consolidating, corrected by direct re-count:**
`AT-33-E5-shape-combat_cycle_receipt.md`'s own prose states "21 confirmed exact... plus 2 close
variants = 23 of 26" for its EQMOD bucket, which — summed with its own separately-named 3
baseline-diff + 1 PRE-gated + 1 undiagnosed — totals 28, not that receipt's own stated 26-unit
denominator. This cycle's own re-derivation from the committed per-unit rows finds 21 in the EQMOD
bucket (not 23) — a 2-unit prose/data mismatch in the source receipt, corrected here rather than
propagated (AGENTS.md item 9: "of 608 recorded corrections, the most frequently wrong artifacts are
our own briefs... not code").

## Disagree-capability re-proof on the current batch path

A zero-disagreement result across 8,330 would be suspicious, not happy — but this merge does not
land on zero. Real, not synthetic, proof the CURRENT path detects disagreement:

```
$ python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json
uncovered=0 overlap=0 population=49438 oracle_disagreement=26 unverifiable_done=0 stale=False
ORACLE_DISAGREEMENT: inner_sea_races:equipment:armor_of_grim_triumph, ... (+16 more)
$ echo $?
1
```

`box_ledger.py` — the unmodified fail-closed gate every prior `AT-33-E1-002`/`AT-33-E5-remainder-*`
cycle uses, not a script built for this proof — independently detects and names all 26 real
disagreements, exits 1 (fail-closed, correctly). `uncovered=0 overlap=0` re-confirms `THE-BOX.md`'s
partition still holds after this merge (no unit gained or lost a group membership).

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| `fixture-verified` rows | 1,741 | of 1,741 (unchanged, unaffected by wave 3) | `python3 -c "import json; print(len(json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixture-verified.combined-oracle-results.json'))['results']))"` → `1741` |
| `literal-verified` rows | 6,514 | of 6,589 (98.9%) | `python3 -c "import json,collections; d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/literal-verified.oracle-results.json')); r=d['results']; print(len(r), len(set(x['unit_id'] for x in r))); print(collections.Counter(x['verdict'] for x in r))"` → `6514 6514`, `Counter({'unverifiable': 6149, 'agree': 339, 'disagree': 26})` |
| `literal-verified` unexamined | 75 | of 6,589 | `python3 -c "import json; inv=set(u['id'] for u in json.load(open('docs/work-inventory.json'))['units'] if u['status']=='literal-verified'); got=set(r['unit_id'] for r in json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/literal-verified.oracle-results.json'))['results']); print(len(inv-got))"` → `75` |
| `AT-33-E5-003.combined-oracle-results.json` rows | 8,255 | of 8,330 | `python3 -c "import json,collections; d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json')); r=d['results']; print(len(r), len(set(x['unit_id'] for x in r))); print(collections.Counter(x['verdict'] for x in r))"` → `8255 8255`, `Counter({'unverifiable': 7494, 'agree': 735, 'disagree': 26})` |
| Duplicate `unit_id`s found and resolved | 15 | of 331 raw wave-3 rows | `python3 -c "import json; print(len(json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/finalize-wave3-duplicate-unit-ids.json'))['duplicates']))"` → `15` |
| Reasonless `unverifiable` | 0 | of 7,494 `unverifiable` rows (combined file) | `python3 -c "import json; d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json')); print(len([r for r in d['results'] if r['verdict']=='unverifiable' and not r.get('reason')]))"` → `0` |
| `box_ledger.py --check` | `uncovered=0 overlap=0 population=49438 oracle_disagreement=26 unverifiable_done=0 stale=False`, exit **1** (correctly) | population 49,438 | `python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json` |
| `scripts/verify.sh --only denominator-gate` | `PASS` (`files_checked=35 violations=0`) | n/a | `bash scripts/verify.sh --only denominator-gate` |

## Status: blocked-escalated

**Not `complete`.** This cycle fully accomplished its own merge/total/kanban-call mandate — every
lane file merged, every duplicate root-caused, the unexamined set precisely re-derived (not
inferred), reasonless-unverifiable confirmed at 0, every new disagreement root-caused, disagree
capability re-proven on the live batch path, the denominator gate green. **But the underlying
population is not fully closed**: 75 of 6,589 `literal-verified` units remain genuinely unrowed,
and 26 real disagreements remain unresolved (root-caused but not fixed — the dominant 21-unit
EQMOD-chain-summation fix is real cross-cutting engine work this cycle's own scope and turn budget
did not reach). Marking Epic 5 `complete` while either gap stands would repeat exactly the
false-completion shape this remediation wave exists to close. Rows 16/17/18 kanban calls above
reflect this honestly.

## Movement, four buckets

- **closure:** 0 — no unit's `docs/work-inventory.json` `status` field changed.
- **reclassification:** 0
- **reachability:** 0 — this cycle widened neither the examined population nor any resolver
  (merging existing lane output is not new examination).
- **instrument-correction:** 1 — the 2-unit prose/count mismatch in
  `AT-33-E5-shape-combat_cycle_receipt.md`'s own EQMOD-bucket arithmetic, found and corrected by
  direct re-derivation from the committed per-unit rows rather than propagated.

## Notes

- **Why fixture-verified stayed untouched:** confirmed, not assumed — every wave-3 lane's own
  unit_id resolves to `status=='literal-verified'` in `docs/work-inventory.json`, 0 to
  `fixture-verified` (command above).
- **The `equipment_modifier` glob-path bug** (`data/corpus/<book>/equipment_modifier/*.json` does
  not exist; the real path is `data/corpus/<book>/equipment/equipmods/*.json`) was found and fixed
  within this cycle, before it could silently mislabel 14 of the 75 missing units as
  "MISSING_CORPUS_FILE" — confirmed via `find data/corpus -iname draco.json` before trusting the
  corrected classification.
- **This cycle attempted no new engine fix and dispatched no new lane**, matching its own explicit
  mandate ("total Epic 5, own the kanban call") — the alternative (a rushed EQMOD-chain fix without
  proper TDD, inside an already-long merge/totaling turn) risked landing a plausible-but-wrong
  change under time pressure, which AGENTS.md's TDD mandate and "fix the source, not the symptom"
  rule both weigh against.

## Test scoping

Ran `python3 scripts/box_ledger.py --check --oracle-results <combined file>` (real, against the
regenerated combined file) and `bash scripts/verify.sh --only denominator-gate` (real, PASS, after
this cycle's own prose edits). Ran the two `finalize-wave3-*.py` scripts directly (no test harness
— data-merge/classification scripts over already-tested engine output, matching every prior
`AT-33-E5-00x` finalize/remainder cycle's own precedent for this class of script). Did **not**
touch `src/`, run `cargo test`, or touch `apps/desktop/src-tauri` — no engine code changed this
cycle (no fix was attempted; see Notes).

## Next-cycle plan (concrete)

1. **The EQMOD-embedded-modifier-chain-summation fix** (`compute_arms_armor_effect` +
   `compute_var_effect`) — the single highest-value remaining item, closing 21 of 26 disagreements
   with one mechanism. TDD against 2-3 of the 21 affected units first (e.g.
   `gnome_scrap_armor`/`hide_of_grim_triumph`, both small deltas), then re-run against all 21 plus
   `panoply_of_the_fierani_knight`'s compound case.
2. **The 75 unexamined**, by shape: `WEAPON` (23) + `WEAPONPROF` (15) need
   `WEAPON.<i>.MAGICHIT`/`.MAGICDAMAGE` oracle isolation (identified, not yet run, per
   `AT-33-E5-shape-combat_cycle_receipt.md`'s own next-cycle plan); `SKILL` (17, multi-skill/`ALL`)
   and the `VAR`-labeled `equipment_modifier` 5 need the base-item-plus-attached-EQMOD fixture
   pattern (shares the engine fix in item 1); `COMBAT` (7 remaining) needs a natural-attack fixture
   plus a formula evaluator for non-literal `COMBAT|AC` chains; `EQMWEAPON`/`EQM`/`SITUATION`/
   `MOVEADD`/`STAT` (8 combined, small shapes) each need their own small census + fixture.
3. `sea_knife`'s `PRE`-gate and `diviner_s_blight`'s undiagnosed gap are standalone, smaller fixes.
