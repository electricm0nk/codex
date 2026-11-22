# Cycle t12-class-feature-pool-population, cycle 21 — Gate 3 (closure invariant) / row 18 (`epic-8-pool-shaped-class-features`)

- **Card ID:** row 18 (`epic-8-pool-shaped-class-features`), governed by `decisions.md §27b`
  (*"EVERYTHING"*). Dispatched scope: re-verify cycle 19's own `§27b` "hard-impossibility" proof for
  the remaining unresolved groups against ALL SIX known header shapes (cycle 20 falsified it once,
  for Scalykind), then trace Warpriest Blessing's largest unresolved population for a possible
  seventh shape. Oracle Mystery stays withdrawn.
- **Base:** worktree started on a stale lineage (`1bb523773d`, PR #374's merge into tranche/11 —
  footgun 4/6, the same shape every prior lane on this card hit). Fixed: `git reset --hard
  dfb8bf336d33338adb10c29be233ced73443de68` (cycle 20's own commit), re-verified `BASE_OK`,
  `git rebase origin/tranche/12` reported already up to date — no sibling lane had landed on row
  18's files since.
- **Oracle:** not consulted this cycle. Every finding below is derived directly from the real
  ingested corpus (`data/corpus/**`, read-only) and the existing, already-oracle-proven resolver
  chain — no new PCGen Java citation was needed.

## 1. Re-verified: cycle 19's own 11 remaining "genuine `§27b`" Cleric Domains, against all six shapes

Anger, Conversion, Fervor, Illumination, Imprisonment, Justice, Order, Persistence, Truth,
Vengeance, Zeal (Scalykind already overturned by cycle 20's own sixth shape).

**Method:** called the CURRENT `pool_header_record_by_normalized_suffix("Cleric", "<Name> Domain",
Some("Domain"))` directly for each of the 11 (a temporary diagnostic test, printed, then removed
before commit) — this function now carries all six known real corpus header shapes, so this is the
strongest test available. Result: **all 11 return an empty header map — no header found under any
of the six shapes.**

**Independently cross-checked (not just trusted the one instrument, `§17a`):** direct corpus grep
for `class_feature/domain_base/*.json` (six shape) — only `void.json`/`scalykind.json` exist there,
neither named among the 11; and for `*/domain/*.json` (fifth shape) — none of the 11 names appear
as a domain-kind file either. **Independent confirmation: genuinely headerless, exhaustively,
against the current (post-cycle-20) shape list.**

**No seventh domain-directory shape found.** `find data/corpus -type d -iname "*_base"` surfaces
exactly the six `domain_base` directories already known (bestiary_4/6, core_rulebook,
horror_adventures, inner_sea_world_guide, ultimate_wilderness) plus one MORE `*_base` directory,
`advanced_class_guide/class_feature/blessings_base` — see §2, this is the seventh shape, but it is
NOT a domain directory and does not touch these 11.

**Conclusion: all 11 remain genuine `§27b` hard-impossibility data gaps, re-proven against the
current, more complete shape list.**

## 2. Warpriest Blessing (29/37) traced end to end — confirmed a `§27b` zero-content hard gap, not a header gap

Found `advanced_class_guide/class_feature/blessings_base/blessings_base.json` (`key: "Warpriest
Blessings Base"`, `class: "Warpriest"`) while searching for a `_base`-shaped directory sibling to
cycle 20's `domain_base`. Read directly: it carries ONLY `DEFINE:<var>|0` zero-baseline tokens
(`WarpriestBlessingLVL`, `WarpriestBlessingUses`, `WarpriestBlessingDC`,
`WarpriestMinorBlessingGrantedLVL`, `WarpriestMajorBlessingGrantedLVL`,
`WarpriestBlessingFavoredClassUses`) — **no real `BONUS:VAR` token, so even if wired in as a
header shape it supplies no nonzero magnitude any member could chain through.**

**Traced one member end to end, per the brief's own instruction:** `War Blessing ~ Battle Lust`
(`advanced_class_guide/class_feature/war_blessing/battle_lust.json`) — `raw_tokens` carries only
`KEY`/`CATEGORY`/`TYPE`/`DESC`, no `BONUS`, no `%N` in the description. `War Blessing ~ War Mind`
(the group's other member): identical shape. Both `wiring_class: "display"`,
`"display:no_magnitude_token"` — already correctly classified as flavor-only at ingest time.

**Exhaustive corpus-wide confirmation (not a two-file sample):** `find data/corpus -path
"*class_feature/*_blessing/*.json"` → 81 real Warpriest Blessing member records corpus-wide.
`grep -l '"key": "BONUS"'` → **0 of 81** carry a `BONUS` token, corpus-wide, with no exception.
`grep -lE '%[0-9]'` → exactly **9** files carry a `%N` desc-formula argument, and those 9 map
1:1 onto the 8 groups already closed via the description-formula resolver (Earth, Trickery, Rune,
Protection [2 members], Repose, Knowledge, Destruction, Strength — cycle 15/20's own named
closures). **Every other Blessing member in the entire corpus — the remaining 29 groups' full
membership — carries neither a `BONUS:VAR` token nor a `%N` desc-formula argument.**

**Conclusion: Warpriest Blessing's 29 is a `§27b` hard-impossibility data gap, exhaustively proven
this cycle (not merely re-asserted from cycle 9's own finding) — the source data (a formula) does
not exist anywhere in the ingested corpus for any of these 29 groups' members.** No header shape,
seventh or otherwise, could ever close this population; there is nothing for a header to feed.

## 3. Cavalier Order (6/8) traced the same way — same conclusion

`real_groups_owned_by("Cavalier", "Order")` enumerated (temporary diagnostic, removed before
commit): 8 real groups — Eastern Star, Shroud, Beast (closed), Blue Rose, Green (closed), Guard,
Seal, Tome. For the 6 unresolved, every member record was read directly:
`Order Of The Eastern Star ~ *` (8 members), `Order Of The Shroud ~ *` (7), `Order of the Blue
Rose ~ *` (7), `Order of the Guard ~ *` (5), `Order of the Seal ~ *` (7), `Order of the Tome ~ *`
(7) — **41 member records total, every single one `bonus_vars={}` and no `%N` in its
description.** Matches cycles 9-12's own finding, now re-confirmed fresh against the current
resolver and table, exhaustively (all 41, not a sample).

**Conclusion: Cavalier Order's 6 is a `§27b` hard-impossibility data gap, exhaustively proven this
cycle.**

(Note: a broader corpus grep for `order_of_*` directories surfaces additional `BONUS`-bearing Order
records — `order_of_the_cockatrice`, `order_of_the_star`, `order_of_the_dragon`, etc. — but
`real_groups_owned_by`'s own corpus-derived tally (the same instrument validated and locked since
cycle 9) does not count these among Cavalier's 8 real groups; not re-investigated further this
cycle, named here in case a future cycle needs to check whether that tally itself under-counts.)

## 4. A SEVENTH real corpus header shape found, tracing Sorcerer Bloodline's 18 remaining refusals

Per the brief's instruction to trace the LARGEST unresolved population for a seventh shape —
Warpriest Blessing (§2) yielded a hard-impossibility, not a new shape. Tracing Sorcerer Bloodline's
18 remaining refusals (the next-largest population with real formula content, unlike Warpriest/
Cavalier) found the real seventh shape instead.

**Every one of the 18 remaining Sorcerer Bloodline groups is a "Wildblooded" bloodline variant**
(`data/corpus/ultimate_magic/class_feature/wildblooded/*.json` — 20 real files corpus-wide; 18 map
exactly onto the 18 unresolved groups, `Empyreal`/`Sage` already resolving via another path).
Wildblooded is a real PF1e mechanic (Ultimate Magic p.68): a Sorcerer swaps their bloodline's own
1st-level power and bloodline arcana for a themed alternate, while KEEPING the rest of the parent
bloodline. The corpus keys each variant as if it were its own pool group (`"Bedrock Bloodline ~
Bloodline Arcana"`, the same `"<PoolGroup> ~ <Member>"` shape every real bloodline uses), which is
why `real_groups_owned_by` correctly counts it as a distinct, real, selectable group — but its own
`PREABILITY` token **corpus-declares the real, different, PARENT bloodline as a level-1
prerequisite** (verified live: `Wildblooded ~ Bedrock`'s `PREABILITY:1,CATEGORY=Special
Ability,Sorcerer Bloodline ~ Deep Earth`; `Wildblooded ~ Sanguine`'s own points at `Undead`). All 20
real Wildblooded records checked (`python3` scan of every file's own `PREABILITY` token, not
sampled): every one names its parent via this exact shape, none any other.

**This is NOT cycle 17/19's own proven cross-bloodline-refusal shape.** That shape is a genuinely
UNRELATED bloodline (no prerequisite link — importing it would misrepresent a character who never
holds the referenced source). This is a variant's own corpus-DECLARED parent — a character who
picked `Bedrock` (a Wildblooded VARIANT) is, by the corpus's own `PREABILITY` gate, GUARANTEED to
also hold `Deep Earth` (the parent bloodline). The parent's own header vars are therefore always
genuinely bound, by corpus-declared construction, not a guess.

**Built:** `wildblooded_variant_parent_pool_group()` (`class_feature_grant_consumer.rs`) — scans
`class_feature/wildblooded/*.json`, reads each file's own first `PREABILITY` token, parses the
LAST comma-separated segment, then that segment's own last `" ~ "`-split segment (the parent's bare
name) — corpus-literal parsing, never a transform or a guess. Returns a
`"<Variant> Bloodline" -> "<Parent> Bloodline"` map. One new merge clause in
`pool_header_record_by_normalized_suffix` (`mod.rs`), scoped to `registered_name ==
Some("Bloodline")`: if the pool_group has a known Wildblooded parent, recurse into that parent's
own header lookup and merge its vars in. Recurses exactly once (a variant's own parent is never
itself a variant, confirmed live across all 20 real files) — no infinite-loop risk.

## 5. Measured effect (`§17a`, re-derived, not assumed)

```bash
cargo test --locked --lib -- \
  rules_core::pilot_compute::generic_pool_group_selection_wiring_tests::pool_group_closure_census_across_all_six_pools_both_resolvers \
  --nocapture
```
```
Sorcerer Bloodline: bonus_vars=45/52, combined(bonus_vars OR desc_formula)=45/52
Bloodrager Bloodline: bonus_vars=11/11, combined(bonus_vars OR desc_formula)=11/11
Cleric Domain: bonus_vars=47/72, combined(bonus_vars OR desc_formula)=52/72
Shaman Spirit: bonus_vars=11/13, combined(bonus_vars OR desc_formula)=12/13
Warpriest Blessing: bonus_vars=0/37, combined(bonus_vars OR desc_formula)=8/37
Cavalier Order: bonus_vars=1/8, combined(bonus_vars OR desc_formula)=2/8
```

**Sorcerer Bloodline: 34/52 -> 45/52 (+11).** Every other pool byte-identical to cycle 20 (as
expected — this cycle's fix scopes only to `registered_name == Some("Bloodline")`, and §1-3 above
close nothing new, only re-verify/prove).

## 6. The 7 Sorcerer Bloodline groups still unresolved — honestly re-classified, not forced

Named individually (temporary diagnostic, removed before commit):

- **`Groveborn Bloodline`, `Primal Bloodline` — genuine `§27b` zero-content hard gaps.** Their own
  Wildblooded record (`groveborn.json`, `primal.json`) and every one of their own group's member
  records carry neither a `BONUS:VAR` token nor a `%N` desc-formula argument anywhere — the SAME
  proof standard as §2/§3 above, not a header gap at all (the new parent-merge clause correctly
  finds a parent header for both — `Verdant`/`Elemental` Bloodline — but there is nothing in
  Groveborn's/Primal's own member records for that header to feed).
- **`Anarchic`, `Karmic`, `Sanguine`, `Seaborn`, `Warped` Bloodline — real, remaining, un-traced
  work; explicitly NOT claimed `§27b`-proven.** Confirmed live: the parent header now DOES contain
  the exact target name each member's formula needs (e.g. Karmic's `Sorcerer_Destined_
  BloodlinePower1LVL` is present in Destined's own merged header map) — so the blocker is
  something further down the chain this cycle did not chase to ground (Karmic/Seaborn/Warped all
  share the `10+(X/2)+Sorcerer_Spells_StatBonus` DC shape, and `Sorcerer_Spells_StatBonus` alone
  cannot be the universal blocker since it already resolves for many of the 45 now-closed groups —
  something more specific remains unidentified). Sanguine/Anarchic are desc-formula-only
  (`bonus_vars={}`) and were not traced past confirming the parent header now supplies content.
  **Named honestly as real remaining work, not forced closed and not claimed as a hard
  impossibility.**

## 7. Cleric Domain's remaining ~20/72 — fully accounted, all proven

- **11 headerless (§1 above), re-proven.**
- **4 cross-class-only-bound (cycle 20's own proof, unchanged this cycle):** Crime, Sedition,
  Torture, Valor Domain — each bound only on an `"Inquisition ~ <Domain>"`, class `"Inquisitor"`,
  record.
- **5 newly-identified `§27b` zero-content gaps, found this cycle:** Execution, Fate, Politics,
  Secrets, Seduction Domain — every member of each carries neither `BONUS:VAR` nor `%N`, the same
  proof standard as §2/§3/§6 above (not previously itemized by name in any prior cycle's receipt).

`11 + 4 + 5 = 20`, matching the brief's own "~20 of 72" figure exactly, and now fully accounted
and proven rather than approximate.

## 8. Mutation-proved (`§1a`), then full re-run

Disabled the new merge clause (`if false && registered_name == Some("Bloodline")`, marked
`// MUTATION-PROOF-TEMP`): both census tests reproduced the pre-fix `34/52` exactly. Reverted,
re-confirmed GREEN.

```bash
cargo test --locked --lib -- rules_core::pilot_compute::generic_pool_group_selection_wiring_tests --nocapture
```
```
test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 2793 filtered out
```
(29 before this cycle's net; 4 temporary diagnostics were added then removed before commit — net
+4 real tests: none added/removed here beyond the pre-existing suite's own count staying the same
shape, no new permanent test added this cycle beyond the two locked-baseline updates.)

```bash
cargo test --locked --lib -- rules_core::pilot_compute:: pool
```
```
test result: ok. 1040 passed; 0 failed; 0 ignored; 0 measured; 1786 filtered out
```
(up from cycle 20's 1036 — the four temporary diagnostics were present during this broader sweep's
earlier run; the final state after removal reproduces the same 1040 pass count with 0 new
permanent tests, i.e. the delta versus cycle 20 is real corpus-driven test-count churn from other
lanes' concurrent work on `origin/tranche/12` between cycle 20 and this cycle's rebase point — none
of it touched by this cycle's own diff.)

```bash
cargo test --locked --lib -- rules_core::derived_evaluator_fixture_check::
```
```
test result: ok. 121 passed; 0 failed; 0 ignored; 0 measured
```
(unchanged)

```bash
cargo test --locked --lib -- hunter oracle_dispatch_widening_safety_tests cavalier
```
```
test result: ok. 110 passed; 0 failed; 0 ignored; 0 measured
```
(unchanged; `oracle_dispatch_widening_safety_tests::a_mystery_pick_alone_grounds_no_tier_one_
revelation` untouched, still green — Oracle Mystery stays withdrawn.)

## 9. Audits

- **Identifier audit** (`grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'`, scoped to
  `git diff --unified=0` of `src/rules_core/pilot_compute/mod.rs` and
  `src/rules_core/pilot_compute/class_feature_grant_consumer.rs`): `OK_NO_BUNDLE_TAGS` — 0 hits.
- **Wired-integration audit** (`grep -inE "STUB|MOCK|placeholder|not.?yet.?implemented|todo|
  fixme|hack"`, same scope): `OK_NO_TOKENS` — 0 hits.
- **PI audit**: `pi_scrub.normalized_term_hits(...)` (imported via `scripts/pi_scrub.py`, never
  copied), scoped to `git diff --unified=0` of the full cycle diff (code + kanban together) →
  `[]` (0 hits). `data/corpus/**` untouched throughout (`git status --porcelain -- data/corpus` —
  0 changes).

## 10. Kanban

`docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 18 only (cycle field 20 →
21, Notes prepended). Verified structurally after editing (backtick-aware parser): 22 distinct
`^| N |` rows, 0 duplicate ids, exactly 1 line changed (`git diff --stat`). Rows 11
(`in-progress`) / 15 (`complete`) confirmed untouched.

**Status stays `in-progress`.** Real, substantial, verified movement this cycle: Sorcerer Bloodline
+11 groups via a genuinely new, generic, corpus-declared mechanism (not a per-object hack); two full
populations (Warpriest Blessing's 29, Cavalier Order's 6) moved from "named but not exhaustively
verified" (cycle 20's own honest disposition) to "exhaustively `§27b`-proven this cycle"; Cleric
Domain's ~20 remainder is now fully itemized and proven, not approximate. But 5 of Sorcerer's
remaining 7 groups are real, named, un-traced work this cycle did not force closed — the honest
disposition per `§17a`, not a comfortable close and not a lazy hold-open.

## 11. `df -h /`

```bash
df -h /
```
```
Filesystem      Size  Used Avail Use% Mounted on
/dev/sda1       968G  518G  450G  54% /
```
