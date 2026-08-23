# Cycle epic-2-t9-onboarding-pi-name-rename-deity-classfeature — Gate 3 (closure invariant) / Card 11

- **Card ID:** 11 (`epic-2-cause-closure`)
- **Commit SHA:** (this cycle's commit — see push output)
- **Files touched:**
  - `scripts/ingest_simple_filename_kinds.py` (`deity` support: `TARGET_KINDS`/`NAME_ALWAYS_PI_KINDS`,
    unconditional neutral-name rename path, `--report-out`)
  - `scripts/codex_neutral_name.py` (reused unchanged — no edits)
  - `src/rules_core/codex_neutral_name.rs` (new — Rust port, own tests)
  - `src/rules_core/mod.rs` (module registration)
  - `src/rules_core/cache_gen/class_feature.rs` (rename path replacing skip-and-continue; `class`
    fallback and directory-placement PI-leak fixes; `pi_field` append-not-overwrite fix; 5 new tests)
  - `src/bin/gen_cache_class_feature.rs` (`CLASS_FEATURE_RENAME_REPORT` env, updated log line)
  - `data/corpus/*/deity/*.json` (459 new)
  - `data/corpus/*/class_feature/**/*.json` (17,954 regenerated; 140 newly under a
    `codex_named_unit_*.json` filename)
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-3-closure-invariant/24-deity-pi-name-renamed-units.json` (new)
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-3-closure-invariant/24-class-feature-pi-name-renamed-units.json` (new)
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (row 11 note prepended)
  - `docs/release/SD-32-compute-library-and-cause-closure/progress.md` (this cycle's entry appended)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (scoped diff, `src/`+`scripts/`; the one match is a
  legitimate existing-filename citation — `scripts/sd32_t9_pi_review_feat_equipment.py` — matching the
  repo-wide precedent of citing that filename in comments, not a bundle-tag literal)
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** Apply `decisions.md §24`'s neutral-name treatment to `deity` (459) and
  `class_feature` (~140), the last two PI-name-blocked populations, following the `ability` lane's
  proved machinery unchanged; report ingestion and shape-classification separately (`§24c`); prove
  the six `§24b` binding conditions.
- **Corpus SHA:** `PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`;
  bootstrapped from empty this cycle via `scripts/fetch-pcgen-oracle.sh`)
- **Status:** complete
- **Notes:** see `progress.md`'s matching cycle entry for full detail. Two real PI-leak defects (not
  design gaps — implementation bugs in this cycle's own new code) were found by re-running the same
  zero-leak proof the `ability` lane used, before committing: (1) the class-derivation/directory
  fallback chain's last tier ships raw key-owner text, which for a `"<Patron> ~ <Boon>"`-shaped
  Demonic-Obedience key can itself be PI (7/140 leaked); (2) the rename branch overwrote `pi_field`
  instead of appending, dropping "description" off 91 both-name-and-desc-PI records. Both fixed,
  re-proven clean, regression tests added for both. A third, narrower gap (2 blacklist misses from a
  known-stale shared term list) was fixed with a locally-scoped supplemental list rather than widening
  the shared `pi_screening::PI_BLACKLIST_TERMS` constant, after that wider fix was tried and reverted
  for newly failing `tests/pi_table_sweep.rs` against an unrelated, pre-existing `feat_gap_tables.rs`
  leak outside this cycle's scope.
- **Discovery forwards:** none filed as `## DISCOVERED` entries — the `pi_screening::PI_BLACKLIST_TERMS`
  staleness (2 terms: a real place-of-learning name and a swordsmanship-school name, missing from the
  57-term Rust copy versus the 60-term Python T9 list) is named here and in `progress.md` rather than
  forwarded separately; a future cycle syncing that shared constant should first reconcile
  `feat_gap_tables.rs`'s own already-shipped hits via `docs/governance/pi-sweep-baseline.tsv` or a
  redaction pass, which is out of this cycle's scope.
