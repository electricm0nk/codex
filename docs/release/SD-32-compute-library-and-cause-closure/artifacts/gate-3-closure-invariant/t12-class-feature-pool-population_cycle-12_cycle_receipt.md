# Cycle t12-class-feature-pool-population, cycle 12 — Gate 3 (closure invariant) / row 18 (`epic-8-pool-shaped-class-features`)

- **Card ID:** row 18 (`epic-8-pool-shaped-class-features`), governed by `decisions.md §27b`
  (*"EVERYTHING"*). Dispatched scope: read PCGen's own variable-resolution path
  (`PlayerCharacter.java`, `BonusManager`, formula/variable resolution classes) to establish
  exactly when PCGen substitutes 0 for an unbound identifier, implement that narrowly, and
  re-derive `pool_group_closure_census_across_all_six_pools`.
- **Base:** worktree `HEAD` was `1bb523773d` (PR #374's merge) at session start, `git merge-base
  --is-ancestor "$PIN" HEAD` returned FALSE. Fixed: `git reset --hard "$PIN"` — `origin/tranche/12`
  HEAD **is** `$PIN` (`04ead3b5da`, cycle 11's own row20 sibling commit), so this was a plain
  fast-forward reset, no rebase needed. `BASE_OK` re-verified after. `git log origin/tranche/12 --
  src/rules_core/pilot_compute/class_feature_grant_consumer.rs src/rules_core/pilot_compute/mod.rs`
  confirms no sibling-lane collision since cycle 11's own commit (`35783ea9d0`).
- **Oracle:** fresh worktree, empty git-ignored slot as expected. Bootstrapped via
  `scripts/fetch-pcgen-oracle.sh --dest <repo-local artifacts/corpus/operator-supplied/pcgen>`;
  pin confirmed `7f818006e371188e5717fd18d74d18a420747fc6`.

## 1. PCGen's real variable-resolution path, traced end to end (file:line, pinned oracle)

Read from the oracle's own git objects (`git show HEAD:<path>`, no checkout, per the brief's own
`git ls-tree` confirmation of 4503 readable `.java` files):

1. **`PlayerCharacter.java:2090-2140` (`getVariable`)** — tries a modern, DECLARED `VariableKey`
   first (`hasVariable`, `:2430-2440`: true only for a name registered via `DEFINE`). Failing
   that, falls to `getVariableValue`, which is exactly the path a `BONUS:VAR|Target|Formula` RHS
   resolves through: `code/src/java/pcgen/cdom/base/JEPFormula.java`'s `resolve()` calls
   `character.getVariableValue(formula, source)` directly.
2. **`VariableProcessor.java:125-139` (`getVariableValue`)** tries the modern JEP parser first
   (`:151-182`/`processJepFormula` `:433-513`). JEP's own contract: EVERY symbol in the expression
   must resolve via `lookupVariable` (`:532-561`) or the whole formula is rejected outright — *"we
   could not get a value for all of the variables, so it must not have been a JEP function after
   all"* (`:469`).
3. **`getVariableValue` then falls through to the LEGACY `+`/`-`/`*`/`/`-delimited parser**
   (`processBrokenParser`, `:215-421`). Its own per-term loop (`:357-421`) calls `lookupVariable`
   for each term; when that returns `null` (not `hasVariable`, no internal/export variable either
   — exactly what a genuinely-never-bound identifier hits), the term's raw text is left unchanged,
   `Float.parseFloat` throws `NumberFormatException`, **caught silently and treated as `0.0`**
   (`:394-402`, the code's own comment: *"Don't care, as it's just zero"*).

**Confirmed: real PCGen genuinely computes 0 for a bare, corpus-wide-unbound identifier inside a
`BONUS:VAR` arithmetic formula — it does not refuse the whole formula.** This module's own
`resolve_pcgen_var_chain` had been refusing instead, by deliberate design (its own doc: *"never
guessed, never defaulted"*), which cycle 11 correctly identified as the blocker but did not
implement, naming it as a structural change needing verification first.

**A second, more precise real corpus fact, found while tracing this**: `DEFINE:VarName|0` is
PCGen's own STANDARD idiom for declaring a `BONUS:VAR` target's zero baseline — confirmed via
`plugin/lsttokens/DefineLst.java` (`:79-92`): a non-literal-zero `DEFINE` triggers PCGen's own
deprecation warning, *"DEFINE with a non zero value has been deprecated, please use a DEFINE of 0
and an appropriate bonus"*. `data/corpus/advanced_class_guide/class_feature/bloodrager/
bloodrager_bloodline_tracker.json` carries exactly this: `DEFINE:BloodragerBloodlinePower1LVLBonus|0`
(and the 4/8/12/16/20 siblings) — a REAL corpus fact, more precise than a bare-fallback guess, that
this ingestion had never read before (only `BONUS` tokens were read, never `DEFINE`).

## 2. Implemented, narrowly (`decisions.md §17a`: "implement the condition, not the convenience")

`resolve_pcgen_var_chain` (`class_feature_grant_consumer.rs`) gets a second, bounded fixed-point
pass after its existing one. It retries ONLY the interpreter's own distinct `"unbound variable
{name:?}"` failure (`formula_interpreter.rs`'s `Expr::Var` lookup miss) — never `classlevel(...)`'s
own separately-worded refusal, division-by-zero, an unknown function, or any other refused shape,
all of which fail with different error text this loop never matches and therefore keep refusing
exactly as before. For the missing identifier, it checks:

1. **`every_corpus_bound_bonus_var_target()`** — the full corpus-wide union of every `BONUS:VAR`
   target name any `class`/`class_feature` record binds anywhere, PRE-gated or not (built from the
   SAME two tables `class_feature_bonus_vars_any_record`/`class_record_bonus_vars` the header/
   member merges already use, so it can never disagree with what those tables ingested). If the
   identifier IS in this set, it is a REAL, possibly-nonzero conditional PCGen value this resolver
   cannot see from here — **refuses, unchanged**, preserving the exact safety property cycles 2 and
   5 proved load-bearing (disabling a refusal once produced a live fabricated value).
2. **`corpus_define_literal_defaults()`** (new) — every `DEFINE:<name>|<literal integer>` fact any
   corpus `class_feature`/`class` record carries. If the identifier is genuinely absent from (1)
   but present here, it binds to the REAL corpus-declared literal (the Bloodrager case above).
3. Otherwise (absent from both) — binds to `0`, real PCGen's own catch-all default (Sorcerer/
   Cleric/Shaman's bare `<Pool>PowerNLVLBonus` family, which carries no `DEFINE` either).

## 3. A locked test corrected, not silently widened (`§1a`)

`resolve_pcgen_var_chain_never_binds_an_unreachable_identifier` used a MADE-UP name
(`SiblingRecordOwnVariable`) standing in for "an identifier the chain can never reach", asserting
the whole formula stayed unbound. Step 1's oracle trace proves that assumption wrong for a name
genuinely absent from the corpus under every condition — real PCGen's own answer there is 0, not a
refusal. **Corrected, not deleted**: renamed to
`resolve_pcgen_var_chain_never_binds_an_identifier_bound_elsewhere_in_the_corpus` and rewritten to
use a REAL corpus `BONUS:VAR` target (`AssassinPoisonSaveBonus`, `data/corpus/core_rulebook/
class_feature/assassin/save_against_poisons.json`) that exists elsewhere but is unreachable from
the test's own local `bonus_vars` map — this is the REAL safety property the original test was
reaching for, and it still refuses, proven green. Two new tests added: `..._defaults_a_corpus_wide_
unbound_identifier_to_zero` (the bare-0-fallback shape) and `..._binds_a_real_corpus_define_zero_
baseline` (the concrete Bloodrager `Draconic` case, `Bloodrager_Draconic_BloodlinePower1LVL` = 7 at
level 7, `BloodragerBloodlinePower1LVLBonus` resolved via the real DEFINE, not the bare fallback).

## 4. `§17a` re-derivation: real movement, and an honest non-movement

```
cargo test --locked --lib -- generic_pool_group_selection_wiring_tests::pool_group_closure_census_across_all_six_pools --nocapture
```
```
Sorcerer Bloodline: 31/53 groups carry a resolvable member   (was 18/53, +13)
Bloodrager Bloodline: 5/12 groups carry a resolvable member  (unchanged)
Cleric Domain: 26/72 groups carry a resolvable member        (unchanged)
Shaman Spirit: 8/14 groups carry a resolvable member         (unchanged)
Warpriest Blessing: 0/37 groups carry a resolvable member    (unchanged)
Cavalier Order: 1/9 groups carry a resolvable member         (unchanged)
```

**Sorcerer Bloodline is the real, verified movement**: its bare `BloodlinePowerNLVLBonus` family
(no `DEFINE` anywhere, `grep -rl "VAR|BloodlinePower1LVLBonus" data/corpus/` → 0 hits as a target)
hits the pure bare-0-fallback path directly, and 13 more groups now carry at least one resolvable
member as a result.

**Bloodrager, Cleric, Shaman, Warpriest, Cavalier are honestly unchanged** — re-run and re-checked,
not assumed. This cycle's own unit tests prove the fix DOES reach real Bloodrager formulas (e.g.
`Bloodrager_Draconic_BloodlinePower1LVL` now resolves through a real corpus `DEFINE:
BloodragerBloodlinePower1LVLBonus|0`), but a brief live diagnostic (`group_has_a_resolvable_member`
run per-group, printed, then removed before commit) showed `Draconic Bloodrager Bloodline` still
`false` at the group level — `resolve_pool_member_sole_magnitude` has its OWN independent per-
member refusals (the single-terminal-target rule, header-chain gaps) beyond var-chain resolution
that this cycle did not trace to a root cause. **Named as an open question for a future cycle,
not guessed at or silently absorbed into this cycle's own claim.**

