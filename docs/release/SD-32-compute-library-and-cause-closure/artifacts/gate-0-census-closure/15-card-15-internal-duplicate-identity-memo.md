---
canonical: true
owner: card15-internal
status: forensic finding — pins the cause of the `class_feature` 179-unit residual
date: 2026-08-23
---

# Card 15 — pinning the cause of the `class_feature` residual (`decisions.md §12b`)

`15-card-15-class-feature-memo.md` §3 found 179 real `class_feature` rows the inventory does not
track, and explicitly left the cause unpinned: *"Root cause not fully pinned within this cycle's
budget — circumstantial evidence... points at a pool-membership de-duplication step... but this is
named as a hypothesis, not a proven mechanism."* The dispatch brief for this cycle required the
cause be pinned before any rescue is attempted. It is pinned here, and it is **not**
`is_internal_category`.

**Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6`.

## 0. Answer, up front

**The dominant, provable cause is `v06_work_inventory.rs`'s corpus-wide `duplicate_identity`
(kind, key) collapse (the `--- id uniqueness ---` / `duplicate_identity` pass, lines ~9582-9604),
not `class_feature_pool_catalog` and not `is_internal_category`.** Re-derived against the pinned
oracle (`15-card-15-class-feature-residual-cause-pin.py`, this directory):

Re-derived **before** this cycle's `is_internal_category` fix landed (matching the population
`15-card-15-class-feature-memo.md` §3 originally measured):

```
179 non-internal class_feature residual rows
  158 (88.3%) share their computed (book, key) — the SAME identity `duplicate_identity` keys on —
       with at least one OTHER class_feature row in the same book
   21 (11.7%) do NOT collide on key — cause still unpinned for this smaller residual
```

(§3 below re-runs the same script **after** this cycle's fix lands and finds 180 — one more, by the
SAME mechanism, demonstrated live.)

Worked example, `advanced_class_guide/acg_abilities_class.lst` (re-derived by direct corpus read,
not the classify script alone): **FOUR** physically distinct rows all display as `Aberrant
Bloodline` (lines 156, 566, 2412, 2754), one per class that gets a bloodline of that name
(Arcanist/Bloodrager/Blood Arcanist/Crossblooded Rager) — each with its own distinct `CATEGORY:`
and `TYPE:` fields and genuinely different mechanical content. None of the three at 566/2412/2754
carries a `KEY:` field (line 156 does, `KEY:...` differs — not part of this collision), so all three
compute `key == "Aberrant Bloodline"` (the bare display name) under `v06_work_inventory.rs`'s own
key rule (`token_value(fields, "KEY:").unwrap_or_else(display_name)`) — the SAME rule
`duplicate_identity`'s `seen.insert((u.kind, u.key.clone()))` dedups on. `docs/work-inventory.json`
carries exactly ONE of the four (`advanced_class_guide:class_feature:aberrant_bloodline`,
`source_line: 156`, the first in file-iteration order); the other three are the ones this cycle's
own `duplicate_identity` counting shows as dropped.

## 1. Why this rules out `is_internal_category`

None of the 179/180 residual rows carry `CATEGORY:Internal` — they were excluded from that
possibility **by construction** before the residual was computed (both this memo's re-derive and
`15-card-15-class-feature-memo.md` §3's original script filter internal rows out first). This
cycle's `is_internal_category` fix (§2 below) only ever ADDS previously-invisible content; it cannot
be the cause of a population that was never internal-tagged in the first place. The two fixes are
independent, as the dispatch brief anticipated.

## 2. Why this rules out (or at least does not require) `class_feature_pool_catalog`

`class_feature_pool_catalog` (`src/rules_core/class_feature_pool_catalog.rs`, consulted from
`v06_work_inventory.rs` at the `wiring_class`/reachability-verdict layer, lines ~7693-7918) governs
whether an ALREADY-enumerated unit's `wiring_class` verdict credits a rendered pool description. It
runs downstream of enumeration and has no code path that removes a unit from `out.units` or from the
final inventory. It cannot be the mechanism that makes a row absent from
`docs/work-inventory.json` in the first place — the absence happens earlier, at `duplicate_identity`
(or, for a smaller number, somewhere still unpinned). The original memo's hypothesis is not
disproven in general (it may still explain something about *why* a pool-membership row's
`wiring_class` looks a certain way once tracked), but it is not the cause of non-enumeration.

## 3. The residual moved 179 → 180, and the mechanism is now doubly demonstrated

Landing this cycle's `is_internal_category` fix (§`15-internal_cycle_receipt.md`) makes MORE content
visible in the SAME `duplicate_identity` competition — including `Kind::ClassFeature` rows that
were previously trapped by the blanket `CATEGORY:Internal` exclusion and never even reached the
dedup pass. One of those newly-visible rows won a key collision against a row that was PREVIOUSLY
the sole occupant of that identity and therefore tracked:

```
ultimate_psionics/up_abilities_class.lst
  line 186: "Disable Device Class Skill"  CATEGORY:Internal   CSKILL:Disable Device
  line 468: "Disable Device Class Skill"  CATEGORY:Special Ability  TYPE:...  CSKILL:Disable Device
