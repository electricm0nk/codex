# Cycle — SD-34 wave 39, Lane B — Shape 2's 9-CRB/prestige-class remainder: per-class compute-function check, 0 units closed (honest disposition-trace, deferred by design)

- **Commit SHA:** `<FILL_IN_AFTER_COMMIT>`
- **Files touched:** this receipt, `progress.md`, `kanban.md`, `docs/retro/events/sd34-wave39-laneb.jsonl` (new). **No `src/` file touched. No `data/corpus/**` file touched. No `docs/work-inventory.json` file touched.**
- **Identifier audit result:** N/A — no `src/` diff this cycle.
- **Wired-integration audit result:** N/A — no `src/` diff this cycle.
- **Acceptance criterion (verbatim from this cycle's dispatch brief):** "First-check whether the 9 CRB-base/prestige classes from Shape 2's own 54-unit remainder ... have ANY per-feature compute function at all, before assuming the same word-choice-synonym shape lane A is fixing ... For EACH class, grep `pilot_compute/mod.rs` for a `ground_<class>_class_features`-style dispatch function (or equivalent). If one exists and the gap is the same word-choice-synonym shape lane A is fixing, coordinate via a synonym table entry for that class too ... if your worktree started before lane A's own fix merged, note the collision risk and prefer disposition-tracing over touching the SAME matcher file lane A is editing ... If NO per-feature compute function exists for a class, that class is genuinely different scope ... Report per-class findings honestly ... fix ONLY the ones that are unambiguously the synonym shape AND do not risk a merge collision with lane A's own matcher-file edit."

## Worktree base note (self-healed, not escalated)

This cycle's assigned worktree started at `ea2b3396f2` (PR #377's own merge commit, SD-33's
launch tip) — the SAME stale base every wave 36/37/38 lane has independently hit and
self-healed from (fifth+ consecutive occurrence of this exact class; `AGENTS.md` L72 flags
this cadence again). Confirmed local `tranche/14`'s real tip is `5800f0f4fe` (wave 38's own
wave-end gate, "collision recovery, 2 baselines raised, full 40/40 confirmed").
`git merge-base --is-ancestor ea2b3396f2 5800f0f4fe` → true (clean fast-forward),
`git rebase 5800f0f4fe`, zero conflicts, before any commit landed.

