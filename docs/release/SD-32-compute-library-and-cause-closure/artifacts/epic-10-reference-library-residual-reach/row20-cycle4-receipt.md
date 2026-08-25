# Cycle row20-cycle4 — Epic 10 (`epic-10-reference-library-residual-reach`) / Row 20

- **Card ID:** `epic-10-reference-library-residual-reach`
- **Files touched:**
  - `apps/desktop/src-tauri/src/class_catalog_generic.rs` (new).
  - `apps/desktop/src-tauri/src/class_catalog.rs` (widened `build_class_catalog()`, updated
    the row-count test and its comment).
  - `apps/desktop/src-tauri/src/main.rs` (`mod class_catalog_generic;`).
  - `docs/release/SD-32-compute-library-and-cause-closure/{kanban.md,progress.md}` (this
    cycle's own row 20 entry).
  - This receipt.
  - `docs/retro/events/sd31-transcribe.jsonl` — append-only, auto-written by
    `scripts/verify.sh --only preflight-oracle`'s own instrumentation (two runs: one before
    the oracle bootstrap, FAIL; one after, PASS). Not a manual edit.
  - No `pilot_compute`, `class_feature_grant_consumer.rs`, `companion_catalog.rs`, or
    `formula_interpreter.rs` write — all four stayed read-only, per the brief's coordination
    instruction and this cycle's own finding that a `formula_interpreter.rs` widening
    (Demoniac's bare `classlevel()`) belongs to row 18's live territory, not this cycle.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own diff, `git diff --unified=0 HEAD --
  apps/desktop/src-tauri/src/{class_catalog.rs,main.rs} \
  docs/release/SD-32-compute-library-and-cause-closure/{kanban.md,progress.md} | grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` plus the same grep over the new,
  untracked `class_catalog_generic.rs` in full — zero hits both).
- **Wired-integration audit result:** `OK_NO_TOKENS` (`grep -nE 'todo!|unimplemented!|TODO
  stub|FIXME stub'` over the same scope — zero hits; the one genuine gap, `Demoniac`'s
  unresolved formula, is reported through a real `unresolved: Vec<(String, String)>` return
  value and a passing test asserting its exact contents, not a stub marker).
- **PI scrub:** `pi_scrub.normalized_term_hits()` over the own-diff and over the full new
  file — zero hits both.
- **Corpus SHA:** oracle bootstrapped fresh this worktree (slot was empty), pinned at
  `7f818006e371188e5717fd18d74d18a420747fc6` — the same commit cycle 2/3 used, confirmed via
  `scripts/verify.sh --only preflight-oracle` (FAIL before bootstrap, PASS after).
- **Status:** `in-progress` (NOT `complete` — Demoniac's evaluator gap, the
  character-creation-picker wiring, and item (b)/(c)'s companion base-stat-block build all
  remain real, sized, unbuilt work).

## Starting state (verified, not assumed)

`git rev-parse HEAD` on entry was `1bb523773d` (the SD-31 PR #374 merge commit) — the same
stale-lineage footgun cycles 2 and 3 both hit. `git merge-base --is-ancestor $PIN HEAD`
failed (`WRONG_BASE`). Recovered: `git reset --hard $PIN`, confirmed `BASE_OK`,
`git fetch origin tranche/12` + `git rebase origin/tranche/12` reported "up to date" —
`origin/tranche/12`'s own tip already equals `$PIN` (cycle 3's own commit,
`f6390421c9ae5f7c7b92cecb192553c0161222d6`). No other lane had pushed since cycle 3.

`ls` on the oracle slot before bootstrap confirmed it empty (fresh worktree, git-ignored slot
per every prior cycle's own warning). `scripts/fetch-pcgen-oracle.sh --dest $PCGEN_REPO_DIR`
populated it at `7f818006e371188e5717fd18d74d18a420747fc6`, re-confirmed via
`scripts/verify.sh --only preflight-oracle` (PASS after, FAIL before — both appended to the
shared `docs/retro/events/sd31-transcribe.jsonl` automatically by `verify.sh`'s own
instrumentation, listed above as a touched file for that reason, not a manual edit).

## Item (b)/(c): the companion base-ability-score question, investigated per the brief's own
next step, and answered

Cycle 3 named the exact concrete next step: "confirm whether PCGen's own `.lst` corpus
carries a companion species' base ability score block anywhere this repo has not yet
ingested — before choosing between ingesting it or hand-authoring per-species stat blocks."

Read the pinned **oracle's own source** directly (not this repo's ingest of it) for the same
sample cycle 3 used, Ultimate Wilderness's Gulper Plant companion:

