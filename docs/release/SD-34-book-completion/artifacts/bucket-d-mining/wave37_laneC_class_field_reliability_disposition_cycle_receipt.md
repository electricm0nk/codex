# Cycle — SD-34 wave 37, Lane C — `/data/class` reliability audit vs. narrow owner-override for wave 36 lane C's Next-cycle-plan item 1 (17 units, investigation only, no fix landed)

- **Commit SHA:** (this cycle's own commit, see structured report)
- **Files touched:** this receipt, `progress.md`, `kanban.md`,
  `docs/retro/events/sd34-wave37-lanec.jsonl`,
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
  (`derived_at` refresh only, from re-running `--check` after the worktree self-heal — no
  bucket, population, or evidence field changed). **No `src/` or `scripts/` file touched —
  investigation cycle, no fix landed.**
- **Identifier audit result:** N/A — no code changed this cycle.
- **Wired-integration audit result:** N/A — no code changed this cycle.
- **Acceptance criterion (verbatim from this cycle's dispatch brief):** "DISPOSITION TRACE
  (investigation, no schema/mechanism build this cycle) on wave 36 lane C's own Next-cycle-plan
  item 1 ... Determine which of the brief's own two named paths is actually viable: (a) audit
  whether `/data/class` is reliable enough corpus-wide to trust as a new owner-resolution
  signal ... or (b) the population is small enough that a narrow, per-group hardcoded owner
  override ... is safer and cheaper. Report your exact population count for this shape
  (re-derive fresh...), your recommendation with reasoning, and if option (b) is clearly the
  right call and the population is small (roughly under 20), you may implement it this
  cycle -- otherwise name it precisely for a dedicated future wave."

## Worktree base note (self-healed, not escalated)

This cycle's assigned worktree started at `ea2b3396f2` (PR #377's own merge commit, SD-33's
launch tip) — **441 commits** behind the LOCAL `tranche/14` branch's real tip (`c1580ac9ba`,
wave 36's own wave-end gate), staler even than the 30-commit gap wave 36 lane C's own receipt
self-healed from. `origin/tranche/14` (still `7ea9651b87`, wave 33 lane D) is also stale — the
LOCAL branch is the live one, same finding as wave 36 lane C's own note. Confirmed
`git merge-base --is-ancestor ea2b3396f2 tranche/14` → true (clean fast-forward), rebased
before any commit landed (`git rebase tranche/14`, zero conflicts). Concretely detectable:
`scripts/completion_atlas.py` did not exist anywhere in this worktree pre-rebase (it exists in
7 other live worktrees and the main checkout) — a hard, unmissable signal the assigned base
predated Epic 1's own tooling. Retro-logged (`incident`, `recurrence_key: wrong-base-worktree`,
`docs/retro/events/sd34-wave37-lanec.jsonl`) per `AGENTS.md` L72's "a key firing more than a
handful of times is a missing mechanism, not bad luck" — this is at least the third wave-36/37
cycle to hit this same class.

## Population — re-derived fresh, not trusted from the brief

The brief's own item 1 names three groups: Undead Savant Subschool (Arcanist), Plant Master
(Hunter archetype), Dragon Shaman (Druid archetype) — "remaining magnitude-bearing
sub-features" whose owner is traced but unresolvable via any existing matcher signal. Filtered
`docs/work-inventory.json`'s 49,438 units to `evidence == "class_feature_of_unmodelled_corpus_
class:{undead,dragon,plant}"` **and** `magnitude_token_count > 0` (the zero-magnitude siblings
in the SAME three id-families were already promoted to `text-complete` by wave 36 lane C's own
matcher fix — those are correctly excluded, they are not "remaining"):

