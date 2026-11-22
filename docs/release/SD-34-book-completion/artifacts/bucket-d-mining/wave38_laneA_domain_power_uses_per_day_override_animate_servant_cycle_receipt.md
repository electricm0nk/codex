# Cycle — SD-34 wave 38, Lane A — `DomainPowerSpec` per-spec uses-per-day-formula override, closing Construct Subdomain's Animate Servant (2 of the 3-unit named population)

- **Commit SHA:** `faca302af1` (feature commit; `9776003d1c` follows with
  the retro-log-only incident entry, this worktree, not merged onto
  `tranche/14`)
- **Files touched:** `src/rules_core/pilot_compute/domain_power.rs` (new
  `uses_per_day_formula: Option<&'static str>` field on `DomainPowerSpec`,
  new `domain_power_uses_per_day_for` function, new Construct Subdomain
  catalog entry, 7 new tests across the module's `tests` and
  `fixture_check_tests` inline modules), `src/rules_core/pilot_compute/mod.rs`
  (new `CONSTRUCT_SUBDOMAIN_SELECTION`/`ANIMATE_SERVANT_ABILITY_ID`
  constants, both dispatch call sites — Cleric's generic loop and
  Inquisitor's single-spec path — switched to the new override-aware
  function with a conditional, honestly-worded detail sentence, both
  catch-all diagnostic strings widened to name the two SD-34 subdomains,
  2 new dispatch-safety tests), `docs/work-inventory.json` (guarded regen),
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
  (regenerated snapshot), this receipt, `progress.md`, `kanban.md`.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff --unified=0 --
  src/rules_core/pilot_compute/domain_power.rs
  src/rules_core/pilot_compute/mod.rs`, against this cycle's own starting
  commit `fb149ce2b1` — the tranche/14 tip this worktree was cut from — no
  `sd[0-9]+_`/`SD[0-9]+_`/`t_[0-9a-f]{8,}` hits; every `SD-34`/`SD-31`
  reference in the diff is prose, not an identifier).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff, no
  `STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack`).
- **Acceptance criterion (verbatim from this cycle's dispatch brief):** "Close
  wave 37 lane A own named next-cycle item: widen `DomainPowerSpec` with a
  per-spec uses-per-day-formula override, closing Construct Subdomain
  Animate Servant (3 units). ... Extend
  `src/rules_core/pilot_compute/domain_power.rs`'s `DomainPowerSpec` with a
  per-spec uses-per-day-formula override field (do not remove or weaken the
  existing shared `3+WIS` default for every other entry). Wire Animate
  Servant own real formula, corpus-verified byte-for-byte against the
  actual record. Follow the SAME rigor lane A own prior cycle used for
  Death's Kiss — do not fabricate a bonus magnitude this record does not
  have."

## Re-deriving the real population before touching anything

Re-checked the 3-unit population named in wave 37 lane A's own next-cycle
plan against the CURRENT `docs/work-inventory.json` (post wave-37
wave-end-gate), not trusted from the prior receipt's own table:

| # | Unit id | Kind | Status before this cycle |
|---|---|---|---|
| 1 | `advanced_players_guide:domain:construct_subdomain` | domain | `engine-does-not-hold` |
| 2 | `advanced_players_guide:class_feature:construct_subdomain_animate_servant` | class_feature | `engine-does-not-hold` |
| 3 | `advanced_players_guide:class_feature:domain_power_animate_servant` | class_feature | `engine-does-not-hold` |

Re-derive: `python3 -c "import json; d=json.load(open('docs/work-inventory.json'));
units={u['id']:u for u in d['units']}; [print(i, units[i]['status']) for i in
[...the 3 ids above...]]"`.

## Confirming the corpus record's real shape before writing any Rust

