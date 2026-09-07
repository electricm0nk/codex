# Cycle — SD-34 wave 37, Lane A — domain-vs-class_feature dual-representation mechanism gap (item 5), 2 of 7 units genuinely closable this cycle, 5 honestly named unclosed

- **Commit SHA:** (local, this worktree — see structured report)
- **Files touched:** `src/rules_core/pilot_compute/domain_power.rs` (new catalog
  entry + `grounds_self_application` field + 2 new tests + bridge function),
  `src/rules_core/pilot_compute/mod.rs` (new selection/ability-id constants,
  gated both dispatch call sites, 2 new tests, fixed a real corpus-citation
  bug in the Inquisitor diagnostic), `src/bin/v06_work_inventory.rs` (new
  classify() check + 2 new tests), `scripts/completion_atlas.py` (4 shifted
  citation pins re-derived), `docs/work-inventory.json` (guarded regen),
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
  (regenerated snapshot), this receipt, `progress.md`, `kanban.md`.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff --unified=0
  c1580ac9ba36...HEAD -- src/rules_core/pilot_compute/domain_power.rs
  src/rules_core/pilot_compute/mod.rs src/bin/v06_work_inventory.rs
  scripts/completion_atlas.py`, no `sd[0-9]+_`/`SD[0-9]+_`/`t_[0-9a-f]{8,}`
  hits).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff, no
  `STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack`).
- **Acceptance criterion (verbatim from this cycle's dispatch brief):** "Close
  wave 36 lane C own named Next-cycle-plan item 5 (7 units): the
  'domain-vs-class_feature dual-representation' shape — Dragon Subdomain ~
  Dragonbreath, Undead Subdomain ~ Death Kiss, Construct Subdomain ~ Animate
  Servant, and their kin... Find each of the 7 units own corpus record...
  read what real PF1 mechanic each one encodes... check whether an existing
  domain-granted-power grounding precedent exists anywhere in
  `src/rules_core/`... reuse a precedented idiom if one fits the same shape...
  Ground each with real corpus-quoted text, no fabricated magnitude."

## Re-deriving the real 7-unit population (the brief names 3 examples, not all 7)

The brief's 3 named examples were the wave 36 lane C receipt's disposition-table
rows tagged "domain-vs-class_feature dual representation". Re-checked against
the CURRENT `docs/work-inventory.json` (post wave-36 wave-end-gate,
`c1580ac9ba`) rather than trusting that table's own pre-regen snapshot — the
same "figures written before establishing them are provisional" discipline
`decisions.md §8` names. Dragonbreath's own class_feature records (all 4
representations, across `inner_sea_world_guide`/`bestiary_6`/
`ultimate_wilderness`) had ALREADY closed to `text-complete` as an emergent
side effect of wave 36 lane A/C's own matcher fix (confirmed:
`inner_sea_world_guide:class_feature:dragon_subdomain_dragonbreath` status
`text-complete`) — so re-deriving from the live tree, not the stale table, was
load-bearing here.

The real remaining 7-unit population (the domain-kind header record for each
of the 3 named subdomains, plus each subdomain's 2-way class_feature
duplicate representation for Undead/Construct — Dragon's own class_feature
duplicates are already DONE):

| # | Unit id | Kind | Status before this cycle |
|---|---|---|---|
| 1 | `advanced_players_guide:domain:construct_subdomain` | domain | `engine-does-not-hold` |
| 2 | `advanced_players_guide:domain:undead_subdomain` | domain | `engine-does-not-hold` |
| 3 | `inner_sea_world_guide:domain:dragon_subdomain` | domain | `ingested-magnitude` |
| 4 | `advanced_players_guide:class_feature:construct_subdomain_animate_servant` | class_feature | `engine-does-not-hold` |
| 5 | `advanced_players_guide:class_feature:domain_power_animate_servant` | class_feature | `engine-does-not-hold` |
| 6 | `advanced_players_guide:class_feature:undead_subdomain_death_s_kiss` | class_feature | `engine-does-not-hold` |
| 7 | `advanced_players_guide:class_feature:domain_power_death_s_kiss` | class_feature | `engine-does-not-hold` |

Re-derive: `python3 -c "import json; d=json.load(open('docs/work-inventory.json'));
units={u['id']:u for u in d['units']}; [print(i, units[i]['status'])
for i in [...the 7 ids above...]]"`.

