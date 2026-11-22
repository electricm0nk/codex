# Cycle — Epic 6 Closure Epilogue / AT-34-E6-001 (GATE LANE B, wave 27) — `site-dashboard-check` fixed: a timeout now fails loudly instead of silently serving a stale cache

- **Commit SHA:** `a893bfcb39` (pushed to `tranche/14`; pre-push local commit `b7c742ca23`, superseded by the rebase onto lane A's concurrently-landed wave-30 work — no conflicts, no content change across the rebase)
- **Files touched:** `scripts/observer/pf1e_dashboard_producer.py`, `scripts/publish-site-dashboard.sh`, `scripts/verify.sh`, `scripts/tests/test_pf1e_dashboard_producer.py`, `scripts/tests/test_publish_site_dashboard.sh`
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` on this cycle's own uncommitted diff, isolated (`git diff -- <5 files above>` before commit) — no hit. The whole-cycle `scripts/` diff since the tranche cut (`ea2b3396f2...HEAD`) does carry 6 hits, all `SD34_BUNDLE_DIR` — a real, pre-existing constant name from `AT-34-E1-006` (`6490738c38`, confirmed by `git log -S`), landed before this cycle touched anything, not introduced here.
- **Wired-integration audit result:** `OK_NO_TOKENS` both on this cycle's isolated diff and on the whole-cycle `scripts/` diff since the tranche cut — zero hits either way.
- **Acceptance criterion (verbatim, dispatch brief):** "GATE LANE B — site-dashboard-check: make the timeout fail loudly instead of lying... Step 4 is the actual defect and it is worse than a red stage: a gate that passes on stale data tells you the site is current when it is not. Fix that, not the number. A timeout should be a loud failure — the stage then either has fresh data or says plainly that it could not get it. Raising the cap remains forbidden as a substitute for fixing the silent fallback. If, once the failure is loud, the honest conclusion is that 600s is simply the wrong bound for a 757s job, you may set it deliberately — with the measurement, the margin you chose, and why, in the receipt. **Territory this wave INCLUDES `scripts/`**."

## What was actually wrong, confirmed by reading the code myself (not just trusting the wave-26 receipt's chain)

1. `verify.sh`'s `run_site_dashboard_check` had no outer `timeout` wrapper around the whole stage — confirmed at (pre-fix) lines 600-624.
2. `publish-site-dashboard.sh --check` runs the real producer (`python3 "$PRODUCER" --out "$TMP"`) even in check mode, into a scratch dir only — confirmed at (pre-fix) line 84.
3. The producer bounds each of its three state-dump binaries (`v06_class_state_dump`, `v06_content_state_dump`, `v06_work_inventory`) with the shared `CLASS_STATE_BUILD_TIMEOUT_SECONDS` (`PF1E_CLASS_STATE_TIMEOUT`, default 600s) — confirmed at `pf1e_dashboard_producer.py:499-559` (pre-fix `_run_state_dump`).
4. **On a `subprocess.TimeoutExpired`, `_run_state_dump` printed a message and returned `None` unconditionally**, and `_load_cached_dump` (pre-fix lines 562-598) treated `None` exactly like every other soft failure: `if produced is None: return cached` — silently serving whatever stale cache sat on disk, with no distinction between "cache is fine, nothing changed" and "the build never finished, we have no idea." `--check` then compares two outputs that can both trace back to the same stale cache and reports the feed **current** when it may not be.

Step 4 is what I fixed — not the 600s number by itself.

## The fix

- **`pf1e_dashboard_producer.py`:** added `StateDumpTimeout(RuntimeError)` and `PF1E_DASHBOARD_STRICT_TIMEOUT=1` (checked by a new `_strict_timeout_mode()`). `_run_state_dump` now catches `subprocess.TimeoutExpired` on its own (previously folded into the generic `except (OSError, subprocess.SubprocessError)`), and **raises** `StateDumpTimeout` when strict mode is on, instead of returning `None`. `_load_cached_dump` does not catch `RuntimeError`, so the raise propagates straight past its own stale-cache fallback — the exact line that was silently swallowing the timeout before. Strict mode is **off** by default, so a live regeneration (cron, or an interactive `./scripts/publish-site-dashboard.sh`) keeps the original stale-cache-preferred behavior unchanged, on purpose — a blank public panel is worse than a stale one, and that design choice was correct all along; only `--check` needed the opposite bias. `main()`'s `if __name__ == "__main__":` block now catches `StateDumpTimeout` and exits 3 with a clean one-line stderr message instead of an unhandled traceback.
- **`publish-site-dashboard.sh`:** the `--check` branch's producer invocation now runs as `PF1E_DASHBOARD_STRICT_TIMEOUT=1 python3 "$PRODUCER" --out "$TMP"`. The live-regen `else` branch is untouched — strict mode is opt-in, scoped to the one caller that needs it.
- **Also split `v06_work_inventory` onto its own timeout**, `WORK_INVENTORY_BUILD_TIMEOUT_SECONDS` (`PF1E_WORK_INVENTORY_TIMEOUT`, default **950s**), separate from the shared 600s cap the two cheaper dumps keep. This is the deliberate act the brief explicitly permits once the failure path is loud, not a substitute for it — done in the same commit as the loud-failure fix, not instead of it. **Measurement behind 950s:** wave-26's own receipt measured `v06_work_inventory --summary` at **757.01s wall time on a confirmed-quiet box** (44Gi free, load 2.80/24, zero other cargo processes); this cycle did not re-run that measurement (forbidden — "do NOT run the inventory regenerator or the dashboard producer from a lane" — and `v06_work_inventory` itself is the read-only carve-out, but re-running it here would still cost ~13 minutes for no new information), and confirmed the code on both sides of that number (the binary being timed, and `CLASS_STATE_BUILD_TIMEOUT_SECONDS`'s own definition) is unchanged since wave-26's inspection by `git log --oneline -- scripts/observer/pf1e_dashboard_producer.py` between wave-26's HEAD and this cycle's pre-fix HEAD — no commit touched it. **Margin chosen:** 950s = 757s + ~193s (~25%), rounded to a clean number — wide enough to absorb ordinary variance on a quiet box without being a rubber stamp; not measured against a *loaded* box, so this is a considered estimate, not a proof, and is named as such here rather than asserted as exact.
- **`verify.sh`:** `run_site_dashboard_check` now wraps the whole stage in `timeout "${SITE_DASHBOARD_CHECK_TIMEOUT_S:-2400}s"` — same `${VAR:-default}` shape `corpus-trap-audit` already uses (whose own comment names this exact stage as the reason it got a wrapper first). 2400s default is a documented ceiling (600+600+950 worst-case-cold sum ≈ 2150s, plus slack for the public-status projection's own `--check` and process overhead), not remeasured end-to-end this cycle for the same reason above. Status 124 (stage-level timeout) and status 3 (producer's own `StateDumpTimeout`) each get a distinct, named `stage_fail` message — neither one is silent.

## Standing gates re-checked this cycle

- `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` → `files_checked=16 violations=0`.
- `cargo run --locked --quiet -j 6 --bin corpus_literal_sweep` (root workspace, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001-root`): `48706 records examined of 51476 read, ... 0 findings` / `CLEAN`. **Repo-vs-brief note:** the dispatch brief's "facts you need" section quotes 48,699/51,473 as the baseline; the live figure at this cycle's HEAD is **48706/51476**, matching wave-26's own already-re-derived figure exactly — the repo wins per the brief's own instruction, and this is the wave-26-confirmed baseline, unmoved by this cycle (0 `data/corpus/**` records touched, `decisions.md §12` L8 satisfied: delta 0, records added 0).

