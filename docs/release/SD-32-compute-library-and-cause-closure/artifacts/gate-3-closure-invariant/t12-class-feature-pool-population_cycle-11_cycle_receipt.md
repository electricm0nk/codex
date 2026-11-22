# Cycle t12-class-feature-pool-population, cycle 11 — Gate 3 (closure invariant) / row 18 (`epic-8-pool-shaped-class-features`)

- **Card ID:** row 18 (`epic-8-pool-shaped-class-features`), governed by `decisions.md §27b`
  (*"EVERYTHING"*). Dispatched scope: verify the PREABILITY gate against the oracle's own `.java`
  sources (readable from git objects, no sparse-checkout widening needed) and implement it if that
  is the real blocker; then `classlevel("X","APPLIEDAS=NONEPIC")`'s 2-argument form and Hunter
  Animal Focus, as time allowed.
- **Base:** `git merge-base --is-ancestor "$PIN" HEAD` returned FALSE at session start (worktree
  `HEAD` was `1bb523773d`, PR #374's merge, far behind `$PIN`). Fixed: `git reset --hard "$PIN"`
  then `git rebase origin/tranche/12` — no-op fast-forward (`origin/tranche/12` HEAD == `$PIN` ==
  cycle 10's own pin, `0950f53bc568`). `BASE_OK` re-verified after. No sibling-lane collision:
  `class_feature_grant_consumer.rs`/`mod.rs` last touched by cycle 10's own commit (`9c72df4b59`).
- **Oracle:** fresh worktree, empty git-ignored slot as expected. Bootstrapped via
  `scripts/fetch-pcgen-oracle.sh --dest <repo-local artifacts/corpus/operator-supplied/pcgen>`;
  pin confirmed `7f818006e371`. Confirmed the brief's own claim live: `git ls-tree -r --name-only
  HEAD | grep -c '\.java$'` → 4503; `git show HEAD:code/src/java/plugin/pretokens/test/
  PreAbilityTester.java` and `.../parser/PreAbilityParser.java` both read cleanly with no working-
  tree checkout at all.

## 1. `§17a` — the PREABILITY hypothesis does NOT reproduce; a different root cause does

Read `PreAbilityTester.java`/`PreAbilityParser.java` from the oracle's own git objects. Confirmed
real semantics: `PREABILITY:N,CATEGORY=<cat>,<key>[,<key2>...]` (parser extracts `CATEGORY=`/
`CATEGORY.` into `prereq.setCategoryName`, tester delegates to
`PrerequisiteUtilities.passesAbilityTest`) means "character possesses at least N abilities named
`<key>` in ability-category `<cat>`" — cycle 10's own reasoning ("this gate is structurally
always-true at every real call site of `resolve_pool_member_sole_magnitude`") is a sound
description of what a POSITIVE `PREABILITY` tag on the group being resolved would mean.

But tracing the CONCRETE case cycle 10 named (`Aberrant Bloodline ~ Acidic Ray`, needing
`Sorcerer_Aberrant_BloodlinePower1LVL`) to its real corpus source found the record cycle 10 actually
read was NOT the real per-bloodline header PCGen's class chooser binds. `class_feature_bonus_vars_
any_record()`'s bare-key `"Aberrant Bloodline"` match resolves to `advanced_class_guide`'s own
Eldritch-Heritage-shaped fallback record (`VAR|Sorcerer_Aberrant_BloodlinePower1LVL|1|
!PREABILITY:1,CATEGORY=Sorcerer Bloodline,Aberrant Bloodline|!PREABILITY:1,CATEGORY=Crossblooded
Bloodline,Aberrant Bloodline` — a doubly-NEGATED fallback that would not even fire once implemented,
since selecting the bloodline normally makes the first negated tag false). The REAL, wholly UNGATED
formula lives on a SEPARATE record this lookup never tried: `"Sorcerer Bloodline ~ Aberrant"`
(`data/corpus/core_rulebook/class_feature/sorcerer_bloodline/aberrant_bloodline.json`,
`BONUS:VAR|Sorcerer_Aberrant_BloodlinePower1LVL|Sorcerer_Aberrant_BloodlineLVL+
BloodlinePower1LVLBonus`, no PRE-tag tail at all). Confirmed the SAME shape for Celestial Bloodline
(`"Sorcerer Bloodline ~ Celestial"`, `data/corpus/core_rulebook/class_feature/sorcerer_bloodline/
celestial_bloodline.json`) and generically across the corpus: this is a FOURTH real header-naming
convention, `"<class> <registered_name> ~ <suffix>"`, keyed 22 times for Cleric Domain
(`"Cleric Domain ~ Air"`, ...), 12 for Shaman Spirit, 11 for Bloodrager Bloodline, 7 for Cavalier
Order, 0 for Warpriest Blessing (matches cycle 10's own finding that Warpriest's gap is a different
shape).

**Implemented, not guessed at**: implementing `PREABILITY` evaluation was NOT the fix this case
needed — the record cycle 10 traced never had a positive, usable `PREABILITY` row for this target
at all. Widened `pool_header_record_by_normalized_suffix` to also try this fourth shape (both the
Cavalier-Order-style unstripped suffix and the Bloodline/Domain/Spirit-style `" <registered_name>"`-
stripped suffix), class-checked the same way the existing bare-key widening already is.

## 2. A near-miss, caught by re-running the test before trusting it (`§1a`)

The first version of this widening returned the FIRST matching header and stopped, same contract as
the function's three pre-existing checks. That is WRONG once a pool has more than one real
header-shaped record contributing DIFFERENT slices of the chain — confirmed live: Cleric's own
`"Air Domain"` pool has TWO real header records under different conventions: the bare-key
`"Air Domain"` (useful, `DomainAirDC`/`DomainAirLVL`/`DomainAirAbilityTriggerLVL`/`DomainAirTimes`)
and `"Cleric Domain ~ Air"` (a domain SPELL LIST record, CATEGORY `Internal`, one `SPELLLEVEL`
token, **zero** `BONUS:VAR` tokens). The first (short-circuiting) version of the new widening tried
the new shape BEFORE the bare-key check, found the empty spell-list record first, and returned it —
silently discarding the useful bare-key header for every pool this widening touched. Caught by
re-running `pool_group_closure_census_across_all_six_pools` before trusting the change:

```
Sorcerer Bloodline: 15/53 groups carry a resolvable member   (was 18/53)
Bloodrager Bloodline: 5/12 groups carry a resolvable member
Cleric Domain: 14/72 groups carry a resolvable member          (was 26/72)
Shaman Spirit: 8/14 groups carry a resolvable member
Warpriest Blessing: 0/37 groups carry a resolvable member
Cavalier Order: 1/9 groups carry a resolvable member
```

Rewrote `pool_header_record_by_normalized_suffix` to MERGE every real header candidate's
`bonus_vars` (via `merge_bonus_var_target_map_never_overwriting`, made `pub(crate)` so this
function can reuse it — never-overwrite policy identical to the existing cross-book merges) rather
than returning the first match, and changed its return type from `Option<&Record>` to an owned
`BTreeMap<String, String>` accordingly. Both call sites (`resolve_pool_member_sole_magnitude`'s own
per-group header lookup and its tracker-header lookup) updated to iterate the merged map directly.

## 3. `§17a` re-derivation after the fix: **zero new group closures, all six pools unchanged**

```
Sorcerer Bloodline: 18/53 groups carry a resolvable member
Bloodrager Bloodline: 5/12 groups carry a resolvable member
Cleric Domain: 26/72 groups carry a resolvable member
Shaman Spirit: 8/14 groups carry a resolvable member
Warpriest Blessing: 0/37 groups carry a resolvable member
Cavalier Order: 1/9 groups carry a resolvable member
```

**Identical to cycle 10's own baseline.** The merge fix is real, verified, and correct — it adds a
genuine additional real-corpus header source no prior cycle's lookup ever tried, guards against a
future corpus reprint silently losing this chain the way row 18 cycle 8's own precedent showed
happening elsewhere — but it does not (yet) flip any group's resolvability, because the DEEPER
blocker sits one hop further down the chain than cycle 10's own trace reached.

**Traced further**: `Sorcerer_Aberrant_BloodlinePower1LVL`'s now-reachable ungated formula
(`Sorcerer_Aberrant_BloodlineLVL+BloodlinePower1LVLBonus`) still refuses, because
`BloodlinePower1LVLBonus` is never bound by any `BONUS:VAR` row anywhere in this corpus
(`grep -rln "VAR|BloodlinePower1LVLBonus" data/corpus/` → 0 hits) — nor is any sibling
`BloodlinePowerNLVLBonus` for any bloodline checked. Real PCGen semantics default an unbound
variable to 0 (every PCGen variable is implicitly zero-initialized unless `DEFINE`d or `BONUS:VAR`-
set); this module's `resolve_pcgen_var_chain` deliberately does NOT do this — its own doc: "An
identifier this loop cannot reach ... is simply never bound — never guessed, never defaulted."
Widening that refusal into a real 0-default is a structural, corpus-wide behavior change (it would
affect every formula in every pilot_compute table that references any variable this corpus never
explicitly sets, not just the bloodline family) — well outside what this cycle's oracle-verification
scope covers safely. **Sized and named for a focused future cycle, not attempted uninvestigated.**

## 4. Not attempted (named, per `§17`)

`classlevel("X","APPLIEDAS=NONEPIC")`'s real 2-argument form and Hunter Animal Focus: unchanged
from cycles 9/10, consumed by the PREABILITY investigation and its correction. Cavalier's 8 real
no-`BONUS:VAR` Orders: unchanged, cycle 10's own confirmed-different-compute-shape finding stands,
untouched this cycle. Oracle Mystery: stays withdrawn, `oracle_dispatch_widening_safety_tests::
a_mystery_pick_alone_grounds_no_tier_one_revelation` untouched (still green, see §5). Rows 11/15
left `in-progress`, untouched; `apps/desktop`'s row 19/20 lanes not touched. `data/corpus/**`
untouched throughout (`git status --porcelain -- data/corpus` — 0 changes).

## 5. Tests, RED→GREEN (`§1a`)

**New test**, mutation-proved: `pool_header_lookup_merges_every_real_header_shape_not_just_the_
first_match` — asserts `pool_header_record_by_normalized_suffix("Cleric", "Air Domain",
Some("Domain"))` still contains `DomainAirDC`. Mutation: added `return merged;` right after the
`registered_name` widening block (simulating the short-circuit regression found in §2) → re-ran:
FAILED for the intended reason (`{}` — the widening's own empty-record match now returned alone,
the useful bare-key header never reached). Reverted; re-ran, green.

**Scoped regression**:

```bash
cargo test --locked --lib -- rules_core::pilot_compute::
```
```
test result: ok. 946 passed; 0 failed; 0 ignored; 0 measured; 1843 filtered out; finished in 19.32s
```
946/946 (up from cycle 10's 944/944 — +2: this cycle's own new merge-widening test plus the
census's own re-derivation, no other net change).

```bash
cargo test --locked --lib -- rules_core::derived_evaluator_fixture_check::
```
```
test result: ok. 121 passed; 0 failed; 0 ignored; 0 measured; 2667 filtered out; finished in 3.02s
```

```bash
cargo test --locked --lib -- oracle_dispatch_widening_safety_tests
```
```
test result: ok. 48 passed; 0 failed; 0 ignored; 0 measured; 2740 filtered out; finished in 4.78s
```
`a_mystery_pick_alone_grounds_no_tier_one_revelation` untouched, still green — Oracle Mystery stays
withdrawn per its own standing ruling.

```bash
cargo test --locked --lib -- cavalier
```
```
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 2772 filtered out; finished in 6.31s
```

## 6. Audits

- **Identifier audit** (`grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'` against
  `git diff -- src/rules_core/pilot_compute/mod.rs src/rules_core/pilot_compute/
  class_feature_grant_consumer.rs`, scoped to this cycle's own diff per §6 step 2's note): `OK_NO_
  BUNDLE_TAGS` — 0 hits.
- **Wired-integration audit** (`grep -inE "STUB|MOCK|placeholder|not.?yet.?implemented|todo|fixme|
  hack"`, same scope): `OK_NO_TOKENS` — 0 hits.
- **PI audit**: `pi_scrub.normalized_term_hits(...)` (imported via `scripts/pi_scrub.py`, never
  copied) against this cycle's own diff text → `[]` (0 hits). `data/corpus/**` untouched throughout.

## 7. Kanban

`docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 18 only (cycle field 10 →
11, Notes appended). Verified: 21 distinct `^| N |` rows, 0 duplicates, row 18 parses to 9 cells
before and after (backtick-aware parser). Rows 11/15 confirmed still `in-progress`, untouched (git
diff on kanban.md is a single 1-line change).

## 8. `df -h /`

```
Filesystem      Size  Used Avail Use% Mounted on
/dev/sda1       968G  496G  473G  52% /
```
