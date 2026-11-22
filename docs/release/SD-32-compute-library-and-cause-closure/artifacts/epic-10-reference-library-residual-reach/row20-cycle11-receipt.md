# Cycle row20-cycle11 — Epic 10 (`epic-10-reference-library-residual-reach`) / Row 20

- **Card ID:** `epic-10-reference-library-residual-reach`
- **Files touched:**
  - `src/rules_core/pilot_compute/companion_base_stat_table.rs` (30 new struct-literal entries:
    the two named dinosaur refusals resolved, `pachycephalosaurus`/`ornithomimosaur`, plus 28 of
    the 120 untagged-outside-`core_rulebook` records; a new module-doc cycle-11 addendum; one
    updated positive-count test; one new 30-species positive test; the now-false refusal test
    replaced with a positive `_no_longer_refuse` test; 30 new entries in the
    `companion_display_name` coverage test).
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (this cycle's own row 20
    entry, appended).
  - This receipt.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own diff, `git diff --unified=0 HEAD` over
  the touched source file, `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — zero
  hits).
- **Wired-integration audit result:** `OK_NO_TOKENS` (`grep -nE '\b(STUB|MOCK|placeholder|not yet
  implemented|todo|fixme|hack)\b'` over the same own-diff scope — zero hits). No new dispatch
  point: the 30 new entries flow through the existing `ground_companion_stat_block` /
  `ground_selected_companion_or_default` dispatch cycle 7 already wired end to end.
- **PI scrub:** `pi_scrub.normalized_term_hits()` (imported, never copied) over both the source
  own-diff and the kanban own-diff — zero hits in either, first pass clean.
- **Acceptance criterion:** Epic 10, row 20 — companion base-ability-score table residual: close
  or precisely size, per species, every `RACETYPE:Companion` corpus record with no verified base
  vector, per `decisions.md §27b` ("EVERYTHING", no carve-outs) and `§1a` (refuse rather than
  fabricate).
- **Status:** `in-progress` (population re-derived, unchanged shape: 196 total. 104 now grounded
  — up from 74 — leaving **92** real, sized, unbuilt: all untagged, 0 named refusals remaining).
- **Corpus SHA:** oracle bootstrapped fresh this worktree (slot was empty), pinned at
  `7f818006e371188e5717fd18d74d18a420747fc6` — the same commit every prior cycle used, confirmed
  via `scripts/fetch-pcgen-oracle.sh --dest $PCGEN_REPO_DIR`.
- **Discovery forwards:** none new this cycle.
- **Next-cycle plan:** grind the remaining 92 untagged records. `ultimate_wilderness` (38) is the
  single largest remaining bucket; `core_rulebook`'s own residual 6 includes `Snake, Viper` and
  `Gar`, both already sampled this cycle's own data-gathering pass but not yet added.

## Starting state (verified, not assumed)

`git rev-parse HEAD` on entry equalled `a901a70904e5408f348f9d925812144f5eeadae7`, which is `$PIN`
itself, and `git rev-parse origin/tranche/12` matched the same commit exactly — no rebase needed,
`BASE_OK` confirmed directly with `git merge-base --is-ancestor "$PIN" HEAD`. `git status
--porcelain` was clean at entry.

Oracle slot was empty (fresh worktree, git-ignored). `scripts/fetch-pcgen-oracle.sh --dest
$PCGEN_REPO_DIR` populated it at `7f818006e371188e5717fd18d74d18a420747fc6`, confirmed via its own
stdout (`OK 7f818006e371...`).

## `§17a` re-derivation

