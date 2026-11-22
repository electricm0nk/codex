# Cycle t12-class-feature-pool-population, cycle 16 — Gate 3 (closure invariant) / row 18 (`epic-8-pool-shaped-class-features`)

- **Card ID:** row 18 (`epic-8-pool-shaped-class-features`), governed by `decisions.md §27b`
  (*"EVERYTHING"*). Dispatched scope: cycle 15's own measuring instrument
  (`pool_group_closure_census_across_all_six_pools`) is blind by construction to the new
  description-formula resolver it just built — fix the instrument first, validate it against
  known truth, then continue the remaining named work in evidence order.
- **Base:** `git merge-base --is-ancestor "$PIN" HEAD` returned FALSE at session start — the
  worktree started on a stale/unrelated lineage (PR #374's merge commit `1bb523773d`, footgun 3,
  same shape cycle 15 hit). Fixed: `git reset --hard "$PIN"`, `git fetch origin tranche/12 &&
  git rebase origin/tranche/12` — `origin/tranche/12`'s tip was still `e2c3e73956` (cycle 15's own
  commit) at rebase time, so no sibling lane had landed on row 18's files since cycle 15.
- **Oracle:** the repo's own `preflight-oracle` initially reported PASS against the FORBIDDEN
  default path (no `PCGEN_REPO_DIR` export) — caught before trusting it, per the brief's own
  footgun 4 warning. Re-run with `PCGEN_REPO_DIR`/`PCGEN_CORPUS_ROOT` explicitly exported to this
  worktree's own `artifacts/corpus/operator-supplied/pcgen` slot: correctly FAILED (empty,
  git-ignored slot). Bootstrapped via `scripts/fetch-pcgen-oracle.sh --dest
  <worktree>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen`
  — `pcgen-oracle: OK 7f818006e371188e5717fd18d74d18a420747fc6`, pin confirmed.

## 1. The census extension (`§17a` — fix the known-wrong instrument first)

Added `pool_group_closure_census_across_all_six_pools_both_resolvers`, a NEW test in
`mod.rs`'s `generic_pool_group_selection_wiring_tests`, alongside (never replacing) cycle 8's own
locked `pool_group_closure_census_across_all_six_pools`:

- `group_has_a_resolvable_member_via_description_formula(class, registered_name, group)` — the
  SAME "at least one member" measure as `group_has_a_resolvable_member`, but calling cycle 15's
  `resolved_description_for_formula_only_desc_argument` instead of
  `resolve_pool_member_sole_magnitude`.
- `group_has_a_resolvable_member_via_either_resolver` — ORs the two. The two resolvers are
  mutually exclusive per-record (the description resolver's own `bonus_vars.is_empty()` guard,
  documented on that function since cycle 15), so no record is ever double-counted.
- The new test's own first assertion re-derives and locks the bonus_vars-only figures EMBEDDED in
  its combined report, and asserts they match `pool_group_closure_census_across_all_six_pools`'s
  own locked baseline byte-for-byte — proof this pass extends the first instrument rather than
  silently redefining it. Only after that passes does the test assert the combined figure.

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

**The honest all-resolver figure, re-derived, is what row 18's closure is judged on from here:**
Warpriest Blessing **0/37 → 8/37**, Cavalier Order **1/9 → 2/9**, plus small movement on three
pools cycle 15's own manual review never checked (Bloodrager 5/12→6/12, Cleric 34/72→35/72, Shaman
11/14→12/14). Sorcerer Bloodline is genuinely unchanged at 31/53.

## 2. Two overstated findings this cycle corrected (`§17a` — validate before trusting, correct don't delete)

A temporary diagnostic (`diagnostic_desc_formula_closures_by_group`, printed then removed before
commit, per this bundle's own methodology) listed every group the description resolver newly
grounds. Two results contradicted cycle 15's own receipt:

**a) Cavalier Order of the Beast genuinely closes — cycle 15's "comparison-as-numeric-term gap"
claim for it was wrong.** Cycle 15 said Wild Mount Shape's `%2` argument —
`1+(CavalierLVL>=10)+(CavalierLVL>=14)+(CavalierLVL>=18)` — "hits the documented
comparison-as-numeric-term gap" and refuses. Direct reproduction disproves this: every comparison
here is **parenthesised**, and `formula_interpreter.rs`'s own module doc already documents wave
26's parenthesised-comparison-as-numeric-primary support (`parse_primary`'s `LParen` branch calling
`parse_arith_or_bool`, doc: `"1+(X>=15)" needs no special coercion in the real engine`). At
character level 5, `%2` resolves to 1 and the whole description renders clean. Order of the Beast
was never blocked by a real gap — cycle 15's own manual per-member walkthrough simply didn't
re-check the interpreter's documented capability before writing the claim. Logged:
`scripts/retro.py correction` `1787582848402-t9-onboarding-9a4294`.

