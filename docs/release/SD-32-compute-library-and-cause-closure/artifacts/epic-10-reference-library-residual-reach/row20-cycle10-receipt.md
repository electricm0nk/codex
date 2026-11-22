# Cycle row20-cycle10 — Epic 10 (`epic-10-reference-library-residual-reach`) / Row 20

- **Card ID:** `epic-10-reference-library-residual-reach`
- **Files touched:**
  - `src/rules_core/pilot_compute/companion_base_stat_table.rs` (22 new struct-literal entries
    for the first 22 of the 142 untagged `core_rulebook` companion records, a new module-doc
    addendum, one updated positive-count test, one new 22-species positive test, 22 new entries
    in the `companion_display_name` coverage test).
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (this cycle's own row 20
    entry, appended).
  - This receipt.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own diff, `git diff --unified=0 HEAD` over
  the touched source file, `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — zero
  hits).
- **Wired-integration audit result:** `OK_NO_TOKENS` (`grep -nE '\b(STUB|MOCK|placeholder|not yet
  implemented|todo|fixme|hack)\b'` over the same own-diff scope — zero hits). No new dispatch
  point: the 22 new entries flow through the existing `ground_companion_stat_block` /
  `ground_selected_companion_or_default` dispatch cycle 7 already wired end to end.
- **PI scrub:** `pi_scrub.normalized_term_hits()` (imported, never copied) over the own-diff —
  zero hits, first pass clean.
- **Acceptance criterion:** Epic 10, row 20 — companion base-ability-score table residual: close
  or precisely size, per species, every `RACETYPE:Companion` corpus record with no verified base
  vector, per `decisions.md §27b` ("EVERYTHING", no carve-outs) and `§1a` (refuse rather than
  fabricate).
- **Status:** `in-progress` (population re-derived, unchanged shape: 196 total. 74 now grounded
  — up from 52 — leaving **122** real, sized, unbuilt: 120 untagged records + the 2 named
  dinosaur refusals).
- **Corpus SHA:** oracle bootstrapped fresh this worktree (slot was empty), pinned at
  `7f818006e371188e5717fd18d74d18a420747fc6` — the same commit every prior cycle used, confirmed
  via `scripts/fetch-pcgen-oracle.sh --dest $PCGEN_REPO_DIR`.
- **Discovery forwards:** none new this cycle.
- **Next-cycle plan:** grind the remaining 120 untagged records. `pachycephalosaurus`/
  `ornithomimosaur` still refuse pending a source neither cycle 7 nor any cycle since could reach.

## Starting state (verified, not assumed)

