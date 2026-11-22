# Cycle t12-class-feature-pool-population, cycle 10 — Gate 3 (closure invariant) / row 18 (`epic-8-pool-shaped-class-features`)

- **Card ID:** row 18 (`epic-8-pool-shaped-class-features`), governed by `decisions.md §27b`
  (*"EVERYTHING"*). Dispatched scope: cycle 9's own named-not-attempted list, largest first —
  the member-table 155-key cross-book merge (cycle 9's own "likely the largest single win
  available"), Cavalier's remaining 8 unresolved Order groups, and (as time allowed) the
  `classlevel("X","APPLIEDAS=NONEPIC")` 2-argument form and Hunter Animal Focus.
- **Base:** `git merge-base --is-ancestor "$PIN" HEAD` returned FALSE at session start (worktree
  `HEAD` was `1bb523773d`, PR #374's merge, far behind `$PIN`). Fixed: `git reset --hard "$PIN"`
  then `git rebase origin/tranche/12` — fast-forward, no-op (`origin/tranche/12` HEAD == `$PIN` ==
  row20 cycle5's own follow-up commit `9f2fa984da`). `BASE_OK` re-verified after. `git log
  origin/tranche/12` checked at session start for recent activity in
  `class_feature_grant_consumer.rs`, `formula_interpreter.rs`, `mod.rs` — last touch to
  `class_feature_grant_consumer.rs` is cycle 8's own commit (`ef4a6ffca2`); no sibling-lane
  collision.
- **Oracle:** fresh worktree, empty git-ignored slot as expected. Bootstrapped via
  `scripts/fetch-pcgen-oracle.sh --dest <repo-local artifacts/corpus/operator-supplied/pcgen>`;
  pin confirmed `7f818006e371`.
- **Files touched:**
  - `src/rules_core/pilot_compute/class_feature_grant_consumer.rs` — (1)
    `class_feature_record_tokens_pre_gate_safe`'s cross-book collision policy changed from
    whole-record `or_insert_with` (first book wins ENTIRELY) to a per-target `BONUS:VAR` merge,
    mirroring cycle 8's own header-side fix exactly. (2) Factored the shared per-target merge
    loop both this table and `class_feature_bonus_vars_any_record` now use into one new private
    helper, `merge_bonus_var_target_map_never_overwriting`, so the two cross-book merges cannot
    independently drift into two different collision policies again. (3) 1 new unit test on the
    helper, mutation-proved. Module docs updated on both tables recording the `§17a`
    re-derivation result (see §1 below).
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 18 only (cycle field
    8 → 10, Notes appended). Verified: 21 distinct `^| N |` rows, 0 duplicates, row 18 parses to 7
    cells before and after.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — scoped to this cycle's own diff (`git diff --
  src/rules_core/pilot_compute/class_feature_grant_consumer.rs`, per §6 step 2's "scope to your
  own diff" note — the full `BASE_BRANCH...HEAD` form on this file pulls in ~500 pre-existing
  lines from cycles 1–9), `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'` → 0 hits.
- **Wired-integration audit result:** `OK_NO_TOKENS` — same scope, `grep -inE
  "STUB|MOCK|placeholder|not.?yet.?implemented|todo|fixme|hack"` → 0 hits. (The full
  `BASE_BRANCH...HEAD` form on this file DOES contain 2 pre-existing hits, both `todo/defects.md`
  path references from cycles 1–7, unrelated to this cycle's own diff — confirmed pre-existing,
  not introduced here.)
- **PI audit:** `pi_scrub.normalized_term_hits(...)` (imported, not copied) against this cycle's
  own diff text → `[]` (0 hits). `data/corpus/**` untouched throughout (`git status --porcelain
  -- data/corpus` — 0 changes).
- **Status:** in-progress — genuine, multi-cycle epic, unchanged from cycles 1–9's own framing.
  **This cycle banks 0 new group closures** — see §4, "Measurement waves are legitimate
  deliverables" (`workflow-instruction.md §9` lesson 6): both dispatched items were real,
  verified investigations that eliminate false hope and correctly narrow the next cycle's scope,
  not a wasted cycle.

---

## 1. Job 1 — member-table cross-book merge (`§17a`: predicted change re-derived, does not reproduce)

Cycle 9 named `class_feature_record_tokens_pre_gate_safe`'s own 155-key member-table cross-book
merge as "the member-side twin of the header-side merge cycle 8 landed for +25 groups. Same
shape, same fix, likely the largest single win available." Built exactly that fix — the table's
whole-record `or_insert_with` (first book wins ENTIRELY, discarding every `bonus_vars` target a
later book's own copy carries that the first book's copy lacks) is now a per-target merge,
identical policy to the header-side table cycle 8 already fixed, factored into one shared helper
(`merge_bonus_var_target_map_never_overwriting`) both tables now call so the two collision
policies cannot independently drift again.

**Command** (re-derivation, independent of the Rust change, to check the fix's real effect before
trusting a predicted number): a standalone script walked `data/corpus/*/class_feature/**/*.json`,
grouped by `KEY:` restricted to records carrying a real (non-null) `description` — this table's
own admission gate — and found every real duplicate-across-book key, then compared each key's
per-book `BONUS:VAR` target-name sets against the union across all its books.

```
total keys with a real description field present: 12211
keys appearing in >1 book (candidates for this merge): 81
keys whose cross-book target-name union differs from the first book alone: 0
```

**Every one of the 81 candidate keys already has an identical target set across all its books.**
The defect is real — cycle 8's own precedent (`core_rulebook`'s complete 308-token "Bloodline
Tracker" copy silently discarded in favour of `advanced_class_guide`'s single leftover row) proves
this EXACT collision shape has manifested before, on the header-side table, for the SAME bare-key
family — but it has not yet manifested on the member-side table for any key currently in the
corpus. A future corpus update that reprints a member record with genuinely different `.MOD`-row
coverage across books would previously have lost data silently under the old policy; it no longer
can.

**Re-ran `pool_group_closure_census_across_all_six_pools`** (cycle 9's own locked test) after
landing the fix:

```
Sorcerer Bloodline: 18/53 groups carry a resolvable member
Bloodrager Bloodline: 5/12 groups carry a resolvable member
Cleric Domain: 26/72 groups carry a resolvable member
Shaman Spirit: 8/14 groups carry a resolvable member
Warpriest Blessing: 0/37 groups carry a resolvable member
Cavalier Order: 1/9 groups carry a resolvable member
```

**Identical to cycle 9's own baseline. Zero movement.** `decisions.md §17a`: "a predicted change
that does not reproduce is itself a finding" — recorded here and in both tables' own module docs,
not silently discovered and dropped.

## 2. Job 2 — Cavalier's remaining 8 unresolved Order groups (`§12c`: population + command named)

Investigated rather than guessed at. Every real Cavalier Order `class_feature` record found across
the corpus (`Cavalier Order`, `Order Of The Eastern Star`, `Order Of The Shroud`, `Order of the
Beast`, `Order of the Guard` — 5 of the census's 9 real groups directly enumerable this way; the
naming-shape filter in `real_groups_owned_by` finds 9 total) was checked for `BONUS:` tokens of
ANY kind, not only the `VAR|`-prefixed shape this resolver reads:

```
Cavalier Order: 7 members, 0 with BONUS:VAR
Order Of The Eastern Star: 8 members, 0 with BONUS:VAR
Order Of The Shroud: 7 members, 0 with BONUS:VAR
Order of the Beast: 10 members, 0 with BONUS:VAR
Order of the Guard: 5 members, 0 with BONUS:VAR (1 member carries a single BONUS:SKILL —
  a different bonus type this table's own VAR-only parser correctly does not surface)
```

**Zero `BONUS:VAR` tokens exist anywhere in these groups' real corpus content.** Their crunch is
delivered through `ABILITY:` (grants an ability-pool slot), `CSKILL:` (class-skill list, no
magnitude), and plain `DESC:` text — a genuinely different, non-magnitude compute shape than the
four "closed magnitude" shapes this row's resolver targets, not a resolver bug or an unexplored
corner. This is the same shape the `wiring_class_signals: ["display:no_magnitude_token"]` field
already carried on several of these records at ingest time. **Confirmed, not a resolver defect —
these groups need a different compute shape (or none at all, per the `v0.6` text-only-features
ruling) to close, not a fix to this resolver.** Named for the next cycle rather than left
unexplained.

## 3. Scoped investigation, not attempted (named, per `§17`: generic passes, no new resolver
built without verification)

The dispatched-priority items above consumed this cycle's time before either the
`classlevel("X","APPLIEDAS=NONEPIC")` widening or Hunter Animal Focus could be attempted safely.
One further finding, sized but explicitly NOT built this cycle:

**Root cause identified for the bulk of Sorcerer Bloodline's remaining 35 unresolved groups (and
almost certainly the same shape for Cleric/Bloodrager/Shaman's own majorities).** Of Sorcerer's 53
real groups, 49 carry SOME `BONUS:VAR` content (re-derived: `Aberrant Bloodline` 6/10 members,
`Draconic Bloodline` 10/13, ... `Sorcerer Bloodline` header itself 32/32) — yet only 18 resolve.
Traced one concrete case (`Aberrant Bloodline ~ Acidic Ray`, formula
`Sorcerer_Aberrant_BloodlinePower1LVL/2`): the terminal variable it needs
(`Sorcerer_Aberrant_BloodlinePower1LVL`) is defined on the `Aberrant Bloodline` header record
itself, but ONLY inside rows tagged `PREABILITY:1,CATEGORY=Sorcerer Bloodline,Aberrant Bloodline`
(and a `!PREABILITY:...` negated sibling) — a PRE-tag kind `parse_bonus_var_tokens_pre_gate_safe`
(via `bonus_stack_reader::extract_addends`) does not recognise (today: `PREVARGTEQ` and `TYPE=`
only). No ungated fallback row exists for this target, so it is correctly refused, not guessed.

**Why this is not fixed here rather than attempted uninvestigated:** a `PREABILITY:1,CATEGORY=<X
Bloodline>,<the exact group being resolved>` gate IS structurally always-true at every real call
site of `resolve_pool_member_sole_magnitude` — it only ever runs for the specific group a
character has already selected, so "does this character have THIS ability" is trivially yes by
construction. That reasoning is sound, but implementing it correctly needs (a) confirming the real
PCGen `PreAbilityTester`/`CATEGORY` semantics — the pinned oracle mirror ships `data/`/`system/`
only, no `.java` sources, so this cannot be independently re-verified from this mirror this cycle
— and (b) threading per-call group/category context into functions that today build their tables
once, corpus-wide, with no per-call context at all (`class_feature_bonus_vars_any_record`'s own
`bonus_vars` map is a `OnceLock`, shared across every call). Guessing the semantics to unblock ~30
groups quickly would risk exactly the "ships a genuinely wrong number" failure `decisions.md §1a`
exists to prevent. Sized and named for a focused future cycle, not attempted uninvestigated.

**Did not attempt** (named, not silently deferred): `classlevel("X","APPLIEDAS=NONEPIC")`'s real
2-argument form (unchanged from cycle 9's own sizing — a Monk unarmed-damage shape, unrelated to
this row's six pools); Hunter Animal Focus (still activation-gated, untouched since cycle 5);
Oracle Mystery (stays withdrawn, safety test untouched, per its own standing ruling). Rows 11/15
left `in-progress`, untouched; `apps/desktop`'s row 19/20 lanes not touched. `data/corpus/**`
untouched throughout.

## 4. Tests, RED→GREEN (`§1a`)

**Job 1's shared merge helper**: `merge_bonus_var_target_map_never_overwriting` — mutation:
body replaced with a no-op (params underscored) → re-ran
`rules_core::pilot_compute::class_feature_grant_consumer::tests::merge_bonus_var_target_map_pulls_in_new_targets_but_never_overwrites_a_seen_one`:
FAILED for the intended reason (`left: None, right: Some("2")` — the "new target from a later
book" assertion, not the "never overwrite" one, catching exactly the regression this fix
prevents). Reverted; re-ran, green.

**Regression, scoped**:

```bash
cargo test --locked --lib -- rules_core::pilot_compute::
```
```
test result: ok. 944 passed; 0 failed; 0 ignored; 0 measured; 1843 filtered out; finished in 19.35s
```
944/944 (up from cycle 9's 932/932 baseline — the gap is row20 cycle5's own commits landing
between cycle 9 and this cycle's pin, +1 net new test this cycle: the merge-helper unit test).

```bash
cargo test --locked --lib -- rules_core::pilot_compute::class_feature_grant_consumer
```
```
test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 2758 filtered out; finished in 3.97s
```

```bash
cargo test --locked --lib -- rules_core::derived_evaluator_fixture_check::
```
```
test result: ok. 121 passed; 0 failed; 0 ignored; 0 measured; 2666 filtered out; finished in 3.25s
```

```bash
cargo test --locked --lib -- oracle_dispatch_widening_safety_tests
```
```
test result: ok. 48 passed; 0 failed; 0 ignored; 0 measured; 2739 filtered out; finished in 4.38s
```
Oracle Mystery's own safety test (`a_mystery_pick_alone_grounds_no_tier_one_revelation`) not
touched this cycle; stays withdrawn, untouched.

```bash
cargo test --locked --lib -- cavalier
```
```
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 2771 filtered out; finished in 6.05s
```

## 5. `df -h /`

```
Filesystem      Size  Used Avail Use% Mounted on
/dev/sda1       968G  500G  469G  52% /
```
