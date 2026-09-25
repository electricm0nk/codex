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

## §9 — Unattended UI test-and-repair; fix on the fly; ui-smoke receipts live under `artifacts/ui-smoke/`

**Operator ruling, 2026-09-17:**

> *"No race could be read from the corpus" on Create Character. Go unattended, fix it on the fly, bundle EVERYTHING (~490 MB: `data/corpus` + `data/sheet_rules`). Build a real red-green UI harness."*

**Decision.** The operator reported a packaged-build defect (Create Character showing no race
roster) and authorized an unattended session to both diagnose and fix it, and to build the
regression harness that should have caught it, rather than hand-patching the one symptom. Two
things followed from this ruling, both already reflected elsewhere in this package and recorded
here as the ruling that authorized them:

1. **The corpus-bundle fix is §8 above** — "bundle everything" is read against the standing
   no-PCGen-residue ruling (§3/§11) rather than literally, per §8's own reasoning.
2. **A red-green UI smoke harness is built and run to closure, unattended, across three repair
   cycles:**
   - `apps/desktop/scripts/ui-smoke/spec.json` (69 rows) + a generated
     `docs/testing/ui-smoke-inventory.md`, driven by a DEV-only DOM command channel
     (`record_ui_probe` / `apps/desktop/src/testSupport/uiProbe.ts`) rather than `xdotool`
     coordinate clicks, which fail 100% of the time against small text links under
     Xvfb+WebKitGTK (cycle 1 finding).
   - Long runs (20-40+ minutes) go `nohup` + pid-poll, never a single foreground `Bash` call, and
     every run pre-fills a **not-run skeleton** for every row before starting, so a truncated run
     leaves visible `not-run` rows instead of a partial file silently reported complete (cycle 2's
     own failure mode, corrected in cycle 3).
   - Closed 2026-09-18 at commit `89bfbf1142`: 69 rows, 66 green, 3 true-manual (native OS file
     dialogs: import/export/portrait), 0 red/blocked/not-run.

**Enforced by:**
- `apps/desktop/scripts/ui-smoke/spec.json` and its generated inventory doc are the harness's own
  spec of record; re-run via the cycle-3 script (`ui-smoke-repair-cycle3-…js`, session-scoped, see
  `sd36-ui-smoke-run` session memory for the resume handle) whenever a UI regression is suspected.
- Receipts live under `docs/release/SD-36-consolidation/artifacts/ui-smoke/final/` — the closing
  cycle's row-by-row results and the receipt that supersedes the two prior (superseded, not
  reverted, per the append-only-history discipline: `5b08fb88a6`, `3d4cebd470`).
- The corpus-root fix itself: `apps/desktop/src-tauri/src/authoring_workbench.rs`'s
  `codex_repo_root()` no longer falls back to a compile-time `CARGO_MANIFEST_DIR` path on a
  packaged build.
- The long-run dispatch discipline this ruling forced into the open (nohup+poll, not-run
  skeleton) is generalised into this bundle's own `ENVIRONMENT GUARD` and recorded as Lesson 1/2/3
  of `docs/retro/sd36-retrospective.md`.

---

## §10 — Complete architecture-docs update added to Epic D closure

**Operator ruling, 2026-09-20:**

> *"Rewrite the architecture docs in full before this bundle closes — not a delta patch, the
> living-documentation set brought current against everything B/A/E/C1 touched."*

**Decision.** Epic D's original scope (`epic-breakdown.md` D1) already required a refresh of
`docs/architecture/`'s boundary/rules-engine/desktop-app/status/testing sections for topics
touched by this bundle. This ruling widens that from a targeted delta to a **complete rewrite
pass** across the whole `docs/architecture/` set — every doc's "Last verified" header brought to
this bundle's own HEAD, not just the sections this bundle's own epics happened to touch — and adds
it explicitly to the closure gate rather than leaving it as an optional tidy.

**Result:** `docs/architecture/README.md`'s "Last verified" header now reads
**2026-09-20 against `tranche/16` (`b22ea9e113`, SD-36 Epic D)**. Nine existing docs updated,
one retired doc deleted (`support-state-matrix.md`, matching Epic B's own retirement of the
module it described), two new docs added (`getting-started.md`, `glossary.md`). Full diffstat and
per-doc breakdown recorded in `release-notes.md`'s "Architecture-docs rewrite (D1)" section.

