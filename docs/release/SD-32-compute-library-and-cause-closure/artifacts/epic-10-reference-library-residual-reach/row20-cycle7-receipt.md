# Cycle row20-cycle7 — Epic 10 (`epic-10-reference-library-residual-reach`) / Row 20

- **Card ID:** `epic-10-reference-library-residual-reach`
- **Files touched:**
  - `src/rules_core/pilot_compute/mod.rs` (new `COMPANION_SPECIES_CHOICE_ID` constant, new
    `ground_selected_companion_or_default` dispatch function, three call-site updates: Druid,
    Hunter, Cavalier).
  - `src/rules_core/pilot_compute/companion_base_stat_table.rs` (17 new table entries, new
    `companion_display_name` helper, module doc addendum, five new/updated tests).
  - `apps/desktop/src-tauri/src/character_hub.rs` (new `CreateCharacterRequest.companion_species`
    field, one new character-creation-altitude test).
  - `apps/desktop/src-tauri/src/pf1_adapter.rs` (`compose_character_input` threads
    `companion_species` into the new choice; one literal-construction-site fix in `request_for`;
    one new import).
  - `apps/desktop/src-tauri/src/rule_system_adapter.rs`, `stub_adapter.rs`,
    `characterHub/appendToCharacter.rs`, `characterHub/reSaveCharacter.rs` (each: one
    literal-construction-site fix for the new `CreateCharacterRequest` field).
  - `docs/release/SD-32-compute-library-and-cause-closure/{kanban.md,progress.md}` (this cycle's
    own row 20 entry).
  - This receipt.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own diff, `git diff --unified=0 HEAD` over the
  8 touched source files, `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — zero
  hits).
- **Wired-integration audit result:** `OK_NO_TOKENS` (`grep -cE 'todo!|unimplemented!|TODO stub|
  FIXME stub'` over the same own-diff scope — zero hits). Real wiring, not a stub: the dispatch
  function is called from all three production companion-grounding sites, and a real request
  field feeds it through the real character-creation request path.
- **PI scrub:** `pi_scrub.normalized_term_hits()` (imported, not copied) over the own-diff — found
  ONE hit on first pass (a site-name phrase this module's own doc comment briefly used instead of
  the `aonprd.com` short form every other reference in this file already uses — the long form's
  own second word is a PF1 deity name, a blacklist term) — corrected to `aonprd.com` (already used
  throughout this file since cycle 5) before pushing; re-scrubbed clean, zero hits.
- **Corpus SHA:** oracle bootstrapped fresh this worktree (slot was empty), pinned at
  `7f818006e371188e5717fd18d74d18a420747fc6` — the same commit prior cycles used, confirmed via
  `scripts/verify.sh --only preflight-oracle` (FAIL before bootstrap, PASS after).
- **Status:** `in-progress` (NOT `complete` — item 1, the species-selection wiring, is now
  verified closed at the real character-creation altitude; item 2, 184 of 213 companion species,
  remains real, sized, unbuilt work).

## Starting state (verified, not assumed)