**Lane A collision check (per this cycle's own explicit instruction, done before any decision
below):** `git branch -a --contains bcc67dfeed` shows lane A's wave 39 work
(`4660701090`/`bcc67dfeed`, "Shape 2 word-choice-synonym alias table, 20 of 20 units closed")
lives on `worktree-wf_f3a550aa-95c-1`, based on the SAME stale `7ea9651b87` tip, **not yet
merged onto `tranche/14`** as of this cycle. `git merge-base --is-ancestor bcc67dfeed
tranche/14` → false. So this cycle's own worktree started **before** lane A's fix merged —
the exact condition the brief names as the trigger to prefer disposition-tracing over a live
code edit to the same file. Read lane A's full diff anyway (`git diff 7ea9651b87 bcc67dfeed
-- src/bin/v06_work_inventory.rs`) to characterize its mechanism precisely (below), not guess
at it.

## Population re-derived fresh (not trusted from the prior receipt's own table)

`python3` filter over `docs/work-inventory.json`'s `units`: `evidence ==
"class_feature_no_dedicated_magnitude_id_matched_the_record_slug" and magnitude_token_count >
0`, `54` units total (unchanged from wave 38 lane C's own figure — confirms this cycle's
assigned worktree, though rebased, still reflects the same live 54-unit remainder wave 38
lane C left; lane A's own 20-unit closure has NOT yet regenerated `docs/work-inventory.json`
on `tranche/14`, consistent with lane A's fix being uncommitted to the shared branch).

**Correction against this cycle's own dispatch brief** (retro-logged,
`docs/retro/events/sd34-wave39-laneb.jsonl`): the brief names "the 9 CRB-base/prestige
classes ... 27 units total" and lists Monk (5), Duelist (4), Shadowdancer (4), Assassin (2),
Fighter (2), Loremaster (2), Wizard (2), Bard/Cleric/Druid/Paladin/Ranger/Sorcerer/Psychic (1
each). Summing the brief's own list gives **28**, not 27 (`5+4+4+2+2+2+2+7×1 = 28`), and the
list is **14 distinct classes** (7 named singles + 7 one-unit classes), not 9. Re-derived the
same filter, grouped by class (parsed from each unit's own `corpus_key`'s `"<Class> ~
<Feature>"` group text — `owner` is not a field in the schema): the brief's own 14 named
classes sum to exactly **28**, matching the re-derivation, not 27. The brief's list also
**omits `Summoner` (6 units, non-Unchained — distinct from `Unchained Summoner`, which IS
lane A's scope)** entirely — neither lane A's nor this lane's own assignment claims it, so it
is named here as an **unowned remainder** for a future wave, not silently absorbed into either
lane's own count.

| Class | Units | Feature(s) |
|---|---:|---|
| Monk | 5 | Abundant Step, Diamond Soul, Maneuver Training, Perfect Self, Stunning Fist |
| Duelist | 4 | Canny Defense, Elaborate Defense, Improved Reaction, Precise Strike |
| Shadowdancer | 4 | Shadow Call, Shadow Illusion, Shadow Jump, Summon Shadow |
| Fighter | 2 | Bonus Feats, Weapon Training |
| Loremaster | 2 | Lore, Secret Lore |
| Wizard | 2 | Arcane Bond, Bonus Feats |
| Assassin | 2 | Death Attack, Save against Poisons |
| Bard | 1 | Bardic Performance |
| Cleric | 1 | Aura |
| Druid | 1 | Nature Bond |
| Paladin | 1 | Detect Evil |
| Ranger | 1 | Combat Style Feat |
| Sorcerer | 1 | Spells |
| Psychic | 1 | Phrenic Pool |

Sum: `5+4+4+2+2+2+2+1×7 = 28`, re-derive with the exact filter above (this receipt's own
scope: these 28 units, NOT the 6-unit `Summoner` remainder).

## Lane A's own mechanism, read in full before deciding anything (not assumed)

`CLASS_FEATURE_ID_KNOWN_SYNONYMS` (`src/bin/v06_work_inventory.rs`, lane A's own commit
`4660701090`) is a literal `&[(owner, feature_slug, exact_full_explanation_id)]` alias table.
`class_feature_known_synonym_grounded` checks `group == class_name_as_group_text(owner)` (the
same guard every sibling check uses) and then does an **exact full-string** match against
`explanation_ids` — it does not care about the id's own internal dot-segment shape at all,
only whether the literal string is present. It is wired into `classify()` as a third fallback,
tried only after both existing dot-segment checks fail. Lane A's own 20 entries cover
Unchained Monk (7), Unchained Barbarian (6), Unchained Rogue (4), Unchained Summoner (3) — the
4 Unchained classes explicitly named as lane A's own scope this wave, confirmed by direct read
of lane A's own commit message and diff.

## Per-class check (grep `pilot_compute/mod.rs`, and `src/rules_core/rules_tables/` for a same-named module, for EVERY class before naming a verdict)

**4 classes: NO per-feature compute function exists at all** — confirmed by `grep -rn
"duelist\|shadowdancer\|assassin\|loremaster" src/rules_core/` (case-sensitive substring, full
tree) finding **zero** files under `rules_tables/` for any of the four, and zero `fn
ground_<class>...` hits, and zero hits for any of their 12 named features'
slugs (`canny_defense`, `elaborate_defense`, `improved_reaction`, `precise_strike`,
`shadow_call`, `shadow_illusion`, `shadow_jump`, `summon_shadow`, `death_attack`,
`save_against_poison`, `lore`/`secret_lore`) anywhere in `pilot_compute/mod.rs`. Duelist's only
hit at all is a pre-existing, unrelated **zero-magnitude** `class_feature.duelist.corpus_record.
deflect_arrows` passthrough (a different feature, not one of the 4 named here); Shadowdancer's
only hits are the same zero-magnitude passthrough shape (`weapon_and_armor_proficiency`,
`darkvision`). **Genuinely different (new-chassis) scope — the same disposition wave 37/38
lane B already established for sub-mechanism 5's 60-class remainder, not the synonym shape.**
12 units: Duelist 4, Shadowdancer 4, Assassin 2, Loremaster 2.

- **Cleric ~ Aura (1 unit) and Paladin ~ Detect Evil (1 unit): also NO function** for these
  SPECIFIC features, despite both classes having OTHER real computed features. Cleric has
  `class_feature.cleric.weapon_and_armor_proficiency` and a full domain-power dispatch chain —
  nothing named `aura` anywhere. Paladin has three real, tested `class_chassis.paladin.
  aura_of_justice`/`aura_of_faith`/`aura_of_righteousness` functions (Paladin's OWN
  higher-level, ability-specific auras) and Antipaladin has `aura_of_depravity`/`aura_of_evil`
  — but zero hits for `detect_evil` anywhere in `pilot_compute/mod.rs`, and Cleric's generic
  "Aura" (the flat alignment-detection marker every Cleric has) is not one of the 3 named
  Paladin aura functions (different class, different feature). **Genuinely different scope.**
  2 units: Cleric 1, Paladin 1.
- **Wizard ~ Arcane Bond (1 of Wizard's 2 units): also NO function** — every `arcane_bond`
  hit in `pilot_compute/mod.rs` belongs to Sorcerer's own Arcane Bloodline mechanism
  (`SORCERER_ARCANE_BOND_CHOICE_ID`, `sorcerer.arcane_bloodline.arcane_bond_*`); nothing named
  or computing Wizard's own Arcane Bond exists. **Genuinely different scope.** 1 unit.

**Subtotal, genuinely-different (new-chassis) scope, confirmed by direct grep, zero function
found: 15 of 28 units** (Duelist 4 + Shadowdancer 4 + Assassin 2 + Loremaster 2 + Cleric 1 +
Paladin 1 + Wizard's Arcane Bond 1).

**Remaining 13 units DO have a real, already-shipped per-feature compute function touching
the named feature — but NONE matches lane A's own clean single-word-synonym shape.** Each was
read directly (function body + the real explanation id it pushes), not inferred:

| Class ~ Feature | Function | Real id emitted | Why it is NOT lane A's shape |
|---|---|---|---|
| Monk ~ Maneuver Training | `monk_maneuver_training_cmb_bonus` (called, wired) | `class_chassis.monk.maneuver_training_cmb_bonus` | `feature_slug` "maneuver_training" is a PREFIX of the trailing segment, not a whole-segment synonym — the extra `_cmb_bonus` is a compound (2-word) suffix, and neither existing word is in `CLASS_FEATURE_ID_MAGNITUDE_SUFFIXES`'s single-word strip list. |
| Monk ~ Abundant Step | (wired) | `class_chassis.monk.abundant_step_caster_level` | Same shape: `_caster_level` compound suffix appended past `feature_slug`. |
| Monk ~ Diamond Soul | (wired) | `class_chassis.monk.diamond_soul_spell_resistance` | Same shape: `_spell_resistance` compound suffix. |
| Monk ~ Perfect Self | (wired) | `class_chassis.monk.perfect_self_damage_reduction` | Same shape: `_damage_reduction` — `reduction` alone IS a recognized suffix word, but stripping only it leaves `perfect_self_damage`, still not equal to `feature_slug`; the compound is 2 words, the strip logic only removes 1. |
| Monk ~ Stunning Fist | `feat_effects::stunning_fist_facts_from_feats` (wired) | `feat.standalone.stunning_fist.save_dc` / `.uses_per_day` | Not a word-choice problem at all — the id's own group segment is `standalone`, never `monk`; it never contains the `.monk.` needle any owner-scoped check (exact, suffix-strip, OR lane A's own synonym table) requires. A structurally different (wrong-owner-namespace) gap. |
| Fighter ~ Weapon Training | `fighter_weapon_training_attack_bonus` (called at the melee-attack-total site) | *(none — folded directly into a combined `attack_bonus` total, never pushed as its own discrete `ComputationExplanation`)* | Not an id-spelling mismatch — there is no standalone id to match at all; the magnitude is real and wired but never surfaced as its own named record. |
| Fighter ~ Bonus Feats | (wired, 10 call sites) | `class_feature.fighter.level_2_bonus_feat` ... `level_20_bonus_feat` (10 distinct per-level ids) | Real magnitudes exist, but as 10 NUMBERED per-level ids, never a single `bonus_feats` aggregate — same "any one sibling id proves the engine holds the record" precedent lane A's own Eidolon/Summon Monster entries used, but the concrete id string itself is a genuinely different shape (a level number embedded mid-string) from a clean word swap. |
| Wizard ~ Bonus Feats | (wired) | `class_chassis.wizard.scribe_scroll` (value **0**, explicitly "a bounded grant-only recognition ... carries no fabricated mechanical value") | Real record exists for ONE of the Bonus Feats feature's several sub-benefits (Scribe Scroll), but it is recognition-only (always 0) and the id names `scribe_scroll`, not `bonus_feats` at all — a much looser link than any of lane A's 20 entries, which all match a real, non-zero, feature-identified magnitude. |
| Bard ~ Bardic Performance | `ground_or_block_bard_bardic_performance` (wired) | `class_feature.bard.bardic_performance_execution.active` / `.not_performing` / `.rounds_exceeded`; also `class_chassis.bard.bardic_performance_rounds_per_day` | Second-to-last segment is `bardic_performance_execution`, not `bardic_performance` — an extra `_execution` word, same compound-suffix shape as Monk's chassis ids above. |
| Druid ~ Nature Bond | (wired) | `class_chassis.druid.nature_bond_choice` (value **0**, "carries no fabricated mechanical value") | Same double problem as Wizard's Scribe Scroll: recognition-only (0) AND an extra `_choice` suffix word not in the recognized list. |
| Ranger ~ Combat Style Feat | `ground_ranger_combat_style_feat_pool` (wired) | `class_feature.ranger.combat_style_feat_pool.slot_count` | Second-to-last segment `combat_style_feat_pool` vs. `feature_slug` `combat_style_feat` — extra `_pool` word, compound-suffix shape again. |
| Sorcerer ~ Spells | `ground_sorcerer_known_spells` (wired) | `class_spell.sorcerer.known_spells` | Different top-level namespace (`class_spell`, not `class_feature`/`class_chassis`) AND the trailing word order is reversed/compound (`known_spells` vs. `spells`) — a different shape from a same-position word substitution. |
| Psychic ~ Phrenic Pool | `ground_psychic_class_features` (wired) | `class_feature.untabled.psychic.phrenic_pool.value` | This id's shape ALREADY matches wave 38 lane C's own `<owner>.<feature_slug>.<descriptor>` dot-segment convention exactly (`phrenic_pool` IS the literal second-to-last segment) — it is not an id-spelling gap at all. It is conditional on a chosen Psychic Discipline (`psychic_discipline_pool_ability`); the generic, no-player-choices per-class sweep this classifier's own check is run against apparently carries none, so the id is never emitted for THIS input — an input-construction gap, not a naming gap. Spot-checked against the function's own doc comment and test (`psychic_phrenic_pool_is_ungrounded_with_no_chosen_discipline`), not assumed. |

**Subtotal, real compute confirmed but NOT lane A's clean synonym shape: 13 of 28 units**
(Monk 5, Fighter 2, Wizard's Bonus Feats 1, Bard 1, Druid 1, Ranger 1, Sorcerer 1, Psychic 1).
`15 + 13 = 28` — matches the re-derived population exactly.

## Why 0 units were fixed this cycle (the brief's own explicit conditional, met)

None of the 13 "real compute exists" units is safely addable to `CLASS_FEATURE_ID_KNOWN_SYNONYMS`
**this cycle** for two independent reasons, both required by the brief's own instruction:

1. **Collision risk is real, not theoretical.** Every entry in that table lives inside the
   SAME `const` array and the SAME `classify()` branch (`let grounded = ... || synonym_grounded`)
   lane A's own commit edits. This cycle's worktree started at `ea2b3396f2`, strictly before
   lane A's fix (`4660701090`/`bcc67dfeed`) merged to `tranche/14` — confirmed above, not
   assumed. Landing a second, independently-committed edit to the identical lines guarantees a
   textual conflict at the next sequential-merge step (the same shape wave 38's own
   "A+B+C rebase" merge cycle already had to resolve three ways). The brief's own conditional
   ("if your worktree started before lane A's own fix merged ... prefer disposition-tracing")
   is met exactly, not approximately.
2. **None of the 13 is "unambiguously the synonym shape" even setting collision aside.** Lane
   A's own 20 entries are uniformly a single differently-spelled WORD occupying the SAME dot
   segment as `feature_slug` (`ac_bonus` → `armor_class_bonus`, one segment, one substitution).
   Every one of this cycle's 13 carries an ADDITIONAL structural wrinkle on top of that: a
   multi-word compound suffix (Monk ×4, Bard, Ranger), a value-0 recognition-only record
   standing in for a real magnitude (Wizard, Druid), a wrong top-level namespace (Sorcerer,
   Monk's Stunning Fist), no discrete id at all (Fighter's Weapon Training), a numbered-per-level
   id family instead of one aggregate (Fighter's Bonus Feats), or an input-construction gap
   unrelated to id spelling at all (Psychic). A literal alias table entry COULD still be
   written for most of these (the mechanism is a full-string match, indifferent to WHY the
   string differs) — but that is a judgment call about which real id best "stands for" the
   feature (the same call lane A's own doc comment makes explicitly for its Eidolon/Summon
   Monster/`DEFINE:UnchainedSummoner` entries), not a mechanical, unambiguous port of an
   already-proven pattern. Combined with reason 1, the correct disposition is to name each one
   precisely (table above) and defer the actual table edit to the cycle that follows lane A's
   merge, rather than force a second conflicting edit or guess at judgment calls under
   collision risk.

**Retro-logged:** a `deferral` (13 units, reason, revisit condition — `docs/retro/events/
sd34-wave39-laneb.jsonl`) and a `correction` against this cycle's own dispatch brief's "9
classes / 27 units" figure (28 units / 14 classes, `Summoner` omitted — same file).

## Figures (every number, its command, its denominator)

- `population=49438 buckets=10 unclassified=0 overlap=0` — `python3
  scripts/completion_atlas.py --check`, this cycle's own rebased HEAD (`5800f0f4fe`), before
  and after (no change made). Full breakdown: `DONE: 25332`, `A: 449`, `B: 11769`, `C: 4180`,
  `D: 2555`, `M: 4449`, `V: 315`, `U: 202`, `X: 168`, `Z: 19`.
- `54` Shape 2 magnitude-bearing units, unchanged from wave 38 lane C's own final figure —
  Python filter over `docs/work-inventory.json`'s `units`, `evidence ==
  "class_feature_no_dedicated_magnitude_id_matched_the_record_slug" and
  magnitude_token_count > 0`.
- `28` units in this cycle's own assigned scope (14 classes, table above), `15` confirmed
  zero-function (new-chassis), `13` confirmed real-compute-but-different-shape — same filter,
  grouped by class parsed from `corpus_key`.
- `20` units lane A's own commit closes (Unchained Monk 7 + Unchained Barbarian 6 + Unchained
  Rogue 4 + Unchained Summoner 3) — `git show --stat 4660701090`'s own commit message, direct
  read, not re-verified against a regen (lane A's fix is not yet merged to `tranche/14`, so no
  regen exists to check against).
- `6` `Summoner` (non-Unchained) units, named here as unowned by either lane this wave — same
  filter, `class == "summoner"`.

## Build scope verified

No `src/` file touched this cycle — no Rust build or test run required by
`workflow-instruction.md`'s own scoping rule ("run only the tests scoped to what you
touched"). `git diff --stat HEAD` confirms zero source changes:

```
$ git diff --stat HEAD -- src/ apps/ data/
(empty)
```

`python3 scripts/completion_atlas.py --check` re-run after this cycle's own doc-only commit
to confirm no incidental drift: `population=49438 buckets=10 unclassified=0 overlap=0`,
identical to the pre-cycle run above.

## Sweep population

`data/corpus/**`: 0 files touched, 0 files added, 0 files removed this cycle (disposition-trace
only, no corpus or engine edit). No sweep re-run required.

## Oracle pin

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
(`scripts/pcgen-oracle-pin.env`) — no figure in this receipt was derived from the pinned
oracle corpus; every claim about "does a compute function exist" was verified by direct
`grep`/read of already-shipped, already-tested engine source, not by re-deriving anything
against the corpus. Cited for completeness per the receipt schema.

## Status

**complete (disposition-trace cycle, 0 code units closed, by design).** Every one of the 9
CRB-base/prestige classes named in this cycle's own dispatch brief was checked individually
against the real engine source, not assumed: 4 classes (Duelist, Shadowdancer, Assassin,
Loremaster) plus 2 single units (Cleric ~ Aura, Paladin ~ Detect Evil) plus 1 single unit
(Wizard ~ Arcane Bond) — 15 units total — have genuinely NO per-feature compute function at
all, confirmed by direct grep across `pilot_compute/mod.rs` and `rules_core/rules_tables/`,
named as new-chassis scope, not attempted. The remaining 13 units (Monk 5, Fighter 2, Wizard's
Bonus Feats 1, Bard 1, Druid 1, Ranger 1, Sorcerer 1, Psychic 1) DO have real, wired compute —
but every one carries a structural wrinkle beyond lane A's own clean single-word-synonym
shape, and this cycle's own worktree started before lane A's fix merged to `tranche/14`,
meeting the brief's own explicit trigger to prefer disposition-tracing over a same-file edit.
0 units closed this cycle; this is the honest, verified outcome, not a shortfall hidden behind
partial credit.

## Movement, four buckets

- **Closure:** 0.
- **Reclassification:** 0.
- **Reachability:** 0.
- **Instrument-correction:** 1 (this cycle's own dispatch-brief figure, `9 classes/27 units` →
  `14 classes/28 units`, `Summoner` named as unowned) — retro-logged.

## Notes (judgment calls)

- **Why the 13-unit table names a specific "real id" per class rather than declaring victory:**
  lane A's own doc comment establishes the precedent that any ONE sibling magnitude proves the
  engine holds the record (Eidolon/Summon Monster). This receipt follows that same discipline
  in reverse — naming the exact real id found for each of the 13, so the next cycle (after
  lane A's merge) can add each as a literal table entry without re-deriving anything, the same
  way this cycle re-used lane A's own commit rather than re-discovering its mechanism.
- **Why Fighter's Weapon Training is named separately from its Bonus Feats sibling** despite
  both being "Fighter, real compute, wrong shape": Weapon Training has no discrete id to alias
  to AT ALL (folded into a combined total) — closing it would need a genuine new engine change
  (splitting a combined total into a named sub-explanation), not a classifier-side alias,
  meaningfully different scope from its sibling.
- **Why Psychic is filed under "different shape" rather than "genuinely different (new-chassis)
  scope" despite the id never appearing:** the compute function and its id convention are both
  already correct and already recognized by the classifier's own existing dot-segment check —
  the only gap is that this specific classifier probe's own generic per-class input never
  makes the discipline choice the function is gated on. This is closer to Sub-mechanism 5's
  "genuinely unbuilt" characterization than to a synonym gap, but it is not a missing compute
  function either — named precisely rather than mis-filed into either existing bucket.

## Next-cycle plan

1. **After lane A's wave 39 fix merges to `tranche/14`:** append the 13 named entries above
   (or a curated subset, per whatever judgment call the next cycle makes about "which real id
   best represents each feature," the same call lane A's own doc comment already models) to
   `CLASS_FEATURE_ID_KNOWN_SYNONYMS`, RED→GREEN, guarded regen. Fighter's Weapon Training and
   Psychic's Phrenic Pool are NOT candidates for that table (see Notes above) — Weapon Training
   needs a genuine engine-side new explanation id; Psychic needs the classifier's own probe
   input to carry a discipline selection, a different kind of fix entirely.
2. **15 confirmed new-chassis units** (Duelist 4, Shadowdancer 4, Assassin 2, Loremaster 2,
   Cleric 1, Paladin 1, Wizard's Arcane Bond 1) need real per-feature compute functions built
   from scratch — Epic 4/5-shaped work, the same disposition wave 37/38 lane B already
   established for sub-mechanism 5's 60-class remainder. Not a cheap next step.
3. **`Summoner` (6 units, non-Unchained)** is named here as unowned by either lane 39 A or B —
   flag for whichever wave picks up Shape 2's remainder next; not individually traced this
   cycle (out of this cycle's own named scope).