**b) The REAL gap is narrower than "comparison-as-numeric-term": it is an UNPARENTHESISED
comparison used as a bare function argument.** `Protection Blessing ~ Increased Defense`'s
`1+min(WarpriestLVL>20,2,WarpriestLVL/10)` genuinely refuses (confirmed:
`resolved_description_for_formula_only_desc_argument("Protection Blessing ~ Increased Defense", 5,
...)` returns `None`) — but NOT for the reason cycle 15 named. `min`/`max`'s parser reads each
comma-separated argument via `parse_expr()` (plain arithmetic), never
`parse_arith_or_bool` (the comparison-aware entry point) — only a PARENTHESISED sub-expression
reaches `parse_arith_or_bool` via `parse_primary`'s `LParen` branch. `WarpriestLVL>20` as a bare,
unparenthesised `min()` argument therefore genuinely fails to parse. This does not affect row 18's
own group counts this cycle (Protection Blessing already closes via its other member, Aura of
Protection), but it is the real, narrower shape a future cycle should size against, not the
broader "comparison-as-numeric-term" cycle 15 named.

## 3. The single-argument `max()`/`min()` fix (`§17` — size then verify against the oracle)

Cycle 15 named `Cavalier Order of the Beast ~ Class Skills`'s `max(floor(CavalierLVL/2))` a real
`formula_interpreter.rs` gap ("`min`/`max` take at least 2 arguments") and refused to widen it,
calling the blast radius corpus-wide.

**Sized corpus-wide** (`python3` walking `data/corpus/**/*.json` for a `min(`/`max(` call with
exactly one top-level, balanced-paren, comma-split argument — script in this receipt's own
history, re-runnable):

```
single-arg min/max() calls (DESC-formula shape): 3 unique corpus records
  Cavalier Order of the Beast ~ Class Skills           max(floor(CavalierLVL/2))
  Barbarian rage power ~ Undead Blood (Lesser)         max(floor(BarbarianLVL/2))
  Voice of the Wild ~ Wild Knowledge                   max(floor(BardLVL/2))
unparenthesized-comparison-as-bare-function-argument (the REAL gap, §2b above): 1 unique record
  Protection Blessing ~ Increased Defense              min(WarpriestLVL>20,2,WarpriestLVL/10)
```

**Verified against the pinned PCGen oracle before implementing** (`§17`):
`git show HEAD:code/src/java/plugin/jepcommands/MaxCommand.java` in the bootstrapped oracle
checkout — `numberOfParameters = -1` (variable-arity); `run()` pops each stack parameter and folds
via `if (first || param > result) result = param;`, so a SINGLE parameter is simply returned
unchanged (`first` is true on the only iteration). The real oracle accepts and correctly evaluates
`max(X)` with one argument. `MinCommand.java` is the documented mirror. **This module's own
`< 2`-argument restriction was itself the bug — a real, disclosed divergence from the oracle, not
an unimplemented corner case.**

**Fix, in `formula_interpreter.rs`:** removed the `"min" | "max" if args.len() < 2` arity guard
entirely. No replacement `< 1` guard is needed or added — `parse_call`'s `min`/`max`/`floor`/
`ceil`/`abs` branch already always supplies `args = vec![self.parse_expr()?]` (one element) before
the comma loop, so a genuine zero-argument call can never reach this arity check at all; the
removed branch was the ONLY thing refusing a valid, oracle-matching 1-argument call.

**A pinned test encoded the wrong assumption** (failure mode 2, named in the brief) —
`formula_interpreter::tests::wrong_arg_counts_refuse` asserted `recognises_shape("max(1)").is_err()`.
**Corrected, not deleted**: the `max(1)` assertion is removed with a comment explaining why, and a
new test proves the real property:
`single_argument_min_max_now_matches_the_oracles_variable_arity_max_min_command` —
`max(7)`/`min(7)` both resolve to `7`, and `max(floor(CavalierLVL/2))` at `CavalierLVL=9` resolves
to `4`. Logged: `scripts/retro.py correction` `1787583677793-t9-onboarding-70c2ad`.

**Mutation-proved RED, then reverted**: temporarily reintroduced the old `< 2` guard (returning a
`MUTATION-PROOF-TEMP` error), re-ran the new test — FAILED as expected (`unwrap()` on the
temporary `Err`) — then restored the fix from a saved pre-mutation copy of the file (never `git
stash`), re-verified GREEN.

