# Cycle t12-class-feature-pool-population, cycle 14 — Gate 3 (closure invariant) / row 18 (`epic-8-pool-shaped-class-features`)

- **Card ID:** row 18 (`epic-8-pool-shaped-class-features`), governed by `decisions.md §27b`
  (*"EVERYTHING"*). Dispatched scope: two more pools moved (cycle 13); the rest are named and
  shaped. First establish whether Warpriest Blessing (0/37) and Cavalier Order's remaining groups
  are a `§16` reclassification (already `§7`-DONE text), not a resolver failure — that check is
  worth more than another resolver widening. Then take whichever remaining named item the evidence
  says is real: `classlevel("X","APPLIEDAS=NONEPIC")`'s 2-argument form, Hunter Animal Focus, or
  Bloodrager's remaining 7. Re-run `pool_group_closure_census_across_all_six_pools` and report
  movement, including zero.
- **Base:** worktree started on a stale/unrelated branch (`worktree-wf_5f7572fc-28e-1`, HEAD
  `1bb523773d`, PR #374's merge) — `git merge-base --is-ancestor "$PIN" HEAD` returned FALSE
  (footgun 1). Fixed: `git fetch origin tranche/12` (tip `ccd7e4992d`, cycle 13's own commit, the
  same as `$PIN`), `git reset --hard origin/tranche/12`, re-verified
  (`git merge-base --is-ancestor "$PIN" HEAD` → 0, HEAD == PIN). `origin/tranche/12` had no cycles
  past cycle 13's own commit at session start — no sibling lane landed on row 18's files since.
- **Oracle:** bootstrapped fresh (`scripts/fetch-pcgen-oracle.sh --dest
  docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen`)
  — `pcgen-oracle: OK 7f818006e371... ` (pin confirmed). Consulted directly:
  `plugin/jepcommands/ClassLevelCommand.java`'s `run` method, read via `git show HEAD:...` (sparse
  cone never widened), to verify the real semantics of `classlevel("X","APPLIEDAS=NONEPIC")` before
  implementing it (§2 below).

## 1. The `§16` reclassification check (evidence, not assumption)