## The precedented idiom found (per the brief's own instruction to check first)

Grepped `src/rules_core/` for an existing domain-granted-power grounding
precedent before writing anything new, per the brief. Found one:
`domain_power::DOMAIN_POWER_CATALOG` (`pilot_compute/domain_power.rs`) — a
formula-transcription catalog, live since SD-31 wave 25/26, already grounding
5 CRB domains (Good/War/Strength/Destruction/Glory) for both Cleric
(`explain_cleric_level1_spell_baseline`) and Inquisitor
(`ground_or_block_inquisitor_domain_power`), both of which read the catalog
**generically** (`resolve_domain_power(selection_id)`, no per-domain match
arms) — confirmed by direct read of both dispatch functions before writing
anything, per this cycle's own "prove before you extend" bar. Extending the
catalog with one more entry needed zero changes to either dispatch function's
own selection-routing logic — only the shared explanation-emission block
needed a new guard (see below).

## What the shape actually is, unit by unit — and which 2 genuinely fit the precedent

**Death's Kiss (Undead Subdomain, units 6+7) — fits, GROUNDED for real.**
`data/corpus/advanced_players_guide/class_feature/undead_subdomain/death_s_kiss.json`'s
own `DESC` token: *"You can cause a creature to take on some of the traits of
the undead with a melee touch attack. Touched creatures are treated as undead
for the purposes of effects that heal or cause damage based on positive and
negative energy. This effect lasts for %1 rounds... You can use this ability
%2 times per day.|max(1,DomainLVL/2)|DomainDeathTimes"* — a single, non-dice,
non-multi-`DESC`-token formula, the exact shape `DOMAIN_POWER_CATALOG` already
covers. `DomainDeathTimes` chains to the shared `3+WIS` formula
(`data/corpus/core_rulebook/class_feature/death/death.json`'s own
`BONUS:VAR|DomainDeathTimes|DomainPowerTimes|TYPE=Domain` token, confirmed by
direct read) — the same chain Good/War/Strength/Destruction/Glory already
ride. **Real difference from every prior entry**: Death's Kiss's own `%1`
formula slot is the power's **effect DURATION in rounds**, not a flat
combat/skill/save bonus — Good's own doc comment already establishes the
catalog's shared explanation sentence as `"a +{magnitude} {label}
{duration}"`, which for a 5-round DURATION would read `"a +2 rounds of
undead-traits self-application for..."` — a plausible-looking but WRONG
sentence (the exact failure mode `AGENTS.md` rule 7 and the catalog's own
Destructive Smite/Touch of Glory precedent both name). **Fix**: added
`DomainPowerSpec::grounds_self_application: bool` (`true` for all 5
pre-existing entries, `false` for Death's Kiss) gating the
magnitude/activation-state block in BOTH dispatch functions — `uses_per_day`
(a fact independent of what the power's own effect actually does) is always
computed and reported; the misleading bonus sentence is simply never emitted
for Death's Kiss. This is a refusal-to-fabricate, not a smaller feature: the
record is `grounded` on its real, honestly-computed uses-per-day count alone.

**Animate Servant (Construct Subdomain, units 1, 4, 5) — does NOT fit, NOT
built.** `data/corpus/advanced_players_guide/class_feature/construct_subdomain/animate_servant.json`'s
own `DESC`: *"At 8th level, as a standard action, you can give life to
inanimate objects. This ability functions as animate objects using your
cleric level as the caster level. You can use this ability %1 times per
day.|DomainArtificeLVL/4-1"*. Two real, disqualifying differences from every
catalog entry including Death's Kiss: (a) its ONE formula slot is itself the
**uses-per-day** count, and it is `DomainArtificeLVL/4-1` — a real,
DIFFERENT-from-`3+WIS` formula the catalog's shared `domain_power_uses_per_day`
function cannot represent without becoming per-spec parameterized (a genuine,
larger structural change, not this cycle's narrow fix); (b) its real effect is
"cast animate objects" (a spell-like ability), which has no self-application
buff magnitude at all to ground — there is no honest number this catalog's
existing shape can report for it. Named precisely for a future cycle: the
catalog needs a `uses_per_day_formula: Option<&str>` per-spec override before
Animate Servant (or any OTHER non-`3+WIS` domain power) can be added without
fabricating its uses/day count.

**Dragon Subdomain's own granted power, Venomous Stare (unit 3's blocker) —
does NOT fit, NOT built, correctly excluded by the catalog's own existing
design.** `data/corpus/inner_sea_world_guide/class_feature/scalykind_domain/venomous_stare.json`
carries TWO `DESC` tokens gated by `PREVARLT`/`PREVARGTEQ` (a level-gated
formula variant) and is an enemy-facing Will-save gaze attack, not a
beneficial self/touch effect — both shapes the catalog's own module doc
already names as deliberately excluded (the same rationale already covers
Evil/Darkness/Madness). This is not a gap; it is the design boundary working
as documented. Confirmed by direct read, not assumed from the wave 36 receipt.

**Dragon Subdomain's own domain header, unit 3** — carries a real
`DomainScalykindDC` formula (`10+(DomainScalykindLVL/2)+CHA`) for Venomous
Stare's own save DC, but since Venomous Stare itself is out of catalog scope
(above), there is no honest magnitude to promote its header to `grounded`
either. Stays `ingested-magnitude`, unchanged.

## Fix implementation (RED → GREEN, both directions proven)

`domain_power::DOMAIN_POWER_CATALOG` gained one entry (`UNDEAD_SUBDOMAIN_SELECTION
= "domain:undead_subdomain"`, `granted_power_name: "Death's Kiss"`,
`magnitude_formula: "max(1,DomainLVL/2)"`, `grounds_self_application: false`).
`explain_cleric_level1_spell_baseline` (Cleric) and
`ground_or_block_inquisitor_domain_power` (Inquisitor) both gained the SAME
`if spec.grounds_self_application { ... }` guard around their existing
magnitude/activation-state block — `uses_per_day` stays unconditional in both.
`v06_work_inventory.rs`'s `classify()` gained one new check (the subdomain-keyed
sibling of the pre-existing `group == "Domain Power"` check), matched by the
catalog's own `(domain_display_name, granted_power_name)` pair via a new
`domain_power::domain_power_catalog_group_and_power_names()` bridge function —
**never** by a bare feature-name check, which a direct corpus scan proved
unsafe: `"Rage Power ~ Strength Surge"` (a Barbarian rage power) and
`"Strength Blessing ~ Strength Surge"` (a Warpriest blessing) both collide
with the catalog's own `"Strength Surge"` granted-power name under an
UNRELATED group.

- **`domain_power.rs`**: `death_s_kiss_does_not_ground_a_self_application_bonus`
  (RED before the field existed — compile error; GREEN after) +
  `group_and_power_names_bridge_carries_death_s_kiss` (RED: bridge function
  didn't exist; GREEN: pairs contain `("Undead Subdomain", "Death's Kiss")`) +
  `UNDEAD_SUBDOMAIN_SELECTION` added to `granted_power_magnitude_formulas_are_
  byte_identical_to_the_corpus`, `catalog_provenance_matches_the_corpus_
  records_own_source_citation`, and `interpreted_magnitude_matches_a_hand_
  computed_table_derived_from_pf1_rule_text` (formula `max(1,DomainLVL/2)` is
  mathematically `max(X/2,1)` with args swapped — same hand-computed table
  applies). **19 of 19 pass** (2 new, 17 pre-existing, 0 regressed).
- **`mod.rs`**: `single_class_cleric_with_undead_subdomain_reaches_computed_
  via_uses_per_day_only` (RED: `UNDEAD_SUBDOMAIN_SELECTION`/
  `DEATH_S_KISS_ABILITY_ID` didn't exist — compile error; GREEN: `Computed`,
  self_application/not_active NEVER emitted even with an activation entry
  present, uses_per_day = 4) + `single_class_inquisitor_with_undead_subdomain_
  grounds_uses_per_day_only` (same proof, Inquisitor path — passed on FIRST
  run once the catalog entry existed, since Inquisitor's generic dispatch
  needed no separate wiring). **56 of 56 domain-related pilot_compute tests
  pass** (2 new, 0 regressed); **1029 of 1029 pilot_compute tests pass**
  full-module.
- **`v06_work_inventory.rs`**: `a_subdomain_keyed_sibling_of_a_domain_power_
  record_the_probe_observed_reaches_grounded` (RED: `assertion left ==
  right failed left: "engine-does-not-hold" right: "grounded"`, confirmed
  failing for the intended reason before the fix; GREEN after) +
  `a_bare_feature_name_collision_with_a_different_group_is_not_credited`
  (NEGATIVE CONTROL: `"Rage Power ~ Strength Surge"` with `"Strength Surge"`
  genuinely in `domain_power_effect_wired` stays `engine-does-not-hold` —
  proves the fix matches the catalog's own group/power pair, never the
  feature name alone). **527 of 527 bin tests pass** (2 new, 0 regressed).

**Also fixed, found while wiring Inquisitor's own diagnostic text (not a
separate unit closure, a correctness fix to shipped prose)**: the
`class_feature.inquisitor.domain_powers.unsupported` diagnostic's closing
message interpolated `domain_display_name` directly into a
`DEFINE:InquisitorDomain{domain}` citation — correct for every single-word
pre-existing domain name (`Good`→`InquisitorDomainGood`) but WRONG for
`"Undead Subdomain"` (would cite the non-existent token
`InquisitorDomainUndead Subdomain` with a space; the real corpus token,
confirmed by direct read of
`data/corpus/advanced_players_guide/class_feature/inquisitor/inquisitor_domains.json`,
is `InquisitorDomainUndeadSubdomain`, no space). Fixed with a
`domain_display_name.replace(' ', "")` derivation before this cycle's own
entry could ship a wrong citation in a diagnostic a player reads.

## Movement — the real, regen-verified delta

**`population=49438 buckets=10 unclassified=0 overlap=0 citation_failures=0`**
(`python3 scripts/completion_atlas.py --check`, this receipt's own final
state, after re-deriving 4 citation pins this cycle's own ~34-line insertion
shifted — `12986→13020`/`12666→12700`/`12891→12925`/`13768→13802`, each
confirmed by fresh `grep -n` against the real construction site).

- **`by_status`: `engine-does-not-hold: 19067→19065` (−2), `grounded:
  4341→4343` (+2).** `class_feature`: `engine-does-not-hold: 1694→1692`,
  `grounded: 13→15`. Byte-level `git diff -- docs/work-inventory.json`
  confirms these are the ONLY two unit records that changed status —
  `advanced_players_guide:class_feature:domain_power_death_s_kiss`
  (`engine-does-not-hold`→`grounded`, evidence
  `domain_power_probe_observed_a_real_computed_magnitude`) and
  `advanced_players_guide:class_feature:undead_subdomain_death_s_kiss`
  (`engine-does-not-hold`→`grounded`, evidence
  `domain_power_probe_observed_a_real_computed_magnitude_for_the_subdomain_record`).
  **2 units closed to DONE.**
- **5 of the 7-unit population honestly NOT closed**, each named precisely
  above with the exact structural reason and the exact next-cycle fix:
  Animate Servant ×3 (needs a per-spec `uses_per_day_formula` override — a
  real catalog-structure widening, not attempted this cycle), Dragon
  Subdomain's own domain header + Venomous Stare (correctly excluded by the
  catalog's own existing enemy-facing/multi-DESC-token design boundary, not a
  gap).
