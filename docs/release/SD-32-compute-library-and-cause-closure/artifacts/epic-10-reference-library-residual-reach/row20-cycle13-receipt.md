# Cycle row20-cycle13 — Epic 10 (`epic-10-reference-library-residual-reach`) / Row 20

- **Card ID:** `epic-10-reference-library-residual-reach`
- **Files touched:**
  - `src/rules_core/pilot_compute/companion_base_stat_table.rs` (54 new
    struct-literal entries closing the full 196-record `RACETYPE:Companion`
    population; a new module-doc cycle-13 addendum; the population-count
    test renamed and updated 142 -> 196; a new positive `griffon` test; the
    refusal test's example slug moved off `griffon`; 54 new entries in the
    `companion_display_name` coverage test).
  - `apps/desktop/src-tauri/src/character_hub.rs` (the same `griffon`
    refusal-example correction at character-creation altitude, plus a new
    positive assertion proving `griffon` now grounds through the real
    request path rather than falling back to Wolf).
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (row
    20's own cell: `in-progress` -> `complete`, cycle list `+13`, this
    cycle's own note appended).
  - This receipt.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own diff, `git diff
  --unified=0 HEAD` over each touched source file, `grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — zero hits in
  either).
- **Wired-integration audit result:** `OK_NO_TOKENS` (`grep -nE
  '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` over
  the same own-diff scope — zero hits). No new dispatch point: all 54 new
  entries flow through the existing `ground_companion_stat_block` /
  `ground_selected_companion_or_default` dispatch cycle 7 already wired end
  to end; `character_hub.rs`'s own edit only corrects/extends an existing
  test, no production code changed there.
- **PI scrub:** `pi_scrub.normalized_term_hits()` (imported, never copied)
  over the own-diff of `companion_base_stat_table.rs`,
  `character_hub.rs`, and `kanban.md` — zero hits in all three, first
  pass clean.
- **Acceptance criterion:** Epic 10, row 20 — companion base-ability-score
  table residual: close or precisely size, per species, every
  `RACETYPE:Companion` corpus record with no verified base vector, per
  `decisions.md §27b` ("EVERYTHING", no carve-outs) and `§1a` (refuse
  rather than fabricate).
- **Status:** `complete` — see "Row 20 closure determination" below.
- **Corpus SHA:** oracle bootstrapped fresh this worktree (slot was empty),
  pinned at `7f818006e371188e5717fd18d74d18a420747fc6` — the same commit
  every prior cycle used, confirmed via
  `scripts/fetch-pcgen-oracle.sh --dest $PCGEN_REPO_DIR`.
