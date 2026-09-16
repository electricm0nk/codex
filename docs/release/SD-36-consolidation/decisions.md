---
canonical: true
owner: operator
bundle_id: SD-36
date: 2026-09-15
---

# SD-36 Decisions

Bundle-specific ADRs. Operator rulings from 2026-09-15 planning session (all verbatim-binding). Every ruling names the command or artifact that enforces it.

---

## §1 — Build version 0.16.0

One version-bump commit across all surfaces: `.github/workflows/publish-tester-release.yml` stamp line, `apps/desktop/package.json`, `apps/desktop/src-tauri/Cargo.toml` + `Cargo.lock`, `apps/desktop/src-tauri/tauri.conf.json`, and 7 desktop test fixtures. Root `Cargo.toml` stays 0.1.0.

**Enforced by:** the commit itself; verify with `git log -1 --name-only` listing exactly those files (root Cargo.toml excluded).

---

## §2 — PCGen tool side KEPT, walled off in own crate

**Operator ruling, 2026-09-15:**

> *"PCGen tool side (converter + oracle) is KEPT for Starfinder but walled off in its own crate so the desktop can never link it. Close the two gate blind spots."*

**Decision.** Move PCGen machinery (converter, oracle validation, tool bins, tool tests, ~82 suites) into dedicated `crates/codex-ingest`. Root workspace gains `[workspace] members = ["crates/*"]`. Desktop lists `codex-ingest` under `[dev-dependencies]` ONLY, never in direct deps. Build gate enforces this:

1. `cargo tree --locked -e normal,build` in desktop shows zero `codex-ingest` lines (build deps only).
2. `codex-ingest` depends on `codex`; never the reverse (no circular dependency).
3. `python3 scripts/pcgen_residue_gate.py --check` exits zero.

The type-identity trap is a real hazard: `codex` compiles once to `lib`, once to `bin`, and if a `dev-dependency` on `codex-ingest` forces `codex` to recompile with a different `CARGO_MANIFEST_DIR`, the ingest module sees a different symbol. The build gate detects this.

**Enforced by:**
- `.github/workflows/publish-tester-release.yml` and `tranche-3-ci.yml:79` `cargo test --locked --workspace`.
- `scripts/verify.sh` new stage `crate-wall` (both stage sets): `cargo tree`, `cargo metadata`, awk manifest check, residue gate.
- `workflow-instruction.md §6` step 1: pre-flight gate checks.

---

## §3 — Residue gate: `lst_file` identifier, no `\.lst\b`

**Operator ruling, 2026-09-15:**

> *"Residue gate: add the `lst_file` identifier pattern now and rename the field; fix the one player-visible `.lst` message with a test. Do NOT add a `\.lst\b` pattern; the 8,592 table-citation strings are provenance and burn down when rules tables become a data package (Starfinder bundle row)."*

**Decision.** The residue gate (`scripts/pcgen_residue_gate.py`) adds `lst_file` to `IDENTIFIER_PATTERNS` to catch any lingering field references post-rename. Rename `SourceRef.lst_file` → `source_path` (50 test files affected).

Do NOT add `\.lst\b` regex to the gate patterns. The 8,592 `.lst` strings in `rules_tables/**` are provenance metadata and schema notation for the data package; they are burned down when those files move to a separate Starfinder-era data package. Catching them now creates a false-positive cascade and obscures real issues.

Fix one player-visible `.lst` message at `character_hub.rs:1485` with a test to prevent regression.

**Enforced by:**
- `scripts/pcgen_residue_gate.py --check --closure` exits zero; `--closure` flag re-derives counts (0 files, 0 hits for both patterns).
- Test at `scripts/tests/test_pcgen_residue_gate.py` for `lst_file` pattern detection and `codex_ingest` walling.
- Test for character_hub `.lst` wording fix (canonical_description text should not expose PCGen `.lst` filename syntax).

---

## §4 — Freeze PF1e status page at 100%; retire producers

**Operator ruling, 2026-09-15:**

> *"Freeze a static snapshot of the PF1e status page; retire `v06_work_inventory`, `support_state_matrix` + desktop bridge, `reach_gate`, and everything that exists only to feed them."*

**Decision.** The public status page (`site/status.html`, `site/status-data.json`) is now static: 49,450 of 49,450 units, 100% complete, dated. The page is regenerated once per closure and never updated in live work.

New gate `scripts/site/check_frozen_status.py` + `scripts/tests/test_check_frozen_status.py` asserts:
- `overall.pct == 100.0`
- `overall.denominator == 49450`
- `partial == 0`, `not_started == 0`
- per-book sums equal overall
- `generated_at` constant (timestamp frozen at closure, never updated)

