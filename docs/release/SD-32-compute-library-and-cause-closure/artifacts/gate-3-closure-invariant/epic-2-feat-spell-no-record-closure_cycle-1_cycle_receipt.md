# Cycle 1 — gate-3-closure-invariant / feat+spell `no_record` closure (`decisions.md §20`)

- **Card ID:** card 11 (`epic-2-cause-closure`)
- **Commit SHA:** (this cycle's own commit, see push log)
- **Files touched:**
  - `src/rules_core/cache_gen/feat_gap.rs` (new) — feat "gap" JSON cache generator
  - `src/bin/gen_cache_feat_gap.rs` (new) — its entry point
  - `src/rules_core/cache_gen/mod.rs` — registers `feat_gap`
  - `src/rules_core/cache_gen/spell_lane_dump.rs` — widened `book_specs()` 6 → 11 books
  - `data/corpus/**/feat/*.json` (649 new files, 19 books)
  - `data/corpus/{adventurers_guide,inner_sea_faiths,inner_sea_magic,inner_sea_temples,horror_adventures}/spell/*.json` (174 new files, 5 books)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff --unified=0 1bb523773d32705d1b7387fd4c494861523f55ba...HEAD -- <files above>`)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff scope)
- **Acceptance criterion:** `decisions.md §20` — "Gate 3's closure condition is `no_record == 0`, not
  'budget not exceeded'." This cycle's scope: drive `feat` (1,202) and `spell` (860) `no_record`
  toward zero.

## What this closes and why (the lever, per `decisions.md §17`)

Both `feat` and `spell` already had a config-driven mechanism that writes a **compiled Rust table**
consumed by the engine/UI (`feat_gap_tables.rs` via `gen_feat_gap_tables.rs`; per-book
`spell_list::SPELL_LIST` tables via `ingest_spells.rs`) — but neither mechanism had ever written the
corresponding `data/corpus/<book>/<kind>/*.json` cache `scripts/shape_ledger.py` joins against. The
compiled table exists, the engine already serves the record, and `scripts/shape_ledger.py` still
reported `no_record` because its join key is a real on-disk citation, not the compiled table. This
is the **identical shape** `cache_gen::equipment_gap` (SD-31 `SD31-E6-F5-002`) already closed for
`equipment`/`equipment_modifier` — this cycle is that fix's `feat` sibling, plus a `spell` widening
of the analogous, already-shipped `cache_gen::spell_lane_dump` module.

**`feat`:** new `cache_gen::feat_gap` module, mirroring `cache_gen::equipment_gap`'s citation
resolution / PI screening / no-clobber-write discipline. Dumps `feat_gap_tables::feat_gap_rows_for()`
(19 books, 649 rows, already engine-registered) to `data/corpus/<book>/feat/*.json`, resolving each
row's real citation against the exact `.lst` file(s) `gen_feat_gap_tables.rs`'s own `BOOK_INPUTS`
already names (mirrored here as `BOOK_SPECS`, with a test (`book_specs_matches_gen_feat_gap_tables_
book_count`) proving the two tables track 1:1).

**`spell`:** widened `cache_gen::spell_lane_dump::book_specs()` from 6 books to 11 — added
`adventurers_guide`, `inner_sea_faiths`, `inner_sea_magic`, `inner_sea_temples`, `horror_adventures`,
each of which already had a compiled `spell_list::SPELL_LIST` table (via `ingest_spells.rs`'s
config-driven `BOOKS`) but zero corpus JSON.

## RED → GREEN (before/after, re-derivable)

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/l.json
python3 -c "
import json,collections
r=json.load(open('/tmp/l.json'))['rows']
nr=[x for x in r if x['join_status']=='no_record']
print(collections.Counter(x.get('kind','?') for x in nr).most_common())"
```

| Kind | Before (RED, `d269963882390bbe776b54b97c9233fda9260148`) | After (GREEN, this cycle) | Delta |
|---|---:|---:|---:|
| `feat` | 1,202 | 901 | **-301** |
| `spell` | 860 | 686 | **-174** |
| Total `no_record` (all 18 kinds) | 20,889 | 20,414 | **-475** |

Corpus SHA: `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`).

`matched` count is unchanged (4,802 → 4,802): neither generator writes `data.raw_tokens` (that is a
separate, pre-existing enrichment pass — `enrich_spell_raw_tokens.rs`'s own doc comment names this
split explicitly). Every closed unit moved `no_record` → `no_formula_tokens` (record now exists;
these `feat`/`spell` gap-table rows carry description/prerequisite/school/level data, not
`DEFINE:`/`BONUS:` formula tokens) — a legitimate, honest terminal state per `decisions.md §20`'s own
three-way split, not a fabricated `matched`.

## Tests

- `cargo test --locked --lib rules_core::cache_gen::feat_gap` — 10/10 pass, including a live
  generation test against the pinned oracle (`generation_against_the_real_pinned_corpus_writes_
  records`) and a citation-resolution regression test for the double-path-join bug this cycle caught
  and fixed before landing (`find_citation` was joining `book_dir` with an already-corpus-root-
  relative path; corrected to search from `corpus_root` directly — the fix is why the first live run
  wrote 0 records and the corrected run wrote all 649).
- `cargo test --locked --lib rules_core::cache_gen::spell_lane_dump` — 9/9 pass, including the
  existing `generation_against_the_real_pinned_corpus_resolves_every_citation` (now covering 11
  books, zero unresolved) and the anti-self-erasure regression test.
- `cargo test --locked --lib rules_core::cache_gen::` (all sibling `cache_gen` modules, to check for
  collateral damage from the `mod.rs` edit) — 117/117 pass, 0 failed.

## What is NOT closed, by exact shape (per `decisions.md §16`/`§17a` — never round into "done")

**`feat` residual: 901.** Not all of this is even feat-gap-table territory:
- `mythic_adventures`: 448 → 353 (95 closed). The residual 353 is real: `gen_feat_gap_tables.rs`'s
  own doc comment records that `ma_feats.lst` carries 208 `.MOD` rows targeting `race_trait`-kind
  base records elsewhere in the corpus — genuinely not feat content, matching this bundle's own
  `decisions.md §16` T2b finding shape. **Not independently re-verified this cycle** — flagged, not
  claimed closed or claimed noise; the next cycle on this population should re-derive the exact
  split by hand rather than trust either this receipt's inference or the earlier brief's "353 is
  noise" claim, per `§17a`.
- Several books (`core_rulebook` 67, `adventurers_guide` 81, `ultimate_psionics` 92,
  `ultimate_campaign` 23, `bestiary` 4, `inner_sea_world_guide` 6) show little or no movement because
  `feat_gap_tables.rs` carries few or zero rows for them (`core_rulebook` has exactly 1) — meaning
  their `no_record` feat population is `declared`-origin content that was **never captured by any
  existing table at all**, hand-authored or gap. That is genuinely new-content ingestion, a different
  shape from this cycle's "compiled-but-uncached" lever, and is unclosed.
- A handful of `mythic_adventures` rows resolved to a citation LINE that does not match the exact
  `source_line` `docs/work-inventory.json`'s own walker recorded for the same key (duplicate first-
  column names across the file), so some gap-table rows joined to a *different* corpus record than
  the inventory unit expected. Not separately quantified this cycle — worth a follow-up diff between
  `feat_gap_tables.rs`'s row order and `v06_work_inventory.rs`'s own citation resolution for
  `mythic_adventures` specifically.

**`spell` residual: 686.** Two distinct shapes, by `origin` field in `docs/work-inventory.json`:
- **363 `mod_only`** (`occult_adventures` 328, `ultimate_magic` 19, `advanced_players_guide` 15,
  `book_of_the_damned_volume_1` 1) — `.MOD` rows widening an EXISTING spell's `CLASSES:` access for
  a new class (e.g. `Occultist Spell ~ Accelerate Poison`), not a new spell declaration.
  `magnitude_token_count: 0`, `wiring_class: display` on every sampled row. These are real, citable
  corpus rows with no formula content of their own — closing them needs a NEW mechanism (a MOD-row
  cache dump citing the MOD row's own line, distinct from `spell_lane_dump`'s base-declaration-only
  scope), not more book config. Not attempted this cycle; named here rather than left silently
  uncounted.
- **~322 `declared`** (`bestiary` 108, `bestiary_4` 56, `inner_sea_races` 29, `inner_sea_intrigue` 26,
  `monster_codex` 24, `inner_sea_world_guide` 22, `book_of_the_damned_volume_2` 12,
  `mythic_adventures` 9, `book_of_the_damned_volume_1` 5, `bestiary_6` 2, and small remainders in
  `occult_adventures`/`ultimate_magic`/`ultimate_combat`/`ultimate_equipment`/`inner_sea_gods`) —
  genuine new spell content never captured by any compiled `SPELL_LIST` table for that book at all.
  `bestiary`/`bestiary_4` were reported by a prior cycle as "monster-intrinsic with no dedicated
  `.lst`" — **not re-verified this cycle**; flagged for re-derivation, not assumed still true.

## Corollary discovery (logged separately, `scripts/retro.py incident`)

This wave's dispatch env block gives every sibling lane the SAME literal
`CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-t9-onboarding`. At least 4 concurrent
worktrees (`wf_875c4562-64a-1/-2/-5/-6`, observed via `ps aux`) were building against that one
directory simultaneously, corrupting cargo's fingerprint cache and producing a spurious
"unresolved import" error for a module that had compiled cleanly moments before. Worked around by
using a worktree-suffixed `CARGO_TARGET_DIR` instead of the literal path. `AGENTS.md`'s own rule
("`CARGO_TARGET_DIR` is one directory per agent *per source tree*, never per agent") already covers
this; the dispatch template's literal value violates it. Logged:
`docs/retro/events/t9-onboarding.jsonl` (recurrence-key `shared-target-dir`).

## Status: complete (for this cycle's own scope)

Real `no_record` reduction, verified against the pinned oracle, tests RED→GREEN, dual-audit clean.
Card 11 / the `feat`+`spell` `no_record` populations are **not** at zero — the residuals above are
named by exact shape and count, not rounded into "done", per `decisions.md §16`/`§17a`.

## Next-cycle plan

1. Build the analogous `.MOD`-row cache dump for spell class-access-widening rows (363 units,
   `occult_adventures` the majority) — same "generic pass" lever, new shape.
2. Extend `gen_feat_gap_tables.rs`'s own `already_held` scan and `feat_gap_tables.rs` row generation
   for books whose `no_record` feat population was never captured by ANY table (`core_rulebook`,
   `adventurers_guide`, `ultimate_psionics`, `ultimate_campaign` chief among them) — this is new-
   content ingestion, not a corpus-cache gap, and needs `gen_feat_gap_tables` re-run with a widened
   predicate, not just this cycle's dump-to-JSON step.
3. Re-verify (not assume) the `mythic_adventures` 353 residual's noise/real-content split and the
   `bestiary`/`bestiary_4` "monster-intrinsic, no dedicated .lst" claim for `spell`, per `§17a`.