```

Neither row carries a `KEY:` field, so both compute `key == "Disable Device Class Skill"`. Before
this cycle, line 186 was trapped by `is_internal_category` (blanket-dropped) and line 468 was the
sole survivor, tracked at `docs/work-inventory.json`'s
`ultimate_psionics:class_feature:disable_device_class_skill`. After this cycle's fix, line 186 is
disposition (A) (`CSKILL:` is on the content list) and now competes for the SAME identity — it comes
first in file order, so `duplicate_identity` now keeps line 186 and drops line 468. The unit's `id`
is stable (still `ultimate_psionics:class_feature:disable_device_class_skill`); only its
`source_line`/`type_facet` provenance moved. **No content is lost — this is one physical corpus
location swapping for another describing the same conceptual feature** — but it is exactly the
`duplicate_identity` mechanism §0 names, demonstrated live by this cycle's own fix, not merely
inferred from the pre-existing 158/179 collisions.

Re-derived count after this cycle's fix (`15-card-15-class-feature-residual-cause-pin.py`, same
script, run against the post-fix `docs/work-inventory.json`): **180** non-internal residual rows
(179 + 1 — the displaced `Disable Device Class Skill` row moved from "matched" to "residual" by the
SAME mechanism), of which **158 still collide on key** with another *non-internal* row in the same
book — the identical 158, confirming the fix did not touch that population's own cause. The
`Disable Device Class Skill` row itself lands in the script's "unexplained" bucket (22, up from 21),
**not** the 158 — a known, documented blind spot in `15-card-15-class-feature-residual-cause-pin.py`'s
collision check, which only tests non-internal-vs-non-internal collisions and does not see a
collision against a newly-internal-turned-content row. The Disable Device case is confirmed by
direct corpus grep and inventory diff above, not by the script — flagged so the script's own 22
figure is not read as "cause fully unpinned" for that specific row.

## 4. What this cycle does and does not do about it

**Does:** pins the cause (§0-§3), with a committed, re-runnable script and worked examples in both
directions (the Aberrant Bloodline case — pre-existing, four genuinely distinct records collapsed
to one; and Disable Device Class Skill — newly demonstrated, a legitimate duplicate declaration in
the corpus itself, same conceptual content declared twice under different `CATEGORY:` tags).

**Does not:** rescue the 158 (or the 180). `duplicate_identity`'s (kind, key) collapse is a
corpus-wide, load-bearing mechanism this bundle relies on everywhere (it is what keeps a
continuation/restatement from double-counting) — the Aberrant Bloodline case shows it INCORRECTLY
merging four genuinely distinct records that happen to share a bare display name (no per-class
disambiguator in the key), while the "Touch of Good" case `15-card-15-class-feature-memo.md` §6
already flagged shows the SAME mechanism CORRECTLY merging two byte-identical duplicate
declarations. Distinguishing these two shapes needs a real per-row test (does the colliding sibling
carry genuinely different mechanical content, or is it a byte-identical restatement?), which is
exactly the kind of "rescue blindly" this cycle's dispatch brief flagged as high-risk and out of
scope. **Escalated as the concrete next-cycle target, not folded into this cycle's `is_internal_category`
fix and not left as a vague hypothesis** — the fix, when written, is a per-book-collision content
comparison at the `duplicate_identity` pass itself (not at `is_internal_category`, `refine_kind`, or
`has_classifying_token`, none of which run at the right layer to see the OTHER colliding row).

## 5. The unexplained 22 (one of which — Disable Device Class Skill — is explained above by hand)

22 of the 180 post-fix residual rows do not collide on `(book, key)` with any OTHER *non-internal*
`class_feature` row in the currently-enumerated population, per the script's own (documented,
narrower-than-ideal) check. One of the 22, `Disable Device Class Skill`, is fully explained by hand
in §3 (it collides with a newly-internal-turned-content row the script's collision check does not
test against) — the other 21 (the same 21 the pre-fix run already found) are genuinely still
unpinned — flagged here, not rounded into the 158 or silently dropped. A plausible next check (not
performed this cycle, out of budget): whether they collide with a row of a DIFFERENT `Kind::` in the
same book under
`v06_work_inventory.rs`'s per-kind `slug_population`/`unit_id` mechanism, or a cross-book
`RACE_TRUE_BOOK`-style `core_essentials` resolution — neither checked here.

## 6. Re-derive command

```bash
export PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data
python3 docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-class-feature-residual-cause-pin.py
```
