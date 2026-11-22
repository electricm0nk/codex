# Cycle t12-class-feature-pool-population, cycle 15 — Gate 3 (closure invariant) / row 18 (`epic-8-pool-shaped-class-features`)

- **Card ID:** row 18 (`epic-8-pool-shaped-class-features`), governed by `decisions.md §27b`
  (*"EVERYTHING"*). Dispatched scope: cycle 14 established that of the 46 groups in the two
  "stuck" pools, 36 were never part of this row's numeric-magnitude population at all (`§16`), and
  the genuine remainder is a real `%N`-substituted-DESC-formula shape whose true home is
  `pcgen_desc.rs`/`description_completion.rs`, not the pool resolver. This cycle: extend that
  path, close the remainder there, re-run `pool_group_closure_census_across_all_six_pools`, and
  report honestly whether its denominator misrepresents the row's real remaining work.
- **Base:** `git merge-base --is-ancestor "$PIN" HEAD` returned FALSE at session start — the
  worktree started on a stale/unrelated lineage (`worktree-wf_5f7572fc-28e-1` / PR #374's merge
  commit, `1bb523773d`, footgun 3 named in the brief). Fixed: `git reset --hard "$PIN"`
  (`a901a70904`, cycle 14's own commit), `git rebase origin/tranche/12` — `origin/tranche/12`'s tip
  was still `a901a70904` at rebase time (`git fetch` confirmed, "Current branch ... up to date"), so
  no sibling lane had landed on row 18's files since cycle 14.
- **Oracle:** bootstrapped fresh (`scripts/fetch-pcgen-oracle.sh --dest
  docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen`)
  — `pcgen-oracle: OK 7f818006e371...` (pin confirmed).

## 1. Corpus re-derivation of the "genuine 9" (`§17a` — re-derive every figure)

The brief's own framing ("8 Warpriest + 1 Cavalier … 9 total … those 9 are now your target") is
imprecise about which of the 9 are ALREADY hand-modelled. Re-derived directly from the corpus
(scoped to real Warpriest Blessing and Cavalier Order groups only — filtered by `class` +
`" ~ "`-group name containing "Blessing"/"Order", not the whole class, which a first unscoped pass
over-counted at 52 hits): exactly 9 real `%N`-substitution-only groups exist (8 Warpriest, 1
Cavalier), matching cycle 14's own tally exactly. Of the 8 Warpriest groups, **two are already
hand-modelled** by dedicated, activation-gated functions this codebase already ships:
`Destruction Blessing ~ Destructive Attacks` (named by cycle 14) **and `Strength Blessing ~
Strength Surge`** (`ground_warpriest_strength_surge`, confirmed live at `mod.rs:19416` — cycle 14's
own receipt named only Destruction, an omission corrected here by direct code inspection, not
assumed). The genuine remainder needing a new generic resolver is therefore **7 groups**: 6
Warpriest (Earth, Trickery, Rune, Protection, Repose, Knowledge Blessing) + 1 Cavalier (Order of
the Beast) — not 8.

## 2. The generic `%N`-formula-DESC resolver, in its proper module (`§17`)

Built exactly where cycle 14 named it, never routed through the pool resolver:

- `pcgen_desc.rs`: added `desc_token_arguments(raw: &str) -> Vec<String>` — a thin, read-only
  exposure of the existing private `split_prose_and_args`'s own argument-tail parsing (no behaviour
  change to any existing function; every prior caller of `split_prose_and_args` is untouched).
- `class_feature_grant_consumer.rs`: added
  `resolved_description_for_formula_only_desc_argument(key, level, ability_modifiers) ->
  Option<(String, i64)>` — for a record whose `bonus_vars` is EMPTY (so
  `resolve_pool_member_sole_magnitude` correctly refuses it, unaffected), evaluates each `%N`
  argument DIRECTLY as a raw PCGen formula expression through the same `PcgenFormulaEvaluator`
  every other resolver in this module already uses, seeded with the same two facts
  (`class_level_var`, ability modifiers). Each resolved value is keyed under the exact argument
  TEXT in a `PcgenDisplayValues` table, so `resolve_desc_argument`'s own unmodified named-lookup
  shape (`values.get(arg)`) finds it — zero changes needed to `render_pcgen_desc_with_values` or
  `resolve_desc_argument` themselves. Returns `None` unless the WHOLE description renders clean (no
  dropped args, no leaked syntax) — the same "drop and report, never partially render" contract
  `resolved_description_for` already enforces, reused unchanged.
