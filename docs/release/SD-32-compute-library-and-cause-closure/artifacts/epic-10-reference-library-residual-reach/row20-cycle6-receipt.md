# Cycle row20-cycle6 — Epic 10 (`epic-10-reference-library-residual-reach`) / Row 20

- **Card ID:** `epic-10-reference-library-residual-reach`
- **Files touched:**
  - `apps/desktop/src-tauri/src/character_hub.rs` (one new test,
    `all_61_generic_classes_reach_a_real_chassis_at_character_creation_altitude`).
  - `src/rules_core/pilot_compute/companion_base_stat_table.rs` (nine new table entries,
    module doc addendum, one updated test, one new test).
  - `docs/release/SD-32-compute-library-and-cause-closure/{kanban.md,progress.md}` (this
    cycle's own row 20 entry).
  - This receipt.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own diff, `git diff --unified=0 HEAD --
  apps/desktop/src-tauri/src/character_hub.rs
  src/rules_core/pilot_compute/companion_base_stat_table.rs | grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — zero hits).
- **Wired-integration audit result:** `OK_NO_TOKENS` (`grep -cE 'todo!|unimplemented!|TODO
  stub|FIXME stub'` over the same own-diff scope — zero hits). One real, honestly-named
  wiring gap was FOUND, not created: `ground_companion_stat_block` has no live caller
  anywhere in the crate (see Item (b)/(c) below) — this is a pre-existing gap from cycle 5,
  surfaced and named here, not a stub this cycle introduced.
- **PI scrub:** `pi_scrub.normalized_term_hits()` (imported, not copied) over the own-diff —
  zero hits.
- **Corpus SHA:** oracle bootstrapped fresh this worktree (slot was empty), pinned at
  `7f818006e371188e5717fd18d74d18a420747fc6` — the same commit cycles 2/3/4/5 used, confirmed
  via `scripts/verify.sh --only preflight-oracle` (FAIL before bootstrap, PASS after).
- **Status:** `in-progress` (NOT `complete` — item (a), the picker, is now verified closed at
  the real altitude; item (b)/(c), 201 of 213 companion species plus a newly-named
  species-selection wiring gap, remains real, sized, unbuilt work).

## Starting state (verified, not assumed)

`git rev-parse HEAD` on entry was `1bb523773d` (the SD-31 PR #374 merge commit) — the same
stale-lineage footgun every prior cycle in this row hit. Recovered: `git reset --hard $PIN`,
confirmed `BASE_OK`. `git fetch origin tranche/12` found `origin/tranche/12`'s own tip already
equal to `$PIN` (row 20 cycle 5's own commit, `9f2fa984da`) — no rebase needed, no other lane
had pushed since. `git log origin/tranche/12 -- apps/desktop/src-tauri/src/{character_hub.rs,
pf1_adapter.rs,class_catalog_generic.rs} src/rules_core/pilot_compute/{companion_base_stat_
table.rs,generic_class_chassis.rs}` confirmed no activity past `$PIN` in any of these five
files before this cycle started — no sibling collision risk.

Oracle slot was empty (fresh worktree, git-ignored). `scripts/fetch-pcgen-oracle.sh --dest
$PCGEN_REPO_DIR` populated it at `7f818006e371188e5717fd18d74d18a420747fc6`, re-confirmed via
`scripts/verify.sh --only preflight-oracle` (PASS after).

## Item 2 (picker): re-derived cycle 5's own closure claim at the real altitude

The brief handed to this cycle asserted "neither wired the character-creation-time picker" as
its own premise, directly contradicting cycle 5's own receipt, which claimed the picker was
"fully closed for all 61 classes" via `character_hub.rs`'s already-free-form `class_id` string
dispatching through `compute_class_chassis` -> `generic_class_chassis::resolve`. Per `§17a`
("re-derive every figure you are handed, including the ones in this brief"), re-derived rather
than trusted either claim.

Read `character_hub.rs` directly: confirmed `CreateCharacterRequest.class_id: String` is real
(no `ClassId` enum anywhere in that file), and confirmed `compute_class_chassis` IS invoked
from the real production compute path (`compute_pilot_base_chassis`, called from
`compose_character_input` -> `build_pilot_headless_receipt`, the same path
`character_hub.rs`'s own `create_character_at_root` uses to decide whether a character reaches
`Computed`). But cycle 5's own proof of "all 61 resolve" lived entirely inside
`generic_class_chassis.rs`'s own unit tests — a crate-internal function exercised in isolation,
never through the real character-creation request/response path `character_hub.rs` itself
owns. A drift between the two (a request-validation layer rejecting an id before it reaches
`compute_class_chassis`, for instance) would have been invisible to cycle 5's own tests.

Added `all_61_generic_classes_reach_a_real_chassis_at_character_creation_altitude`
(`character_hub.rs`'s own test module): reads the 60-of-61-class population from
`class_catalog_generic::load_generic_class_progressions` (asserting its own `unresolved` list
names only the known Demoniac gap), adds Demoniac back in by name (its real corpus record name,
confirmed via direct read of `data/corpus/book_of_the_damned_volume_2/class/demoniac.json`),
and for each of the 61 calls the existing `claim_blocking_diagnostic_ids("race:human",
"class:<slug>", 1)` helper (already used by the fighter/paladin/ranger golden-path tests
directly above it) — asserting `class_chassis.unsupported` is never present. **Result: passes
for all 61.** Cycle 5's own claim holds at the altitude that matters; the brief's own premise
("still not wired") is retracted by this cycle as stale, not re-done.

## Item 1/2/3 (companion table): `§17` re-derivation, nine species closed, one gap named

### Re-derived the corpus-derivation question against the RAW oracle, independently

Cycle 5 already refuted the shared-category-base hypothesis (Gulper Plant vs Hunting Cactus,
same `RACESUBTYPE:PlantCompanion`, different backed-out base vectors) by reading the ingested
JSON. This cycle's own brief asked the `§17` question again: can the vector be derived from
corpus data (`BONUS:STAT` deltas plus a per-`RACETYPE`/size baseline) rather than hand-typed?
Checked by an INDEPENDENT method — the raw pinned PCGen oracle `.lst` source line itself, not
the ingested JSON shape:

```
grep -n "Gulper Plant" \
  $PCGEN_REPO_DIR/data/pathfinder/paizo/roleplaying_game/ultimate_wilderness/uw_races_companion.lst
```

The full RACE line for `Companion (Gulper Plant)` carries `BONUS:STAT|STR|2`,
`BONUS:STAT|CON|2`, `BONUS:STAT|INT|-10`, `BONUS:STAT|CHA|-8`, `BONUS:VAR|AC_Natural_Armor|1|
TYPE=Base`, and no other stat-shaped token — **no `STR:`/`DEX:`/`CON:`/etc absolute field
anywhere on the line.** There is no "baseline" value encoded in PCGen's own source for this
RACETYPE to fall back on; the base score is Java/table-computed upstream and genuinely absent
from every corpus form this engine reads. This confirms cycle 5's finding by an independent
check rather than merely trusting it — no corpus-only derivation shortcut exists. Hand-authoring
remains the only path.

### Nine `AnimalCompanionDinosaur` species added, each independently verified

Per the brief's own instruction ("hand-author by category first, largest first, reporting the
count closed"), picked `AnimalCompanionDinosaur` — 31 records by the oracle's own raw count, 28
in this repo's ingested `data/corpus/*/companion/*.json` (re-derived via a direct Python walk of
every `companion/*.json` file's own `RACETYPE`/`RACESUBTYPE` tokens — the discrepancy from 31 is
itself worth naming for a future cycle, not chased down here). Verified and added 9 of the 28,
each via AoN's own "Starting Statistics" block (re-fetched or re-searched independently per
species — two of the nine, Velociraptor and the Brachiosaurus/Amargasaurus pair, were
specifically cross-checked with a SECOND independent search after the first fetch, since one
early general-purpose fetch of a different page returned an internally-inconsistent Deinonychus
figure that a direct per-species fetch then corrected) plus the corpus's own `BONUS:STAT` delta
as the numeric tiebreaker (printed total minus corpus delta backs out the base):

| species | AoN printed Str/Con | corpus delta | backed-out base Str/Con | natural armor |
|---|---|---|---|---|
| Allosaurus | 14 / 10 | STR+4, (no CON) | 10 / 10 | 4 (direct) |
| Ankylosaurus | 10 / 9 | (no STR), CON,CHA−2 | 10 / 11 | 9 (direct) |
| Pteranodon | 8 / 10 | STR−2, (no CON) | 10 / 10 | 0 (direct) |
| Deinonychus | 11 / 17 | (no STR), CON+6 | 11 / 11 | 1 (direct) |
| Velociraptor | 11 / 17 (2nd source confirmed identical to Deinonychus) | byte-identical delta to Deinonychus | 11 / 11 | 1 (direct) |
| Triceratops | 10 / 11 | (no STR/CON delta) | 10 / 11 | 6 (direct) |
| Tyrannosaurus | 14 / 10 (same printed block as Allosaurus) | STR+4, (no CON) | 10 / 10 | 4 (direct) |
| Amargasaurus | 11 / 9 (2 independent fetches agree) | (no STR), CON−2 | 11 / 11 | 3 (direct) |
| Brachiosaurus | 13 / 11 (2 independent fetches agree) | STR+2, (no CON) | 11 / 11 | 3 (direct) |

**New simplification, useful for future batches**: the corpus's own `BONUS:VAR|AC_Natural_
Armor|n|TYPE=Base` token IS the base natural armor value directly, never a delta needing
backing-out — confirmed by agreement with AoN's own printed "+n natural armor" line in all 9
cases above, so future species need external verification only for Strength/Constitution, not
natural armor.

`companion_base_stat_table.rs`'s table grew from 3 to 12 entries. Added
`the_nine_dinosaur_companions_ground_their_own_verified_base_scores` (pins each species' base
Str/Con/natural-armor/hit-die through `ground_companion_stat_block` itself, not just the
private table) and updated the population-refusal test from "three of 213" to "twelve of 213".

### Named, not hidden: `ground_companion_stat_block` has no live caller at all

While tracing where a real character would reach this table, `grep -rn
"ground_companion_stat_block(" src/rules_core/pilot_compute/*.rs` (excluding the module's own
file) returned **zero matches** — confirmed independently by `cargo build`'s own dead-code
warnings on both `ground_companion_stat_block` and `CompanionBaseStats`. Every existing
companion-bearing class dispatch (`grep -n ground_wolf_companion_stat_block\( src/rules_core/
pilot_compute/mod.rs`) shows four real production call sites, all hardcoded to Wolf (Druid/
Hunter) or Horse (Cavalier) — no class in this engine offers a character-creation-time CHOICE
among companion species at all. There is consequently no dispatch point today for the generic,
species-parameterized table this cycle (and cycle 5) built to be wired INTO. Populating the
table with verified data is necessary but not sufficient for that data to reach a real
character; the real remaining work is a separate, cross-file wiring project (a new
`CharacterInput` companion-species choice slot, a new `pf1_adapter.rs`/`character_hub.rs`
request field, a new compute call site dispatching on it) that this cycle's own scope — data
verification, not new UI/request-shape surface — does not cover. Named exactly here, per
`§16`/`§17`, rather than silently assumed solved by the table's own existence, or hidden behind
a passing test suite that never actually exercises the missing seam.

## Test evidence

```
cargo test --locked -p codex --lib companion_base_stat_table   # 7 passed, 0 failed
cargo test --locked -p codex --lib generic_class_chassis       # 5 passed, 0 failed (unchanged, re-confirmed)
```

Full `apps/desktop/src-tauri` suite re-run: `cargo test --locked --bin codex-desktop` ->
**547 passed, 0 failed** (77.50s) — cycle 5's own 546/0 baseline plus this cycle's one new
`character_hub.rs` test (the companion-table tests live in the `codex` lib crate and run
separately from the `codex-desktop` bin crate's own suite).

## PI / audit

- Own-diff: `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — zero hits
  (`OK_NO_BUNDLE_TAGS`).
- Same scope, `grep -cE 'todo!|unimplemented!|TODO stub|FIXME stub'` — zero hits
  (`OK_NO_TOKENS`).
- `pi_scrub.normalized_term_hits()` (imported from `scripts/pi_scrub.py`, never copied) over
  the own-diff — zero hits.
- No `data/corpus/` write this cycle (both touched modules read-only at runtime; the
  `companion_base_stat_table.rs` entries are hand-authored Rust constants, not corpus
  records), so the `declared_pi_shipping_audit` before/after requirement does not apply.

## Territory

`git status --porcelain` confirmed clean before every write, and confirmed to list only the two
touched files after. No `class_catalog_generic.rs`, `class_catalog.rs`, `companion_catalog.rs`,
`pf1_adapter.rs`, `formula_interpreter.rs`, or `mod.rs` write. `kanban.md` row parsing verified:
21 data rows + 1 header + 1 separator = 23 pipe-lines, 0 duplicate row IDs, row 20's own cells
split to exactly 7 with a backtick-aware parser (matching row 18 cycle 9's own precedent
method). Rows 11 and 15 left untouched at `in-progress`.

## Next-cycle plan

1. **Companion base-stat table**: 19 of the 28 ingested `AnimalCompanionDinosaur` records
   remain, then `Aquatic` (13), `PlantCompanion` (7 remaining), `AnimalCompanionPrimate` (4),
   `ConstructCompanion` (3) — repeat this cycle's own two-independent-source-plus-corpus-
   tiebreaker method, reusing the "natural armor is direct, only Str/Con need external
   verification" simplification found this cycle.
2. **Companion species-selection wiring** (new, named this cycle): a real
   character-creation-time choice among companion species for Druid/Hunter/Cavalier, threaded
   through `CharacterInput`/`pf1_adapter.rs`/`character_hub.rs` into the compute pipeline this
   cycle confirmed has no dispatch point for it today. Separate, cross-file, unbuilt.
3. Row 20 stays `in-progress` under `decisions.md §10` until the companion residual and its
   wiring gap close or are further resized with evidence. Item (a), the character-creation
   picker, is now verified closed at the real altitude and needs no further cycle work.
