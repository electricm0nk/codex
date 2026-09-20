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

## §8 — Ship a sanitised corpus bundle, not the raw corpus

**Operator ruling, 2026-09-17:**

> *"bundle everything (~490 MB)"*

**Decision.** The operator's directive was to bundle the desktop app's full data dependency so a packaged build never has an empty race roster (the defect commit `217f712bab` was chasing). Read literally, "everything" is `data/corpus/` (237 MB) plus `data/sheet_rules/` (254 MB) — measured together:

```
$ du -c -sh data/corpus data/sheet_rules | tail -1
491M	total
```

— matching the ruling's own "~490 MB" figure. But `data/corpus/**/*.json` carries ingest-time PCGen residue (`raw_tokens`/`raw_bonus_chains` arrays, unstripped `DESC:`-token trailing clauses, free-text provenance fields that can themselves quote token syntax) that no live consumer reads and that `decisions.md` §11 / the residue gate's ruling B17 forbid on the shipped side, with **zero** carve-outs (`scripts/pcgen_residue_gate.py`'s `EXCLUDED_PREFIXES` is empty by a prior, standing ruling). Bundling `data/corpus/` verbatim, as commit `3e1a8f8d39` did on an interim basis, puts PCGen token text on a tester's disk. The standing "nothing of PCGen in the shipped product" ruling outranks the literal "bundle everything" instruction where the two collide: the fix is a bundle that ships the SAME population the raw corpus does, with the residue removed, not a bundle that ships the residue too.

**Implementation:**
- `scripts/gen-corpus-bundle.mjs` (Node — see "why Node" below) mirrors only the six `data/corpus/<book>/<kind>/` directories the live loaders (`src/rules_core/corpus_loader.rs`, `race_resolver.rs`, `trait_pool.rs`) actually read, trims each record to the fields that loader reads, and strips every occurrence of the residue gate's own pattern vocabulary from every surviving string as a defense-in-depth net.
- `data/sheet_rules/` ships RAW (254 MB, unchanged) because it already carries no residue — confirmed by running the full gate after the corpus-bundle change:

  ```
  $ python3 scripts/pcgen_residue_gate.py --check --closure
  ...
  shipped_data_files=0 shipped_data_hits=0 shipped_scanned=68815
  live_files=0 live_hits=0 verdict=PASS
  ```

  (`shipped_scanned=68815` = 54,775 `data/sheet_rules/` files + 14,029 sanitised corpus-bundle files + 11 other bundled-resource files; re-derive with `find data/sheet_rules -type f | wc -l` and `find apps/desktop/src-tauri/resources/corpus_bundle -type f | wc -l`.)

- Net shipped size: **64 MB** (sanitised corpus bundle) **+ 254 MB** (`data/sheet_rules`, raw) **= 318 MB** shipped, against the ruling's ~490 MB "bundle everything" figure — the difference is entirely the ~173 MB of PCGen ingest scaffolding (`raw_tokens`, `raw_bonus_chains`, unread fields) the sanitiser strips from `data/corpus/`'s 237 MB, not a reduction in the RECORD POPULATION shipped.

  ```
  $ du -sh data/corpus apps/desktop/src-tauri/resources/corpus_bundle data/sheet_rules
  237M	data/corpus
  64M	apps/desktop/src-tauri/resources/corpus_bundle
  254M	data/sheet_rules
  ```

- **Why Node, not Python, for the generator.** `.github/workflows/publish-tester-release.yml` builds the desktop app on `ubuntu-latest`, `macos-latest`, AND `windows-latest` (`publish-tester-release`, `publish-tester-release-macos`, `publish-tester-release-windows` jobs) — every one of those already runs `npm ci`/`npx tauri build`, so Node.js is guaranteed; Python is not pinned or installed on the macOS/Windows runners at all for this app. The generator is therefore `scripts/gen-corpus-bundle.mjs`, ported 1:1 from the original Python draft with zero new dependencies.
- **Why `scripts/`, not `apps/desktop/scripts/`, despite the SD-36 workflow instruction's own working assumption.** `apps/desktop/**` is a `LIVE_ROOT` for the residue gate with no carve-outs. A generator that sanitises PCGen vocabulary must NAME that vocabulary in its own source (`raw_tokens`, `BONUS:`, `PRE[A-Z]+:`, ...) to remove it. Verified directly: an earlier draft at `apps/desktop/scripts/gen-corpus-bundle.mjs` made the gate fail (`apps/desktop files=1 hits=4`, from the generator's own pattern array, not from anything it copied) before any bundle content was even considered; moving the file to repo-root `scripts/` (a non-live, tooling root) restored `live_files=0 live_hits=0 verdict=PASS`. `scripts/gen-corpus-bundle.mjs`'s own header comment records this as a verified, not merely asserted, fact.