- **Discovery forwards:**
  1. `dolphin_orca`'s corpus `natural_armor` token (6) disagrees with two
     independent printed sources (1) — traced to a PCGen oracle bug, not a
     corpus ingestion defect (full detail below). Documented in the module
     doc, not silently resolved either way.
  2. The `companion_pool_catalog.rs` "4-record `mod_only`" exclusion from
     cycles 1-2 (a *different* subsystem — companion ABILITY
     descriptions, not base ability SCORES) was never revisited after
     cycle 2 and reads, in today's code, as a settled architectural
     decision backed by a live passing test. Its own reasoning ("needs a
     real creature/character context ... which a browse-only catalog
     cannot supply") resembles the "no consumer reaches it" shape
     `decisions.md §27b` ruled inadmissible elsewhere. Flagged for the
     operator to confirm whether `§27b` reopens it; not investigated
     further this cycle (out of this cycle's own dispatched scope).

## Starting state (verified, not assumed) — footgun 4 fired, corrected

`git rev-parse HEAD` on entry was `1bb523773d32705d1b7387fd4c494861523f55b`
— a `tranche/11` merge-commit descendant (PR #374's merge), **not** a
descendant of `$PIN` (`0fc6c02f182a814e0b10cd46a438150dffec567b`). `git
merge-base --is-ancestor "$PIN" HEAD` failed. Corrected: `git fetch origin
tranche/12` confirmed `origin/tranche/12`'s own tip equalled `$PIN`
exactly; `git reset --hard "$PIN"` restored `BASE_OK`, no rebase needed
afterward. `git status --porcelain` was clean at that point.

Oracle slot was empty (fresh worktree, git-ignored). `scripts/fetch-pcgen-
oracle.sh --dest $PCGEN_REPO_DIR` populated it, confirmed at pin
`7f818006e371188e5717fd18d74d18a420747fc6` via the script's own stdout —
the same pin every prior cycle used. `scripts/verify.sh --only
preflight-oracle` re-run explicitly with both env vars exported: PASS.

## `§17a` re-derivation

Re-derived the residual population directly from `data/corpus/<book>/
companion/*.json` across all nine books cycle 12 named. **A first pass
using the `companion_*.json` filename glob every prior cycle used
undercounted `advanced_race_guide` by 2** — `brute_steed_camel.json`/
`brute_steed_horse.json`, both real `MONSTERCLASS:Companion:2` records,
are filed without the usual `companion_` filename prefix. Widened the
glob to every file in each book's `companion/` directory (still filtered
to `monster_class` starting with `"Companion"`, still excluding
`companion_advancement_*`/`companion_body_type_*`, a different record
type). Corrected count: **54 exactly**, matching cycle 12's own
book-by-book figure once `advanced_race_guide` is corrected from 4 to 6:
`beastiary` (14), `ultimate_magic` (9), `advanced_race_guide` (6),
`bestiary_6` (6), `core_rulebook` (6), `monster_codex` (5), `bestiary_5`
(4), `inner_sea_combat` (3), `horror_adventures` (1) = 54. Cross-checked
against the table's own grounded-key set (142 keys, confirmed via source
inspection, not assumed) — zero overlap, confirming none of the 54 were
already grounded under a different key.

## Work this cycle

**48 of 54** were sourced from their own printed "Starting Statistics"
line via `aonprd.com/DruidCompanions.aspx?ItemName=<species>` — the same
per-species fetch method cycles 6-12 used, one fetch per species, each
response independently confirming its own source book/page. Every one of
the 48 agreed with the corpus's own `natural_armor` token, **except one**
(see "The `dolphin_orca` finding" below).

**6 of 54** have no independent printed stat block and were derived
instead from an already-grounded table entry plus the source's own
explicit textual rule, each cross-checked against the corpus's own
`natural_armor` token exactly like every other entry:

- `carnivorous_flower`/`crawling_vine`/`puffball`/`sapling_treant`
  (Advanced Race Guide p.26): the Treesinger elf druid archetype's own
  plant companions. `aonprd.com`'s "Plant" category index only tracks
  Ultimate Wilderness's 8 plant companions, not these — found via a
  Paizo-blog search snippet and independently confirmed via
  `d20pfsrd.com`'s own Treesinger archetype page, both agreeing verbatim
  on all four species' Str/Con/natural-armor.
- `brute_steed_camel`/`brute_steed_horse` (Advanced Race Guide p.56): the
  Fell Rider hobgoblin cavalier archetype's own class feature text, found
  via search and confirmed on `d20pfsrd.com`: *"A fell rider's mount is
  unusually large and fierce. It gains a +2 bonus to Strength, but takes a
  –2 penalty to Dexterity. This ability otherwise works like the
  cavalier's mount ability."* Not an independent stat block — a modifier
  on the already-grounded `camel`/`horse` entries (this table doesn't
  track Dex, so only the +2 Str matters): Camel Str 18+2=20, Con unchanged
  14; Horse Str 16+2=18, Con unchanged 15. Both derivations independently
  confirmed: the corpus's own `natural_armor` token for both (1 for
  camel-based, 4 for horse-based) matches the unmodified `camel`/`horse`
  entries exactly, since Fell Rider's own ability text never touches
  natural armor.
- `devolved_humanoid` (Horror Adventures p.50): the Devolutionist druid
  archetype's own nature-bond text, found via search: *"Use the stats for
  an ape animal companion."* Reuses the already-grounded `ape` entry (Str
  13, Con 10, +1 natural armor) directly. Independently confirmed: the
  corpus's own `natural_armor` token (1) matches `ape`'s own value
  exactly.

All 54 filenames were independently confirmed to exist under their
respective `data/corpus/<book>/companion/` directories (via the `§17a`
re-derivation above) before being added. No delta was backed out anywhere
in this cycle's diff — every Str/Con value is the source's own printed
(or, for the 6 derived entries, once-removed) total, stored directly,
matching cycle 9's corrected methodology throughout.

Table: **142 → 196** (142 + 54). The full `RACETYPE:Companion` base-race
population is now **fully closed** — 0 of 196 remain ungrounded, in any
book.

### The `dolphin_orca` finding

`companion_dolphin_orca.json`'s corpus `natural_armor` field is 6
(`BONUS:VAR|AC_Natural_Armor|6|TYPE=Base`). Two independent sources
(`aonprd.com`'s own "Orca" companion page, re-fetched twice for
confidence, and a corroborating web search citing the same printed text)
agree the printed Starting Statistics natural armor is **+1**, not +6.
Traced directly against the pinned PCGen oracle rather than left as an
unexplained disagreement: `git show HEAD:data/pathfinder/paizo/
roleplaying_game/bestiary/b1_races_companion.lst` (line 16, the
`Companion (Dolphin (Orca))` record) really does carry `BONUS:VAR|
AC_Natural_Armor|6|TYPE=Base` — this is PCGen's own third-party data, not
a repo ingestion error. But the SAME file's own standalone-monster
`Dolphin (Orca)` race entry (`b1_races.lst`, `RACETYPE:Animal`,
`MONSTERCLASS:Animal:9`, a completely different Huge-sized wild-creature
record) carries the identical `AC_Natural_Armor|6` value — strong evidence
PCGen's companion entry was copy-pasted from the standalone monster's stat
block rather than authored from the companion-specific Medium starting
statistics. Per `decisions.md §1a` (grounded by measurement, not by
whichever source is more convenient), the printed total (1) is what is
stored in the table; the corpus/oracle's disagreeing token is documented
in the module doc rather than silently overridden or silently trusted.
This is the same class of finding the brief names — "reading the oracle
disproves our code as often as it confirms it" — applied to data this
time rather than restriction logic.