`git rev-parse HEAD` on entry equalled `1bb523773d` (the SD-31 PR #374 merge commit), the same
stale-lineage footgun most prior cycles in this row hit (footgun 3) — the fresh worktree's clean
checkout defaulted to a branch tip well behind `$PIN`. Recovered with `git reset --hard
origin/tranche/12`, which landed exactly on `$PIN`
(`ccd7e4992d1f6c585538159b5836fe407396592f` — row 18 cycle 13's own commit, the current tip of
`origin/tranche/12`, superseding cycle 9's `5b1d4e6ea5`), re-verified `git merge-base
--is-ancestor "$PIN" HEAD` (`BASE_OK`) and `git rev-parse HEAD` matching `$PIN` exactly — no
rebase needed at entry, no sibling collision.

Oracle slot was empty (fresh worktree, git-ignored). `scripts/fetch-pcgen-oracle.sh --dest
$PCGEN_REPO_DIR` populated it at `7f818006e371188e5717fd18d74d18a420747fc6`, confirmed via its own
stdout (`OK 7f818006e371...`).

## `§17a` re-derivation

Re-derived the population directly from `data/corpus/*/companion/*.json`, filtering
`data.monster_class` starting with `"Companion"` (note: the field lives under the record's
`data` key, not the top level) and excluding "Companion Advancement" records: **196 total, 144
untagged** — identical to cycles 8 and 9's own figures, confirming no movement in the population
itself before this cycle's own additions.

## Read cycle 9's primary finding before adding a single species

Per the dispatch brief's own instruction, read `row20-cycle9-receipt.md` in full before touching
the table. Its finding: `ground_companion_stat_block` applies only the companion CLASS's own
universal `floor(MasterLevel/3)` advance on top of the table's `strength`/`constitution` fields —
it never reads the corpus RACE record's own per-species `BONUS:STAT` delta. The table must
therefore hold the species' printed 1st-level "Starting Statistics" total DIRECTLY, never backed
out by the corpus delta. This cycle's own 22 new entries follow that corrected methodology from
the start (no delta subtraction anywhere in this cycle's diff).

## Work this cycle: 22 of the 142 untagged records, using cycle 9's already-gathered data

Cycle 9's own receipt ("Data gathered but not yet committed") already fetched and verified printed
AoN totals for 18 of the 142 untagged `core_rulebook` species (cross-checked against d20pfsrd for
Bear/Boar, an exact match), explicitly staged for a future cycle to consume without re-fetching.
This cycle consumed that data directly and mapped it onto the corpus's own per-record filenames —
several of those 18 AoN "species" cover more than one distinct corpus record (Bird: Eagle/Hawk/Owl
= 3 records sharing one stat block; Cat Big: Lion/Tiger = 2; Cat Small: Cheetah/Leopard = 2), which
the 196-record population counts separately, so the 18 AoN entries expand to **22 table entries**:

`badger_wolverine`, `bear`, `bird_eagle`, `bird_hawk`, `bird_owl`, `boar`, `camel`,
`cat_big_lion`, `cat_big_tiger`, `cat_small_cheetah`, `cat_small_leopard`, `crocodile_alligator`,
`dog`, `pony`, `dire_rat`, `herd_animal_ram`, `hippopotamus`, `primate_baboon`, `ray_manta`,
`ray_stingray`, `turtle_giant_snapping`, `snake_constrictor`.

Every one of the 22 corpus filenames was independently confirmed to exist under
`data/corpus/core_rulebook/companion/` before being added (`scratch_check.py`, not committed —
verification tooling, not shipped code). For every entry, the corpus's own `data.natural_armor`
field (or its absence, treated as base 0) was independently compared against AoN's printed "+n
natural armor" line: **agreement on all 22**, the same independent-tiebreaker discipline cycles
6-9 already established, re-run fresh this cycle rather than assumed from the brief's own summary
table. `Snake, Constrictor`'s corpus record (`companion_snake_constrictor.json`) exists separately
from the two `companion_snake_viper.json`/`companion_advancement_snake_viper.json` records cycle
9's own gathered-data table flagged as unresolved (Viper); only Constrictor was added this cycle,
Viper left for a future cycle to re-fetch by the direct-URL route cycle 9 already identified as
reliable.

Table: **52 → 74**.

## Test evidence

```
cargo test --locked -p codex --lib companion_base_stat_table   # 12 passed, 0 failed (was 11)
cargo test --locked -p codex --lib pilot_compute::               # 956 passed, 0 failed (was 952)
cargo test --locked -p codex --lib companion                    # 122 passed, 0 failed (was 121)
```

`apps/desktop/src-tauri` (separate cargo workspace) re-run per the brief's own instruction — no
desktop-crate file touched this cycle (`codex`-lib-only change): **548 passed, 0 failed**
(82.02s) — exactly matching cycle 9's own exit state, confirming this cycle's change is
zero-regression.

## PI / audit

- Own-diff (`git diff --unified=0 HEAD` over `companion_base_stat_table.rs`):
  `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — zero hits (`OK_NO_BUNDLE_TAGS`).
- Same scope, `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` — zero
  hits (`OK_NO_TOKENS`).
- `pi_scrub.normalized_term_hits()` (imported from `scripts/pi_scrub.py`, never copied) over the
  own-diff — zero hits, first pass clean.
- No `data/corpus/` write this cycle.

## Territory

`git status --porcelain` confirmed clean before every write and listed only
`companion_base_stat_table.rs` (plus `kanban.md` and this receipt) after. `kanban.md` row parsing
verified before and after the edit with a backtick-aware Python parser: same row count, same
unique row ids, 0 duplicates, row 20's own diff is exactly one line. Rows 11 and 15 left
untouched (not present in the diff).

## Next-cycle plan

1. **Grind the remaining 120 untagged records**: `Snake, Viper` and `Gar` (cycle 9's own two
   unresolved fetches — a more targeted direct-URL fetch, not `WebSearch`, is the route cycle 9
   found reliable), plus the remainder of the untagged population outside `core_rulebook`
   (`beastiary`, `ultimate_wilderness`, `inner_sea_combat`, `bestiary_5`, `advanced_race_guide`,
   etc — not yet sampled this cycle).
2. `pachycephalosaurus`/`ornithomimosaur`: still refuse, unchanged from cycle 7.
3. Row 20 stays `in-progress` under `decisions.md §10` until the full 196-record companion
   population is grounded or the residual is further resized with evidence.
