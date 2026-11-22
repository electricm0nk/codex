# AT-35-E7-001 — final-acceptance scan — cycle 2 receipt

**Verdict: FAIL. The cycle stops here.** No retrospective, no sweep, no PR
(`acceptance-and-verification.md §3a`, `epic-breakdown.md § AT-35-E7-001`).

- **HEAD scanned:** `8d0d4acbf2` (`docs(sd35): stamp AT-35-E6-004 cycle 2's landed SHA in its receipt, progress and kanban rows`)
- **Cycle-1 HEAD:** `24084e1782` — this cycle re-derives every figure at the new HEAD; nothing below is quoted from cycle 1
- **tranche/15 cut SHA:** `4c6c57eb9f`
- **Scanned:** 2026-09-14
- `SCOPE_GATE: EXEMPT (Epic 7 acceptance-scan cycle — closes zero units by design, decisions.md §2)`
- `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=0`

The full `verify.sh` was **not** run. `§3a` ends with **"If anything is short: STOP."**, and the
shortfalls below were derived before the build-scope step. Stating that plainly rather than
implying a green all-stage build ran.

---

## What is green at HEAD (re-derived, not quoted)

| Check | Command | Result at `8d0d4acbf2` |
|---|---|---|
| Completion atlas | `python3 scripts/completion_atlas.py --check` | `population=49438 buckets=10 unclassified=0 overlap=0`; `DONE: 49438`; A/B/C/D/M/V/U/X/Z all `0`; `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`; exit 0 |
| PCGen residue closure | `python3 scripts/pcgen_residue_gate.py --check --closure` | every pattern `files=0 hits=0`; all five live roots `files=0 hits=0`; `identifier_files=0 identifier_hits=0`; **`shipped_data_files=0 shipped_data_hits=0 shipped_scanned=11`**; `live_files=0 live_hits=0 verdict=PASS`; exit 0 |
| Token-coverage partition | `python3 scripts/token_coverage.py --check` | `verdict=PASS`; all six sub-checks `ok=True`; `units=49438 non_done=0 tokened=0 token_less=0 refused_non_done=0` |
| Converted rules carry no PCGen syntax | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | `0` |
| No carve-out in the closure instruments | `grep -rn 'EXCLUDED_\|EXCLUDE_\|SKIP_\|_ALLOWLIST\|ALLOW_LIST\|EXEMPT' scripts/{pcgen_residue_gate,completion_atlas,token_coverage,cycle_scope_gate,missing_engine_tables}.py` | Five hits, all in `pcgen_residue_gate.py`: `EXCLUDED_PREFIXES: tuple[str, ...] = ()` — **empty and pinned empty** — and `EXCLUDED_DIR_NAMES = {node_modules, dist, target, .git}`, build-output directories, not source paths. **No book list, no live-path allow-list.** |
| No active `## Open blockers` entry | `progress.md §Open blockers` | All three entries struck and ruled (B14 §17, B15 §18, B16 §19). Clean. |

### Cycle 1's S5 is RESOLVED at this HEAD

Cycle 1 reported two tracked, shipped desktop resources
(`resources/corpus_fixtures/equipment/equip_{longsword,chain_shirt}.json`) carrying a raw
`"raw_tokens"` array. Operator ruling **B17** (`decisions.md §20`) and the
`AT-35-E6-005-SHIPPED-DATA` cycle landed since. Re-derived independently here, not from the
gate's own pattern list:

```
python3 -c "import json;print(json.load(open('apps/desktop/src-tauri/tauri.conf.json'))['bundle']['resources'])"
-> ['resources/authoring_workbench/guard-stance-package/', 'resources/corpus_fixtures/',
    'resources/corpus_fixtures/spell/', 'resources/corpus_fixtures/equipment/',
    'resources/corpus_fixtures/_settled/']

cd apps/desktop/src-tauri && find resources -type f | wc -l            -> 11
grep -rlE 'raw_tokens|raw_bonus_chains|BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|%LIST|SAB:|DESC:|TYPE=' resources/ | wc -l  -> 0
```

Every one of the 11 shipped files is clean, and the gate's own `shipped_scanned=11` matches the
independently derived shipped population. The surviving token arrays now live in
`apps/desktop/src-tauri/fixtures_src/equipment/equip_{longsword,chain_shirt}.json`, which is
**not** in `bundle.resources` — a converter *input*, which `decisions.md §11` keeps.

