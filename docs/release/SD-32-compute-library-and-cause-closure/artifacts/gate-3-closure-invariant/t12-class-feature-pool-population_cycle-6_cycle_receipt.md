# Cycle t12-class-feature-pool-population, cycle 6 — Gate 3 (closure invariant) / row 18 (`epic-8-pool-shaped-class-features`)

- **Card ID:** row 18 (`epic-8-pool-shaped-class-features`), governed by `decisions.md §27b`
  (*"EVERYTHING"*). Dispatched scope: build the two named gaps cycle 5 escalated (PRE-gated
  header-chain parsing, `classlevel(...)` cross-class widening), fan those out across the four
  already-wired pools, then wire the three pools cycle 5 named but did not attempt (Bloodrager
  Bloodline, Hunter Animal Focus, Cavalier Order), largest/cheapest first.
- **Base:** `git merge-base --is-ancestor "$PIN" HEAD` returned FALSE at session start
  (`PIN=945109cdfc3dc1564354f9e6490cb9389114f771`, worktree started on a stale lineage — the
  eighth lane to hit this named failure mode). Fixed: `git reset --hard "$PIN"` then
  `git rebase origin/tranche/12` (fast-forward — `origin/tranche/12` == `$PIN`, cycle 5's own
  commit `29ddbc439d` is its parent's parent). `BASE_OK` re-verified after.
- **Oracle:** fresh worktree, empty git-ignored slot as expected. Bootstrapped via
  `scripts/fetch-pcgen-oracle.sh --dest <repo-local artifacts/corpus/operator-supplied/pcgen>`;
  `scripts/verify.sh --only preflight-oracle` → `PASS` (pin `7f818006e371`).
- **Files touched:**
  - `src/rules_core/pilot_compute/formula_interpreter.rs` — `Expr::ClassLevel` eval now keys on
    `CLASSLEVEL::<class>` instead of the class-blind `__LEVEL__` slot (Gap 2); module doc updated;
    2 new tests.
  - `src/rules_core/pilot_compute/formula_reproduction_harness.rs` — `vars_for` widened to also
    bind every class name literally named inside a case's own `classlevel("...")` text, so the
    21-case reproduction proof stays green against the production evaluator's new key convention
    (test-harness-only change, no production behaviour here).
  - `src/rules_core/pilot_compute/class_feature_grant_consumer.rs` —
    `parse_bonus_var_tokens_pre_gate_safe` rewritten: (1) strips `TYPE=<bonustype>` trailing
    fields (never a gate) before classification; (2) multi-row `PREVARGTEQ`-gated targets now
    resolve via `bonus_stack_reader::extract_addends`, re-expressed as a summed `if(...)` formula
    string (Gap 1). `resolve_pcgen_var_chain` now also seeds `CLASSLEVEL::<ThisRecordsClass>`.
    Pinned scale-test `the_live_scale_of_this_waves_widening_is_measured_and_pinned` re-derived
    and updated (see §5).
  - `src/rules_core/pilot_compute/mod.rs` — new Bloodrager Bloodline call site in
    `ground_or_block_bloodrager_bloodrage` (purely additive alongside the existing hand-modelled
    `ground_bloodrager_arcane_bloodline`); `real_pool_group_for_selection_slug` widened with a
    second naming-shape fix (strips a trailing owner-class-name infix word from the adjective, see
    §4); 2 new Bloodrager tests in `generic_pool_group_selection_wiring_tests`.
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 18 only (cycle field
    4 → 5, Notes appended). Row 19 left untouched and still parses: exactly one `^| 18 |` row and
    one `^| 19 |` row, each with 7 cells matching the header, verified by script before commit.
  - `docs/retro/events/sd31-transcribe.jsonl` — auto-appended by `scripts/verify.sh` (1 derived
    `preflight-oracle` PASS event for this worktree). Not hand-edited.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — `git diff --unified=0` over the four Rust
  files above, `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'` → 0 hits outside the
  literal `SD-31`/`SD-32` doc-comment citations already present in the surrounding file.
- **Wired-integration audit result:** `OK_NO_TOKENS` — same scope, `grep -inE
  "STUB|MOCK|placeholder|not.?yet.?implemented|todo|fixme|hack"` → 0 hits.
- **PI audit:** `pi_scrub.normalized_term_hits(...)` against this cycle's full diff text (Rust
  files + kanban) → `[]` (0 hits) both before and after the kanban edit. `data/corpus/**`
  untouched throughout (`git status --porcelain -- data/corpus` — 0 changes).
