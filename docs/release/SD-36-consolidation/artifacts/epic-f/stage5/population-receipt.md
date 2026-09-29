# SD-36 Epic F1 stage 5 -- population run: the repaired rule package written in place

**Spec:** `docs/release/SD-36-consolidation/epic-f-class-completion.md` 3.5, "Run, scope and
diff". **Precondition (checked):** `git status --short` empty on `sd36/epic-f1` at
`a1089a10fd`; `data/sheet_rules/_report.json` baseline `records=49450 converted=49450
refused=0 rules_written=71862 var_tables=5309`; `data/sheet_rules/_defects/
unresolved-references.json` baseline length `11925`.

## 1. Converter run

`crates/codex-ingest/src/bin/sheet_rule_convert.rs`'s write form is `--write` (the corpus-wide
in-place rewrite of `data/sheet_rules`, gated behind that explicit flag per its own doc comment
-- SD-36 Epic F1 re-check round 1, finding 3).

**Preview first, in a scratch dump (never touching the tracked package), to build and prove the
structural-diff enumerated classes before committing to the in-place write:**

```
cargo run --locked --quiet -j 8 -p codex-ingest --bin sheet_rule_convert -- --dump <scratch>/dump-preview
records=49450 converted=49450 refused=0 rules=71863 var_tables=5882 dumped=55348 -> <scratch>/dump-preview (113.4s)
real 2m15.427s
```

**Then the real in-place write:**

```
cargo run --locked --quiet -j 8 -p codex-ingest --bin sheet_rule_convert -- --write
records=49450 converted=49450 refused=0 rules=71863 var_tables=5882 files=55348 (116.4s)
real 1m57.459s
EXIT=0
```

Both runs produced the same headline numbers (`rules=71863`, `var_tables=5882`), and the
official structural diff (section 2) confirms the in-place write's `data/sheet_rules/` is
byte-identical in content to what the scratch dump would have produced.

## 2. Structural diff: HEAD's committed package vs. the now-written working tree

`git stash` is forbidden on this checkout (whole-repo scope hazard). A read-only baseline tree
was taken instead:

```
git worktree add <scratch>/wt-baseline HEAD   # HEAD a1089a10fd, data/sheet_rules pre-write
python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py \
    data/sheet_rules --baseline <scratch>/wt-baseline/data/sheet_rules --max-examples 15
git worktree remove <scratch>/wt-baseline
```

Full output: `<scratch>/structural-diff-final.txt`. Summary:

```
== counts (baseline -> fresh) ==
  records: 49450 -> 49450
  converted: 49450 -> 49450
  refused: 0 -> 0
  rules_written: 71862 -> 71863
  var_tables: 5309 -> 5882

== expected delta classes ==
  added granted_by edges: 4491  (class_feature 3722, race_trait 426, companion 192, ability 118, domain 33)
  added grants: 102
  added _vars/ tables: 573
  added _defects/ files: 1
  provenance deltas on the pinned current_class-fix record list: 30 of 30 pinned records
  naturalattacks suffix-fix rename: 1290 of 1290 pinned old ids -> 1659 of 1659 pinned new ids
    (net +369), plus 8 field deltas on 6 of 6 pinned same-id content-shift ids
  added rule ids: 1
    ultimate_psionics:class:psion#bonus5: (current_class closure fix, commit 71c729e408)

unexpected field deltas: 0
removed granted_by edges: 0
removed grants: 0

verdict=PASS
```

`records 49450 -> 49450` unmoved. `rules_written` 71862 -> 71863 (+1, exactly the ONE new rule
`ultimate_psionics:class:psion#bonus5`, already named and explained by
`KNOWN_ADDED_RULE_CAUSES`). The NATURALATTACKS rename does NOT move `rules_written`: it is a
same-array-length SWAP of ids on entries the converter already emitted on both sides (1,290 old
ids replaced by 1,659 new ones is a REPACKAGING of which ids the shadowed vs. surfaced entries
carry, not new entries -- `rules_written` counts total JSON array entries, unaffected by which
id string an entry carries). What the rename changes is the LOADED, deduplicated count
(`package.rules.len()`): before this run it was 71,493 (369 short of `rules_written`, every one
silently shadowed); after this run it is 71,863 -- equal to `rules_written` for the first time,
with zero `DuplicateRuleId` diagnostics (section 4's corpus-loader gate). `var_tables` 5309 ->
5882 (+573, one table per newly-added `GatedFactGrant`'s condition variable -- F1-2's own
declared mechanism).

### 2.1 The NATURALATTACKS suffix fix's blast radius, confirmed and enumerated

The ORCHESTRATOR spec named this class as "will now ALSO change 369 previously-shadowed rule
ids". That figure is the class's **net** size (369 rules that were silently shadowed in the
tracked package now surface as live, distinct rules -- exactly `rule-gap-receipt.md`'s own
number). The **raw** id churn measured is larger, because the fix
(`crates/codex-ingest/src/pcgen_import/sheet_rule/convert.rs`'s `NATURALATTACKS` arm now
suffixing with `acc.lines.len()`, the record-global running count, instead of a
per-occurrence-local index reset to 0 every time the token recurs) renumbers the WHOLE
`#natural<N>` id family on every record whose line accumulator held any content before its
NATURALATTACKS arm ran -- not only the 306 groups that were previously colliding:

| | count | denominator |
|---|---|---|
| Old ids removed by the rename | 1,290 | every `#natural<N>` id present in the pre-write baseline and absent from the post-write package |
| New ids added by the rename | 1,659 | every `#natural<N>` id present post-write and absent pre-write |
| Net (= previously-shadowed rules now live) | **369** | `1659 - 1290`, matches `rule-gap-receipt.md`'s pinned figure exactly |
| Files touched | 1,133 | rule files under `data/sheet_rules/` whose `#natural<N>` id set changed at all |
| Same-id, different-content deltas | 6 ids / 8 field lines | ids the old and new numbering both happen to assign, to DIFFERENT attack content, because the whole record's numbering shifted (4 `bestiary_3:template:imperial_dragon_attacks_*#natural1`, 2 `occult_adventures:race:ghost_mount_{l,m}#natural1`) |

**Proof this is a pure rename, not a content change**, per file: `naturalattacks_rename_scan.py`
(committed, `docs/release/SD-36-consolidation/artifacts/epic-f/scripts/`) loads every touched
rule file on BOTH sides, filters to `#natural<N>`-suffixed entries, and asserts the MULTISET of
every field OTHER than `id` (label, dice/value, ...) is identical between the old and new array
for that file. Verified directly against the real chimera/glabrezu collision cases named in
`rule-gap-receipt.md`:

```
OLD bestiary:monster:chimera#natural0  | Bite (dragon head) | 2d6   <- literal in-array duplicate id
OLD bestiary:monster:chimera#natural0  | Bite (lion head)   | 1d8   <-   (both entries shared one id)
NEW bestiary:monster:chimera#natural8  | Bite (dragon head) | 2d6
NEW bestiary:monster:chimera#natural9  | Bite (lion head)   | 1d8
```

and against a non-colliding, single-attack record whose id simply moved (no duplicate ever
existed for it -- the running-count fix alone changed its number):

```
OLD bestiary:companion:companion_dinosaur_ankylosaurus#natural0 | Tail | 1d6
NEW bestiary:companion:companion_dinosaur_ankylosaurus#natural8 | Tail | 1d6
```

Run: `python3 naturalattacks_rename_scan.py <baseline_dir> <fresh_dir> <out_json>` ->
`PASS: 1133 files, 1290 old ids -> 1659 new ids (net +369), 6 same-id content-shift ids`. Every
one of the 1,133 touched files passed the multiset check (zero `bad_files`); had any failed,
the script exits 1 and names the offending file and its differing content instead of writing
the pinned list.