## Row-count command output (this cycle's own artifact: the new test coverage proving the fix)

```
$ cd scripts/tests && python3 -m unittest test_pf1e_dashboard_producer.StateDumpTimeoutIsLoudUnderStrictModeTest -v
test_load_cached_dump_falls_back_to_cache_on_timeout_when_not_strict ... ok
test_load_cached_dump_raises_under_strict_mode_on_timeout ... ok
test_load_work_inventory_uses_its_own_wider_timeout ... ok
test_run_state_dump_raises_state_dump_timeout_when_strict ... ok
test_run_state_dump_returns_none_on_timeout_when_not_strict ... ok

Ran 5 tests in 0.015s

OK
```

5 of 5 new cases, matching `grep -c '^    def test_' <(sed -n '/class StateDumpTimeoutIsLoudUnderStrictModeTest/,/^if __name__/p' test_pf1e_dashboard_producer.py)` = 5.

**Full producer suite (unchanged cases + these 5):** `python3 -m unittest test_pf1e_dashboard_producer -v` → `Ran 26 tests ... OK` (21 pre-existing + 5 new).

**`publish-site-dashboard.sh` self-test (2 new plumbing cases added):** `bash scripts/tests/test_publish_site_dashboard.sh` → `passed: 8  failed: 0` / `SELF-TEST PASSED.` (6 pre-existing + 2 new: "--check sets PF1E_DASHBOARD_STRICT_TIMEOUT=1 for the producer", "a real (non---check) run leaves PF1E_DASHBOARD_STRICT_TIMEOUT unset").

## TDD discipline

**RED confirmed for the intended reason, live, by mutation** — not asserted: temporarily removed the `if _strict_timeout_mode(): raise StateDumpTimeout(msg) from exc` line from `_run_state_dump` (restoring the old unconditional-`None`-on-timeout shape) and re-ran the 5-case class:

```
FAIL: test_load_cached_dump_raises_under_strict_mode_on_timeout — StateDumpTimeout not raised
FAIL: test_run_state_dump_raises_state_dump_timeout_when_strict — StateDumpTimeout not raised
Ran 5 tests in 0.019s
FAILED (failures=2)
```