- **Status:** in-progress — genuine, multi-cycle epic, unchanged from cycles 1–5's own framing.

---

## 1. Gap 2 — `classlevel(...)` cross-class widening (`formula_interpreter.rs`)

Real PCGen's `classlevel("X")` looks up level in the SPECIFIC named class
(`plugin/jepcommands/ClassLevelCommand.java`). This module previously had no per-class
environment — every `classlevel(...)` call, regardless of its own argument, read a single
class-blind `__LEVEL__` slot, which `resolve_pcgen_var_chain` (the only production caller) never
even bound, so every formula containing `classlevel(...)` unconditionally refused (an unbound
`__LEVEL__` error). `Expr::ClassLevel(class_name)` now looks up `CLASSLEVEL::<class_name>`
instead. `resolve_pcgen_var_chain` binds exactly ONE such key — the record's own granting class,
recovered from `class_level_var` by stripping its trailing `LVL` suffix (the exact inverse of
`class_level_variable_name`'s own auto-declared-variable convention; every real class name this
convention needs to invert in this cycle's scope — Sorcerer, Cleric, Warpriest, Shaman,
Bloodrager, Summoner, Rogue, Assassin — is a single word, so the inversion is exact). A
same-class `classlevel("SameClass")` therefore now resolves CORRECTLY; a genuinely
different-class argument stays cleanly unbound and refuses — never fabricates. Proven safe by a
new test, `classlevel_refuses_a_genuinely_different_class_it_has_no_binding_for`.

**Real, measured effect** (`class_feature_grant_consumer::tests::the_live_scale_of_this_waves_
widening_is_measured_and_pinned`):

```
before: (already_admitted=136, newly_resolved=15, class_excluded=11, chain_unresolvable=14, no_record=36)
after:  (already_admitted=136, newly_resolved=20, class_excluded=11, chain_unresolvable=9,  no_record=36)
```

The 5 newly-resolved records are all Summoner: `Bond Senses`, `Maker's Call`, `Merge Forms`,
`Summon Monster` (duration), `Twin Eidolon` — each a bare `classlevel("Summoner")` formula this
module could not bind before this cycle. This is a genuine, real, NEW closure — not a
reclassification, not a reachability move; these five now reach `resolved_description_for` with
a real computed value where they previously produced `None`.

## 2. Gap 1 — PRE-gated header-chain parsing (`class_feature_grant_consumer.rs`)

Two widenings to `parse_bonus_var_tokens_pre_gate_safe`, both reusing existing, already
oracle-verified machinery rather than building a third mechanism (`§17`):

