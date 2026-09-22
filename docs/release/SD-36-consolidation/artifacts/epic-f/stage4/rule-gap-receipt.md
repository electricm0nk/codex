# SD-36 Epic F1 -- rule-count gap between `_report.json` and the live package

**Question:** `data/sheet_rules/_report.json` says `rules_written = 71862`. The live loader
(`rules_core::corpus_loader::load_sheet_rules`, called by both `rules_core::sheet_rule_package`
and the desktop crate's `character_hub::sheet_rule_package`) only ever held 71,493 rules in
`SheetRulePackage.rules`. Where did the other 369 go, and is losing them a live defect?

**Denominators, exactly, every command given:**

- `data/sheet_rules/_report.json`: `records=49450`, `converted=49450`, `refused=0`,
  `rules_written=71862`, `var_tables=5309`. (`python3 -c "import json; print(json.load(open('data/sheet_rules/_report.json')))"`)
- `cargo test --locked -j 8 --lib -- rules_core::sheet_rule::evaluate_tests::package`
  (via `eprintln!` in the test): loaded package holds `71493` rules from `49450` rule files,
  `5309` var files -- matches `_report.json`'s `records`/`var_tables` exactly; only
  `rules_written` diverges from the loaded count.

## Method

A script (`rule_gap_scan.py`, this directory) replicates
`corpus_loader::load_sheet_rules_filtered`'s exact walk byte-for-byte (same top-level `_vars`/
`_`-prefix skip, same recursive per-kind-dir walk skipping `_parity/` and `LICENSE.json`,
same sorted file order) over the real, tracked `data/sheet_rules/`, but WITHOUT deduping by
rule id -- it counts every `SheetRule` JSON-array entry across every file, and separately
tracks, per id, every file+index that wrote it, in the same insertion order the real loader
uses (sorted path order, then array order within a file).

```
rule_files_found            = 49450
total_array_entries         = 71862   <- matches _report.json's rules_written exactly
parse_failures               = 0       <- every file parses (no silent deserialize-skip)
unique_rule_ids_after_dedup  = 71493  <- matches the loaded package.rules.len() exactly
duplicate_id_groups          = 306
shadowed_entries (lost)      = 369    <- 71862 - 71493, exactly
```

Run: `python3 rule_gap_scan.py data/sheet_rules dup_ids.json` from the repo root.

**Ruled out, by direct evidence, not assumption:**
- **Files the loader does not walk** (a directory, an extension, a `_`-prefixed dir): no --
  `rule_files_found` (49,450) already matches `_report.json`'s `records` (49,450) exactly, and
  every file the script's independent walk found is the same file the loader's own
  `find_json_files` would find (same walk logic).
- **Deserialize failures skipped silently**: no -- `parse_failures = 0`; every file is valid
  `Vec<SheetRule>` JSON. The pre-existing `evaluate_tests::package()` test already asserted
  `load.diagnostics.is_empty()` before this investigation and was GREEN, which independently
  confirms this (parse-failure diagnostics were the only kind that existed before this fix).
- **Multi-rule files where only the first is read**: no -- every entry in every file's JSON
  array is read and inserted; nothing is truncated at read time.

**The real cause: same-id collisions, `SheetRulePackage::insert_rule`'s silent overwrite.**
`package.rules` is a `BTreeMap<RuleId, SheetRule>`
(`src/rules_core/sheet_rule.rs::SheetRulePackage::insert_rule`:
`self.rules.insert(rule.id.clone(), rule)`). When two different `SheetRule`s share an id,
the later write silently replaces the earlier one in the map -- no diagnostic, no error,
nothing. All 306 duplicate-id groups (369 shadowed rules) are this shape, and every single
one is **within one file**, not across files (`dup_ids.json`: every group's `writes` list
names the SAME file 2-4 times).

**Root cause of the id collisions themselves (traced into the converter):**
`crates/codex-ingest/src/pcgen_import/sheet_rule/convert.rs`'s `NATURALATTACKS` match arm
suffixed every emitted `Line` `format!("natural{i}")`, where `i` is the position within ONE
`v.split('|')` call -- reset to 0 every time this match arm runs. A record whose source row
carries the `NATURALATTACKS:` tag more than once (the real shape, verified against two of the
306 real ids: `bestiary:monster:chimera`'s `b1_races.lst:69` carries it TWICE, once per
draconic head, each a single-entry occurrence; `bestiary:monster:demon_glabrezu`'s
`b1_races.lst:95` carries it THREE times, bite + claws + pincers) hits the arm 2-4 times per
record, and every occurrence's first (only, for a single-entry occurrence) attack mints
`#natural0` again -- one head's Bite rule silently overwrites the other's.

Every OTHER multi-emit arm in this same match (`weapon{}` / `bonus{}` / `spell{}_`, a few arms
below `NATURALATTACKS`) already suffixes with `acc.lines.len()` -- the record-GLOBAL running
line count, unique no matter how many times a token occurs -- for exactly this reason. Only
`NATURALATTACKS` used the local, per-occurrence index instead.

## Evidence this is a LIVE DEFECT, not benign report/loader noise

`subject=Character`, `print=true` for 292 of the 306 duplicate ids (the rest are `print=false`
selector/helper lines); every one is a `monster` (274), `template` (13), `companion` (12),
`race_trait` (6) or `class_feature` (1) natural-attack line -- content a held character or
monster stat block prints on its sheet. A chimera character/NPC today prints only ONE of its
two dragon-head bites; a demon glabrezu prints only one of its bite/claw/pincer trio. Content
the converter produced never reaches a sheet -- exactly the shape
`docs/governance/no-stub-mvp-doctrine.md` names a live defect, not a silent report/loader
mismatch to wave off.

## Fix