Exactly the two cases that assert strict-mode behavior went red (for the right reason — the assertion itself, not a setup error); the other three (non-strict fallback, and the work-inventory wider-timeout wiring, which is independent of strict mode) stayed green, isolating the mutation's effect precisely. Restored the fix from a pre-edit backup copy and re-ran: 5/5 green again. Same discipline applied to `test_publish_site_dashboard.sh`'s two new cases: reverted the `PF1E_DASHBOARD_STRICT_TIMEOUT=1` prefix in `publish-site-dashboard.sh`'s `--check` branch, re-ran → case 7 ("--check sets...") went red with `STRICT_TIMEOUT_ENV=<unset>` in its output (the exact wrong value, not a crash), case 8 stayed green (a real run was never supposed to set it, mutation didn't touch that path); restored, re-ran, 8/8 green.

- **Build scope verified:**
  - `cargo test --locked --no-run -j 6` (whole workspace, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001-root`): **exit 0**, run at this cycle's pre-push local HEAD `b7c742ca23` (content-identical to the pushed `a893bfcb39` — the rebase onto lane A's wave-30 work touched no files this cycle shares, confirmed by `git diff --cached --numstat` showing only this cycle's own 5 files before the push, and the rebase log reporting no conflicts).
  - `apps/desktop/src-tauri` (separate cargo workspace): `cargo test --locked --no-run -j 6 --manifest-path apps/desktop/src-tauri/Cargo.toml` (`CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001-desktop`): **exit 0**. Full `cargo test --locked` (not just `--no-run`) NOT run this cycle — this lane's own diff touches zero desktop files (`scripts/` only), and wave-27 lane B's own sibling desktop-fix cycle already closed `desktop` GREEN (572/0) two commits prior on this same branch; re-running the full ~95s suite for a diff that cannot move it would not add information.
- **Sweep population:** `corpus_literal_sweep` 48706 → 48706 (unmoved; 0 `data/corpus/**` records added or removed this cycle).
- **Oracle pin:** N/A — no figure in this receipt is sourced from the pinned PCGen oracle corpus.
- **Status:** complete
- **Movement, four buckets:**
  - **Closure:** the `site-dashboard-check` **defect** (silent stale-cache fallback masking a timeout as "current") is closed — genuinely fixed in code, tested, mutation-proven. This is **not** the same claim as "the `site-dashboard-check` verify.sh stage is green" — this lane did not (and per the brief's own hazard note, may not) run the real producer end-to-end to observe the stage's live PASS/FAIL outcome; that requires either the operator running `./scripts/publish-site-dashboard.sh` interactively past its now-950s work-inventory bound, or a full `verify.sh` FULL run with real cargo/corpus access from a context permitted to do so.
  - **Reclassification:** none this cycle.
  - **Reachability:** none this cycle.
  - **Instrument-correction:** `PF1E_CLASS_STATE_TIMEOUT`'s shared 600s cap no longer governs `v06_work_inventory`; it now has its own, wider, measured `PF1E_WORK_INVENTORY_TIMEOUT` (950s) — a deliberate correction to the bound, made only after (and alongside, in the same commit as) the loud-failure fix, per the brief's own permission and the wave-26 receipt's own naming of this as the next mechanical step.
- **Notes:** The brief's population figures (48,699/51,473) were stale by wave-26's own already-recorded re-derivation (48,706/51,476); this receipt states the repo's live figure and notes the disagreement rather than quoting either silently, per `decisions.md §12` L2/L14 and `every-figure-states-its-denominator`. This cycle did not attempt to run the real producer or dashboard regenerator in any mode, matching every prior lane's precedent (wave-24, wave-26, wave-26-settle) for the same reason: the brief's hazard note forbids it, and the fix here is provably correct at the unit level (mocked timeouts, mutation-proven) without needing a 13+-minute live run this lane is not permitted to make. `kanban.md` row 26 (`final-acceptance-scan`) intentionally **not** touched this cycle, matching every prior gate-remediation sub-wave's own precedent (no board row tracks individual sub-waves).
- **Next-cycle plan:** the whole-gate re-measure (lane C's "sweep and report honestly" obligation) should now find `site-dashboard-check` either (a) genuinely PASS, if a live run completes within the new bounds and confirms currency, or (b) a **new, loud, named** FAIL/timeout message distinct from the old silent-current report — both are correct outcomes of this fix; a report of "current" that cannot be traced to a real, non-timed-out producer run would itself be a regression back to the defect this cycle closed. Whoever runs that live check next should also verify no other caller of `_load_cached_dump`/`load_engine_content_state`/`load_engine_class_states` relies on the old unconditional-fallback shape being silent under strict mode — none currently pass `PF1E_DASHBOARD_STRICT_TIMEOUT=1` except `--check`, so this is not expected to surface anywhere else, but worth confirming if a future cycle adds a new strict-mode caller.
