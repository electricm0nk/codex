# Cycle row20-cycle5 — Epic 10 (`epic-10-reference-library-residual-reach`) / Row 20

- **Card ID:** `epic-10-reference-library-residual-reach`
- **Files touched:**
  - `src/rules_core/pilot_compute/companion_base_stat_table.rs` (new).
  - `src/rules_core/pilot_compute/generic_class_chassis.rs` (new).
  - `src/rules_core/pilot_compute/mod.rs` (two `mod` declarations, additive; one new
    `compute_class_chassis` dispatch arm inserted between the `untabled_base_class_chassis`
    arm and the `prestige_class_entry_gate` fallback — no existing arm's logic edited).
  - `docs/release/SD-32-compute-library-and-cause-closure/{kanban.md,progress.md}` (this
    cycle's own row 20 entry).
  - This receipt.
  - `docs/retro/events/sd31-transcribe.jsonl` — append-only, auto-written by
    `scripts/verify.sh --only preflight-oracle`'s own instrumentation. Not a manual edit.
  - No `class_catalog_generic.rs`, `class_catalog.rs`, `companion_catalog.rs`,
    `character_hub.rs`, `pf1_adapter.rs`, or `formula_interpreter.rs` write. The picker ask
    ("wire `character_hub.rs`/`pf1_adapter.rs`") resolves, on inspection, to wiring
    `compute_class_chassis` — the SAME character-creation-time dispatch function
    `untabled_base_class_chassis::resolve` already plugs into — because `character_hub.rs`'s
    own `class_id` field is already a free-form `"class:<slug>"` string (confirmed by direct
    read: `CreateCharacterRequest.class_id: String`, no `ClassId`-enum-gated dropdown exists
    anywhere in that file), so the real gap was the DISPATCH arm, not a UI picker widget.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own diff, `git diff --unified=0 HEAD --
  src/rules_core/pilot_compute/{mod.rs,companion_base_stat_table.rs,generic_class_chassis.rs}
  docs/release/SD-32-compute-library-and-cause-closure/{kanban.md,progress.md} | grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` plus the same grep over both new files in
  full — zero hits both).
- **Wired-integration audit result:** `OK_NO_TOKENS` (`grep -nE 'todo!|unimplemented!|TODO
  stub|FIXME stub'` over the same scope — zero hits; every unmapped companion species in
  `companion_base_stat_table` REFUSES via `ground_companion_stat_block` returning `false`,
  proven by its own passing test — the same "refuse, never guess" posture `generic_class_
  chassis::resolve` also proves for any level past `max_level` or any unrecognized class id).
- **PI scrub:** `pi_scrub.normalized_term_hits()` over the own-diff and over both new files in
  full — zero hits both.
- **Corpus SHA:** oracle bootstrapped fresh this worktree (slot was empty), pinned at
  `7f818006e371188e5717fd18d74d18a420747fc6` — the same commit cycles 2/3/4 used, confirmed
  via `scripts/verify.sh --only preflight-oracle` (FAIL before bootstrap, PASS after).
- **Status:** `in-progress` (NOT `complete` — item (a), the character-creation picker, is now
  fully closed for all 61 classes; item (b)/(c), 210 of 213 companion species, remains real,
  sized, unbuilt work).

## Starting state (verified, not assumed)

`git rev-parse HEAD` on entry was `1bb523773d` (the SD-31 PR #374 merge commit) — the same
stale-lineage footgun cycles 2/3/4 all hit. `git merge-base --is-ancestor $PIN HEAD` failed.
Recovered: `git reset --hard $PIN`, confirmed `BASE_OK`. `git fetch origin tranche/12` found
`origin/tranche/12`'s own tip already equal to `$PIN` (row18 cycle8's own commit,
`ef4a6ffca20d`) — no rebase needed, no other lane had pushed since. `git log origin/tranche/12
-- src/rules_core/pilot_compute/mod.rs apps/desktop/src-tauri/src/class_catalog_generic.rs`
confirmed no activity past `$PIN` in either file before this cycle started.

Oracle slot was empty (fresh worktree, git-ignored). `scripts/fetch-pcgen-oracle.sh --dest
$PCGEN_REPO_DIR` populated it at `7f818006e371188e5717fd18d74d18a420747fc6`, re-confirmed via
`scripts/verify.sh --only preflight-oracle` (PASS after).

## Item 1: the companion base-ability-score table — built, and cycle 4's sizing corrected

Read cycle 4's receipt first, per this cycle's own brief. Cycle 4 concluded the base-stat
block is "category-keyed (a handful of `RACESUBTYPE:` categories), not per-species" — inferred
from several Ultimate Wilderness companions sharing a `RACESUBTYPE:PlantCompanion` tag, but
**not itself checked against a second same-category member's own printed numbers.**

### Re-derived, per `§17a` — the population is 213, and the category hypothesis fails a direct check

```
grep -rh "RACETYPE:Companion" $PCGEN_REPO_DIR/data | wc -l                     # -> 213
grep -rh "RACETYPE:Companion" $PCGEN_REPO_DIR/data \
  | grep -oE "RACESUBTYPE:[A-Za-z]+" | sort | uniq -c | sort -rn
#  31 RACESUBTYPE:AnimalCompanionDinosaur
#  13 RACESUBTYPE:Aquatic
#   8 RACESUBTYPE:PlantCompanion
#   4 RACESUBTYPE:AnimalCompanionPrimate
#   3 RACESUBTYPE:ConstructCompanion
# (154 of 213 carry NO RACESUBTYPE tag at all -- each an individually-named,
#  individually-statted real-world/Bestiary species, e.g. "Companion (Elephant)")
```

59 of 213 carry any `RACESUBTYPE:` tag at all — already smaller than "a handful of categories
covers everything" implies once the 154 untagged records are counted. Direct check on the
category hypothesis itself: two Ultimate Wilderness companions sharing `RACESUBTYPE:
PlantCompanion` — Gulper Plant and Hunting Cactus — verified via aonprd.com's own Druid
Companions pages (Gulper Plant: Str 12/Dex 11/Con 13/Int 1/Wis 10/Cha 3; Hunting Cactus: Str
14/Dex 13/Con 17/Int 2/Wis 13/Cha 6), backed against their own corpus `BONUS:STAT` deltas
(Gulper Plant `STR|2 CON|2 INT|-10 CHA|-8`; Hunting Cactus `STR|4 DEX|2 CON|6 INT|-8 WIS|2
CHA|-4`): Gulper Plant backs out a base of Str 10/Dex 11/Con 11/Int 11/Wis 10/Cha 11; Hunting
Cactus backs out Str 10/Dex 11/Con 11/Int 10/Wis 11/Cha 10. **Same category, two different
base vectors.** The shared-category-base hypothesis does not survive this check — this is
genuinely per-species data. Correction logged here and in `kanban.md`'s own row 20 cell,
`decisions.md §17a`.

### Built, table-driven, generalizing Wolf/Horse — not 213 hand-authored functions

`companion_base_stat_table.rs` (new, `pilot_compute`): `CompanionBaseStats` (strength,
constitution, natural armor, hit die size — the four fields with a live downstream reader,
matching Wolf/Horse's own scope) keyed by species slug; `ground_companion_stat_block` is
ONE function, table-driven, reusing this module's parent's own already-proven universal
advancement math (`animal_companion_table_index`/`_natural_armor_bonus`/`_stat_bonus`/
`_hit_points`) — confirmed species-agnostic because every candidate record (including Gulper
Plant's own) carries the identical `MONSTERCLASS:Companion:2` progression tag Wolf/Horse
already read. Hit Die size is 8 for every entry (the PF1 Core Rulebook's own companion
mechanic fixes this regardless of the companion's real creature type — confirmed by Wolf and
Horse, two different real types, already sharing it).

Populated with three entries: **Wolf and Horse, re-derived (not copied) from the existing
hand-authored constants**, proven to reproduce the shipped, tested output byte-for-byte
(`generic_wolf_reproduces_the_existing_hand_authored_wolf_function`/`generic_horse_
reproduces_the_existing_hand_authored_horse_function`, excluding only each species' own
natural-attack-damage record — bite/hoof — which stays out of this module's scope exactly as
its own doc states: "grounds only the fields with a live downstream reader"). **Plus one new
species, Gulper Plant**, externally verified (aonprd.com), with the corpus's own `BONUS:STAT`
deltas as the tiebreaker (both agree once the delta is backed out).

### Named, not fabricated: 210 of 213 refuse

`ground_companion_stat_block` returns `false` (grounds nothing) for any species not in the
table — proven directly by `an_unknown_species_slug_refuses_rather_than_guesses` and by
`only_three_of_the_corpus_s_213_racetype_companion_records_have_a_base_stat_entry` asserting
the table's own length is exactly 3. Building all 213 (or even the 59 `RACESUBTYPE`-tagged)
to the same two-independent-source verification bar Wolf/Horse's own precedent sets is real,
sized, per-species sourcing work no single cycle can complete without lowering that bar — and
an unverified number here is a worse outcome than none: a silently-wrong ability score
corrupts a real character's combat math, the exact failure this codebase's own
anti-fabrication test suite (`class_feature_grant_consumer.rs`'s thirteen-test gate) exists to
refuse. Refuse-not-guess, named exactly, per `§1a`/`§16`.

## Item 2: the character-creation-time `ClassId` picker — wired for all 61 classes

Read `character_hub.rs` directly before building anything: `CreateCharacterRequest.class_id`
is already a free-form `String` (e.g. `"class:fighter"`), never a `ClassId`-enum-gated
dropdown — confirmed by grep, zero `ClassId::ALL` references anywhere in that file. The real
picker mechanism, for every class family this engine already supports, is `pilot_compute::
mod.rs`'s own `compute_class_chassis` string-keyed dispatch chain (CRB -> APG -> ACG -> PU ->
UC -> `untabled_base_class_chassis::resolve` -> `prestige_class_entry_gate` -> `None`). Cycle
4's own generic BAB/save table (`class_catalog_generic.rs`, `apps/desktop/src-tauri`) had no
arm in that chain — a character picking any of the 61 resolved classes at creation could
browse it in the reference catalog but never actually compute a real chassis for it.

**Could not reuse `class_catalog_generic.rs`'s own functions directly**: `pilot_compute::
mod.rs` lives in the core `codex` crate; `apps/desktop/src-tauri` is a separate, downstream
crate that depends on `codex`, never the reverse — importing from it into `mod.rs` would be an
illegal reverse dependency. Built `generic_class_chassis.rs` (new, `pilot_compute`) as the
crate-internal sibling: the SAME classification/extraction logic (`classify_class_record`/
`select_baseab_formula`/`select_save_formulas`/`max_level_for`), re-implemented at the crate
boundary it is actually needed at — the same "parallel per-family module" shape `class_
slayer.rs`/`class_ultimate_combat.rs` already use. `resolve(class_id_str, level)` returns a
real `GenericChassisRow` (BAB + 3 saves), or `None` (any level past `max_level`; any
unrecognized id) — never a guess.

Wired a new `compute_class_chassis` arm, `else if let Some(row) = generic_class_chassis::
resolve(...)`, positioned between the `untabled_base_class_chassis` arm and the
`prestige_class_entry_gate` fallback (so every class family already dispatched — including
Vigilante, which is BOTH one of cycle 4's 61 classes AND one of the 20 `untabled_base_class_
chassis` classes — keeps resolving through its existing, higher-priority arm; this new arm
only ever catches an id nothing else recognizes). Same explanation-id shape as every sibling
arm (`class_chassis.base_attack_bonus`/`base_save.{fortitude,reflex,will}`).

**All 61 resolve, including Demoniac — closed mid-cycle by this cycle's own rebase.** At the
first write of this receipt (before step 6's rebase), Demoniac still refused: its bare
`classlevel()` tripped the same `formula_interpreter.rs` grammar gap cycle 4 named
(`grep -n classlevel src/rules_core/pilot_compute/formula_interpreter.rs` at this cycle's
starting base `ef4a6ffca20d`, row 18 cycle 8's own commit, showed the grammar arm's error text
unchanged: `"classlevel(...) expects a string literal class name"`). Step 6's own `git fetch
origin tranche/12 && git rebase origin/tranche/12` (required regardless, per the concurrent-
write protocol) picked up row 18 cycle 9 (`49306a805c`, pushed between this cycle's start and
its own push), which widened the grammar to PARSE a bare `classlevel()` call but deliberately
left EVALUATION refusing until a caller binds the resulting empty `CLASSLEVEL::` sentinel key
(its own doc: "No caller today binds `CLASSLEVEL::` (empty key)"). Per this cycle's own brief
("if the widening has landed, close your 61st and say so"): it landed, and `generic_class_
chassis::resolve` is exactly that caller — a bare `classlevel()` inside a class's own record
can only mean that SAME class's own level, so `resolve` now also binds
`CLASSLEVEL::` -> the record's own already-known `level`, never a guess. Updated
`generic_class_chassis.rs`'s own tests to assert 61/61 (not 60/61) and added
`demoniac_resolves_via_the_bare_classlevel_binding`, hand-verified against Demoniac's real
corpus tokens read directly (`BONUS:SAVE|BASE.Will,BASE.Reflex|(classlevel()+1)/3` — Reflex is
packed with Will, not Fortitude, confirmed by direct read rather than assumed uniform). Full
targeted re-run after the fix: `companion_base_stat_table` (6/0), `generic_class_chassis`
(5/0), `formula_interpreter`'s own `classlevel*` tests (4/0) — all green.

## Test evidence

```
cargo test --locked -p codex --lib companion_base_stat_table   # 6 passed, 0 failed
cargo test --locked -p codex --lib generic_class_chassis       # 5 passed, 0 failed
cargo test --locked -p codex --lib animal_companion             # 14 passed, 0 failed (unchanged)
cargo test --locked -p codex --lib chassis_unsupported          # 3 passed, 0 failed (unchanged)
cargo test --locked -p codex --lib prestige_class_entry_gate_wiring_tests   # 3 passed, 0 failed
cargo test --locked -p codex --lib untabled_base_class_chassis_wiring_tests # 3 passed, 0 failed
```

Full `apps/desktop/src-tauri` suite re-run post-rebase: `cargo test --locked --bin
codex-desktop` -> **546 passed, 0 failed** (78.19s) — exactly cycle 4's own baseline, confirming
this cycle's core-crate-only changes (`apps/desktop/src-tauri` is untouched) introduce no
regression there.

## PI / audit

- Own-diff plus both new files in full: `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|
  t_[0-9a-f]{8,})'` — zero hits (`OK_NO_BUNDLE_TAGS`).
- Same scope, `grep -nE 'todo!|unimplemented!|TODO stub|FIXME stub'` — zero hits
  (`OK_NO_TOKENS`).
- `pi_scrub.normalized_term_hits()` over the same scope — zero hits.
- No `data/corpus/` write this cycle (both new modules read-only at runtime), so the
  `declared_pi_shipping_audit` before/after requirement does not apply.

## Territory

`git status --porcelain` confirmed clean before every write. No `class_catalog_generic.rs`,
`class_catalog.rs`, `companion_catalog.rs`, `character_hub.rs`, `pf1_adapter.rs`, or
`formula_interpreter.rs` write. Both new files confirmed unowned via `git log origin/tranche/12
-- src/rules_core/pilot_compute/{companion_base_stat_table.rs,generic_class_chassis.rs}` (no
history — genuinely new). `mod.rs` last touched at row 18 cycle 8's own commit, an ancestor of
`$PIN`; no live sibling activity in it at this cycle's start.

## Next-cycle plan

1. **Companion base-stat table**: pick the next batch of species by population weight
   (`AnimalCompanionDinosaur`'s 31 records is the largest untouched bucket) and repeat this
   cycle's own two-source-plus-corpus-tiebreaker verification per species, adding entries to
   `companion_base_stat_table.rs`'s table one verified batch at a time. 210 of 213 remain.
2. Row 20 stays `in-progress` under `decisions.md §10` until the companion residual closes or
   is further resized with evidence. Item (a), the character-creation picker, is fully closed
   (61 of 61) and needs no further cycle work.