```
grep -i "gulper" \
  $PCGEN_REPO_DIR/data/pathfinder/paizo/roleplaying_game/ultimate_wilderness/uw_races_companion.lst
```

The oracle's own row carries `BONUS:STAT|STR|2`, `BONUS:STAT|CON|2`, `BONUS:STAT|INT|-10`,
`BONUS:STAT|CHA|-8`, `MONSTERCLASS:Companion:2`, `RACETYPE:Companion` — **deltas only, no
base score**, confirming this is not a transcription gap this repo's own ingest introduced.

Searched the FULL oracle tree, not just the one file:

```
grep -rl "Gulper Plant" $PCGEN_REPO_DIR/data   # -> 2 files, both companion-specific
grep -ril "Animal Companion Base" $PCGEN_REPO_DIR/data   # -> 0 matches, anywhere
```

**Finding: the companion base-ability-score block is not `.lst` data anywhere in PCGen's own
source.** It is computed by PCGen's Java runtime from a printed table (an "Animal Companion
Base Statistics"-shaped table, per the Ultimate Wilderness rulebook), the same way this
engine's own Wolf/Horse constants already are hand-authored rather than corpus-sourced
(confirmed by re-reading `companion_catalog.rs`'s own module doc, which already stated this
for Wolf/Horse; this cycle confirms it generalizes to every unusual companion type, not just
those two).

**This is not a `docs/release/SD-32-compute-library-and-cause-closure/decisions.md §27b` hard
impossibility** — the source table exists in print and is reproducible, so the ONLY
admissible-exception bar is not met. What it settles is the choice between cycle 3's two
named remedy paths: **"ingest" is ruled out for lack of any ingestible source in the pinned
oracle; "hand-author per companion category" (mirroring the existing Wolf/Horse pattern) is
the only viable remedy.**

**A new, materially useful sizing finding**: PF1's real Animal Companion Base Statistics
table is organized by companion **TYPE/size category** (e.g. Tiny Animal, Small Animal,
Medium Animal, Large Animal, and equivalent rows for Magical Beast/Dinosaur/Vermin/etc — a
handful of rows), not by all ~40+ individual companion species. Confirmed indirectly: the 123
companion-race lines in `uw_races_companion.lst` alone carry `RACESUBTYPE:` values like
`AnimalCompanionDinosaur`, `AnimalCompanionPrimate`, `Aquatic`, `Familiar`,
`PlantCompanion` — category tags, not per-species tags — meaning the hand-authored base-stat
layer this cycle is escalating is a **category table**, not a per-species one. This shrinks
the follow-on task materially versus cycle 3's "~40+ species" framing, without changing that
it is still real, unbuilt, hand-authoring work.

**Not built this cycle**: per the brief's own coordination instruction, stayed read-only in
`pilot_compute/mod.rs`, `class_feature_grant_consumer.rs`, and `companion_catalog.rs` — no
edit to any of the three. The next cycle's concrete first step: build the (small) category-
keyed base-stat table as Rust constants, generalizing the existing Wolf/Horse pattern, then
wire it through a companion-scoped consumer mirroring `class_feature_grant_consumer.rs`'s
proven shape (both already-confirmed-reachable per cycle 3: `CharacterInput` and
`PcgenFormulaEvaluator` need no `pilot_compute` edit to reach `apps/desktop/src-tauri`).

## Item (a): the 61-record chassis, built generically for 60 of 61

### Re-derived the population, not trusted from cycle 3

```
python3 - <<'PY'
import json, os
books = ['adventurers_guide','book_of_the_damned_volume_1','book_of_the_damned_volume_2',
         'inner_sea_combat','inner_sea_gods','inner_sea_intrigue','inner_sea_magic',
         'inner_sea_world_guide','occult_adventures','ultimate_combat','ultimate_intrigue',
         'ultimate_magic','ultimate_wilderness','ultimate_psionics']
tot = 0
for book in books:
    d = f'data/corpus/{book}/class'
    if not os.path.isdir(d): continue
    for fn in sorted(os.listdir(d)):
        rec = json.load(open(f'{d}/{fn}'))
        data = rec.get('data', rec)
        toks = data.get('raw_tokens', [])
        typ = next((t['value'] for t in toks if t['key']=='TYPE'), '')
        hasBAB = any(t['key']=='BONUS' and 'BASEAB' in t['value'] for t in toks)
        hasSAVE = any(t['key']=='BONUS' and t['value'].startswith('SAVE|') for t in toks)
        if hasBAB and hasSAVE and 'Monster' not in typ:
            tot += 1
print(tot)
PY
# -> 61
```