## Test evidence

```
cargo test --locked -p codex --lib companion_base_stat_table   # 15 passed, 0 failed (same count: +2 new — griffon positive test, renamed population test — net even against the removed old griffon-refusal assertion set, which moved to a new function name rather than being deleted)
cargo test --locked -p codex --lib pilot_compute::               # 968 passed, 0 failed (was 965, +3)
cargo test --locked -p codex --lib companion                    # 125 passed, 0 failed (was 124, +1)
```

`apps/desktop/src-tauri` (separate cargo workspace) re-run per the
brief's own instruction. **First run FAILED**: 547 passed, 1 failed —
`character_hub::tests::a_druid_who_selects_gulper_plant_grounds_gulper_
plant_not_wolf_at_character_creation_altitude` asserted that selecting
`companion_species: "griffon"` must fall back to Wolf because griffon had
no verified table row. That assumption is now wrong — griffon is grounded.
This is exactly failure mode #2 the brief names: a pinned test encoding an
assumption this cycle's own work invalidated. **Corrected, not deleted**:
the fallback-example slug moved to `not_a_real_companion_species` (never a
real PF1 species, matching the same correction made to
`companion_base_stat_table.rs`'s own refusal test), and a new positive
assertion block added proving `griffon` now grounds its own real stat
block through the actual `CreateCharacterRequest` → `compose_character_
input` → `build_pilot_headless_receipt` path, not merely through the
table directly. Re-run after the fix: **548 passed, 0 failed** (81.57s) —
exactly matching cycle 12's own exit state (test count unchanged: the
correction extended an existing test function rather than adding a new
one).

## PI / audit

- Own-diff (`git diff --unified=0 HEAD` over each touched file): `grep
  -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — zero hits
  (`OK_NO_BUNDLE_TAGS`).
- Same scope, `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|
  todo|fixme|hack)\b'` — zero hits (`OK_NO_TOKENS`).
- `pi_scrub.normalized_term_hits()` (imported from `scripts/pi_scrub.py`,
  never copied) over the own-diff of `companion_base_stat_table.rs`,
  `character_hub.rs`, and `kanban.md` — zero hits in all three, first
  pass clean.
- No `data/corpus/` write this cycle.

## Territory

`git status --porcelain` confirmed clean before every write and listed
only the four files named above afterward. `kanban.md` row parsing
verified before and after the edit: 21 data rows (`^\| [0-9]+ \|`), 21
unique row ids (1-21), `git diff --stat` shows exactly 1 line changed (1
insertion, 1 deletion — the row grows in place, still one physical line).
Rows 11 and 15 left untouched (not present in the diff). No other row
touched.

## Row 20 closure determination

Row 20's own title names three original items (from the `row19-cycle4`
receipt this row was created to close): (a) a 43-family class-chassis
reach gap, (b) the companion formula-scaled residual, (c) 30 delta-row
companions needing a creature-template engine.

- **Item (a)**: closed at cycle 5 — "all 61 of 61 conventional classes now
  resolve a REAL base-attack-bonus/save chassis at character-creation
  time" (cycle 5's own kanban text, confirmed by reading it directly this
  cycle rather than trusting the summary).
- **Items (b)/(c)**: cycles 3-4 re-investigated and reframed them as one
  problem — "the missing corpus input is the hand-authored companion
  base-stat-block build" — which is exactly what cycles 5-13's
  `companion_base_stat_table.rs` work has been. That population (196
  `RACETYPE:Companion` base-race records) is now grounded at **196/196,
  0 remaining**, closing (b)/(c) as reframed.
- **The separate "4-record `mod_only`" finding** from cycles 1-2 (a
  different file, `companion_pool_catalog.rs` — companion ABILITY
  descriptions, not base ability SCORES) does not appear again in the row's
  own text after cycle 2; cycle 12's own "Not `complete`" reasoning — the
  most recent, most-informed statement of the row's residual before this
  cycle — named only the companion base-stat-block gap, not this. Read
  directly this cycle: it is a currently-enforced, deliberate architectural
  exclusion backed by a live passing test
  (`a_mod_only_delta_row_is_refused_even_though_it_renders_clean`), not an
  abandoned TODO. Flagged above as a discovery-forward (its own reasoning
  echoes a `§27b`-inadmissible shape) rather than silently treated as
  either closed or blocking.

Given (a) confirmed closed, (b)/(c) now closed by this cycle's own work,
and the one adjacent finding read as a settled design decision (not
"real, sized, unbuilt work" in the sense every prior cycle's own "Not
`complete`" line used) — **row 20 is set to `complete`** in `kanban.md`
this cycle. **Cycles remaining: 0** — this closes the card.
