# Cycle t12-class-feature-pool-population, cycle 8 — Gate 3 (closure invariant) / row 18 (`epic-8-pool-shaped-class-features`)

- **Card ID:** row 18 (`epic-8-pool-shaped-class-features`), governed by `decisions.md §27b`
  (*"EVERYTHING"*). Dispatched scope: re-derive cycle 7's own named blocker (`§17a`) now that row
  21 restored the `.MOD`-appended tokens cycle 7 traced it to, then wire Hunter Animal Focus and/or
  whatever the re-derivation newly unblocks, largest first. Oracle Mystery stays withdrawn.
- **Base:** `git merge-base --is-ancestor "$PIN" HEAD` returned FALSE at session start
  (`PIN=f6390421c9ae5f7c7b92cecb192553c0161222d6`, worktree started on a stale lineage). Fixed:
  `git reset --hard "$PIN"` then `git rebase origin/tranche/12` — fast-forward, no-op
  (`origin/tranche/12` HEAD == `$PIN`). `BASE_OK` re-verified after. `git log origin/tranche/12`
  checked at session start for recent activity in this cycle's target files
  (`class_feature_pool_catalog.rs`, `class_feature_grant_consumer.rs`, `pilot_compute/mod.rs`) —
  none since row 21's own `f6390421c9` (the pin itself); no sibling-lane collision.
- **Oracle:** fresh worktree, empty git-ignored slot as expected. Bootstrapped via
  `scripts/fetch-pcgen-oracle.sh --dest <repo-local artifacts/corpus/operator-supplied/pcgen>`;
  pin confirmed `7f818006e371`.
- **Files touched:**
  - `src/rules_core/pilot_compute/class_feature_grant_consumer.rs` — `class_feature_bonus_vars_
    any_record`'s table builder changed from `or_insert_with` (first-book-wins) to a per-target
    MERGE across every book sharing the same bare key, and widened to tolerate `class: null`
    (kept `""`, never fabricated) rather than excluding the record; new `class_record_bonus_vars`
    table (`data/corpus/*/class/*.json`, keyed by `class_id`) mirroring the existing header table
    one directory level up.
  - `src/rules_core/pilot_compute/mod.rs` — `pool_header_record_by_normalized_suffix`'s bare-key
    branch widened to accept `header.class.is_empty()`; `resolve_pool_member_sole_magnitude` gained
    a `registered_name_for_tracker: Option<&str>` parameter (merges a second, class-wide shared
    "Tracker" header via the SAME `pool_header_record_by_normalized_suffix` lookup) and an
    unconditional third merge from the new `class_record_bonus_vars` table; both existing call
    sites (`push_generic_pool_choice_magnitude` passes `None`/no new call, `push_generic_pool_
    group_selection_magnitude` passes `Some(registered_name)`) updated; 3 new tests in
    `generic_pool_group_selection_wiring_tests`.
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 18 only (cycle field
    6 → 7, Notes appended). Verified: 21 distinct `^| N |` rows, 0 duplicates, row 18 parses to 7
    cells before and after.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — `git diff --unified=0` over both touched
  `.rs` files, `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'` → 0 hits.
- **Wired-integration audit result:** `OK_NO_TOKENS` — same scope, `grep -inE
  "STUB|MOCK|placeholder|not.?yet.?implemented|todo|fixme|hack"` → 0 hits.
- **PI audit:** `pi_scrub.normalized_term_hits(...)` (imported, not copied) against this cycle's
  full diff text (both `.rs` files) → `[]` (0 hits). `data/corpus/**` untouched throughout
  (`git status --porcelain -- data/corpus` — 0 changes).
- **Status:** in-progress — genuine, multi-cycle epic, unchanged from cycles 1–7's own framing.

---

## 1. `§17a` re-derivation: cycle 7's 0-groups-closable finding does NOT reproduce as "fully fixed by row 21 alone" — two further, code-level gaps found and fixed this cycle

Cycle 7 traced the Sorcerer Bloodline/Cleric Domain/Bloodrager Bloodline/Shaman Spirit blocker one
hop deeper than the record-level `PREABILITY` gate, to a corpus-ingestion gap: `bloodline_
tracker.json`'s own `.MOD`-appended `BONUS:VAR` rows (`BloodlineLVL`, `BloodlineCasterLVL`,
`BloodlineProgressionLVL`) were dropped during ingestion, and escalated it — correctly, per
`§27b` — as out of this row's guarded-path-only write scope.