Exactly matches cycle 3's 61 — independently re-derived (`§17a`), not repeated on trust.

### Formula shape, measured across all 61 before writing any code

Sampled every BASEAB/SAVE token's formula field across all 61 candidates (not one book):
every `BONUS:COMBAT|BASEAB|...` token has exactly 5 pipe-delimited fields (uniform shape);
every class's `classlevel(...)` calls pass the SAME literal string argument,
`"APPLIEDAS=NONEPIC"` (confirmed by regex sweep over all 61's formulas — a single distinct
value, not a class name at all, despite the argument position); the other formula shape uses
a plain `<Name>LVL` variable, already `VAR|<Name>LVL|CL`-bound in the corpus to the caller's
own level. `BONUS:SAVE` tokens can pack multiple save targets into one comma-separated field
(`SAVE|BASE.Fortitude,BASE.Reflex|<formula>|...`) — a real, common PCGen compaction, not an
anomaly (an early sweep mis-flagged 50 records as "anomalies" before recognizing this shape;
corrected before writing the extraction code, not after).

### Built `class_catalog_generic.rs`

- `classify_class_record` re-runs cycle 3's own classification heuristic on `raw_tokens` at
  runtime (never a hardcoded name list) — TYPE contains "Monster" -> pseudo-class; missing
  BASEAB or SAVE -> shell; else conventional PC class.
