# Cycle epic-2-ability-ingest — gate-3-closure-invariant / `decisions.md §20`

- **Card ID:** `epic-2-cause-closure` (kanban row 11; rows 11 and 15 left `in-progress` per dispatch
  instruction).
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`;
  `scripts/verify.sh --only preflight-oracle` → PASS, oracle bootstrapped fresh into the repo-local
  `artifacts/corpus/operator-supplied/pcgen` slot, never `~/workspace/repos/pcgen`).

## Scope

`decisions.md §20`: `Kind::Ability`'s 4,824 enumerated units (landed by the `card-15-ability`
cycle, `15-ability_cycle_receipt.md`) were 100% `join_status: no_record` — enumerated but never
ingested, so Gate 1's "every unit's shape is measured" was unmet for the whole kind. This cycle
ingests them.

## §17a re-derivation before building anything

The brief's "4,824 un-ingested units" figure held (re-derived below, before and after). The
brief's `docs/release/.../15-ability_cycle_receipt.md` pointer was read in full, as instructed —
it is the classifier's own memo of what an ability row looks like, and was the design guide for
this cycle's PI screen and token transcription (same `_ABILITY_CONTENT_RE`-adjacent shape: bare
`*_abilities*.lst` rows, KEY:/CATEGORY:/TYPE:/DEFINE:/BONUS:/DESC: tokens).

## Search for an existing ingest path — found a real, reusable mechanism; ported it, did not fork it

`grep -rl "data/corpus" src/bin/` and a read of `src/rules_core/cache_gen/class_feature.rs` found
`class_feature`'s own generator (`cache_gen::class_feature::generate`, SD-31 E5-F1) is the SAME
LST-token-to-JSON transcription pattern `ability` needs: given a unit's own
`(book, source_file, source_line, key, name)` citation (already established by
`v06_work_inventory`'s enumeration, never re-derived), read the cited row, tab-tokenize it
(skip the identity column, split each field on its first `:`), PI-screen name and description,
and write one JSON record citing the real corpus line. `ability` does NOT need `class_feature`'s
class-resolution machinery (7-tier owner-class inference, `.MOD` grant-fact lookups) — ability
rows are not owned by a class the way class features are — so this cycle ports the shared
*shape* (citation → tokens → PI screen → JSON write), not the class-specific code, as a new,
smaller module rather than importing `class_feature.rs`'s private (non-`pub`) helpers across a
module boundary they were never designed to cross.

`scripts/transcribe_monster_tables.py` (the brief's other named precedent) was also read in full.
It is NOT reusable here: its `ability`-adjacent parsing (`parse_special_ability_refs`,
`find_internal_bundle_ability_refs`) exists to resolve `monster_ability` rows back to an OWNING
monster row — a relationship `ability`'s own bare, standalone rows do not have. Its
`resolve_book_file`/`_find_under` book-directory-resolution helper, however, is exactly the
generic mechanism this cycle's population needs (28 books, no fixed per-book file list, several
`decisions.md §9`-reattributed `core_essentials` files) — ported (re-derived independently in
Python, not imported, since `transcribe_monster_tables.py`'s own `resolve_book_file` is scoped to
its `BOOKS` dict of 9 books and this cycle's population spans all 28 book directories the
inventory itself names, verified by direct `os.walk` resolution check below).

**PI screen: reused, not forked, and upgraded to the amended blacklist.** `src/rules_core/pi_screening.rs`
(the Rust module `class_feature.rs` calls) still carries the pre-`decisions.md §19a` 57-term,
bare-substring list — `docs/governance/ogl-pi-blacklist.md`'s own frontmatter states the 60-term,
word-boundary, OCR-normalized amendment is deliberately NOT yet applied there, because "that
change belongs to the T9 onboarding cycle that actually transcribes corpus data under this
amended blacklist." This cycle transcribes corpus data under exactly that condition, so it
imports `scripts/sd32_t9_pi_review_feat_equipment.py`'s own `normalized_term_hit`/
`extract_free_text`/`PI_BLACKLIST_TERMS` (60 terms) directly — the corrected, operator-approved
implementation — rather than writing a fifth copy of the term list.

## A defect found and fixed before landing anything (validate before trusting a confident claim)

An early draft mirrored `transcribe_monster_tables.py::read_row`'s soft-hyphen (U+00AD)
substitution (`.replace(SOFT_HYPHEN, "-")`), because the brief's own named precedent uses it.
Running `corpus_literal_sweep` against the first draft's 4,424 written records found **exactly
one** `MISMATCH`: `data/corpus/inner_sea_gods/ability/hellfire_blast.json`'s `DESC` token did not
byte-match `isg_abilities_faith.lst:53`'s cited row. Root cause: `transcribe_monster_tables.py`'s
substitution serves a DIFFERENT consumer (a compiled Rust source table, where
`clippy::invisible_characters` is deny-by-default) than this cycle's (a JSON record
`corpus_literal_sweep` independently re-derives byte-for-byte from the same cited bytes) — the two
pipelines need opposite behaviour at this one point. Fixed by removing the substitution entirely
(the cited row is transcribed byte-verbatim, invisible character included); re-ran the full ingest
from a clean `data/corpus/*/ability/` state; re-ran `corpus_literal_sweep`: **0 findings** (see
below).

The same review pass found a second, independent gap: the PI name-screen only scanned each unit's
bare `name` field, never its full `key`. `isg_abilities_faith.lst:53`'s own row is the live
counter-example — `name: "Hellfire Blast"` (clean) but `key: "Exalted Boon ~ Asmodeus ~ Hellfire
Blast"` (carries the blacklisted deity "Asmodeus"). Fixed by scanning both `name` and `key`
before the union with the row's own `NAMEISPI:YES` declaration. This raised `name_pi_skipped` from
400 to 576 in the corrected run — every one of those 176 additional skips is a real key-embedded
term (spot-checked; see "PI screening" below), not a regression.

## What landed

**`scripts/ingest_ability.py`** (new, ~290 lines) — a single generic pass, driven entirely by
`docs/work-inventory.json`'s own `kind: "ability"` units (no per-book table):

1. `build_dir_index(root)` — one `os.walk` over `PCGEN_CORPUS_ROOT`, indexing every directory by
   its basename. Verified against the live population before writing anything: all 28 book ids the
   `ability` population names resolve to exactly one directory each (`missing: []`, `multi: []`).
2. `resolve_file(...)` — finds a unit's `source_file` under its book's directory, falling back to
   `core_essentials` for the `decisions.md §9`-reattributed rows (mirrors
   `transcribe_monster_tables.py::resolve_book_file`'s two-pass rule: book's own root always wins;
   `core_essentials` only checked when the book's own directory has no match; ambiguous >1-hit
   resolution is refused, never guessed). Verified: all 102 (book, source_file) pairs in the
   population resolve to exactly one real file.
3. `read_row`/`row_tokens` — byte-verbatim row read (no substitution, see the defect above), tab
   split, skip the identity column, split each field on its first `:`.
4. PI screen (declared `NAMEISPI:`/`DESCISPI:` union with the amended 60-term normalized blacklist
   scan on `name`, `key`, and the row's own free-text tags) — `decisions.md §15`/`§19` standing rule:
   a name-level hit skips the WHOLE record (reported by name/key/line, never silently dropped); a
   description-level hit redacts `DESC` (both `data.description` and the matching `raw_tokens`
   entry) to `shape_b_v1::REDACTED_PI_MARKER`, matching `class_feature.rs`'s own
   `redact_desc_token_if_pi` precedent.
5. `wiring_class` — `static` (signal `static:has_magnitude_token`) when `raw_tokens` carries a
   `DEFINE`/`BONUS*` token, else `display` (`display:no_magnitude_token`). This is a narrower,
   honest default, not the full `.MOD`/`.COPY=` closure analysis `cache_gen::WiringClassIndex`
   performs — named as a scope limit below, not silently assumed equivalent.
6. Writes `data/corpus/<book>/ability/<slug>.json`, one file per unit, `source.path` set to the
   file's REAL path relative to `PCGEN_CORPUS_ROOT` (needed for `corpus_literal_sweep`'s own
   `book_dir_of` re-derivation to find the right bytes — independent of which `<book>` directory
   the record is written under).

## Population, before and after

**This cycle's own run** (`python3 scripts/ingest_ability.py --out <report.json>`):

```
population: 4824
written:    4248
name_pi_skipped: 576  (§15 stop — every one named in the run's own report, see below)
unresolved: 0
```

**`shape_ledger.py`** (`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/l.json`
then `Counter(x['kind'] for x in rows if x['join_status']=='no_record')`):

| | before | after | delta |
|---|---:|---:|---:|
| `ability` `no_record` | 4,824 | 576 | **−4,248** |
| `ability` `matched`/`no_formula_tokens` (combined) | 0 | 4,248 | +4,248 |

The 576 residual `no_record` units are EXACTLY this run's own `name_pi_skipped` count — every
un-ingested unit is accounted for by a named §15 stop, none silently missing.

## §15 — Product Identity: 576 records stopped, named by the run's own report

Every one of the 576 records this cycle did NOT transcribe is listed by
`(book, source_file, line, name, key, reason)` in `/tmp/ability_report2.json`'s
`pi_skipped_records` array (reproduced in full at
`docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-3-closure-invariant/17-ability-pi-skipped.json`,
committed with this cycle so the list survives the scratch file). Spot-checked a representative
sample (all of `advanced_players_guide/apg_abilities.lst`'s 15 named Trait rows: "Asmodean Demon
Hunter (Asmodeus)", "Calistrian Prostitute (Calistria)", "Divine Warrior (Iomedae)", ...) — every
hit is a genuine deity/place name embedded in the record's own display name or key, not a
false-positive ("next"/"Nex"-class collision; the word-boundary guard holds). None were
transcribed, none silently skipped: every one is named in the committed list. **These 576 units
stay `no_record` until an operator PI ruling (name-carrying-PI has no redaction path — `§15`'s own
"a name cannot be redacted" rule) clears them, matching this cycle's own scope limit, not a defect
to fix in a later cycle without one.**

## §16 — a unit moved out of a shape is not a unit closed

No `ability` unit was reclassified into another kind by this cycle. All 4,248 written records keep
`kind: ability` in the sense that matters here — they are still exactly the units
`docs/work-inventory.json` already enumerates as `ability`; this cycle only adds their corpus
record. `population: "in_scope"` on every written record (never re-scoped).

## Fixture discipline (`decisions.md §3`) — `corpus_literal_sweep`

```bash
export PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data
cargo run --locked --bin corpus_literal_sweep
```

First run (soft-hyphen substitution present): 1 MISMATCH (`hellfire_blast.json`, see "A defect
found and fixed" above). After the fix and a clean re-run of the full ingest:

```
corpus-literal-sweep: 30786 records examined of 31956 read, 285525 tokens compared (9 synthesized),
31943 digests checked, 0 findings
corpus-literal-sweep: CLEAN
```

Every one of the 4,248 written `ability` records' `raw_tokens` matches the pinned oracle's cited
bytes exactly.

## Reachability (`apps/desktop/src-tauri/src/reach_gate.rs`) — honest claim: 0

`reach_gate.rs` defines no `AbilityRecord`/`"ability"` reachability entry at all (`grep -n
"ability\|Ability"` finds only `MonsterAbilityRecord`/`CompanionAbilityRecord`, a different kind).
No engine consumes `Kind::Ability` yet — Gate 2 (engines) has not run against this population, as
the `card-15-ability` receipt already flagged ("enumerated, not engineered"). **This cycle claims
zero reachability for `ability`.** It closes Gate 1 measurability (the shape can now be read from
a real corpus record) for 4,248 of 4,824 units; it does not claim any of them reach a player. That
is a separate, unattempted Gate 2 cycle.

## Tests / RED → GREEN

- No Rust code touched this cycle (Python-only generator + JSON data), so no Rust test suite is
  affected by this cycle's own diff. `docs/work-inventory.json` is untouched (0 stamps at risk).
- **RED → GREEN, real, on the generator itself**: the soft-hyphen defect above IS this cycle's
  RED→GREEN — `corpus_literal_sweep` failing with a named MISMATCH is the red proof (the generator
  really did diverge from the corpus for the intended, findable reason), the fix is the minimal
  correction (remove the one substitution), and the full re-run + re-sweep is the green proof
  (0 findings). The key-scan gap is the same shape: found by manual review of the PI-skip list
  rather than by an automated red proof (no unit test harness exists for this new script this
  cycle; noted as a real gap below, not hidden).
- `scripts/ingest_ability.py` has no committed unit tests of its own — reported, not hidden. A
  follow-up should add `scripts/tests/test_ingest_ability.py` covering `resolve_file`'s
  ambiguous/missing/core_essentials-fallback cases and the name-vs-key PI scan, mirroring
  `scripts/tests/test_census_independent.py`'s own style.

## Identifier / wired-integration audit (this cycle's own diff)

```bash
git diff --unified=0 <pre-cycle-sha> -- scripts/ingest_ability.py \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
```

Two hits, both are literal references to the ALREADY-EXISTING module
`scripts/sd32_t9_pi_review_feat_equipment.py` this cycle imports (its own real filename, not a new
identifier this cycle invented) — `from sd32_t9_pi_review_feat_equipment import (...)` and one
module-doc-comment mention. Not a new bundle-tag leak; the file predates this cycle
(`decisions.md §18`/`§19` chain). No other match.

```bash
git diff --unified=0 <pre-cycle-sha> -- scripts/ingest_ability.py \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
```

`OK_NO_TOKENS`.

## Files touched

- `scripts/ingest_ability.py` — new.
- `data/corpus/<book>/ability/*.json` — 4,248 new records across 28 books (never hand-edited;
  written only by the generator above, regenerated end-to-end from a clean state after the
  soft-hyphen fix).
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-3-closure-invariant/17-ability-pi-skipped.json`
  — the 576 named §15 stops (committed so the list is not scratch-only).
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-3-closure-invariant/epic-2-ability-ingest_cycle-1_cycle_receipt.md`
  — this receipt.

## A third defect found post-push, after rebasing onto sibling lanes' work

After the initial push (`c240206cc`), re-running `shape_ledger.py` against the freshly-rebased
branch showed `ability` `no_record` at **606**, not the expected 576 — a 30-unit regression. Cause:
`scripts/shape_ledger.py::BOOK_CORPUS_DIR_ALIASES` (landed by a sibling `t9-monster-companion-race-
no-record` cycle earlier the same day, `8970327b0`) maps the inventory's `book: "bestiary"` to the
historical directory spelling `data/corpus/beastiary/` for the join walk — every OTHER kind's
`bestiary`-book records already live there. This generator wrote its 30 `bestiary`-book records
under the literal (and, for this book, wrong) `data/corpus/bestiary/ability/`, invisible to the
join. Fixed two ways: (1) `git mv`'d the 30 existing files to `data/corpus/beastiary/ability/`
(no content change, path only); (2) added the same `CORPUS_WRITE_DIR_ALIASES` mapping to
`scripts/ingest_ability.py` itself so a future re-run writes to the correct directory the first
time. Re-derived: `ability` `no_record` back to **576**, matching the run's own `name_pi_skipped`
count exactly. Lesson applied from this bundle's own standing caution: re-check a figure after
every rebase, not just after the first run — the alias landed on this branch WHILE this cycle was
mid-flight, and only a post-rebase re-derive caught the interaction.

## An unrelated, pre-existing defect observed (not this cycle's, not fixed)

A post-rebase full-corpus `corpus_literal_sweep` run fatals immediately on
`data/corpus/advanced_class_guide/domain/battle_spirit.json` (`source.path` missing its leading
`pathfinder/` segment, landed by the sibling `card15-simple-filename-kinds-ingest` cycle,
`71a6f3746`, before this cycle's rebase). This blocks a full-corpus sweep from completing post-
rebase and is unrelated to this cycle's own diff (a different kind, a different generator, a
pre-existing commit). This cycle's own 4,248 `ability` records were confirmed CLEAN by a full sweep
BEFORE the rebase (0 findings, reproduced above); the rebase changed no bytes in any `ability` file
(only the file-move above, path-only). Named here rather than silently worked around or fixed
out-of-scope (AGENTS.md rule 3).

## Next-cycle plan

1. **The 576 name-level PI stops** need an operator ruling before they can ever be transcribed (a
   name cannot be redacted) — or a per-book override the way `ogl-pi-blacklist.md §3` already
   provides for other kinds. Named, not silently deferred.
2. **`wiring_class` is a narrower heuristic than `WiringClassIndex`'s full `.MOD`/`.COPY=` closure**
   for this population — every written record is `static` or `display` only; a future cycle that
   builds a real Gate-2 engine for `ability` should re-derive `wiring_class` through the shared
   `WiringClassIndex` mechanism instead, the way `class_feature.rs` does.
3. **No unit tests exist yet for `scripts/ingest_ability.py`** — a real gap, named above.
4. Gate 2 (engine) has not been attempted for `ability` — 4,248 units are now measurable (Gate 1)
   but not reachable (Gate 2); a follow-up cycle is needed before any reach claim is made.

## Disk

`df -h /`: (pasted after this cycle's writes, below).