- **Reclassification (bucket → different non-DONE bucket):** 0 — no unit
  outside the 2 closures moved buckets.
- **Reachability:** 0 units newly reached or lost reachability.
- **Instrument-correction:** 4 `completion_atlas.py` citation pins
  re-derived (this cycle's own insertion shifted them,
  `citation_failures` 4→0, no bucket population moved by that fix alone) + 1
  real prose-citation bug fixed in the Inquisitor diagnostic (named above,
  not previously live-observed as wrong since no prior catalog entry had a
  multi-word `domain_display_name`).

## Figures (every number, its command, its denominator)

- `population=49438`, `by_status` deltas above — `python3
  scripts/completion_atlas.py --check`, of the full corpus, before/after this
  cycle's own guarded regen at the SAME (correct) base `c1580ac9ba36`.
- `2` units closed, `5` of the 7-unit population honestly not closed — `git
  diff -- docs/work-inventory.json`, per-id status comparison, of this
  cycle's own re-derived 7-unit shape (table above).
- `48706 of 51476` corpus records examined, CLEAN, unchanged before/after —
  `corpus_literal_sweep --json-out`, of the full corpus (no `data/corpus/**`
  record added/changed/removed this cycle — Rust classifier + catalog logic
  only).
- `1839 units cleared over 2580 fixture rows, 0 failed` —
  `derived_evaluator_fixture_check --json-out`, of the fixture's own
  2,580-row coverage, unchanged before/after.
