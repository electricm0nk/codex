# Cycle 2 — gate-3-closure-invariant / `feat` `no_record` closure (`decisions.md §20`)

- **Card ID:** card 11 (`epic-2-cause-closure`)
- **Commit SHA:** (this cycle's own commit, see push log)
- **Files touched:**
  - `src/rules_core/cache_gen/hand_authored_feat_dump.rs` (new) — hand-authored feat table JSON cache generator
  - `src/bin/gen_cache_hand_authored_feat_dump.rs` (new) — its entry point
  - `src/rules_core/cache_gen/mod.rs` — registers `hand_authored_feat_dump`
  - `data/corpus/{core_rulebook,ultimate_psionics,advanced_class_guide,ultimate_campaign}/feat/*.json` (558 new files)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion:** `decisions.md §20` — "Gate 3's closure condition is `no_record == 0`."
  This cycle's scope: the `feat`/`spell` no-record closure cycle's own next-cycle-plan item 2 —
  "Extend `gen_feat_gap_tables.rs`'s own `already_held` scan ... for books whose `no_record` feat
  population was never captured by ANY table."

## What this closes and why (the lever, per `decisions.md §17`)

Re-deriving that claim found it half right: `core_rulebook` (67 `no_record`), `ultimate_psionics`
(92), `advanced_class_guide` (39) and `ultimate_campaign` (23) are **not** un-captured new content —
every one of these units' `source_file` is the exact `.lst` file `feats_all::hand_authored_feat_
tables()`'s own compiled table for that `RuleSetId` was built from (`crb::feats`, `upsi_records()`,
`acg_records()`, `uca_records()` — 185/141/~120/~112 entries respectively). The compiled table
already exists and the engine already serves every one of these records
(`feats_all::all_feat_tables()` chains hand-authored tables ahead of `feat_gap_tables`'s residue) —
but `data/corpus/<book>/feat/` held **1** file for `core_rulebook` and near-zero for the other three
before this cycle, so `scripts/shape_ledger.py`'s join found nothing. This is the identical shape
`cache_gen::feat_gap` closed for the OTHER half of `feat`'s population (the gap-table residue);
`cache_gen::hand_authored_feat_dump` is that fix's sibling for the HAND-AUTHORED half, reusing
`cache_gen::feat_gap`'s own `find_citation`/`declared_pi_at`/`BookSpec` rather than reimplementing
citation resolution.

`adventurers_guide` (81) has **no** entry in `hand_authored_feat_tables()` at all (confirmed:
`RuleSetId::Ag` is absent from that function's book list) — genuinely new content, out of this
cycle's scope, named below rather than silently swept in.

## RED → GREEN (before/after, re-derivable)

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger.json
python3 -c "
import json,collections
r=json.load(open('/tmp/ledger.json'))['rows']
print(collections.Counter(x.get('kind','?') for x in r if x['join_status']=='no_record').most_common())"
```

| Kind | Before (this cycle's own start) | After | Delta |
|---|---:|---:|---:|
| `feat` | 901 | 680 | **-221** |
| Total `no_record` (14 kinds still open) | 8,087 | 7,866 | **-221** |

Corpus SHA: `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`).

558 records written (0 unresolved citations, 0 name-PI exclusions, 0 pre-existing-slug skips); 221
of them landed on lines the ledger's `no_record` population actually needed — the rest are real,
already-`matched` content the run also (harmlessly, honestly) dumped because `hand_authored_feat_
tables()` iterates whole-book, not filtered to the `no_record` subset.

## Tests

- `cargo test --locked --lib rules_core::cache_gen::hand_authored_feat_dump` — 2/2 pass, including a
  live generation test against the pinned oracle (`generation_against_the_real_pinned_corpus_writes_
  records`, asserts >150 records written).
- `cargo test --locked --lib rules_core::cache_gen::` (all sibling modules) — 128/128 pass, 0 failed,
  10 ignored (pre-existing, unrelated).
- **RED → GREEN proved by mutation:** replaced the `hand_authored_feat_tables().iter().find(|b| b.
  rule_set == spec.rule_set)` lookup with `.find(|_b| false)` (always `None`) — the live-corpus test
  failed for the intended reason (`expected >150 records, got 0`). Reverted; both tests pass again.

## Status: complete (for this cycle's own scope)

Real `no_record` reduction (feat 901 → 680), verified against the pinned oracle, tests RED→GREEN,
dual-audit clean. `feat`'s `no_record` population is **not** at zero — the residual is named by
exact shape below.

## What is NOT closed (residual: 680, re-derived post-cycle)

```bash
python3 -c "
import json,collections
r=json.load(open('/tmp/ledger.json'))['rows']
c=collections.Counter(x.get('book','?') for x in r if x['join_status']=='no_record' and x.get('kind')=='feat')
for k,v in c.most_common(30): print(v,k)"
```

- `mythic_adventures` 353 — flagged by the prior cycle as likely `.MOD`/`race_trait`-continuation
  noise (`gen_feat_gap_tables.rs`'s own doc comment: `ma_feats.lst` carries 208 `.MOD` rows targeting
  `race_trait` base records elsewhere). **Not re-verified this cycle either** — still an open
  re-derivation, per `§17a`, not assumed either way.
- `adventurers_guide` 81 — genuinely new content, no `hand_authored_feat_tables()` entry and no
  `feat_gap_tables.rs` `BOOK_INPUTS` entry. Needs a new ingest cycle (parse `ag_feats.lst`, add a
  `BookInput` to `gen_feat_gap_tables.rs`, regenerate `feat_gap_tables.rs`, then dump via
  `cache_gen::feat_gap`) — the mythic_adventures/inner_sea_taverns precedent shape, not attempted
  this cycle.
- `ultimate_combat` 51, `ultimate_magic` 48, `ultimate_wilderness` 35, `horror_adventures` 17,
  `inner_sea_races` 22, `ultimate_intrigue` 16, `inner_sea_magic` 7, `inner_sea_world_guide` 6,
  `bestiary` 4, `advanced_race_guide` 1, `inner_sea_combat` 1, `inner_sea_faiths` 1 — these ARE
  already in `feat_gap_tables.rs`'s `BOOK_INPUTS` (`gen_feat_gap_tables.rs`) and already dumped once
  by `cache_gen::feat_gap` (per the prior cycle's own receipt) — this residual was **already present
  before this cycle**, unaffected by either generator, and is real: either `.MOD`/`VISIBLE:EXPORT`
  noise `gen_feat_gap_tables.rs`'s parser correctly excludes, or a citation-resolution miss. Not
  re-derived this cycle; flagged for the next one rather than assumed closed or assumed noise.

## Next-cycle plan

1. Re-derive `mythic_adventures`'s 353 by direct read (the T9 feat-population re-derivation the
   brief cites found the REAL transcribable share of a similarly-shaped population was a small
   fraction of the nominal count — check whether the same holds here rather than assume it).
2. `adventurers_guide`'s 81 needs a genuine new ingest cycle (new `BookInput`/`BookSpec` pair,
   parse `ag_feats.lst`, regenerate both `feat_gap_tables.rs` and its corpus dump) — real per-object
   work, not a generic-pass widening.
3. Sample a handful of the `already-in-BOOK_INPUTS` residual books (`ultimate_combat`,
   `ultimate_magic`) by direct citation lookup to determine whether their gap is noise or a
   resolvable citation-search miss before dispatching a fix.