Retire all supporting machinery:
- Delete 8 binaries and support modules: `v06_work_inventory.rs`, `support_state_matrix.rs`, `support_state_matrix_bridge.rs`, `reach_gate.rs`, `fixture_verified_oracle_probe.rs`.
- Delete 9 test files: `sd13_support_state_matrix.rs`, `v06_work_inventory.rs`, 4 race-bounded tests, 1 desktop bridge test, 1 dashboard script test, 1 atlas test, 1 site vocabulary test.
- Delete desktop UI components: `SupportDebtPanel`, `BreadthClaimAuditPanel`, `supportStateTone` wiring, desktop-to-Rust bridge code.
- Delete `scripts/publish-site-dashboard.sh`, `scripts/shape_engine_boundary.py`, `scripts/completion_atlas.py` and their tests.
- Delete `.claude/skills/swarm-status-sync/` skill (no longer used).
- Delete architecture doc `docs/architecture/support-state-matrix.md`.

Keep `docs/work-inventory.json` (it is the sheet-rule population input, `sheet_rule/mod.rs:280`). Add sidecar `docs/work-inventory.FROZEN.md` recording the hash and rule.

**Enforced by:**
- Gate `scripts/site/check_frozen_status.py --check` exits zero; regenerated status data files pass it.
- `scripts/verify.sh`: drop 6 site-dashboard stages; add `site-status-frozen-check` to BOTH stage sets.
- Deleted file counts in closure receipt (8 binaries + modules, 9 test files, 2 support scripts, 1 artifact generator, 3 architecture docs).

---

## §5 — Public page denominator counts every DONE unit

**Operator ruling, 2026-09-15:**

> *"Public page freezes at 100%: change `scripts/site/build_public_status.py` to count every unit with the inventory's own DONE vocabulary (overrides the SD-31 public-denominator rule)."*

**Decision.** Rewrite `scripts/site/build_public_status.py` to read `docs/work-inventory.json` and count every unit in the five DONE statuses (as defined in the inventory's own vocabulary) toward the denominator. This replaces SD-31's public-denominator rule, which had carved out the excluded packaging set.

Regenerate `site/status-data.json` + all `site/status-data/*.json` from committed `site/dashboard/units/*.json` (Python-only rebuild, no Rust; verify the 8 excluded books are not in the output set before asserting per-book sums).

Rewrite old-rule cases in `scripts/tests/test_build_public_status.py` to match the new logic.

**Enforced by:**
- `scripts/site/build_public_status.py --check` exits zero on regenerated data.
- `scripts/tests/test_build_public_status.py` test green.
- `site/status-data.json` production records exactly 49,450 / 49,450 (verify with `python3 -c "import json;print(json.load(open('site/status-data.json'))['overall'])"` → `{'pct': 100.0, 'denominator': 49450, ...}`).

---

## §6 — Box housekeeping (no repo change)

**Operator ruling, 2026-09-15:**

> *"Disk: delete the 234G SD-33 scratch dir and the 38G `~/cargo-targets/w31-integrate`. Keep `target/`."*

**Decision.** Clear old build artifacts and data from the run box. Phase 0b (no repo change):

```bash
rm -rf /tmp/claude-1000/-home-ubuntu-workspace-repos-codex-docs-release-SD-33-computed-value-verification   # 234G
rm -rf /home/ubuntu/cargo-targets/w31-integrate                                                             # 38G
df -h /                                                                                                      # confirm ≥ 850G free
crontab -l | wc -l            # count BEFORE
crontab -l | grep -v 'dashboard-watchdog.sh\|dashboard-renderer.sh\|swarm-status-heartbeat.py' | crontab -
crontab -l | wc -l            # count AFTER = before - 3; reclaim.sh line still present
```

**Enforced by:**
- `df -h /` in closure receipt shows ≥ 850G free.
- `crontab -l | wc -l` before/after counts in closure receipt (before N, after N-3).
- Manual step; recorded in workflow-instruction Phase 0b.

---

## §7 — Epic order: B → A → C → D, with reason

**Operator ruling, 2026-09-15:**

> *"Epic order B → A → C → D. B first because it deletes 55k+ lines that A would otherwise move, and A must precede C because C's `pilot_compute` split and test rewrite would collide with A's test-module eviction."*

**Decision.** Execute epics in this order:

1. **Epic B** (freeze status, retire producers) — deletes 55,827 lines of supporting machinery.
2. **Epic A** (PCGen wall, crate `codex-ingest`) — moves machinery that would collide with C's test eviction.
3. **Epic C** (bloat cuts: split `pilot_compute`, consolidate paths, rewrite tests) — two-pass refactor dependent on B and A being complete.
4. **Epic D** (closure) — architecture-docs refresh, retrospective, PR.

The reason: B's deletions reduce A's scope (A doesn't have to move what B already deleted). A's test-module eviction (36 `#[cfg(test)]` modules in `src/rules_core`) does not collide with C's test rewrite if A runs first, because A moves/evicts those modules and C can then rewrite the remaining test structure without re-opening the same directories.

**Enforced by:** workflow-instruction.md epic dispatch order (§6 through §9).

---

