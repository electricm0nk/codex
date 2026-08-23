# Cycle t9-t2-race-monster-class-racetrait-no-record-closure — gate-3-closure-invariant / `no_record` tail closure

- **Card ID:** card 11 (`epic-2-cause-closure`)
- **Commit SHA:** (this cycle's own commit, see push log)
- **Files touched:**
  - `scripts/ingest_generic_kind.py` (new) — kind-parameterized generic verbatim `no_record` ingest
  - `scripts/tests/test_ingest_generic_kind.py` (new) — 13 tests
  - `data/corpus/{race,monster,class,race_trait}_generic/*.json` under 41 book directories (108 new records)
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-3-closure-invariant/t9-t2-{race,monster,class,race-trait-tail}-generic-ingest-report.json` (new, run reports)
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (card 11 row, prepended note)
- **Identifier audit result:** self-healable, both pre-existing: (1) `scripts/ingest_generic_kind.py`
  imports the pre-existing `sd32_t9_pi_review_feat_equipment` module by its real filename — the
  identical import both `ingest_race_trait_generic.py` and `ingest_ability.py` already carry, a
  genuine cross-module reference to code that predates this cycle, not a new leaked tag; (2) the
  `kanban.md` diff shows the whole single-line row 11 as changed (one 165KB physical line), which
  makes git attribute PRE-EXISTING content further down the same line (an unrelated sibling note's
  `sd27_feat_prerequisite_enforcement.rs` filename reference) to the "added" side — content
  unchanged, a line-diff artifact, not new text. `scripts/*.py` restricted to my own new lines:
  `OK_NO_BUNDLE_TAGS`.
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** `decisions.md §20` — "Gate 3's closure condition is `no_record == 0`."
  This cycle's scope: the tail kinds with no active sibling lane — `race` (59), `monster` (28),
  `class` (21), `race_trait`'s residual (6). `feat` (680) is investigated but not landed
  (see "What is NOT closed" below).
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`)
- **Status:** complete (for this cycle's own scope: race/monster/class/race_trait)
- **Discovery forwards:** the `feat` mythic_adventures dual-record shape below, filed in this
  receipt rather than a separate `## DISCOVERED` entry (single-cycle scope, not queued work).
- **Next-cycle plan:** see "What is NOT closed" below.

## What this closes and why (the lever, per `decisions.md §17`)

Search for an existing path first: `scripts/ingest_race_trait_generic.py` (landed this same wave,
commit `75ea0c910`) is the exact generic-verbatim-transcription shape `race`/`monster`/`class` also
need — a book-agnostic, kind-filtered pass into a `<kind>_generic/` sibling directory, never touching
the curated `<kind>/` directory the kind's own Rust/Python ingesters own. Rather than fork three more
near-identical scripts, `scripts/ingest_generic_kind.py` parameterizes the SAME logic by `--kind`,
reusing `ingest_race_trait_generic.py`'s functions near-verbatim (`row_tokens`, `declared_pi`,
`slugify`, `resolve_file`, PI screening via `sd32_t9_pi_review_feat_equipment`).

**`decisions.md §24` (PI-name-blocked units ingest under a Codex-generated neutral name) applies
directly and was reused, not reinvented.** A dry-run for `class` found ALL 21 units are
`NAMEISPI:YES` (every Pathfinder prestige-class name in `adventurers_guide`/`inner_sea_*` is
Product Identity); `monster` found 19/28 name-PI-blocked (demon lords, empyreal lords, Great Old
Ones, kaiju); `race` found 1/59. Rather than re-derive a second scheme, `ingest_generic_kind.py`
imports `scripts/codex_neutral_name.py`'s `neutral_name`/`neutral_key`/`divergence_entry` directly —
the exact `ability` reference implementation (commit `e9d02c840`, this same wave) — and its own
`scrub_name_pi_tokens` (below, one real defect found and fixed).

## Re-derivation of the brief's own figures (`§17a`)

The brief's per-kind table (re-derived at cycle start, `python3 scripts/shape_ledger.py --inventory
docs/work-inventory.json`, count `join_status == "no_record"`) matched exactly: `race` 59, `monster`
28, `class` 21, `race_trait` 6 (not the brief's "6" being stale — it matched). Total `no_record`
3,440 at cycle start, confirmed against the brief.

## RED → GREEN (before/after, re-derivable)

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger.json
python3 -c "
import json,collections
r=json.load(open('/tmp/ledger.json'))['rows']
print(collections.Counter(x.get('kind','?') for x in r if x['join_status']=='no_record').most_common())"
```

| Kind | Before | After | Delta |
|---|---:|---:|---:|
| `race` | 59 | **0** | -59 |
| `monster` | 28 | **0** | -28 |
| `class` | 21 | **0** | -21 |
| `race_trait` | 6 | **0** | -6 |
| Total `no_record` (this cycle's scope) | 114 | **0** | -114 |
| Bundle-wide `no_record` | 3,440 | 3,326 | -114 |

## Two real defects found and fixed on the way (mutation-proved RED→GREEN both times)

1. **`scrub_name_pi_tokens`'s space-preserving substring check missed identifier-form leaks.**
   `ingest_ability.py`'s original scrub (copied here first, verbatim) checks `needle in
   value.lower()` where `needle` keeps its original spacing. PCGen's own `DEFINE`/`BONUS` tokens
   frequently concatenate a class/race name into a variable identifier with NO separator
   (`RedMantisAssassinLVL`, `WestcrownDevilLVL`) — a needle of `"red mantis assassin"` never matches
   `"redmantisassassinlvl"` because the value has no spaces. Found live: a full-corpus cross-check of
   all 41 renamed records against `docs/work-inventory.json`'s own `name`/`corpus_key` fields (not
   just eyeballing one sample) found **64 leaks across 15 of the 41 renamed `class`/`monster` records**
   before the fix — every one a PI class/creature name embedded in an unredacted `DEFINE`/`BONUS`
   value. Fixed by adding an alphanumeric-normalized substring check (bounded to needles ≥6
   normalized characters, to avoid over-redacting short/generic strings) alongside the original
   space-preserving check. Re-ran the full cross-check after the fix: **0 leaks across all 41 renamed
   records, confirmed by two independent methods** (a hand-curated needle list from the pre-fix
   dry-run output, and a second pass deriving needles fresh from `work-inventory.json`).
   Mutation-proved: `scripts/tests/test_ingest_generic_kind.py::
   test_pascalcase_compound_variable_identifier_embedding_the_name_is_redacted` fails RED against the
   original `ingest_ability.py`-copied logic (`AssertionError: False is not true`), passes GREEN
   after the fix.
2. **Slug-collision detection only checked the current run, not the filesystem.** `used_by_book`
   started empty every invocation, so a SECOND, separate script run (this cycle's own `race_trait`
   pass, after the `race`/`monster`/`class` passes had already run) could silently overwrite a
   pre-existing file if a different unit happened to slugify to the same string. Found live: a
   `class_feature` row (`Warpriest ~ Favored Class Blessings`, line 471, ALREADY committed by the
   prior `75ea0c910` cycle under `race_trait_generic/`) and a genuinely-distinct `race_trait` `.MOD`
   row sharing the identical `corpus_key` text at line 43 — both slugify to
   `warpriest_favored_class_blessings`. My first real (non-dry-run) `race_trait` invocation
   overwrote the pre-existing committed file; caught via `git status --porcelain` showing ` M`
   (modified, not `??` added) on a file this cycle never should have touched pre-existing content
   on. Reverted (`git checkout --`), fixed by seeding each book's slug-`used` set from the target
   directory's existing filenames before assigning new slugs, re-ran: the colliding unit now lands
   at `warpriest_favored_class_blessings_2.json`, the pre-existing file untouched.
   Mutation-proved: `ExistingOnDiskSlugIsNeverOverwrittenTests::
   test_a_second_run_writing_a_colliding_slug_gets_a_suffix_not_an_overwrite` fails RED with the seed
   removed (asserts the pre-existing file's content changed), passes GREEN restored.

## Product Identity (`decisions.md §15/§19/§24`)

41 of the 114 units (36%) are name-PI-blocked and ingest under a `§24` Codex-generated neutral name:
`class` 21/21, `monster` 19/28, `race` 1/59, `race_trait` 5/6 (the pre-existing 5 from `75ea0c910`,
untouched). Zero units were skipped/dropped — `§24` means every name-PI unit still ships, under a
name derived only from `(kind, book, source_file, source_line)`.

## Tests

- `python3 -m unittest scripts.tests.test_ingest_generic_kind` — 13/13 pass.
- **RED → GREEN proved twice by mutation** (both defects above).
- Determinism proved: regenerated all 108 files twice (`--dry-run` diff, then real write, then
  delete+re-write), diffed byte-for-byte excluding `ingested_at` — 0 mismatches across 108 files.
- `cargo run --locked --bin corpus_literal_sweep`: 968 findings, **0 in any of my four kinds'
  directories** (`grep race_generic|monster_generic|class_generic|race_trait_generic` on the tool's
  own output — 0 hits). All 968 are pre-existing, in the sibling `ability` lane's
  `advanced_players_guide/ability/codex_named_unit_*` files (commit `e9d02c840`, same wave, not
  mine) — `[redacted PI]` tokens are by design not byte-present in the raw corpus; the tool's own
  exit code (0) treats this as non-fatal.

## Pinned-count sweep

```bash
grep -rn "\b59\b.*race\|\b28\b.*monster\|\b21\b.*class" tests/ src/ scripts/ apps/ 2>/dev/null | grep -v /target/
```
No pinned assertion anywhere depends on the pre-cycle `race`/`monster`/`class`/`race_trait`
`no_record` counts. `scripts/shape_coverage_standing_gate.py`'s Gate 3 budget constants
(`no_record 21521 / population 36028`, repin 4) are a ratchet against an OLDER, much larger `not-done`
population definition than `shape_ledger.py`'s `no_record` figure quoted throughout this receipt —
confirmed by re-reading its own doc comment before touching anything nearby; **not touched**, per the
brief's explicit instruction. This cycle only ever REDUCES `shape_ledger.py`'s `no_record`, never
raises it, so the ratchet cannot be tightened in the wrong direction by this work.

## What is NOT closed: `feat`'s 680 (sibling scope, investigated per brief, not landed)

Re-derived by book (all 680, matches the brief exactly):

```bash
python3 -c "
import json,collections
r=json.load(open('/tmp/ledger.json'))['rows']
c=collections.Counter(x['book'] for x in r if x['join_status']=='no_record' and x.get('kind')=='feat')
for k,v in c.most_common(30): print(v,k)"
```

`mythic_adventures` 353, `adventurers_guide` 81, `ultimate_combat` 51, `ultimate_magic` 48,
`advanced_players_guide` 37, `ultimate_wilderness` 35, `inner_sea_races` 22, `horror_adventures` 17,
`ultimate_intrigue` 16, `inner_sea_magic` 7, `inner_sea_world_guide` 6, `bestiary` 4,
`advanced_race_guide`/`inner_sea_combat`/`inner_sea_faiths` 1 each.

**`§17a` correction of the brief's own claim.** The brief states "`mythic_adventures`'s 353 ... are
misclassified `.MOD`/`VISIBLE:EXPORT` non-feat noise" — re-derived by direct read, this is **half
right, not uniform**:

- **208 of the 353 ARE noise**: `.MOD` rows under `CATEGORY=Special Ability|<Race> ~ <Trait>.MOD`
  with `TYPE:<Race> MA Racial Trait` (`ma_feats.lst`'s own `###Block: Enable Racial Traits that
  qualify` section, lines ~430-673) — these flag existing race_trait records as Mythic-qualified,
  they never declare a feat. Already correctly excluded by `gen_feat_gap_tables.rs`'s existing
  `RuleSetId::Mythic` `BookInput` (its own doc comment names this exact population and count).
  Confirmed: `origin: "mod_only"` on all 208 in `docs/work-inventory.json`.
- **The other 145 are real `CATEGORY:FEAT` records**, e.g. line 244:
  `Accursed Hex (Mythic)  KEY:Mythic Feat Output ~ Accursed Hex  CATEGORY:FEAT  TYPE:Mythic
  VISIBLE:EXPORT  DESC:Your hexes flare with persistent potency....` — a real name, real
  description, real `BENEFIT` text. `origin: "declared"` on all 145.

**These 145 are NOT simply un-ingested — they are a second, distinct record sharing a display name
with a record the compiled table ALREADY holds.** `ma_feats.lst` line 41 separately declares
`Accursed Hex (Mythic)  KEY:Accursed Hex  CATEGORY:Mythic Feat  ...
ABILITY:FEAT|AUTOMATIC|Mythic Feat Output ~ Accursed Hex` — the actual Mythic Path feat, which
AUTO-grants the `CATEGORY:FEAT` "output" record at line 244 as a companion ability.
`src/rules_core/rules_tables/feat_gap_tables.rs::MYTHIC_ADVENTURES_FEAT_GAP_ROWS` already holds all
145 of the LINE-41-style records (confirmed: `grep -c "(Mythic)" feat_gap_tables.rs` → 145; ran
`cargo run --locked --bin gen_cache_feat_gap`, got `FATAL: zero records written` with all 145
`mythic_adventures:<Name>` slugs in the "already claims that slug" skip list — i.e. that half is
fully dumped already). The 145 LINE-244-style "Mythic Feat Output ~ <Name>" companion records are a
DIFFERENT citation the generator never parses. `gen_feat_gap_tables.rs`'s `RuleSetId::Mythic` needs
a second citation pass (parse `KEY:Mythic Feat Output ~ ` rows as a second feat set for the same
book) to close this 145 — real, well-scoped work, not attempted this cycle (sibling-owned kind;
this cycle's budget went to the four closed kinds instead).

The remaining residual (680 − 353 = 327, across `adventurers_guide`/`ultimate_combat`/
`ultimate_magic`/`advanced_players_guide`/`ultimate_wilderness`/`inner_sea_races`/
`horror_adventures`/`ultimate_intrigue`/`inner_sea_magic`/`inner_sea_world_guide`/`bestiary`/
`advanced_race_guide`/`inner_sea_combat`/`inner_sea_faiths`) is **not re-derived this cycle** — named
by book and count above, unexamined by shape, left for the next `feat`-scoped cycle.

## Next-cycle plan

1. **`feat`'s mythic_adventures 145**: extend `gen_feat_gap_tables.rs`'s `RuleSetId::Mythic`
   `BookInput` (or add a sibling one) to parse the `KEY:Mythic Feat Output ~ <Name>` `CATEGORY:FEAT`
   rows as a second citation set for the same book, regenerate `feat_gap_tables.rs`, dump via
   `cache_gen::feat_gap`. Closes 145/680 (21%) of `feat`'s residual by itself.
2. **`feat`'s remaining 327** (14 books): sample-verify shape by direct read before building
   anything, per the same `§17a` discipline this cycle applied to `mythic_adventures` — do not
   assume uniform noise OR uniform real content from the book name alone.
3. `adventurers_guide` 81 (already flagged by the prior `44713d770` cycle): needs a new
   `BookInput`/`BookSpec` (no `RuleSetId::Ag` entry exists in `hand_authored_feat_tables()` or
   `gen_feat_gap_tables.rs`'s `BOOK_INPUTS`) — genuinely new content, real per-book work.