| Group (id prefix) | Real owner (traced to `TYPE:`/corpus directory) | Magnitude-bearing remainder |
|---|---|---|
| `undead_savant_subschool*` | Arcanist (`advanced_class_guide/class_feature/arcanist_school_undead*.json`) | **1** (`undead_savant_subschool`, mag=2; its sibling `_bolster` is mag=0 but was NOT promoted by wave 36's fix — a separate, un-investigated gap, out of this cycle's assigned scope) |
| `dragon_shaman_*` | Druid (`ultimate_magic/class_feature/dragon_shaman/*.json`, 7 of 9 files) | **7** (`animal_companion`, `bonus_feats`, `druid_domain`, `nature_s_bond`, `totem_transformation`, `totemic_summons`, `wild_shape`; `dragon_bite`/`wild_empathy` already `text-complete`) |
| `plant_master_plant_focus_*` | Hunter (`ultimate_wilderness/class_feature/plant_master/plant_master_plant_focus.json`'s own parent, which independently resolves correctly via `type_facet: "Hunter Class Features..."`) | **9** (`assassin_vine`, `brambles`, `creeping_vine`, `giant_flytrap`, `mushroom`, `oak`, `shrieker`, `spore`, `water_lily`, all mag=2) |

**Total: 17 units.** (`python3 -c` filter over `docs/work-inventory.json`, script:
`/tmp/.../scratchpad` one-off, reproducible by the same `evidence`/`magnitude_token_count`
predicate above — command shown in Figures below.) Under the brief's own "roughly under 20"
threshold.

## Path (a) — `/data/class` corpus-wide reliability audit (the real work this cycle)

Read every `class_feature` corpus JSON (`data/corpus/*/class_feature/**/*.json`, `18,074`
files at this cycle's base — `find data/corpus -path "*class_feature*" -name "*.json" | wc -l`),
grouped by `(book, immediate-subdirectory)` — the natural "one archetype/subfeature-family"
unit — and counted how many DISTINCT non-null `data.class` values appear inside each group.

**Result: 118 of 3,804 such groups (3.10%) carry more than one distinct `data.class` value
within the SAME archetype/subfeature family** — reproducible via
`/tmp/claude-1000/.../scratchpad/audit_class_field.py` (committed logic: `glob` the files,
bucket by `(book, subdir)`, `Counter` the `data.class` field, filter `len(non-null-values) > 1`).
Full 118-row table captured in this cycle's own tool output; representative shape (every row
independently spot-readable against its own corpus JSON):

- **Self-referential collision** (the exact shape found in all three of item 1's own groups):
  `data.class` holds the archetype/subfeature-group's OWN display name instead of its real base
  class — e.g. `advanced_class_guide/underground_chemist`: `{'Rogue': 3, 'Underground
  Chemist': 10}`; `horror_adventures/hate_monger`: `{'Mesmerist': 5, 'Hate-Monger': 31}`;
  `ultimate_intrigue/skinshaper`: `{'Skinshaper': 24, 'Druid': 8}`.
- **Systemic Druid/Shaman dual-labelling** — not a one-off, a RECURRING pattern across at
  least 9 separate "X Shaman" archetypes, `dragon_shaman` being only one of them:
  `advanced_players_guide/{bear,eagle,lion,serpent,wolf}_shaman` (each `{'Druid': 4,
  'Shaman': 4}`), `ultimate_combat/{ape,bat,boar}_shaman` (each `{'Druid': 6, 'Shaman': 2}`),
  `ultimate_magic/{saurian,shark}_shaman` (each `{'Druid': 5, 'Shaman': 2}`), and
  `ultimate_magic/dragon_shaman` itself (`{'Druid': 7, 'Shaman': 2}`) — **10 archetypes, same
  family, same disagreement shape**, spanning 3 different books.
- Spans at least 13 distinct books: `advanced_class_guide`, `advanced_players_guide`,
  `advanced_race_guide`, `adventurers_guide`, `core_rulebook`, `horror_adventures`,
  `inner_sea_intrigue`, `inner_sea_magic`, `occult_adventures`, `ultimate_combat`,
  `ultimate_intrigue`, `ultimate_magic`, `ultimate_wilderness`.

**Conclusion: `/data/class` is NOT reliable enough corpus-wide to trust as a new
owner-resolution matcher signal.** This is not a 3-group anecdote (the population that
motivated wave 36 lane C's own "not safely generalizable" caution) — it is a systemic,
recurring pattern, confirmed at scale. Trusting it blindly would need a per-group curation
pass of comparable size to the fix it was meant to avoid, which defeats its own cost
argument. **Path (a): REJECTED.**

## Path (b) — narrow per-group hardcoded owner override

**Shape confirmed right, population confirmed small (17, under the brief's own 20
threshold) — but traced through the actual code path before implementing, and NOT
implemented this cycle.** Reasoning:

1. **Where a fix would land.** `classify()`'s `Kind::ClassFeature` arm
   (`v06_work_inventory.rs`, the `key_group_owner`/`corpus_class_collision` chain,
   current HEAD ~L12233–12323) tries, in order: modelled-class group-text match →
   `type_facet` marker → `CLASS_FEATURE_POOLS` catalog → (only then) the
   `corpus_class_names`-wide collision check that currently mis-fires for these 17 units. A
   narrow override — same discipline as `CLASS_FEATURE_POOLS`/
   `CLASS_FEATURE_POOL_FALSE_SUFFIX_MATCHES` — would need to be tried as a NEW step before
   that final collision check, keyed on the exact group text (`"undead subschool"`,
   `"dragon shaman"`, `"plant focus"`), book-scoped to avoid a future unrelated same-word
   group elsewhere in the corpus false-matching.
2. **Root cause confirmed identical across all three groups**, and it is NOT primarily a
   `/data/class` problem — it is the SAME `type_facet` formatting defect wave 36 lane C's own
   receipt already named for Undead Savant Subschool (`"ArcanistClassFeatures..."`, no space
   before `ClassFeatures`), now confirmed present in Dragon Shaman too
   (`"DruidClassFeatures.ArchetypeDruid..."`, 6 of 9 records) and in Plant Master's own pool
   MEMBERS (`type_facet: "PlantMasterPlantFocusAbility"`, no class name token at all — the
   PARENT record `plant_master_plant_focus` itself carries a correctly-spaced
   `"Hunter Class Features..."` and already resolves fine, evidenced by its own DIFFERENT,
   correct evidence string `no_explanation_id_and_no_diagnostic_names_this_feature`, not the
   collision string).
3. **Traced the actual outcome of resolving `owner` correctly, before writing any code.**
   Every one of the 17 units has `magnitude_token_count > 0` (so `text_only` is false) and none
   passes `class_feature_pool_catalog_holds`/`class_feature_standalone_catalog_holds` (none is
   a registered choice-pool member with a rendered description already served) — the SAME two
   preconditions `class_feature_owner_via_pool_catalog`'s own doc comment already proves are
   required for a recovered owner to ever change a verdict past `engine-does-not-hold`
   ("can only ever route a record to `engine-does-not-hold`... never `grounded`"). Confirmed
   directly, not assumed: resolving `owner` here changes only the evidence STRING (from
   `class_feature_of_unmodelled_corpus_class:{dragon,undead,plant}` to something like
   `class_feature_owner_matched_by_name_but_record_not_held_by_engine`, the string already
   observed on `plant_master_green_empathy`, a Hunter-owned sibling with the same shape) — the
   `status` field, and therefore the `D` bucket population `completion_atlas.py` counts,
   **does not move**.
4. **Zero bucket movement is not a new outcome in this receipt chain** — it is the identical
   verdict wave 36 lane C's own receipt already reached for the directly analogous PaDFE
   Construct/Ooze/Undead finding ("real misattribution... zero achievable bucket movement even
   if fixed") and declined to implement for exactly that reason.

Given (a) is rejected outright and (b)'s benefit is confirmed to be evidence-string accuracy
only — zero measurable `D`→anything bucket movement, against the real cost of a new
fallback-matching code path (book-scoped keys, RED→GREEN tests, the same safety-argument
documentation every existing fallback in this file carries) — implementing (b) this cycle is
**not** "clearly the right call" under this cycle's own no-schema/mechanism-build framing, even
though the population qualifies by count alone. **Named precisely below for a dedicated future
wave rather than rushed.**

## Named for a dedicated future wave (not built this cycle)

- **Exact fix shape:** a new `&[(&str, &str)]` table (`group_text_lowercase`, `owner`) —
  `("undead subschool", "arcanist")`, `("dragon shaman", "druid")`, `("plant focus", "hunter")`
  (book-scoped: `plant focus` only inside `ultimate_wilderness`, to avoid a hypothetical future
  same-word collision elsewhere) — tried via a new `.or_else()` arm inserted between
  `class_feature_owner_via_pool_catalog` and the `corpus_class_names` collision check at
  `v06_work_inventory.rs` ~L12245, matching `class_feature_owner_via_pool_catalog`'s own
  "can never widen what GROUNDS a record" safety argument verbatim (it is the same shape of
  fallback, owner-only, no new grounding path).
- **Expected outcome, stated up front so it is not re-discovered as a surprise:** 0 units close
  to DONE; the 17 units' evidence strings change to correctly name Arcanist/Druid/Hunter as
  owner instead of the bestiary pseudo-class collision word. Value is label-integrity for
  FUTURE dispatch counting (so a script that groups by
  `evidence.startswith("class_feature_of_unmodelled_corpus_class:")`'s trailing slug does not
  mistake `undead`/`dragon`/`plant` for genuine unmodelled-class candidates — confirmed this
  cycle they are already correctly excluded from sub-mechanism 5's own 60-class table, so this
  is a latent-risk fix, not an active miscount), not bucket closure.
- **Bundle with:** wave 36 lane C's own Next-cycle-plan items 1 (this item) and 3 (PaDFE's
  real total is 13, not 10) — both are the same class of "correct a misattribution, zero
  bucket movement" cleanup, best done together in one small wave rather than three separate
  ~zero-yield cycles.
- **`undead_savant_subschool_bolster` (mag=0) not promoted despite being zero-magnitude** —
  noticed but NOT investigated this cycle (out of the assigned item-1 scope, which named only
  the magnitude-bearing remainder). Flagged for whoever picks up item 1's fix: check why this
  one zero-magnitude sibling did not reach `text-complete` the way `dragon_shaman_dragon_bite`/
  `_wild_empathy` and 214 others did.

## Figures (every number, its command, its denominator)

- `population=49438 buckets=10 unclassified=0 overlap=0`, `D: 2662` — `python3
  scripts/completion_atlas.py --check`, this cycle's own final HEAD (post-rebase,
  pre-existing state, unchanged by this cycle — no code touched).
- `17` magnitude-bearing units (1 undead_savant_subschool + 7 dragon_shaman + 9
  plant_master_plant_focus) — Python filter over `docs/work-inventory.json`'s `units`:
  `evidence in {"class_feature_of_unmodelled_corpus_class:undead",
  "class_feature_of_unmodelled_corpus_class:dragon",
  "class_feature_of_unmodelled_corpus_class:plant"} and magnitude_token_count > 0`, of the
  49,438-unit population. Re-run and confirmed identical before and after this cycle's own
  rebase (18,076→18,074 total class_feature corpus files shifted by 2 from unrelated corpus
  changes across the 441-commit gap; the 17-unit figure itself did not move).
- `118` of `3,804` `(book, subdir)` groups with internal `/data/class` disagreement (`3.10%`) —
  `python3 /tmp/.../scratchpad/audit_class_field.py`, of `18,074` `class_feature` corpus JSON
  files read at this cycle's rebased HEAD (`find data/corpus -path "*class_feature*" -name
  "*.json" | wc -l`).
- `10` "X Shaman" archetypes showing the identical Druid/Shaman split, `13` distinct books
  carrying at least one inconsistent group — direct enumeration of the same script's full
  output table (all 118 rows read, not sampled).

## Row-count command output

```
$ grep -n "^| 37 |" docs/release/SD-34-book-completion/kanban.md | tail -1
| 37 | `mine-bucket-d` | 3 | wave 32, lane C (no AT-34-E# card yet) | partial | ...
```
Row 37 (`mine-bucket-d`) is the same accumulating row wave 32/35/36 all appended into (no
dedicated `AT-34-E#` card exists for generic bucket-D mining) — this cycle appends its own
sentence to that row.

## Build scope verified

No code touched this cycle (investigation only). `cargo test`/`--no-run` not re-run — no
production file changed, so there is nothing new to verify at build scope; the last verified
state is wave 36's own wave-end gate (`c1580ac9ba`, `40/40` per `progress.md`'s own prior
entry), unchanged by this cycle.

## Sweep population

N/A — no `data/corpus/**` record added, changed, or removed this cycle.

## Oracle pin

N/A — no figure in this receipt came from the pinned PCGen oracle corpus.

## Status

**complete** — the disposition trace fully resolves the brief's own (a)-vs-(b) question with a
corpus-wide audit (not a 3-group anecdote), re-derives the exact population fresh (17, not
trusted from the brief), and reaches a reasoned, traced-through-the-code decision NOT to
implement (b) this cycle (confirmed zero bucket movement, not merely suspected) — named
precisely for a dedicated future wave per the brief's own explicit alternative. No unit closed
to DONE this cycle; that is the honest, correct outcome of an investigation-only assignment
whose own optional-fix condition was evaluated and did not clear the bar.

## Movement, four buckets

- **Closure:** 0.
- **Reclassification:** 0.
- **Reachability:** 0.
- **Instrument-correction:** 1 (the brief's own unstated assumption that resolving owner would
  matter for bucket movement — traced and found to be zero-movement, same as the PaDFE
  precedent; retro-logged as a `deferral`, not a `correction`, since no prior claimed figure
  was contradicted).

## Notes (judgment calls)

- **Why this cycle's own conditional "you may implement" was not exercised despite population
  qualifying (17 < 20):** the brief's threshold is a NECESSARY, not sufficient, condition —
  it names population size as the gate on COST (a small population is cheap to hand-curate
  safely), not on VALUE. Tracing the actual code path first (rather than implementing on the
  assumption that resolving owner obviously helps) found the value is zero bucket movement,
  which the SAME cycle-chain already declined to spend a real code change on once before
  (PaDFE). Implementing here would be spending this cycle's one real-code-change budget on a
  second zero-yield fix while a corpus-wide reliability audit — the higher-value, actually
  assigned deliverable — still needed doing. Consistent with `AGENTS.md`'s "prefer the smallest
  compliant change" and this bundle's own standing precedent.
- **The `type_facet` missing-space defect is the REAL common root cause across all three
  groups**, not `/data/class` unreliability per se — `/data/class`'s corpus-wide unreliability
  is real and independently disqualifies path (a), but the actual matcher gap these three
  groups hit is the same formatting defect already named (not fixed) in wave 36 lane C's own
  receipt for Undead Savant Subschool alone. Naming this connects the three groups' fix to a
  single root cause rather than three separate coincidences, which should make the eventual
  fix (whichever shape it takes) cheaper to scope correctly.

## Next-cycle plan

1. **Implement the 3-entry owner-override table** named above, bundled with wave 36 lane C's
   PaDFE fix (item 3) — expect 0 bucket movement, evidence-string correction only for all ~20
   combined units (17 here + 3 PaDFE). RED→GREEN: assert the evidence string changes and the
   `status` field does NOT (a negative control proving no false `grounded`/`text-complete`
   verdict is fabricated, matching every existing fallback's own test discipline).
2. **`undead_savant_subschool_bolster`'s own non-promotion** (flagged above) — a small,
   separate trace: why did this one zero-magnitude sibling not reach `text-complete` when 215
   others in the same wave did.
3. **Sub-mechanism 5's own 832-unit/60-class remainder still needs re-deriving** (wave 36 lane
   C's own Next-cycle-plan item 2, unchanged, not this cycle's scope) — confirmed this cycle
   that `undead`/`dragon`/`plant` (and by the same logic `animal`/`construct`/`ooze`) are
   already correctly excluded from that 60-class table (they are handled as separate
   collision-word rows in wave 36 lane C's own disposition table, not folded into the
   genuine-new-chassis count), so the re-derivation does not need to additionally guard against
   these three groups leaking in as false "classes."