**Enforced by:**
- `git diff --stat 50572eebad..HEAD -- docs/architecture/` (the tranche/15-cut tree vs. this
  bundle's HEAD) — the acceptance evidence `epic-breakdown.md` D1 already names, now satisfied at
  full-set scope rather than delta scope.
- `docs/architecture/README.md`'s own "Last verified" header, updated in place per that doc's own
  maintenance contract (§Maintenance contract, that same file).
- This ruling is recorded here, and the closure receipt (`receipts.md`, "Architecture-truth-up
  receipt") records the pass as complete before Epic D's graphify/PR steps run — per
  `docs/release/SD-36-consolidation/kanban.md`'s own D1 row.

---

## §11 — Close the class gaps inside SD-36, before PR #393 merges

**Operator ruling, 2026-09-21:**

> *"Close the class gaps INSIDE SD-36 before PR #393 merges."*

**Decision.** A permanent census instrument (`tests/zz_class_census.rs`, run once, then deleted —
`docs/release/SD-36-consolidation/artifacts/epic-f/docs-truth/class-census.md`, generated
2026-09-20 against `tranche/16` @ `424e93e93c`) measured the engine's true class coverage,
corpus-wide across every registry `compute_class_chassis`'s dispatch chain reads: **42 of 135**
distinct class ids reach `HeadlessReceiptStatus::Computed` at every swept level. The prior
"31 of 31" / "42 of 42" figures quoted elsewhere in this bundle's own docs were each true of a
narrower registry population, not the corpus-wide one this census measures — see that document's
§3 "The refuted claims, verified by this instrument" for the three specific corrections.

Rather than defer class completion to a successor bundle, the operator ruled the gap closes
inside SD-36 itself, as **Epic F — Class completion**, scoped between Epic D's D1 (architecture
docs, already done) and D2–D6 (retrospective, release notes, graphify, PR, worktree sweep — held
until Epic F lands). **Epic order, corrected:** B → E → A → C → D(docs) → **F** → D(closure).

**Target: 135 of 135** — 61 base-type ids alone at every level, plus 74 prestige ids in their
canonical carrier mix at every prestige level (prestige classes are never swept alone for the
Computed column; a separate `alone_status` column requires all 74 of 74 to be `Blocked` with a
named game-rule diagnostic, per §14 below). Full batch plan (F0–F5), every command, every
file:line, RED-first tests, sizing (100-140 agent-hours) and the adversarial review that raised it
from an original 81-115: `docs/release/SD-36-consolidation/epic-f-class-completion.md`.

**Enforced by:**
- `epic-breakdown.md`'s Epic F criteria tables (F0.1–F5.3).
- The census baseline `BASELINE_CENSUS_IDS=135`, `BASELINE_CENSUS_COMPUTED=42` (can only rise,
  never fall) in `scripts/verify-baselines.env`, set once F0 lands.
- `kanban.md` / `progress.md` Epic F rows, all `open` until each batch's acceptance commands pass.

---

## §12 — Converter link fix scope = Option A (all 4,456 parent-category links)

**Operator ruling, 2026-09-21:**

> *"Converter link fix scope = option A."*

**Decision.** `data/sheet_rules/_defects/unresolved-references.json` carries 11,925 unresolved
references, classified by mechanism (`unres2.py`, copied into this bundle at
`docs/release/SD-36-consolidation/artifacts/epic-f/scripts/unres2.py`):

| # | Mechanism | Rows | Fixed by parent-category map? |
|---|---|---|---|
| A | Child ability category; target IS a converted record under the parent | **4,456** | **Yes — all** |
| B | Child category; target exists in the oracle but is not a converted unit | 63 of 11,925 | No |
| D | Plain category; target IS a converted record; resolver misses for another cause | 3,033 of 11,925 | No — not diagnosed |
| E | Plain category; target exists in the oracle but its book/family is not ingested | 3,565 of 11,925 | No — correct as a defect |
| F | Target found nowhere (bracketed, comma-joined, case, nested-paren syntax) | 808 of 11,925 | No — 3+ small parser causes |

Two options were weighed: **B** — fix only the 99 of 11,925 proficiency-related mechanism-A rows
needed to unblock the census; **A** — fix all 4,456 of 11,925 mechanism-A rows corpus-wide, since
the resolver defect is generic (parent-category lookup) and the same fix closes every child-
category miss at once, not just the ones a class happens to need this cycle. **Ruled: Option A.**

**Consequences accepted, named in the ruling:** a new print-path reconciliation batch (F1b) is
required, because holding ~4,456 previously-unresolved rules (amplified by sibling rules —
measured, not assumed, before the population run — see `epic-f-class-completion.md` §3b.0)
surfaces new lines on live character sheets that must be de-duplicated against the bespoke
`pilot_compute` path, one extra full `scripts/verify.sh` pass, and +25-35 agent-hours over
option B (later revised further by an adversarial review of Epic F's own execution plan — see
`epic-f-class-completion.md` §12's review log — to a total of 100-140 agent-hours; the review's
five additional findings were correctness fixes to option A's execution, not a reason to revisit
this ruling).

**Mechanisms B (63 of 11,925), D (3,033 of 11,925), E (3,565 of 11,925), F (808 of 11,925) stay out
of scope**, recorded in `forward-scope-register.md` with their per-book/per-cause breakdown —
**EXCEPT** the rows that sit inside a class's own grant closure and keep that class's
`closure_complete` flag false: measured at 2 closures / 5 rows for mechanism D (Antipaladin 1,
Sanguine Angel 4) and 7 closures / 15 rows for mechanism F (Alchemist 9, Slayer 1, Ex-Antipaladin
1, Diabolist 1, Exalted 1, Magus 1, Marksman 1) — 9 classes, 16 rows of the D+F total, per the
class-closure simulation (`docs/release/SD-36-consolidation/artifacts/epic-f/scripts/closure.py`).
Those 16 rows are Epic F's own job (each fixed by its named parser cause), because leaving them
open would leave a class's proficiency answer `Unknown` rather than a real answer. Every other D/E/F/B
row is a genuine forward-scope deferral, not a closure gap.

**Enforced by:**
- `epic-f-class-completion.md` §1 (the mechanism table, reproducible via `unres2.py`) and §3
  (the converter change and its structural-diff gate).
- `forward-scope-register.md`'s new rows for mechanisms B/D/E/F, each naming its count and its
  re-derive command, and naming which D/F rows Epic F itself closes.
- F1.3/F1.6 in `epic-breakdown.md` (links closed = 11,925 - 4,456 = 7,469 of 11,925, or the
  difference explained row by row; structural diff proves no other field moved).

### §12.1 — Resolver scope addendum (F3b2b, 2026-09-24): same-object reprints resolve to the newest printing

**Rule applied, not a new ruling.** The standing supersession ruling (operator, 2026-08-16, SD-31:
*"if a duplicate is found, the most recent publishing takes precedence and the older one is flagged
as superseded"*, with its amendment the same day: *"rogue and unchained rogue are two completely
different classes - one does not replace the other"*) now applies inside the converter's reference
resolver. Before F3b2b, a reference whose `(category, key)` named two printings of one object was
left ambiguous (`ambiguous-parent-category-target`, F1 adversarial finding 4: never guess).

**Mechanism** (`crates/codex-ingest/src/pcgen_import/sheet_rule/reprint.rs`, one rule, no
per-record case). An ambiguous pair resolves to its newest printing when every candidate is a
printing of the same object:

- same kind; no candidate in a variant-line book (`mythic_adventures`, `pathfinder_unchained`:
  the amendment's default answer is "variant");
- each candidate's base oracle row agrees on name, `KEY:` (else name) and `CATEGORY:`;
- identity is proved field by field: every row states a `DESC:` and the descriptions (lower-cased,
  whitespace collapsed) are prefix-ordered (the reprint repeats the older text and may extend it),
  OR the rows are identical token for token apart from `SOURCE*` bookkeeping;
- publication order from each book's `.pcc` `SOURCEDATE:` header (`PinnedTree::source_dates`),
  never from memory; exactly one candidate carries the latest date.

Anything else stays ambiguous, named. Also in F3b2b: a child ability category whose declarations
disagree only on `TYPE:` keeps its parent (`closure.rs`; before, the TYPE disagreement also dropped
the parent -- Cyphermage Class Feature, `ism_abilitycategories.lst:56` / `ag_abilitycategories.lst:7`).

**Measured** (`_defects/ambiguous-parent-category-target.json`, F3b2 package -> F3b2b package): the
ambiguous-target population under the repaired parent map is 31 rows (the 30 F3b2 rows plus
Cyphermage's, reachable once its parent is kept); **13 of 31 resolve**, 18 stay:

- resolved: 10 Advanced Race Guide racial-subtype choices whose target Advanced Player's Guide
  printed first (`SOURCEDATE:2010-08` vs `2012-06`, same KEY, the two descriptions equal after
  normalisation); Red Mantis Assassin's `Class Feature|RMA Weapon Proficiencies` in both class printings
  (`iswg_abilities_class.lst:151` and `ag_abilities_class.lst:419`, token-identical; newest
  `adventurers_guide`, 2017-06); Cyphermage's `Cyphermage ~ Cypher Lore`
  (`ism_abilities_class.lst:8`, 2011-07, is a prefix of `ag_abilities_class.lst:103`, 2017-06).
- stay ambiguous, 17: Advanced Race Guide reprints of Advanced Player's Guide racial traits whose
  text was reworded (not a prefix; e.g. `Dwarf ~ Stubborn`: "renowned for being stubborn" vs
  "renowned for their stubbornness"). Mechanism: *reworded reprint -- identity not provable by the
  prefix or token-identity test*. Closes in: a field comparison over the mechanical tokens, not
  taken here.
- stays ambiguous, 1: `Master Of Many Styles ~ Perfect Style`, declared twice inside one book
  (`uc_abilities_class.lst:1096` and `support/uc_abilities_class_ag.lst:80`, both `ultimate_combat`,
  2011-01). Mechanism: *same-book double declaration -- no newest printing*.

The direct (non-parent-retry) lookup keeps its first-wins behaviour (unchanged; F1 finding 4's
scope note).

---

## §13 — Weapon proficiency is read from the converted record, not authored as new Rust rows

**Operator ruling, 2026-09-21** (restates and extends §2/§7's ruling-7 "no `rules_tables` move
before Starfinder" against the specific question Epic F raised — do the ~93 classes still
missing a weapon-proficiency answer get their own hand-typed Rust table rows, or does the engine
read the record the converter already produced):

> *"Read the proficiency answer from the converted record; ruling 7 forbids moving `rules_tables`
> to data before Starfinder, it does not forbid a new reader over existing converted data."*

**Decision.** `src/rules_core/rules_tables/crb/weapon_tables.rs` keeps its 42 hand-transcribed
rows verbatim (first precedence, unchanged, ruling 7). For every other class, a new
`class_proficiency_sheet_rules.rs` reader builds a `HeldSeed` for that one class, runs the
existing `held_set` fixpoint, and collects `Effect::FactGrant(Fact::Proficiency(..))` from the
held rules — the same converted vocabulary `data/sheet_rules/` already carries (0.1 in
`epic-f-class-completion.md`: 159 class_feature + 6 class + others already hold a converted
proficiency grant today; the defect is the broken LINK from class to grant, not a missing
converted fact). **Reasoning:**
1. Ruling 7 forbids moving `rules_tables` **to data**; it says nothing about authoring **new**
   data as a Rust literal instead of reading data that already exists. Precedent: `class_chassis_
   sheet_rules.rs` already reads `data/sheet_rules/<book>/class/<slug>.json` at runtime, and
   `apps/desktop/src-tauri/tauri.conf.json:40` already bundles `data/sheet_rules/` as a shipped
   resource (0.9).
2. Ruling 4 (§3, prior ruling) named 93 hand-typed rows as the fabricated-row hazard this bundle
   exists to avoid — the doctrine's "no fabricated row… never a rule row from DESC prose alone"
   applies exactly as hard to typing 93 new proficiency rows by hand as it did to the 42 that
   already ship without an oracle pin (0.6; F1 adds that pin as part of the same batch, F1.2).
3. The residue gate is already 0 of 0 on the converted vocabulary — reading MORE of it adds no
   PCGen surface; authoring 93 new Rust literals from the same source prose would.
4. **Known-empty vs unknown**, never fabricated: the converter writes a per-class
   `closure_complete` boolean; the reader returns `Some(empty-set)` only when true, `Unknown`
   otherwise. A class whose closure carries a still-gated AUTO grant (review finding 1 in
   `epic-f-class-completion.md` §0.1a) also reads `Unknown`, not `Some(set)`, until the gate is
   carried onto the effect (§3.1 item 4).

**Enforced by:**
- `epic-f-class-completion.md` §3.4 (the reader design) and §0.6 (no converter-backed weapon pin
  exists today; F1 adds one).
- F1.2 in `epic-breakdown.md`: reader output matches each of the 42 static rows AND each reader
  row is independently re-derived from oracle rows.
- `scripts/pcgen_residue_gate.py --check --closure` stays 0 of 0 through F1 (F1.7).

---

## §14 — Prestige classes: legal only in a mix, entry gate printed; Ex-* classes census-only

**Operator ruling, 2026-09-21:**

> *"Unmet prestige entry requirements PRINT met/unmet and never block. Ex-* classes are
> census-only, not offered in the Create picker."*

**Decision, two parts.**

1. **Prestige entry requirements print, never block.** A prestige class's converted `applies`
   gate already carries its entry requirements in Codex vocabulary (0.7 in
   `epic-f-class-completion.md`: e.g. Arcane Archer — `BaseAttack >= 6`, three feats,
   `HighestSpellLevel Arcane >= 1`). F0's census reports each requirement `met` or `unmet` in a
   printed `entry_gate` column; unmet requirements never count against a class's `Computed`
   status and never raise a claim-blocking diagnostic — the paper-sheet doctrine's "print the
   rule text, do not simulate" applies to entry gates exactly as it does to every other rule.
   A prestige class taken **alone** (no base-class levels) is the one case that DOES block: F2
   adds a claim-blocking diagnostic `prestige_class.requires_base_class_levels`
   ("A prestige class cannot be a character's first class. Add levels in a base class first."),
   and the census's `alone_status` column requires all 74 of 74 prestige ids to be `Blocked`
   with that diagnostic — a negative control proving the rule fires, not a carve-out.
2. **Ex-* classes (Ex-Barbarian, Ex-Paladin, and `ex_antipaladin` if the census finds it a
   distinct id) are census-only.** They are legal sheet STATES — a character who breaks their
   code falls to one — and stay counted in the census denominator (they do not shrink the 135).
   They are never offered as a choice in the desktop Create Character picker: F4's roster rule
   only ever offers a class the census reports Computed at every level, grouped by family, and
   Ex-* states are reached by the game's own fall-from-grace mechanic, not by a player picking
   "Ex-Paladin" at character creation.

**Enforced by:**
- `epic-f-class-completion.md` §2 (F0's `entry_gate`/`alone_status` columns) and §4 (F2's
  prestige-alone diagnostic and its acceptance row F2.2).
- `epic-breakdown.md` F0.1/F2.2/F4 criteria; census `alone_blocked=74` of 74.
- `epic-f-class-completion.md` §6 (F4's roster rule: "Ex-* states: census-only, never offered at
  creation").

### §14.1 — Prestige saves the oracle's formula cannot state stay Blocked (F3c2, 2026-09-24)

Six prestige classes' carrier mixes stop at `multiclass.save_shape.unrecognized`: Evangelist,
Exalted, Mammoth Rider, Pure Legion Enforcer, Sentinel and Ulfen Guard. Their converted save
`Expr`s are faithful conversions of oracle `BONUS:SAVE` formulas that match no PF1 save form
(PCGen divides before it adds, so `classlevel()+1/3` is level + 1/3 and prints +10 at 10th).

**Decision.** They stay Blocked, a named remainder of **6 of 74** prestige carrier mixes in the
census. Printing the oracle's number would put a save on the sheet that the book does not state;
repairing the formula from memory (reading `CL+1/3` as `(CL+1)/3`) is a fabricated row. The
mechanism is an **upstream oracle data defect**, closable only by a book-cited override: each
class's save progression read from its own book's class table -- Inner Sea Gods p.198 (Evangelist),
p.200 (Exalted), p.202 (Sentinel); Inner Sea Combat p.32 (Pure Legion Enforcer), p.34 (Ulfen
Guard); Adventurer's Guide p.128 (Mammoth Rider) -- recorded with its citation and pinned by a
hand-worked test. Recorded as `forward-scope-register.md` FS-15; evidence
`artifacts/epic-f/stage-f2-f3/f3b3-receipt.md` §3, `f3c-remainder.md`, `f3c2-receipt.md`.

**Enforced by:** `multiclass_fold::tests::the_four_unrecognized_prestige_saves_are_oracle_formula_defects_not_a_missed_shape`,
`multiclass_fold::tests::two_more_prestige_saves_the_f3c_carriers_reach_are_the_same_oracle_formula_defect`,
and the census floor `BASELINE_CENSUS_PRESTIGE_MIX_COMPUTED=67` (`scripts/verify-baselines.env`).

### §14.2 — Multiclass negative controls assert status parity with the class alone (F3d, 2026-09-25)

**Orchestrator ruling (autonomous mode), 2026-09-25**, on the F3d blocker
(`artifacts/epic-f/stage-f2-f3/f3d-blocker.md`, commit `157dc5496a`).

**Measurement.** The 187 "multiclass X must stay claim-blocked in this slice" tests
(`f3d-sites.tsv`: 64 `MULTICLASS_NEG_ROWS` + 59 `multiclass_negative_controls!` rows + 64
hand-written, in `sd18_widening`, `sd13_progression` and 43 top-level binaries) widen a
class-specific fixture to a Class+Fighter mix. Those fixtures are not in the GE-06 posture, so
the class ALONE is Blocked on `combat.baseline_unsupported` + `skill.selected_modifier.unsupported`
(187 of 187), and every mix's claim-blocking set equals the class-alone set apart from the
`multiclass.<class>.` re-scope (187 of 187). Flipping assertion (b) to `Computed` ran 187 of 187
red (`f3d-red.log`): asserting it would fabricate a success. The Computed proof for mixes already
lives in the census (mix panel 185 of 185 under the GE-06 fixture; prestige carrier mixes, floor
`BASELINE_CENSUS_PRESTIGE_MIX_COMPUTED`) and in `tests/sd36_multiclass_any_class.rs`.

**Decision.** Assertion (b) becomes **status parity** (the F3b `class_dispatch` precedent,
`..._computes_exactly_when_it_computes_alone`): the mix's receipt status equals the unmodified
class-alone fixture's, AND the mix's claim-blocking set, with the fold's `multiclass.<class>.`
re-scope stripped, equals the class-alone set. A class that computes alone computes in a mix; a
class blocked alone stays blocked on exactly its own lines -- none lost, no multiclass-only line
added. Assertion (a) (the bounded single-class explanations stay withheld) is kept verbatim. Vacuity
guard: the mix loads >= 2 classes (and more than the class alone). Test names unchanged; `--list`
byte-identical (sd18_widening 891, sd13_progression 1,136). One helper,
`tests/common/mod.rs::assert_multiclass_status_parity`, serves all 187 sites.

**Sabotage parity, re-measured.** Disabling the fold's carry-over of each class's own claim-blocking
lines turns **14 of 187** red (the Monk mixes, whose bonus-feat line only the carry-over supplies);
restored, 0 red. The other 173 are held by lines the mix raises itself -- the character-level
pillars (129) and class-feature checks that run for the class mixed as well as alone (sorcerer 25,
cleric 19) -- so this sabotage cannot move them; the status/set assertion guards them against any
change to those. Evidence `artifacts/epic-f/stage-f2-f3/f3d-sabotage-log.md`, `f3d-verify.log`.

**Enforced by:** the 187 tests; `epic-breakdown.md` F3.1–F3.3 (rewritten to this measurement).

---