### `§3a`'s independent grep — 33 lines, every one ruled out, none claimed as a shortfall

```
grep -rn 'raw_tokens\|PcgenFormulaEvaluator\|render_pcgen_desc' \
  src/rules_core src/saved_character src/campaign src/homebrew_authoring apps/desktop \
  | grep -v '/target/'   -> 33 lines
```

Each was classified by running the gate's own `cfg_test_ranges()` over the file, not by reading
the line:

| Lines | Character | Ruling |
|---|---|---|
| 24 | `//!` / `///` doc-comments and inline provenance prose in `src/rules_core/**` and `apps/desktop/src-tauri/src/{corpus_fixtures,race_trait_picker}.rs`, plus one commented-out `use` (`race_resolver.rs:102`) | **B14** (`decisions.md §17`) — a live-side doc-comment is not a read |
| 7 | Lines confirmed **inside** `#[cfg(test)]` regions by the gate's own `cfg_test_ranges()`, e.g. `trait_pool.rs:479` (`.get("raw_tokens")`, the retired-read oracle), `class_feature_grant_consumer.rs:1355` (`render_pcgen_desc_with_values`, the converter-parity oracle), `corpus_loader.rs:453` (a test fn name) | **B15** (`decisions.md §18`) — a `#[cfg(test)]` module is not live code |
| 2 | `fixtures_src/equipment/equip_{longsword,chain_shirt}.json` | Converter inputs, not in `bundle.resources` — kept by `decisions.md §11` |

The classification was run mechanically, not by eye: every line was bucketed by
`pcgen_residue_gate.cfg_test_ranges()` plus a comment-prefix test, and the **`OTHER` bucket came
back empty** — `33 {'cfg_test': 7, 'comment': 24, 'json_input': 2}`. There is no live executable
read in the residue.

Claiming any of these would be manufacturing a shortfall against a standing operator ruling,
which is the failure `§3a`'s closing line warns about.

---

## Shortfalls

### S1 — 55 of 99 kanban rows are not `complete`, including the criterion row for AT-35-E6-003

`§3a`: *"Every kanban row `complete` with its receipt path resolving."*
`epic-breakdown.md § AT-35-E7-001`: *"Every criterion `AT-35-E1-001` … `AT-35-E6-004` is
`complete` and every `kanban.md` card is `complete`. **There is no 'complete or filed under Open
blockers'.**"*

```
awk -F'|' '/^\| *[0-9]+ *\|/ {gsub(/^ +| +$/,"",$6); print $6}' \
  docs/release/SD-35-corpus-sheet-completion/kanban.md | sort | uniq -c
```

| Status | Rows at `8d0d4acbf2` | Rows at cycle 1's `24084e1782` |
|---|---|---|
| `complete` | **44** | 42 |
| `in-progress` | **51** | 51 |
| `blocked` | **2** (row 76 `AT-35-E6-003-FINISH` cycle 1; row 28 `AT-35-E7-001`) | 1 |
| `blocked-escalated` | **1** (row 59 `AT-35-E6-003-SWEEP` cycle 1) | 1 |
| `not-started` | **1** (row 29 `AT-35-E7-002 + AT-35-E7-003`) | 2 |
| **total** | **99** | 97 |

Two rows closed and two rows were added between the cycles; the non-`complete` population did not
fall. Row **26** — the criterion card `desktop-and-prose-leave-pcgen` / **`AT-35-E6-003`** — still
reads `in-progress`, and so do rows 43–46 (`AT-35-E6-002` cycles 2–5), 48–58, 60–75, 77–95. A
criterion row that is not `complete` is a failed precondition by the criterion's own words,
whatever the instruments read.

This may be bookkeeping lag behind the closed cycle rows. It is not this scan's job to decide
that, and it is explicitly not this scan's job to relabel 55 rows to make itself pass — `§5`
forbids a `status: complete` unsupported by the mechanical receipt rows, and the inverse edit is
the same move in reverse. Each non-`complete` row needs its owning cycle to close it, or an
operator ruling that cycle rows under a closed criterion do not carry their own status.

### S2 — `data/sheet_rules/_refused.json` is not empty: 142 entries

`§3a`: *"`sheet_rule_convert --check` at HEAD → ids agree with the corpus; `_refused.json` empty."*

```
python3 -c "import json;d=json.load(open('data/sheet_rules/_refused.json'));print(d['records'],d['converted'],d['refused'],d['by_token_type'],len(d['entries']))"
-> 49438 49296 142 {'no_corpus_record': 142} 142
```

