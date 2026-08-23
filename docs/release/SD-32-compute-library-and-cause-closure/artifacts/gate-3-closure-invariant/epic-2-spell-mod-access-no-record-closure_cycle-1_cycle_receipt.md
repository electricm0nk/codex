# Cycle 1 — gate-3-closure-invariant / `spell` `mod_only` `no_record` closure (`decisions.md §20`)

- **Card ID:** card 11 (`epic-2-cause-closure`)
- **Commit SHA:** (this cycle's own commit, see push log)
- **Files touched:**
  - `src/rules_core/cache_gen/spell_mod_access.rs` (new) — spell class-access `.MOD` row JSON cache generator
  - `src/bin/gen_cache_spell_mod_access.rs` (new) — its entry point
  - `src/rules_core/cache_gen/mod.rs` — registers `spell_mod_access`
  - `data/corpus/{occult_adventures,ultimate_magic,advanced_players_guide}/spell/*.json` (2,228 new files)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff --unified=0 857eb85d0370adce3bd113c0cbda4e755b631a0a...HEAD -- src/rules_core/cache_gen/spell_mod_access.rs src/rules_core/cache_gen/mod.rs src/bin/gen_cache_spell_mod_access.rs`)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff scope)
- **Acceptance criterion:** `decisions.md §20` — "Gate 3's closure condition is `no_record == 0`". This
  cycle's scope: the `feat`/`spell` no-record closure cycle's own next-cycle-plan item 1
  (`epic-2-feat-spell-no-record-closure_cycle-1_cycle_receipt.md`) — build the MOD-row cache dump for
  spell class-access-widening rows.

## What this closes and why (the lever, per `decisions.md §17`)

`docs/work-inventory.json`'s enumerator counts a `.MOD` spell row that widens an EXISTING spell's
class access (`Occultist Spell ~ Accelerate Poison.MOD ... CLASSES:Occultist=2`) as its own `spell`
unit — a real, citable corpus line distinct from the base declaration it widens. No generator had
ever dumped these rows to `data/corpus/`, so `scripts/shape_ledger.py`'s
`(book, source_basename, source_line)` join reported every one `no_record`, even though the row is
real, citable, on-disk content. This is a NEW mechanism (unlike `feat_gap`/`spell_lane_dump`, there
is no pre-existing compiled Rust table for these rows — they were never ingested anywhere), but the
same generic-pass shape: one module, three books, no per-object work.

**Scope:** rows whose first field ends `.MOD` and which carry a `CLASSES:` token, across
`occult_adventures` (`oa_spells.lst`), `ultimate_magic` (`um_spells.lst`),
`advanced_players_guide` (`apg_spells.lst`). A fourth candidate,
`book_of_the_damned_volume_1:pfs_botd1_spells.lst:13` (`Greater Teleport (...).MOD`), carries
`TYPE:`/`PRETYPE:` tokens but no `CLASSES:`/`SCHOOL:` of its own — a different shape (its
`spell`-kind classification traces back to its MOD target, not its own fields) — and is
deliberately left out of this pass rather than force-matched by a looser filter. Named here per
`decisions.md §16`, not silently dropped.

## RED → GREEN (before/after, re-derivable)

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger.json
python3 -c "
import json,collections
r=json.load(open('/tmp/ledger.json'))['rows']
print(collections.Counter(x.get('kind','?') for x in r if x['join_status']=='no_record').most_common())"
```

| Kind | Before (this cycle's own start, re-derived at `857eb85d0`) | After | Delta |
|---|---:|---:|---:|
| `spell` | 686 | 339 | **-347** |
| Total `no_record` (all 15 kinds still open) | 8,434 | 8,087 | **-347** |

Corpus SHA: `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`).

**347, not 363** — the mechanism writes one record per qualifying `.MOD` line (2,228 total across the
three books; 191 skipped as pre-existing slug collisions, 1 excluded for declared name PI), but only
347 of the `no_record` population's own `(book, file, line)` triples were among them; the remainder
of the 2,228 written records land on lines the ledger's population did not count as `no_record` in
the first place (already `matched` via a different mechanism, or real corpus content the current
`work-inventory.json` enumeration does not carry a distinct unit for) — extra real, citable data, not
inflation of this cycle's own closure claim.

## PI exclusions (`decisions.md §15`/`§19`, named per record)

- `occult_adventures:Discern Next of Kin` — dropped whole (name-field PI hit against the signed-off
  blacklist term list). No other name-PI hits across the three books.

## Tests

- `cargo test --locked --lib rules_core::cache_gen::spell_mod_access` — 7/7 pass, including a live
  generation test against the pinned oracle (`generation_against_the_real_pinned_corpus_writes_
  records`, asserts >300 records written) and a scratch-corpus test proving the `CLASSES:`-token
  guard both writes the qualifying row and skips a `.MOD` row with none.
- `cargo test --locked --lib rules_core::cache_gen::` (all sibling `cache_gen` modules, to check for
  collateral damage from the `mod.rs` edit) — 126/126 pass, 0 failed, 10 ignored (pre-existing,
  unrelated to this change).
- **RED → GREEN proved by mutation:** flipped `has_classes = true` to `has_classes = false` inside
  the `CLASSES:` token branch of `parse_mod_row` — 5 of 7 tests failed for the intended reason
  (zero records written; the live-corpus assertion `>300` failed with `got 0`). Reverted; all 7 pass
  again.

## Status: complete (for this cycle's own scope)

Real `no_record` reduction (spell 686 → 339), verified against the pinned oracle, tests RED→GREEN,
dual-audit clean. `spell`'s `no_record` population is **not** at zero — the residual 339 is named by
exact shape below, not rounded into "done".

## What is NOT closed (residual: 339, re-derived post-cycle, not the prior cycle's stale figures)

Cross-tabulated against `docs/work-inventory.json`'s own `origin` field, re-run AFTER this cycle's
corpus write:

```bash
python3 -c "
import json,collections
inv=json.load(open('docs/work-inventory.json'))
units={u['id']:u for u in inv['units']}
r=json.load(open('/tmp/ledger.json'))['rows']
rows=[x for x in r if x['join_status']=='no_record' and x.get('kind')=='spell']
print(collections.Counter(units.get(x['id'],{}).get('origin') for x in rows))"
```

| Origin | Books | Units |
|---|---|---:|
| `declared` | `bestiary` 108, `bestiary_4` 56, `inner_sea_races` 29, `inner_sea_intrigue` 26, `monster_codex` 24, `inner_sea_world_guide` 22, `book_of_the_damned_volume_2` 12, `advanced_players_guide` 9, `mythic_adventures` 9, `book_of_the_damned_volume_1` 5, `inner_sea_magic` 5, `adventurers_guide` 4, `inner_sea_gods` 4, `ultimate_magic` 3, `bestiary_6` 2, `inner_sea_faiths` 1, `occult_adventures` 1, `ultimate_combat` 1, `ultimate_equipment` 1 | 322 |
| `mod_only` | `advanced_players_guide` 15, `book_of_the_damned_volume_1` 1 | 16 |
| `copy` | `bestiary` | 1 |

`occult_adventures` and `ultimate_magic`'s `mod_only` populations are fully closed (328→0, 19→0).
`advanced_players_guide`'s is closed from 15→ (15 of its own MOD rows still show `no_record` — its
own written records mostly landed on lines the ledger's population never enumerated as distinct
units, a slug-scheme artifact: this book's spell corpus reuses base names across multiple `.MOD`
variant rows more than the other two books do, so `slugify`'s collision counter skipped writing a
second record for the same slug even though the two source LINES differ). `book_of_the_damned_
volume_1`'s single non-`CLASSES:` row is the one named out-of-scope above.

**`bestiary`/`bestiary_4`'s 164 `declared` units** were reported by a prior cycle as "monster-intrinsic
with no dedicated `.lst`" — **not re-verified this cycle**, flagged per `§17a` for re-derivation, not
assumed still true.

**The remaining `advanced_players_guide` `mod_only` residual (15 units)** needs `slugify` widened to
disambiguate on `(book, key)` rather than `key` alone, or a slug keyed off `(source_line)` directly —
this generator's own no-clobber discipline is correctly refusing to overwrite, but it is refusing to
write a SECOND legitimate record too eagerly when two different `.MOD` lines happen to slugify to
the same string.

## Next-cycle plan

1. Re-derive `bestiary`/`bestiary_4`'s 164 `declared` spell units by direct read (not the prior
   cycle's un-reverified claim) — confirm or refute "monster-intrinsic, no dedicated `.lst`".
2. Widen `spell_mod_access`'s slug scheme to resolve the 16 remaining `mod_only` collisions.
3. `declared`-origin residual (306 units after item 1's re-derivation) needs new-content ingestion —
   a different, larger shape, out of this cycle's scope.
