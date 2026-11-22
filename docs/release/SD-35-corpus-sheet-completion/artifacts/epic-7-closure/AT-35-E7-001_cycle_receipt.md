# AT-35-E7-001 — final-acceptance scan — cycle 1 receipt

**Verdict: FAIL. The cycle stops here.** No retrospective, no sweep, no PR
(`acceptance-and-verification.md §3a`, `epic-breakdown.md § AT-35-E7-001`).

- **HEAD scanned:** `24084e1782` (`docs(sd35): stamp AT-35-E6-WRAPUP-FIX cycle 1's landed SHAs …`)
- **tranche/15 cut SHA:** `4c6c57eb9f`
- **Scanned:** 2026-09-14
- `SCOPE_GATE: EXEMPT (Epic 7 acceptance-scan cycle — closes zero units by design, decisions.md §2)`
- `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=0`

The full `verify.sh` was **not** run. `§3a` ends with **"If anything is short: STOP."** and the
shortfalls below were derived before the build-scope step; a 40-minute all-stage run cannot change
a verdict already fixed by them. Naming that plainly rather than implying a green build ran.

---

## What is green at HEAD (re-derived, not quoted)

| Check | Command | Result |
|---|---|---|
| Completion atlas | `python3 scripts/completion_atlas.py --check` | `population=49438 buckets=10 unclassified=0 overlap=0`, `DONE: 49438`, A/B/C/D/M/V/U/X/Z all `0`, `done_evidence_violations=0`, `missing_clearing_mechanisms=0`, `stale_derived_at=False`, `citation_failures=0` |
| PCGen residue closure | `python3 scripts/pcgen_residue_gate.py --check --closure` | `live_files=0 live_hits=0 verdict=PASS`; every pattern `files=0 hits=0`; all five live roots `files=0 hits=0` |
| Token coverage partition | `python3 scripts/token_coverage.py --check` | `verdict=PASS`; all six sub-checks `ok=True`; `non_done=0 refused_non_done=0` |
| No exclusion list in the residue gate | `git show 72103a69bc -- scripts/pcgen_residue_gate.py` | The commit that first read zero adds `mask_non_code()` (a comment/string-literal mask for brace matching) and **no** path allow-list, no `EXCLUDED_*`, no widened exemption. It *returned* 620 previously-blanked shipping lines to measurement and the count stayed at zero. This is the carve-out `§3` item 5 most feared, and it is not present here. |

Epic 6's substance genuinely reached zero: `AT-35-E6-003-RULED` cycle 17 (`fd5e4ddc69`) left
`4 / 4`, and cycle 18 (`7270e4a2d7` + `c094391246`, kanban row 96, `complete`) closed it by
settling the class-feature pool gates at ingest and splitting the canonical envelope's payload —
code work, not an instrument correction. **That is not the reason this scan fails.**

---

## Shortfalls

### S1 — 53 of 97 kanban rows are not `complete`, including the criterion row for AT-35-E6-003

`§3a`: *"Every kanban row `complete` with its receipt path resolving."*
`epic-breakdown.md § AT-35-E7-001`: *"Every criterion `AT-35-E1-001` … `AT-35-E6-004` is
`complete` and every `kanban.md` card is `complete`. **There is no 'complete or filed under Open
blockers'.**"*

```
awk -F'|' '/^\| *[0-9]+ *\|/ {gsub(/^ +| +$/,"",$6); print $6}' \
  docs/release/SD-35-corpus-sheet-completion/kanban.md | sort | uniq -c
```

| Status | Rows |
|---|---|
| `complete` | 42 |
| `in-progress` | 51 |
| `blocked` | 1 (row 76, `AT-35-E6-003-FINISH` cycle 1) |
| `blocked-escalated` | 1 (row 59, `AT-35-E6-003-SWEEP` cycle 1) |
| `not-started` | 2 (rows 28/29 — `AT-35-E7-001`/`E7-002+003`, this cycle and its successor) |

