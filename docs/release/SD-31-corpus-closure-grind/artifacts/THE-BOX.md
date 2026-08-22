---
canonical: true
wave: 28
wave_29_addendum: true
purpose: >
  The consolidated map of the whole box. Six lanes looked at every not-done unit corpus-wide,
  banked nothing, and named what they found. Two independent adversarial reviews stress-tested
  every count. This document merges the two, applies every correction, reconciles the piles
  against the board total, and ranks what to do next. It is the wave's entire deliverable.
board_at_close: "13,456 / 38,372 (35.07%) at wave 28 close; 13,458 / 38,372 (35.08%) at wave 29 close — see the wave-29 addendum immediately below"
---

# THE BOX — SD-31 wave 28 consolidated visibility map

## Wave 29 addendum (2026-08-21) — what this wave actually proved or disproved against §3's ranked list

**#1, fix the self-erasing fixture generator — DONE, and genuinely proven dead, not merely
patched.** Reproduced the exact defect this map named (2,110 rows, 8 families, one run from
committed state — the map's own "~2,109" was one unit low, corrected here). The landed fix had
TWO root causes, not the one this map's own filed "obvious one-line fix" theory assumed:
preserve-by-exclusion (fixes the write step rebuilding the whole document and dropping every
sibling family) is what actually restores all 8 families; `HELD_STATUSES` missing
`fixture-verified` is a real, separate, independently-necessary bug, but adding `literal-verified`
alongside it (the integration lane's first attempt) restores **zero** additional rows — it is
structurally unreachable (`literal-verified` only stamps `wiring_class == static`; this generator
only ever selects `wiring_class == "derived"`). Corrected in `todo/defects.md` D7 / `todo/sweeps.md`
S6. Both real causes are now independently mutation-proven (module-level `assert` +
`main()`-level FATAL guard for the `HELD_STATUSES` half, `shrunk_families` for the preserve half) —
run twice from the merged state by the integration cycle itself, byte-identical both times. Ruling
§20's interpreter-scale regen can now proceed against this seam.

**#2, wire `class_feature_pool_group_matches()` — DONE, exactly as sized.** 612 units moved
`unmeasurable` → mostly `not-started` (610) / `deferred` (2). Confirmed SOUND by a THIRD independent
review (this map's own #2 already called it SOUND; wave 29's own adversarial review re-confirmed
via full guarded regen, catching two self-inflicted test breaks the lane itself had not disclosed,
both fixed by integration). Zero doneness gain from this lever alone, as predicted.

**#3, book onboarding — STARTED, not finished, and it surfaced Ruling §18's #1 residual risk.**
`adventurers_guide` registered via its spell family (45/49 base spells; PI-screened, 4 correctly
dropped). All 973 `adventurers_guide` units moved off the `no_compiled_rule_set_for_book` gate this
map's G4 group named. But unblocking the book also unblocked 5 archetype-adjacent Rage Power
`class_feature` records that reached `done` through the SAME pool-catalog seam this map's §2.2 #2
lever uses — 3 of the 5 are genuinely archetype-locked (`PREABILITY CATEGORY=Archetype`, a
character who never takes that archetype can never take the option) and would have been a real,
if small, Ruling §18 violation if left uncaught. **This is the first live-fire case of `blocked.md`
B3** ("has anyone checked whether the shipped pool catalog honours §18's prerequisite condition") —
answered this cycle: `is_archetype_locked()` now refuses any `PREABILITY ... CATEGORY=Archetype`
record corpus-wide (6 of 300 Rogue Talent/Rage Power records total), with **zero regression to any
of the 123 units already banked through this same catalog** — the fix is scoped precisely enough
that it costs nothing already shipped. B3 closed with a real corpus-wide count, not a proposal.
Remaining 3 books (`inner_sea_magic`, `inner_sea_temples`, `inner_sea_taverns`) and this book's own
feat/equipment/class_feature-chassis families are still open, ≥1,300 units behind the same gate
shape — see `todo/levers.md` L10.

**New this wave, not on the wave-28 map at all**: F2 (the class_feature→feat cross-reference
bridge, `todo/levers.md` L9) was built, tested, and independently re-verified to identify 471 real
records — but adversarial review found the render path it rides cannot reach ANY current character
sheet (0 of 471, not "up to 471"; only 1 of 471 even names a holdable class token, and that one has
no engine explanation to attach to). Filed as a corrected, explicitly-guarded lever rather than a
ready-to-hook doneness lever — the wrong hook here would have been a 471-unit manufactured-credit
trap. L8 (F1's render-surface gap) reconfirmed unchanged: still genuinely blocked on a new frontend
surface, not a `classify()` change. Full detail: `progress.md`'s wave 29 receipt.

---

## 0. Board state (confirm unchanged)

```
md5sum docs/work-inventory.json
de0dfa8614efdd027316ccf274ad8490   (matches e90ba9ec1, matches wave-start, matches wave-close)
```

No lane wrote production logic, reclassified a unit, ran the guarded regen, or touched
`docs/work-inventory.json`. Both adversarial reviews independently confirm this — each lane's
entire git diff is exactly one new file under `artifacts/`. **Board movement this wave = 0.** That
is correct; this wave banks nothing by design.

```
python3 -c "
import json,sys,collections
sys.path.insert(0,'scripts/observer')
import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
v=lambda u: P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind'))
c=collections.Counter(v(u) for u in U)
print(len(U), c.most_common())
"
# -> 38372 [('not-started', 18188), ('done', 13456), ('unmeasurable', 4270),
#           ('in-progress', 1231), ('held', 1185), ('deferred', 42)]
```

`13,456/38,372 = 35.07%`. Not-done = `18,188 + 4,270 + 1,231 + 1,185 + 42 = 24,916`. **The
dispatch brief's in-progress/held figures (1,241/1,187) were a few hours' drift against the
committed inventory — the real, current, byte-verified figures are 1,231/1,185, plus a 42-unit
`deferred` bucket the brief did not name at all.** Corrected here, not treated as an error anywhere
below.

---

## 1. Reconciliation — do the piles sum to 24,916?

**No, not as filed — and finding out why is the check no individual lane could perform.** Two
things had to be corrected before the piles line up: an undercount (295 units nobody examined) and
an overcount (1,212 units two lanes both examined and both counted).

### 1.1 Per-kind ground truth (the board split by `kind`, independent of any lane)

```
python3 -c "
import json,sys,collections
sys.path.insert(0,'scripts/observer')
import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
v=lambda u: P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind'))
by_kind=collections.defaultdict(collections.Counter)
for u in U: by_kind[u.get('kind')][v(u)] += 1
for k in sorted(by_kind):
    c=by_kind[k]; nd=sum(n for verdict,n in c.items() if verdict!='done')
    print(k, dict(c), 'not-done=', nd)
"
```

| kind | not-started | unmeasurable | in-progress | held | deferred | **not-done** |
|---|---:|---:|---:|---:|---:|---:|
| class | 157 | – | – | – | – | **157** |
| class_feature | 11,971 | 3,058 | – | 38 | 40 | **15,107** |
| companion | 769 | – | – | 56 | – | **825** |
| equipment | 222 | 205 | 261 | 207 | – | **895** |
| equipment_modifier | 63 | 416 | 568 | 17 | – | **1,064** |
| feat | 496 | 565 | 1 | 87 | 2 | **1,151** |
| monster | 28 | – | – | 253 | – | **281** |
| monster_ability | 879 | – | – | 273 | – | **1,152** |
| race | 57 | – | 3 | – | – | **60** |
| race_trait | 2,712 | – | 241 | 1 | – | **2,954** |
| spell | 834 | 26 | 157 | 253 | – | **1,270** |
| **TOTAL** | **18,188** | **4,270** | **1,231** | **1,185** | **42** | **24,916** |

This table is the actual population. Every lane pile is a slice or union of it.

### 1.2 What each lane pile actually is, in these terms

| Lane pile (as filed) | Population | What it really covers |
|---|---:|---|
| class_feature not-started | 11,971 | class_feature's `not-started` slice ONLY — not its `held` (38) or `deferred` (40) |
| unmeasurable | 4,270 | **ALL** `unmeasurable` units, cutting across 5 kinds: class_feature 3,058 / feat 565 / equipment_modifier 416 / equipment 205 / spell 26 |
| race_trait | 2,954 | **ALL** of race_trait's not-done population (every verdict) |
| monster-companion | 2,258 | **ALL** of monster_ability (1,152) + companion (825) + monster (281) — every verdict in all three kinds |
| spell-feat-equipment | 4,380 | **ALL** of spell (1,270) + feat (1,151) + equipment_modifier (1,064) + equipment (895) — every verdict in all four kinds, **including their unmeasurable subsets** |
| sweeps | n/a | Not a unit-population pile — a structural/mechanism census (6 sweeps + one net-new defect measurement). Excluded from this sum; reconciled separately in §6. |

### 1.3 The overcount: unmeasurable ∩ spell-feat-equipment = 1,212 units, counted twice

The unmeasurable lane's 4,270 includes the unmeasurable subsets of spell/feat/equipment_modifier/
equipment (26+565+416+205 = 1,212). The spell-feat-equipment lane's 4,380 is the **entire**
not-done population of those same four kinds — which by construction also includes those same
1,212 unmeasurable units. Verified directly against both lanes' own printed sub-tables (feat
516+49=565 exactly; equipment_modifier 416 exactly; equipment 205 exactly; spell's 26-unit
"hand to unmeasurable lane" sub-group exactly). **These are the same 1,212 records, not
look-alikes** — both lanes filter `docs/work-inventory.json` by the identical `kind` + verdict
predicate.

### 1.4 The undercount: 295 units no lane examined at all

```
11,971 + 4,270 + 2,954 + 2,258 + 4,380 = 25,833
25,833 − 1,212 (dedup) = 24,621 distinct units covered
24,916 − 24,621 = 295 units covered by NO lane this wave
```

The 295 resolve exactly to:

| Uncovered group | Count | Why |
|---|---:|---|
| `class` kind, entire population | 157 | No lane was dispatched against `class` this wave. It carries forward wave 27's own census (§sweeps.md, the 157-classified-by-shape table) unchanged — not re-derived, not stale-checked. |
| `race` kind, entire population | 60 | **A genuine gap, not a carry-forward.** `race` (the race chassis itself — distinct from `race_trait`) was not named, referenced, or examined by any of the six lanes. No prior wave's census is on file for it either. |
| `class_feature` held | 38 | Falls between lane 1 (not-started only) and lane 2 (unmeasurable only). |
| `class_feature` deferred | 40 | Same gap. |
| **Total** | **295** | 157 + 60 + 38 + 40 = 295 ✓ |

**Reconciled: 24,621 (98.8%) examined by at least one lane, 1,212 examined redundantly by two
(now deduplicated in every table below), 295 (1.2%) examined by none.** The race-kind gap (60
units, a whole kind never once named) is the single most useful thing this reconciliation step
surfaced — worth a dedicated look next wave, sized here for the first time as its own line item.

---

## 2. The six piles, corrected

Every group below carries the adversarial-review-corrected figure where one exists; the lane's
original figure is shown struck through only where it changed. "SOUND" piles are reported as
filed. Full per-command reproduction lives in each `VISIBILITY-*.md`; only the corrected headline
numbers are restated here.

### 2.1 class_feature not-started — 11,971 — **verdict PARTIAL**

| Group | Count | Fix |
|---|---:|---|
| G1 option-pool records, no modelled owner | 3,347 | Split OPEN vs EXCLUSIVE per Ruling §18 first; 817 pool names, only 161 units (6 names) cross-checked against the 27-pool registry — 3,186 units/811 names never examined |
| G2 real owner, no explanation-id/pool match | 2,890 | Feeds F1 (below), blocked on the same gates |
| G3 no consumer function at all | 2,583 | 1,751 computed / 526 derived / 302 static / 4 ambiguous — real per-feature computation (hand function or Ruling §20's interpreter); the 554/882/1,690 regex-proxy split is explicitly **not verified**, do not plan against it |
| G4 no compiled rule set for book | 928 | adventurers_guide 699 / inner_sea_magic 218 / inner_sea_taverns 11 — book-onboarding lever, see §4 |
| G5 unmodelled corpus class | 2,194 | 80 distinct classes, gated behind L0/L1. **Correction:** sweeps.md's "18 real base classes with zero table" is **≥20**, not 18 — Magus, Vigilante, and Shifter all carry `class_absent_from_ClassId_ALL_and_book_class_id_enums`; only Magus was named |
| G6 near-miss (S9 candidate) | 29 | **Correction:** book split is `pathfinder_unchained 24 / core_rulebook 3 / advanced_players_guide 2`, **not** `advanced_class_guide 17 / pathfinder_unchained 7 / advanced_players_guide 3 / ultimate_wilderness 2` as filed. A follow-up S9 trace pointed at the original split would search two books holding none of it. |
| Cross-cutting F2 (feat bridge) | 1,378 | 431 units cheap-fixable outright (bridge feat_catalog's existing render path); **correction:** the group's title ("sole content is ABILITY:FEAT\|AUTOMATIC") is false for ≥475 of the 1,378 (463 carry a real local description, 12 carry no ABILITY token at all) — the 431/463 sub-figures themselves reproduced exactly |

**F1 (biggest_single_lever as filed) — DOWNGRADED, not a cheap lever.** Filed ceiling 3,536, filed
as "one classify()-side branch change." Corrected:

1. **Ceiling drops to ≤2,763.** 773 of the 3,536 (21.9%) carry a live `ENGINE_EFFECT_TOKEN_KEYS`
   token (ABILITY 657 / CSKILL 52 / AUTO 51 / SELECT 12 / ADD 1) — the exact refusal gate wave-22
   review forced into existence for this same class of mistake. Filed report never mentions this
   gate.
2. **The proposed fix cannot reach a player.** `apps/desktop/src/characterHub/classFeaturesModel.ts`
   builds every row by iterating `explanations`; a record with no explanation id (which is what
   defines G1/G2) produces no row at all, regardless of what `classify()` decides. A NEW,
   unscoped, uncosted frontend surface is required first.
3. **1,656 of the 3,536 are G1 option-pool members** whose OPEN/EXCLUSIVE classification (Ruling
   §18) is unresolved for 811 of 817 pool names. Blind widening is exactly what §18 forbids.

F1 is a real three-part scoping project (refusal gate + new render surface + per-pool ruling),
not a lever to schedule as filed.

### 2.2 unmeasurable — 4,270 — **verdict SOUND, no corrections**

Both reviews tried to break this one; neither could. Every number reproduced exactly, including
the two that required real work (a live cargo test + a full, non-sampled 136-row oracle sweep).

| Group | Count | Fix |
|---|---:|---|
| class_feature bucket A — matches an already-registered pool | 612 | **Wire `class_feature_pool_group_matches()` into `classify()`'s ClassFeature owner-resolution fallback.** Matcher, 27-entry table, false-positive guard all already exist and are already tested elsewhere in the same file. Zero regression risk (none of the 612 are currently `done`). **The cheapest confirmed unit-mover in the whole census — see §3.** |
| class_feature bucket B — real content, wrong owner (correctly excluded) | 53 | Per-group ownership work, no shortcut |
| class_feature bucket C — unrecognized group prefix (958 distinct groups) | 2,393 | Extend `CLASS_FEATURE_POOLS`; this wave's own 4-group spot check found 2 of 4 are NOT pool-shaped at all — bulk registration would introduce false positives |
| feat — real magnitude, no observed consumer | 516 | Posture/opponent/action simulation-coverage gap, not a description problem |
| **equipment_modifier — description-closure defect** | 136 declared | **111 of 136 (82%) confirmed via full oracle sweep to carry real SPROP:/DESC:/BENEFIT: text and are misclassified.** A real extraction bug, new to `defects.md`. |
| equipment_modifier — .COPY= alias rows | 280 | Needs a real alias-chain resolver; 22 of 24 spot-checked base rows confirmed real content (verified lower bound only) |
| equipment — genuinely description-free (stat-only) | 173/173 | Confirmed via full oracle sweep — render-surface gap (`EquipmentCatalogEntryDto` has no weight/damage/crit field), not an extraction bug |
| equipment — .FORGET / pfs_*.lst | 32 | 2 confirmed PCGen loader-retraction directives (ruling candidate); 30 Pathfinder-Society legality files (scoping question) |
| feat — placeholder marker / genuinely empty | 38 / 10 of 11 | Correctly refused; permanent |
| spell — genuinely empty | 26/26 | Confirmed, permanent for this printing |

### 2.3 race_trait — 2,954 — **verdict PARTIAL**

**G1 survives fully: 1,619 of 2,954 (55%) are not race-trait content at all.** Seven whole files
(b3/b2/ce/acg/cr/ma/pu `abilities_race.lst`) return **zero** race-trait-shaped rows from the
corpus discriminator while two control files return real counts (arg 236, isr 95). This is the
wave's single most consequential finding: no prior wave asked whether the class_feature-adjacent
denominator was measuring the right thing for over half its own not-done population.

The rest needed correcting — the filed partition did not sum to 2,954 because G3 mixed three
doneness states as if it were one not-done group, then G4 was derived by subtracting from it:

| Group | Filed | **Corrected** | Fix |
|---|---:|---:|---|
| G1 misclassified non-race content | 1,619 | 1,619 | TYPE-facet triage tool (see §4) |
| G2 CamelCase-glued race suffix | 159 | 159 | Widen the space-separated regex — trivial, additive |
| G3 real races out of ingest scope | ~~284~~ | **249** | 284 was a corpus-ROW count spanning done(25)/in-progress(10, already inside G6)/not-started(249); only 249 are actually not-done and outside G6. Per-race: Skinwalker 74 (not 75); **Rougarou (8 rows, 8/8 already done) and Samsaran (9 of 11 already done) should not be counted here at all.** |
| G4 undetermined remainder, race-not-modelled | ~~518~~ | **553** | Absorbs the G3 correction so the partition sums; same triage method, not yet run |
| G5 undetermined remainder, absent-from-race-traits | 130 | 130 | Same triage tool |
| G6 real magnitude, no compute seam | 234 | 234 | Continue L4 exactly as scoped, one race at a time |
| G7 universal-modifier size-trait shape | 8 | 8 | Extend `SIZE_ONLY_RACE_TRAIT_BUNDLE` — trivial |
| G8 loaded but never applies | 2 | 2 | Too small to generalize |
| **Sum** | 2,954 (did not add up) | **2,954 ✓** | |

**The lane's proposed new sweep is dangerous as stated and must NOT be scheduled that way.**
Widening `refine_kind()`'s monster-ability check from first-TYPE-segment-only to any-segment would
strip **539 already-done** race_trait units (confirmed: 2,371 rows corpus-wide carry a monster
facet in a later segment; joined to unit verdicts, 539 are `done`, 1,561 not-started, 233
in-progress, 1 held, 37 unmatched). A non-first SpecialQuality/SpecialAttack facet does not by
itself mean "this is a monster ability" — the real fix needs the fuller triage tool from §4, not a
one-line widening. (The lane's own filed figure of "1,253 not-first-facet rows" also does not
reproduce from its own stated predicate — 2,371 does; use 2,371, flagged as needing the triage
tool's finer classification before any code changes, not as a ready-to-ship count.)

### 2.4 monster / companion / monster_ability — 2,258 — **verdict PARTIAL**

monster_ability's not-started partition (879) now sums exactly after one correction:

| Group | Filed | **Corrected** | Fix |
|---|---:|---:|---|
| G1 native content, book not registered | 172 | 172 | Register 8 books in `MONSTER_BOOKS`; hand-transcribe |
| G2 Core Essentials glossary, no table | 190 | 190 | Architecture ruling needed (shared pseudo-table vs. per-book duplication) |
| G3a epic-tier creatures (bundles with `monster` kind) | 87 | 87 | Hand-build 19 stat blocks; closes 106 units total (19 monster + 87 monster_ability) |
| G3b template-shaped, no single stat block | ~~498~~ | **430** | 517 total absent_from units, 209 distinct parent names; exactly 19 exact-match a monster-kind unit (= G3a's 87); **517 − 87 = 430**, not the filed ~498 (which double-counted ~68 units via imprecise substring matching, e.g. "Cthulhu" hitting two different names) |
| **Sum (not-started)** | 947 (over by 68) | **879 ✓** | |

held (273): 197 derived-formula-blocked + 49 universally-refused-display + 27 unexamined
static/ambiguous remainder = 273 ✓ (unchanged, sums correctly as filed).

**L7 (template-application mechanism, biggest_single_lever) corrected: 430 + 49 (companion's own
template-shaped EMPTY-facet group) = 479, not the filed "≥547."** The filed number used the
uncorrected 498; the lane's own body text elsewhere calls 498 an upper bound while the summary
called 479's predecessor a floor — use 479 as the reproducible figure. Only 2 of the pile's 3
kinds were checked for this shape (monster itself was not), so 479 is legitimately a
**partial-coverage floor**, just not the specific inflated number filed.

companion (825): 310 Eidolon/SLA content (ruling needed, same shape as blocked.md B4/B5) + 130
archetype-chain rows (hand-transcribe) + 142 feat/trick sub-catalogs (need new tables) + 49
template EMPTY-facet + 43 Core Essentials glossary (same ruling as monster_ability's G2) ≈ 674 of
825; the remaining ~151 (a ~72-unit "other" type_facet remainder plus an 11-unit CompStatChoice
pool group not checked against Ruling §18) were **not individually characterized** — said plainly,
per the lane's own could_not_determine.

monster (281): 253 held (already correctly censused at wave-17's OPEN-ISSUES row 310, not
re-derived) + 28 not-started (19 bundle with G3a above, 9 standalone: Gug Savant, both Hydra
variants, both Iron Cobra variants, Herd Animal Storval Aurochs, Magma Ooze Poisonous, Kami
Shikigami).

### 2.5 spell / feat / equipment_modifier / equipment — 4,380 — **verdict PARTIAL**

Population and every per-kind evidence-code group reproduced **exactly** — this is the best census
in the wave on population arithmetic. It fails on exactly the one tool it told the operator to
build first.

**REJECTED — do not schedule: the `equipment_key_is_wired()` two-clause widen.** Filed as
`biggest_single_lever` and "BUILD FIRST," closing "at least 162" units at "zero new production
logic." Proven false by reading the two functions it names:

- The probe that would exercise the widen constructs its selection with
  `applied_modifiers: Vec::new()`. `resolve_intelligent_item_contribution()` iterates that slice and
  ends `found.then_some(total)` — with an empty slice, `found` is always `false`.
  **`intelligent_item.is_some()` can never be true under this probe.**
- `to_hit_bonus` is `is_weapon_record(record).then(...)` — `Some(0)` for weapon records, `None`
  for the 154 named Intelligent Item / Legendary Item modifier rows the lever was justified by
  (those rows carry no DAMAGE token, so `is_weapon_record` is false for them).

**The widen closes zero of the claimed 162 units, and the only records it CAN touch it would flip
on a fabricated literal zero — the exact anti-gaming shape Decision 1a forbids.** This is the
inverse of the wave's own target pattern: it looks like an S9 one-row fix and is not one.

Other corrections, both low materiality: spell's undiagnosed remainder is **284** units (not the
filed "~55" — 722 total minus 329 occult_adventures minus 109 bestiary-attributed = 284, spread
across 9+ books); book-onboarding reach within this lane's own 4 kinds is closer to 424 (its own
groups sum to 437) than the filed 373 — the corpus-wide, all-kind 1,372 figure is correct and
unaffected.

**Standing findings that hold up, unaffected by either correction:**

- **B6 spell gap (confirmed by both reviews independently).** `derived_evaluator_fixture_check.rs`
  has zero references to B6/bestiary_6, while `spell_resolver.rs` defines `SPELL_BOOK_B6`. Dormant,
  zero live impact today — but the coverage test meant to catch this recurring hardcodes its own
  comparison list instead of deriving from `spell_resolver::SPELL_BOOK_*`, so it cannot catch the
  *next* occurrence either. New `defects.md` candidate.
- Mythic Adventures feat-catalog key registration — 353 units, one mechanism, book already has 358
  ingested non-.MOD records to hang it on.
- equipment_modifier's 504 `VISIBLE:NO` units (47% of its not-done population, essentially unique
  to this kind) — new `blocked.md`-shaped ruling question, parallel to B4/B5.
- .COPY=-alias description-inheritance gap, up to 861 units across equipment + equipment_modifier
  (reported by the lane, not independently re-verified by either review at this specific figure —
  treat as lane-reported pending confirmation).

### 2.6 sweeps — structural census, not a unit-population pile — **verdict PARTIAL, two closures reverted**

See §6 for the full todo/ reconciliation. Headline correction: the lane's own `todo/sweeps.md`
edit (this session did not carry forward, see the merge note in §7) marked S9 and S2 **CLOSED**;
both closures are premature and are reverted to **PARTIAL** below.

---

## 3. Ranked recommendation — the next three things, by units-closed-per-unit-of-effort

Ranked by ROI, not size. The units-closed number is only trustworthy where a reviewer confirmed it.

### #1 — Fix the self-erasing fixture generator, BEFORE anything else touches it (protective, ~1 hour)

`scripts/derive_derived_evaluator_fixtures.py` destroys its own committed fixture on its very first
run from the committed state: **2,109 fixture entries across 8 families** (not the originally-filed
"11 equipment units") — confirmed by literally running it twice in an isolated worktree. The
one-line `HELD_STATUSES` fix restores only 11 of the fixture's own 94 entries; the other 2,015
losses (spell_entries, companion_entries, class_feature_description_entries, etc.) come from a
separate cause (a hardcoded sibling-key allowlist) and need root-causing beyond the one-line patch.

**This does not close a single unit by itself, and it is still #1.** Ruling §20 just authorised the
formula interpreter, whose correctness gate IS `derived_evaluator_fixture_check` — the exact seam
this bug lives in. Scaling that seam by orders of magnitude, as §20 explicitly calls for, without
first closing this hole means the next large interpreter-fixture regen can silently zero out
thousands of already-banked units in one run, with the program's own test suite reporting green
(the filed "twice-run-diff" test cannot catch it — proven by running it twice and getting a clean
diff on the very defect it was built to catch). Build the corrected check instead: run the
generator **once** against the committed fixture and assert, per top-level `*_entries` key,
`len(after) >= len(before)`. Cheap, and it goes red immediately on today's bug.

### #2 — Wire `class_feature_pool_group_matches()` into `classify()`'s owner-resolution fallback (612 units, near-zero cost)

Confirmed SOUND by adversarial review (cargo test re-run + independent group→unit join). The
matcher, its 27-entry pool table, and its false-positive guard already exist and are already used
elsewhere in the same file for the wiring probe — this is a wiring change, not new construction.
612 units move from `unmeasurable` to a real, honest verdict (mostly `not-started`, per the lane's
own characterization) with **zero regression risk**, since none of the 612 are currently `done`.
Cheapest confirmed unit-mover in the entire census.

### #3 — Book onboarding for the 4-book cluster: adventurers_guide, inner_sea_magic, inner_sea_temples, inner_sea_taverns (2,300+ units, moderate & bounded cost)

The single largest corpus-wide-reaching lever this wave measured, and it recurs across **five
different kinds independently**, which no single lane could see on its own:

```
class_feature G4:  adventurers_guide 699 + inner_sea_magic 218 + inner_sea_taverns 11  =  928
spell/feat/equipment/equipment_modifier (lane 5, same 3 books + inner_sea_taverns overlap) = 1,372
                                                                          non-overlapping total ≥ 2,300
```

Already a named, calibrated lever (`docs/release/.../e13-book-ingest-cost-calibration` memory: fixed
cost dominates at ~1.5–2h/book, content is nearly free once ~7 count-pinning files exist per book).
Not cheap in absolute terms, but the best return of any multi-hundred-unit item this wave found,
and unlike #1/#2 it actually moves the board once done, not just re-verdicts units.

**Runner-up, not in the top 3 only because it is smaller:** Mythic Adventures feat-catalog key
registration (353 units, one mechanism, book already ingested) — do it in the same cycle as #3 if
capacity allows; it is cheaper than #3 per unit and does not compete for the same files.

---

## 4. Tools worth building, across the whole box

Individual lanes each proposed a tool for their own pile. Looking across all six:

### Worth building — serves 3+ piles

**A TYPE-facet / group-name corpus-row triage tool.** This is the SAME underlying need in three
different piles, proposed independently and never connected:

- class_feature not-started G1 (817 unclassified pool names, 3,186 units)
- unmeasurable bucket C (958 unrecognized group prefixes, 2,393 units)
- race_trait G1/G4/G5 (proven on 7 files at 1,619 units; ~30 files, 683 units, not yet run)

All three need the identical operation: read every dot-segment of a corpus row's TYPE/group-prefix
chain (not just the first), classify it against known catalogs, and route non-matches to a named
bucket instead of leaving them ambiguously typed. `scripts/classify_race_trait_rows.py`'s `classify()`
predicate is the closest existing implementation and is proven on 7 files. **Extend it once,
generalize its destination-bucket logic, and it directly answers up to ~7,200 units' worth of
"what is this, really" question across three piles** — none of which by itself would justify
building it, together they clearly do. Estimated at about half a day per the original lane's
sizing, unchanged by review.

**The corrected S6 fixture regression check** (§3, item #1) — small, but its reach is not one
pile, it is every kind the interpreter (Ruling §20) will eventually touch, i.e. the whole program
going forward. This is infrastructure, not content, and the two adversarial reviews' independent
confirmation that the ORIGINAL proposed tool cannot catch its own motivating bug makes the
corrected version urgent rather than optional.

### Rejected — do not build

- **`equipment_key_is_wired()` two-clause widen.** Closes 0, risks fabricating a computed zero.
  See §2.5.
- **The twice-run-diff pytest as originally specified.** Provably cannot detect the S6 bug it was
  proposed for (destruction happens on run 1 from committed state, stable after — a two-run diff is
  clean). Build the corrected once-run-vs-committed check instead (already folded into §3 #1).

### Named but not costed this wave — real, not urgent

- GROUP_PREFIX→CLASS remap table for S8/D5 (2,682-unit reach) — real and tool-shaped, but needs
  validation against the upstream PCGen `.lst` granting-class text, not against the same key text
  the mapping itself is built from (review's caveat).
- The equipment_modifier SPROP-extraction fix (111+ confirmed misclassified units) — a genuine bug
  worth its own cycle, but root-causing it means reading the closure-construction code path, which
  a measurement-only wave correctly declined to do.

---

## 5. Expensive but unavoidable

- **L0 — prestige-class entry-requirement gating.** Does not exist anywhere in the codebase. Gates
  77 of the 157 `class` units (re-confirmed, not re-derived, this wave — `class` itself was one of
  the two uncovered kinds, §1.4). No shortcut; a real gating mechanism has to be built.
- **The formula interpreter itself (Ruling §20).** Authorised, and it is the real fix behind G3
  (2,583 class_feature units), monster_ability's 246-of-273 held units, and a large share of
  spell/equipment's held/in-progress buckets. Linear-cost hand-modelling was accepted for years
  specifically to avoid this; now that it is authorised, it remains genuinely expensive per unit —
  just less expensive than the alternative at this remaining scale. Gate #1 (§3) is a **precondition**
  for spending this cost safely, not an alternative to it.
- **L7 — template-application mechanism.** ≥479 units (monster_ability + companion alone, 2 of 3
  kinds checked) need representing a PF1e creature/familiar TEMPLATE, which the current
  one-stat-block-per-creature chassis structurally cannot do. A genuinely new engine mechanism, not
  a data-transcription task.
- **G1's 817 pool names (class_feature) / 958 group prefixes (unmeasurable bucket C).** The triage
  tool (§4) only classifies; each large group still needs individual oracle verification before
  registration (this wave's own 4-group spot check found 2 of 4 large groups are NOT pool-shaped).
  Real, unavoidable, per-group hand-work — the tool reduces the search space, not the verification
  cost.

## 6. Probably not worth doing

- **G3's regex-proxy formula-shape split (554 likely-refused / 882 likely-readable / 1,690
  unclassified) as a planning input.** The lane itself flags it as unverified; it is a substring
  scan, not a dry run of the real tokenizer. Do not size interpreter work against it — re-run
  against the real parser once it exists instead.
- **Bulk-registering class_feature/race_trait pool or group names without per-group oracle
  verification.** Both piles found real false positives on small spot checks (2 of 4 large
  race_trait/bucket-C groups this wave alone). A bulk tool that skips verification will bank wrong
  content faster than a human would notice.
- **The race_trait "any-segment monster-facet" reclassification, as a blanket rule.** Would strip
  539 already-done units (§2.3). Only worth revisiting once the full triage tool (§4) can
  distinguish real monster-ability rows from co-facet false positives.
- **equipment_key_is_wired() widen** — covered in §2.5/§4; actively harmful, not merely low-value.

---

## 7. Needs operator ruling (consolidated across all piles)

New this wave:

1. **Do the ~310 companion-kind Eidolon evolution/SLA-upgrade units belong under the companion
   doneness gate at all**, or are they class_feature-shaped content misfiled (same question shape
   as `blocked.md` B4/B5)?
2. **Core Essentials shared ability-glossary files** (`ce_abilities_race.lst` for monster_ability,
   `ce_*_familiar*.lst` for companion — 233 units across 2 kinds) were re-attributed by book label
   under Ruling §16 but never given a table. Shared pseudo-table, or manual per-book duplication?
3. **Do equipment_modifier's 504 `VISIBLE:NO` units** (47% of its not-done population, essentially
   unique to this kind) belong under the doneness gate at all — structurally parallel to `blocked.md`
   B4/B5?
4. **Are equipment `.FORGET`-suffixed rows** (confirmed PCGen loader-retraction directives, 2 units)
   a Structural Exclusion Register entry, same disposition as core_essentials residuals (§16)?
5. **Is `pfs_*.lst`** (Pathfinder Society legality cross-reference, 30 equipment units) in-corpus
   scope at all?
6. **Should ABP** (Automatic Bonus Progression, 12 class_feature units) be reclassified out of
   `kind=class_feature` — it is a whole-table optional variant rule owned by no class.
7. **Should feat-owned content wrongly modelled as class-owned option pools** (e.g. "First Boon"
   demon-lord Obedience boons, 10 units) get a distinct classification mechanism?

Still open, re-raised, not newly answered:

- `blocked.md` B1 (`mod_only_rescue`, 249 units) — new evidence added (36 of the 249 now duplicate
  content Ruling §16 already deleted), core question unchanged.
- `blocked.md` B4/B5 (48 + 5 units, structurally-non-PC-class question) — the new companion/
  equipment_modifier ruling asks (above) are the same question shape recurring in two more kinds;
  sequence S8's closure behind these, not before.

---

## 8. Wave-28 receipt (for `progress.md`)

Six lanes, plus two independent adversarial-review lanes, ran corpus-wide census work across every
kind except `class` and `race` (the two uncovered kinds identified in §1.4). Banked zero units by
design. All six lane deliverables merged as documents only:

```
git log --oneline e90ba9ec1..HEAD -- docs/release/SD-31-corpus-closure-grind/artifacts/
5bdfb846f docs(sd31): W28 visibility census — 5 lane deliverables merged (no banking)
e61adbb6d docs(sd31): W28 visibility census — sweeps lane deliverable (no banking)
```

`docs/work-inventory.json` confirmed byte-identical (`de0dfa8614efdd027316ccf274ad8490`) before and
after every merge. Board: **13,456/38,372 (35.07%), unchanged.**

**Reconciliation found:** 24,621 of 24,916 not-done units (98.8%) examined by at least one lane;
1,212 units examined by two lanes redundantly (now deduplicated, §1.3); 295 units (`class` 157,
`race` 60, `class_feature` held+deferred 78) examined by none (§1.4) — most notably `race`, a whole
kind no lane touched or even named.

**Two adversarial reviews found:** 3 of 6 lanes SOUND-or-PARTIAL-with-real-value, 3 of 6
PARTIAL-with-a-load-bearing-error; zero lanes GAMED. The most consequential error (§2.5) was a
tool recommendation that would have closed zero units while fabricating computed zeros on the
records it touched — caught before scheduling, not after building.

Full detail in §§1–7 above; per-lane detail in `artifacts/VISIBILITY-*.md` (6 files).
