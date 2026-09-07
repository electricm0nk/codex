# Cycle 7 — Epic 4 (Ultimate Campaign to zero) / AT-34-E4-002

- **Commit SHA:** `5e13b79821`
- **Provenance.** This cycle re-derived the split fresh at HEAD before touching anything, per
  its own dispatch's explicit instruction, rather than trusting the dispatch brief's stated
  baseline (`DONE=151`, measured at a much older SHA `651966b83e`). At the actual current
  `origin/tranche/14` HEAD (`07678e0601`, before this cycle's own rebase), `python3
  scripts/completion_atlas.py --book ultimate_campaign --check` read `DONE=193 M=47 D=2 U=21
  X=2`, population 265, unclassified 0 — six prior cycles (`AT-34-E4-002_cycle_receipt.md`
  through `_6.md`) had already landed past the dispatch brief's stale baseline. The dispatch's
  own named "previous cycle" (`_cycle_receipt_3.md`, "proved no capability exists ... named the
  build and deliberately did not start it") does not match what `_cycle_receipt_3.md` actually
  says on disk (it already BUILT the capability and landed 31 units) — the true immediately-prior
  cycle is `_cycle_receipt_6.md` (fourth slice, flat `BONUS:SAVE` traits, `DONE 191→193`). This
  cycle picked up from the real, re-derived current state, not the dispatch's stale figures, and
  re-confirmed the "no character trait/drawback selection capability" finding is now moot (six
  cycles have built and progressively widened exactly that capability since).
- **Files touched:** `src/rules_core/trait_effects.rs` (+424/-14: new `TraitInitiativeBonus`/
  `TraitConcentrationBonus` structs, `INITIATIVE_TRAIT_BONUSES`/`CONCENTRATION_TRAIT_BONUSES`
  2-entry tables, `find_initiative_by_trait_id`/`find_concentration_by_trait_id`,
  `initiative_bonus_from_traits`/`concentration_bonus_from_traits`,
  `initiative_or_concentration_trait_magnitude_is_grounded_for_corpus_key`, module doc "Fifth
  slice" section + corrected "what this module does NOT cover" census, 8 new tests),
  `src/rules_core/pilot_compute/mod.rs` (+43/-0: new `ground_orphan_trait_facts`, called
  alongside `ground_orphan_feat_facts`, pushing `trait.standalone.initiative_bonus`/
  `trait.standalone.concentration_bonus` explanations), `src/bin/v06_work_inventory.rs`
  (+72/-10: fifth `.or_else` classifier fallback onto
  `initiative_or_concentration_trait_magnitude_is_grounded_for_corpus_key`, doc-comment update,
  2 new positive-classifier tests — single-pillar Tactician and dual-pillar Arcane Temper —
  negative-control comment widened to name the fifth entry point), `scripts/completion_atlas.py`
  (+1/-1: instrument-correction, the bucket-V citation line pin re-derived after this cycle's own
  insertions into `v06_work_inventory.rs` shifted it, `12914 → 12924`).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` on this cycle's own diff (against the
  tranche/14 HEAD this cycle started from, scoped to the 4 files above — zero hits). Re-run at
  the workflow-instruction's own literal §6 formula (`BASE_BRANCH=$(git merge-base HEAD
  origin/develop)` = `ea2b3396f2`, i.e. the whole bundle's diff since the `tranche/14` cut across
  `src/rules_core/`/`src/bin/`) surfaces 20 pre-existing hits, all inside `src/rules_core/
  pilot_compute/class_feature_grant_consumer.rs` — a file this cycle never touched, confirmed by
  walking each hit back to its own `diff --git` header. Identical finding to every prior
  `AT-34-E4-002` cycle's own re-run of this same wider formula (`_cycle_receipt_6.md`): legitimate
  pre-existing `sd13_*`/`sd25_*` test-file-name citations from concurrent Epic-3 work, not a
  bundle-tag leak.
- **Wired-integration audit result:** `OK_NO_TOKENS` on this cycle's own diff (same 4-file scope
  — zero hits, confirmed by re-running the exact grep against the tranche/14-HEAD-scoped diff
  alone). The wider whole-bundle-history formula surfaces hits, all pre-existing occurrences of
  the English word "placeholder" describing PCGen's own corpus token shapes (`%LIST`
  player-chosen-target placeholders, PI-redaction placeholder, CHOOSE-menu "no selection"
  placeholder rows) in doc comments/prose written by cycles 1–6 and other concurrent Epic-3 work
  — never a marker for incomplete shipping code; the one hit inside this cycle's own added text
  (module doc line ~23032 in the whole-history diff) is verbatim-identical wording already present
  before this cycle, describing the corpus-format term, not new prose.
- **Acceptance criterion (verbatim, epic-breakdown.md AT-34-E4-002):** `python3
  scripts/completion_atlas.py --book ultimate_campaign --check` exits 0 with `DONE=265 of 265`,
  every other bucket zero, plus `artifacts/epic-4-ultimate-campaign/
  ultimate-campaign-completion-manifest.json`. **Not met this cycle** — real, incremental,
  fixture-verified progress on top of six prior cycles: `DONE=196 of 265` (functional, this
  cycle's own local regen; was `193 of 265` at this cycle's start), remainder `M:44 U:21 X:2 D:2`
  = 69. The completion manifest artifact remains out of scope until every bucket clears.

## Figures + their re-derive commands

| Figure | Value | Command / denominator |
|---|---:|---|
| `ultimate_campaign` bucket split, re-derived at cycle start (this cycle's own rebase point) | `DONE=193, M=47 (trait 17 + ability 30), U=21, D=2, X=2, V=0` of 265 | `python3 scripts/completion_atlas.py --book ultimate_campaign --check` at `origin/tranche/14` HEAD `07678e0601`, superseding the dispatch brief's stale `DONE=151` baseline (six cycles had already landed) |
| Whole-tree grep re-confirming the trait-capability build already exists (this cycle did NOT re-derive from a "does not exist" premise — six prior cycles already built it) | non-zero matches | `grep -rniE 'selected_traits\|character_traits\|CharacterTrait\b' src/ apps/desktop/src-tauri/src/ apps/desktop/src/` — hundreds of hits across `trait_effects.rs`, `trait_picker.rs`, `character_input.rs`, `CreateCharacterForm.tsx`, etc.; the dispatch's cited "zero matches" finding is six cycles stale |
| `ultimate_campaign` `trait_content` M-bucket (17 remaining after cycle 6), full corpus-JSON read of every record's `BONUS` `raw_tokens`, this cycle's own fresh census | 2 flat `COMBAT\|INITIATIVE`-bearing (`trait_tactician` only-COMBAT, `trait_arcane_temper` COMBAT+CONCENTRATION), 2 flat `CONCENTRATION\|ALLSPELLS`-bearing (`trait_arcane_temper` again, `trait_desperate_resolve` only-CONCENTRATION) = **3 distinct records** (Arcane Temper counted once), + 4 ability-score-difference-formula-shaped (**corrected from cycle 6's stated 3** — `trait_precise_treatment` mixes a flat `SKILL\|Heal\|1` token with a second formula-shaped `SKILL\|Heal\|max(INT,WIS)-WIS` token and was omitted from every prior cycle's remainder census; retro-logged below) + 10 further mixed `VAR`/`SITUATION`/`ABILITYPOOL`/`CASTERLEVEL` records + 1 pre-existing corpus data gap (`trait_shadow_whispers`) = 17 | direct read of `data/corpus/ultimate_campaign/trait_generic/{trait_almost_human,trait_arcane_temper,trait_blood_of_dragons,trait_bruising_intellect,trait_deathtouched,trait_desperate_resolve,trait_eldritch_delver,trait_fate_s_favored,trait_loyalty_across_lifetimes,trait_planar_savant,trait_pragmatic_activator,trait_precise_treatment,trait_sacred_conduit,trait_self_taught_scholar,trait_tactician,trait_trustworthy}.json`'s `data.raw_tokens` (16 files; `trait_shadow_whispers` confirmed still absent from the corpus directory by any name/key search) |
| Units genuinely promoted M → DONE (`grounded`), this cycle | **3** in `ultimate_campaign` (`Trait ~ Tactician`, `Trait ~ Arcane Temper`, `Trait ~ Desperate Resolve`); 0 corpus-wide payoff elsewhere (checked, no other book's corpus carries these exact `KEY`s) | id-set diff of `docs/work-inventory.json` before (`git show <pre-cycle-HEAD>:docs/work-inventory.json`) vs. after (this cycle's own local regen): `0 added, 0 removed` units; 3 changed `status`/`evidence`, all `ingested-magnitude → grounded`, zero elsewhere — see `/tmp/claude-1000/.../scratchpad/diff_inventory.py`'s own printed diff, reproduced in Notes below |
| `ultimate_campaign` bucket state after this cycle (functional, local regen) | `DONE 193→196, M 47→44`, all other buckets unchanged (`D:2 U:21 X:2 V:0`) | `python3 scripts/completion_atlas.py --book ultimate_campaign --check` (post local regen, then `docs/work-inventory.json` restored to committed HEAD before this commit — regeneration is the wave's shared step) |
| `completion_atlas.py --check` corpus-wide (after this cycle's own local regen, before restore) | `population=49438 unclassified=0 overlap=0 done_evidence_violations=0 missing_clearing_mechanisms=0 citation_failures=0` (re-derived after fixing the shifted V-bucket pin, `12914→12924`) | `python3 scripts/completion_atlas.py --check` |
| `corpus_literal_sweep --json-out` | `48708 records examined of 51482 read, 413336 tokens compared (9 synthesized), 51469 digests checked, 0 findings CLEAN` — unchanged from cycle 6's own baseline, no `data/corpus/**` file touched this cycle | `corpus_literal_sweep --json-out <report>` |
| `derived_evaluator_fixture_check --json-out` | `1839 unit(s) cleared over 2580 fixture row(s); 0 failed; 0 not ingested` — unchanged | `derived_evaluator_fixture_check --json-out <report>` |
| Row-count command output (see below) | `3` distinct trait ids across both new tables | see next section |

## Row-count command output

```
$ awk '/pub static INITIATIVE_TRAIT_BONUSES/,/^\];/' src/rules_core/trait_effects.rs | grep -oE 'trait_id: "trait:[a-z_]+"' > /tmp/init.txt
$ awk '/pub static CONCENTRATION_TRAIT_BONUSES/,/^\];/' src/rules_core/trait_effects.rs | grep -oE 'trait_id: "trait:[a-z_]+"' > /tmp/conc.txt
$ cat /tmp/init.txt /tmp/conc.txt | sort -u
trait_id: "trait:trait_arcane_temper"
trait_id: "trait:trait_desperate_resolve"
trait_id: "trait:trait_tactician"
$ cat /tmp/init.txt /tmp/conc.txt | sort -u | wc -l
3
```

This cycle's own artifact is the two new tables; their **union of distinct `trait_id`s** (3, not
the raw 4 rows — Arcane Temper appears in both tables for its two independent tokens) is exactly
the `ultimate_campaign` `M → DONE`-bucket delta this cycle claims, per `decisions.md §4`.

## Build scope verified

`cargo build --locked --lib`: exit 0 (only pre-existing warnings, no new ones from this cycle's
code, re-verified after this cycle's own rebase onto `origin/tranche/14`'s concurrent
`AT-34-E3-002` cycle 6 commit — no conflict, different territory). `cargo test --locked --lib --
trait_effects`: **40/40 passed** (8 new: table-shape checks, no-selected-traits, single/dual-pillar
selected-trait contributions, the fixture-executed grounding check for every entry including
Arcane Temper's dual-pillar case, the ungrounded-key negative case, plus a cross-table collision
check against all four earlier pillar tables). `cargo test --locked --bin v06_work_inventory`:
**475/475 passed** (2 new positive classifier tests — single-pillar Tactician, dual-pillar Arcane
Temper — plus the existing Bruising Intellect negative control, updated in comment only, still
green). `cargo test --locked --no-run`: full workspace **exit 0** (re-run after this cycle's own
rebase, at HEAD `5e13b79821`). `apps/desktop/src-tauri` (separate cargo workspace) **not run this
cycle** — this cycle touched no file under `apps/desktop/`, so per §2.5 ("test it explicitly or
not at all") it is honestly reported skipped rather than run on an unrelated crate for no reason;
the last cycle to touch it (`_cycle_receipt_6.md`) recorded 531 passed / 28 pre-existing failures,
unrelated to any `AT-34-E4-002` territory.

## Sweep population

Three-pass pipeline, in order, **no `--allow-stamp-loss`** (local, uncommitted regen only — the
wave's shared closing cycle owns the committed `docs/work-inventory.json`):

1. `corpus_literal_sweep --json-out` → `clean:true records_examined:48708` (unchanged — no
   `data/corpus/**` file touched this cycle; `decisions.md §12` L8 does not apply).
2. `derived_evaluator_fixture_check --json-out` → `1839 unit(s) cleared over 2580 fixture row(s);
   0 failed` (unchanged).
3. `CORPUS_LITERAL_SWEEP_REPORT=... DERIVED_FIXTURE_CHECK_REPORT=... ./v06_work_inventory` → exit
   0, no stamp-loss refusal, `docs/work-inventory.json` regenerated with `0 added, 0 removed`
   units; exactly 3 changed (`status`/`evidence`), all `ultimate_campaign trait_content`. Restored
   to committed HEAD via `git restore docs/work-inventory.json
   docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` before this
   commit.

## Oracle pin

Not applicable — no figure here came from the pinned PCGen oracle checkout; every figure was
derived from the live repo's `data/corpus/` tree and this cycle's own executed fixture tests
(`trait_effects.rs`'s `every_initiative_and_concentration_entry_is_genuinely_grounded_by_
fixture_execution`, which genuinely builds fixture characters and runs them through the real
`pilot_compute::compute_pilot_base_chassis` consumer).

- **Status:** partial

## Movement, four buckets (`decisions.md §9`)

- **Closure:** 3 units in `ultimate_campaign` (`M → DONE`, via a real, fixture-executed
  standalone-fact compute path for two genuinely new pillars — initiative checks and
  concentration checks, neither computed by this engine before this cycle). Genuine
  compute-and-apply closure, not a relabelling: every entry in both new tables is re-verified by
  `every_initiative_and_concentration_entry_is_genuinely_grounded_by_fixture_execution`, which
  builds a real character selecting exactly that trait and runs it through the real
  `pilot_compute::compute_pilot_base_chassis` consumer, reading the standalone-fact explanation
  it produces — and `Trait ~ Arcane Temper` specifically is only reported grounded because BOTH
  of its independently-pillared tokens fixture-execute correctly, not just one.
- **Reclassification:** 0.
- **Reachability:** 0 (this cycle builds the compute path itself — two new standalone-fact
  producers and their consumer wiring — not a display/explanation wire onto an already-computed
  value).
- **Instrument-correction:** 1 (`completion_atlas.py`'s bucket-V citation line pin, shifted by
  this cycle's own insertions into `v06_work_inventory.rs`, `12914 → 12924`; no bucket population
  moved by this correction).

## Notes

- **Dispatch-brief staleness, corrected rather than inherited.** The dispatch that launched this
  cycle quoted a baseline (`DONE=151`, `M=89`) measured at `651966b83e` and characterized the
  "previous cycle" (`_cycle_receipt_3.md`) as having found no trait capability and NOT started the
  build. Neither matched the real repo state: six `AT-34-E4-002` cycles had already run on
  `origin/tranche/14` (this worktree started at a stale base commit, `ea2b3396f2`, and had to be
  reset onto `origin/tranche/14` before any of this cycle's own work began), and `_cycle_receipt_3.md`
  on disk already describes building the capability and landing 31 units. This cycle re-derived
  the real split (`DONE=193 M=47`) before writing any code, per its own instruction to never trust
  an inherited baseline, and proceeded from there rather than attempting to re-build already-built
  machinery.
- **Retro-logged correction of cycle 6's own remainder census:** `_cycle_receipt_6.md`'s "3
  ability-formula traits, 13 mixed-bonus-type traits" enumeration (16 items) silently omitted
  `trait_precise_treatment` (a 17th trait_content M-record) entirely — its corpus record mixes a
  flat `SKILL|Heal|1` token with a second, formula-shaped `SKILL|Heal|max(INT,WIS)-WIS` token, so
  it belongs with the ability-formula-shaped group (now 4, not 3) but was named in neither list.
  The underlying `docs/work-inventory.json` was never wrong (it always held all 17 records
  correctly) — only the prose census in that receipt undercounted by one. Corrected in this
  cycle's own module doc comment; `python3 scripts/retro.py correction` filed below.
- **`initiative_or_concentration_trait_magnitude_is_grounded_for_corpus_key`'s all-pillars-must-
  ground discipline** is this cycle's one genuinely new design decision (no earlier
  `trait_effects.rs` entry point had to handle a record with two independently-pillared `BONUS`
  tokens): a record grounds only when EVERY pillar it carries fixture-executes to its transcribed
  value, never on the strength of just one of several declared magnitudes. This is the same
  "don't over-claim a partially-covered record" discipline `trait_precise_treatment`'s own
  continued exclusion already demonstrates (its flat `Heal|1` half is a shape this module could
  trivially cover, but doing so alone would silently drop its formula half).
- **`U(21), D(2), X(2)` were not touched, reopened, or reclassified.** Verified by the inventory
  diff: zero `ultimate_campaign` units with those starting statuses appear in the 3-unit changed
  set.
- **No stubs.** Both new standalone-fact producers are real, fixture-executed compute paths
  reaching a real consumer (`pilot_compute::compute_pilot_base_chassis`'s own `explanations`
  vector) — the same idiom `ground_orphan_feat_facts` already established and this bundle's own
  doctrine cites as the correct shape for a magnitude this engine computes no integrated total
  for. No desktop UI change was needed this cycle: the existing trait picker already surfaces
  every selected trait generically (it does not need to display initiative/concentration bonuses
  specially for the record to be genuinely computed and applied — the standalone-fact channel
  IS the real, verified application, exactly as it already is for Improved Initiative/Endurance).
- **`git status --porcelain` before every write; no `git add -A`; no `git stash`.** Explicit
  `git add` of the 4 touched files only. `docs/work-inventory.json` and
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` were `git
  restore`d before this commit (both GENERATED files this cycle must not hand-commit).

## Next-cycle plan

The `ultimate_campaign` remainder is `M:44 U:21 X:2 D:2` = 69 non-DONE, 44 all in `trait_content`
(0 remaining in `ability_content` beyond the pre-existing 30 Drawback/Retrain records already
named out of scope by cycle 3's own direct reading). Named by sub-cause, from this cycle's own
fresh 16-record census (the 17th, `trait_shadow_whispers`, is a corpus data gap, not a compute
shape):
- **4 ability-score-difference-formula records** (`trait_bruising_intellect`,
  `trait_planar_savant`, `trait_pragmatic_activator`, `trait_precise_treatment`) — needs a
  `max(X,Y)-Y`-shaped formula evaluator this crate does not have; cheapest of the remaining
  shapes since all four share the identical `max(A,B)-B` structure.
- **10 further mixed records**: 3 `VAR`-only (`trait_fate_s_favored`, `trait_loyalty_across_
  lifetimes`, `trait_sacred_conduit` — the last carries 7 VAR tokens, all channel-energy-DC
  variables), 3 `SITUATION`-only (`trait_almost_human`, `trait_self_taught_scholar`,
  `trait_trustworthy` — the last also carries a flat `SKILL|Diplomacy|1` token mixed with its
  `SITUATION` Bluff bonus, same "don't over-claim a partial cover" hazard as Precise Treatment),
  2 `ABILITYPOOL`-only (`trait_blood_of_dragons`, `trait_deathtouched` — both a player CHOOSE
  among several distinct effect types, not a flat magnitude), 1 mixed `CASTERLEVEL`+`SKILL`
  (`trait_eldritch_delver` — same partial-cover hazard again, its `SKILL` half is trivially
  coverable but its `CASTERLEVEL|SUBSCHOOL.Teleportation` half needs a per-subschool caster-level
  pillar this crate does not have) — each a genuinely separate compute pillar, none sharing a
  mechanism with any other, so each is its own future slice.
- **1 corpus data gap** (`trait_shadow_whispers`) — unrelated to any compute path, not chased.
- **30 `ability_content` records** (18 Drawback incl. `default`, 10 Retrain, 2 Retraining) — house
  rule bookkeeping / GM-adjudicated narrative penalties or a different mechanic entirely
  (character-progression rebuild), out of scope per cycle 3's own direct reading, unchanged.

Re-run `python3 scripts/completion_atlas.py --book ultimate_campaign --check` after each sub-wave;
current remainder is `M:44 U:21 X:2 D:2` summing to `69 of 265` non-DONE.