Row 21 fixed that: `core_rulebook/class_feature/bloodline_tracker/bloodline_tracker.json` now
carries all three vars (confirmed live, 308 real tokens including the full `BloodlineLVL`/
`BloodlineCasterLVL`/`BloodlineProgressionLVL`/`BloodlinePowerTimes`/`BloodlineFeatProgression`
chain). Per `§17a`, re-derived rather than trusting that this alone closes the family — it does
not. A direct measurement (`resolve_pool_member_sole_magnitude` probed against every real
Sorcerer/Bloodrager Bloodline/Cleric Domain group, level 5, real `class_feature_id_slug`d group
names, not a guess) found the count UNCHANGED from cycle 7's own baseline until two further,
purely code-level gaps — both squarely inside this cycle's `pilot_compute` write scope, never
`data/corpus` — were also fixed:

### 1a. Cross-book key collision in the table builder (blocks the WHOLE Bloodline family)

`class_feature_bonus_vars_any_record`'s table builder iterates book directories in sorted order
and used `out.entry(key).or_insert_with(...)` — first book wins, every later book's SAME-keyed
record silently discarded. This corpus's real per-bloodline HEADER records are NOT book-unique:
`"Bloodline Tracker"` alone is real-ingested from 8 separate books (`core_rulebook`,
`advanced_class_guide`, `advanced_players_guide`, `advanced_race_guide`, `occult_adventures`,
`ultimate_combat`, `ultimate_magic`, `monster_codex`), and a direct corpus survey found 154 MORE
bare `class_feature` keys sharing this exact shape (e.g. `"Verdant Bloodline"` in 4 books,
`"Celestial Bloodline"` in 3) — each book's own copy of the SAME real ability carries a DIFFERENT
subset of its `.MOD`-appended rows, the identical per-book `.MOD`-collision defect row 21 fixed at
the per-FILE level, surviving here at the per-KEY, cross-file level. Since `"advanced_class_guide"`
sorts before `"core_rulebook"`, the prior code kept ACG's single leftover `DEFINE` token and
discarded `core_rulebook`'s own complete 308-token copy — even after row 21's real fix landed the
correct data, the CONSUMER never read it.

Fixed: merge `bonus_vars` per target name across every book sharing a key (`.or_insert`, never
overwriting an already-bound target — the same collision policy `parse_bonus_var_tokens_pre_gate_
safe` already applies within one record, extended from "one record" to "one key, merged across
books"). Also widened the same table to tolerate a real header record's `class: null` (confirmed
live on every per-bloodline header — `"Marid Bloodline"`, `"Draconic Bloodline"`, `"Aberrant
Bloodline"`, ...) as unowned (kept `""`, never fabricated) rather than excluding the record
outright — safe because a bare, un-namespaced header key is already globally unique corpus-wide (no
other class defines a same-named bare-key header), so this carries none of the cross-class-
collision risk the pre-existing `header.class == class` ownership check exists to prevent.

### 1b. A second, class-wide shared header the per-group merge never reached

Even with 1a fixed, a per-bloodline member's chain (e.g. `Sorcerer_Abyssal_BloodlineLVL ->
BloodlineLVL`) still needed `BloodlineLVL` itself, defined only on `"Bloodline Tracker"` — a
SEPARATE record from the per-group header (`"Abyssal Bloodline"`) `resolve_pool_member_sole_
magnitude` already merges. `pool_header_record_by_normalized_suffix` was never called a second
time to find it. Fixed generically: `resolve_pool_member_sole_magnitude` gained `registered_name_
for_tracker: Option<&str>`; when `Some("Bloodline")`, it ALSO merges `pool_header_record_by_
normalized_suffix(owning_class, "Bloodline Tracker")` — reusing the exact same generic lookup
function (already handling BOTH the bare-key shape Sorcerer's own tracker uses and the `"<class> ~
<name>"` shape Bloodrager's own `"Bloodrager ~ Bloodline Tracker"` uses), not a new resolver or a
per-class table (`§17`).

### 1c. Cleric Domain's own SEPARATE, larger gap: `DomainLVL` lives on the CLASS record, not any `class_feature` record

