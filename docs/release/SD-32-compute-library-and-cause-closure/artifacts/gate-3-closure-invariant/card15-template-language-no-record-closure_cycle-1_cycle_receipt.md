# Cycle card15-template-language-no-record-closure — Gate 3 closure invariant / `decisions.md §20`

- **Card ID:** kanban.md card 15 (census-scope-closure), `decisions.md §20` no_record-to-zero mandate; assigned kind: `template` (+ `language` finished as the same-mechanism residual)
- **Commit SHA:** (recorded after push)
- **Files touched:**
  - `scripts/ingest_simple_filename_kinds.py` (fix — `resolve_out_dir()` honours `shape_ledger.BOOK_CORPUS_DIR_ALIASES`; `row_identity()` honours a row's own `KEY:` token over its leading display-name column)
  - `scripts/tests/test_ingest_simple_filename_kinds.py` (+5 unit tests: 2 `OutputDirAliasTests`, 3 `RowIdentityTests`)
  - `data/corpus/beastiary/{template,language}/*.json` (new — 1,050 + 14 records, the same content wave 1 wrote to the wrong `data/corpus/bestiary/` directory, regenerated through this cycle's own fixed script)
  - `data/corpus/{core_rulebook,horror_adventures,mythic_adventures,bestiary_2}/{template,language}/*.json` (new — 13 records: the citation-mismatch residual, now resolved via `KEY:` matching)
  - `data/corpus/bestiary/{template,language}/` (deleted — the misdirected copies)
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (card 15 notes cell, prepended entry; card left `in-progress` per dispatch instruction — other kind-unenumerable buckets remain open)
  - `docs/release/SD-32-compute-library-and-cause-closure/progress.md` (append)

## Rebase note — a concurrent lane fixed a third, related defect first

This cycle originally also fixed `source.path`'s missing leading `pathfinder/` segment (footgun 2
named in the dispatch brief). Mid-cycle, `git fetch`+`rebase` surfaced `af2f07f68` ("repair
`source.path` defect blocking `corpus_literal_sweep` corpus-wide") already landed on
`origin/tranche/12`, fixing the identical defect corpus-wide (all 3,124 affected records, not just
this cycle's two kinds) with a more careful, report-gated regen
(`CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` set, stamps diffed both directions).
Rather than resolve ~2,400 conflicting corpus-record hunks by hand, this cycle **discarded its own
`source.path` fix and superseding commit** (`git reset --hard origin/tranche/12`), re-derived
`template`/`language`'s `no_record` fresh against the new tip (confirmed **unchanged** — the repair
fixed the path *string*, not the two defects below), and re-applied only the two fixes still needed.
Named here per `decisions.md §17a`'s validate-before-trusting discipline, applied to this cycle's
own prior work rather than only to a sibling's claim.

## Scope and baseline — re-derived at the rebased tip (`22212f87e`, after `af2f07f68`) before writing anything

```
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/l_new.json
python3 -c "import json,collections; r=json.load(open('/tmp/l_new.json'))['rows']; print(collections.Counter(x.get('kind','?') for x in r if x['join_status']=='no_record').most_common())"
```
`template` 1,062, `language` 15 — unchanged from the dispatch brief's own figures at `857eb85d0`
despite the intervening `source.path` repair and a `spell` closure cycle (both touched different
records). Bundle-wide `no_record` at this tip: 8,092 (13 kinds; `spell`'s own concurrent closure
already dropped it from the brief's 8,434).

## Investigation — search for an existing ingest path first (mandatory per dispatch brief)

Wave 1's `scripts/ingest_simple_filename_kinds.py` already closed `template` 2,248 -> 1,062 and
`language` 136 -> 15 (its own receipt:
`artifacts/gate-3-closure-invariant/card15-simple-filename-kinds-ingest_cycle-1_cycle_receipt.md`).
**Its own re-derived numbers said 12/1 remaining, not 1,062/15** — the receipt's own claim was stale
against the live corpus. Investigating why surfaced two independent defects in that one script, not
a need for a new mechanism (`decisions.md §17`):

### Defect 1 — the writer never applied the bestiary/beastiary corpus-directory alias

`scripts/shape_ledger.py`'s `BOOK_CORPUS_DIR_ALIASES = {"bestiary": "beastiary"}` is read by the
*reader* (`build_corpus_index`) — `docs/work-inventory.json` names the book `bestiary`, but its
corpus directory is the historically-spelled `beastiary`. `ingest_simple_filename_kinds.py`'s writer
computed `out_dir = os.path.join(out_root, book, kind)` with the **unaliased** `book`, so every
`bestiary`-book `template`/`language` record it wrote landed in `data/corpus/bestiary/` — a directory
`shape_ledger.py` never reads for that book. 1,050 `template` + 14 `language` units, real corpus
JSON on disk, permanently invisible to the join.

**This is the exact footgun the dispatch brief named** ("A lane wrote 30 records to the wrong
directory and only caught it by re-deriving after push") — refired by a second writer against the
same alias, at 35x the scale, and confirmed still present at the rebased tip (the `source.path`
repair touched the `path` field's string content, not the directory a record is written to).

Fix: extracted `resolve_out_dir(out_root, book, kind)`, importing
`BOOK_CORPUS_DIR_ALIASES` directly from `shape_ledger` (no second alias table to drift from the
reader's).

### Defect 2 — citation matching ignored a row's own `KEY:` token

The remaining 12 `template` + 1 `language` "citation mismatches" (wave 1's receipt named these
honestly rather than force-matching) turned out not to be naming drift at all. PCGen's LST format
lets a row declare an explicit `KEY:` field that **overrides** the leading display-name column as
the row's real identifier — the identical convention already used elsewhere in this repo
(`src/bin/ingest_races.rs`'s `row.first("KEY")`, `src/bin/ingest_race_traits.rs`'s
`.find(|f| f.key == "KEY")`, `scripts/derive_monster_ability_save_dc_fixtures.py`'s
`token(fields, "KEY") or fields[0]`). Every one of the 13 mismatched rows carries a `KEY:` token
whose value is byte-identical to the inventory's `corpus_key` — e.g. `ma_templates.lst:15`:
`Has Swim Speed<TAB>KEY:Swimming Master ~ Has Swim<TAB>...`, inventory `corpus_key` =
`"Swimming Master ~ Has Swim"`. Verified all 13 by hand against the pinned oracle before writing the
fix (`docs/release/.../artifacts/corpus/operator-supplied/pcgen/data/pathfinder/.../ma_templates.lst`,
`ce_templates_familiar_cr.lst`, `ha_templates.lst`, `fetchling_languages.lst`).

Fix: `row_identity(raw_line)` returns the `KEY:` token's value when present, else falls back to the
leading column (unchanged behaviour for the ~99.4% of rows with no `KEY:` token).

## Verification

**Unit tests (RED->GREEN proved live, both fixes independently):**
```
python3 -m unittest scripts.tests.test_ingest_simple_filename_kinds -v
# 18/18 OK (includes the sibling af2f07f68 lane's own 3 ComposeSourcePathTests, unmodified)
```
- Defect-1 RED: mutated `resolve_out_dir` to drop the alias lookup (`os.path.join(out_root, book,
  kind)`) — `test_out_dir_book_segment_matches_shape_ledger_alias_for_bestiary` failed
  (`'data/corpus/bestiary/template' != 'data/corpus/beastiary/template'`); the sibling non-aliased-book
  test stayed green, confirming the mutation's blast radius was exactly the alias branch. Reverted,
  re-ran GREEN.
- Defect-2 RED: mutated `row_identity` to always return the leading column — 
  `test_key_token_overrides_leading_column_when_present` failed
  (`'Has Swim Speed' != 'Swimming Master ~ Has Swim'`); the fallback and KEYSTAT-guard tests stayed
  green. Reverted, re-ran GREEN.

**Sibling suite (unaffected, run to confirm no collateral regression to the reader side):**
`python3 -m unittest scripts.tests.test_shape_ledger` — 30/30 OK.

**Regeneration discipline:** every touched `data/corpus/**` path was written by
`scripts/ingest_simple_filename_kinds.py` (fixed), never hand-edited. Procedure:
1. `rm -rf data/corpus/bestiary/template data/corpus/bestiary/language` (the misdirected copies,
   still present at the rebased tip — the `source.path` repair did not move them).
2. Re-ran the fixed script scoped `--kind template --kind language` (idempotent — full population,
   not just the affected books). `template:written` 2,248/2,248, `language:written` 136/136,
   `citation_mismatches: []` — zero, down from 13.
3. The re-run also rewrote the ~2,371 already-correctly-placed records (same content, new
   `ingested_at` timestamp only — confirmed via `git diff` on a sample, byte-identical apart from
   that one field; `source.path` was already correct on these from `af2f07f68`, so this pass did not
   touch it). Reverted that timestamp-only churn on the unaffected files via
   `git checkout -- 'data/corpus/*/template/*.json' 'data/corpus/*/language/*.json'` (glob pathspec,
   restores tracked-file content to HEAD without touching the untracked `beastiary/` additions)
   **then re-deleted `data/corpus/bestiary/{template,language}/`** — the glob checkout also restores
   deletions under the same pathspec, so the wrong-directory files came back and had to be removed a
   second time. Final `git status --porcelain` on `data/corpus/`: 1,064 `D` (the misdirected
   originals) + 15 new files/dirs under the correct paths, zero `M`.
4. No existing `data/corpus` record outside `template`/`language` was touched (confirmed:
   `git status --porcelain` shows no `M`/`D`/`??` under any other kind directory).

**Result, re-derived after the write:**
```
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/l_final3.json
python3 -c "import json,collections; r=json.load(open('/tmp/l_final3.json'))['rows']; print(collections.Counter(x.get('kind','?') for x in r if x['join_status']=='no_record').most_common())"
```

| Kind | Before (rebased tip) | After | Closed |
|---|---:|---:|---:|
| `template` | 1,062 | **0** | 1,062 |
| `language` | 15 | **0** | 15 |

Neither kind appears in the post-fix `no_record` Counter output at all (zero, not "small").
**Bundle-wide `no_record`: 8,092 -> 7,015 (-1,077)**, re-derived at
`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.

**No regression check** — every other kind's `matched`/`no_formula_tokens`/`no_record` count diffed
before vs after, full population (35,423 rows both runs): only `template` and `language` moved.
Command: diff the two `/tmp/l*.json` outputs' `Counter` per `join_status` per `kind`; zero non-zero
diffs for any of the other 13 kinds.

**PI / template-vs-object status (`decisions.md §15`/`§16`):** not re-litigated. `template`'s full
2,343-unit disposition (object, not a modifier on an object already counted) was already settled by
`artifacts/gate-0-census-closure/15-card-15-other-kinds-memo.md` §1 (0 `.COPY=` derivations, 0-hit
join against every other kind) and wave 1's PI screen already ran over every unit this cycle wrote
(the fix only changed *where*/*whether* a citation-verified record gets written, not the PI-screening
logic in `main()`, which is untouched). PI redaction counts are unchanged from wave 1's own figures
(`template` 39, `language` 19) since the citation-verified population is now a strict superset of
what wave 1 screened, applying the identical unmodified screen.

**Identifier audit** (own diff vs `HEAD`, `scripts/ingest_simple_filename_kinds.py` +
`scripts/tests/test_ingest_simple_filename_kinds.py`): `OK_NO_BUNDLE_TAGS`.

**Wired-integration audit** (same diff): `OK_NO_TOKENS`.

**Pinned-count sweep:** grepped `2,248`/`2248`/`1,062`/`1062` across `tests/`, `src/`, `scripts/`,
`apps/` — the only hits are unrelated `source_line: 1062` fields in
`src/rules_core/rules_tables/{bestiary,bestiary_3}/monster_data.rs` (line numbers, not counts). No
pinned assertion needed updating.

**Full unscoped `cargo test` NOT run** (dispatch instruction: it may never finish on this box). This
cycle adds no Rust code — no `.rs` file in this cycle's diff — so no Rust suite is affected.

**Reachability:** not claimed. `template`/`language` remain `wiring_class: "display"` with zero
`reach_gate.rs` entry, unchanged from wave 1's own honest scoping — this cycle closes Gate 1
shape-measurability only.

**Corpus SHA:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
(`scripts/pcgen-oracle-pin.env`), bootstrapped fresh in this worktree via
`scripts/fetch-pcgen-oracle.sh` (a fresh worktree's oracle slot is git-ignored and starts empty).

- **Status:** complete (`template` and `language` both fully closed — 0 `no_record` for either kind)
- **Notes:** the bestiary/beastiary alias defect found here is **generic to this one writer script**,
  not to the pipeline as a whole — but the *reader's* alias table only covers `bestiary`; if any
  sibling lane's own writer computes its own `out_dir` from an unaliased `book` field the same way,
  it will reproduce this exact failure for its own kind. Also: this cycle's own mid-flight discovery
  (a sibling lane landing an overlapping fix first, `af2f07f68`) is itself an instance of
  `decisions.md §17a`'s lesson — re-derive before trusting a prior state, including one's own.
- **Discovery forwards:** none opened as new backlog items — both defects were found, fixed, and
  closed within this cycle.
- **Next-cycle plan:** card 15's other kind-unenumerable buckets (`class_feature` 18,231/15,439
  disagreement, `ability_category:*` 5,886, `unclassified:<file>` 179) remain open, per
  `decisions.md §12b`/§16 — out of this cycle's assigned kind (`template`). Bundle-wide `no_record`
  is 7,015 across 13 remaining kinds (`race_trait` 1,859 down to `monster` 28) — reported here per
  `decisions.md §12c`'s "state every population" rule, not this cycle's own scope to close.
