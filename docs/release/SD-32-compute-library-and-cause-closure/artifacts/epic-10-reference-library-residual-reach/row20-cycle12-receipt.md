# Cycle row20-cycle12 — Epic 10 (`epic-10-reference-library-residual-reach`) / Row 20

- **Card ID:** `epic-10-reference-library-residual-reach`
- **Files touched:**
  - `src/rules_core/pilot_compute/companion_base_stat_table.rs` (38 new
    struct-literal entries covering all remaining `ultimate_wilderness`
    untagged companion records; a new module-doc cycle-12 addendum; the
    population-count test updated 104 -> 142; a new 38-species positive
    test; 38 new entries in the `companion_display_name` coverage test).
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (this
    cycle's own row 20 entry, appended).
  - This receipt.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own diff, `git diff
  --unified=0 HEAD` over the touched source file, `grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — zero hits).
- **Wired-integration audit result:** `OK_NO_TOKENS` (`grep -nE
  '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` over
  the same own-diff scope — zero hits). No new dispatch point: the 38 new
  entries flow through the existing `ground_companion_stat_block` /
  `ground_selected_companion_or_default` dispatch cycle 7 already wired end
  to end.
- **PI scrub:** `pi_scrub.normalized_term_hits()` (imported, never copied)
  over both the source own-diff and the kanban own-diff — zero hits in
  either, first pass clean.
- **Acceptance criterion:** Epic 10, row 20 — companion base-ability-score
  table residual: close or precisely size, per species, every
  `RACETYPE:Companion` corpus record with no verified base vector, per
  `decisions.md §27b` ("EVERYTHING", no carve-outs) and `§1a` (refuse
  rather than fabricate).
- **Status:** `in-progress` (population re-derived, unchanged shape: 196
  total. 142 now grounded — up from 104 — leaving **54** real, sized,
  unbuilt, all untagged, 0 named refusals introduced this cycle).
- **Corpus SHA:** oracle bootstrapped fresh this worktree (slot was empty),
  pinned at `7f818006e371188e5717fd18d74d18a420747fc6` — the same commit
  every prior cycle used, confirmed via
  `scripts/fetch-pcgen-oracle.sh --dest $PCGEN_REPO_DIR`.
- **Discovery forwards:** none new this cycle.
- **Next-cycle plan:** grind the remaining 54 untagged records, largest
  first: `beastiary` (14), `ultimate_magic` (9), `advanced_race_guide` (6),
  `bestiary_6` (6), `core_rulebook` (6), `monster_codex` (5 residual),
  `bestiary_5` (4 residual), `inner_sea_combat` (3, including `griffon`, a
  live pinned refusal example), `horror_adventures` (1).

## Starting state (verified, not assumed) — footgun 3 fired, corrected

`git rev-parse HEAD` on entry was `1bb523773d32705d1b7387fd4c494861523f55b`
— a `tranche/11` merge-commit descendant (PR #374's merge into
`tranche/11`), **not** a descendant of `$PIN`
(`e2c3e73956701988090b56e085b6a8ba45d22937`). `git merge-base --is-ancestor
"$PIN" HEAD` failed (`BASE_FAIL`) — the fresh worktree started on a stale
lineage, the exact failure mode this bundle's brief names as having hit
twenty-five other lanes.

Corrected per the brief's own instruction: `git fetch origin tranche/12`
confirmed `origin/tranche/12`'s own tip is `e2c3e73956...` — **exactly
`$PIN`** (not merely a descendant, the identical commit). `git reset --hard
"$PIN"` restored `BASE_OK` (`git merge-base --is-ancestor "$PIN" HEAD`
passed) with no rebase needed afterward, since `$PIN` already equalled
`origin/tranche/12`'s tip. `git status --porcelain` was clean at that
point.

Oracle slot was empty (fresh worktree, git-ignored). `scripts/fetch-pcgen-
oracle.sh --dest $PCGEN_REPO_DIR` populated it at
`7f818006e371188e5717fd18d74d18a420747fc6`, confirmed via its own stdout.

## `§17a` re-derivation

Re-derived the `ultimate_wilderness` base-race population directly from
`data/corpus/ultimate_wilderness/companion/*.json`, filtering
`data.monster_class` starting with `"Companion"` and excluding
`companion_advancement_*.json` (a different record type, ability grants,
not base-race entries) and `companion_body_type_*.json` (shared template
records, not per-species base entries): **52 total** records carry
`MONSTERCLASS:Companion:*`. Of those 52, `gulper_plant` (cycle 5) and
`ornithomimosaur` (cycle 11) were already grounded, leaving exactly **38**
— matching cycle 11's own next-cycle figure exactly, confirming no
movement in the underlying population.