Cycle 7 already found this is a genuinely different, larger gap than Bloodline's: `BONUS:VAR|
DomainLVL|ClericLVL` binds on `core_rulebook/class/cleric.json` (the CLASS record itself,
confirmed live), never on any `class_feature` record at all — out of `class_feature`'s ingestion
scope entirely. Row 21 restored `raw_tokens` onto all 168 real class records (previously absent);
this cycle built the missing READ side: `class_record_bonus_vars()` (new function, mirrors `class_
feature_bonus_vars_any_record`'s own shape one directory level up: scans `data/corpus/*/class/
*.json`, keyed by `class_id` — confirmed a plain display name, `"Cleric"`, never `"class:"`-
prefixed). `resolve_pool_member_sole_magnitude` merges this table unconditionally (no gating flag —
a class with no such record simply merges nothing, same as an absent header above).

## 2. Measured effect (`§17a`, `§12c` — population and command named)

Command: `resolve_pool_member_sole_magnitude` invoked directly (not the census script, which
measures the STATIC catalog, not runtime reachability — `§16`) against every real `" ~ "`-group
majority-owned by the given class, level 5, `AbilityModifiers::default()`:

```
Sorcerer Bloodline:    15/53 -> 18/53  (+3: Abyssal, Accursed, Rakshasa)
Bloodrager Bloodline:   4/12 ->  5/12  (+1: Verdant)
Cleric Domain:          5/72 -> 26/72  (+21: Animal, Artifice, Chaos, Charm, Community, Darkness,
                                        Destruction, Evil, Glory, Knowledge, Law, Liberation, Luck,
                                        Madness, Magic, Protection, Repose, Strength, Sun, Tactics,
                                        Travel, Weather)
Shaman Spirit:          measured 8/14 resolvable (Battle, Flame, Heavens, Life, Mammoth, Stone,
                                        Waves, Wind) via the same cross-book merge -- not wired to
                                        a new test this cycle, time-boxed, named in §5
Warpriest Blessing:     0/37 (unchanged from cycle 7 -- correctly refuses; this family's own
                                        remaining gap is NOT the two this cycle fixed)
```

The 15/53, 4/12, 5/72 baselines were re-measured fresh this cycle against the SAME probe before
either fix landed and reproduced cycle 7's own figures exactly, confirming the baseline is real and
the gain is attributable to this cycle's two fixes, not measurement noise.

No new chassis call site: both fixes live entirely inside `resolve_pool_member_sole_magnitude`,
reached through the SAME Sorcerer/Bloodrager/Cleric `push_generic_pool_group_selection_magnitude`
call sites cycle 5 already wired into the real chassis (`compute_apg_class_chassis` /
`ground_or_block_*` paths) — purely additive reachability through already-wired plumbing, exactly
the `§16` distinction cycles 4–7 already established.

## 3. `§16`: static catalog re-derivation — unchanged, and correctly so

```bash
python3 scripts/census_class_feature_pool_population.py
```
```
RESIDUAL numeric-magnitude needing compute       6018
```
Unchanged from the pre-cycle baseline. This cycle's closures are RUNTIME reachability for real
recorded selections through the already-existing generic resolver, not a change to which records
the static catalog counts as numeric-magnitude-bearing — the same distinction cycles 4–7 drew for
every prior generic-resolver widening.

## 4. Tests, RED→GREEN, both altitudes (`§1a`)

3 new tests in `generic_pool_group_selection_wiring_tests`:

- `sorcerer_generic_bloodline_pass_grounds_abyssal_via_the_cross_book_tracker_merge` (§1a/§1b
  together — Abyssal's own chain needs BOTH the cross-book header merge for `"Abyssal Bloodline"`
  itself to be findable at all, AND the tracker merge for `BloodlineLVL`).
- `bloodrager_generic_bloodline_pass_grounds_verdant_via_the_tracker_merge` (§1b, Bloodrager's own
  `"<class> ~ Bloodline Tracker"` shape).
- `cleric_generic_domain_pass_grounds_animal_via_the_class_record_merge` (§1c, the new
  `class_record_bonus_vars` table).

**Mutation altitude 1 (library logic):**

- `class_record_bonus_vars` merge wrapped in `if false { ... }` → re-ran
  `generic_pool_group_selection_wiring_tests`: `cleric_generic_domain_pass_grounds_animal_via_the_
  class_record_merge` FAILED, 13 others passed. Reverted.
- `registered_name_for_tracker` neutralised (`.filter(|_| false)`) → re-ran: BOTH `sorcerer_
  generic_bloodline_pass_grounds_abyssal_via_the_cross_book_tracker_merge` and `bloodrager_generic_
  bloodline_pass_grounds_verdant_via_the_tracker_merge` FAILED (both depend on the tracker merge),
  12 others passed. Reverted.

**Mutation altitude 2 (chassis call site):** the Cleric Domain `push_generic_pool_group_selection_
magnitude(...)` call wrapped in `if false { ... }` → re-ran: `cleric_generic_domain_pass_grounds_a_
never_hand_modelled_domain` (cycle 5's own pre-existing test) AND this cycle's new `cleric_generic_
domain_pass_grounds_animal_via_the_class_record_merge` BOTH failed, 12 others passed — confirms the
real chassis call site, not just the library function, is what carries this cycle's closure to a
player-facing path. Reverted; call site restored verbatim.

**Regression check**, scoped:

```bash
cargo test --locked --lib -- rules_core::pilot_compute::generic_pool_group_selection_wiring_tests
```
```
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 2758 filtered out
```
```bash
cargo test --locked --lib -- rules_core::pilot_compute::
```
```
test result: ok. 929 passed; 0 failed; 0 ignored; 0 measured; 1843 filtered out
```
```bash
cargo test --locked --lib -- rules_core::derived_evaluator_fixture_check::
```
```
test result: ok. 121 passed; 0 failed; 0 ignored; 0 measured; 2651 filtered out
```
```bash
cargo test --locked --lib -- oracle_dispatch_widening_safety_tests
```
```
test result: ok. 48 passed; 0 failed; 0 ignored; 0 measured; 2724 filtered out
```
```bash
cargo test --locked --lib -- cavalier
```
```
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 2756 filtered out
```

929/929 (up from cycle 7's 926/926: +3 net new tests, 0 broken). Oracle Mystery's own safety test
(`a_mystery_pick_alone_grounds_no_tier_one_revelation`) untouched and still passes — Oracle Mystery
stays withdrawn, not attempted, per its own standing ruling.

## 5. Scope discipline — did not attempt, real scoped follow-on, named rather than silently deferred

- **Hunter Animal Focus** (21 real records, exact match) — activation-gated, needs careful
  activation-state integration; cycles 3–7 all flagged this and it remains untouched. This cycle's
  time went to the `§17a` re-derivation (which surfaced two real, cross-cutting fixes worth more
  closed groups than a first activation-gated pool would have) rather than Hunter.
- **Shaman Spirit's own remaining unmodelled groups** — measured 8/14 resolvable via the SAME
  cross-book header merge fix (§1a), but no new test added and not separately verified by mutation
  this cycle; time-boxed. A future cycle can add the equivalent test cheaply (the mechanism is
  already proven generically by this cycle's three tests).
- **Warpriest Blessing's own remaining gap** — re-measured, still 0/37 (unchanged from cycle 7).
  This family's blocker is neither of the two this cycle fixed; not investigated further this
  cycle.
- **`class_feature_record_tokens_pre_gate_safe` (the MEMBER-record table)** — a direct corpus
  survey found 155 real duplicated member keys sharing the SAME cross-book-collision shape this
  cycle fixed for HEADER records (e.g. `"Core Domain ~ Void Domain"` in 4 books). Deliberately left
  unmerged: narrower, safer scope (this table's `class` field is load-bearing for real ownership
  decisions elsewhere in this file, e.g. `real_pool_group_for_selection_slug`'s own majority tally
  and Cavalier's ownership-proof fallback, so a cross-book class-value disagreement here carries
  more risk than the header table's own merge did), and not needed for either of this cycle's two
  closures. Named for a future cycle, not silently dropped.
- **Oracle Mystery** — untouched, stays withdrawn. `§1a`'s safety test (`oracle_dispatch_widening_
  safety_tests::a_mystery_pick_alone_grounds_no_tier_one_revelation`) not touched; the budgeted-
  revelation modelling gap cycle 5 found is not closed by this cycle's work.
- Rows 11/15 (left `in-progress`, untouched); `apps/desktop`'s row 19/20 lanes not touched (no
  changes outside `pilot_compute/mod.rs`, `pilot_compute/class_feature_grant_consumer.rs`, and this
  row's own kanban cell). `data/corpus/**` untouched throughout.

`df -h /`: reported in the dispatch's final report.
