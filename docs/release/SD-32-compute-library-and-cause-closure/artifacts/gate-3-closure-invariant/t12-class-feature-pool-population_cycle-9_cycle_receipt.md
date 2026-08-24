# Cycle t12-class-feature-pool-population, cycle 9 — Gate 3 (closure invariant) / row 18 (`epic-8-pool-shaped-class-features`)

- **Card ID:** row 18 (`epic-8-pool-shaped-class-features`), governed by `decisions.md §27b`
  (*"EVERYTHING"*). Dispatched scope: two jobs. (1) Re-derive, across ALL SIX real pools sharing
  `push_generic_pool_group_selection_magnitude` (not just the three cycle 8 measured), how many
  groups the cross-book bare-key-header-merge + `DomainLVL` class-record fixes now unblock. (2)
  Widen `formula_interpreter.rs`'s `classlevel` grammar to accept a bare, zero-argument call — a
  real corpus shape (`book_of_the_damned_volume_2/demoniac.json`) row 20 cycle 4 named in a
  passing test without editing this row's own file.
- **Base:** `git merge-base --is-ancestor "$PIN" HEAD` returned FALSE at session start (worktree
  `HEAD` was `1bb523773d`, PR #374's merge, far behind `$PIN`). Fixed: `git reset --hard "$PIN"`
  then `git rebase origin/tranche/12` — fast-forward, no-op (`origin/tranche/12` HEAD == `$PIN` ==
  cycle 8's own commit `ef4a6ffca2`). `BASE_OK` re-verified after. `git log origin/tranche/12`
  checked at session start for recent activity in `formula_interpreter.rs`,
  `class_feature_grant_consumer.rs`, `pilot_compute/mod.rs` — none since cycle 8's own commit (the
  pin itself); no sibling-lane collision on these files (a sibling worktree, `wf_c237d149-02d-2`,
  was independently running `cargo test companion_base_stat_table` against row 20's own files in
  its own target dir throughout this cycle — no overlap).
- **Oracle:** fresh worktree, empty git-ignored slot as expected. Bootstrapped via
  `scripts/fetch-pcgen-oracle.sh --dest <repo-local artifacts/corpus/operator-supplied/pcgen>`;
  pin confirmed `7f818006e371`. (The pinned oracle mirror ships `data/`/`system/` only, no `.java`
  sources — the module doc's `ClassLevelCommand.java` citations are prior-cycle knowledge, not
  independently re-verifiable from this mirror; job 2's widening is scoped to the GRAMMAR shape
  only, per the dispatch brief, not a new claim about the real function's own zero-arg semantics.)
- **Files touched:**
  - `src/rules_core/pilot_compute/formula_interpreter.rs` — `parse_call`'s `"classlevel"` arm: a
    bare `classlevel()` (no string-literal argument) now parses to `Expr::ClassLevel(String::new())`
    instead of refusing outright, reusing the existing `CLASSLEVEL::<name>` lookup with `""` as the
    "no class name given" sentinel (no evaluator change needed — `format!("CLASSLEVEL::{class_name}")`
    with an empty `class_name` already produces the right key). No caller in this codebase binds the
    empty key yet, so evaluation still refuses cleanly until row 20 (or a future cycle) does. 2 new
    tests (`classlevel_with_no_argument_parses_and_reads_the_empty_key_binding`,
    `classlevel_with_no_argument_refuses_without_a_binding`), module doc updated.
  - `src/rules_core/pilot_compute/mod.rs` — `generic_pool_group_selection_wiring_tests`: new
    `real_groups_owned_by`/`group_has_a_resolvable_member` helpers (mirror
    `real_pool_group_for_selection_slug`'s own ownership + naming-shape gates, generalized across
    ALL group names for a class rather than resolving one slug) and a new locked test,
    `pool_group_closure_census_across_all_six_pools`.
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 18 only (cycle field
    8 → 9, Notes appended). Verified: 21 distinct `^| N |` rows, 0 duplicates, row 18 parses to 7
    cells before and after.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — `git diff --unified=0` over both touched
  `.rs` files, `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'` → 0 hits.
- **Wired-integration audit result:** `OK_NO_TOKENS` — same scope, `grep -inE
  "STUB|MOCK|placeholder|not.?yet.?implemented|todo|fixme|hack"` → 0 hits.
- **PI audit:** `pi_scrub.normalized_term_hits(...)` (imported, not copied) against this cycle's
  full diff text (both `.rs` files) → `[]` (0 hits). `data/corpus/**` untouched throughout
  (`git status --porcelain -- data/corpus` — 0 changes).
- **Status:** in-progress — genuine, multi-cycle epic, unchanged from cycles 1–8's own framing.

---

## 1. Job 1 — `§17a` re-derivation across ALL SIX pools (`§12c`: population + command named)

Cycle 8 measured the cross-book-header-merge / `class_record_bonus_vars` (`DomainLVL`) fixes'
effect against only 3 of the 6 real pools sharing `push_generic_pool_group_selection_magnitude`
(Sorcerer/Bloodrager/Cleric; Shaman measured but not locked into a test; Warpriest re-checked
unchanged). Cavalier Order was never measured by any prior cycle at all.

**Instrument validated against a known case first (`§17a`)**: a first version of the census
counted every `" ~ "`-qualified group majority-owned by the class, without also requiring the
group's own NAME to match the registered pool word (`"Bloodline"`/`"Domain"`/...) — this silently
inflated both numerator and denominator (Sorcerer measured 72 groups instead of the real 53,
4/72 instead of any reproduction of cycle 8's own 18/53). Adding the same naming-shape filter
`real_pool_group_for_selection_slug` already applies (exact `" <registered_name>"` suffix, or
case-insensitive `"<registered_name> of the "` prefix) fixed the denominators to match cycle 8's
own exactly (53/12/72/14/37). A second correction: measuring "does EVERY compute-needing member of
the group resolve" (a stricter bar than cycle 8 used) reproduced neither cycle 8's numerators nor
denominators (4/53, not 18/53) — switching to "does AT LEAST ONE member resolve", the SAME measure
cycle 5/8's own doc comments already use ("found only 15 ... carry a member resolvable"),
reproduced cycle 8's three baselines exactly. Both corrections are recorded as retro-shaped
findings in the test's own doc comment, not silently fixed and forgotten.

**Command**: `cargo test --locked --lib -- rules_core::pilot_compute::
generic_pool_group_selection_wiring_tests::pool_group_closure_census_across_all_six_pools
--nocapture`

```
Sorcerer Bloodline: 18/53 groups carry a resolvable member
Bloodrager Bloodline: 5/12 groups carry a resolvable member
Cleric Domain: 26/72 groups carry a resolvable member
Shaman Spirit: 8/14 groups carry a resolvable member
Warpriest Blessing: 0/37 groups carry a resolvable member
Cavalier Order: 1/9 groups carry a resolvable member
```

- **Sorcerer/Bloodrager/Cleric**: reproduce cycle 8's own figures exactly (18/53, 5/12, 26/72) —
  the fix's effect on these three is fully accounted for, no further movement.
- **Shaman Spirit (8/14)**: reproduces cycle 8's own untested measurement exactly. Now LOCKED by
  a real, mutation-proved test for the first time (cycle 8 named this as "measured ... but no new
  test added ... time-boxed").
- **Warpriest Blessing (0/37)**: reproduces cycle 8's own re-check — this family's remaining gap is
  genuinely neither of cycle 8's two fixes; unaffected, correctly still 0.
- **Cavalier Order (1/9)**: a **new figure, never previously measured by any cycle**. Cavalier's
  own pool uses the THIRD real corpus naming/ownership shape (`real_pool_group_for_selection_slug`'s
  own doc: `"Order of the X"`, header-record-proven ownership) — this census is the first time it
  has been run against that shape at all. 1 of 9 real Orders (Cavalier Order cycle 7 already
  hand-wired "Order of the Green" by name separately from this generic census) carries a
  resolvable member through the shared resolver.

No new chassis call site needed for any of the six — all resolve through the SAME six real
`push_generic_pool_group_selection_magnitude` call sites cycles 5–7 already wired (`§16`: this is
runtime reachability through already-wired plumbing, not a new mechanism or a static-catalog
change — `python3 scripts/census_class_feature_pool_population.py`'s own RESIDUAL figure is
unaffected by this cycle, same as cycle 8).

**Totals**: 58 of 197 real pool groups across the six families now carry at least one resolvable
member (up from a partial, 3-pool-only picture after cycle 8: 49 of 137 measured there). "Take the
largest first" (`§12c` scoping instruction): Cleric Domain (72 real groups, largest denominator)
and Sorcerer Bloodline (53, second largest) were already fully re-measured and confirmed unchanged
by cycle 8; this cycle's own largest NEW contribution is completing the picture for the 3 pools
cycle 8 left unmeasured/unlocked, largest of those first (Warpriest Blessing 37, Shaman Spirit 14,
Cavalier Order 9).

## 2. Job 2 — `formula_interpreter.rs`'s `classlevel` grammar widened to accept a bare, zero-argument call

Row 20 cycle 4 built a generic class-progression table; 60 of 61 real conventional PC classes
resolve. The 61st, `Demoniac` (`book_of_the_damned_volume_2`), fails because its BASEAB/SAVE
formulas call bare `classlevel()` with **no argument** (`classlevel()*3/4`, `(classlevel()+1)/2`,
`(classlevel()+1)/3`) — a shape the pre-existing grammar refused outright (parse error, before ever
reaching evaluation), named explicitly in row 20's own passing test
(`the_13_families_reproduce_cycle_3s_61_record_conventional_population_minus_one_named_gap`,
asserting `unresolved == [("book_of_the_damned_volume_2", "Demoniac")]`) rather than special-cased
around in a file outside its own write scope.

Fixed, in this row's own live-owned file: `parse_call`'s `"classlevel"` arm now checks for an
immediate `RParen` before requiring a string literal; on a bare `classlevel()` it parses to
`Expr::ClassLevel(String::new())`, reusing the EXACT SAME `CLASSLEVEL::<name>` lookup cycle 6 built
for the named form, with the empty string as the "no class name given" sentinel — the same
"unowned = `\"\"`, never fabricated" convention this codebase already applies elsewhere (row 18
cycle 8's own bare-key header merge). This is a pure PARSER-shape widening, not a semantic one: no
consumer in this codebase binds the empty `CLASSLEVEL::` key today, so `classlevel()` still refuses
cleanly at evaluation time, exactly the "refuse, never guess" contract this module holds
everywhere — it now fails for a DIFFERENT, later reason (unbound variable) than before (parse
error), which is exactly what unblocks row 20: a caller (row 20's own `class_catalog_generic.rs`,
outside this row's write scope, untouched here) can now bind `CLASSLEVEL::` to Demoniac's own class
level the same way it already binds `CLASSLEVEL::APPLIEDAS=NONEPIC` for the other observed shape.

**Row 20's lane: this unblocks your 61st.** `formula_interpreter.rs`'s grammar now accepts
`classlevel()`; bind `vars.insert("CLASSLEVEL::".to_string(), i64::from(level))` in
`class_catalog_generic.rs`'s per-record loop (mirrors the existing `CLASSLEVEL::APPLIEDAS=NONEPIC`
binding at line ~315) and Demoniac should resolve the same way the other 60 already do. Not built
here — `class_catalog_generic.rs` is row 20's own live territory, per the dispatch brief.

## 3. Tests, RED→GREEN, both altitudes (`§1a`)

**Job 2 (`formula_interpreter.rs`)**: 2 new tests. Mutation: the new `if let Some(Tok::RParen) =
self.peek() { ... }` branch wrapped in `if false { ... }` → re-ran
`rules_core::pilot_compute::formula_interpreter::` (34 tests): the new
`classlevel_with_no_argument_parses_and_reads_the_empty_key_binding` FAILED for the intended reason
(`"classlevel(...) expects a string literal class name or no argument, got Some(RParen)"`), 33
others passed. Reverted; re-ran, 34/34 green.

**Job 1 (`mod.rs` census test)**: mutation: `group_has_a_resolvable_member` short-circuited to
`if true { return false; }` → re-ran `pool_group_closure_census_across_all_six_pools`: FAILED for
the intended reason (every pool reported `0/N`, the assertion's own baseline-match check caught
it). Reverted; re-ran, green.

**Regression, scoped**:

```bash
cargo test --locked --lib -- rules_core::pilot_compute::
```
```
test result: ok. 932 passed; 0 failed; 0 ignored; 0 measured; 1843 filtered out; finished in 26.90s
```
932/932 (up from cycle 8's 929/929: +3 net new tests — 2 `formula_interpreter` + 1 census — 0
broken).

```bash
cargo test --locked --lib -- rules_core::derived_evaluator_fixture_check::
```
```
test result: ok. 121 passed; 0 failed; 0 ignored; 0 measured; 2654 filtered out; finished in 4.83s
```

```bash
cargo test --locked --lib -- oracle_dispatch_widening_safety_tests
```
```
test result: ok. 48 passed; 0 failed; 0 ignored; 0 measured; 2727 filtered out; finished in 6.64s
```

```bash
cargo test --locked --lib -- cavalier
```
```
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 2759 filtered out; finished in 8.84s
```

Oracle Mystery's own safety test (`a_mystery_pick_alone_grounds_no_tier_one_revelation`) not
touched this cycle; Oracle Mystery stays withdrawn, per its own standing ruling — untouched.

## 4. Scope discipline — did not attempt, real scoped follow-on, named rather than silently deferred

- **Hunter Animal Focus** (21 real records) — activation-gated; still untouched. This cycle's time
  went to (1) the §17a re-derivation across all six pools (the dispatched priority, "take the
  largest first" already satisfied by cycle 8's own Cleric/Sorcerer coverage) and (2) the
  formula-grammar widening unblocking a sibling row. A future cycle should pick Hunter up.
- **`class_feature_record_tokens_pre_gate_safe`'s own member-table cross-book merge** (155 real
  duplicated member keys, named by cycle 8 as deliberately left unmerged — narrower/riskier scope,
  the `class` field is load-bearing elsewhere) — still not attempted; unchanged from cycle 8's own
  sizing.
- **Oracle Mystery** — untouched, stays withdrawn. Safety test not touched.
- **Cavalier's remaining 8 Order groups** (of 9 real, only 1 resolves) — sized by this cycle's own
  census, not investigated further (same family shape as Sorcerer/Bloodrager/Cleric's own
  unresolved majority: likely the SAME per-order "Tracker"/header-chain gap, unconfirmed this
  cycle — named for a future cycle rather than guessed at).
- **`classlevel("X", "APPLIEDAS=NONEPIC")`'s real 2-argument form** — still not investigated (named
  in this module's own doc as "not covered", unchanged by this cycle's zero-argument widening,
  which is a genuinely separate grammar shape).
- Rows 11/15 (left `in-progress`, untouched); `apps/desktop`'s row 19/20 lanes not touched beyond
  the read-only investigation this cycle's brief directed (no edits to `class_catalog_generic.rs`
  or any `apps/desktop` file). `data/corpus/**` untouched throughout.

`df -h /`: reported in the dispatch's final report.