## Work this cycle

`aonprd.com/DruidCompanions.aspx?ItemName=All&Category=Animal` — a full
alphabetical index of animal-companion species this site tracks across
every sourcebook, discovered this cycle — was fetched first and confirmed
to list all 38 target slugs by printed name (with a few punctuation/typo
quirks in the site's own `ItemName` query values, e.g. "Assassin Bug,
Giant" rather than "Assassin Bug (Giant)", each confirmed by fetching the
individual page and checking the returned source book/page number matched
"Ultimate Wilderness").

Each of the 38 species' own `aonprd.com/DruidCompanions.aspx?ItemName=
<species>` page was then fetched directly, returning its printed "Starting
Statistics" line verbatim (Str/Dex/Con/Int/Wis/Cha and natural armor bonus)
plus its own confirmed source book and page (Ultimate Wilderness pp.
178–185, one page per species, printed in every fetch response). Every
fetch independently named its own source book, serving as this cycle's own
cross-check that the correct species/book was reached (the same
verification role a second independent search served in cycles 6–11, here
achieved by the page's own self-reported citation on a domain distinct
from the corpus).

Every one of the 38 filenames was independently confirmed to exist under
`data/corpus/ultimate_wilderness/companion/` (via the `§17a` population
re-derivation above) before being added. For every entry, the corpus's own
`natural_armor` field was compared against the fetched page's own printed
natural-armor bonus: **agreement on all 38** — the same 100% agreement
rate every prior cycle found. No delta was backed out anywhere in this
cycle's diff — every Str/Con value is the printed total, stored directly,
matching cycle 9's corrected methodology throughout.

Table: **104 → 142** (104 + 38). `ultimate_wilderness`'s base-race
`RACETYPE:Companion` population (52 records) is now **fully closed** (0
remain ungrounded).

## Test evidence

```
cargo test --locked -p codex --lib companion_base_stat_table   # 15 passed, 0 failed (was 14)
cargo test --locked -p codex --lib pilot_compute::               # 965 passed, 0 failed (was 963)
cargo test --locked -p codex --lib companion                    # 124 passed, 0 failed (was 123)
```

`apps/desktop/src-tauri` (separate cargo workspace) re-run per the brief's
own instruction — no desktop-crate file touched this cycle (`codex`-lib-
only change): **548 passed, 0 failed** (80.94s) — exactly matching cycle
11's own exit state, confirming this cycle's change is zero-regression.

## PI / audit

- Own-diff (`git diff --unified=0 HEAD` over
  `companion_base_stat_table.rs`): `grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — zero hits
  (`OK_NO_BUNDLE_TAGS`).
- Same scope, `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|
  todo|fixme|hack)\b'` — zero hits (`OK_NO_TOKENS`).
- `pi_scrub.normalized_term_hits()` (imported from `scripts/pi_scrub.py`,
  never copied) over the own-diff of both `companion_base_stat_table.rs`
  and `kanban.md` — zero hits, first pass clean.
- No `data/corpus/` write this cycle.

## Territory

`git status --porcelain` confirmed clean before every write and listed
only `companion_base_stat_table.rs` (plus `kanban.md` and this receipt)
after. `kanban.md` row parsing verified before and after the edit: 21 data
rows (`^\| [0-9]+ \|`), 21 unique row ids, `git diff --stat` shows exactly
1 line changed (1 insertion, 1 deletion — the row grows in place, still
one physical line). Rows 11 and 15 left untouched (not present in the
diff).

## Next-cycle plan

1. **Grind the remaining 54 untagged records**, largest first: `beastiary`
   (14), `ultimate_magic` (9), `advanced_race_guide` (6), `bestiary_6` (6),
   `core_rulebook` (6, including `Snake, Viper` and `Gar`), `monster_codex`
   (5 residual: `cave_salamander`, `gorthek`, `python_riding`,
   `rat_riding`, `yzobu`), `bestiary_5` (4 residual: `frog_father`,
   `frog_goliath`, `polar_bear`, `polar_bear_dire`), `inner_sea_combat` (3,
   including `griffon`, still this module's own live pinned refusal
   example — must stay ungrounded until that test is updated alongside
   it), `horror_adventures` (1).
2. Row 20 stays `in-progress` under `decisions.md §10` until the full
   196-record companion population is grounded or the residual is further
   resized with evidence.
