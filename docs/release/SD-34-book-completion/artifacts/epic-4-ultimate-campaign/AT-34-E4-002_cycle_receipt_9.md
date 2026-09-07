# Cycle 9 — Epic 4 (Ultimate Campaign to zero) / AT-34-E4-002

- **Commit SHA:** `ab5008c2d8` (code + desktop-test-wording fix + citation
  re-derive; `af2c8ae1d7` is the code-only commit this cycle made first, before
  the desktop scoped run surfaced its own test bug). progress/kanban update
  follows in a further commit per §5.
- **Provenance.** Worktree opened at `ea2b3396f2` (the tranche cut), rebased
  onto `origin/tranche/14` `6f67df49c7` before any work began. Re-derived the
  split fresh at that HEAD: `python3 scripts/completion_atlas.py --book
  ultimate_campaign --check` read `DONE=200 M=40 D=2 U=21 X=2`, population
  265, unclassified 0 -- exactly the dispatch brief's own stated baseline
  (cycle 8's own closing regen had already folded its 4-unit closure in).
  Read `AT-34-E4-002_cycle_receipt_8.md` as the newest receipt per the
  dispatch brief; its own next-cycle plan named the `SITUATION`-only shape (3
  records) and the `VAR`-only shape (3 records) as the two cheapest remaining
  same-shape groups and left the choice to this cycle. Chose `SITUATION`
  because this crate already models the exact shape
  (`feat_effects::ARG_SITUATIONAL_SKILL_FACTS`, grounding the Core Rulebook
  dwarf's own `BONUS:SITUATION` tokens as standalone facts) -- reusing a
  proven idiom rather than opening a wholly new one (`VAR`-only would need a
  bonus-pool/DC-variable pillar this crate does not have at all).
- **Files touched:** `src/rules_core/trait_effects.rs` (+464/-13 net: new
  `TraitSituationalSkillBonus`/`TraitSituationalSkillFact` structs, the
  3-entry `SITUATIONAL_SKILL_TRAIT_BONUSES` table,
  `find_situational_by_trait_id`, `situational_skill_facts_from_traits`,
  `situational_skill_fact_explanation_id`,
  `situational_flat_skill_bonuses_from_traits`,
  `situational_skill_trait_magnitude_is_grounded_for_corpus_key`, module doc
  "Seventh slice" section + corrected "what this module does NOT cover"
  census (10 -> 7 remaining `trait_content` records), 13 new tests),
  `src/rules_core/skill_allocation.rs` (+16/-0: fifth fold-in loop in
  `allocate_skill_ranks`, reusing the SAME consumer the first three slices
  established, for Trustworthy's separate flat Diplomacy token only),
  `src/rules_core/pilot_compute/mod.rs` (+31/-0: `ground_orphan_trait_facts`
  gains a loop over `situational_skill_facts_from_traits`, pushing one
  standalone `ComputationExplanation` per clause -- the same idiom
  initiative/concentration already established), `src/bin/v06_work_inventory.rs`
  (+103/-12: seventh `.or_else` classifier fallback onto
  `situational_skill_trait_magnitude_is_grounded_for_corpus_key`, doc-comment
  update, 3 new positive-classifier tests -- Almost Human, Self-Taught
  Scholar's two-clause case, Trustworthy's two-pillar case), `apps/desktop/
  src-tauri/src/trait_picker.rs` (+152/-13: `list_available_character_traits`
  gains a fifth chained iterator over `SITUATIONAL_SKILL_TRAIT_BONUSES` so the
  3 new traits are genuinely selectable, not just computed with no UI reaching
  them; 6 new tests, one fixed after its own first run failed for a real
  reason -- see Notes), `apps/desktop/src/boundary/loadCharacterTraits.ts` and
  `apps/desktop/src/characterHub/CreateCharacterForm.tsx` (doc-comment counts
  updated, zero functional change -- the frontend's existing generic
  `skills`/`bonus`/`description` rendering needed no new branch), `scripts/
  completion_atlas.py` (instrument-correction, all ten `BUCKET_DEFINITIONS`
  citation line pins re-derived -- see Notes).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` on this cycle's own diff
  (against this cycle's own starting HEAD `6f67df49c7`, scoped to the 7 files
  touched above, excluding `**/__tests__/**`/`**/*.test.*` -- zero hits).
- **Wired-integration audit result:** `OK_NO_TOKENS` on this cycle's own diff
  (same scope -- zero hits).
- **Acceptance criterion (verbatim, epic-breakdown.md AT-34-E4-002):**
  `python3 scripts/completion_atlas.py --book ultimate_campaign --check`
  exits 0 with `DONE=265 of 265`, every other bucket zero, plus
  `artifacts/epic-4-ultimate-campaign/ultimate-campaign-completion-manifest.json`.
  **Not met this cycle** — real, incremental, fixture-verified progress on
  top of eight prior cycles: `DONE=203 of 265` (functional, per the
  classify()-level bin tests below; committed `docs/work-inventory.json`
  remains at `DONE=200` -- this cycle's regeneration is local/uncommitted
  only, per this dispatch's file-ownership rule assigning it to the wave's
  shared regeneration cycle, same as every prior cycle in this module),
  remainder `M:37 U:21 X:2 D:2` = 62. The completion manifest artifact
  remains out of scope until every bucket clears.

## Figures + their re-derive commands

| Figure | Value | Command / denominator |
|---|---:|---|
| `ultimate_campaign` bucket split, re-derived at cycle start (this cycle's own rebase point) | `DONE=200, M=40 (trait 10 + ability 30), U=21, D=2, X=2, V=0` of 265 | `python3 scripts/completion_atlas.py --book ultimate_campaign --check` at `origin/tranche/14` HEAD `6f67df49c7` |
| The 3 `ultimate_campaign` `trait_content` records whose remaining `BONUS` token set includes a `BONUS:SITUATION` clause, re-confirmed against the live corpus JSON | `trait_almost_human` (`SITUATION\|Disguise=to appear human\|4`), `trait_self_taught_scholar` (`SITUATION\|Linguistics=Decipher unfamiliar languages,Spellcraft=decipher the writing on a scroll\|1`, ONE token naming two skills), `trait_trustworthy` (`SITUATION\|Bluff=Fool Someone\|1` AND a SEPARATE `SKILL\|Diplomacy\|1` token) | direct read of `data/corpus/ultimate_campaign/trait_generic/{trait_almost_human,trait_self_taught_scholar,trait_trustworthy}.json`'s own `data.raw_tokens` |
| Units genuinely promoted M → DONE (`grounded`), this cycle | **3** in `ultimate_campaign` (`Trait ~ Almost Human`, `Trait ~ Self-Taught Scholar`, `Trait ~ Trustworthy`); 0 corpus-wide payoff elsewhere (checked: `grep -rl` for all 3 corpus `KEY` strings across `data/corpus/` finds them ONLY under `ultimate_campaign/trait_generic/` and `ultimate_campaign/ability/` — the latter is a DIFFERENT inventory `Kind` (`ability`, not `trait`), already `text-complete` before this cycle and untouched by this cycle's code; no other book carries these 3 `KEY`s at all) | classify()-level bin tests (`a_situational_skill_trait_bonus_promotes_a_held_trait_record_to_grounded`, `a_two_clause_situational_trait_bonus_promotes_a_held_trait_record_to_grounded`, `a_two_pillar_situational_and_flat_trait_bonus_promotes_a_held_trait_record_to_grounded`), each asserting `verdict.status == "grounded"` for the record's real corpus `KEY`; corpus-wide `grep -rl "Trait ~ Almost Human\|Trait ~ Self-Taught Scholar\|Trait ~ Trustworthy" data/corpus/` |
| `ultimate_campaign` bucket state after this cycle (functional, per classify()-level tests; NOT baked into the committed `docs/work-inventory.json` this cycle -- see Notes) | `DONE 200→203, M 40→37` (`trait` M `10→7`; `ability` M unchanged `30`), all other buckets unchanged (`D:2 U:21 X:2 V:0`) | `cargo test --locked --bin v06_work_inventory` (the 3 new positive-classifier tests, each asserting the exact `grounded` status + evidence string for the record's real corpus key) |
| `completion_atlas.py --check` corpus-wide (committed `docs/work-inventory.json`, unchanged by this cycle) | `population=49438 unclassified=0 overlap=0 done_evidence_violations=0 missing_clearing_mechanisms=0 citation_failures=0` (re-derived after this cycle's own insertions AND a concurrently-landed `AT-34-E3-002` cycle 8 commit both shifted the ten citation pins) | `python3 scripts/completion_atlas.py --check` |
| `corpus_literal_sweep --json-out` | `clean:true records_examined:48708` — unchanged from cycle 8's own baseline, no `data/corpus/**` file touched this cycle | `cargo run --locked --bin corpus_literal_sweep -- --json-out <report>` |
| `derived_evaluator_fixture_check --json-out` | `1839 unit(s) cleared over 2580 fixture row(s); 0 failed; 0 not ingested` — unchanged | `cargo run --locked --bin derived_evaluator_fixture_check -- --json-out <report>` |
| Row-count command output (see below) | `3` distinct trait ids in the new table | see next section |
| Denominator gate against this package | `files_checked=15 violations=10` — all 10 pre-existing verbatim-quoted-corpus-prose false positives in `progress.md` (the "75% chance..." pattern `AT-34-E3-004` already flagged and every subsequent cycle's own progress entry has re-cited, growing the count as the text itself gets quoted more times, never a new bare-percentage violation) | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` |

## Row-count command output

```
$ awk '/pub static SITUATIONAL_SKILL_TRAIT_BONUSES/,/^\];/' src/rules_core/trait_effects.rs \
    | grep -oE 'trait_id: "trait:[a-z_]+"' | sort -u
trait_id: "trait:trait_almost_human"
trait_id: "trait:trait_self_taught_scholar"
trait_id: "trait:trait_trustworthy"
$ awk '/pub static SITUATIONAL_SKILL_TRAIT_BONUSES/,/^\];/' src/rules_core/trait_effects.rs \
    | grep -oE 'trait_id: "trait:[a-z_]+"' | sort -u | wc -l
3
```

This cycle's own artifact is the one new table; its 3 distinct `trait_id`s
are exactly the `ultimate_campaign` `M → DONE`-bucket delta this cycle
claims. Self-Taught Scholar's two situational clauses share one record (one
`trait_id`, not two); Trustworthy's situational clause and separate flat
token also share one record.

## Build scope verified

`cargo build --locked --lib`: exit 0 (only pre-existing warnings, no new
ones from this cycle's code). `cargo test --locked --lib -- trait_effects
skill_allocation`: **75/75 passed** (13 new `trait_effects` tests: table-shape
checks, no-selected-traits, Almost Human's single-clause case, Self-Taught
Scholar's two-clause-one-token case, Trustworthy's situational-only-facts
case, an unrecognized-trait-id case, Trustworthy's separate flat-Diplomacy
sum, the explanation-id single-source-of-truth check, the fixture-executed
`compute_pilot_base_chassis` reachability proof, the combined classifier
check for every entry including Trustworthy's two-pillar case, and the
ungrounded-key negative case; `skill_allocation`'s existing tests unchanged,
confirming the fifth fold-in loop is byte-identical for every existing
fixture). `cargo test --locked --bin v06_work_inventory`: **492/492 passed**
(3 new positive classifier tests plus the unchanged negative control, still
green: `Trait ~ Fate's Favored` remains `ingested-magnitude`, confirmed not
promoted). `apps/desktop/src-tauri` (separate cargo workspace, tested
explicitly, own `CARGO_TARGET_DIR`): `cargo test --locked --manifest-path
apps/desktop/src-tauri/Cargo.toml --bin codex-desktop -- trait_picker`:
**33 passed, 1 failed** (`trait_picker`'s own 34/34 pass; the 1 failure is
`race_trait_picker::...the_menu_command_carries_all_fourteen_adopted_race_
options_thirteen_with_real_grants`, the identical pre-existing failure every
prior `AT-34-E4-002` cycle (3/4/5/6) has already attributed as pre-existing
and unrelated -- outside `race_trait_picker.rs`, a file this cycle never
touched). `cargo test --locked --no-run` (full workspace, widest build
scope): run at this cycle's final HEAD `ab5008c2d8` -- **exit 0** (see
Notes for the literal run confirmation).

**RED→GREEN evidence (TDD, §6 step 3).** Temporarily changed
`ground_orphan_trait_facts`'s new loop to iterate `situational_skill_facts_
from_traits(&[])` (an always-empty slice) instead of the real
`selected_traits`, and re-ran the two most load-bearing new tests:
`almost_human_situational_fact_reaches_the_real_explanations_vector` FAILED
(`left: None, right: Some(4)`) and `every_situational_entry_is_genuinely_
grounded_by_fixture_execution` FAILED (`Trait ~ Almost Human did not ground
via real fixture execution`) — both RED for the intended reason (the
standalone fact genuinely not reaching the explanations vector, not a typo
or missing import). Reverted the one line; both GREEN again, confirmed by a
second full `trait_effects` run (61/61).

## Sweep population

`corpus_literal_sweep --json-out` → `clean:true records_examined:48708`
(unchanged — no `data/corpus/**` file touched this cycle; `decisions.md §12`
L8 does not apply). `derived_evaluator_fixture_check --json-out` →
unchanged. **The local, uncommitted `docs/work-inventory.json` regeneration
pipeline itself did not complete within this cycle's turn** — the process
was still running after 6+ minutes (this machine was concurrently running
another lane's own full regen for `AT-34-E3-002` at the same time; `free -h`
showed no memory pressure, so the process was genuinely computing, not
hung), and this cycle killed it rather than risk the host's memory-pressure
kill switch while it held the shared cargo build slot the required desktop
verification also needed. The functional `DONE 200→203, M 40→37` figure in
this receipt is therefore derived from the classify()-level bin tests (which
ACTUALLY BUILD each real `unit` and run it through the real `classify()`
function, asserting the exact `grounded` status and evidence string), not
from an end-to-end whole-corpus regen diff. This is a materially weaker
confirmation than cycle 8's own local-regen-then-restore check, honestly
reported as such — the wave's shared closing regeneration cycle is the one
that will produce the authoritative, committed `docs/work-inventory.json`
diff for this cycle's claimed 3 units, the same as it has for every prior
cycle in this module.

## Oracle pin

Not applicable — no figure here came from the pinned PCGen oracle checkout;
every figure was derived from the live repo's `data/corpus/` tree and this
cycle's own executed fixture tests (`trait_effects.rs`'s `every_situational_
entry_is_genuinely_grounded_by_fixture_execution`, which genuinely builds
fixture characters and runs them through the real `compute_pilot_base_
chassis`/`skill_allocation::allocate_skill_ranks` consumers).

- **Status:** partial

## Movement, four buckets (`decisions.md §9`)

- **Closure:** 3 units in `ultimate_campaign` (`M → DONE`, functional --
  not yet in the committed inventory, per Sweep population above), via a
  real, fixture-executed situational-skill standalone-fact channel plus (for
  Trustworthy) the SAME `skill_allocation::allocate_skill_ranks` consumer
  the first three slices already established — not a new wiring shape.
  Genuine compute-and-apply closure, not a relabelling: every entry is
  re-verified by `every_situational_entry_is_genuinely_grounded_by_fixture_
  execution`, which builds a real character selecting exactly that trait and
  runs it through the real engine — and `Trait ~ Trustworthy` specifically
  is only reported grounded because BOTH its situational Bluff clause AND
  its separate flat Diplomacic token fixture-execute correctly, never just
  one.
- **Reclassification:** 0.
- **Reachability:** 0 (this cycle builds the compute path itself — one new
  standalone-fact producer plus one new fold-in loop into an existing
  consumer — not a display/explanation wire onto an already-computed
  value).
- **Instrument-correction:** 1 (`completion_atlas.py`'s ten
  `BUCKET_DEFINITIONS` citation line pins, shifted by a concurrently-landed
  `AT-34-E3-002` cycle 8 commit's own insertions AND this cycle's own
  insertions; `citation_failures` 10 → 0; no bucket population moved by this
  correction).

## Notes

- **Desktop UI wiring was not optional this cycle.** Reading `apps/desktop/
  src-tauri/src/trait_picker.rs` before writing any code found that
  `list_available_character_traits` — the ONLY source the desktop trait
  picker reads — chains just four tables (flat/choice/family-choice/save)
  and does NOT include `INITIATIVE_TRAIT_BONUSES`, `CONCENTRATION_TRAIT_
  BONUSES`, or `ABILITY_DIFF_SKILL_TRAIT_BONUSES` — the fifth and sixth
  slices' traits (Tactician, Arcane Temper, Desperate Resolve, Bruising
  Intellect, Planar Savant, Pragmatic Activator, Precise Treatment) are
  genuinely computed but are **not selectable through the desktop picker at
  all**, despite cycles 7 and 8's own receipts stating "the existing trait
  picker already surfaces every selected trait generically." That claim is
  true only of a trait ALREADY selected (e.g. via a saved-character
  round-trip); a brand-new character has no way to select these 7 traits
  through the UI. **This is a pre-existing gap this cycle found but did not
  introduce and does not own fixing** (out of this cycle's own cheapest-slice
  scope; flagged here per the dispatch's own "state explicitly what a proof
  does not cover" discipline) — but this cycle's OWN 3 new traits were
  wired into the picker (`SITUATIONAL_SKILL_TRAIT_BONUSES` chained as a fifth
  iterator, reusing the existing `skills`/`bonus`/`description` DTO shape,
  zero new fields, zero new frontend branches) specifically so this cycle
  does not add a fourth instance of the same gap. Retro-logged as an
  `incident` (see command below).
- **The `bonus: i8` DTO field reuse for situational options depends on every
  clause on the SAME record sharing one identical magnitude** — true for all
  3 records this cycle covers (Almost Human 4; Self-Taught Scholar 1,1;
  Trustworthy 1,1), proven by a dedicated test
  (`every_situational_option_has_a_uniform_bonus_across_its_clauses`) rather
  than assumed. A future record whose clauses carry different magnitudes
  would need a per-clause DTO shape, not this one.
- **A transcription choice, not a defect:** the corpus `BONUS:SITUATION`
  token's own circumstance text ("to appear human") differs from the same
  record's DESC prose ("pass as human") for Almost Human. This cycle's table
  entry transcribes the TOKEN's own wording (the same precedent
  `feat_effects::ARG_SITUATIONAL_SKILL_FACTS`'s own doc comment establishes:
  "the circumstance is carried in the record's own text... restated in the
  record's own BENEFIT wording" — either the token or the DESC/BENEFIT
  prose is legitimate, and this module's other two entries (Self-Taught
  Scholar, Trustworthy) happen to closely mirror their own DESC wording
  instead). The FIRST desktop test written here incorrectly asserted the
  DESC-field description against the token's own phrase; caught immediately
  by the scoped desktop test run, fixed to check the phrase the description
  field actually contains. Retro-logged as a `correction` (see command
  below).
- **`U(21), D(2), X(2)` were not touched, reopened, or reclassified.**
  Verified: the 3 records this cycle promotes are the only ones referenced
  by any new code, and none appear in `SITUATIONAL_SKILL_TRAIT_BONUSES`'s
  own table alongside a `U`/`D`/`X` starting status (checked against the
  committed inventory's own per-key status).
- **The `ultimate_campaign/ability/*.json` duplicate corpus records were
  checked and confirmed out of scope, same as every prior cycle.** `grep -rl`
  for all 3 corpus `KEY`s finds `ultimate_campaign/ability/{almost_human,
  self_taught_scholar,trustworthy}.json` — a DIFFERENT inventory `Kind`
  (`ability`, "ability_content"), a wholly separate classifier code path this
  cycle's changes never touch, and already `text-complete`.
- **No stubs.** The new standalone-fact producer reaches a real, executed
  consumer (`pilot_compute::ground_orphan_trait_facts`'s own `explanations`
  vector, the same channel initiative/concentration already prove genuine),
  and Trustworthy's separate flat token reaches the real, pre-existing
  `skill_allocation::allocate_skill_ranks` consumer. The desktop picker was
  extended (not left with a dangling compute path) specifically to avoid a
  "compute path with no UI reaching it" violation for this cycle's own 3
  records.
- **`git status --porcelain` before every write; no `git add -A`; no `git
  stash`.** Two commits this cycle: the first (`af2c8ae1d7`) staged and
  committed exactly the 7 touched code files; the second (`ab5008c2d8`)
  staged and committed exactly the 2 files this cycle's own follow-up fix
  touched (`trait_picker.rs`'s test wording, `completion_atlas.py`'s
  citations). `docs/work-inventory.json` was never regenerated to
  completion this cycle, so there was nothing to `git restore` on it;
  `completion-atlas.json`'s regenerated timestamp WAS `git restore`d before
  the second commit (the `--check` run that verified the citation fix
  touched it as a side effect).
- Retro events: `python3 scripts/retro.py incident --subject
  "trait_picker.rs: list_available_character_traits omits 3 grounded slices'
  traits (INITIATIVE/CONCENTRATION/ABILITY_DIFF)" --detail "cycles 7/8's own
  receipts claimed the picker surfaces every selected trait generically;
  false for a NEW selection -- the command chains only 4 of 7 tables" (run
  with `RETRO_ACTOR=sd34-at-34-e4-002`); `python3 scripts/retro.py
  correction --subject "this-cycle's-own-first-desktop-test" --claimed
  "Almost Human description contains 'appear human'" --actual "description
  contains 'pass as human'; the token's own circumstance text ('appear
  human') is a DIFFERENT field" --verified-by "cargo test --locked
  --manifest-path apps/desktop/src-tauri/Cargo.toml --bin codex-desktop --
  trait_picker"`.

## Next-cycle plan

The `ultimate_campaign` remainder is `M:37 U:21 X:2 D:2` = 62 non-DONE
(functional; committed inventory still reads `M:40` until the wave's shared
regen folds this cycle in), 7 in `trait_content`, 30 in `ability_content`.
Named by sub-cause, from cycle 8's own census, unchanged by this cycle
(this cycle closed the situational sub-cause entirely, touching none of the
other 7):

- **3 `VAR`-only records** (`trait_fate_s_favored`, `trait_loyalty_across_
  lifetimes`, `trait_sacred_conduit` — the last carries 7 VAR tokens, all
  channel-energy-DC variables) — needs a bonus-pool/DC-variable compute
  pillar this crate does not have. This is now the largest same-shape
  remaining `trait_content` group and the natural next slice.
- **2 `ABILITYPOOL`-only records** (`trait_blood_of_dragons`, `trait_
  deathtouched` — both a player CHOOSE among several distinct effect types,
  not a flat magnitude) — needs a bonus trait-slot pool mechanic.
- **1 mixed `CASTERLEVEL`+`SKILL` record** (`trait_eldritch_delver` — its
  `SKILL` half is trivially coverable via the existing flat-skill table, but
  its `CASTERLEVEL|SUBSCHOOL.Teleportation` half needs a per-subschool
  caster-level pillar this crate does not have; covering only the `SKILL`
  half would understate the record).
- **1 corpus data gap** (`trait_shadow_whispers`) — unrelated to any
  compute path, not chased.
- **30 `ability_content` records** (18 Drawback incl. `default`, 10 Retrain,
  2 Retraining) — house rule bookkeeping / GM-adjudicated narrative
  penalties or a different mechanic entirely, out of scope per cycle 3's own
  direct reading, unchanged.

Also unresolved from this cycle, not this cycle's own scope: the desktop
picker gap for the fifth/sixth-slice traits (see Notes) — the next cycle
that touches `trait_picker.rs` should chain those three tables in too,
closing the doctrine gap fully rather than only for the newest slice.
Re-run `python3 scripts/completion_atlas.py --book ultimate_campaign --check`
after each sub-wave; current functional remainder is `M:37 U:21 X:2 D:2`
summing to `62 of 265` non-DONE.