`progress.md`'s own **status matrix agrees independently**: Epic 6 reads `4 | 2 | 1 | 1`
(criteria / complete / in progress / not started) and the total reads **25 of 30 complete**.

Row **26** — the criterion card `desktop-and-prose-leave-pcgen` / **`AT-35-E6-003`** itself —
reads `in-progress`. So do rows 43–46 (`AT-35-E6-002` cycles 2–5) and rows 48–58, 60–75, 77–95
(`AT-35-E6-003` and its `-SWEEP` / `-FINISH` / `-RULED` cycles). A criterion row that is not
`complete` is a failed precondition by the criterion's own words, regardless of what the
instruments read.

This *may* be pure bookkeeping lag behind row 96 and `AT-35-E6-004` (row 27, `complete`). It is
not this scan's job to decide that, and it is explicitly not this scan's job to re-label 51 rows
to make itself pass — `§5`: *"A lane's `status: complete` unsupported by the mechanical receipt
rows"* does not satisfy a criterion, and the inverse edit is the same move in reverse. Each
non-`complete` row needs its owning cycle to close it, or an operator ruling that the cycle rows
under a closed criterion do not carry their own status.

### S2 — `data/sheet_rules/_refused.json` is not empty: 142 entries

`§3a`: *"**`sheet_rule_convert --check` at HEAD → ids agree with the corpus; `_refused.json`
empty.**"*

```
python3 -c "import json;d=json.load(open('data/sheet_rules/_refused.json'));print(d['records'],d['converted'],d['refused'],d['by_token_type'])"
-> 49438 49296 142 {'no_corpus_record': 142}
```

142 ids refused under a single token type, `no_corpus_record` (e.g.
`advanced_race_guide:race:dhampir`, `…:drow`, `…:duergar`, `…:dwarf`). Every one of the 142 is
`DONE` in the atlas by an earlier route (`refused_non_done=0`), so no unit is un-rendered — but
the deliverable-integrity check names the **file**, not the unit disposition, and the file is not
empty.

### S3 — `token_coverage.py --check` reports 142 refused units

`§3a`: *"**`token_coverage.py --check` at HEAD → zero refused units.**"*

```
python3 scripts/token_coverage.py --check
-> refused=142 refused_non_done=0 … verdict=PASS
```

Same population as S2. The gate's own verdict is `PASS` because its partition checks balance;
the acceptance bar is stricter than the gate and reads `refused=142`, not zero.

### S4 — 83 open deferrals stand at HEAD

`§3` item 8: *"Enumerate open deferrals. None may defer DoD scope."*

```
python3 scripts/retro.py summary --since 2026-09-07 --json  ->  deferrals.open = 83
```

By actor: `AT-35-E6-003-RULED` 17, `AT-35-E6-003` 16, `AT-35-E6-003-SWEEP` 12 (+6 lower-cased),
`AT-35-E2-005` 5, `AT-35-E6-002` 5, `AT-35-E6-003-FINISH` 2, `AT-35-E3-001` 2,
`AT-35-E5-005` 2, `AT-35-E2-005-DISPOSITION` 2, and 14 single-deferral actors.

At least one defers DoD scope on its face: kanban row 41's `AT-35-E5-005` cycle 2 records *"the
10-unit residue is unchanged and still handed on, not exempted"* under deferral
`1789064249278-at-35-e5-005-579de8`, re-affirming `1788994100821-at-35-e5-005-5973cb`. 83 is a
population no closure scan can wave through; each needs to be closed, or shown to defer only
post-DoD capability scope (`docs/governance/deferral-revisit-doctrine.md`).

### S5 — the independent grep `§3a` mandates returns 39 lines, not none

`§3a`: *"Then an independent grep by the scan itself, not the gate's own pattern list:
`grep -rn 'raw_tokens\|PcgenFormulaEvaluator\|render_pcgen_desc' src/rules_core src/saved_character src/campaign src/homebrew_authoring apps/desktop` → **no output**."*

39 lines. Attribution by file:

