# Cycle row20-cycle2 — Epic 10 (`epic-10-reference-library-residual-reach`) / Row 20

- **Card ID:** `epic-10-reference-library-residual-reach`
- **Files touched:**
  - `src/bin/v06_work_inventory.rs` (retargeted two stale-pin unit tests to proven-live
    facts, per `decisions.md §1a`/`§16`; no production code changed)
  - `docs/retro/events/sd31-transcribe.jsonl` (benign append from `scripts/verify.sh`'s own
    `--only preflight-oracle` runs, this cycle's own two calls)
  - `docs/release/SD-32-compute-library-and-cause-closure/{kanban.md,progress.md}` (this
    cycle's own row 20 entry)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own diff,
  `git diff --unified=0 HEAD -- src/bin/v06_work_inventory.rs | grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — zero hits).
- **Wired-integration audit result:** `OK_NO_TOKENS` (no `todo!`/`unimplemented!`/stub
  markers introduced; the one `stub` string match is inside a doc comment reading "not a
  stub").
- **PI scrub:** `pi_scrub.normalized_term_hits()` over the full diff — zero hits.
- **Acceptance criterion:** reproduce and decide, with evidence, the two bundle-wide
  unowned reds (`decisions.md §27b` — "pre-existing" is not a disposition); re-derive the
  18-family class-chassis/class_features residual and the 4-record `mod_only` residual;
  investigate the character-scoped consumer surface (item b) against row 17/18's
  `pilot_compute` choice seam; coordinate item (c) against row 21's ingest-token-loss
  finding; set row 20 `complete` only when all items are closed or precisely sized with
  evidence; re-confirm the desktop workspace at ≥538/0.
- **Corpus SHA:** oracle re-bootstrapped this cycle (worktree slot was empty, a fresh
  worktree per the standing footgun) — `scripts/fetch-pcgen-oracle.sh --dest
  $PCGEN_REPO_DIR`, confirmed populated, `pcgen-oracle: OK
  7f818006e371188e5717fd18d74d18a420747fc6`.
- **Status:** `in-progress` (NOT `complete` — all three cycle-1 residuals remain real,
  sized, unbuilt).

## Starting state (verified, not assumed)

Worktree started on a stale lineage: `git rev-parse HEAD` was `1bb523773d` (the SD-31 PR
#374 merge commit), far behind `$PIN`. `git merge-base --is-ancestor $PIN HEAD` failed
(`BASE_BAD`). Recovered via `git reset --hard $PIN` + `git rebase origin/tranche/12`
(reported "up to date" — `origin/tranche/12`'s own tip equalled `$PIN`, i.e. cycle 1's own
commit). Re-verified (`BASE_OK`) before any edit.

Oracle slot was empty (git-ignored, fresh worktree). A first `scripts/verify.sh --only
preflight-oracle` call (run before `PCGEN_REPO_DIR` was exported) silently PASSED against
`$HOME/workspace/repos/pcgen` — the standing-precedent path this bundle's own docs say
never to reference directly — because that is the script's undocumented fallback default.
Caught this before trusting any figure: re-ran `scripts/fetch-pcgen-oracle.sh --dest
$PCGEN_REPO_DIR` explicitly, confirmed the worktree-local slot itself is now populated
(`7f818006e371188e5717fd18d74d18a420747fc6`), and used ONLY that path for every oracle
read below.

Checked `git log origin/tranche/12` for recent activity in target files at the start:
`reach_gate.rs`/`companion_pool_catalog.rs`/`spell_resolver.rs` unchanged since cycle 1's
own commit (`ccc2c4ec8a`, which IS `$PIN`); `pilot_compute/mod.rs`,
`class_feature_grant_consumer.rs`, `class_feature_pool_catalog.rs` (row 18's territory,
read this cycle but never written) last touched at `dbf2c71e2c`, also before `$PIN` — no
territory conflict at start.

## The two unowned reds (`decisions.md §27b`: "pre-existing" is not a disposition)

Both reproduced first (`cargo test --locked --bin v06_work_inventory <test> -- --exact`),
then investigated to a decision with evidence, per the brief's instruction not to assume
either is a stale pin or a real bug without checking.

### Red 1 — `e14_harness_tests::a_key_two_books_share_grounds_only_the_book_whose_corpus_was_read`

**Reproduced.** Panic: `"Ultimate Equipment has no corpus directory at all -- nothing
observed ITS record, so nothing may claim it"` — i.e. `wired` now contains
`(ultimate_equipment, "Celestial Shield")`, which the test's negative assertion forbids.

**Investigated, not assumed.** The test's own doc comment already claims `ue::
equipment_tables()`'s doc comment "documents 65 keys... deliberately EXCLUDED as
cross-book republished items — `Celestial Shield` is one of them (`Dogslicer` is the
module's own spot-checked example)". That spot-check does not generalise: read the
corpus JSON for both books' "Celestial Shield" records directly —

- ARG (`data/corpus/advanced_race_guide/equipment/arms_armor/celestial_shield.json`):
  `cost_gp: 13170`, feather-fall/overland-flight ability, full description.
- UE (`data/corpus/ultimate_equipment/equipment/celestial_shield.json`): `cost_gp: 4020`,
  `raw_bonus_chains: [COMBAT, AC, 1, TYPE=Shield]`, `description: null`,
  `completeness: "chassis_only"`.

These are not byte-identical — they are two different items sharing a display name.
Confirmed against the pinned oracle directly, not just the corpus JSON transcription:
`ue_equip_arms_armor.lst:126` — `SOURCEPAGE:p.131`, `PROFICIENCY:SHIELD|Shield (Light)`,
`COST:4020`, `ACCHECK:-1`, `SPELLFAILURE:5`, `BONUS:COMBAT|AC|1|TYPE=Shield`. Real, cited,
distinct UE content — the hand-authored table's blanket 65-key exclusion swept this key in
incorrectly (it generalised from `Dogslicer`'s real duplicate without checking each of the
other 64).

Traced why the record now surfaces: `gen_equipment_gap_tables.rs`'s complement pass
(`held` = each book's own `hand_authored_equipment_rows()` key set) does not know about
the hand table's deliberate cross-book exclusion — it correctly treats "not in UE's own
hand table" as "gap, re-derive from UE's own corpus text", and re-emits the record
book-scoped to `"UE"` in `equipment_gap_tables::ULTIMATE_EQUIPMENT_GAP_ROWS`. That is the
MORE correct behaviour under `decisions.md §27b` ("EVERYTHING" — no unit dropped because
it is inconvenient), not a regression: the hand table's original exclusion was itself the
defect, silently dropping a real, oracle-confirmed UE item.

**Decision: stale pin, not a loader bug.** Retargeted the test to the now-proven truth:
kept the ARG-grounds assertion, added the new UE-grounds-its-own-record assertion (with
the oracle citation in the failure message), and replaced the negative example with a book
that genuinely has no equipment corpus directory at all (`inner_sea_taverns`, confirmed via
`comm -23` against every `data/corpus/*/equipment` directory) rather than a book that turned
out to have real content.

**Mutation-proved**: disabled the new UE-grounds assertion (`false &&
wired.contains(...)`) → failed with the new message, for the intended reason → reverted →
green. Separately, swapped the negative-example coordinate back to the known-now-ingested
`fetchling_abilities_race.lst:32` coordinate as a second sanity check on Red 2's own logic
(see below) — not applicable to Red 1's own mutation, listed here only for cross-reference.

### Red 2 — `race_trait_grounding_tests::the_t2b_residual_population_is_never_ingested_not_a_matcher_miss`

**Reproduced.** Panic: `"fetchling_abilities_race.lst:32 must NOT be present in
data/corpus/*/race_trait -- if it now is, T2b's residual population has genuinely shrunk
by real ingestion and this test's own premise... needs re-deriving against the real
corpus"` — the test's own failure message named the correct next step.

**Investigated.** `data/corpus/bestiary_2/race_trait/fetchling/adopted_race_fetchling.json`
exists: `completeness: "full"`, real transcribed `raw_tokens` (`KEY`, `CATEGORY`, `TYPE`,
`MULT`, `CHOOSE`, `ABILITY`) matching the oracle's `fetchling_abilities_race.lst:32` line
exactly — genuine ingestion, not a stub, from a prior generic-ingest cycle. The other two
pinned samples (`acg_abilities_race.lst:9`, `arg_abilities_race.lst:2204`) remain
genuinely absent from the corpus (re-verified directly).

Scoped the check wider than the single coordinate: every `KEY:Adopted Race ~` row across
`core_essentials/races/**/*_abilities_race.lst` (51 files carry one) cross-referenced
against every ingested `race_trait` record's own `data.key`/`source` fields. **14 of 51
now genuinely closed**, 37 still not — the underlying "Adopted Race ~ <non-CRB race>"
ingestion-gap cause this test pins is still real for the great majority of its population;
only the one sampled coordinate moved.

**Decision: stale pin, not a loosening.** Retargeted the sample to
`tiefling_abilities_race.lst:37` ("Adopted Race ~ Tiefling"), confirmed still absent by the
same scan. The other two samples are untouched. This raises no bar and drops no coverage —
it moves one pinned coordinate off a cause that partially closed onto one that has not.

**Mutation-proved**: temporarily swapped the tiefling coordinate back to the known-ingested
fetchling one → failed with the real, on-point panic message → reverted → green.

### Combined result

`cargo test --locked --bin v06_work_inventory` (full binary, 359 tests): **359 passed, 0
failed** — both reds closed, no other test moved. `git diff --unified=0 HEAD --
src/bin/v06_work_inventory.rs | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|
t_[0-9a-f]{8,})'`: zero hits (`OK_NO_BUNDLE_TAGS`). `pi_scrub.normalized_term_hits()` over
the full diff: zero hits.

## Item (a): re-derivation of the 18-family class-chassis/class_features residual (`§17a`)

Re-counted fresh from `reach_gate.rs`'s own `UNREACHED_RECORD_FINDINGS` array (not
assumed from the brief or cycle 1's receipt): a Python scan of every `("<book>",
"classes", "Gap: <N>` and `("<book>", "class_features", "Gap: <N>` tuple found **exactly
17 `classes` families, 107 records total**, and `ultimate_psionics`'s `class_features`
entry is **exactly 1,573**. Both match cycle 1's and the brief's figures precisely —
confirmed, not merely repeated.

(Note for the next cycle: `UNREACHED_RECORD_FINDINGS` also carries 21 OTHER
`class_features` families beyond UPsi's, 14,004 records total — these are a pre-existing,
separately-scoped `epic-4-mechanism` population, not part of row 20's own 18-family gap;
cycle 1 and this cycle both restrict scope to the 17 `classes` families + UPsi's 1,573
specifically, per row 19 cycle 4's own original sizing. Flagged here so a future cycle
does not conflate the two populations.)

**Not built.** Sampled the smallest-record single-class book (`horror_adventures`, 1
`class` record, "Undead Phantom") to sanity-check whether any of the 17 families might be
cheaper than assumed: its corpus record carries only `MAXLEVEL: 20` — no BAB progression,
no saves, no skill points, nothing else. This suggests at least some of the 17 families
may not be conventional player-facing base classes at all (companion/summon-shaped class
shells use exactly this minimal-token pattern elsewhere in the corpus) and would need a
per-family read before any chassis work starts, not a uniform build. A real `ClassId`-
shaped chassis + character-creation/level-up picker per book remains genuinely new
per-book engineering, unchanged in kind from cycle 1's and row 19 cycle 4's own sizing,
and out of this cycle's reach.

## Item (b): the character-scoped consumer surface — investigated, not built

Read `src/rules_core/pilot_compute/mod.rs`'s `CharacterInput`/`choice_selection` seam
(`pub(crate) fn choice_selection`, line 11723 — reads a player's per-choice-set selection
off `CharacterInput.chosen.selected_choices`) and `src/rules_core/pilot_compute/
class_feature_grant_consumer.rs` (1,901 lines) per the brief's direction, before assuming
a second consumer surface was needed.

**Confirmed: this IS a real, proven, reusable character-scoped consumer pattern.**
`class_feature_grant_consumer.rs` already combines `formula_interpreter::
PcgenFormulaEvaluator` with per-character variable resolution
(`resolve_pcgen_var_chain`, `ability_modifier_seed_vars`, `class_level_variable_name`) and
is wired into `pilot_compute`'s main entry points (`compute_pilot_base_chassis`,
`build_pilot_headless_receipt`). It is exactly the shape item (b)'s companion-formula
residual needs (HD/CON/feat-possession-scoped `%N` substitution).

**But it has never been wired to companions.**
`apps/desktop/src-tauri/src/companion_catalog.rs::list_companion_catalog()` takes **zero
parameters** — confirmed by reading its signature directly. No `CharacterInput` reaches
the companion catalog anywhere in the desktop app. The seam and its proven pattern exist;
wiring them to companions is real, scoped, un-started work — building it would mean
mirroring `class_feature_grant_consumer.rs`'s pattern into a new companion-ability grant
consumer threaded through a character-scoped companion command, not inventing a second
mechanism. Precisely resized with a concrete remedy path; not built this cycle (row 18's
own territory for `pilot_compute`/`class_feature_grant_consumer.rs` — reading only, no
write, per the brief's coordination instruction, and no sibling activity on these files
since `$PIN` was confirmed at cycle start).

Zero companion records closed this item — an honest re-sizing, not a re-filed blocker.

## Item (c): the 4 `mod_only` companion fragments — coordinated against row 21, re-sized

Per the brief's explicit instruction, checked row 21's (`epic-11-ingest-token-loss`)
finding before touching anything: row 21's defect is `.MOD`-appended `BONUS:VAR` lines
colliding under one JSON key at ingest, leaving affected records with only 1-2 raw tokens
and ZERO `BONUS:VAR` (its own kanban cell names 8 `class_feature/bloodline_tracker`
records as the confirmed instance).

**Confirmed: NOT the same defect.** Read all 4 target records directly
(`data/corpus/beastiary/companion/universal_monster_rule_{change_shape,disease_
extraordinary,fast_healing,poison_extraordinary}.json` — filed under the `beastiary`
corpus directory, `CORPUS_DIR_ALIASES` maps it to engine book `bestiary_1`/display name
"beastiary1"). Each carries a real, intact token (`ASPECT: Ability Benefit|...` for
Change Shape) — never empty, never collapsed to 1-2 tokens. Row 21's token-collision shape
does not apply here; no duplicate fix attempted.

**Re-derived the real shape by reading the oracle source directly**
(`ce_abilities_familiar_race_cr.lst`): "Universal Monster Rule ~ Poison (Extraordinary)"
and its siblings are base declarations that MANY different creatures' own `.MOD` rows
modify independently, each supplying its OWN creature-specific `DESC`/`ASPECT` clause —
Viper's poison description (line 69) differs entirely from the Imp's (line 151) and the
Homunculus's (line 142). **There is no single canonical description to delta-merge onto a
context-free companion-catalog browse view** — cycle 1's sizing ("needs a real base-record
delta-merge mechanism, adapt `ingest_spells.rs`'s `build_global_base_index` pattern") does
not fit this shape, because a `.COPY=` base-lookup assumes ONE base identity per key,
where these 4 keys have MANY creature-scoped variants and no default.

**Corrected disposition**: this is the SAME shape as item (b), not a delta-merge
mechanism — both need a real creature/character context to select the correct value,
which a browse-only catalog structurally cannot supply. Re-sized accordingly (from "needs
a new delta-merge mechanism" to "blocked on the same character-scoped consumer surface as
item (b)"), not built this cycle.

## Full-sweep re-run

`v06_work_inventory` binary (root workspace): 359 passed, 0 failed (see above).
`apps/desktop/src-tauri cargo test --locked --bin codex-desktop`: this cycle's diff
touches only `src/bin/v06_work_inventory.rs` (root binary) and this row's own kanban/
progress cells — no `apps/desktop` source file was written — so no regression is
possible in that binary from this diff; re-ran it anyway per the brief's own instruction
to reconfirm ≥538/0, and it stands unchanged: **538 passed, 0 failed** (78.20s).

## Territory

`git status --porcelain` confirmed clean before every write: touched only
`src/bin/v06_work_inventory.rs`, `docs/retro/events/sd31-transcribe.jsonl` (a benign
`verify.sh` append), and this row's own kanban/progress cells — none overlapping row 18's
pool-magnitude files (`pilot_compute/mod.rs`, `class_feature_pool_catalog.rs`,
`class_feature_grant_consumer.rs`, read for item (b)'s investigation but never written)
or row 21's ingest-fix territory. Rebased on `origin/tranche/12` immediately before push
and re-ran the targeted tests after (§5/§6 requirement).

## Next-cycle plan

1. **Wire the proven `pilot_compute`/`class_feature_grant_consumer.rs` pattern to
   companions.** A new companion-ability grant consumer, mirroring the class-feature one,
   threaded through a character-scoped companion command (`list_companion_catalog` needs
   a `CharacterInput` parameter or a sibling command that takes one). This closes item (b)
   AND item (c) together — both are the same missing surface.
2. **Per-family read of all 17 `classes` families before any chassis build** — at least
   one (`horror_adventures`) may not be a conventional player-facing base class at all.
   Size each family individually rather than assuming a uniform 17x cost.
3. Row 20 stays `in-progress` under `decisions.md §10` until all three residuals reach
   zero.