A brief diagnostic (`cycle14_diagnose_warpriest_and_cavalier_zero_magnitude_shape`, printed via
`--nocapture`, then **removed before commit**, same methodology as cycle 13's own) walked every
real member record of Warpriest Blessing's and Cavalier Order's own groups
(`class_feature_record_tokens_pre_gate_safe`, the row's own live data structure — not a fresh
python re-derivation) and classified each GROUP by whether any of its members carry (a) a real
`BONUS:VAR` token, (b) a `%N`-substituted `DESC:` formula (e.g. `"...gains a +%1 morale
bonus...|max(1,WarpriestLVL/2)"`) with no `BONUS:VAR`, or (c) neither:

```
Warpriest Blessing: 37 groups total, 29 zero-magnitude (no BONUS:VAR, no %N desc-substitution),
  8 %N-desc-substitution-only, 0 carry a real BONUS:VAR
Cavalier Order: 9 groups total, 7 zero-magnitude (no BONUS:VAR, no %N desc-substitution),
  1 %N-desc-substitution-only, 1 carries a real BONUS:VAR
```

### 1a. 29 Warpriest + 7 Cavalier groups (36 total) — genuinely zero-magnitude, real `§16` finding

Direct corpus inspection of two representative records (`Law Blessing ~ Axiomatic Strike`,
`Artifice Blessing ~ Crafter's Wrath`) confirms: `raw_tokens` carries only `KEY`/`CATEGORY`/`TYPE`/
`DESC` — no `BONUS`, no `DEFINE`, no `%N` reference in the description text at all (real prose,
e.g. "an additional 1d6 points of damage" spelled out as text, not a substituted variable). This is
the SAME shape `description_completion.rs`'s own module doc names as the canonical zero-magnitude
case (Deflect Arrows): "There is no magnitude to ground... not because the engine is unfinished,
but because the feature genuinely has none."

**Cross-checked against the row's own official population instrument**
(`scripts/census_class_feature_pool_population.py`, `NUMERIC_MAGNITUDE_KEYS = {"BONUS","DEFINE"}`
or a `%N`-substituted `DESC`): these 36 groups' member records carry NEITHER, so they were **never
counted in the row's own `numeric_magnitude_records` / `RESIDUAL numeric-magnitude needing
compute`** population (currently 6,018) to begin with — confirmed by re-running the census script
unchanged (`RESIDUAL numeric-magnitude needing compute 6018`, identical before and after this
cycle's own change). This is the finding: `pool_group_closure_census_across_all_six_pools`'s own
"0/37" is real for its own narrower "carries a resolvable magnitude" measure, but it is **not** 37
units of this row's TRUE workload — 29 of those 37 groups were never part of the population this
card must close. **`§16` names the movement correctly: these are not units closed this cycle
(nothing changed about them), but they are also not units of unclosed magnitude-bearing work —
they are catalog-servable text, a DIFFERENT shape than the row's own numeric-magnitude population
already excludes them from.** They are NOT yet stamped `text-complete` (`description_completion.rs`
is scoped to `feat` kind only today, not `class_feature`; wiring a `class_feature`-kind sibling of
that module is real future work, not claimed done here) — reported honestly as a real, named,
DIFFERENT population, not silently marked complete.

### 1b. 8 Warpriest + 1 Cavalier groups (9 total) — a real, distinct, unimplemented compute shape

These carry no `BONUS:VAR` but DO carry a `%N`-substituted `DESC:` formula (e.g. `Destruction
Blessing ~ Destructive Attacks`: `"...gains a +%1 morale bonus on weapon damage rolls.|max(1,
WarpriestLVL/2)"`). This IS real numeric-magnitude work by the row's own population definition
(the census script's `has_percent_substitution` check), and is genuinely unresolved by BOTH this
cycle's resolvers: `resolve_pool_member_sole_magnitude` only ever reads a record's `bonus_vars`
(confirmed: `ClassFeatureRecordTokens` has no `%N`/description-formula field at all), and
`formula_interpreter.rs`'s own module doc explicitly scopes `%N`/`%N` parameter substitution in
`DESC:` text OUT of this module ("a text-rendering mechanism, not formula arithmetic; its consumer
is `description_completion.rs`/`pcgen_desc.rs`, out of this lane's write scope"). One instance
(Warpriest's own Destructive Attacks) is ALREADY hand-modelled outside the generic pool resolver
(`class_feature.acg.warpriest.destruction_blessing.destructive_attacks_self_application`,
`warpriest_dispatch_widening_safety_tests::
single_class_warpriest_actively_using_destructive_attacks_grounds_the_real_bonus`, unaffected by
this cycle, still green) — confirming the shape is real and resolvable, just not yet generic. The
other 8 groups' `%N`-substitution resolver is a genuinely different lane's write scope
(`pcgen_desc.rs`), named here, not forced.

## 2. Implemented: `classlevel("X","APPLIEDAS=NONEPIC")`'s real 2-argument form (`§17`)

Verified against the real oracle before writing (`ClassLevelCommand.java`'s `run`): a second string
argument starting `APPLIEDAS=` is a QUALIFIER, not a second class name; `NONEPIC` is the only value
the oracle recognises (any other throws `ParseException("Did not understand APPLIEDAS=" +
applied)`). Its real effect: `cl += ";BEFORELEVEL=" + (mode.getMaxNonEpicLevel() + 1)` — the class
level read is capped at the game mode's non-epic ceiling. This engine never models epic levels, and
every class chassis it resolves already gates its own level at that class's own corpus-derived
`max_level` (`untabled_base_class_chassis.rs`/`generic_class_chassis.rs`, <= 20 for every real base
class) — so the cap can never actually bind for any character this engine represents, making the
2-argument form **observationally identical** to the 1-argument form for every real input. Widened
`formula_interpreter.rs`'s `classlevel` parser branch: after a string class-name argument, an
optional `,"APPLIEDAS=NONEPIC"` is now accepted and reuses the SAME `Expr::ClassLevel(class_name)`
binding cycle 6/9 already built — no new AST variant, no new lookup table. Any other qualifier
value still refuses, matching the oracle's own `ParseException` exactly (never silently accepted).

**Real corpus shape confirmed live** (not assumed): `core_rulebook/class_feature/monk/
standard_monk.json` (KEY `"Monk ~ Standard Class"`) and `pathfinder_unchained/class_feature/monk/
unchained_monk.json`'s BASEAB/SAVE `TYPE:` chassis tokens, both real ` ~ `-group-qualified
`class_feature` records — part of this row's own ~1,913-group corpus-wide population, though not
one of the six named pools' own member groups (this shape lives in class chassis tokens, not a
pool-member's own `BONUS:VAR`/`DESC:`).

## 3. Tests, RED→GREEN (`§1a`)

3 new tests in `formula_interpreter.rs`:
`classlevel_two_argument_appliedas_nonepic_form_reads_the_same_binding` (the real Monk shape
resolves to the same `CLASSLEVEL::Monk` binding as the 1-arg form),
`classlevel_two_argument_form_still_refuses_an_unbound_or_wrong_class` (the widening does not leak
into answering for an unbound class), `classlevel_two_argument_form_refuses_an_unrecognised_
appliedas_qualifier` (an unrecognised second-argument value still refuses, matching the oracle's
own `ParseException`).

Mutation-style proof: temporarily disabled the widening's comma-handling branch (reverted to the
pre-cycle 1-arg-only parse, keeping the new tests in place) —
`classlevel_two_argument_appliedas_nonepic_form_reads_the_same_binding` FAILED
(`FormulaEvalError("expected RParen, got Some(Comma)")`), proving the test exercises the real
change. Restored from a saved copy of the file (`cp` from a pre-mutation snapshot, never `git
stash`), re-verified green.

```bash
cargo test --locked --lib -- rules_core::pilot_compute::formula_interpreter::tests::classlevel_two_argument
```
```
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 2798 filtered out
```

```bash
cargo test --locked --lib -- rules_core::pilot_compute::
```
```
test result: ok. 958 passed; 0 failed; 0 ignored; 0 measured; 1843 filtered out; finished in 25.87s
```
(up from cycle 13's 955/955 — +3, exactly the 3 new tests above)

```bash
cargo test --locked --lib -- rules_core::pilot_compute::generic_pool_group_selection_wiring_tests::pool_group_closure_census_across_all_six_pools --nocapture
```
```
Sorcerer Bloodline: 31/53 groups carry a resolvable member
Bloodrager Bloodline: 5/12 groups carry a resolvable member
Cleric Domain: 34/72 groups carry a resolvable member
Shaman Spirit: 11/14 groups carry a resolvable member
Warpriest Blessing: 0/37 groups carry a resolvable member
Cavalier Order: 1/9 groups carry a resolvable member
```

**Real movement this cycle: ZERO on the six-pool census, reported as loudly as movement (`§17a`).**
The `classlevel("X","APPLIEDAS=NONEPIC")` widening does not occur in any of the six pools' own
member records (§2 above) — it closes a real corpus-wide grammar gap (part of the row's broader
~1,913-group population) but does not move this specific census. All six pools reproduce cycle 13's
exact baseline.

```bash
cargo test --locked --lib -- rules_core::derived_evaluator_fixture_check::
```
```
test result: ok. 121 passed; 0 failed; 0 ignored; 0 measured; 2680 filtered out; finished in 8.74s
```

```bash
cargo test --locked --lib -- oracle_dispatch_widening_safety_tests
```
```
test result: ok. 48 passed; 0 failed; 0 ignored; 0 measured; 2753 filtered out; finished in 4.20s
```
`a_mystery_pick_alone_grounds_no_tier_one_revelation` untouched, still green (Oracle Mystery stays
withdrawn per its own standing ruling; no new resolver built, per `§17`).

```bash
cargo test --locked --lib -- cavalier
```
```
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 2785 filtered out; finished in 8.74s
```

## 4. Not attempted (named, per `§17`)

The 8 Warpriest + 1 Cavalier `%N`-substituted-DESC groups (§1b) — a real, named,
`pcgen_desc.rs`-scoped blocker (formula_interpreter.rs's own module doc explicitly excludes this
shape), not this lane's write scope, not forced. Hunter Animal Focus: unchanged since cycle 5,
still activation-gated, not attempted. Bloodrager's remaining 7 single-terminal members: unchanged
from cycle 13's own finding (a cross-class-only-bound `BloodlineLVL`, a real blocker). Rows 11/15
left as found (`in-progress`/`complete`), untouched. `apps/desktop`'s row 19/20 lanes not touched.
`data/corpus/**` untouched throughout (`git status --porcelain -- data/corpus` — 0 changes).

## 5. Audits

- **Identifier audit** (`grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'` against
  `git diff -- src/rules_core/pilot_compute/formula_interpreter.rs`, scoped to this cycle's own
  diff): `OK_NO_BUNDLE_TAGS` — 0 hits.
- **Wired-integration audit** (`grep -inE "STUB|MOCK|placeholder|not.?yet.?implemented|todo|fixme|
  hack"`, same scope): `OK_NO_TOKENS` — 0 hits.
- **PI audit**: `pi_scrub.normalized_term_hits(...)` (imported via `scripts/pi_scrub.py`, never
  copied), scoped to `git diff --unified=0` of both this cycle's code diff AND the kanban diff →
  `[]` (0 hits) each. `data/corpus/**` untouched throughout.

## 6. Kanban

`docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 18 only (cycle field 13 →
14, Notes prepended). One raw-pipe-in-quoted-string bug caught and fixed before commit (a `%N`
formula example containing a literal `|` split an extra cell; wrapped in backticks, per this
bundle's own backtick-aware-parser convention). Verified: 21 distinct `^| N |` rows, 0 duplicates,
row 18 parses to 9 raw pipe-split fields (7 real columns) before and after, single physical line.
Rows 11 (`in-progress`) / 15 (`complete`) confirmed untouched from their pre-cycle state.

## 7. `df -h /`

```
Filesystem      Size  Used Avail Use% Mounted on
/dev/sda1       968G  505G  464G  53% /
```