- `mod.rs`: added `push_generic_pool_group_selection_description_magnitude`, the sibling of
  `push_generic_pool_group_selection_magnitude` for this different shape — same selection-reading
  and `real_pool_group_for_selection_slug` resolution, but calls the new function above instead of
  `resolve_pool_member_sole_magnitude`, with an `already_hand_modelled_keys` exclusion list so it
  never double-emits for Destruction/Strength Blessing (both already grounded, with
  activation-state gating this generic pass has no way to reproduce). Wired at both call sites:
  Warpriest Blessing (`class_feature.acg.warpriest.blessing_description.generic`) and Cavalier
  Order (`class_feature.apg.cavalier.order_description.generic`).

**Real corpus outcome, verified per-member (not assumed to all resolve):**
- Earth Blessing ~ Armor of Earth: `if(WarpriestLVL<19,1+((WarpriestLVL/2)-5),5)` — resolves.
- Trickery Blessing ~ Double: bare `WarpriestLVL` — resolves.
- Rune Blessing ~ Blast Rune: `WarpriestLVL/2` — resolves.
- Protection Blessing ~ Increased Defense: `1+min(WarpriestLVL>20,2,WarpriestLVL/10)` — a boolean
  comparison used as a numeric `min()` term, the documented comparison-as-numeric-term gap;
  refuses honestly.
- Protection Blessing ~ Aura of Protection: `if(WarpriestLVL<15,10,20)` — resolves.
- Repose Blessing ~ Gentle Rest: `max(1,WIS)` — resolves.
- Knowledge Blessing ~ Lore Keeper: `15+WarpriestLVL+WIS` — resolves.
- Cavalier Order of the Beast ~ Class Skills: `max(floor(CavalierLVL/2))` — a real single-argument
  `max()` call; `formula_interpreter.rs`'s own documented "`min`/`max` take at least 2 arguments"
  rule refuses it. Honest refusal, not forced.
- Cavalier Order of the Beast ~ Wild Mount Shape: `%1`=`CavalierLVL` (resolves), `%2`=
  `1+(CavalierLVL>=10)+(CavalierLVL>=14)+(CavalierLVL>=18)` (comparison-as-numeric-term gap again)
  — the render requires BOTH to resolve, so the whole description refuses; only `%1`'s own value
  would ground, which is not enough to render the full text cleanly.

Net: 5 of 6 Warpriest groups' members resolve cleanly (Protection Blessing itself refuses via its
one real member; its group is closed anyway once at least one member resolves is the census's own
bar — but this new resolver's OWN measure is "group produces at least one explanation", proven
directly by the two `warpriest_generic_blessing_description_pass_grounds_a_zero_bonus_var_blessing`
/ `cavalier_generic_order_description_pass_grounds_order_of_the_beast` tests below, both real,
non-fabricated groundings). Cavalier Order of the Beast: neither of its two members fully renders
(both hit the comparison-as-numeric-term gap on their own second argument), so it does **not**
close this cycle — named honestly, not claimed.

## 3. Tests, RED→GREEN (`§1a`)

