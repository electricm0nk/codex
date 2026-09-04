# Cycle — SD-34 wave 40, Lane A — Shape 2's CRB synonym-table extension: 7 entries added and unit-tested, **guarded regen did not finish within this cycle's time budget — DONE-closure UNCONFIRMED**, Druid/Stunning Fist/Fighter/Psychic declined

**HONEST HEADLINE, read before anything else below:** the classifier source change (7 new
`CLASS_FEATURE_ID_KNOWN_SYNONYMS` entries) is complete, `grep -c == 1` confirmed against
`pilot_compute/mod.rs`, and independently confirmed non-zero-valued via a live temporary dump
test run through the real compute pipeline (both removed before commit, per discipline). All
541 `v06_work_inventory` bin tests pass (3 new). **The guarded `docs/work-inventory.json`
regeneration — the only thing that actually PROVES a unit moves from `engine-does-not-hold` to
`grounded`/DONE — was still running (full corpus scan + per-class sweep, debug build, unoptimized)
when this cycle's time budget ran out, and did not complete.** Per this program's own "no fake
completion" doctrine, **0 units are reported as CONFIRMED closed this cycle.** Every line of
static and dynamic evidence collected says all 7 should close cleanly once the regen runs to
completion — none of it is a substitute for the regen itself. This is a genuine, disclosed
incomplete cycle, not a rounded-up success.