- **Next-cycle plan:** none required for this card's `§24` scope — `deity` and `class_feature` were
  the last two named PI-name-blocked populations. Campaign `no_record` (2,664, re-derived via
  `python3 scripts/shape_ledger.py`) is now dominated by `monster_ability` (967), `feat` (682),
  `spell` (339), `companion` (217), `equipment_modifier`/`equipment` (175/170) — none PI-name-blocked;
  ordinary not-yet-ingested populations for a future cycle.

---

## 0. Environment and PIN

```
PIN=60721c68a9f8d2a6c546ae6fb5ecdc4351bc0bdb
```

Worktree started on an UNRELATED tip (`worktree-wf_da4b9b71-e79-2`, no `docs/`/`data/`/`scripts/`
history reachable from `origin/tranche/12` — footgun 1). Remediated: `git reset --hard 60721c68a9f8`
(the pin itself, and confirmed identical to `origin/tranche/12`'s current tip), re-verified
`PIN_OK`, then `git rebase origin/tranche/12` (no-op, already current).

**Shared `.git` object-store race, unrelated to the base-worktree footgun**: a concurrent sibling
agent's background auto-gc left ~54 zero-byte loose objects mid-repack, failing `git rebase` with
"object file ... is empty". Diagnosed via `git fsck --full`; resolved with `git prune-packed`
(removed the loose duplicates now safely covered by the newly-completed pack) rather than any
destructive operation. `git fsck --full` clean afterward; rebase then succeeded.

**Shared `CARGO_TARGET_DIR` race, discovered mid-cycle**: the dispatched `CARGO_TARGET_DIR=
/home/ubuntu/.cache/codex-targets/sd32-t9-onboarding` (from the dispatch brief's own env block) was
simultaneously in use by a DIFFERENT sibling worktree (confirmed via the scratchpad's own
env.sh being silently overwritten mid-session with a different worktree's paths under the identical
`RETRO_ACTOR=t9-onboarding` name) — cargo served a stale rlib from the OTHER tree, producing a
spurious "no field `name_pi_renamed_records`" compile error against code that unquestionably had
that field. Per `AGENTS.md`'s own standing rule ("`CARGO_TARGET_DIR` is one directory per agent *per
source tree*"), switched to a target dir keyed on this worktree's own name
(`/home/ubuntu/.cache/codex-targets/sd32-wf_da4b9b71-e79-2`); the error vanished on the next build.
PCGen oracle slot was empty (fresh worktree, git-ignored); bootstrapped via
`scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR"` → `pcgen-oracle: OK
7f818006e371188e5717fd18d74d18a420747fc6`.

## 1. Re-derived the population before writing anything (§17a)

```bash
python3 -c "... count kind=='deity' / 'class_feature' in docs/work-inventory.json ..."
```
`deity: 459`, `class_feature: 18043` total (of which `name_pi_skipped` — re-derived below — is 140).
Matches the brief's `deity` figure exactly; `class_feature`'s "~140" was confirmed, not assumed, by
actually running the fixed generator (§4).

## 2. `deity` — lifted the standing exclusion, unconditional rename

`scripts/ingest_simple_filename_kinds.py --kind deity --dry-run` (pre-flight, no writes):
```
"deity:seen": 459, "deity:written": 459, "renamed_count": 459, citation_mismatches: []
```
All 459 resolve cleanly against the pinned oracle; 0 citation mismatches. Ran for real:
```
python3 scripts/ingest_simple_filename_kinds.py --pcgen-root "$PCGEN_CORPUS_ROOT" --kind deity \
  --report-out .../24-deity-pi-name-renamed-units.json
```
459 files written under `data/corpus/*/deity/codex_named_unit_deity_*.json`.

## 3. `class_feature` — enumerating WAS the rename

No separate "list first" step exists for a generator that already computes the disposition per unit
(`declared.name || name_license == PiRedacted`) — the coordinate list is a byproduct of running the
fixed `generate()`, not a prerequisite artifact to produce first. Built `src/rules_core/
codex_neutral_name.rs` (Rust port, own 5 tests: swap-invariance, coordinate purity, determinism,
no-collision, slug-edge-cases — all pass), wired it into `class_feature.rs`'s `name_pi_skipped`
branch (rename-and-write instead of skip-and-continue), added `scrub_name_pi_tokens` (5 tests: hits,
no-op, both-PI-fields, e2e rename, directory-leak guard). `cargo run --locked --release --bin
gen_cache_class_feature`:
```
class_feature cache generated: 17954 records across 23 books (140 renamed under a Codex-generated
neutral name, decisions.md §24); ingested_at=...
```

## 4. Two real defects found by re-running the zero-leak proof, fixed before landing

See `progress.md`'s matching entry for full detail (directory/`data.class` leak on
Demonic-Obedience-shaped keys; `pi_field` overwrite dropping "description" on both-PI records). Both
reproduced with a dedicated regression test (`generate_renames_a_name_pi_row_instead_of_skipping_it`,
`generate_keeps_description_in_pi_field_when_both_name_and_desc_are_pi`), fixed, re-verified clean by
re-running the full 599-file zero-leak scan (§5) against the corrected output — not trusted from the
fix's diff alone.

## 5. Zero-leak proof, both kinds, all 599 renamed files

- Blacklist term scan (60-term Python T9 list): 0 hits.
- Original name/key self-check, joined by `(book, source_file, source_line)` coordinate (never by
  filename substring, which produces false positives on short names): 0 hits.
- Directory-collision check: every one of the 599 renamed files landed at a NEW path (`git status
  --porcelain` `??`), none overwrote an existing tracked record.
- `declared_pi_shipping_audit` (built and run over the FULL `data/corpus` tree, not scoped): 0
  violations naming `deity` or `class_feature`. Its remaining 28 violations are pre-existing
  `language`/`template` gaps; confirmed unmodified by this cycle (`git status --porcelain` on all 28
  named files is empty).

## 6. Determinism proved (§24b-6)

Both kinds regenerated a second time from a full `data/corpus` snapshot, diffed programmatically
(field-by-field, `ingested_at` excluded): 0 differences across all 459 `deity` files and all 18,051
`class_feature` files (140 renamed + 17,814 unchanged + 97 files the regen also touches that are not
under a per-record path this check globs, confirmed accounted for by the `18051` vs `17954`-written
count matching the directory's real file total).

## 7. Suites run

```
cargo build --locked --lib                                        # clean
cargo test  --locked --lib class_feature::                        # 42 passed, 0 failed
cargo test  --locked --lib codex_neutral_name                     # 5 passed, 0 failed
cargo test  --locked --lib pi_screening::                         # 17 passed, 0 failed (unchanged)
cargo test  --locked --bin gen_cache_class_feature                # 0 tests (binary has none)
cargo build --locked --release --bin declared_pi_shipping_audit   # clean, run over full corpus
```
Full unscoped `cargo test --locked --no-fail-fast` NOT run per this dispatch's own standing
instruction (may never finish on this box) — scoped suites above cover every file this cycle touched.

## 8. §15 — no undecidable Product Identity encountered

Every `deity` row renamed per the operator's own unconditional `§24` ruling for this kind (no
per-record judgment call to make — the row identity IS the PI content in every case, per `§24a`'s own
argument). Every `class_feature` name-PI row's disposition came from the existing `declared.name`/
blacklist union the generator already computed before this cycle; nothing encountered here was
ambiguous enough to stop on.

## 9. Gate 3's `no_record` figure, re-derived — NOT repinned

```
python3 scripts/shape_ledger.py --output .../ledger_after.json
```
```
population 35328, no_record 2664 (7.5%)
```
Per-kind breakdown of the ledger's own rows: `deity` and `class_feature` both absent from the
`no_record` list (both fully closed). Remaining `no_record` by kind: `monster_ability` 967, `feat`
682, `spell` 339, `companion` 217, `equipment_modifier` 175, `equipment` 170, `race` 59, `monster` 28,
`class` 21, `race_trait` 6. `NO_RECORD_BUDGET_COUNT`/`NO_RECORD_BUDGET_POPULATION` in
`shape_coverage_standing_gate.py` **untouched** — no budget constant edited this cycle.
