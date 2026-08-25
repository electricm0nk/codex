---
canonical: true
owner: card-15-category-internal-adjudication
status: forensic cycle deliverable — settles decisions.md §14c item 4
date: 2026-08-22
---

# Card 15 — adjudicating the contested 2,614 `CATEGORY:Internal` rows (`decisions.md §14c` item 4)

**Scope:** the tension `decisions.md §14c` item 4 named as unresolved between card 15's two
sibling lanes over the 2,614 `CATEGORY:Internal` rows in `_abilities_class.lst` files. This memo
settles it by class, with a committed classifier and a code fix in `scripts/census_independent.py`.
Does **not** touch `docs/work-inventory.json` or its producer (enumeration lane's scope) or
`scripts/card15_reconcile.py`/`15-reconcile.json` (flagged stale below, not fixed here — out of
this cycle's granted scope).

**Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`;
`scripts/verify.sh --only preflight-oracle` → PASS at cycle start).

## 0. Answer, up front

**The class_feature lane's blanket-(B) verdict was wrong for 90.7% of the population. The 2,614
splits: 2,371 (A) / 243 (B) — proven per-row, by class, not by the file-kind rule the class_feature
lane relied on.**

```
2,614 CATEGORY:Internal rows (_abilities_class.lst files)
  = 2,371  (A) IS an object — real content, or a gateway proven to resolve to one
  +   243  (B) NOT an object — 203 proven facets (gateway resolves to a real, already-counted
             target) + 40 proven inert (zero content field, zero gateway token)
  --------
  = 2,614
```

Both lanes' own reasoning was sound method, applied to the wrong test. The class_feature lane
correctly identified that `census_independent.py` already excludes `CATEGORY:Internal` rows on
*other* `_abilities_*.lst` files — but that exclusion has always been **per-row**, testing each
row's own content (`_row_category_tag` + the ability_category lane's own classifier), never a
blanket file-kind exclusion. Applying it as a blanket rule to `_abilities_class.lst` — and citing
two hand-picked examples ("Damage Reduction ~ All/Silver") that were themselves misclassified by a
too-narrow content test — reproduced the exact defect `decisions.md §14a` reopened Gate 3 for: a
rule proven correct on a document row cannot be applied to a population it was never tested against.

## 1. Re-deriving the 2,614

Independently re-walked (bucket == `row_dependent_class_feature`, `cat.upper() == "INTERNAL"`),
not trusting `census_independent.py`'s own label:

```bash
export PCGEN_CORPUS_ROOT="$(git rev-parse --show-toplevel)/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data"
python3 docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-category-internal-classify.py \
  --repo-root . \
  --corpus-root "$PCGEN_CORPUS_ROOT" \
  --inventory docs/work-inventory.json \
  --ability-category-rows docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-ability-category-rows.jsonl \
  --diff-json docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/diff.json \
  --output-jsonl docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-category-internal-rows.jsonl \
  --output-summary-md docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-category-internal-summary.md
```

→ `rows: 2614 (expected 2614)` / `self-check: MATCH` (the script fails closed — non-zero exit if
the independent re-walk's count disagrees with the population `decisions.md §14c` names). **No
difference from the 2,614 figure.** Full per-row output and stats:
`15-card-15-category-internal-rows.jsonl` / `-summary.md` (this directory).

## 2. Testing the (B) claim on its own terms — the per-row classifier

Reused the sibling `ability_category` lane's own method (`15-card-15-ability-category-classify.py`)
rather than inventing a new one: a row is disposition (A) if it carries independent content of its
own, (B) if it is a proven wrapper/gateway to something already real, or (B) if it is a proven bare
picklist entry with zero payload.

### 2a. The content test had to be widened twice, in both directions the AGENTS.md concurrency rule warns about

The class_feature memo's own worked (B) examples were **"Damage Reduction ~ All"** and
**"Damage Reduction ~ Silver"** — `CATEGORY:Internal`, `DR:ClassFeatureDR_ALL/-` — disposed (B)
because "neither carries a `DEFINE:`/`BONUS:` formula token a player-facing feature would." That
test is `shape_ledger`'s own formula-extraction field list, not a content test — `DR:` **is** real
mechanical content: it names the class-feature-specific damage-reduction variable the engine's DR
machinery reads. Both examples flip to (A) under the ability_category lane's own broader test:

```bash
python3 -c "
import json
rows = [json.loads(l) for l in open('docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-category-internal-rows.jsonl')]
for r in rows:
    if 'Damage Reduction' in r['identity']:
        print(r['identity'], r['disposition'])
"
# Damage Reduction ~ All    A
# Damage Reduction ~ Silver A
```

Starting from the ability_category lane's own 14-field content list (`DEFINE:`/`BONUS*:`/`DESC:`/
`ASPECT:`/`CSKILL:`/`MOVE:`/`AUTO:`/`TEMPLATE:`/`SPROP:`/`QUALITY:`/`SR:`/`DR:`/`SAB:`/`VISION:`),
a first pass found 1,034/2,614 (39.6%) content-bearing — clearly too narrow given the DR: finding
above already contradicts it being exhaustive. **Doing exactly what AGENTS.md's concurrency rule
warns against** ("a grep filtered to BONUS/PRE hides STACK/MULT and other application-governing
fields"), a whole-record field inventory over all 2,614 rows (not a filtered grep) surfaced real
mechanical fields the 14-field list missed: `SPELLKNOWN*:` (1,185 rows), `TEMPBONUS:` (70),
`CHOOSE:` (116, a real chooser), `NATURALATTACKS:` (15), `COMPANIONLIST:` (25), `ADD:` (8),
`FOLLOWERS:` (18), `UDAM:`/`UMULT:` (113 each — literal unarmed-damage values), `SELECT:`,
`COST:`, `MOVECLONE:`, `SPELLS:`, `SERVESAS:`, `DEFINESTAT:`, `UNENCUMBEREDMOVE:`, `BENEFIT:` (157
— narrative benefit prose, same class as `DESC:`), and `SPELLLEVEL:` (95 — a class-level-to-spell
mapping, e.g. `Summon Nature's Ally Spell List`'s per-level spell table). Widening the content test
to this full field set moved 1,034 → 2,219 → **2,369** content-bearing rows across three
iterations, each one re-inspecting what remained to confirm the newly-excluded rows were still
genuinely bare rather than stopping at a round number. Full field inventory and the three-test
comparison table: `15-card-15-category-internal-summary.md` §"Content-test comparison."

### 2b. The parent-resolution test (`decisions.md §12b`'s proof-of-(B) requirement)

A row with no content of its own but an `ABILITY:...|AUTOMATIC|<target>` gateway token is only
proven (B) if `<target>` resolves to something already real. Resolution universe: the ten tracked
kinds (own KEY:-or-identity, per PCGen's own reference convention — most feats carry no explicit
`KEY:` field at all, e.g. `cr_feats.lst`'s "Double Slice", and are referenced by bare identity),
`docs/work-inventory.json`'s already-tracked `class_feature` units, the ability_category lane's own
5,108 disposition-A rows, **and** sibling rows within this same 2,614-row population that
themselves carry independent content (e.g. "Brawler Unarmed Damage LVL 1" gateways to 9 sibling
per-size rows, each its own `BONUS:VAR`-bearing row in the same file — resolving only against the
*external* population would have wrongly called this unresolved).

```bash
# gateway resolution, from the summary:
#   rows with an ABILITY:...|AUTOMATIC|<target> token: 437
#   of those, target resolves to an already-counted unit: 382 (after within-population resolution)
#   of those, target does NOT resolve: 55 -- kept disposed (A), not excluded (burden of proof is on B)
```

The 55 unresolved-target rows are not proof of orphaned content — inspection found 17 are `%LIST`
targets (PCGen's own runtime placeholder for a dynamically-chosen target, not a static reference)
and several more (22 "`<Element> Domain`" rows) target the census's separate `domain` kind-
unenumerable bucket (`decisions.md §12b`'s "everything else," 183 units), which this classifier's
resolution universe does not yet include — real content, just outside this cycle's resolution
scope, not proof of inertness. Per `decisions.md §12b`'s burden of proof (a (B) must be *proven*,
not assumed), all 55 remain disposed (A) rather than being excluded on an unproven claim.

### 2c. Final disposition

| disposition | count | proof |
|---|---:|---|
| A (content-bearing) | 2,369 | carries ≥1 of the full field list above |
| A (unresolved-gateway) | 2 | gateway token, target not resolvable by this cycle's universe — not proven (B) |
| B-gateway-resolved | 203 | gateway token, target proven to resolve to an already-real, already-counted unit |
| B-picklist | 40 | zero content field, zero gateway token — genuinely bare (`CATEGORY:`/`KEY:`/`TYPE:`/`VISIBLE:`/`SOURCEPAGE:` only) |
| B-duplicate | 0 | (no exact-`KEY:` collision with a tracked kind found) |
| **A total** | **2,371 (90.7%)** | |
| **B total** | **243 (9.3%)** | |

The 40 `B-picklist` rows are inspected in full in `15-card-15-category-internal-summary.md`'s
disposition table and are uniformly the same shape the class_feature memo described — bare
`"<Name> Tracker"` / `"<Name> Bloodline ~ Feat Tracker"` / `"<Name> Qualifier"` rows with no field
beyond structural markers. This narrow class is the genuine (B) population; it is not 2,614 rows.

## 3. Confirming/refuting the Opus verifier's figures (`decisions.md §14c` item 4)

**910-unresolved figure — refuted, and traced to its likely method.** A literal re-derive against
`corpus_key` (the field `docs/work-inventory.json` actually uses — bare identity, not `KEY:`) or
against `KEY:` fields both produce different numbers (0 matched via `KEY:`-to-`corpus_key` join, no
overlap at all, since `corpus_key` is populated from bare identity; 1,388/2,614 matched via a naive
bare-identity join, but that reintroduces the "shared-name hazard" the ability_category lane's own
classifier explicitly refuses to rely on — a shared display string is not proof of a shared object).
Neither reproduces 910. This cycle's own KEY:-scoped, gateway-target-specific resolution test (§2b)
found **55** unresolved after within-population resolution was added, not 910 — the verifier's
figure most likely predates the within-population resolution step (its own worked example,
`KEY:Bloodrager Bloodline Feat ~ Alertness` → `ABILITY:FEAT|AUTOMATIC|Alertness`, is exactly the
external-tracked-kind case this cycle's classifier *does* resolve, confirming the method is
directionally the same). Reported honestly as unreconciled rather than silently adjusted to match.

**2,420/2,614 (92.6%) "carry independent mechanical content" — directionally confirmed, and the
per-token breakdown reconciles exactly once the counting convention is identified.** The verifier's
six per-token counts (`SPELLKNOWN:` 1,185, `DEFINE:` 151, `TEMPBONUS:` 70, `AUTO:` 38) reproduce
**exactly** against this cycle's own row-presence counts for the same rows:

```bash
python3 -c "
import json, re
rows = [json.loads(l) for l in open('docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-category-internal-rows.jsonl')]
for tok in ('SPELLKNOWN[A-Z]*', 'DEFINE', 'TEMPBONUS', 'AUTO'):
    print(tok, sum(1 for r in rows if re.search(tok + ':', r['line'])))
"
# SPELLKNOWN[A-Z]* 1185   DEFINE 151   TEMPBONUS 70   AUTO 38   -- exact match
```

`BONUS:` (verifier 675, this cycle's row-presence 605) and `ABILITY:` (verifier 512, row-presence
437) do **not** match at row-presence, but reproduce **exactly** as a plain substring search with no
word-boundary:

```bash
python3 -c "
import json
rows = [json.loads(l) for l in open('docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-category-internal-rows.jsonl')]
print('BONUS: substring', sum(1 for r in rows if 'BONUS:' in r['line']))
print('ABILITY: substring', sum(1 for r in rows if 'ABILITY:' in r['line']))
"
# BONUS: substring 675    ABILITY: substring 512   -- exact match
```

**Traced cause:** a substring search for `"BONUS:"` also matches `"TEMPBONUS:"` (no word boundary
before `B`), and a substring search for `"ABILITY:"` also matches `"PREABILITY:"` (a prerequisite
*gate*, not independent content — 118 rows carry `PREABILITY:` per the full field inventory, and
`675 - 605 = 70` is exactly the `TEMPBONUS:` row count found above). **Correction filed:** the
verifier's `BONUS:`/`ABILITY:` counts overstate two token families by conflating them with
`TEMPBONUS:` and `PREABILITY:` respectively — a token-presence bookkeeping artifact, not a wrong
conclusion (both `TEMPBONUS:` and genuine `ABILITY:...|AUTOMATIC|` gateways are still real evidence
for (A) or a provable (B), just not evidence *of the specific token family named*). This cycle's own
disposition-of-record (§2c) does not depend on this distinction — it tests the full field list
directly, not per-token labels — so the 92.6%-vs-90.7% headline gap is explained entirely by test
scope (six tokens vs. the full field list) and is not a disagreement about which rows are real.

## 4. Reconciling against the sibling ability_category lane's 81.6% (685/839)

**Different, disjoint populations — no overlap, not a contradiction to resolve to one number.** The
839 is the *original* `ability_category:Internal` bucket: `CATEGORY:Internal` rows in **bare**
`*abilities*.lst`/`*abilities_other.lst` files (`_classify_kind_by_filename`'s `row_dependent`
branch — files that are neither `_class` nor `_race`). The 2,614 is `CATEGORY:Internal` rows in
**`_abilities_class.lst`** files specifically (`row_dependent_class_feature` branch) — a completely
different file set, walked by a different branch of the same classifier, holding structurally
different content (per-class bloodline/tier trackers and class-feature grant machinery, vs. the 839
population's mix of monster/racial special-ability trackers and pick-lists). `decisions.md §14c`
itself already names them as two branches of the same file-kind rule, never merged in code before
this reroute existed.

Both populations skew heavily (A) — directionally consistent — but at different rates (81.6% vs.
90.7%) because the *shape* of `CATEGORY:Internal` content differs by file context: the
`_abilities_class.lst` population is dominated by bloodline/archetype tier-power trackers
(`BONUS:VAR` mechanics, `SPELLKNOWN:` per-level spell grants) that are almost always real content by
construction, while the 839 bare-file population includes a larger share of genuine
gateway/tracker/pick-list shapes (`Path Dabbling` 100% gateway, `Ability Focus`-adjacent pick-lists)
that were always meant to be facets. **Both rates are correct for their own population; neither
should be applied to the other's rows.**

## 5. Applied to code

`scripts/census_independent.py`'s `row_dependent_class_feature` branch previously rerouted **all**
`CATEGORY:Internal` rows in `_abilities_class.lst` files to `ability_category:Internal`
unconditionally. Fixed to test each row (`_row_is_bare_internal_marker`, reusing the same field/
gateway test this memo derives): only a row with **no** content field from the full list **and no**
gateway token reroutes; everything else (2,574 of 2,614 — the 2,371 (A) rows plus the 203
proven-facet (B) rows, since a proven facet is still a real class_feature-shaped row the walker
should not lose track of by relabelling — see note below) stays counted as `class_feature`.

**Deliberate scope narrowing versus §2c's disposition:** the shipped exclusion rule in
`census_independent.py` only implements the *content-or-gateway* test (2,574 stay counted, 40
excluded), not the full gateway-target-resolution test (which would additionally exclude the 203
proven facets). Cross-file target resolution is a heavier, two-pass computation this walker's
single-pass-per-line architecture does not otherwise do anywhere, and — per the anti-gaming
doctrine (`decisions.md §1a`) and the burden-of-proof rule (`§12b`) — the safer default when a
mechanically cheaper, more robust test is available is to under-exclude, not over-exclude.
**The 203 stay counted as `class_feature`, flagged here (not silently absorbed) as real objects a
future card could additionally exclude once a resolution mechanism is added to the production
walker** — they are not lost; they are simply not mechanically distinguished from ordinary
content-bearing class_feature rows by this cycle's shipped code.

Re-derived: `total_kind_unenumerable_units` unchanged at **27,838** (a pure bucket reshuffle within
the already-`kind_unenumerable` population — `class_feature` 15,617 → 18,191, `ability_category:Internal`
3,453 → 879, `18,191 + 879 = 19,070 = 15,617 + 3,453` ✓). `diff.json` regenerated and committed
(`docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/diff.json`)
via its own designated command:

```bash
export PCGEN_CORPUS_ROOT="$(git rev-parse --show-toplevel)/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data"
python3 scripts/census_independent.py --pcgen-root "$PCGEN_CORPUS_ROOT" \
  --inventory docs/work-inventory.json \
  --output docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/diff.json
```

**Tests** (`scripts/tests/test_census_independent.py`): the pre-existing test asserting the old
blanket behaviour was itself wrong (asserted the class_feature memo's own worked example rerouted —
exactly the finding this memo overturns) and is replaced with four tests: (1) a genuinely bare
tracker reroutes; (2) the "Damage Reduction ~ All" `DR:`-bearing row stays `class_feature`, **not**
excluded — the proof that the exclusion rule cannot swallow a real object; (3) a gateway-only row
(no content field) stays `class_feature`, not silently excluded, since this cycle's shipped rule
deliberately does not attempt gateway-target resolution; (4) an ordinary non-`CATEGORY:Internal`
row is unaffected by the new check. RED confirmed against the pre-fix module (loaded standalone,
same test body run against `git show HEAD:scripts/census_independent.py`): test (2)'s row returned
`ability_category:Internal`, not `class_feature`, under the old code. GREEN: all 16 tests in the
file pass against the fixed code (`python3 -m unittest scripts.tests.test_census_independent -v`).

## 6. Handoff to the enumeration lane (card 15's own scope, not this cycle's)

- **2,371 (A) + 203 (B-gateway-resolved, still counted per §5's conservative default) = 2,574 real
  `class_feature`-kind rows** are census-counted (post-fix, in `diff.json`'s
  `kind_unenumerable["class_feature"]` = 18,191) but **not yet added to `docs/work-inventory.json`**
  — same disposition and same next step as the class_feature memo's own 179-row residual (§3 of that
  memo), just 2,574 rows larger. Full row list with book/file/line/identity:
  `15-card-15-category-internal-rows.jsonl` (filter `disposition in ("A","A-unresolved-gateway","B-gateway-resolved")`).
- **40 (B-picklist) rows are proven not-objects** and stay excluded via the code fix in §5 — no
  further action needed on these.
- **Stale downstream artifact, not fixed by this cycle (out of granted scope):**
  `scripts/card15_reconcile.py` hardcodes `"units": 2614` as the fully-disposed-(B) reroute count
  (line 96) and its `class_feature_lane_claim`/`ability_category_lane_claim` narrative fields
  describe the now-superseded tension this memo settles. `15-reconcile.json` (generated by that
  script) is stale for the same reason. Both need updating to: excluded (B) = 40 (not 2,614);
  pending-(A)-not-yet-in-inventory grows by 2,574 (2,371 content-bearing + 203 proven-facet, added
  to the existing 179-row `class_feature` residual, `ability` 5,108, `skill` 170, and the six
  other-kinds candidates 3,551 → new pending_a_total). This is the enumeration/integration lane's
  next cycle, not this one's (scope: `census_independent.py` + its test + this memo + `progress.md`
  + `kanban.md` only).

## 7. Retrospective events logged this cycle

`scripts/retro.py correction --subject 15-card-15-class-feature-memo.md --claimed
"2614 CATEGORY:Internal rows are (B) not an object" --actual "2371/2614 (90.7%) are (A); only
40/2614 (1.5%) are genuinely inert" --verified-by "python3 docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-category-internal-classify.py"`