142 ids under a single token type, `no_corpus_record`. Every one is `DONE` in the atlas by an
earlier route (`refused_non_done=0`), so no unit is un-rendered — but the deliverable-integrity
check names the **file**, and the file is not empty. Unchanged from cycle 1.

### S3 — `token_coverage.py --check` reports 142 refused units

`§3a`: *"`token_coverage.py --check` at HEAD → **zero refused units**."*

```
python3 scripts/token_coverage.py --check
-> units=49438 non_done=0 census_records=49438 token_types=233 unmapped_token_types=2
   refused=142 refused_non_done=0 shapes=1 token_less_non_done=0 … verdict=PASS
```

Same population as S2. The gate's verdict is `PASS` because its partition balances; the
acceptance bar is stricter than the gate and reads `refused=142`, not zero. Unchanged from
cycle 1.

### S4 — 83 open deferrals stand at HEAD

`§3` item 8: *"Enumerate open deferrals. None may defer DoD scope."*

```
python3 scripts/retro.py summary --since 2026-09-07 --json   ->  deferrals.open = 83
```

By actor: `AT-35-E6-003-RULED` 17, `AT-35-E6-003` 16, `AT-35-E6-003-SWEEP` 12 (+6 lower-cased
`at-35-e6-003-sweep`), `AT-35-E2-005` 5, `AT-35-E6-002` 5, then `AT-35-E2-005-DISPOSITION`,
`AT-35-E3-001`, `AT-35-E5-005` and `AT-35-E6-003-FINISH` at 2 each, and 14 actors at 1 each.
Unchanged in count from cycle 1. At least one defers DoD scope on its face: kanban row 41's
`AT-35-E5-005` cycle 2 records *"the 10-unit residue is unchanged and still handed on, not
exempted"* (`1789064249278-at-35-e5-005-579de8`, re-affirming
`1788994100821-at-35-e5-005-5973cb`). Each open deferral must be closed, or shown to defer only
post-DoD capability scope (`docs/governance/deferral-revisit-doctrine.md`).

---

## Scan steps performed

| `§3` step | Done | Note |
|---|---|---|
| 1. Derive SETS, not sizes | partial | Atlas id-population re-derived at HEAD (49,438 = 49,438 `DONE`); the launch-inventory id-set diff was not reached before the stop |
| 2. Re-run every headline command | yes | Atlas, residue closure, token coverage, `_refused.json`, the `data/sheet_rules/` token grep, the shipped-resource grep, the deferral summary — all re-run at `8d0d4acbf2`, none quoted |
| 3. Read commit diffs | partial | The three commits between `24084e1782` and HEAD read by `--name-status`; the `AT-35-E6-005-SHIPPED-DATA` result re-derived independently rather than read from its receipt |
| 4. Re-derive failure attribution from `git` vs `4c6c57eb9f` | partial | No gate-weakening commit on `scripts/pcgen_residue_gate.py` or `scripts/pcgen-residue-baseline.env`; the full attribution pass was not reached |
| 5. Grep closure instruments for exclusion lists | yes | `EXCLUDED_PREFIXES` empty and pinned empty; `EXCLUDED_DIR_NAMES` is build output only. No carve-out. |
| 6. Widest build scope / full `verify.sh` | **no** | Stopped per `§3a`; see the note at the top |
| 7. Read `## Open blockers` | yes | All three entries struck and ruled. **No active blocker.** |
| 8. Enumerate open deferrals | yes | **83 open** — S4 |
| 9. Re-prove the gates still fail | **no** | Not reached |
| 10. Every receipt carries a scope-gate line | **no** | Not reached |
| 11. Every build-scope row names its SHA | **no** | Not reached |

---

## What has to happen before this scan can be re-run

1. Close, or obtain a ruling on, the 51 `in-progress` + 2 `blocked` + 1 `blocked-escalated` kanban
   rows — **row 26 (`AT-35-E6-003`) first**, since it is a criterion row.
2. Empty `data/sheet_rules/_refused.json` (142 `no_corpus_record` ids), or obtain a ruling amending
   `§3a`'s "empty" to "zero non-`DONE` refusals". The same ruling disposes of S3.
3. Dispose of the 83 open deferrals: closed, or each shown not to defer DoD scope.
4. Then re-run `AT-35-E7-001` in full, including the whole `verify.sh` and steps 9–11.