Re-derived the population directly from `data/corpus/*/companion/*.json`, filtering
`data.monster_class` starting with `"Companion"` and excluding "Companion Advancement" records:
**196 total, 144 untagged (`race_subtype` absent)** — identical to cycles 8-10's own figures,
confirming no movement in the population itself before this cycle's own additions. Of the 144
untagged, 24 were already grounded (`wolf`, `horse`, cycle 10's 22 `core_rulebook` records),
leaving exactly **120** — matching cycle 10's own next-cycle figure exactly.

## Work this cycle

### Part 1: the two named dinosaur refusals

Cycle 7 named `pachycephalosaurus` and `ornithomimosaur` as refusals — no source that cycle (or
any since) could reach separated the companion's own "Starting Statistics" block from either the
full-grown monster stat block or an ambiguous shared-template baseline.

`legacy.aonprd.com/bestiary/animalCompanions.html` — a single consolidated page carrying dozens of
companions' own printed "Starting Statistics" blocks directly, discovered this cycle — lists
**Pachycephalosaurus** in full: Str 15, Dex 16, Con 13, Int 2, Wis 12, Cha 5, +3 natural armor.
Cross-checked against an independent, targeted second fetch
(`aonprd.com/DruidCompanions.aspx?ItemName=Dinosaur+(Pachycephalosaurus)`) — exact agreement. The
corpus's own `data/corpus/bestiary_3/companion/companion_pachycephalosaurus.json` `natural_armor`
field reads `3` — agrees with both.

`Ornithomimosaur` is not on the consolidated page (an Ultimate Wilderness record, outside that
page's Bestiary-3-era scope). A targeted `aonprd.com/DruidCompanions.aspx?ItemName=Ornithomimosaur`
fetch returned Str 11, Dex 15, Con 12, Int 2, Wis 13, Cha 8, +1 natural armor. Cross-checked against
the pinned PCGen oracle's own raw source directly (`git show HEAD:data/pathfinder/paizo/
roleplaying_game/ultimate_wilderness/uw_races_companion.lst` inside `$PCGEN_REPO_DIR`):
`BONUS:STAT|DEX|4 BONUS:STAT|CON|2 BONUS:STAT|INT|-8 BONUS:STAT|WIS|2 BONUS:STAT|CHA|-2` (no STR
delta) and `BONUS:VAR|AC_Natural_Armor|1|TYPE=Base` — internally consistent with the fetched
printed total and an exact match on natural armor against the corpus's own `natural_armor: 1`
token (`data/corpus/ultimate_wilderness/companion/companion_ornithomimosaur.json`).

Both now ground real, verified stat blocks. **28 of 28 `AnimalCompanionDinosaur` records are
grounded.**

### Part 2: 28 of the 120 remaining untagged records

The same `legacy.aonprd.com` consolidated page turned out to carry the printed "Starting
Statistics" for most of `bestiary_3` and `bestiary_4`, plus part of `bestiary_5` — closing **all 12
of `bestiary_3`** (`antelope`, `archelon`, `axe_beak`, `baluchitherium`, `basilosaurus`, `elk`,
`giant_chameleon`, `giant_gecko`, `giant_vulture`, `kangaroo`, `megalania`, `thylacine`), **all 7 of
`bestiary_4`** (`weasel_giant`, `giraffe`, `seahorse`, `stag`, `tortoise`, `trumpeter_swan`,
`walrus`), and **8 of 12 in `bestiary_5`** (`whale_blue`, `chalicotherium`, `digmaul`,
`kaprosuchus`, `moa`, `narwhal`, `uintatherium`, `wolliped`) — 27 records. `bestiary_5`'s remaining
4 (`frog_father`, `frog_goliath`, `polar_bear`, `polar_bear_dire`) are not on the consolidated page
and are left for a future cycle.

`Giant Vulture` is reprinted verbatim in both `bestiary_3` and `monster_codex`
(`data/corpus/bestiary_3/companion/companion_giant_vulture.json` and `data/corpus/monster_codex/
companion/companion_giant_vulture.json` — identical `stat_adjustments` and `natural_armor` tokens,
confirmed field by field). Per this module's own precedent (`bird_eagle`/`bird_hawk`/`bird_owl`,
`cat_big_lion`/`cat_big_tiger`: the 196-record population counts records, not species), both
records get their own table entry: `giant_vulture` (`bestiary_3`) and
`giant_vulture_monster_codex` (`monster_codex`) — the 28th entry.

Every one of the 28 filenames was independently confirmed to exist under its own book's
`companion/` directory before being added. For every entry, the corpus's own `natural_armor` field
(or its documented absence, treated as base 0) was independently compared against the consolidated
page's own printed "AC ... natural armor" line: **agreement on all 28**, the same
independent-tiebreaker discipline cycles 6-10 already established. No delta was backed out anywhere
in this cycle's diff — every Str/Con value is the printed total, stored directly, matching cycle 9's
corrected methodology throughout.

Table: **74 → 104** (74 + 2 refusals resolved + 28 untagged).

## Test evidence

```
cargo test --locked -p codex --lib companion_base_stat_table   # 13 passed, 0 failed (was 12)
cargo test --locked -p codex --lib pilot_compute::               # 960 passed, 0 failed (was 956)
cargo test --locked -p codex --lib companion                    # 123 passed, 0 failed (was 122)
```

`apps/desktop/src-tauri` (separate cargo workspace) re-run per the brief's own instruction — no
desktop-crate file touched this cycle (`codex`-lib-only change): **548 passed, 0 failed**
(83.42s) — exactly matching cycle 10's own exit state, confirming this cycle's change is
zero-regression.

## PI / audit

- Own-diff (`git diff --unified=0 HEAD` over `companion_base_stat_table.rs`):
  `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — zero hits (`OK_NO_BUNDLE_TAGS`).
- Same scope, `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` — zero
  hits (`OK_NO_TOKENS`).
- `pi_scrub.normalized_term_hits()` (imported from `scripts/pi_scrub.py`, never copied) over the
  own-diff of both `companion_base_stat_table.rs` and `kanban.md` — zero hits, first pass clean.
- No `data/corpus/` write this cycle.

## Territory

`git status --porcelain` confirmed clean before every write and listed only
`companion_base_stat_table.rs` (plus `kanban.md` and this receipt) after. `kanban.md` row parsing
verified before and after the edit with a Python parser: 21 data rows, 21 unique row ids, 0
duplicates, `git diff --stat` shows exactly 1 line changed (1 insertion, 1 deletion — the row
grows in place, still one physical line). Rows 11 and 15 left untouched (not present in the diff).

## Next-cycle plan

1. **Grind the remaining 92 untagged records**, largest first: `ultimate_wilderness` (38),
   `beastiary` (14), `ultimate_magic` (9), `advanced_race_guide` (6), `bestiary_6` (6),
   `core_rulebook` (6, including `Snake, Viper` and `Gar` — both already have corpus deltas
   sampled this cycle, ready for the next cycle to source directly), `monster_codex` (5 residual:
   `cave_salamander`, `gorthek`, `python_riding`, `rat_riding`, `yzobu`), `bestiary_5` (4 residual:
   `frog_father`, `frog_goliath`, `polar_bear`, `polar_bear_dire`), `inner_sea_combat` (3, including
   `griffon`, which the module's own `an_unknown_species_slug_refuses_rather_than_guesses` test
   pins as a live refusal example — must stay ungrounded until this test is updated alongside it),
   `horror_adventures` (1).
2. Row 20 stays `in-progress` under `decisions.md §10` until the full 196-record companion
   population is grounded or the residual is further resized with evidence.