| Lines | Path | Character |
|---|---|---|
| 7 | `apps/desktop/src-tauri/target/debug/deps/*.d` | Build artifacts in a git-ignored `target/` dir — noise the literal grep picks up, not shipping code |
| 2 | `apps/desktop/src-tauri/resources/corpus_fixtures/equipment/equip_{longsword,chain_shirt}.json` | **Tracked, shipped desktop resources carrying a `"raw_tokens"` array in their payload.** The one hit here that is neither prose nor a build artifact. |
| 28 | `src/rules_core/**`, `apps/desktop/src-tauri/src/race_trait_picker.rs` | Doc-comments (`//!`, `///`), inline provenance prose, and `#[cfg(test)]` oracle code — governed by operator rulings **B14** (`decisions.md §17`: a live-side doc-comment is not a read) and **B15** (`§18`: a `#[cfg(test)]` module is not live code) |

The 28 are ruled-out by B14/B15 and are **not** claimed as a shortfall here — manufacturing one
against a standing operator ruling is the failure mode `§3a`'s closing line warns about. The
**2 shipped fixture JSON files** are: `apps/desktop` is a live root, those files ship inside the
desktop bundle's `resources/`, and they carry raw PCGen ingest tokens. Either they are converted /
settled the way `AT-35-E6-003-RULED` cycle 15 settled `data/corpus/**/_settled/`, or the check's
wording needs an operator ruling that a corpus fixture payload is data, not a read. The gate does
not see them (`live_files=0`) — which is precisely why `§3a` asks the scan to grep independently.

---

## Scan steps performed

| `§3` step | Done | Note |
|---|---|---|
| 1. Derive SETS, not sizes | partial | Atlas id-population re-derived at HEAD (49,438); the launch-inventory id-set diff was not reached before the stop |
| 2. Re-run every headline command | yes | Atlas, residue closure, token coverage, `_refused.json` — all re-run here, none quoted |
| 3. Read commit diffs | yes | `72103a69bc` read in full for the residue gate; `fd5e4ddc69..72103a69bc..HEAD` log read to attribute the 4/4 → 0 fall to cycle 18's code work, not to an edited instrument |
| 4. Re-derive failure attribution from `git` vs `4c6c57eb9f` | yes | `git log 4c6c57eb9f..HEAD -- scripts/pcgen_residue_gate.py scripts/pcgen-residue-baseline.env` → 5 commits, each a named criterion cycle; no silent baseline edit |
| 5. Grep closure instruments for exclusion lists | yes | No `EXCLUDED_*` / path allow-list added to `pcgen_residue_gate.py` on the tranche; row 25's receipt records `EXCLUDED_PREFIXES` **emptied and pinned empty** with 15 unit tests |
| 6. Widest build scope / full `verify.sh` | **no** | Stopped per `§3a`. See the note at the top of this receipt. |
| 7. Read `## Open blockers` | yes | All three entries in `progress.md` are struck through and resolved (B14 §17, B15 §18, B16 §19). **No active blocker entry** — this is the one `§3` step that is clean. |
| 8. Enumerate open deferrals | yes | **83 open** — S4 |
| 9. Re-prove the gates still fail | **no** | Not reached |
| 10. Every receipt carries a scope-gate line | **no** | Not reached |
| 11. Every build-scope row names its SHA | **no** | Not reached |

---

## What has to happen before this scan can be re-run

1. Close, or obtain a ruling on, the 51 `in-progress` + 1 `blocked` + 1 `blocked-escalated` kanban
   rows — **row 26 (`AT-35-E6-003`) first**, since it is a criterion row.
2. Empty `data/sheet_rules/_refused.json` (142 `no_corpus_record` ids), or obtain a ruling amending
   `§3a`'s "empty" to "zero non-DONE refusals".
3. Dispose of the 83 open deferrals: closed, or each shown not to defer DoD scope.
4. Settle or rule on the two shipped desktop corpus fixtures carrying `raw_tokens`.
5. Then re-run AT-35-E7-001 in full, including the whole `verify.sh` and steps 9–11.
