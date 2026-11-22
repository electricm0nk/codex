# Cycle t12-class-feature-pool-population, cycle 17 — Gate 3 (closure invariant) / row 18 (`epic-8-pool-shaped-class-features`)

- **Card ID:** row 18 (`epic-8-pool-shaped-class-features`), governed by `decisions.md §27b`
  (*"EVERYTHING"*). Dispatched scope: the three named items cycle 16 left, in evidence order —
  (1) verify and, if safe, close the unparenthesised-comparison-as-bare-function-argument gap;
  (2) establish, by reading the oracle, whether Bloodrager's cross-record refusal guard should
  loosen; (3) take Hunter Animal Focus, named since cycle 5, never attempted.
- **Base:** worktree started on a STALE lineage (`1bb523773d`, PR #374's merge into tranche/11,
  footgun 4, same shape hitting most lanes on this card). Fixed: `git reset --hard "$PIN"` —
  `origin/tranche/12`'s tip was still exactly `$PIN` (`0fc6c02f18`, cycle 16's own commit) at
  reset time, so no rebase was needed and no sibling lane had landed on row 18's files since.
- **Oracle:** worktree's oracle slot was empty (git-ignored) as expected for a fresh worktree.
  Bootstrapped via `scripts/fetch-pcgen-oracle.sh --dest
  <worktree>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen`
  — `pcgen-oracle: OK 7f818006e371188e5717fd18d74d18a420747fc6`, pin confirmed. Re-ran
  `scripts/verify.sh --only preflight-oracle` explicitly with both env vars exported — PASS
  against the real, non-default slot (not the forbidden default path).

## 1. The unparenthesised-comparison-as-bare-function-argument gap — verified and CLOSED

Cycle 16 sized this precisely: 1 corpus record, `Protection Blessing ~ Increased Defense`'s
`1+min(WarpriestLVL>20,2,WarpriestLVL/10)`, and named the fix (routing `min`/`max`/`floor`/
`ceil`/`abs` argument parsing through `parse_arith_or_bool` instead of `parse_expr`) a
"grammar-shape change with a wider blast radius" and did not force it.