- `19` of 19 `domain_power.rs` tests pass (2 new), `56` of 56 domain-related
  `pilot_compute` tests pass (2 new), `1029` of 1029 full `pilot_compute`
  module tests pass, `527` of 527 `v06_work_inventory` bin tests pass (2
  new) — `cargo test --locked --lib` / `--bin v06_work_inventory`, this
  cycle's own final HEAD.

## Row-count command output

```
$ grep -n "^| [0-9]* |" docs/release/SD-34-book-completion/kanban.md | tail -1
| 37 | `mine-bucket-d` | 3 | wave 32, lane C (no AT-34-E# card yet) | partial | ...
```
Row 37 (`mine-bucket-d`) is the SAME accumulating row every prior bucket-D
mining cycle appends into (no dedicated `AT-34-E#` card exists for generic
bucket-D mining) — this cycle appends its own sentence to that row rather
than opening a new one, per house style.

## Build scope verified

- `cargo test --locked --lib` (pilot_compute module) → 1029/1029 pass.
- `cargo test --locked --bin v06_work_inventory` → 527/527 pass.
- `python3 scripts/tests/test_completion_atlas.py` → 38/38 pass.
- `cargo test --locked --no-run` → exit 0 (full workspace, every target
  builds), run BEFORE the guarded regen (the last commit that could move a
  figure this build-scope row depends on).
