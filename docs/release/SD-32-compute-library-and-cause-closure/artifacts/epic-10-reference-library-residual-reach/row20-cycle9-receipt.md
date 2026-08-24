# Cycle row20-cycle9 — Epic 10 (`epic-10-reference-library-residual-reach`) / Row 20

- **Card ID:** `epic-10-reference-library-residual-reach`
- **Files touched:**
  - `src/rules_core/pilot_compute/companion_base_stat_table.rs` (44 struct-literal corrections,
    a new cycle-9 module-doc addendum, two corrected `gulper_plant_*` tests, three corrected
    "first tuple carries `i16`" test-table entries the earlier bulk regex pass missed).
  - `apps/desktop/src-tauri/src/character_hub.rs` (one pinned test's hardcoded expected value
    corrected — see "A pinned test encoded the old wrong assumption" below).
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (this cycle's own row 20
    entry, appended).
  - This receipt.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own diff, `git diff --unified=0 HEAD` over
  the three touched source/kanban files, `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  — zero hits, each file checked separately).
- **Wired-integration audit result:** `OK_NO_TOKENS` (`grep -nE '\b(STUB|MOCK|placeholder|not yet
  implemented|todo|fixme|hack)\b'` over the same own-diff scope — zero hits). No new dispatch
  point, no new dead code: this cycle corrected existing grounded values, added no new species.
- **PI scrub:** `pi_scrub.normalized_term_hits()` (imported, never copied) over each own-diff —
  zero hits, all three files, first pass clean.
- **Acceptance criterion:** Epic 10, row 20 — companion base-ability-score table residual: close
  or precisely size, per species, every `RACETYPE:Companion` corpus record with no verified base
  vector, per `decisions.md §27b` ("EVERYTHING", no carve-outs) and `§1a` (refuse rather than
  fabricate — extended this cycle to: a wrong-but-confident number is the same failure as a
  fabricated one, and must be found and fixed with the same urgency as a gap).
- **Corpus SHA:** oracle bootstrapped fresh this worktree (slot was empty), pinned at
  `7f818006e371188e5717fd18d74d18a420747fc6` — the same commit every prior cycle used, confirmed
  via `scripts/fetch-pcgen-oracle.sh --dest $PCGEN_REPO_DIR`.
- **Status:** `in-progress` (unchanged — 144 of 196 companion species remain real, sized, unbuilt
  work: 142 untagged records plus 2 named dinosaur refusals; **this cycle's own scope was a
  correctness fix to the 52 already-grounded species, not new coverage — the 52/196 count is
  unchanged**).
- **Notes:** see body below for the correctness defect this cycle found and fixed, and why it
  took priority over grinding the 142 untagged species this cycle's own brief named as its scope.
- **Discovery forwards:** none new this cycle (the defect below was found and closed same-cycle,
  not forwarded).
- **Next-cycle plan:** grind the 142 untagged species — now with the CORRECTED methodology (the
  printed AoN "Starting Statistics" total grounded DIRECTLY, never backed out by the corpus's own
  per-species `BONUS:STAT` delta). This cycle already gathered verified printed totals for 21 of
  those 142 (all core_rulebook, badger/bear/bird/boar/camel/cat-big/cat-small/crocodile/dog/pony/
  dire-rat/herd-ram/hippo/primate/ray-manta/ray-stingray/turtle) via `WebFetch`/`WebSearch`
  against aonprd.com — not yet added to the table this cycle (time went to the correctness fix
  instead); a future cycle can consume that already-gathered data directly rather than re-fetching
  it. See "Data gathered but not yet committed" below.

## Starting state (verified, not assumed)