**Parity, not merely presence, is the proof nothing was lost.** `apps/desktop/src-tauri/src/corpus_bundle_parity_test.rs` (`cargo test -p codex-desktop corpus_bundle_parity`) regenerates the bundle and runs the SAME production loaders against the raw corpus and the bundle in turn, per book, asserting equal equipment/spell record counts, equal race rosters (`RaceCorpus::race_keys()`), and zero loader diagnostics on both sides (which also proves `validate_license` still accepts every race/race-trait record the sanitiser produced — license/PI fields are the one thing the trim step must never break). Mutation-proved during this cycle: temporarily narrowing the generator's kind list to drop `race`/`race_trait` made the test fail and name the exact books and missing race ids (`advanced_race_guide`, `beastiary`, `bestiary_2`, `bestiary_5`, `bestiary_6`, `core_rulebook` — 6 books, matching every book in the raw corpus that carries a `race/` or `race_trait/` directory); reverting the generator made it pass again.

**Scope of the parity claim, stated precisely.** "Equal to the raw corpus" holds for the population `corpus_loader.rs`/`race_resolver.rs`/`trait_pool.rs` read, over the six mirrored kind directories — that is what the parity test above proves, and it is the only population any live game-mechanics path reads. It does not (and, by the residue-avoidance rationale above, should not by default) extend to `apps/desktop/src-tauri/src/reference_library_catalog.rs`, a registered Tauri command reading twelve corpus kind directories (only one of which, `trait_generic`, is mirrored) that no frontend currently invokes. That module fixed its own separate defect this cycle — it resolved its corpus root from a hardcoded, build-time `CARGO_MANIFEST_DIR` path rather than `codex_repo_root()`, so it returned nothing at all on any packaged build regardless of bundle coverage — and now refuses (a named error, not a silent empty catalog) any of its eleven un-bundled kinds when its resolved root is not a full source checkout. See `scripts/gen-corpus-bundle.mjs`'s own comment for the maintenance note if that command is ever wired into the UI.

**Enforced by:**
- `scripts/verify.sh --only corpus-bundle` (regenerates, asserts non-empty output, asserts 0 residue hits both by a scoped grep over the bundle and by the full `pcgen_residue_gate.py --check --closure`).
- `scripts/verify.sh --only tauri-resources-tracked` (every `tauri.conf.json` `bundle.resources` key resolves to at least one git-tracked file on a clean checkout — the `.gitkeep` under the gitignored `corpus_bundle/` counts; this is the check that would have caught commit `3e1a8f8d39`'s untracked-resource defect).
- `cargo test -p codex-desktop corpus_bundle_parity` (correctness: same population, not just same file count).
- `apps/desktop/package.json`'s `build` script (`node ../../scripts/gen-corpus-bundle.mjs && vite build`), reached by `tauri.conf.json`'s `beforeBuildCommand: "npm run build"` on every `tauri build` invocation (dev is unaffected: `beforeDevCommand` resolves the repo corpus directly).
- `.github/workflows/publish-tester-release.yml` and `.github/workflows/tranche-3-ci.yml`: an explicit "Generate corpus bundle" step runs `node scripts/gen-corpus-bundle.mjs` before the desktop crate's `cargo test` step in both the `test`/`desktop-typecheck-and-test` jobs (the three `publish-tester-release*` build jobs generate it implicitly via `beforeBuildCommand`).

---