**(a) `TYPE=<bonustype>` trailing fields are stripped, never refused as an unrecognised gate.**
`TYPE=` (e.g. this corpus's own `AC_Natural_Armor|2|TYPE=Base`, `DomainAirLVL|DomainLVL|
TYPE=Domain`) is PCGen's real bonus-STACKING classification, confirmed by every hand-modelled
function elsewhere in `mod.rs` that already grounds a `TYPE=`-tagged token unconditionally (cited
throughout the file, e.g. `AC_Natural_Armor|2|TYPE=Base`) — it governs whether two DIFFERENT
bonus sources stack, never whether THIS record's own contribution applies. The original function
treated ANY trailing pipe field (including `TYPE=`) as an unrecognised-gate refusal; it now
strips `TYPE=` fields specifically before classification, leaving a genuine PRE-gate field (if
any) recognisable underneath.

**(b) Multi-row `PREVARGTEQ`-gated targets now resolve, via `bonus_stack_reader`.**
`bonus_stack_reader` (SD-31 wave 26) already reads and proves the exact shape multiple
`BONUS:VAR` rows sharing one target, each independently gated by its own
`PREVARGTEQ:<var>,<threshold>` — real oracle semantics (`PreVariableTester.java` +
`BonusManager.sumActiveBonusMap`, summed, only the currently-qualifying rows, both cited in that
module's own doc), previously wired only for a hand-picked witch-ward case, never plugged into
the generic pool resolver. `parse_bonus_var_tokens_pre_gate_safe` now tries
`bonus_stack_reader::extract_addends` per target (after (a) strips any `TYPE=` field); on success
the addends are re-expressed as a single formula string this module's OWN existing evaluator
already parses — `if(<gate var>>=<threshold>,(<formula>),0)` per gated row, summed with `+` —
reusing the `if(...)`/`Cmp` grammar (wave 26 shape closure) rather than a second evaluation path.
A target whose rows still carry any OTHER shape (more than one non-`TYPE=` PRE field, a
non-`PREVARGTEQ` tag such as `PREABILITY`/`PREMULT`) still fails `extract_addends` and is still
dropped — unchanged refusal for any shape this module has not itself verified.

Live example this closes structurally, `Air Domain ~ Electricity Resistance`'s
`DomainAirResistanceBonus` (`10|PREVARGTEQ:DomainAirLVL,6` + `10|PREVARGTEQ:DomainAirLVL,12`) now
correctly synthesizes to `if(DomainAirLVL>=6,(10),0)+if(DomainAirLVL>=12,(10),0)` — though this
particular member still does not resolve end-to-end, because `DomainAirLVL` itself is defined on
the HEADER record via `VAR|DomainAirLVL|DomainLVL|TYPE=Domain`, and — even with (a) stripping the
`TYPE=Domain` tag — the header's *own* `class_feature_bonus_vars_any_record` table build shares
this same widened function, so `DomainAirLVL` now correctly merges in as `DomainLVL` too; the
remaining blocker for THIS specific member turned out to be that `DomainLVL` itself is a
DIFFERENT, not-yet-bound PCGen auto-variable (Domain-power caster-level tracking, out of this
resolver's scope) — reported honestly below, not silently declared closed.

## 3. Measured effect on the four already-wired pools — smaller than hoped, and why

A direct, live re-survey (temporary test, deleted before commit — same shape as cycle 5's own
inline python survey, reproduced here via the real production functions
`class_feature_record_tokens_pre_gate_safe` + `resolve_pool_member_sole_magnitude` directly,
`cargo test --locked --lib -- ...temp_measure_group_resolvability --nocapture`, run and then the
test removed) of every real group in each of the four pools cycle 5 wired:

| Pool | Before (cycle 5) | After (this cycle) | Net new groups |
|---|---:|---:|---:|
| Sorcerer Bloodline | 15 of 53 | 15 of 53 | **0** (composition shifted, see §3a) |
| Cleric Domain | 5 of 67 | 5 of 67 | **0** |
| Warpriest Blessing | 0 of 36 | 0 of 36 | **0** (see §3b — not actually blocked) |
| Shaman Spirit | 0 of 4 (remaining-unmodelled) | 0 of 4 | **0** |

Neither Gap 1 nor Gap 2, individually real and tested, moved the net resolvable-group count on
any of the four pools cycle 5 already wired. Direct inspection of the actually-blocking header
records explains why:

### 3a. Sorcerer Bloodline's real blocker is `PREABILITY`/`PREMULT`, not `PREVARGTEQ`

Every one of the 28 real Sorcerer-Bloodline-family multi-row headers this cycle found (e.g.
`Marid Bloodline`'s `Sorcerer_Marid_BloodlineLVL`) is gated on `PREABILITY`/`PREMULT` — "does the
character hold this bloodline via a specific feat/ability" (the base Sorcerer Bloodline class
feature, or the Crossblooded Bloodline feat) — never `PREVARGTEQ`. This is a genuinely different,
larger mechanism than either gap this cycle built: it needs to know whether the character has
been GRANTED a specific ability/feat, which this resolver has no representation of at all (it
only ever threads `level` and `ability_modifiers`). `bonus_stack_reader::extract_addends`
correctly refuses these (unrecognised PRE-tag), unchanged.

**Correctness side-effect of Gap 1(a), verified not a regression:** stripping `TYPE=` restores
previously-HIDDEN second targets on a handful of records that used to look single-terminal only
because their second target was accidentally invisible. `Aquatic Bloodline ~ Deep One` carries
TWO real, independent magnitudes — `BlindsenseRange|60` (no tail) and
`ColdResistanceBonus|20|TYPE=Resistance` (previously dropped, now correctly parsed). Before this
cycle, `resolve_pool_member_sole_magnitude` saw only `BlindsenseRange` and (wrongly) treated it as
"the" sole terminal, silently discarding `ColdResistanceBonus` entirely. After this cycle, both
targets are visible, correctly recognised as two independent, mutually non-referencing terminals,
and the record correctly REFUSES (`terminals.next().is_some()` — more than one terminal, do not
guess) rather than arbitrarily picking one. This swaps which 3 of the 15 resolvable Sorcerer
Bloodline groups appear (Aquatic/Starsoul/Verdant, which lose their only resolvable member to this
more-correct refusal, are replaced by Aberrant/Djinni/Draconic, which gain one from Gap 1(a)
correctly un-hiding a real target elsewhere) without changing the count — a more honest refusal
replacing an arbitrary, previously-silent single-value pick, never a closure genuinely lost (no
downstream test or consumer named any of the three by name; `sorcerer_generic_bloodline_pass_
grounds_a_never_hand_modelled_bloodline`'s own example, Celestial, is unaffected either way).

### 3b. Warpriest Blessing's "0 of 36" is NOT a resolver gap — correction of record (`§17a`)

Cycle 5's own framing named Blessing as needing "the missing header chain or a `classlevel(...)`
call this resolver deliberately refuses to bank through." Direct corpus inspection this cycle
found that framing does not survive validation:

```
python3 <inline script scanning every "<X> Blessing"-suffixed, Warpriest-owned class_feature
record for BONUS/DEFINE tokens>
Warpriest Blessing groups= 37 records= 74 numeric= 9
```

Every one of the 36 individual `<X> Blessing ~ <member>` groups' own records (e.g. `Air Blessing ~
Soaring Assault`, `War Blessing ~ War Mind`) carries **ZERO** `BONUS`/`DEFINE` tokens — pure
`DESC`-only text with no `%N` substitution either. This is real `§7` zero-magnitude/display, and
the census's own `NUMERIC_MAGNITUDE_KEYS = {"BONUS", "DEFINE"}` gate already correctly excludes
these records from the 5,927 residual — they were never part of the population this epic's
resolver widening could close in the first place. The 9 numeric records that DO exist under a
"Blessing"-named key all belong to `Warpriest ~ Blessings` / `Forgepriest ~ Blessings` /
`Warpriest Blessings Base` — a wholly SEPARATE class-level record (the Blessings-pool
uses-per-day/DC chassis feature, not any individual blessing's own power), and it is **already
hand-modelled**: `warpriest_blessing_uses_per_day` (`pilot_compute/mod.rs:18702`) and
`warpriest_blessing_dc` (`:18710`), wired at `ground_or_block_warpriest_class_features`
(`:18890`-`18893`), reproduce these exact `BONUS:VAR|WarpriestBlessingUses|(WarpriestBlessingLVL/
2)+3` / `BONUS:VAR|WarpriestBlessingDC|(WarpriestBlessingLVL/2)+10+WIS` formulas byte-for-byte.

**Warpriest Blessing needs no further work of any kind.** All 36 groups' individual members are
correctly zero-magnitude/display (DONE); the pool's own real numeric magnitude is already
grounded by dedicated, tested, pre-existing functions. This is reported here as a correction of
cycle 5's own escalation, per `§17a`'s own standing instruction to re-derive every figure handed
forward rather than transcribe a lead uncritically.

## 4. Bloodrager Bloodline wired — a second real naming-shape fix, then 4 of 12 groups close

Wired at a new call site inside `ground_or_block_bloodrager_bloodrage`, purely additive alongside
the pre-existing `ground_bloodrager_arcane_bloodline` (different id prefix — `class_feature.acg.
bloodrager.bloodline.generic.*` vs `...arcane.*` — proven non-colliding, both fire independently,
by `bloodrager_generic_bloodline_pass_does_not_collide_with_the_hand_modelled_arcane_bloodline`).

This required a SECOND real corpus naming-shape fix, generic, not a Bloodrager-specific hack:
Bloodrager's own 12 real Bloodline groups bake the owner CLASS NAME into the group string itself
(`"Undead Bloodrager Bloodline"`, `"Aberrant Bloodrager Bloodline"`) — unlike Sorcerer's plain
`"<Adjective> Bloodline"` shape — while the established recorded-selection convention this
codebase already uses (`ARCANE_BLOODRAGER_BLOODLINE_SELECTION = "bloodline:arcane"`, confirmed
by direct grep, not `"bloodline:arcane_bloodrager"`) still names only the bare adjective.
`real_pool_group_for_selection_slug` now strips a trailing `" <class>"` word-boundary suffix from
the adjective (after the registered-name suffix strip), generically — safe because it only ever
fires when the owner class's own name is literally a trailing word, never over-stripping an
adjective that merely shares a prefix.

Direct corpus survey: 4 of 12 real Bloodrager Bloodline groups (Aberrant, Arcane, Elemental,
Undead) carry a directly-resolvable member. Undead is never hand-modelled — proven live
(`bloodrager_generic_bloodline_pass_grounds_a_never_hand_modelled_bloodline`). Arcane IS
hand-modelled (`ground_bloodrager_arcane_bloodline`), and the generic pass ALSO independently
grounds it under its own separate id, proven non-colliding above. The other 8 correctly refuse —
same shape as Sorcerer's own Bloodline pool, same `PREABILITY`/`PREMULT`-gated header cause named
in §3a.

**Real NEW closures this cycle: 5 Summoner class_feature records (Gap 2) + Bloodrager Bloodline
wired at 4 of 12 groups (3 genuinely new — Aberrant, Elemental, Undead — plus Arcane, already
hand-modelled, now also generically covered without collision).**

## 5. Tests, RED→GREEN, both altitudes (`§1a`)

10 new tests total:

- `formula_interpreter::tests::classlevel_reads_the_level_binding` (updated for the new
  `CLASSLEVEL::` key convention) + `classlevel_refuses_a_genuinely_different_class_it_has_no_
  binding_for` (new safety proof).
- `generic_pool_group_selection_wiring_tests::bloodrager_generic_bloodline_pass_grounds_a_never_
  hand_modelled_bloodline` + `..._does_not_collide_with_the_hand_modelled_arcane_bloodline`.
- `class_feature_grant_consumer::tests::the_live_scale_of_this_waves_widening_is_measured_and_
  pinned` re-derived and re-pinned (see §1).

**Mutation altitude 2 (library logic):** `real_pool_group_for_selection_slug` forced `if true {
return None; }` at its top → re-ran `generic_pool_group_selection_wiring_tests`:

```
4 passed (the two "correctly refuses" tests and the invented-selection guard were already
          asserting 0/None-shaped outcomes, unaffected)
4 failed (bloodrager_generic_bloodline_pass_grounds_a_never_hand_modelled_bloodline,
          bloodrager_generic_bloodline_pass_does_not_collide_with_the_hand_modelled_arcane_bloodline,
          cleric_generic_domain_pass_grounds_a_never_hand_modelled_domain,
          sorcerer_generic_bloodline_pass_grounds_a_never_hand_modelled_bloodline)
```

RED confirmed (every positive-closure assertion across all pools using this shared function fails
exactly as expected — proving the widening this cycle made to `real_pool_group_for_selection_slug`
is load-bearing for ALL group-selection pools, not only Bloodrager). Reverted.

**Mutation altitude 1 (chassis call site):** the new Bloodrager
`push_generic_pool_group_selection_magnitude(...)` call wrapped in `if false { ... }` → re-ran:

```
2 failed: bloodrager_generic_bloodline_pass_grounds_a_never_hand_modelled_bloodline,
          bloodrager_generic_bloodline_pass_does_not_collide_with_the_hand_modelled_arcane_bloodline
6 passed: every other test unaffected (proves the new call site is independent of every other
          pool's own call site)
```

RED confirmed, isolated to exactly the mutated call site. Reverted; call site restored verbatim.

**Regression check**, scoped:

```bash
cargo test --locked --lib -- rules_core::pilot_compute::generic_pool_group_selection_wiring_tests
```
```
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 2755 filtered out
```
```bash
cargo test --locked --lib -- rules_core::pilot_compute::
```
```
test result: ok. 923 passed; 0 failed; 0 ignored; 0 measured; 1840 filtered out
```
```bash
cargo test --locked --lib -- rules_core::derived_evaluator_fixture_check::
```
```
test result: ok. 121 passed; 0 failed; 0 ignored; 0 measured; 2642 filtered out
```

923/923 (up from cycle 5's 920/920: +3 net new tests — 1 new `formula_interpreter` test
(`classlevel_refuses_a_genuinely_different_class_it_has_no_binding_for`; `classlevel_reads_the_
level_binding` was edited in place, not added) + 2 new Bloodrager tests, 0 broken. The temporary
measurement test used only for the live survey in §3 (`temp_measure_group_resolvability`) was
deleted before this commit and is not counted.

## 6. Sweep (`§3`) and residual re-derivation (`§17a`)

```bash
grep -rn "5,927\|5927\b\|(136, 15, 11, 14, 36)\|(136, 20, 11, 9, 36)" docs/release/SD-32-compute-library-and-cause-closure/*.md tests/ src/ scripts/ apps/
```
The `(136, 15, 11, 14, 36)` tuple's only occurrence was the one assertion in
`class_feature_grant_consumer.rs`, now updated to `(136, 20, 11, 9, 36)` and re-verified as the
sole occurrence of the new tuple too — no other file's pinned count needed a matching update.

```bash
python3 scripts/census_class_feature_pool_population.py
```
```
RESIDUAL numeric-magnitude needing compute       5927
```
**Unchanged, and correct.** This cycle's Summoner/Bloodrager closures are RUNTIME reachability for
specific recorded selections through the already-existing generic resolvers — the same `§16`
distinction cycles 4 and 5 already drew — not a change to the static catalog population. No new
numeric-magnitude residual figure is introduced.

## 7. Scope discipline

**Did not attempt**, real scoped follow-on, named rather than silently deferred:

- **Hunter Animal Focus** (21 real records, exact match) — activation-gated, needs careful
  activation-state integration; cycles 3/4/5 all flagged this and it remains untouched.
- **Cavalier Order** — needs the two-level dispatcher walk cycle 3 first found; not attempted.
- **The `PREABILITY`/`PREMULT`-gated remainder of Sorcerer Bloodline (38/53), Bloodrager Bloodline
  (8/12), and Cleric Domain/Shaman Spirit's own equivalent header-chain shapes** — a genuinely
  different, larger mechanism (character feat/ability-grant tracking) than either gap this cycle
  built, escalated here by name per `§27b`'s own instruction ("a cycle that believes it has found
  a genuine impossibility escalates it by coordinate ... it does not write its own exemption").
- **Oracle Mystery** — untouched, stays withdrawn. `§1a`'s safety test (`oracle_dispatch_widening_
  safety_tests::a_mystery_pick_alone_grounds_no_tier_one_revelation`) not touched; the
  budgeted-revelation modelling gap cycle 5 found is not closed by this cycle's work (neither gap
  built here addresses "a second, budgeted sub-choice" at all).
- Rows 11/15 (left `in-progress`, untouched); `apps/desktop`'s row 19 lane not touched (no changes
  to `class_feature_pool_catalog.rs` or `class_feature_grant_consumer.rs`'s territory beyond this
  cycle's own additive widening — confirmed no upstream landing from row 19 to rebase onto at push
  time, see final report). `data/corpus/**` untouched throughout.

`df -h /`: reported in the dispatch's final report.