- Desktop crate (`apps/desktop/src-tauri`) — not run this cycle: `git diff
  --stat c1580ac9ba36...HEAD -- apps/desktop/` is empty, no file under
  `apps/desktop/` touched, honestly reported skipped
  (`workflow-instruction.md §6` step 3 scopes this to "if touched").

## Sweep population

- `corpus_literal_sweep`: `48706 examined of 51476 read, 0 findings, CLEAN`
  — no `data/corpus/**` record added, changed, or removed this cycle (Rust
  classifier + catalog logic only), so the delta vs. the prior receipt's own
  sweep is 0, consistent with 0 records added.

## Oracle pin

- `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`) — no figure in this receipt was derived
  from the pinned oracle corpus (every formula/magnitude here is transcribed
  directly from `data/corpus/**`'s own committed JSON, not the PCGen
  checkout); cited for completeness per the receipt schema.

## Status

**complete** — the real 7-unit population was re-derived from the live tree
(not the stale wave-36 table), an existing domain-granted-power grounding
precedent was found and checked before writing anything new (per the brief's
own instruction), 2 of 7 units are genuinely grounded with real corpus-quoted
magnitude and RED→GREEN test proof at every layer (catalog, both class
dispatch sites, and the classifier), and the 5 remaining units are named
precisely with the exact structural reason they do not fit the existing
precedent — none left as "the rest," none force-fit into a misleading
"+N bonus" sentence to inflate the closure count.