1. **Converter, root cause** (`crates/codex-ingest/src/pcgen_import/sheet_rule/convert.rs`,
   `NATURALATTACKS` arm): suffix now uses `acc.lines.len()`, matching the established
   `weapon{}`/`bonus{}`/`spell{}_` convention, instead of the per-occurrence-local `i`. Proven
   directly against the converter with two real, pinned-oracle units
   (`crates/codex-ingest/tests/sheet_rule_natural_attack_suffix_collision.rs`: chimera's 2-way
   collision, demon_glabrezu's 3-way collision) -- RED against the pre-fix code (confirmed by
   temporarily reverting the fix and re-running: both tests panic naming the collided
   `#natural0` id), GREEN after.
   - This fix takes effect at the NEXT `sheet_rule_convert -- --write` regen. This stage's own
     invariant is that the TRACKED `data/sheet_rules/` is not regenerated here, so the 369
     shadowed entries remain shadowed in the current on-disk package until that regen runs.

2. **Loader, defense-in-depth** (`src/rules_core/corpus_loader.rs`,
   `load_sheet_rules_filtered`): a rule id collision is now a loud, named
   `SourceContentDiagnosticKind::DuplicateRuleId` diagnostic (new variant,
   `src/rules_core/source_content.rs`) naming the exact id and the file that shadowed an
   earlier write -- never a silent `BTreeMap` overwrite with zero trace. This is a permanent
   regression guard: if a FUTURE converter change (any arm, not just NATURALATTACKS) ever
   mints two rules under the same id again, this diagnostic fires immediately instead of
   quietly dropping content for another `rules_written` vs. loaded-count investigation to
   rediscover from scratch.
   - New test, `corpus_loader::tests::the_real_package_accounts_for_every_converted_rule`:
     asserts `package.rules.len() + diagnostics.len() == rules_written` (read live from
     `_report.json`) -- every converted rule is accounted for as either LIVE or a named,
     justified exclusion, never an unexplained gap -- AND that every diagnostic's message is
     the one justified, already-fixed class (`#natural` suffix); any OTHER duplicate-id class
     fails the test loudly instead of silently re-pinning a bigger/different number. The
     current pinned count (369) is named by mechanism in the test's own doc comment, with
     the path back to this receipt and to `rule_gap_scan.py` for re-deriving it.
   - `evaluate_tests::package()` (`src/rules_core/sheet_rule.rs`) updated to tolerate ONLY
     this one named, justified diagnostic class (previously asserted `diagnostics.is_empty()`
     outright, which the new diagnostic would otherwise break against this stage's
     un-regenerated tracked data).

3. **Desktop loader**: confirmed to have the SAME defect, because it is not a separate
   reimplementation -- `apps/desktop/src-tauri/src/character_hub.rs`'s `sheet_rule_package()`
   calls `codex::rules_core::corpus_loader::load_sheet_rules` directly (line 727), the exact
   same function fixed above. No separate desktop-side code change is needed; the fix and its
   loud-diagnostic defense apply to the desktop's shipped path automatically. Desktop crate
   confirmed green after the fix (`cargo test --locked -j 8 --manifest-path
   apps/desktop/src-tauri/Cargo.toml --bin codex-desktop`, filtered to the sheet-line/racial-
   trait tests that exercise `sheet_rule_package()`/`sheet_lines_for` -- all pass).

## Numbers, before / after this stage's changes

| | before | after |
|---|---|---|
| `rules_written` (`_report.json`, unregenerated) | 71862 | 71862 (unchanged; `data/sheet_rules/` not touched this stage) |
| `package.rules.len()` (live loaded package) | 71493 | 71493 (unchanged -- the SAME 369 rules remain shadowed on THIS tracked package until the next regen) |
| Silent gap | 369, unexplained, undiagnosed | 0 -- every one of the 369 is now a loud, named `DuplicateRuleId` diagnostic, pinned by a test that fails if the count or its cause class ever changes unexpectedly |
| Converter (`convert.rs`) | mints colliding `#naturalN` ids across repeated `NATURALATTACKS` occurrences | mints a unique suffix per record; proven against 2 real multi-way collisions |
| Next `sheet_rule_convert -- --write` regen (not run this stage) | -- | expected to raise `rules_written`'s loaded-count match to 71862/71862 (0 duplicates); `corpus_loader::tests::the_real_package_accounts_for_every_converted_rule`'s pinned `369` and `evaluate_tests::package()`'s tolerance list should both be deleted at that point per their own doc comments |

## Verification run this stage

- `cargo test --locked -j 8 --lib` (whole `codex` crate): 2641 passed, 0 failed, 7 ignored.
- `cargo test --locked -j 8 -p codex-ingest --test sheet_rule_natural_attack_suffix_collision`:
  2 passed (both RED-before/GREEN-after verified by temporarily reverting the convert.rs fix).
- `cargo clippy --locked -j 8 --workspace --all-targets -- -D warnings`: clean.
- `python3 scripts/pcgen_residue_gate.py --check --closure`: `verdict=PASS`.
- `cargo test --locked -j 8 -p codex-ingest --test sheet_rule_convert_gate
  package_on_disk_is_fresh_and_clean`: still FAILS, as EXPECTED and invariant for this stage
  (the tracked package is not regenerated here).
- `cargo check --locked -j 8 --manifest-path apps/desktop/src-tauri/Cargo.toml`: clean.
- `cargo test --locked -j 8 --manifest-path apps/desktop/src-tauri/Cargo.toml --bin
  codex-desktop -- rules_and_features racial_trait sheet_rule`: 3 passed (sheet-line-path
  desktop tests).
- `git status --short data/corpus data/sheet_rules site`: empty -- invariant held.
- `data/sheet_rules/_report.json`'s `records`: 49450 -- frozen count unmoved.