`git rev-parse HEAD` on entry equalled `1bb523773d` (the SD-31 PR #374 merge commit), the same
stale-lineage footgun most prior cycles in this row hit (footgun 3). Recovered: `git reset --hard
$PIN` (`56398621466837ff844b7e53bdefd52296d4ad5a`), re-verified `git merge-base --is-ancestor "$PIN"
HEAD` (`BASE_OK`). `git rebase origin/tranche/12` reported "up to date" — `origin/tranche/12`'s own
tip already equalled `$PIN` (cycle 8's own commit, `daa6804a94`... actually confirmed as
`5639862146` after `git log`, the T12 row18 cycle12 commit; both `HEAD` and `origin/tranche/12`
matched `$PIN` exactly, no rebase needed, no sibling collision).

Oracle slot was empty (fresh worktree, git-ignored). `scripts/fetch-pcgen-oracle.sh --dest
$PCGEN_REPO_DIR` populated it at `7f818006e371188e5717fd18d74d18a420747fc6`, confirmed via its own
stdout (`OK 7f818006e371...`).

## `§17a` re-derivation: the population count (144/196) is unchanged, unlike three prior figures

Re-derived the same way cycle 8 did (`python3` filtering `data/corpus/*/companion/*.json` records
whose `monster_class` starts with `"Companion"`, excluding "Companion Advancement (...)" records):
**196 total**, **144 untagged** (`race_subtype: None`), 28 `AnimalCompanionDinosaur`, 12 `Aquatic`,
8 `PlantCompanion`, 4 `AnimalCompanionPrimate` — identical to cycle 8's own figures, no movement.
Of the 144 untagged, 2 (`companion_wolf.json`, `companion_horse.json`) already have grounded
table entries (`"wolf"`, `"horse"`), leaving **142 unbuilt untagged records**, plus the 2 named
dinosaur refusals cycle 7 left standing (`pachycephalosaurus`, `ornithomimosaur`) — **144 total**,
matching this cycle's own brief exactly. Reporting the non-movement, per `§17a`'s own standing
instruction to report it as loudly as movement: this figure did not move this cycle.

## The correctness defect this cycle found before grinding the 142

Before adding any new species, re-deriving the population handed down (`§17a`) led to reading
[`ground_companion_stat_block`] (`companion_base_stat_table.rs`) to confirm exactly how the
table's `strength`/`constitution` fields get consumed, ahead of gathering the first new AoN data
point. That read surfaced a real defect:

```
let strength_bonus = super::animal_companion_stat_bonus(companion_level);
let strength_score = stats.strength + strength_bonus;
```

`animal_companion_stat_bonus` is a **universal, species-agnostic** formula
(`floor(MasterLevel/3)`, the companion CLASS's own `BONUS:STAT|STR,DEX|floor(MasterLevel/3)`,
`core_rulebook/cr_abilities_companion.lst:60`) — it is applied identically to every species and
never reads the corpus RACE record's own per-species `BONUS:STAT` delta. `stats.strength`
therefore must hold the species' **printed 1st-level "Starting Statistics" total directly** —
exactly what `WOLF_COMPANION_STRENGTH_SCORE`/`HORSE_COMPANION_STRENGTH_SCORE` (this module's
parent, the two hand-verified foundational species) already do: `WOLF_COMPANION_STRENGTH_SCORE =
13`, the printed AoN/d20pfsrd/corpus-citation total, confirmed against that constant's own doc
comment ("Base ability scores ... Str 13") and its consumer's own inline comment ("The companion
class's own level-scaling Strength bonus stacks on **the race's base score**").

Cycles 5-8 instead subtracted the corpus's own per-species `BONUS:STAT` delta from the printed AoN
total before storing it, on the theory that the delta needed "backing out" to reach a
pre-advancement base. Re-fetching Gulper Plant's own AoN page directly
(`aonprd.com/DruidCompanions.aspx?ItemName=Gulper%20Plant`) confirmed the printed total is **Str
12 / Con 13** — but the table stored **Str 10 / Con 11**, the delta-backed-out value, matching
cycle 5's own doc comment exactly ("both agree on a base of Str 10 / Con 11 once the delta is
backed out"). The corpus's own per-species `BONUS:STAT` delta is PCGen's own internal
delta-from-template bookkeeping (how the RACE file reconstructs the printed total from PCGen's own
default ability array) — unrelated to, and never read by, this engine's own companion-advancement
math.

**Scale of the defect:** every one of the 44 species (of 52 total grounded) with a nonzero
Strength or Constitution corpus delta was affected — all 23 aquatic/plant/primate species (cycle
8), 20 of 26 dinosaurs (cycles 6-7; six had a genuine zero delta on both stats and were never
wrong — `triceratops`, `stegosaurus`, `diplodocus`, `styracosaurus`, `kentrosaurus`,
`tylosaurus`), and `gulper_plant` itself (cycle 5). Wolf and Horse were never affected — they were
always grounded directly from the printed total, the correct method all along.

**Confirmed systematically, not just for Gulper Plant:** re-parsed every entry's own doc comment
(which already recorded the correct printed AoN total in "AoN: Str X ... Con Y" text — only the
struct literal itself was wrong) and compared against the stored struct-literal value:

```python
# for every out.insert(...) block, parse the "AoN: Str X ... Con Y" comment and compare
# to the struct literal's strength/constitution fields
# -> 44 of 48 regex-matched entries mismatched (the other 4 had a genuine zero delta)
```

## The fix

All 44 affected struct literals now hold the printed AoN total each entry's own doc comment
already recorded — **no new external verification was needed**, the correct number was already
sitting in the comment. Natural armor was **never affected**: the corpus's own `AC_Natural_Armor`
token was always the base value directly (not a delta), independently confirmed against AoN's own
printed "+n natural armor" line for every entry across cycles 6-8 (100% agreement, unchanged by
this fix).

A new module-doc addendum (`# Cycle 9 addendum`) records the full derivation at the top of
`companion_base_stat_table.rs`, ahead of the pre-existing cycle-8 addendum. The per-entry inline
"Base Str X-Y=Z" arithmetic sentences (48 of them, describing the now-superseded pre-fix
derivation) were **not** hand-edited line by line at this scale — the struct literals and the
tests below them are the operative, now-corrected source of truth, and the new module-doc addendum
is the authoritative record naming this explicitly, so a future reader is never misled by the
stale inline arithmetic.

### Tests corrected

Five tests asserted exact Str/Con values and needed updating to match the corrected data (not
weakened — corrected to the real values, same discipline as `§3`):

- `the_nine_dinosaur_companions_ground_their_own_verified_base_scores` (9 species)
- `the_seventeen_cycle_seven_dinosaur_companions_ground_their_own_verified_base_scores` (17 species)
- `the_twenty_three_cycle_eight_aquatic_plant_and_primate_companions_ground_their_own_verified_base_scores`
  (23 species)
- `gulper_plant_grounds_a_real_new_species_at_master_level_1` (base_attack_bonus: 1 → 2, since Str
  12's modifier is +1, not Str 10's +0)
- `gulper_plant_base_saves_and_armor_class_at_master_level_1` (hit_points: 13 → 15, since Con 13's
  modifier is +1, not Con 11's +0; armor_class unaffected, natural armor was always correct)

The first bulk regex correction pass (`fix_tests.py`) missed 3 of the 43 species-tuple entries
(`allosaurus`, `elasmosaurus`, `eel_giant_moray`) because those three happen to be the FIRST tuple
in their own array literal, which carries an explicit `i16` type-annotation suffix on all three
numeric fields (`10i16, 10i16, 4i16`) rather than the bare `10, 10, 4` every other tuple in the
same array uses — the regex's number-pattern did not account for the suffix appearing on the
*second* field too. Caught by running the tests, not assumed clean from the script's own reported
match count: `cargo test` failed those exact three with the OLD (wrong) values still asserted,
diagnosed by comparing the failing assertion's own printed detail string (which correctly showed
the NEW table value) against the test's own stale expectation, then fixed by hand for those three
lines specifically.

### A pinned test encoded the old wrong assumption — corrected, not deleted

`apps/desktop/src-tauri/src/character_hub.rs`'s own
`a_druid_who_selects_gulper_plant_grounds_gulper_plant_not_wolf_at_character_creation_altitude`
asserted `base_attack.value == 1` with an inline comment citing the exact Str-10-modifier-+0
derivation this cycle's fix corrects. Running the full desktop suite (below) caught this exact
failure. Per the brief's own "a pinned test can encode a wrong assumption" guidance: the test's
own SAFETY property (a Druid who selects a verified companion species must ground that species'
real stat block through the real character-creation request path, not Wolf's) is still true and
still worth proving — only its hardcoded expected NUMBER was wrong, inherited from the same defect
this cycle fixed. Corrected to `2` (Str 12's +1 modifier) with an inline comment explaining the
correction and pointing at `companion_base_stat_table.rs`'s own cycle-9 addendum, rather than
deleted or weakened.

## Why this took priority over the 142 untagged species this cycle's own scope named

`decisions.md §1a`: a gate that cannot fail is worse than no gate — an uncaught systematic error in
already-shipped, character-creation-reachable compute output is the same failure by another name.
Adding 20-30 more species entries this cycle using the same wrong subtraction method (which this
cycle had already begun gathering AoN data for, using the established cycles-6-8 methodology,
before discovering the defect mid-gather) would have compounded the defect at scale rather than
fixing it. Fixing the 44 already-"closed" species first, and leaving a corrected methodology and
already-gathered data for the next cycle, is more honest work than adding new entries on a broken
foundation.

## Data gathered but not yet committed (for the next cycle)

Before the defect was found, this cycle fetched AoN "Starting Statistics" (cross-checked against
d20pfsrd for Bear/Boar via a single combined fetch, an exact match on both) for 21 of the 142
untagged `core_rulebook` species: Badger (Wolverine), Bear, Bird (Eagle/Hawk/Owl — one shared
stat block covering 3 corpus records), Boar, Camel, Cat Big (Lion/Tiger — one shared stat block
covering 2 corpus records), Cat Small (Cheetah/Leopard — one shared stat block covering 2 corpus
records), Crocodile (Alligator), Dog, Pony, Dire Rat, Herd Animal (Ram), Hippopotamus, Primate
(Baboon), Ray (Manta), Ray (Stingray), Turtle (Giant Snapping). Printed totals (Str/Con) and
natural armor, verified against the corpus's own `natural_armor` field matching exactly in every
case checked:

| species | AoN Str/Con | AoN natural armor | corpus armor field |
|---|---|---|---|
| Badger (Wolverine) | 10/15 | +2 | 2 |
| Bear | 15/13 | +2 | 2 |
| Bird (Eagle/Hawk/Owl) | 10/12 | +1 | 1 (all three) |
| Boar | 13/15 | +6 | 6 |
| Camel | 18/14 | +1 | 1 |
| Cat, Big (Lion/Tiger) | 13/13 | +1 | 1 (both) |
| Cat, Small (Cheetah/Leopard) | 12/13 | +1 | 1 (both) |
| Crocodile (Alligator) | 15/15 | +4 | 4 |
| Dog | 13/15 | +2 | 2 |
| Pony | 13/12 | +2 | 2 |
| Dire Rat | 10/12 | none | None |
| Herd Animal (Ram) | 10/11 | +1 | 1 |
| Hippopotamus | 11/12 | +6 | 6 |
| Primate (Baboon) | 12/12 | none | None |
| Ray (Manta) | 8/11 | +1 | 1 |
| Ray (Stingray) | 6/13 | none | None |
| Turtle (Giant Snapping) | 8/9 | +10 | 10 |
| Snake, Constrictor | 15/13 | +2 | 2 |

`Snake, Viper` and `Gar` searches did not return a distinguishable page this cycle (WebSearch
returned only the Constrictor page for "Viper", and no page at all for "Gar"); not fabricated,
left for the next cycle to re-fetch by a more targeted route (the direct
`DruidCompanions.aspx?ItemName=...` URL pattern, not `WebSearch`, worked reliably wherever tried
directly). **None of the above 18 species were added to the table this cycle** — verified data
only, named here so the next cycle does not need to re-fetch it, per `§12c` (every figure names
its population and how it was obtained).

## Test evidence

```
cargo test --locked -p codex --lib companion_base_stat_table   # 11 passed, 0 failed (unchanged count)
cargo test --locked -p codex --lib pilot_compute::               # 952 passed, 0 failed (was 950 cycle 8)
cargo test --locked -p codex --lib companion                    # 121 passed, 0 failed (unchanged)
```

`apps/desktop/src-tauri` (separate cargo workspace) re-run per the brief's own instruction:
first run surfaced the one pinned-test failure above (547 passed, 1 failed); after the fix,
`cargo test --locked --bin codex-desktop` -> **548 passed, 0 failed** (81.67s) — matching cycle
8's own 548/0 exit state exactly.

## PI / audit

- Own-diff (`git diff --unified=0 HEAD`), each of the three touched files checked separately:
  `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — zero hits (`OK_NO_BUNDLE_TAGS`).
- Same scope, `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` — zero
  hits (`OK_NO_TOKENS`).
- `pi_scrub.normalized_term_hits()` (imported from `scripts/pi_scrub.py`, never copied) over each
  own-diff — zero hits, all three files, first pass clean.
- No `data/corpus/` write this cycle.

## Territory

`git status --porcelain` confirmed clean before every write and listed only the three intended
files after (plus this receipt). `kanban.md` row parsing verified before and after the edit with
a backtick-aware Python parser: 21 pipe-lines (21 data rows), 21 unique row ids, 0 duplicates, row
20's own cells split to 9 raw segments (7 logical columns), unchanged shape — the diff against
`kanban.md` is exactly one line (row 20's own), confirmed via `git diff --stat`. Rows 11 and 15
left untouched (not present in the diff).

## Next-cycle plan

1. **Grind the 142 untagged species using the CORRECTED methodology**: ground the printed AoN
   "Starting Statistics" total directly, never back it out by the corpus's own per-species
   `BONUS:STAT` delta. The 18 species this cycle already gathered data for (table above) can be
   added directly without re-fetching.
2. `pachycephalosaurus`/`ornithomimosaur`: still refuse, unchanged from cycle 7 — revisit if a
   future cycle finds a source neither cycle 7 nor any cycle since could reach.
3. Row 20 stays `in-progress` under `decisions.md §10` until the full 196-record companion
   population is grounded or the residual is further resized with evidence.