## Notes (judgment calls)

- **Why Death's Kiss got a NEW `grounds_self_application` field instead of a
  bespoke duplicate branch**: the alternative (special-casing Death's Kiss
  out of the generic loop and writing its own explanation block, mirroring
  Good's own special integration) would have needed the SAME logic written
  TWICE (Cleric's loop and Inquisitor's function each call the generic block
  independently) and would have required Inquisitor's own function — which
  this cycle's brief did not ask me to touch structurally — to gain a
  parallel bespoke branch too. A one-field guard on the EXISTING shared block
  is smaller, and correctly generalizes: any FUTURE non-bonus domain power
  (a duration, a boolean-state grant, ...) can reuse the same guard without
  another bespoke branch.
- **Why Animate Servant was not force-fit**: its uses-per-day formula
  (`DomainArtificeLVL/4-1`) is genuinely different from the shared `3+WIS`
  every other catalog entry (including Death's Kiss) rides — reusing
  `domain_power_uses_per_day` for it would have reported a FABRICATED wrong
  number, the exact failure this cycle's brief explicitly forbids ("no
  fabricated magnitude"). Widening the catalog to carry a per-spec
  uses-per-day formula is real, doable work — just not this cycle's narrow
  fix, and named precisely below.
- **Why Dragon Subdomain's own domain-kind header (unit 3) was not touched
  at all**: distinct from Death's Kiss/Animate Servant — its blocker
  (Venomous Stare's enemy-facing, multi-`DESC`-token shape) is not a gap in
  this catalog's reach, it is the catalog's OWN documented exclusion
  boundary, already correctly applied. Forcing it in would mean building the
  exact "guess at an enemy-facing effect's self-application substitute"
  mechanism the module's own doc comment names as a deliberate refusal.
- **A domain-kind (`Kind::Domain`) unit reaching `grounded` at all** was
  investigated (mirroring `Kind::Skill`'s `grounded_magnitude` hook) and
  found to need a genuinely different question — "what single number
  represents a domain HEADER's own grounded state" has no obvious honest
  answer even for Death's Kiss's own parent record
  (`advanced_players_guide:domain:undead_subdomain`, not part of the 7-unit
  population traced above since it was never claimed grounded by the wave 36
  table either) — not attempted, named as a structurally separate, larger
  question for whoever picks up domain-kind grounding generally.

## Next-cycle plan

1. **Animate Servant's real closure needs `DomainPowerSpec` to carry a
   per-spec uses-per-day formula** (currently hardcoded to the shared
   `3+WIS` via `domain_power_uses_per_day`) — a real, bounded structural
   widening: add `uses_per_day_formula: Option<&'static str>` (`None` falls
   back to the current shared formula, `Some(f)` interprets `f` the same way
   `magnitude_formula` already is), then Animate Servant's `DomainArtificeLVL/4-1`
   becomes groundable via the SAME `grounds_self_application: false` pattern
   this cycle already proved for Death's Kiss's own effect-shape mismatch.
   3 units close: `advanced_players_guide:class_feature:construct_subdomain_
   animate_servant`, `advanced_players_guide:class_feature:domain_power_
   animate_servant`, plus the sibling classify() check this cycle already
   built for the subdomain-keyed shape (no NEW classify() code needed, only
   the catalog widening).
2. **Domain-kind (`Kind::Domain`) header records reaching `grounded`** is a
   structurally separate, larger question this cycle deliberately did not
   attempt (see Notes) — worth a dedicated investigation cycle before the
   next lane assumes it is the same shape as the class_feature closures
   above.
3. **This cycle's own corpus-collision finding** (`"Rage Power ~ Strength
   Surge"`/`"Strength Blessing ~ Strength Surge"` both collide with the
   Cleric/Inquisitor Strength domain's own granted-power name under an
   unrelated group) is proven safe by this cycle's own negative-control test,
   but is a live hazard for ANY future widening of the subdomain-sibling
   check to match by feature name alone — keep matching by the catalog's own
   `(domain_display_name, granted_power_name)` pair, never by feature name in
   isolation.