## 5. Not attempted (named, per `§17`)

`classlevel("X","APPLIEDAS=NONEPIC")`'s real 2-argument form and Hunter Animal Focus: unchanged
from cycles 9/10/11. Cavalier's 8 real no-`BONUS:VAR` Orders: unchanged. Oracle Mystery: stays
withdrawn, `oracle_dispatch_widening_safety_tests::a_mystery_pick_alone_grounds_no_tier_one_
revelation` untouched (still green, see §6). Rows 11/15 left `in-progress`, untouched;
`apps/desktop`'s row 19/20 lanes not touched. `data/corpus/**` untouched throughout (`git status
--porcelain -- data/corpus` — 0 changes).

## 6. Tests, RED→GREEN (`§1a`)

New tests, all green: `resolve_pcgen_var_chain_never_binds_an_identifier_bound_elsewhere_in_the_
corpus` (corrected), `resolve_pcgen_var_chain_defaults_a_corpus_wide_unbound_identifier_to_zero`,
`resolve_pcgen_var_chain_binds_a_real_corpus_define_zero_baseline`. Mutation-style proof inherent
in the correction itself: with this cycle's own code REVERTED (temporarily, verified via a local
diff before restoring — never committed), the ORIGINAL pinned `the_live_scale_of_this_waves_
widening_is_measured_and_pinned` tuple `(136, 20, 11, 9, 36)` reproduces exactly; with the fix in
place it moves to `(136, 21, 11, 8, 36)` — the ONE newly-resolved record identified and named
(`mystic theurge/Mystic Theurge ~ Combined Spells@1`, `CombinedSpellsMaxLevel|(CombinedSpellsLVL+1)/2`
resolving `CombinedSpellsLVL` through its own real `DEFINE:CombinedSpellsLVL|0`).

```bash
cargo test --locked --lib -- rules_core::pilot_compute::
```
```
test result: ok. 951 passed; 0 failed; 0 ignored; 0 measured; 1843 filtered out; finished in 25.42s
```
951/951 (up from cycle 11's 946/946 — +5: three new tests above, plus the census re-derivation and
the pinned-scale test's own count, no other net change).

```bash
cargo test --locked --lib -- rules_core::derived_evaluator_fixture_check::
```
```
test result: ok. 121 passed; 0 failed; 0 ignored; 0 measured; 2673 filtered out; finished in 8.93s
```

```bash
cargo test --locked --lib -- oracle_dispatch_widening_safety_tests
```
```
test result: ok. 48 passed; 0 failed; 0 ignored; 0 measured; 2746 filtered out; finished in 4.17s
```
`a_mystery_pick_alone_grounds_no_tier_one_revelation` untouched, still green (verified individually
too) — Oracle Mystery stays withdrawn per its own standing ruling.

```bash
cargo test --locked --lib -- cavalier
```
```
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 2778 filtered out; finished in 8.74s
```

## 7. Audits

- **Identifier audit** (`grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'` against
  `git diff -- src/rules_core/pilot_compute/mod.rs src/rules_core/pilot_compute/
  class_feature_grant_consumer.rs`, scoped to this cycle's own diff per §6 step 2's note): `OK_NO_
  BUNDLE_TAGS` — 0 hits.
- **Wired-integration audit** (`grep -inE "STUB|MOCK|placeholder|not.?yet.?implemented|todo|fixme|
  hack"`, same scope): `OK_NO_TOKENS` — 0 hits.
- **PI audit**: `pi_scrub.normalized_term_hits(...)` (imported via `scripts/pi_scrub.py`, never
  copied), scoped to `git diff --unified=0` of this cycle's own diff → `[]` (0 hits). `data/
  corpus/**` untouched throughout.

## 8. Kanban

`docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 18 only (cycle field 11 →
12, Notes appended). Verified: 21 distinct `^| N |` rows, 0 duplicates, row 18 parses to 9 cells
before and after (backtick-aware parser). Rows 11/15 confirmed still `in-progress`, untouched.

## 9. `df -h /`

```
Filesystem      Size  Used Avail Use% Mounted on
/dev/sda1       968G  497G  472G  53% /
```