**Verified against the pinned oracle before implementing** (`§17`): `pcgen/util/PJEP.java`
confirms the real grammar is `org.nfunk.jep` (`extends org.nfunk.jep.JEP`), a standard
operator-precedence expression parser — relational operators sit at their own precedence level
and are valid anywhere an `expr` nonterminal appears, including a function call's
comma-separated arguments, not gated behind a parenthesised sub-expression the way this
module's grammar previously required. Comparisons already evaluate to a plain `1.0`/`0.0`
(`Expr::Cmp`'s own eval arm, same `org.nfunk.jep.function.Comparative.run()` citation).

**Why the "wider blast radius" cycle 16 flagged is actually safe:** `parse_arith_or_bool` is a
strict superset of `parse_expr` — identical behaviour whenever no comparison/`&&` operator
follows the argument's arithmetic portion. Widening the shared `min`/`max`/`floor`/`ceil`/`abs`
argument-parsing branch therefore cannot change how any previously-accepted argument parses; it
only accepts a new shape `parse_expr` alone refused. No oracle citation restricts comparisons to
only `min`/`max` positions specifically, so applying it uniformly to `floor`/`ceil`/`abs` too
(rather than special-casing two of the five functions) is the correct, not arbitrary, scope.

**Fix, in `formula_interpreter.rs`'s `parse_call`:** the `min`/`max`/`floor`/`ceil`/`abs` branch
now calls `self.parse_arith_or_bool()` instead of `self.parse_expr()` for each comma-separated
argument. Doc comments updated on both `parse_call`'s branch and `parse_arith_or_bool`'s own doc
(the prior doc's "exactly one of those three positions" claim is now four).

**New test**, real corpus formula:
`bare_comparison_as_a_min_max_function_argument_matches_the_warpriest_corpus_shape` —
`1+min(WarpriestLVL>20,2,WarpriestLVL/10)` at level 20 → 1, at level 21 → 2; plus
`max(WarpriestLVL>20,0)` at level 25 → 1, at level 5 → 0 (proves the general widening, not a
min()-only special case).

**Mutation-proved RED, then reverted**: temporarily restored the old `parse_expr()` calls
(marked `// MUTATION-PROOF-TEMP`), re-ran the new test — FAILED as expected (`"expected RParen,
got Some(Gt)"`) — then restored the fix from a saved pre-mutation copy of the file (never `git
stash`), re-verified GREEN (39 tests in `formula_interpreter::tests::`, up from cycle 16's 38).

**Effect on row 18's own group counts: none.** Protection Blessing already closes via its other
member (Aura of Protection), confirmed unchanged by the full re-census (§4 below). The fix is
real, oracle-verified, engine-level capability that closes this shape corpus-wide regardless of
whether any single census group happens to move.

## 2. Bloodrager's remaining 7 single-terminal members — the cross-record guard stays, now with the oracle citation proving it correct

**What was investigated:** cycle 13 found each of the 7 chains to a per-bloodline `Bloodrager_
<X>_BloodlineLVL` identifier bound only on a record this module's `owning_class` guard correctly
refuses to import. This cycle's task was to establish, by reading PCGen's own Java, what the
real engine actually does with a variable bound only on an unrelated record — and match it
narrowly, not loosen the guard on assumption.

**Read**: `PlayerCharacter.getVariable` (`code/src/java/pcgen/core/PlayerCharacter.java:2090`) —
sums `getTotalBonusTo("VAR", variableString)`, EVERY `BONUS:VAR` contribution to that exact
variable name from EVERY source the character actually possesses, character-wide, with **no
per-class scoping at all**. This is a stronger, more surprising fact than "the guard is a
reasonable conservative margin" — the real oracle has NO class boundary here whatsoever; this
module's per-record `owning_class` check is doing something the oracle's own variable namespace
does not do.

**But that global sum is only ever nonzero for a variable a character's OWN held sources
actually bonus.** Re-derived exhaustively (corpus-wide grep for `<X>_BloodlineLVL|BloodragerLVL`
or `<X>_BloodlineLVL|BloodragerBloodlineLVL`, any class): confirmed exactly TWO real corpus
records bind any `Bloodrager_<X>_BloodlineLVL` identifier, NEITHER of which a plain Bloodrager
who merely picked that bloodline through the class's own bloodline-choice mechanism would hold:

1. `data/corpus/advanced_class_guide/ability/<x>_bloodline.json` — an ABILITY record,
   `CATEGORY:"Raging Blood Feat Bloodline"`, `PREMULT`-gated on already holding a DIFFERENT
   `Eldritch Heritage Bloodline` or `Sorcerer Bloodline` ability. Binds the identifier to a flat
   constant `1`, not `BloodragerLVL` — a feat a plain Bloodrager has no reason to hold.
2. `eldritch_scion_<x>_bloodline.json` — `class: "Sorcerer"`, the cross-class archetype cycle 13
   named, binding `Bloodrager_<X>_BloodlineLVL|BloodragerBloodlineLVL`.

No THIRD record exists anywhere binding a plain Bloodrager's own per-bloodline `BloodlineLVL` to
anything level-scaled. Importing either candidate would misrepresent EVERY Bloodrager as if they
also held the Raging Blood feat or the Eldritch Scion archetype — a live fabricated value, the
exact failure mode this bundle's own dispatch briefs warn happened once already when a guard
like this was loosened.

**Conclusion, matched narrowly per the brief's own instruction: the guard stays exactly as
cycle 12 built it.** This cycle's contribution is not a code change here but the oracle citation
proving the guard is the CORRECT model of the real engine's semantics (not merely an unverified
safety margin), plus confirming the remaining 7 are a genuine data/ingestion gap (no record
anywhere grants a plain Bloodrager's per-bloodline level at all) rather than a resolvable
compute-shape gap this lane's scope could close. Doc comment added on `resolve_pcgen_var_chain`
in `class_feature_grant_consumer.rs` recording the citation and the two candidate records by
name, so the next cycle does not have to re-derive this from scratch.

**Non-movement reported loudly, per `§17a`**: Bloodrager Bloodline stays 6/12 combined,
unchanged. This is the honest outcome of "hard impossibility" scope, not an unexplored gap.

## 3. Hunter Animal Focus — widened from Bull-only to all 13 real corpus options

Named since cycle 5, never attempted (a card-adjacent, not-in-the-six-pools feature this brief
explicitly assigned this cycle). Verified: `ground_or_block_hunter_animal_focus` was already a
real, LIVE consumer (called from Hunter's own chassis dispatch, `mod.rs:26125` pre-cycle), not a
stub — Bull was the only one of 13 real corpus options (`data/corpus/advanced_class_guide/
class_feature/hunter_animal_focus/*.json`) recognized; the other 12 claim-blocked.

**Sized the real shapes** (`§17a`, all 13 read directly from their own corpus records):
- 10 options share Bull's exact tiered shape (base + `PREVARGTEQ:...,8` tier + `PREVARGTEQ:
  ...,15` tier, additive `BONUS:VAR` stacking): Bear (CON), Tiger (DEX), Falcon (Perception),
  Frog (Swim + Acrobatics-when-jumping), Monkey (Climb), Owl (Stealth), Snake (flat AoO/AC
  bonus, no downstream `BONUS:` target — a standalone fact, same bar Wild Empathy already
  established), Stag (land speed), Wolf (scent range).
- Bat shares the tiered shape but with NO third numeric tier on its own `VAR` (60 base + 30 at
  level 8); level 15's real benefit is a SEPARATE boolean "blindsense to 10 feet" fact, not a
  third numeric increment.
- Mouse: a boolean evasion/improved-evasion (`PREVARGTEQ:...,12`) posture, no `BONUS:VAR` at
  all.
- No Ability: text-only (`Ability not being used`), no `BONUS:VAR` at all.

**Generic pass, not 13 near-duplicate functions (`§17`)**: added one `HunterAnimalFocusTieredOption`
struct + `HUNTER_ANIMAL_FOCUS_TIERED_OPTIONS` table (11 rows) + one
`hunter_animal_focus_tiered_bonus(option, level)` function, replacing the Bull-specific
`hunter_animal_focus_bull_bonus` body (kept as a thin named wrapper over the table for its own
pre-existing test's continuity). `ground_or_block_hunter_animal_focus`'s active-state branch now
dispatches generically over the table, plus Mouse's and No Ability's own small branches, instead
of a single Bull-only equality check.

**5 new tests**:
- `every_tiered_animal_focus_option_matches_its_own_real_level_gates` — all 11 tiered options'
  base/+8/+15 values, each independently verified against its own corpus record.
- `single_class_hunter_actively_focused_on_tiger_applies_the_real_bonus` — proves the widened
  dispatch through the full end-to-end pipeline, not only the standalone formula.
- `bat_animal_focus_blindsense_appears_only_at_level_15_and_above`.
- `mouse_animal_focus_grounds_evasion_and_improved_evasion_at_its_own_gate`.
- `no_ability_animal_focus_grounds_cleanly_with_zero_magnitude` — never claim-blocks.

**Mutation-proved RED, then reverted**: temporarily reduced `hunter_animal_focus_tiered_bonus` to
`option.base` unconditionally — `every_tiered_animal_focus_option_matches_its_own_real_level_gates`
FAILED as expected (`animal_focus:bull at level 8: left: 2, right: 4`) — then restored from a
saved pre-mutation copy, re-verified GREEN.

**Effect on row 18's own six-pool census: none — by design, not oversight.** Hunter Animal Focus
is a flat 13-member "select ONE" choice pool, structurally different from the six census pools'
"select ONE group, inherit every member of that group" shape (`Hunter Animal Focus ~ <member>`
vs. `<Bloodline-name> <class> Bloodline ~ <member>`). It is reported here, separately and
honestly, not folded into or claimed against the census figures.

## 4. Full re-census, honest figures re-run (`§17a` — re-derived, not assumed)

```bash
cargo test --locked --lib -- \
  rules_core::pilot_compute::generic_pool_group_selection_wiring_tests::pool_group_closure_census_across_all_six_pools_both_resolvers \
  --nocapture
```
```
Sorcerer Bloodline: bonus_vars=31/53, combined(bonus_vars OR desc_formula)=31/53
Bloodrager Bloodline: bonus_vars=5/12, combined(bonus_vars OR desc_formula)=6/12
Cleric Domain: bonus_vars=34/72, combined(bonus_vars OR desc_formula)=35/72
Shaman Spirit: bonus_vars=11/14, combined(bonus_vars OR desc_formula)=12/14
Warpriest Blessing: bonus_vars=0/37, combined(bonus_vars OR desc_formula)=8/37
Cavalier Order: bonus_vars=1/9, combined(bonus_vars OR desc_formula)=2/9

test ... ok
```

**UNCHANGED from cycle 16 on every one of the six figures — reported loudly, not silently, per
`§17a`.** This is the correct outcome given this cycle's own scope: item 1 closed a real engine
gap that happened not to gate any census group's own numerator; item 2 confirmed (not loosened)
a guard whose correct disposition is refusal; item 3 (Hunter Animal Focus) is not one of the six
census pools at all.

## 5. Tests, full re-run

```bash
cargo test --locked --lib -- rules_core::pilot_compute::formula_interpreter::tests::
```
```
test result: ok. 39 passed; 0 failed; 0 ignored; 0 measured; 2772 filtered out
```
(up from cycle 16's 38 — +1, the new min/max-comparison-argument test)

```bash
cargo test --locked --lib -- hunter
```
```
test result: ok. 45 passed; 0 failed; 0 ignored; 0 measured; 2771 filtered out
```
(up from 40 pre-cycle — +5, all new)

```bash
cargo test --locked --lib -- rules_core::pilot_compute::
```
```
test result: ok. 973 passed; 0 failed; 0 ignored; 0 measured; 1843 filtered out
```
(up from cycle 16's 966 — +7: +1 formula_interpreter comparison-argument test, +6 mod.rs tests
covering the Hunter widening)

```bash
cargo test --locked --lib -- rules_core::derived_evaluator_fixture_check::
```
```
test result: ok. 121 passed; 0 failed; 0 ignored; 0 measured; 2695 filtered out
```
(unchanged from cycle 16 — no fixture-checked evaluated value touched this cycle)

```bash
cargo test --locked --lib -- oracle_dispatch_widening_safety_tests
```
```
test result: ok. 48 passed; 0 failed; 0 ignored; 0 measured; 2768 filtered out
```
`a_mystery_pick_alone_grounds_no_tier_one_revelation` untouched, still green — Oracle Mystery
stays withdrawn, unchanged.

## 6. Not attempted / not applicable this cycle

- **The 7 Bloodrager members and the remaining Warpriest/Cleric/Sorcerer/Cavalier gaps** — the
  underlying gaps other than the three named items are unchanged data/ingestion or deeper-shape
  gaps this cycle's own scope did not touch (Warpriest's own `empty=74` gap named by cycle 13,
  Sorcerer's own cycle-12 unchanged baseline).
- Rows 11/15 left as found (`in-progress`/`complete`), untouched. `apps/desktop`'s row 19/20
  lanes not touched. `data/corpus/**` untouched throughout (`git status --porcelain --
  data/corpus` — 0 changes).

## 7. Audits

- **Identifier audit** (`grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'`, scoped to
  `git diff --unified=0` of this cycle's own diff of `src/rules_core/pilot_compute/mod.rs`,
  `src/rules_core/pilot_compute/formula_interpreter.rs`,
  `src/rules_core/pilot_compute/class_feature_grant_consumer.rs`, and the kanban diff):
  `OK_NO_BUNDLE_TAGS` — 0 hits.
- **Wired-integration audit** (`grep -inE "STUB|MOCK|placeholder|not.?yet.?implemented|todo|
  fixme|hack"`, same code scope): `OK_NO_TOKENS` — 0 hits.
- **PI audit**: `pi_scrub.normalized_term_hits(...)` (imported via `scripts/pi_scrub.py`, never
  copied), scoped to `git diff --unified=0` of this cycle's code diff AND the kanban diff →
  `[]` (0 hits) each. `data/corpus/**` untouched throughout.

## 8. Kanban

`docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 18 only (cycle field 16 →
17, Notes prepended). Verified structurally after editing (backtick-aware parser): 21 distinct
`^| N |` rows, 0 duplicates, row 18 parses to 9 backtick-aware raw pipe-split fields (7 real
columns) before and after (matching cycles 15/16's own count exactly, re-derived fresh, not
trusted from the prior receipt). Rows 11 (`in-progress`) / 15 (`complete`) confirmed untouched
(`git diff` shows exactly 1 line changed in the whole file). Status stays `in-progress` — real
remaining scope exists across all six pools and the corpus-wide data gaps named above.

## 9. `df -h /`

```bash
df -h /
```
```
Filesystem      Size  Used Avail Use% Mounted on
/dev/sda1       968G  514G  454G  54% /
```
