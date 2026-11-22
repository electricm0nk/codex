# Cycle row20-cycle8 — Epic 10 (`epic-10-reference-library-residual-reach`) / Row 20

- **Card ID:** `epic-10-reference-library-residual-reach`
- **Files touched:**
  - `src/rules_core/pilot_compute/companion_base_stat_table.rs` (23 new table entries across the
    `Aquatic`/`PlantCompanion`/`AnimalCompanionPrimate` buckets, module-doc cycle-8 addendum, two
    updated/renamed tests, one new positive test, extended `companion_display_name` coverage
    list).
  - `docs/release/SD-32-compute-library-and-cause-closure/{kanban.md,progress.md}` (this cycle's
    own row 20 entry).
  - This receipt.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own diff, `git diff --unified=0 HEAD` over the
  one touched source file, `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — zero
  hits).
- **Wired-integration audit result:** `OK_NO_TOKENS` (`grep -nE '\b(STUB|MOCK|placeholder|not yet
  implemented|todo|fixme|hack)\b'` over the same own-diff scope — zero hits). Real, tested table
  entries reachable through the already-wired `ground_selected_companion_or_default` dispatch
  point cycle 7 closed — no new stub, no new dead code (the table's consumer already existed).
- **Acceptance criterion:** Epic 10, row 20 — companion base-ability-score table residual: close
  or precisely size, per species, every `RACETYPE:Companion` corpus record with no verified base
  vector, per `decisions.md §27b` ("EVERYTHING", no carve-outs) and `§1a` (refuse rather than
  fabricate).
- **Corpus SHA:** oracle bootstrapped fresh this worktree (slot was empty), pinned at
  `7f818006e371188e5717fd18d74d18a420747fc6` — the same commit prior cycles used, confirmed via
  `scripts/fetch-pcgen-oracle.sh --dest $PCGEN_REPO_DIR` (its own stdout printed `OK
  7f818006e371188e5717fd18d74d18a420747fc6`).
- **Status:** `in-progress` (NOT `complete` — 144 of 196 companion species remain real, sized,
  unbuilt work: 142 untagged records plus 2 named dinosaur refusals).
- **Notes:** see body below for the `§17a` population correction and the `§17` Java re-check.
- **Discovery forwards:** none new this cycle.
- **Next-cycle plan:** the 142 untagged `RACESUBTYPE`-less records are the largest remaining
  residual and have no established per-bucket verification shortcut (no shared `RACESUBTYPE:` tag
  to group by); a future cycle should either grind them in sourced batches (same two-independent-
  source-plus-corpus-tiebreaker method this cycle and cycles 6-7 used) or find a faster
  verification instrument before committing to hand-authoring all 142 one at a time.

## Starting state (verified, not assumed)

`git rev-parse HEAD` on entry equalled `1bb523773d` (the SD-31 PR #374 merge commit), the same
stale-lineage footgun most prior cycles in this row hit. Recovered: `git reset --hard $PIN`
(`04ead3b5da169f3885ed2f5db2c1a8bc66c72a13`), re-verified `git merge-base --is-ancestor "$PIN"
HEAD`. `git fetch origin tranche/12` found `origin/tranche/12`'s own tip already equal to `$PIN`
(cycle 7's own commit) — no rebase needed, no sibling collision.

Oracle slot was empty (fresh worktree, git-ignored). `scripts/fetch-pcgen-oracle.sh --dest
$PCGEN_REPO_DIR` populated it at `7f818006e371188e5717fd18d74d18a420747fc6`.

## `§17a`: the population handed to this cycle was re-derived, not trusted

The brief's own figures ("184 of 213 species remain... `Aquatic` 13, `PlantCompanion` 7,
`AnimalCompanionPrimate` 4, `ConstructCompanion` 3, 154 untagged") were re-derived directly
against `data/corpus/*/companion/*.json` rather than carried forward, per `§17a`'s own standing
instruction ("validate an instrument, and re-derive every figure you are handed"). Filtering to
records whose `data.monster_class` field starts with `"Companion"` — the real per-record signal
the companion mechanic uses, since some genuine companion races (e.g. Inner Sea Combat's own
Hippocampus) carry `race_type: "Magical Beast"` in the ingested JSON despite a real
`MONSTERCLASS:Companion:2` token — and excluding the separate "Companion Advancement (...)"
ability records (`monster_class: None`, a different record type entirely, sharing only the
`companion_*` filename prefix, not a base-race entry) finds:

```
python3 -c "
import json, glob
recs = []
for f in glob.glob('data/corpus/*/companion/*.json'):
    d = json.load(open(f))['data']
    name = d.get('name') or ''
    mc = d.get('monster_class') or ''
    if mc.startswith('Companion') and not name.startswith('Companion Advancement'):
        recs.append(d)
print(len(recs))
"
# -> 196
```

Breaking that 196 down by `race_subtype`: **144 untagged** (`None`), **28**
`AnimalCompanionDinosaur` (unchanged from cycles 6-7), **12 `Aquatic`** (not 13 — the prior
count double-counted `Familiar`-racetype "Tiny Named Animal" records that merely share the
`Aquatic` `RACESUBTYPE:` tag but carry `MONSTERCLASS:Animal:1`/`RACETYPE:Animal`, e.g.
`ultimate_wilderness/companion/lamprey.json`), **8 `PlantCompanion`** (7 not already grounded by
`gulper_plant`), **4 `AnimalCompanionPrimate`**. Sum: 144+28+12+8+4 = 196, matching the
independently-derived total exactly.

**`ConstructCompanion` does not exist in this corpus at all.** `grep -rl "ConstructCompanion"
data/corpus/` returns zero hits. The 3 raw `RACESUBTYPE:ConstructCompanion` records the prior
brief cited (`grep -rlh "RACESUBTYPE:ConstructCompanion" $PCGEN_REPO_DIR/data`) live in
`data/pathfinder/ascension_games/path_of_iron/poi_races_companion.lst` on the pinned oracle — a
third-party (Ascension Games) supplement `data/corpus/` has never ingested (`ls data/corpus/ |
grep -i iron` finds nothing). That bucket is **out of** the 196-record population this table
targets, not merely unverified within it. Ingesting a wholly new book is separate, much larger
scope than this row's own per-species base-stat-table work (`docs/governance/book-ingestion-
playbook.md`'s own per-book cycle, not a row-20-cycle-sized unit) — named precisely here rather
than silently folded into or silently dropped from this row's own residual count.

## Closed all three remaining tagged buckets: 23 new entries

Same two-independent-source-plus-corpus-tiebreaker method cycles 6-7 established: aonprd.com's own
"Starting Statistics" block (fetched via `WebFetch`/`WebSearch` against
`aonprd.com/DruidCompanions.aspx?ItemName=...`), cross-checked against a second independent source
where readily reachable (d20pfsrd for Octopus — exact match, Str 12/Dex 17/Con 14/Int 2/Wis 12/Cha
3 both sides — and this module's own cycle 6 doc comment for `hunting_cactus`, already externally
verified there as a `§17a` correction worked example but never actually added to the table), plus
the corpus's own `BONUS:STAT` delta (read directly from each record's `raw_tokens`) as the numeric
tiebreaker. Natural armor, read directly from the corpus's own `AC_Natural_Armor` token, matched
AoN's own printed "+n natural armor" line for **all 23 of 23** — the same 100% agreement rate
cycles 6-7 found, now confirmed across 46 species combined (23 dinosaurs + 23 this cycle).

| species | bucket | AoN Str/Con | corpus delta | base Str/Con | natural armor |
|---|---|---|---|---|---|
| Eel (Giant Moray) | Aquatic | 14/12 | STR+4 CON+2 | 10/10 | 5 |
| Octopus | Aquatic | 12/14 | STR+2 CON+4 | 10/10 | 1 |
| Squid | Aquatic | 14/11 | STR+4 (no CON) | 10/11 | 1 |
| Cameroceras | Aquatic | 14/11 | STR+4 (no CON) | 10/11 | 1 |
| Dunkleosteus | Aquatic | 14/10 | STR+4 (no CON) | 10/10 | 4 |
| Shark | Aquatic | 13/15 | STR+2 CON+4 | 11/11 | 4 |
| Hippocampus | Aquatic | 16/15 | STR+6 CON+4 | 10/11 | 4 |
| Crab (Giant) | Aquatic | 13/13 | STR+2 CON+2 | 11/11 | 5 |
| Anglerfish | Aquatic | 13/12 | STR+2 CON+2 | 11/10 | 1 |
| Armorfish | Aquatic | 13/15 | STR+2 CON+4 | 11/11 | 6 |
| Hammerhead Shark | Aquatic | 13/12 | STR+2 CON+2 | 11/10 | 3 |
| Squid (Giant) | Aquatic | 12/13 | STR+2 CON+2 | 10/11 | 1 |
| Corpse-Eater Fungus | PlantCompanion | 14/12 | STR+4 CON+2 | 10/10 | 2 |
| Creeping Puffball | PlantCompanion | 12/14 | STR+2 CON+4 | 10/10 | 1 |
| Hunting Cactus | PlantCompanion | 14/17 | STR+4 CON+6 | 10/11 | 3 |
| Rash Creeper | PlantCompanion | 10/13 | CON+2 (no STR) | 10/11 | 1 |
| Slithering Sundew | PlantCompanion | 14/13 | STR+4 CON+2 | 10/11 | 1 |
| Snapping Flytrap | PlantCompanion | 12/14 | STR+2 CON+4 | 10/10 | 2 |
| Sniper Cactus | PlantCompanion | 10/14 | CON+4 (no STR) | 10/10 | 2 |
| Ape | AnimalCompanionPrimate | 13/10 | STR+2 (no CON) | 11/10 | 1 |
| Chimpanzee | AnimalCompanionPrimate | 13/12 | STR+2 CON+2 | 11/10 | 1 |
| Devil Monkey | AnimalCompanionPrimate | 15/8 | STR+4 CON-2 | 11/10 | 3 |
| Megaprimatus | AnimalCompanionPrimate | 13/10 | STR+2 (no CON) | 11/10 | 1 |

Note on parsing the corpus's own combined `BONUS:STAT` tokens: several records apply one delta to
two abilities in a single token (e.g. `companion_squid.json`'s own `BONUS:STAT|STR,DEX|4`) — a
first-draft extraction script that split naively on `|` mis-keyed these as a single combined key
(`"STR,DEX"`) rather than crediting each named ability its own delta, silently zeroing out the
STR delta for Squid, Octopus, and several others. Caught before any table entry was written by
cross-checking the very first species (Cameroceras) against its own printed total by hand and
finding the script's computed base didn't match; fixed by splitting each token's ability-list on
`,` and crediting the delta to every named key, re-verified against all 23 species matching their
printed totals exactly afterward.

Table: 29 → 52 entries. `only_fifty_two_of_the_corpus_s_196_racetype_companion_records_have_a_
base_stat_entry` (renamed from cycle 7's `only_twenty_nine...213_racetype...`, its own doc comment
explaining the population correction) and a new
`the_twenty_three_cycle_eight_aquatic_plant_and_primate_companions_ground_their_own_verified_base_
scores` pin all 23 new entries' exact base scores and natural armor.
`companion_display_name_matches_every_table_entrys_documented_name` extended with all 23 new
slugs.

## `§17`, re-asked against PCGen's own Java before grinding the untagged 142

Per the brief's own instruction ("check whether PCGen's own Java computes any part of it"), read
the pinned oracle's own git objects directly (`git -C $PCGEN_REPO_DIR ls-tree -r --name-only
HEAD`, `git -C $PCGEN_REPO_DIR show HEAD:<path>` — no checkout, no cone widening):

- `git -C $PCGEN_REPO_DIR ls-tree -r --name-only HEAD | grep -i companion | grep '\.java$'` finds
  30 files, every one of them handling the companion-**MOD** linking mechanic (which class's
  companion follows which master's own level/HD/skills —
  `code/src/java/pcgen/core/character/CompanionMod.java`,
  `code/src/java/plugin/lsttokens/companionmod/{CopymasterbabToken,CopymasterhdToken,
  UsemasterskillToken}.java`, `code/src/java/plugin/lsttokens/CompanionListLst.java`, and their
  siblings) — none compute an ability score.
- `git -C $PCGEN_REPO_DIR show HEAD:data/pathfinder/paizo/roleplaying_game/core_rulebook/
  cr_classes_companion.lst`'s own `CLASS:Companion` definition (the shared progression every
  companion species reads regardless of species) carries `BONUS:COMBAT|BASEAB|...`,
  `BONUS:SAVE|BASE.Fortitude,BASE.Reflex|...`, `BONUS:VAR|BaseClassSkillPts|...` — real formula
  content for attack, saves, and skill points — but **no ability-score-granting token of any
  kind**.
- Re-reading `cr_races_companion.lst`'s own RACE lines directly (the same file cycle 6 already
  checked, re-confirmed here by an independent read) shows every companion race carries only
  `BONUS:STAT|<ability>|<delta>` — a bonus, never an absolute score.

This is a **third independent confirmation** (raw `.lst` grep in cycle 6, ingested-JSON delta
backing-out in cycles 5-8, and now the class definition's own complete token list in cycle 8) that
the base ability-score vector is genuinely fixed, per-species, printed prose with no PCGen-side
derivation shortcut anywhere in the engine's own source. `§17`'s own question is answered the same
way a third time: it must be hand-authored. This cycle did so honestly for the 23 species it had
room to verify to the two-independent-source bar, and named the remaining 142 as real, sized,
unbuilt work rather than guessing at them.

## Test evidence

```
cargo test --locked -p codex --lib companion_base_stat_table   # 11 passed, 0 failed (was 9)
cargo test --locked -p codex --lib pilot_compute::               # 950 passed, 0 failed (was 948)
cargo test --locked -p codex --lib companion                    # 121 passed, 0 failed (was 120)
```

No `apps/desktop/src-tauri` source file was touched this cycle (only
`src/rules_core/pilot_compute/companion_base_stat_table.rs`, a `codex`-lib-only compute module,
consumed through the dispatch point cycle 7 already wired). Full `apps/desktop/src-tauri` suite
re-run anyway per the brief's own instruction: `cargo test --locked --bin codex-desktop` ->
**548 passed, 0 failed** (78.46s) — unchanged from cycle 7's own 548/0 exit state, confirming the
table growth touches no desktop-crate compute path, exactly as expected for a `codex`-lib-only
change.

## PI / audit

- Own-diff (`git diff --unified=0 HEAD` over the one touched file):
  `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — zero hits (`OK_NO_BUNDLE_TAGS`).
- Same scope, `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` — zero
  hits (`OK_NO_TOKENS`).
- `pi_scrub.normalized_term_hits()` (imported from `scripts/pi_scrub.py`, never copied) over the
  own-diff — zero hits, first pass clean.
- No `data/corpus/` write this cycle (the touched module reads corpus data read-only at runtime;
  the 23 new table entries are hand-authored Rust constants).

## Territory

`git status --porcelain` confirmed clean before every write and listed only the one intended
source file after (plus `kanban.md`/`progress.md`/this receipt). `kanban.md` row parsing verified
before and after the edit with a backtick-aware parser: 23 pipe-lines (21 data rows + header +
separator), 21 unique row ids, 0 duplicates, row 20's own cells split to 9 raw segments (7 logical
columns), unchanged shape. Rows 11 and 15 left untouched.

## Next-cycle plan

1. **The 142 untagged (`RACESUBTYPE`-less) companion records** are the largest remaining
   residual and the only bucket left with no shared tag to group verification work by species
   family. A future cycle should grind them in sourced batches (same two-independent-source-plus-
   corpus-tiebreaker method this cycle and cycles 6-7 used) — there is no shortcut this cycle
   found; `§17`'s own question was re-asked and re-confirmed closed (hand-authoring is the only
   path) before concluding this.
2. **`pachycephalosaurus`/`ornithomimosaur`**: still refuse, unchanged from cycle 7 — revisit if a
   future cycle finds a source neither cycle 7 nor this one could reach.
3. Row 20 stays `in-progress` under `decisions.md §10` until the full 196-record companion
   population is grounded or the residual is further resized with evidence.