**The exact id list is committed beside the script**
(`docs/release/SD-36-consolidation/artifacts/epic-f/scripts/
structural_diff_naturalattacks_renames.json`: `old_ids` (1,290), `new_ids` (1,659),
`content_shift_ids` (6)), and `structural_diff.py` now loads it and treats it as one named,
enumerated class: `old_ids`/`new_ids` are excluded from `removed_rule_ids`/`added_rule_ids`
gating (never a blanket allowance -- an id off this exact pinned set still gates as an ordinary
add/remove), and a `content_shift_ids` id's `label`/`value` delta (the only two fields a rename
can ever touch) is excluded from `unexpected_field_deltas` gating; any OTHER field delta on
that same id still gates. This closed the run's initial `verdict=FAIL (removed rule ids: 1290;
unexpected field deltas: 8)` to `verdict=PASS` legitimately, not by loosening the gate's
general contract.

## 3. F1.3 -- links closed (per spec's exact acceptance command)

```
python3 -c "import json;print(len(json.load(open('data/sheet_rules/_defects/unresolved-references.json'))))"
7469
```

`11,925 - 4,456 = 7,469` -- **exact match.**

```
python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/unres2.py
```

`A by source kind []` / `distinct class sources in A 0` -- **mechanism A = 0**, confirmed.
`D categories` total **3,033** (unchanged), `E` (plain-category-not-ingested) **3,565**
(unchanged), `F` (dangling/malformed) **808** (unchanged, plus a new `B` bucket of 63 --
child-category cases the parent map now resolves, a byproduct of the SAME closure work
already landed on this branch). `63 (B) + 3033 (D) + 3565 (E) + 808 (F) = 7469` -- reconciles
to the exact defect-file count above.

## 4. Gates (all run against the now-written `data/sheet_rules/`)

| Gate | Command | Result |
|---|---|---|
| Package byte-reproducible | `cargo run --locked --quiet -j 8 -p codex-ingest --bin sheet_rule_convert -- --check` | exit 0 |
| Convert gate suite | `cargo test --locked -j 8 -p codex-ingest --test sheet_rule_convert_gate` | 39 passed, 0 failed (`package_on_disk_is_fresh_and_clean` now GREEN) |
| Residue gate | `python3 scripts/pcgen_residue_gate.py --check --closure` | `verdict=PASS` |
| Frozen status | `python3 scripts/site/check_frozen_status.py --check` | `OK: ... frozen at 100% (49450 units)` |
| `data/corpus`/`site/` untouched | `git status --porcelain -- data/corpus site \| wc -l` | `0` |
| Corpus bundle regen | `node scripts/gen-corpus-bundle.mjs` | `files_copied=14029` -- succeeds (bundle is gitignored, `apps/desktop/.gitignore:10`) |
| Corpus loader lib test | `cargo test --locked -j 8 --lib corpus_loader` | 9 passed, 0 failed -- `the_real_package_accounts_for_every_converted_rule`: `package.rules.len() == rules_written` (71,863), zero `DuplicateRuleId` diagnostics |
| Whole `codex` lib suite | `cargo test --locked -j 8 --lib` | 2666 passed, 0 failed, 7 ignored |
| Clippy | `cargo clippy --locked -j 8 --workspace --all-targets -- -D warnings` | clean |
| Desktop bundle parity | `cargo test --locked -j 8 --manifest-path apps/desktop/src-tauri/Cargo.toml corpus_bundle_parity` | 1 passed |
| Desktop sheet-rule tests | `cargo test --locked -j 8 --manifest-path apps/desktop/src-tauri/Cargo.toml --bin codex-desktop -- rules_and_features racial_trait sheet_rule` | 3 passed |
| Frontend typecheck | `cd apps/desktop && npm run typecheck` | clean |
| Frontend tests | `cd apps/desktop && npm test` | 125/125 test files passed |

## 5. Test-file updates this stage's population run made necessary

`rule-gap-receipt.md`'s own "Numbers, before/after" table named these as the expected follow-up
once the regen ran (their own doc comments said so):

- `src/rules_core/corpus_loader.rs::the_real_package_accounts_for_every_converted_rule`: the
  pinned `KNOWN_NATURALATTACKS_SUFFIX_COLLISION_COUNT = 369` tolerance assertion is gone
  (diagnostics dropped to 0, as the test's own panic message said it would once regenerated);
  the test now asserts `load.diagnostics.is_empty()` and `package.rules.len() == rules_written`
  directly -- no shadowed-rule exclusion needed any more.
- `src/rules_core/sheet_rule.rs::evaluate_tests::package()`: the `DuplicateRuleId`-only
  tolerance filter is gone; back to the original strict `load.diagnostics.is_empty()`.

Both changes are committed in the same commit as the data (section 6) -- the data and the test
that proves it holds a matching invariant are one unit; landing them separately would leave a
red gate in between.

## 6. Commit

`converter_version` checked: **unchanged** (`sheet_rule_convert/0.15.0` before and after) --
per spec, this means ONE commit, not a separate mechanical stamp commit.

```
git add data/sheet_rules \
        docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py \
        docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff_naturalattacks_renames.json \
        docs/release/SD-36-consolidation/artifacts/epic-f/scripts/naturalattacks_rename_scan.py \
        src/rules_core/corpus_loader.rs \
        src/rules_core/sheet_rule.rs \
        docs/release/SD-36-consolidation/artifacts/epic-f/stage5/population-receipt.md
git commit   # feat(sd36,epic-f1): repaired rule package -- 4,456 child-category links resolved, gated grants, weapon sets, no shadowed rule ids
git push origin sd36/epic-f1
```

Worktree clean (`git status --short`) after push.