Row 18's own group counts are unaffected by this fix this cycle — Cavalier Order of the Beast
already closes via Wild Mount Shape (§2a above), so Class Skills grounding too does not move the
census's per-group "at least one member" bar further. The fix is real, oracle-verified, and
closes 3 corpus records' engine-level capability regardless.

## 4. Tests, full re-run

```bash
cargo test --locked --lib -- rules_core::pilot_compute::formula_interpreter::tests::
```
```
test result: ok. 38 passed; 0 failed; 0 ignored; 0 measured; 2771 filtered out
```
(up from cycle 15's untouched baseline — this file was not modified before this cycle; +1 new
test, `wrong_arg_counts_refuse` corrected in place)

```bash
cargo test --locked --lib -- rules_core::pilot_compute::
```
```
test result: ok. 966 passed; 0 failed; 0 ignored; 0 measured; 1843 filtered out
```
(up from cycle 15's 963/963 — +1 combined-census test, +1 min/max oracle test, +1 net after two
temporary diagnostic tests were added then removed before commit)

```bash
cargo test --locked --lib -- rules_core::derived_evaluator_fixture_check::
```
```
test result: ok. 121 passed; 0 failed; 0 ignored; 0 measured; 2688 filtered out
```
(unchanged from cycle 15 — this cycle's `formula_interpreter.rs` change touches only the parser's
arity gate, never any fixture-checked evaluated value)

```bash
cargo test --locked --lib -- oracle_dispatch_widening_safety_tests
```
```
test result: ok. 48 passed; 0 failed; 0 ignored; 0 measured; 2761 filtered out
```
`a_mystery_pick_alone_grounds_no_tier_one_revelation` untouched, still green — Oracle Mystery
stays withdrawn, unchanged.

## 5. Not attempted this cycle (named, per `§17`)

- **The real, narrower unparenthesised-comparison-as-bare-function-argument gap** (§2b/§3 above,
  1 corpus record, `Protection Blessing ~ Increased Defense`) — real, sized, but its fix (routing
  `min`/`max`/`floor`/`ceil`/`abs` argument parsing through `parse_arith_or_bool` instead of
  `parse_expr`) is a grammar-shape change with a wider blast radius than the single-arg fix above
  (it changes what EVERY function-argument position accepts, not just an arity check), so it was
  sized and verified but not implemented this cycle. Does not affect any of row 18's own group
  counts this cycle (Protection Blessing's group already closes via its other member).
- **Bloodrager's remaining 7 single-terminal members** — unchanged since cycle 13's own finding,
  each chains to a per-bloodline `BloodlineLVL` genuinely bound only on a same-named cross-class
  (`Eldritch Scion`) record.
- **Hunter Animal Focus** — activation-gated, unchanged since cycle 5.

Rows 11/15 left as found (`in-progress`/`complete`), untouched. `apps/desktop`'s row 19/20 lanes
not touched. `data/corpus/**` untouched throughout (`git status --porcelain -- data/corpus` — 0
changes).

## 6. Audits

- **Identifier audit** (`grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'` against this
  cycle's own diff of `src/rules_core/pilot_compute/mod.rs`,
  `src/rules_core/pilot_compute/formula_interpreter.rs`,
  `docs/release/SD-32-compute-library-and-cause-closure/kanban.md`): `OK_NO_BUNDLE_TAGS` — 0 hits.
- **Wired-integration audit** (`grep -inE "STUB|MOCK|placeholder|not.?yet.?implemented|todo|fixme|
  hack"`, same scope): `OK_NO_TOKENS` — 0 hits.
- **PI audit**: `pi_scrub.normalized_term_hits(...)` (imported via `scripts/pi_scrub.py`, never
  copied), scoped to `git diff --unified=0` of this cycle's code diff, the retro log diff, AND the
  kanban diff → `[]` (0 hits) each. `data/corpus/**` untouched throughout.

## 7. Kanban

`docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 18 only (cycle field 15 →
16, Notes prepended). Verified structurally after editing (backtick-aware parser): 21 distinct
`^| N |` rows, 0 duplicates, row 18 parses to 9 backtick-aware raw pipe-split fields (7 real
columns) before and after (matching cycle 15's own count exactly, confirmed by re-deriving it
fresh rather than trusting the prior receipt's figure), single physical line. Rows 11
(`in-progress`) / 15 (`complete`) confirmed untouched from their pre-cycle state (`git diff` shows
exactly 1 line changed in the whole file).

## 8. `df -h /`

```bash
df -h /
```
```
Filesystem      Size  Used Avail Use% Mounted on
/dev/sda1       968G  510G  459G  53% /
```