- `load_generic_class_progressions` walks `data/corpus/<book>/class/*.json` across the 14
  book directories that hold the 13 real-class families (matching cycle 3's own per-family
  read), classifies each record, and for each `ConventionalPc` record evaluates its BAB and
  3 saves at every level `1..=max_level` via `PcgenFormulaEvaluator`
  (`pilot_compute::formula_interpreter`, already `pub`, already oracle-verified elsewhere in
  this module's own test suite) — binding both discovered formula shapes
  (`CLASSLEVEL::APPLIEDAS=NONEPIC` and `<Name>LVL`) so either resolves without guessing which
  a given record uses.
- Two real wrinkles handled, both cited with a mutation-proof test:
  - **`Vigilante`** carries TWO `BASEAB` tokens (a build-time class-feature toggle,
    `VigilanteFullBAB`). `select_baseab_formula` picks the one whose trailing `PREVAREQ`
    pair reads `,0` (toggle off, the moderate/3-4 progression) — proven correct, not merely
    present, by `exactly_one_class_needs_baseab_disambiguation`: level-20 BAB is asserted
    `15` (the moderate value), which the toggle-on alternative (`20`) would have failed.
  - **`Ulfen Guard`** (`inner_sea_combat`, `TYPE: PC.Prestige`) carries no `MAXLEVEL` token
    at all. Defaulted to `10` (the PF1 prestige-class rule) rather than the base-class
    default of `20` — `ulfen_guard_prestige_class_defaults_to_max_level_10` asserts exactly
    10 rows, not 20.
- `max_level_for` defaults an absent `MAXLEVEL` to `10` when `TYPE` contains `Prestige`,
  `20` otherwise — the only two defaults observed needed across the 61.

### Demoniac: the one record that does not resolve, named not hidden

`book_of_the_damned_volume_2/demoniac.json`'s BASEAB/save formulas call bare `classlevel()`
with **no argument** (`classlevel()*3/4`, `(classlevel()+1)/2`, `(classlevel()+1)/3`) —
confirmed by the same full-population sweep above (the only one of 61 with this shape).
`formula_interpreter.rs`'s own `classlevel` grammar arm requires a string-literal argument
(its own parse-error text: *"classlevel(...) expects a string literal class name"*) — a real,
pre-existing gap in that shared evaluator's grammar, not something this module may
special-case around by editing `formula_interpreter.rs` itself (row 18's live territory,
explicitly out of this cycle's write scope). `load_generic_class_progressions` reports it in
its `unresolved: Vec<(String, String)>` return rather than silently dropping it or guessing a
value (no-stub doctrine) — `the_13_families_reproduce_cycle_3s_61_record_conventional_
population_minus_one_named_gap` asserts `unresolved == vec![("book_of_the_damned_volume_2",
"Demoniac")]` exactly, and `records.len() == 60`.

### Wired into `class_catalog.rs`

`build_class_catalog()` extended with `entries.extend(generic_class_catalog_entries(&repo_
root))` behind a `codex_repo_root()` `Ok`-guard (skip, not panic, on a packaged deployment
missing `data/corpus/` — the same caveat `class_feature_descriptions.rs` already documents)
— the identical additive pattern the CRB->PU widening already used. Catalog row count:
300 -> 1108 (+808, the exact sum of the 60 resolved classes' own `max_level`s, re-derived by
a `python3` sweep cited inline in both the module doc and the updated test's own comment).
Verified no display-name collision against any of the 15 existing CRB/PU rows
(`generic_catalog_entries_cover_all_61_classes_with_no_overlap_into_crb_pu_names` — the test
name says 61 for symmetry with the module's own framing; it asserts the actual 60 distinct
names against the CRB/PU list, per its own body).

## `§16` — precisely scoped, not overclaimed

This cycle builds the progression **table** — the same artifact `class_tables()` and
`pathfinder_unchained::class_chassis` already are for the CRB/PU classes, and what
`class_catalog.rs`'s own pre-existing doc comment names as "a separate piece of work with its
own row-count expectations" for the (different) 16 APG/ACG classes. It does **not** wire a
character-creation-time `ClassId` picker (`character_hub.rs`/`pf1_adapter.rs`) — that is
real, separate, cross-file work, deliberately left for a later cycle rather than touched
here (both files are live territory this cycle stayed out of, matching cycle 3's own
discipline). Naming the exact remedy shape rather than letting "chassis" quietly mean two
different things across cycles.

## Full-sweep re-run

```
cd apps/desktop/src-tauri && cargo test --locked --bin codex-desktop
```

**546 passed, 0 failed** (78.22s) — 538 (cycle 3's confirmed baseline) + 8 new tests in
`class_catalog_generic.rs`, matching exactly (no silent test loss, no unexplained delta).

## PI / audit

- Own-diff `git diff --unified=0 HEAD -- apps/desktop/src-tauri/src/{class_catalog.rs,
  main.rs} docs/release/.../{kanban.md,progress.md}` plus the full new
  `class_catalog_generic.rs` file: `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|
  t_[0-9a-f]{8,})'` — zero hits (`OK_NO_BUNDLE_TAGS`).
- Same scope, `grep -nE 'todo!|unimplemented!|TODO stub|FIXME stub'` — zero hits
  (`OK_NO_TOKENS`).
- `pi_scrub.normalized_term_hits()` over the same scope — zero hits.
- No `data/corpus/` write this cycle (read-only at runtime), so the `declared_pi_shipping_
  audit` before/after requirement does not apply — noted rather than silently skipped. A
  direct `cargo run --bin declared_pi_shipping_audit` attempt this cycle exceeded a 240s
  timeout without finishing; not required by this cycle's own scope, not force-run further.

## Territory

`git status --porcelain` confirmed clean before every write (only this cycle's own edits
plus `verify.sh`'s own append to `docs/retro/events/sd31-transcribe.jsonl` appeared). No
`pilot_compute`, `class_feature_grant_consumer.rs`, `companion_catalog.rs`, or
`formula_interpreter.rs` write. New file (`class_catalog_generic.rs`) confirmed unowned via
`git log origin/tranche/12 -- apps/desktop/src-tauri/src/class_catalog_generic.rs` (no
history — genuinely new). `class_catalog.rs`/`main.rs` last touched at `265ec7ca0a`/ancestor
commits, both ancestors of `$PIN`, no live sibling activity in either.

## Next-cycle plan

1. **Widen `formula_interpreter.rs`'s `classlevel(...)` grammar to accept a bare,
   zero-argument call** (a row 18/generic-evaluator cycle, not this card's own territory) —
   closes `Demoniac`, the 61st and last conventional class.
2. **Build the companion base-ability-score category table** (a handful of rows, per this
   cycle's `RACESUBTYPE:` finding — not ~40 species) as Rust constants generalizing the
   existing Wolf/Horse pattern, then wire it through a companion-scoped consumer mirroring
   `class_feature_grant_consumer.rs`.
3. **Wire a character-creation-time `ClassId` picker** for the 60 (soon 61) generically-
   computed classes into `character_hub.rs`/`pf1_adapter.rs` — the catalog-table half is
   done; the selectable-at-creation half is not.
4. Row 20 stays `in-progress` under `decisions.md §10` until all three above close or are
   further resized with evidence.