`git rev-parse HEAD` on entry was `1bb523773d` (the SD-31 PR #374 merge commit) — the same
stale-lineage footgun every prior cycle in this row hit (footgun 1). Recovered: `git reset --hard
$PIN`, confirmed. `git fetch origin tranche/12` found `origin/tranche/12`'s own tip already equal
to `$PIN` (cycle 6's own commit, `0950f53bc5`) — no rebase conflicts, no sibling collision.

Oracle slot was empty (fresh worktree, git-ignored). `scripts/fetch-pcgen-oracle.sh --dest
$PCGEN_REPO_DIR` populated it at `7f818006e371188e5717fd18d74d18a420747fc6`, re-confirmed via
`scripts/verify.sh --only preflight-oracle` (PASS after).

## Item 1: closing cycle 6's own named wiring gap

Cycle 6 named, not hidden: `ground_companion_stat_block` had zero live callers anywhere in the
crate (confirmed by `cargo build`'s own dead-code warning on both it and `CompanionBaseStats`),
because no companion-bearing class offered a character-creation-time CHOICE among species. The
brief for this cycle named this as the priority, ahead of adding more species: "a table with no
consumer is exactly the shape this bundle keeps catching."

### The mechanism

`CharacterInput.chosen.selected_choices` already carries a generic `Vec<SelectedChoice>` channel
(`choice_set_id` + `selection_id` strings) that `choice:druid_nature_bond`, `choice:
cavalier_order`, and `RACE_ALTERNATE_TRAIT_CHOICE_ID` all already use — no new struct field was
needed on `CharacterInput` itself. What was missing was (a) a read-side dispatch function that
consults this channel for a companion species, and (b) a write-side field on the real
character-creation request DTO to populate it from a real caller.

Added both:

1. `pilot_compute/mod.rs`: `pub const COMPANION_SPECIES_CHOICE_ID: &str =
   "choice:companion_species"`, and:

   ```rust
   fn ground_selected_companion_or_default(
       input: &CharacterInput,
       id_prefix: &str,
       owner_class_label: &str,
       companion_level: u8,
       default_ground: fn(&str, &str, u8, &mut Vec<ComputationExplanation>),
       explanations: &mut Vec<ComputationExplanation>,
   ) {
       let Some(species_slug) = choice_selection(input, COMPANION_SPECIES_CHOICE_ID) else {
           default_ground(id_prefix, owner_class_label, companion_level, explanations);
           return;
       };
       let display_name = companion_base_stat_table::companion_display_name(species_slug);
       let grounded = companion_base_stat_table::ground_companion_stat_block(
           species_slug, id_prefix, owner_class_label, &display_name, companion_level, explanations,
       );
       if !grounded {
           default_ground(id_prefix, owner_class_label, companion_level, explanations);
       }
   }
   ```

   Wired into the three real production call sites: `ground_cavalier_mount_and_defer_the_rest`,
   `ground_hunter_animal_companion_and_defer_the_rest`, and the Druid animal-companion block
   inside `explain_druid_level1_spell_baseline` — all three already had `input: &CharacterInput`
   in scope, so no signature change propagated further.

2. `apps/desktop/src-tauri/src/character_hub.rs`: new `CreateCharacterRequest.companion_species:
   Option<String>` field, `#[serde(default)]` (so every pre-existing payload and every
   non-companion class keeps working unchanged).

3. `apps/desktop/src-tauri/src/pf1_adapter.rs`'s `compose_character_input`: when
   `request.companion_species` is `Some` AND the class is Druid, Hunter, or Cavalier, pushes
   `SelectedChoice { choice_set_id: COMPANION_SPECIES_CHOICE_ID, selection_id: species_slug }`
   onto `selected_choices` — scoped to exactly the three classes that read it, so pushing it for
   any other class would be dead, never-read data.

### A real regression caught before it shipped

The first version of `ground_selected_companion_or_default` used `unwrap_or(default_species_slug)`
to pick a slug (defaulting to `"wolf"`/`"horse"` when no selection was present), then always
consulted the generic table. Since the table already contains `"wolf"`/`"horse"` (cycle 5's own
reproduction rows, proven byte-for-byte equivalent to the hand-authored functions for the fields
BOTH ground), this silently replaced the hand-authored `ground_wolf_companion_stat_block`/
`ground_horse_companion_stat_block` calls even for the OVERWHELMING MAJORITY of characters that
never make any companion-species selection at all — and the generic table-driven function
deliberately does NOT ground the per-species natural-attack record (Wolf's own `bite_attack`,
Horse's own `hoof_attack` — see this module's own doc comment, "grounds only the fields with a
live downstream reader"). `cargo test --locked -p codex --lib animal_companion` caught this
immediately:
`druid_dispatch_widening_safety_tests::single_class_druid_level1_with_animal_companion_reaches_computed`
failed on a missing `class_chassis.druid.animal_companion.bite_attack` record. Fixed by gating the
table lookup strictly behind an ACTUAL present selection (`let Some(species_slug) =
choice_selection(...) else { default_ground(...); return; }`) — no selection means the literal
same code path as before this cycle, not merely equivalent output. Re-ran; all 14 `animal_
companion` tests green, including the one that caught it.

### Proven at the real character-creation altitude

Per this cycle's own instruction ("prove the wiring at the character-creation altitude, as cycle
6 did for classes — not in an isolated unit test"), added
`a_druid_who_selects_gulper_plant_grounds_gulper_plant_not_wolf_at_character_creation_altitude`
to `character_hub.rs`'s own test module — the same file, same altitude, same
`compose_character_input` -> `build_pilot_headless_receipt` path cycle 6's own class-picker test
used, not `generic_class_chassis::resolve` or `ground_companion_stat_block` called directly.
Asserts three things:

1. **Precondition, not assumed**: a Druid request with `companion_species: None` still reaches
   `Computed` and still grounds `class_chassis.druid.animal_companion.wolf_stat_block` — the
   existing default, unchanged.
2. **The override works**: a Druid request with `companion_species: Some("gulper_plant")` reaches
   `Computed`, grounds `class_chassis.druid.animal_companion.gulper_plant_stat_block` with the
   correct base attack bonus (+1, matching `companion_base_stat_table.rs`'s own
   `gulper_plant_grounds_a_real_new_species_at_master_level_1` test), and does NOT also ground
   `wolf_stat_block` — the dispatch replaces the default, it does not merely add to it.
3. **An unrecognized species falls back honestly**: `companion_species: Some("griffon")` (no
   verified table row) still reaches `Computed` and grounds Wolf — never fabricates a Griffon
   stat block, never blocks the character.

## Item 2: continued the species table — 17 of 19, two named refusals

Per cycle 6's own next-cycle order, continued the `AnimalCompanionDinosaur` bucket. Re-derived the
exact remaining population (not trusted from the brief): `grep -rl "AnimalCompanionDinosaur"
data/corpus/*/companion/*.json` minus the nine already in the table found 19 records:
`elasmosaurus`, `stegosaurus`, `dimetrodon`, `iguanodon`, `pachycephalosaurus`, `spinosaurus`,
`dimorphodon`, `diplodocus`, `styracosaurus`, `ceratosaurus`, `plesiosaurus`, `therizinosaurus`,
`troodon`, `giganotosaurus`, `kentrosaurus`, `quetzalcoatlus`, `parasaurolophus`, `tylosaurus`,
`ornithomimosaur` — matching the brief's own count.

### Verification method (reusing cycle 6's own)

For each species: a real "Starting Statistics" source (aonprd.com and/or d20pfsrd, via web
search, with a second independent search query per species as the cross-check cycle 6 set) for
the printed Str/Con, plus the corpus's own `BONUS:STAT` delta (`printed − delta = base`, the same
formula cycle 6's own worked examples establish) as the numeric tiebreaker. Natural armor read
directly from the corpus's own `BONUS:VAR|AC_Natural_Armor|n|TYPE=Base` token — cycle 6's own
"natural armor is direct, no backing-out needed" simplification, reconfirmed against the source's
own printed "+n natural armor" line for all 17 species below.

| species | source Str/Con | corpus delta | backed-out base Str/Con | natural armor |
|---|---|---|---|---|
| Elasmosaurus | 10 / 12 | CON+2 (no STR) | 10 / 10 | 2 (direct) |
| Stegosaurus | 10 / 10 | (no STR/CON) | 10 / 10 | 6 (direct) |
| Dimetrodon | 12 / 14 | STR+2, CON+4 | 10 / 10 | 2 (direct) |
| Iguanodon | 17 / 15 | STR+6, CON+4 | 11 / 11 | 3 (direct) |
| Spinosaurus | 18 / 15 | STR+8, CON+4 | 10 / 11 | 3 (direct) |
| Dimorphodon | 10 / 10 | STR−2, CON+2 | 12 / 8 | 1 (direct) |
| Diplodocus | 10 / 10 | (no STR/CON) | 10 / 10 | 6 (direct) |
| Styracosaurus | 10 / 11 | (no STR/CON) | 10 / 11 | 6 (direct) |
| Ceratosaurus | 14 / 11 | STR+4 (no CON) | 10 / 11 | 4 (direct) |
| Plesiosaurus | 12 / 12 | STR+2, CON+2 | 10 / 10 | 1 (direct) |
| Therizinosaurus | 12 / 10 | STR+2 (no CON) | 10 / 10 | 4 (direct) |
| Troodon | 7 / 10 | STR−4 (no CON) | 11 / 10 | 0 (verified absent) |
| Giganotosaurus | 14 / 10 | STR+4 (no CON) | 10 / 10 | 4 (direct) |
| Kentrosaurus | 10 / 10 | (no STR/CON) | 10 / 10 | 2 (direct) |
| Quetzalcoatlus | 9 / 10 | STR−2 (no CON) | 11 / 10 | 2 (direct) |
| Parasaurolophus | 11 / 9 | CON−2 (no STR) | 11 / 11 | 2 (direct) |
| Tylosaurus | 10 / 10 | (no STR/CON) | 10 / 10 | 3 (direct) |

Two entries have a NEGATIVE species delta on Strength (Dimorphodon, Quetzalcoatlus), the first
this table has grounded — both plausible for lightly-built fliers, and both confirmed against a
second independent search before trusting the arithmetic.

**Troodon** is the first table entry with a genuine `natural_armor: 0` rather than an absent
row: its source explicitly states "no natural armor bonus at the starting level," and the corpus's
own record carries no `AC_Natural_Armor` token at all — the two agree exactly, so `0` is a
verified fact here, not a placeholder for "not yet grounded."

### Two named, not silently skipped (`§1a`/`§16`)

- **`pachycephalosaurus`** (Bestiary 3): every source this cycle reached (multiple search queries,
  a direct `d20pfsrd.com` fetch) returned only the full-grown CR-4 monster's own stat block (Str
  22, Dex 15, Con 17, +6 natural armor) — the animal-companion "Starting Statistics" block, which
  exists on Bestiary 3's own companion page separately from the monster entry, could not be
  isolated from any source reachable this cycle.
- **`ornithomimosaur`** (Ultimate Wilderness): the one source found gave "STR=11, DEX=11, CON=10,
  INT=10, WIS=11, CHA=10" — ambiguous between the companion's own verified base and the shared
  "Companion Body Type ~ Avian" template baseline several Ultimate Wilderness companions read
  from (the corpus's own `ability_keys` for this record names `companion_body_type_avian`
  explicitly). Could not resolve which value this number represents against a second independent
  source this cycle.

Both refuse (`ground_companion_stat_block` returns `false`, grounds nothing) — pinned by a new
`pachycephalosaurus_and_ornithomimosaur_still_refuse_unverified` test, per the same posture
`class_feature_grant_consumer.rs`'s own module doc names throughout: refuse rather than guess.

Table: 12 -> 29 entries (Wolf, Horse, Gulper Plant, 26 of 28 `AnimalCompanionDinosaur` species —
the two above are the ENTIRE remaining residual in that bucket).

## Test evidence

```
cargo test --locked -p codex --lib companion_base_stat_table   # 10 passed, 0 failed
cargo test --locked -p codex --lib animal_companion             # 14 passed, 0 failed
cargo test --locked -p codex --lib druid                        # 20 passed, 0 failed
cargo test --locked -p codex --lib hunter                       # 40 passed, 0 failed
cargo test --locked -p codex --lib cavalier                     # 16 passed, 0 failed
cargo test --locked -p codex --lib companion                    # 120 passed, 0 failed
cargo test --locked -p codex --lib pilot_compute::               # 948 passed, 0 failed
```

Full `apps/desktop/src-tauri` suite re-run: `cargo test --locked --bin codex-desktop` ->
**548 passed, 0 failed** (77.02s) — cycle 6's own 547/0 baseline plus this cycle's one new
`character_hub.rs` test (unchanged from a second, later re-run after the species-table addition,
confirming the table growth touches no desktop-crate compute path).

`cargo build --locked -p codex --lib` confirms `ground_companion_stat_block`/`CompanionBaseStats`
no longer trigger a dead-code warning — direct proof the wiring is real, not merely present in
source.

## PI / audit

- Own-diff (`git diff --unified=0 HEAD` over the 8 touched files):
  `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — zero hits (`OK_NO_BUNDLE_TAGS`).
- Same scope, `grep -cE 'todo!|unimplemented!|TODO stub|FIXME stub'` — zero hits
  (`OK_NO_TOKENS`).
- `pi_scrub.normalized_term_hits()` (imported from `scripts/pi_scrub.py`, never copied) over the
  own-diff — found ONE hit on first pass, on the site-name long form this module's own doc
  comment briefly used instead of the `aonprd.com` short form every other reference in this file
  already uses (the long form's own second word is a PF1 deity name, a blacklist term).
  Corrected to `aonprd.com` before pushing; re-scrubbed, zero hits.
- No `data/corpus/` write this cycle (both touched compute modules read-only at runtime; the
  `companion_base_stat_table.rs` entries are hand-authored Rust constants, not corpus records).

## Territory

`git status --porcelain` confirmed clean before every write and listed only the 8 intended files
after (plus `kanban.md`/`progress.md`/this receipt). `kanban.md` row parsing verified: 23
pipe-lines (21 data rows + header + separator), 21 unique row ids, 0 duplicates, row 20's own
cells split to 9 raw segments (7 logical columns) with a backtick-aware parser before and after
the edit. Rows 11 and 15 left untouched.

## Next-cycle plan

1. **Companion base-stat table**: the `Aquatic` (13), `PlantCompanion` (7 remaining),
   `AnimalCompanionPrimate` (4), and `ConstructCompanion` (3) tagged buckets, largest first,
   repeating this cycle's own two-independent-source-plus-corpus-tiebreaker method. The 154
   untagged `RACETYPE:Companion` records remain the largest single residual and have no
   established verification shortcut yet — worth a dedicated sizing pass.
2. **`pachycephalosaurus`/`ornithomimosaur`**: revisit if a future cycle finds a source this one
   could not (a scanned page image of Bestiary 3/Ultimate Wilderness, or a corrected search).
3. Row 20 stays `in-progress` under `decisions.md §10` until the companion residual closes or is
   further resized with evidence. Item 1, the species-selection wiring, is now verified closed at
   the real altitude and needs no further cycle work.