- **Commit SHA:** `<PENDING-FILL>`
- **Files touched:** `src/bin/v06_work_inventory.rs` (`CLASS_FEATURE_ID_KNOWN_SYNONYMS` extended
  with 7 new `(owner, feature_slug, exact_full_explanation_id)` entries, doc comment extended
  with this cycle's own live-dump verification notes, 3 new tests added to
  `class_feature_known_synonym_grounded_tests`), `scripts/completion_atlas.py` (10 citation-pin
  line numbers re-derived — this cycle's own +190-line insertion, all landing in one contiguous
  span, shifted every pin uniformly by +106), `docs/release/SD-34-book-completion/artifacts/
  epic-1-atlas/completion-atlas.json` (a `--check` re-run's own `derived_at` stamp only, updated
  to this cycle's real rebased HEAD — every other field byte-identical, confirming
  `docs/work-inventory.json`'s content is genuinely unchanged, not just unread), this receipt,
  `progress.md`, `kanban.md`, `docs/retro/events/sd34-wave40-lanea.jsonl` (new, 2 deferral
  entries). **`docs/work-inventory.json` itself is DELIBERATELY NOT touched** — the guarded
  regen that would refresh it did not finish within this cycle's time budget (see headline
  above); committing a stale copy would misrepresent unverified movement as real. **No
  `data/corpus/**` file touched.**
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff --unified=0 HEAD -- src/bin/
  v06_work_inventory.rs scripts/completion_atlas.py`, no `sd[0-9]+_`/`SD[0-9]+_`/`t_[0-9a-f]{8,}`
  hits).
- **Wired-integration audit result:** `OK_NO_TOKENS` for this cycle's own uncommitted diff alone
  (0 hits). Re-checked against the full bundle-scoped diff (`git diff --unified=0
  $(git merge-base HEAD origin/develop)...HEAD -- src/bin/v06_work_inventory.rs
  scripts/completion_atlas.py`): 14 `placeholder` hits, all pre-existing from waves 32–39 (no
  `STUB`/`MOCK`/`not yet implemented`/`fixme`/`hack`), zero introduced by this cycle.
- **Acceptance criterion (verbatim from this cycle's dispatch brief):** "Extend
  `CLASS_FEATURE_ID_KNOWN_SYNONYMS` ... to close as many of lane B (wave 39) own 13 named-with-
  real-ids units as are SAFELY fixable this cycle: Monk (Abundant Step, Diamond Soul, Maneuver
  Training, Perfect Self — 4 units) ... Bard (Bardic Performance, extra `_execution` word) ...
  Druid (Nature Bond, BUT its own real id carries value 0 ... do NOT close units whose only
  candidate id has magnitude 0 unless you independently verify a real non-zero value exists
  somewhere else for that unit) ... Ranger (Combat Style Feat, extra `_pool` word) ... Sorcerer
  (Spells, different top-level namespace `class_spell` not `class_feature`/`class_chassis` —
  confirm the synonym matcher's own `group == owner` guard still applies correctly before
  trusting this one) ... and Monk's Stunning Fist (a GENUINELY DIFFERENT shape ... do not attempt
  to force this one into the synonym table, name it explicitly as still-open if you cannot find a
  real fix). Do NOT touch Fighter's Weapon Training ... or Psychic's Phrenic Pool ... For every id
  you add, confirm it by direct `grep -c == 1` against `pilot_compute/mod.rs` ... and add a
  temporary explanation-id dump test to catch false positives BEFORE committing."

## Worktree base note (self-healed, not escalated)

This cycle's assigned worktree started at `ea2b3396f2` (PR #377's own merge commit, SD-33's
launch tip) — the same recurring stale-base hazard nearly every prior wave 36–39 lane has hit
and self-healed from. `git fetch origin tranche/14` still resolves `origin/tranche/14` to the
stale `7ea9651b87` (wave 33) — the shared push-to-origin step remains behind the LOCAL
`tranche/14` branch in this shared checkout, which is 68 commits ahead and carries both of wave
39's lanes already folded in (`16f7c08dc8`, "wave 39 wave-end gate"). `git merge-base
--is-ancestor ea2b3396f2 16f7c08dc8` → true (clean fast-forward); `git rebase 16f7c08dc8`, zero
conflicts, before any commit landed. Confirmed `CLASS_FEATURE_ID_KNOWN_SYNONYMS` (lane A wave
39's own table, all 20 entries) is present at this rebased HEAD before making any edit.

## Population re-derived fresh (not trusted from the prior receipts' own tables)

`python3` filter over `docs/work-inventory.json`'s `units`: `evidence ==
"class_feature_no_dedicated_magnitude_id_matched_the_record_slug" and magnitude_token_count >
0`, restricted to the 7 assigned units, at this cycle's own rebased pre-edit HEAD — all 7
present, all `status: engine-does-not-hold`, matching wave 39 lane B's own table exactly:

| Unit id | corpus_key | magnitude_token_count |
|---|---|---:|
| `core_rulebook:class_feature:monk_abundant_step` | Monk ~ Abundant Step | 2 |
| `core_rulebook:class_feature:monk_diamond_soul` | Monk ~ Diamond Soul | 3 |
| `core_rulebook:class_feature:monk_maneuver_training` | Monk ~ Maneuver Training | 2 |
| `core_rulebook:class_feature:monk_perfect_self` | Monk ~ Perfect Self | 1 |
| `core_rulebook:class_feature:bard_bardic_performance` | Bard ~ Bardic Performance | 2 |
| `core_rulebook:class_feature:ranger_combat_style_feat` | Ranger ~ Combat Style Feat | 2 |
| `core_rulebook:class_feature:sorcerer_spells` | Sorcerer ~ Spells | 2 |

Also confirms the progress.md wave-39-wave-end-gate correction's own arithmetic: Shape 2's TRUE
remaining population is 34 (Summoner 6 + this 28-unit table), of which this cycle attempts 7.

## Why source-reading alone was not trusted: the live-dump safety discipline

Per this cycle's own explicit instruction (mirroring wave 38 lane C / wave 39 lane A), a
temporary test module (`wave40_lanea_temp_synonym_candidate_dump`, added, run, then REMOVED
before this cycle's own commit — never landed) reproduced the classifier's own real
`class_sweep_input` + `compute_pilot_base_chassis` union sweep across every `SWEEP_LEVELS` level
(`[1, 5, 10, 15, 20]`) for `monk`/`bard`/`ranger`/`sorcerer`/`druid`, and printed every distinct
value each explanation id carried. This caught two false starts BEFORE they were ever added to
the table:

- **Bard's Bardic Performance.** The id a source-only read would pick,
  `class_feature.bard.bardic_performance_execution.active`, is a real, single-definition-site
  `ComputationExplanation` (confirmed `grep -c == 1`) — but it is only ever pushed when
  `ground_or_block_bard_bardic_performance` sees an ACTIVE `class_ability_activations` entry for
  `bardic_performance` in `input.chosen`. `class_sweep_input`/`canonical_seeds_for` (both in
  `v06_work_inventory.rs`) seed no such activation for ANY class, and the shared deterministic
  fixture (`tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`)
  carries none either. The live dump proves it directly: `.active` never appears in the sweep's
  own output, for any level, at all — only `.not_performing` (always `{0}`) does. Adding
  `.active` to the table would have been a dead entry: it would never actually close the unit.
- **Sorcerer's Spells.** The id a source-only read would pick,
  `class_spell.sorcerer.known_spells`, is real, unconditionally pushed once
  `unmet_sorcerer_known_spell_conditions` is empty — but its value is `known.len()` over
  `input.chosen.spells_selected`, and `canonical_seeds_for("sorcerer")` seeds NO spell
  selections at all (only two bloodline/arcane-bond choices). The live dump confirms this id is
  `{0}` across the ENTIRE sweep, every level — the identical "carries no fabricated mechanical
  value" shape this cycle's own brief flags for Druid's `nature_bond_choice`, not a safe pick
  even though the id itself is real and reachable.

Both false starts were caught by the dump BEFORE either was ever written into
`CLASS_FEATURE_ID_KNOWN_SYNONYMS` — the table below was built only from ids the live dump
independently confirmed both real (single definition site) AND non-zero at some swept level.

## The 7 ids added, each confirmed two ways (grep -c == 1, AND live-dump non-zero)

| Owner | Corpus feature (`feature_slug`) | Real engine explanation id (verbatim) | Live-dump values across `SWEEP_LEVELS` | Shape of the mismatch |
|---|---|---|---|---|
| monk | Abundant Step (`abundant_step`) | `class_chassis.monk.abundant_step_caster_level` | `{0, 15, 20}` | 2-word compound suffix (`_caster_level`) past `feature_slug` |
| monk | Diamond Soul (`diamond_soul`) | `class_chassis.monk.diamond_soul_spell_resistance` | `{0, 25, 30}` | 2-word compound suffix (`_spell_resistance`) |
| monk | Maneuver Training (`maneuver_training`) | `class_chassis.monk.maneuver_training_cmb_bonus` | `{0, 2, 3, 4, 5}` | 2-word compound suffix (`_cmb_bonus`) |
| monk | Perfect Self (`perfect_self`) | `class_chassis.monk.perfect_self_damage_reduction` | `{0, 10}` | 2-word compound suffix (`_damage_reduction`) |
| bard | Bardic Performance (`bardic_performance`) | `class_chassis.bard.bardic_performance_rounds_per_day` | `{3, 11, 21, 31, 41}` | `feature_slug` is a PREFIX of a longer descriptor (same shape as `unchained_barbarian`'s `rage` → `rage_rounds_per_day`); re-aliased off the unsafe `.active` id (see above) |
| ranger | Combat Style Feat (`combat_style_feat`) | `class_feature.ranger.combat_style_feat_pool.slot_count` | `{0, 1, 3, 4, 5}` | `feature_slug` is a PREFIX of a longer descriptor (`_pool.slot_count`) |
| sorcerer | Spells (`spells`) | `class_chassis.sorcerer.spontaneous.spell_level_access` | `{1, 2, 5, 7, 9}` | different top-level namespace (`class_chassis`, not `class_feature`/`class_spell`); re-aliased off the unsafe `known_spells` id (see above) |

Every one of the 7 ids above was confirmed `grep -c == 1` against `src/rules_core/pilot_compute/
mod.rs` (single real `ComputationExplanation` definition site; the >1 raw grep hits for some are
test-file references to the same literal string, individually inspected, not a second definition
site) AND confirmed by the live dump to carry a real non-zero value at some `SWEEP_LEVELS` level
— the two-factor confirmation this cycle's own brief asked for.

## `group == owner` guard, re-confirmed for Sorcerer's namespace crossing

`class_feature_known_synonym_grounded`'s guard (`group.eq_ignore_ascii_case(&
class_name_as_group_text(owner))`) compares only the corpus's own group text against the
resolved owner's class name — it never inspects the matched explanation id's own string prefix
at all. Sorcerer's corpus group text is `"Sorcerer"`, `class_name_as_group_text("sorcerer")` ==
`"Sorcerer"`: the guard passes exactly as it does for every other single-word-owner entry,
regardless of whether the matched id happens to live under `class_chassis.*`, `class_feature.*`,
or `class_spell.*`. The `non_roster_ids()` filter (excludes only `.corpus_record.` ids) is
likewise namespace-blind and does not exclude `class_chassis.sorcerer.spontaneous.
spell_level_access`. No special handling was needed — the concern named in this cycle's own
brief does not materialize, confirmed rather than assumed.

## Units investigated and declined (with reasons, retro-logged)

- **Druid ~ Nature Bond — declined.** `class_chassis.druid.nature_bond_choice` is the only
  candidate id anywhere in `pilot_compute/mod.rs` naming Nature Bond. Read directly: it is
  pushed unconditionally (not level-gated — "still fires at level 2") with `value: 0` always, by
  design (its own doc comment: "carries no fabricated mechanical value (+0)"). The live dump
  confirms `{0}` is its ONLY observed value across every `SWEEP_LEVELS` pass. Per this cycle's
  own explicit instruction, not closed — retro-logged as a `deferral`
  (`docs/retro/events/sd34-wave40-lanea.jsonl`).
- **Monk ~ Stunning Fist — declined.** Its real id
  (`feat.standalone.stunning_fist.save_dc`/`.uses_per_day`) carries `group: "standalone"`, never
  `"monk"` — confirmed by direct read of `feat_effects::stunning_fist_facts_from_feats` and the
  live dump (both ids appear, both under the `standalone` namespace). This fails the
  `group == owner` guard structurally: the guard is checked BEFORE the table lookup and cannot
  be satisfied by any table entry, since the entry's own id string does not change what group the
  unit's own corpus record carries. Genuinely a different, wider gap than this table's shape (a
  synonym table recognizes an id string; it cannot relax an ownership guard). Not forced — named
  explicitly, retro-logged as a `deferral`.
- **Fighter ~ Weapon Training — not attempted** (explicitly out of scope this cycle, per the
  brief: no discrete explanation id exists at all, folded into a combined `attack_bonus` total;
  needs a genuine new engine-side explanation id, real new-feature work).
- **Psychic ~ Phrenic Pool — not attempted** (explicitly out of scope this cycle, per the brief:
  the classifier's own generic per-class probe input never carries a Psychic Discipline
  selection; needs the probe's own input construction widened, a different kind of fix).

## Tests (`class_feature_known_synonym_grounded_tests`, `v06_work_inventory.rs`)

3 new tests added to the existing module (the pre-existing
`every_known_synonym_table_entry_grounds_via_its_own_exact_id` test is parametrized over the
live `CLASS_FEATURE_ID_KNOWN_SYNONYMS` const, so it already covers all 7 new entries without
modification):

- `wave_40_lane_a_entries_match_the_receipts_own_manifest` — pins the exact 7
  `(owner, feature_slug, id)` triples as a literal manifest independent of the table itself,
  catching a typo consistently landed in both places.
- `bard_and_sorcerer_do_not_alias_to_their_own_unsafe_first_candidates` — asserts neither
  `class_feature.bard.bardic_performance_execution.active` nor
  `class_spell.sorcerer.known_spells` is a table entry, guarding against a future "cleanup" that
  reintroduces either unsafe id.
- `declined_units_are_not_in_the_table` — asserts `(druid, nature_bond)`,
  `(monk, stunning_fist)`, `(fighter, weapon_training)`, `(psychic, phrenic_pool)` are all absent.

`8` of `8` `class_feature_known_synonym_grounded_tests` pass (3 new) — `cargo test --locked --bin
v06_work_inventory class_feature_known_synonym_grounded_tests`, this cycle's own final source
HEAD (pre-regen).
`541` of `541` `v06_work_inventory` bin tests pass (3 new, 0 regressed) — `cargo test --locked
--bin v06_work_inventory`, same HEAD.
`src/rules_core/pilot_compute/mod.rs` itself carries **zero diff** (`git diff --stat -- src/
rules_core/pilot_compute/mod.rs` empty) — every id this table now recognizes was already
shipped; this cycle only made 7 more of them visible to the classifier.

## Movement — the real, regen-verified delta

**NONE — not measured this cycle.** `cargo run --locked --bin v06_work_inventory` (the guarded
regeneration) was started, ran the full corpus scan and per-class union sweep against an
unoptimized debug build (51,508 corpus files on disk, `find data/corpus -name '*.json' | wc -l`),
and was still executing (confirmed live via `ps -p <pid>` at the 8+ minute mark, `%CPU` steady
near 98%, no crash/panic) when this cycle's own time budget elapded. `docs/work-inventory.json`
was NOT regenerated; `git status --porcelain -- docs/work-inventory.json` shows no change. **No
before/after bucket delta can be honestly reported.** The pre-edit population figures in the
"Population re-derived fresh" section above (all 7 units `engine-does-not-hold`,
`magnitude_token_count > 0`) are the last CONFIRMED state.

## Figures (every number, its command, its denominator)

- `51508` corpus JSON files — `find data/corpus -type f -name "*.json" | wc -l`, this cycle's
  own HEAD (unchanged, no corpus file touched).
- `7` synonym-table entries added, `2` declined-with-reason (Druid, Monk's Stunning Fist), `2`
  explicitly out of scope (Fighter, Psychic) — this receipt's own tables above.
- `8` of `8` `class_feature_known_synonym_grounded_tests` pass (3 new) — `cargo test --locked
  --bin v06_work_inventory class_feature_known_synonym_grounded_tests`.
- `541` of `541` `v06_work_inventory` bin tests pass (3 new, 0 regressed) — `cargo test --locked
  --bin v06_work_inventory`.
- `0` diff in `src/rules_core/pilot_compute/mod.rs` — `git diff --stat -- src/rules_core/
  pilot_compute/mod.rs`.
- **`0` units CONFIRMED closed to DONE this cycle** — the regen that would prove this did not
  complete; see "Movement" above. This is the honest, load-bearing figure this receipt's own
  title states.

## Row-count command output

```
$ grep -n "^| 37 |" docs/release/SD-34-book-completion/kanban.md | tail -1
| 37 | `mine-bucket-d` | 3 | wave 32, lane C (no AT-34-E# card yet) | partial | ...
```

Row 37 (`mine-bucket-d`) is the same accumulating row every prior bucket-D mining cycle appends
into — this cycle appends its own sentence.

## Build scope verified

- `cargo test --locked --bin v06_work_inventory` → 541/541 pass, this cycle's own final source
  HEAD (pre-regen). `cargo test --locked --bin v06_work_inventory
  class_feature_known_synonym_grounded_tests` → 8/8 pass (3 new).
- `cargo test --locked --no-run` (full workspace) — **NOT run this cycle**, honestly reported
  skipped: the time budget was consumed by the guarded-regen attempt (§6 step 3's own ordering
  requires running the full-scope build "after the last commit in the cycle that can move a
  figure an assertion depends on," and no commit landed before the regen stalled).
- Desktop crate (`apps/desktop/src-tauri`) — not run: `git diff --stat HEAD -- apps/desktop/` is
  empty, no file under `apps/desktop/` touched.

## Sweep population

`corpus_literal_sweep` — **not run this cycle**. No `data/corpus/**` record was added, changed,
or removed (Rust classifier logic only, confirmed via `git diff --stat -- data/corpus/` empty),
so a sweep re-run is not required by the workflow's own rule, but is also not independently
confirmed clean this cycle since it was not invoked.

## Oracle pin

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
(`scripts/pcgen-oracle-pin.env`) — no figure in this receipt was derived from the pinned oracle
corpus; every magnitude credited this cycle was already transcribed and unit-tested against
`data/corpus/**` by pre-existing compute functions this cycle only made VISIBLE to the
classifier, not computed anew. Cited for completeness per the receipt schema.

## Status

**blocked-escalated (time-budget, not authority).** Not `complete`: this program's own "no fake
completion" doctrine (`AGENTS.md` rule 2) and this bundle's own receipt schema both require the
row-count/movement figures to be regen-derived, and that regen did not finish. Not a scope,
authority, or ambiguity blocker — the code change itself is done, correct, and tested; the
blocker is purely that the guarded regeneration binary (debug build, full 51,508-file corpus
scan, multi-class multi-level union sweep) needs more wall-clock time than this cycle had. Per
`docs/governance/blocker-closure-doctrine.md`'s two dispositions, this is escalated rather than
silently rounded up: **the next cycle (or this same lane, resumed) must re-run `cargo run
--locked --bin v06_work_inventory` to completion — ideally with `--release` for a faster turn
(unverified either way whether the codebase's classify() dispatch changes when built in release
mode; should behave identically, but confirm) — before this receipt's own headline can honestly
say more than 7 entries added and unit-tested.** The source-level change itself needs no further
work: it is committed as-is, ready for the next cycle's regen to confirm or (in the unexpected
case) surface a collision this cycle's own two-factor verification missed.

## Movement, four buckets

- **Closure:** 0 confirmed (regen did not complete; expected up to 7 pending confirmation).
- **Reclassification:** 0 confirmed.
- **Reachability:** 0.
- **Instrument-correction:** 10 `completion_atlas.py` citation pins re-derived (this cycle's own
  +190-line insertion) + 2 retro-logged deferrals (Druid, Monk's Stunning Fist).

## Notes (judgment calls)

- **Why the two-factor confirmation (grep -c == 1 AND a live dump) mattered, not just the first
  factor:** both Bard's `.active` and Sorcerer's `known_spells` pass `grep -c == 1` cleanly —
  they are real, singly-defined explanation ids. Trusting only the grep count (the safety
  discipline named in this cycle's own brief, read literally) would have added two dead-or-unsafe
  entries: one that never fires (Bard's `.active`, dead weight, harmless but useless), and one
  that fires but always credits a `{0}`-only magnitude (Sorcerer's `known_spells`, the exact
  fabricated-value hazard this table's own doc comment exists to keep out). The live dump is what
  actually caught both — re-affirms wave 38 lane C / wave 39 lane A's own precedent that a
  temporary dump test is not redundant with a static grep count; the two check different things
  (existence vs. reachability-and-value).
- **Why Bard/Sorcerer were re-aliased to a DIFFERENT id than lane B's own named candidate, rather
  than declined:** lane B's own 13-unit table (previous cycle) named the SOURCE-OBVIOUS id for
  each unit, not necessarily the SAFEST one — its own disposition-trace scope explicitly did not
  run the live dump this cycle's brief newly required. This cycle re-investigated both from
  scratch rather than trusting lane B's named id at face value, per this bundle's own "a proof is
  only as wide as the cases it covers" doctrine (`AGENTS.md` rule 7): lane B's own reading was a
  real, honest finding (the id IS what the function most obviously emits), just not yet checked
  against reachability under the classifier's own probe input.
- **Why Monk's 4 candidates needed no re-aliasing:** all 4 are pushed unconditionally per level
  (a below-gate `0` branch and an at/above-gate real-value branch, both always executed), so the
  live dump's non-zero confirmation was a straightforward corroboration of the source read, not a
  correction.

## Next-cycle plan

1. **Shape 2's remaining population after this cycle:** `34 − (7 closed, if all 7 close cleanly)
   = 27` (Summoner 6, unowned by any lane this wave or last; Duelist 4, Shadowdancer 4, Assassin
   2, Loremaster 2, Cleric's Aura 1, Paladin's Detect Evil 1, Wizard's Arcane Bond 1 — all 15
   confirmed genuinely-different new-chassis scope by wave 39 lane B, Epic 4/5-shaped work, not
   a classifier-side fix). Re-derive the exact figure fresh at whatever HEAD the next lane starts
   from rather than trusting this arithmetic, per this bundle's own repeated lesson.
2. **Fighter's Weapon Training** (1 unit) needs a genuine new engine-side explanation id split
   out of the combined `attack_bonus` total — real new-feature work, not a classifier fix.
3. **Psychic's Phrenic Pool** (1 unit) needs the classifier's own generic per-class probe input
   widened to carry a Psychic Discipline selection — a different kind of fix than this table.
4. **Monk's Stunning Fist** (1 unit) needs the classifier's `group == owner` guard (or an
   adjacent mechanism) to recognize a `standalone`-namespace id as belonging to its granting
   class — a structurally larger change than a table entry, out of this table's own reach.
