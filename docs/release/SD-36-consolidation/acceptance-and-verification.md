---
canonical: true
owner: operator
bundle_id: SD-36
date: 2026-09-15
---

# SD-36 Acceptance and Verification

End-to-end verification gates and measured-baseline table.

---

## Verification (end to end)

1. **Hotfix:** merged to develop; CI run green.
   - Command: `gh run list --limit 3 --jq '.[] | select(.name == "Publish tester release") | .conclusion'` → "success"

2. **Per epic:** compile gate, `--list` diff artifact, ONE `bash scripts/verify.sh` run.
   - Compile gate: `cargo test --locked --no-run` at root and `apps/desktop/src-tauri` succeeds.
   - `--list` artifact: `cargo test --locked -- --list > tests-before.txt` (pre-epic), same after → diff shows only module-segment normalisation or expected additions/removals.
   - Full gate: `bash scripts/verify.sh` with **stage list accurate** (B drops 6, A adds 2, C2 reorders tests). Every `test result: FAILED` line attributed to its `Running` suite line in the transcript.

3. **Desktop:** browser-side tests and type-checking.
   - Commands: `cargo test --locked --no-run -j 6` in `apps/desktop/src-tauri`, `npm run typecheck`, `npm test` (run inside `verify.sh`, or manually if debugging).

4. **Wall:** build enforcement + gate.
   - `cargo build --locked -p codex` succeeds with `crates/codex-ingest/src` temporarily renamed (one-time manual proof: proves codex builds without ingest).
   - `bash scripts/verify.sh --list | grep crate-wall` → stage found; run it: `bash scripts/verify.sh` green on the `crate-wall` stage.
   - Residue gate: `python3 scripts/pcgen_residue_gate.py --check --closure` exits zero; output shows `lst_file files=0 hits=0` and `codex_ingest files=0 hits=0`.

5. **Snapshot:** frozen-status gate.
   - Command: `scripts/site/check_frozen_status.py --check` exits zero on `site/status-data.json`.
   - Verify: `python3 -c "import json;d=json.load(open('site/status-data.json'));print(f\"overall: {d['overall']['pct']}%, {d['overall']['denominator']} units\")"` → "overall: 100.0%, 49450 units".
   - Verify: `site/status.html` still has "Planned / other systems" rows (static HTML unchanged for non-PF1e entries).

6. **Counts:** path helpers, deleted files, helpers consolidated.
   - `git grep -w 'fn repo_root' | wc -l` → 1 (only in `src/support/paths.rs`).
   - `git grep -w 'fn corpus_root' | wc -l` → 1.
   - `git grep -w 'fn find_json_files' | wc -l` → 1.
   - Deleted file check: `wc -l src/bin/v06_work_inventory.rs` exits nonzero (file gone).
   - Baseline table re-derived into `release-notes.md` with each row's command.

7. **Box:** disk reclaimed, crontab cleaned.
   - `df -h /` shows ≥ 850G free.
   - `crontab -l | wc -l` = baseline - 3 (three cron lines removed).
   - `crontab -l | grep 'reclaim.sh'` (1+ line, still present).

---

## Measured baseline (re-derive at each pass; write into release notes with the command)

| figure | value (SD-35) | command | target (SD-36) |
|---|---|---|---|
| src lines | 465,469 | `find src -name '*.rs' -exec cat {} + \| wc -l` | <derive-at-epic-b> |
| tests lines / files | 182,070 / 552 | `find tests -name '*.rs' -exec cat {} + \| wc -l; find tests -name '*.rs' \| wc -l` | <derive-at-epic-c2> |
| pilot_compute/mod.rs | 88,828 | `wc -l src/rules_core/pilot_compute/mod.rs` | <4,000 (split across submodules) |
| v06_work_inventory.rs | 33,091 | `wc -l src/bin/v06_work_inventory.rs` | 0 (deleted, Epic B) |
| support_state_matrix.rs | 7,475 | `wc -l src/rules_core/support_state_matrix.rs` | 0 (deleted, Epic B) |
| reach_gate.rs | 8,846 | `wc -l apps/desktop/src-tauri/src/reach_gate.rs` | 0 (deleted, Epic B) |
| test entries | 8,723 | `cargo test --locked -- --list` + list-diff | <same count, table-driven rewrite only> |
| root full tests / binaries / desktop tests | 8,926 / 419 / 570 | `scripts/verify-baselines.env` last stanza | 8,9?? / 412 / 570 (B removes 7 bins; A removes 47 bins) |
| live PCGen residue | 0 files / 0 hits | `python3 scripts/pcgen_residue_gate.py --check --closure` | 0 / 0 (A enforces via crate wall) |
| public status | 95.0% of 37,892 | (SD-35 pre-freeze) | 100.0% of 49,450 (B freezes at 100%) |

---