Read both corpus records directly (never assumed from the wave 37 receipt's
own prose alone, per this cycle's own "prove before you extend" bar):

- `data/corpus/advanced_players_guide/class_feature/construct_subdomain/animate_servant.json`
  — `DESC`: *"At 8th level, as a standard action, you can give life to
  inanimate objects. This ability functions as animate objects using your
  cleric level as the caster level. You can use this ability
  %1 times per day.|DomainArtificeLVL/4-1"*. ONE formula segment, and the
  surrounding prose states outright it IS the uses-per-day count.
- `data/corpus/advanced_players_guide/class_feature/domain_power/animate_servant.json`
  — the sibling "Domain Power ~" record carries the SAME `DESC` plus two
  `ASPECT` tokens absent from every other catalog entry read this cycle:
  `ASPECT|CheckType|Uses per Day` and
  `ASPECT|CheckCount|%1|DomainArtificeLVL/4-1` — an INDEPENDENT corpus
  confirmation (not just the prose text) that `DomainArtificeLVL/4-1` is
  genuinely the uses-per-day formula, not a magnitude formula that happens
  to read like one.
- `data/corpus/advanced_players_guide/domain/construct_subdomain.json` (the
  domain-kind header, unit 1) — `completeness: "chassis_only"`, carrying
  `PREMULT`/`ABILITY`/`SOURCEPAGE`/`SPELLLEVEL` tokens only. **No
  `BONUS:VAR|Domain<X>LVL|DomainLVL|TYPE=Domain` or
  `BONUS:VAR|Domain<X>Times|DomainPowerTimes|TYPE=Domain` chain at all** —
  confirmed by direct read, not assumed from the Undead Subdomain
  precedent. Its Cleric legality is real (`PREMULT:1,[PREDOMAIN:1,Construct
  Subdomain],[PREVARLT:ArtificeDomain,1]`, the same substitution shape
  Undead Subdomain uses), and Inquisitor legality is real too
  (`inquisitor_domains.json`'s own
  `DEFINE:InquisitorDomainConstructSubdomain|0` token, confirmed by direct
  read, chaining to a real `BONUS:VAR|DomainArtificeLVL|DomainLVL|TYPE=Domain`
  on the INQUISITOR-side record) — but the domain HEADER record itself
  carries no chain to ground, unchanged from the Undead Subdomain header's
  own shape wave 37 lane A already found and declined to force.

## The widening (RED → GREEN, both directions proven)

Added `DomainPowerSpec::uses_per_day_formula: Option<&'static str>` —
`None` for all 6 pre-existing entries (Good/War/Strength/Destruction/Glory/
Death's Kiss, unchanged), `Some("DomainArtificeLVL/4-1")` for the new
Construct Subdomain entry. Added `domain_power_uses_per_day_for(spec,
class_level, modifiers)`: reads `spec.uses_per_day_formula` when present,
interpreting it under the SAME `domain_power_env` the magnitude formula
already uses (class-level-aware, unlike the fixed `3+WIS` call); falls
back to `DOMAIN_POWER_TIMES_FORMULA` (`3+WIS`) when absent — the exact
value `domain_power_uses_per_day` computes, so every pre-existing entry's
own number is provably unchanged (see the new
`domain_power_uses_per_day_for_falls_back_to_the_shared_formula_when_absent`
test, which asserts equality against `domain_power_uses_per_day` across
16 Wisdom modifiers). **`domain_power_uses_per_day` itself was not touched,
removed, or weakened** — Good's own special branch and Healing's Rebuke
Death (which has no catalog spec at all) keep calling it directly; only
the two GENERIC per-spec dispatch sites (Cleric's `other_catalog_domains`
loop, Inquisitor's single resolved spec) were switched to the new
override-aware function, since only those iterate over a live `spec` that
might carry an override.

`grounds_self_application: false` for Construct Subdomain, for a
STRUCTURALLY DIFFERENT reason than Death's Kiss (named explicitly in the
catalog entry's own doc comment so a future reader does not conflate the
two): Death's Kiss has a real, corpus-stated formula that is merely the
wrong SHAPE to be a bonus (a duration). Animate Servant's real effect is
casting *animate objects* — a spell-like ability with no self-application
buff magnitude at all — so there is no honest number this catalog's
magnitude/activation-state block could report for it under any shape.

- **`domain_power.rs`**:
  `animate_servant_does_not_ground_a_self_application_bonus_and_carries_a_uses_per_day_override`
  (RED: the field/entry didn't exist — compile error; GREEN: asserts
  `grounds_self_application == false` AND `uses_per_day_formula ==
  Some("DomainArtificeLVL/4-1")`, and that every OTHER entry — Death's Kiss
  included — keeps `uses_per_day_formula: None`, a regression guard against
  a future edit silently widening the override to an entry whose uses/day
  genuinely IS the shared chain) +
  `domain_power_catalog_uses_per_day_override_formulas_all_parse` (sibling
  of the pre-existing `domain_power_catalog_formulas_all_parse`, scoped to
  `Some(...)` entries) +
  `group_and_power_names_bridge_carries_animate_servant` (proves the
  classifier bridge carries `("Construct Subdomain", "Animate Servant")`) +
  `domain_power_uses_per_day_for_uses_the_override_when_present` (asserts
  the override's OWN computed value, 2 at level 12, and asserts it is
  DIFFERENT from what the shared `3+WIS` formula would give at the same
  Wisdom modifier — a passing test that merely happened to agree would not
  prove the override branch was genuinely taken) +
  `domain_power_uses_per_day_for_falls_back_to_the_shared_formula_when_absent`
  (the fallback half, Good's own spec, equality against
  `domain_power_uses_per_day` across 16 WIS modifiers) + fixture-side
  `animate_servant_carries_a_uses_per_day_override_and_no_self_application_bonus`
  (regression guard, `fixture_check_tests` module) +
  `interpreted_uses_per_day_for_animate_servant_matches_a_hand_computed_table_derived_from_pf1_rule_text`
  (guarantee-4-style: expected values computed BY HAND —
  `floor(level/4)-1`, floored at 0 overall — never read back from
  `eval_expr`/`domain_power_uses_per_day_for`; a mutated evaluator, e.g.
  `Div` swapped for `Mul` or the formula's own `Sub` swapped for `Add`,
  fails this test even though it would still satisfy the byte-identity
  transcription test). Plus the pre-existing
  `granted_power_magnitude_formulas_are_byte_identical_to_the_corpus` and
  `catalog_provenance_matches_the_corpus_records_own_source_citation`
  tests, both widened to include the new entry (new `ANIMATE_SERVANT_JSON`
  `include_str!` constant, pinned against the SUBDOMAIN-keyed record per
  the Death's Kiss precedent). **26 of 26 pass** (7 new against wave 37
  lane A's own pinned baseline of 19, 0 regressed).
- **`mod.rs`**:
  `single_class_cleric_with_construct_subdomain_reaches_computed_via_its_own_uses_per_day_formula`
  (RED: `CONSTRUCT_SUBDOMAIN_SELECTION`/`ANIMATE_SERVANT_ABILITY_ID` didn't
  exist — compile error; GREEN: `Computed`, no
  self_application/not_active explanation ever emitted even with an
  activation entry present, `uses_per_day == 2` at Cleric level 12 — NOT
  the shared-formula value of 4 the fixture's own WIS 12 would give,
  proving the override was genuinely read) +
  `single_class_inquisitor_with_construct_subdomain_grounds_its_own_uses_per_day_formula`
  (same proof, Inquisitor path — passed on FIRST run once the catalog
  entry existed, mirroring wave 37 lane A's own observation that
  Inquisitor's generic dispatch needs no separate wiring). **2 of 2 new
  tests pass, 0 regressed.**

Full scoped run: `cargo test --locked --lib pilot_compute::` →
**1038 of 1038 pass** (9 new against wave 37 lane A's own pinned baseline
of 1029 — 7 in `domain_power` + 2 dispatch-safety tests in `mod.rs`,
exactly accounting for the delta: 1029 + 7 + 2 = 1038).
`cargo test --locked --bin v06_work_inventory` → **527 of 527 pass** (0
new — no NEW `classify()` code was needed, per wave 37 lane A's own
next-cycle plan: the existing subdomain-sibling check, matched by the
catalog's own `(domain_display_name, granted_power_name)` pair, already
covers any FUTURE catalog entry generically).

## Movement — the real, regen-verified delta

**`population=49438 buckets=10 unclassified=0 overlap=0`**
(`python3 scripts/completion_atlas.py --check`, this receipt's own final
state). `citation_failures=0` — **no citation pin needed re-deriving this
cycle**, unlike wave 37 lane A: this cycle's own insertion touched
`domain_power.rs` and `pilot_compute/mod.rs` only, never
`src/bin/v06_work_inventory.rs` (the ONLY file `completion_atlas.py`'s
`BUCKET_DEFINITIONS` citations point into), so no line number the atlas
cites could have shifted.

- **`by_status`: `engine-does-not-hold: 19065→19063` (−2), `grounded:
  4343→4345` (+2).** `class_feature`: `engine-does-not-hold: 1692→1690`,
  `grounded: 15→17`. Byte-level `git diff -- docs/work-inventory.json`
  confirms these are the ONLY two unit records that changed status:
  `advanced_players_guide:class_feature:construct_subdomain_animate_servant`
  (`engine-does-not-hold`→`grounded`, evidence
  `domain_power_probe_observed_a_real_computed_magnitude_for_the_subdomain_record`)
  and `advanced_players_guide:class_feature:domain_power_animate_servant`
  (`engine-does-not-hold`→`grounded`, evidence
  `domain_power_probe_observed_a_real_computed_magnitude`). **2 units
  closed to DONE.**
- **Bucket-level (`completion_atlas.py --check`): `B: 11770→11769` (−1),
  `D: 2661→2660` (−1), `DONE: 25244→25246` (+2).** The two closed units'
  own pre-cycle evidence strings sorted into DIFFERENT non-DONE buckets
  before this cycle (`construct_subdomain_animate_servant`'s
  `class_feature_of_unmodelled_corpus_class:construct` landed in D;
  `domain_power_animate_servant`'s
  `class_feature_option_pool_record_with_magnitude_not_held_by_engine`
  landed in B) — a real, expected difference from wave 37 lane A's own
  Death's Kiss closure (both of ITS units happened to land in the same
  bucket pre-cycle); re-derived from the atlas's own bucket counts, not
  assumed identical.
- **1 of the 3-unit population honestly NOT closed:** the domain-kind
  header, `advanced_players_guide:domain:construct_subdomain` — stays
  `engine-does-not-hold`, unchanged. Named precisely above: its own corpus
  record carries no `Domain<X>LVL`/`Domain<X>Times` chain at all
  (`completeness: "chassis_only"`), the identical structural gap wave 37
  lane A already found and declined to force for the Undead Subdomain
  header, confirmed by direct read this cycle rather than assumed from
  that precedent.
- **Reclassification (bucket → different non-DONE bucket):** 0 — no unit
  outside the 2 closures moved buckets.
- **Reachability:** 0 units newly reached or lost reachability.
- **Instrument-correction:** 0 — this cycle's own insertion never touched
  `src/bin/v06_work_inventory.rs`, so no citation pin needed re-deriving
  (confirmed `citation_failures=0` both before and after).

## Figures (every number, its command, its denominator)

- `population=49438`, bucket deltas above — `python3
  scripts/completion_atlas.py --check`, of the full corpus, before/after
  this cycle's own guarded regen at the SAME (correct) base `fb149ce2b1`
  (this cycle's own starting tranche/14 tip).
- `2` units closed, `1` of the 3-unit population honestly not closed —
  `git diff -- docs/work-inventory.json`, per-id status comparison, of
  this cycle's own re-derived 3-unit shape (table above).
- `48706 of 51476` corpus records examined, CLEAN, unchanged before/after
  — `corpus_literal_sweep --json-out`, of the full corpus (no
  `data/corpus/**` record added/changed/removed this cycle — Rust
  interpreter + catalog logic only).
- `1839 units cleared over 2580 fixture rows, 0 failed` —
  `derived_evaluator_fixture_check --json-out`, of the fixture's own
  2,580-row coverage, unchanged before/after.
- `26` of 26 `domain_power.rs` tests pass (7 new), `2` of 2 new dispatch
  tests pass (0 regressed), `1038` of 1038 full `pilot_compute` module
  tests pass, `527` of 527 `v06_work_inventory` bin tests pass (0 new) —
  `cargo test --locked --lib` / `--bin v06_work_inventory`, this cycle's
  own final HEAD.

## Row-count command output

```
$ grep -n "^| [0-9]* |" docs/release/SD-34-book-completion/kanban.md | tail -1
| 37 | `mine-bucket-d` | 3 | wave 32, lane C (no AT-34-E# card yet) | partial | ...
```
Row 37 (`mine-bucket-d`) is the SAME accumulating row every prior bucket-D
mining cycle appends into (no dedicated `AT-34-E#` card exists for generic
bucket-D mining) — this cycle appends its own sentence to that row rather
than opening a new one, per house style, unchanged from wave 37 lane A's
own precedent.

## Build scope verified

- `cargo test --locked --lib "domain_power::"` → 26/26 pass.
- `cargo test --locked --lib "construct_subdomain"` → 2/2 pass.
- `cargo test --locked --lib pilot_compute::` (full module) → 1038/1038
  pass.
- `cargo test --locked --bin v06_work_inventory` → 527/527 pass.
- `cargo test --locked --no-run` → exit 0 (full workspace, every target
  builds), re-run AFTER the guarded regen (the last write in this cycle
  that could move a figure this row depends on — the regen itself changes
  only `docs/work-inventory.json`, a data file no compiled target reads at
  build time, so ordering here is for `decisions.md §12` L7's discipline,
  not because the regen could break compilation).
- Desktop crate (`apps/desktop/src-tauri`) — not run this cycle: `git diff
  --stat -- apps/desktop/` is empty, no file under `apps/desktop/` touched,
  honestly reported skipped (`workflow-instruction.md §6` step 3 scopes
  this to "if touched").

## Sweep population

- `corpus_literal_sweep`: `48706 examined of 51476 read, 0 findings, CLEAN`
  — no `data/corpus/**` record added, changed, or removed this cycle (Rust
  interpreter + catalog logic only), so the delta vs. wave 37 lane A's own
  sweep is 0, consistent with 0 records added.

## Oracle pin

- `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`) — no figure in this receipt was derived
  from the pinned oracle corpus (every formula/magnitude here is
  transcribed directly from `data/corpus/**`'s own committed JSON, not the
  PCGen checkout); cited for completeness per the receipt schema.

## Status

**complete** — the real 3-unit population was re-derived from the live
tree, both corpus records (the class_feature record and its "Domain
Power ~" sibling) were read directly before writing any Rust, the
`DomainPowerSpec::uses_per_day_formula` override was added WITHOUT
removing or weakening the existing shared `3+WIS` default for any of the 6
pre-existing entries (proven by a dedicated fallback-equality test, not
merely asserted), Animate Servant's real corpus formula
(`DomainArtificeLVL/4-1`) was wired byte-for-byte identical to both the
class_feature record and its Domain-Power sibling with RED→GREEN proof at
every layer (catalog, both class dispatch sites, and a hand-computed
table independent of the evaluator), and the one remaining unit (the
domain-kind header) is named precisely with the exact structural reason
it does not fit — the identical gap wave 37 lane A already found for
Undead Subdomain's own header, confirmed fresh by direct corpus read
rather than assumed transferable.

## Notes (judgment calls)

- **Why the domain-kind header (unit 1) was not force-closed**: distinct
  question from either class_feature closure — "what single number
  represents a domain HEADER's own grounded state" has no obvious honest
  answer, the exact structural question wave 37 lane A's own Notes section
  named as separate and larger, deliberately not attempted again here
  without first re-deriving it against Construct Subdomain's OWN record
  (done: confirmed no `BONUS:VAR` chain at all, an even starker absence
  than Undead Subdomain's header, which at least had SOME record to
  examine).
- **Why `grounds_self_application: false` is documented with a DIFFERENT
  rationale than Death's Kiss's own**, even though both entries carry the
  same field value: conflating "wrong shape for a bonus" (Death's Kiss)
  with "no bonus to have at all" (Animate Servant) would misdescribe the
  next domain power a future cycle adds under either shape — the catalog
  entry's own doc comment states the distinction explicitly so a future
  reader extending the catalog picks the right precedent to copy.
- **Why the two catch-all diagnostic strings (Cleric's `else` branch,
  Inquisitor's initial `else`) were widened to name both SD-34
  subdomains**: both strings were ALREADY stale before this cycle (neither
  named Undead Subdomain, closed wave 37) — a pre-existing gap in the
  exact function this cycle edits, not a new one this cycle introduced.
  Left uncorrected, the Inquisitor initial diagnostic and the Cleric
  catch-all would both continue asserting Construct Subdomain's granted
  power "is not implemented anywhere in this codebase" for the specific
  posture where THIS diagnostic fires (no recognized domain at all, or an
  unrecognized domain alongside a recognized one) — a plausible-looking
  but wrong claim in shipped diagnostic text, the exact failure shape
  `AGENTS.md` rule 7 names. Fixed both while already inside the function,
  rather than deferred to a future cycle that might not notice the
  staleness again.
- **Why the "reads spec.uses_per_day_formula, else falls back" design was
  chosen over a bespoke Animate-Servant-only branch**: mirrors wave 37
  lane A's own `grounds_self_application` precedent exactly — a one-field
  guard on the EXISTING shared per-spec loop is smaller than duplicating
  logic across Cleric's loop and Inquisitor's function, and correctly
  generalizes: any FUTURE catalog entry whose corpus formula slot is
  genuinely its own uses-per-day count (not `3+WIS`) can carry
  `Some(formula)` without another bespoke dispatch branch.

## Next-cycle plan

1. **Domain-kind (`Kind::Domain`) header records reaching `grounded`**
   remains the one structurally separate, larger question named by both
   this cycle and wave 37 lane A — now confirmed against TWO real headers
   (Undead Subdomain, Construct Subdomain), both carrying no
   `Domain<X>LVL`/`Domain<X>Times` chain at all. Worth a dedicated
   investigation cycle before any future lane assumes a domain header can
   ride the same per-spec catalog shape as its granted power.
2. **No further Animate-Servant-shaped units remain in this catalog's
   reach** — the 3-unit population named by wave 37 lane A is now fully
   dispositioned (2 closed, 1 named-and-excluded). A future cycle widening
   `DOMAIN_POWER_CATALOG` further should re-scan the corpus fresh for
   OTHER domain powers whose formula slot is genuinely their own
   uses-per-day count (this cycle checked only Construct Subdomain, not
   the wider corpus) before assuming Animate Servant was the only such
   shape.
3. **This cycle's own catch-all-diagnostic staleness fix** (Notes, above)
   is a template for a future cycle: any time a NEW catalog entry is
   added, grep both dispatch functions' own catch-all diagnostic strings
   for a hardcoded domain-name list before assuming they stay accurate
   automatically — they do not, by construction (both are literal `&str`s,
   not generated from `DOMAIN_POWER_CATALOG`).