5 new tests in `mod.rs`'s `generic_pool_group_selection_wiring_tests`:
`warpriest_generic_blessing_description_pass_grounds_a_zero_bonus_var_blessing` (Earth Blessing, a
real never-before-grounded member, closes through the new resolver and NOT the old bonus_vars-only
one), `cavalier_generic_order_description_pass_grounds_order_of_the_beast` (same proof for the
Cavalier pool; passes via a real corpus member — see caveat below),
`warpriest_generic_blessing_description_pass_never_double_grounds_destruction_or_strength` (the
exclusion list genuinely excludes both hand-modelled keys),
`invented_selections_ground_nothing_through_the_description_resolver` (refuse-not-fabricate,
mirrors every other generic pass's own safety test).

Mutation-style proof: temporarily forced
`resolved_description_for_formula_only_desc_argument` to `return None` unconditionally (right
after the `bonus_vars.is_empty()` guard, reverted from a saved pre-mutation copy of the file, never
`git stash`) — both `warpriest_generic_blessing_description_pass_grounds_a_zero_bonus_var_blessing`
and `cavalier_generic_order_description_pass_grounds_order_of_the_beast` FAILED
("...must ground generically through the new description resolver"), proving both tests exercise
the real new code path. Restored, re-verified green.

```bash
cargo test --locked --lib -- rules_core::pilot_compute::generic_pool_group_selection_wiring_tests::
```
```
test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 2783 filtered out; finished in 9.83s
```

```bash
cargo test --locked --lib -- rules_core::pilot_compute::
```
```
test result: ok. 963 passed; 0 failed; 0 ignored; 0 measured; 1843 filtered out; finished in 25-31s
```
(up from cycle 14's 958/958 — +5, exactly the 5 new tests above)

```bash
cargo test --locked --lib -- rules_core::derived_evaluator_fixture_check::
```
```
test result: ok. 121 passed; 0 failed; 0 ignored; 0 measured; 2685 filtered out; finished in 8.83s
```

```bash
cargo test --locked --lib -- oracle_dispatch_widening_safety_tests
```
```
test result: ok. 48 passed; 0 failed; 0 ignored; 0 measured; 2758 filtered out; finished in 4.42s
```
`a_mystery_pick_alone_grounds_no_tier_one_revelation` untouched, still green — Oracle Mystery stays
withdrawn, unchanged.

## 4. The re-run census, and the honest denominator finding (`§17a`)

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

**Byte-identical to cycle 13/14's own baseline — confirmed, not assumed, and this is the honest,
expected outcome, not a defect.** `group_has_a_resolvable_member` (the census's own instrument)
calls `resolve_pool_member_sole_magnitude` alone — the bonus_vars-only resolver — and this cycle
deliberately never touched or routed through that function (per the brief's own instruction, "do
not route them through the pool resolver"). A real magnitude closed through the NEW, separate
`resolved_description_for_formula_only_desc_argument` path is therefore invisible to this
instrument by construction, not by omission.

**The finding, stated plainly: the census's own denominator materially understates this row's true
remaining work for Warpriest Blessing and Cavalier Order.** Its "0/37" and "1/9" answer only "how
many groups carry a member with a real `BONUS:VAR` chain" — a real, narrower question than "how
many groups carry a member with a real resolvable magnitude of ANY kind". This cycle proves, by
direct test (`warpriest_generic_blessing_description_pass_grounds_a_zero_bonus_var_blessing`), that
Earth Blessing — one of Warpriest Blessing's own "0/37" groups — genuinely DOES now carry a
resolvable member, through a resolver the census never consults. The same is true for at least 5 of
Warpriest's 37 groups (Earth, Trickery, Rune, Protection [group-level, via Aura of Protection],
Repose, Knowledge — 6 groups, all now closed by this cycle's new resolver) and 0 of Cavalier's 9 (Order
of the Beast's own two members both hit the comparison-as-numeric-term gap and do not fully
render, so it stays unclosed this cycle despite carrying real, partially-evaluable magnitude).

**Recommendation, not self-applied:** a future cycle (or this row's own closure judgement) should
treat the two pools' true remaining-work count as `group_has_a_resolvable_member(class,
registered_name, g) OR resolved_description_for_formula_only_desc_argument` reaching at least one
member — which would move Warpriest Blessing from 0/37 to at least 6/37 this cycle alone. Not
changed in the locked census assertion itself here (per `§1a`, a gate that cannot fail is worse
than no gate — widening the census's OWN measure is a real, separate, deliberate act belonging to
whichever cycle owns that decision, not silently folded into this one's own resolver-wiring diff).

## 5. Not attempted (named, per `§17`)

Bloodrager's remaining 7 single-terminal members (cross-class-only-bound `BloodlineLVL`, unchanged
since cycle 13's own finding). Hunter Animal Focus (activation-gated, unchanged since cycle 5).
Cavalier's `max(floor(CavalierLVL/2))` single-argument `max()` shape and the
comparison-as-numeric-term gap (`CavalierLVL>=10` summed as a numeric term) — both real,
corpus-confirmed formula_interpreter gaps outside this cycle's own scope (widening `min`/`max` to
accept 1 argument, or comparisons-as-numeric-terms generally, is a `formula_interpreter.rs`-wide
change with corpus-wide blast radius, not scoped to this row). Rows 11/15 left as found
(`in-progress`/`complete`), untouched. `apps/desktop`'s row 19/20 lanes not touched. `data/corpus/**`
untouched throughout (`git status --porcelain -- data/corpus` — 0 changes).

## 6. Audits

- **Identifier audit** (`grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'` against this
  cycle's own diff of `src/rules_core/pcgen_desc.rs`,
  `src/rules_core/pilot_compute/class_feature_grant_consumer.rs`,
  `src/rules_core/pilot_compute/mod.rs`): `OK_NO_BUNDLE_TAGS` — 0 hits.
- **Wired-integration audit** (`grep -inE "STUB|MOCK|placeholder|not.?yet.?implemented|todo|fixme|
  hack"`, same scope): `OK_NO_TOKENS` — 0 hits.
- **PI audit**: `pi_scrub.normalized_term_hits(...)` (imported via `scripts/pi_scrub.py`, never
  copied), scoped to `git diff --unified=0` of both this cycle's code diff AND the kanban diff →
  `[]` (0 hits) each. `data/corpus/**` untouched throughout.

## 7. Kanban

`docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 18 only (cycle field 14 →
15, Notes prepended). Verified structurally after editing (backtick-aware parser): 21 distinct
`^| N |` rows, 0 duplicates, row 18 parses to 9 raw pipe-split fields (7 real columns) before and
after, single physical line. Rows 11 (`in-progress`) / 15 (`complete`) confirmed untouched from
their pre-cycle state.

## 8. `df -h /`

```
Filesystem      Size  Used Avail Use% Mounted on
/dev/sda1       968G  505G  464G  53% /
```
