---
canonical: true
owner: sd31-wave28-class-feature-lane
purpose: Wave 28 visibility pass. class_feature not-started is the single biggest pile in the
  board (11,971 units, 48% of all remaining work) and had never been systematically examined —
  only ever sampled. This document turns it face-up, groups it, and names the general fix per
  group. NO UNITS ARE BANKED BY THIS DOCUMENT. It is a census and a set of tool proposals.
started: 2026-08-21
population: class_feature, doneness_verdict() == "not-started", 11,971 units
board_at_start: "13,456 / 38,372 (35.07%)"
---

# VISIBILITY — `class_feature` not-started (11,971 units)

## 0. Scope, method, and the one honesty rule this document follows

Every count below is reproducible from `docs/work-inventory.json` plus the repo's own corpus/grant
files, read-only. `docs/work-inventory.json` was not written to. No `cargo` build, no regen, no
CARGO_TARGET_DIR was needed — every number here comes from `python3` reading JSON already on disk.
**Nothing in this document is banked.** No production code, table, or fixture was touched.

The population, and its reproduction command:

```
python3 -c "
import json, sys, collections
sys.path.insert(0,'scripts/observer')
import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
cf=[u for u in U if u.get('kind')=='class_feature']
def v(u): return P.doneness_verdict(u.get('wiring_class'), u.get('status'), u.get('kind'))
ns=[u for u in cf if v(u)=='not-started']
print(len(ns))
"
# -> 11971
```

`class_feature` totals 15,439 units (non-excluded). Of those: **11,971 not-started**, 3,058
`unmeasurable` (status `unknown`), 332 `done`, 40 `deferred`, 38 `held`. This document is about the
11,971 only.

`not-started` itself is two raw statuses: **11,043 `not-ingested`** (the corpus/closure scan found
the row; the engine does not hold it) + **928 `not-started`** (the raw `.lst` closure scan found the
row at all — a slightly weaker claim). Both collapse to the same doneness bucket by
`_doneness_verdict_uncapped()`'s own rule (`scripts/observer/pf1e_dashboard_producer.py:3877`).

**The single most useful discovery of this pass is not a new instrument — it is that one already
exists.** `v06_work_inventory.rs`'s `Kind::ClassFeature` classify arm already stamps every one of
these 11,971 units with a specific, mutually-exclusive `evidence` code naming exactly why it isn't
ingested (`src/bin/v06_work_inventory.rs:6636-6937`). Section 1 below is that engine's own taxonomy,
not a new one layered on top — re-deriving it from the code (not the field alone) is what let this
pass go past "11,971 units, mostly unexamined" into seven load-bearing groups with named fixes.

## 1. The exhaustive split — the engine's own diagnosis, re-derived and verified exact

```
python3 -c "
import json, sys, collections
sys.path.insert(0,'scripts/observer')
import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
cf=[u for u in U if u.get('kind')=='class_feature']
def v(u): return P.doneness_verdict(u.get('wiring_class'), u.get('status'), u.get('kind'))
ns=[u for u in cf if v(u)=='not-started']
c=collections.Counter(u.get('evidence') for u in ns)
for k,n in c.most_common(): print(n,k)
print('SUM', sum(c.values()))
"
```

| Group | Count | Share | What it means (traced to `v06_work_inventory.rs`, cited by line) |
|---|---:|---:|---|
| G1 — `class_feature_option_pool_record_not_held_by_engine` | **3,347** | 28.0% | Zero-magnitude, text-only record. Its corpus-key group prefix names neither an engine-modelled class nor a corpus-declared one — it's an option-pool member (a talent, a hex, a bonus-feat-list entry, a domain power…). No table/picker/catalog holds it by name (line 6733). |
| G2 — `class_feature_owner_matched_by_name_but_record_not_held_by_engine` | **2,890** | 24.1% | Zero-magnitude, text-only record whose group prefix **does** name a real, engine-modelled class — but no explanation id, diagnostic, or pool-catalog entry names this specific record (line 6935). The class is buildable; this one feature just isn't wired. |
| G3 — `no_explanation_id_and_no_diagnostic_names_this_feature` | **2,583** | 21.6% | Same as G2 but **magnitude-bearing** (`text_only == false`) — a real per-character formula/value, owned by a real modelled class, with no consumer function and no diagnostic (line 6937). |
| G4 — `no_compiled_rule_set_for_book` | **928** | 7.8% | The whole book has no compiled rule set at all. 699 `adventurers_guide`, 218 `inner_sea_magic`, 11 `inner_sea_taverns`. Already named in `sweeps.md` L1 at the `class`-kind grain (25/3); this is its `class_feature`-kind size. |
| G5 — `class_feature_of_unmodelled_corpus_class:<X>` | **2,194** | 18.3% | Group prefix names a class the *corpus* declares but the *engine* does not model at all (80 distinct classes, line 6667). Gated entirely behind class-chassis work — the L0/L1 lever. |
| G6 — `class_feature_no_dedicated_magnitude_id_matched_the_record_slug` | **29** | 0.2% | A **near miss**: a broad match (including the generic PU roster id) found something, but the strict, real-magnitude-only match did not (line 6874). Small enough to hand-examine — see §3.7. |
| **Total** | **11,971** | 100% | Sums exact — verified by the script above, not assumed. |

This is a genuinely different shape from the `class`-kind census (157 units, `sweeps.md`): here the
mass is in G1–G3 (8,820 units, 73.7%), all of which sit **under classes that already exist as real,
buildable engine chassis or real corpus-declared classes** — G5's "the class itself doesn't exist"
problem is the *minority* shape for this kind, not the majority one class kind sees.

## 2. Cross-cutting: does the record have real content, and does it need arithmetic?

The `evidence` taxonomy answers "why is this not ingested." It does not answer the operator's other
questions — real prose vs. empty vs. no-record, and formula shape — without joining every unit
against its own corpus JSON. Built here for the first time this pass (12,481 `class_feature` corpus
files indexed by `(book, corpus_key)`, 11,971 units joined against it):

```
python3 -c "
import json, sys, os, collections, re
sys.path.insert(0,'scripts/observer')
import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
cf=[u for u in U if u.get('kind')=='class_feature']
def v(u): return P.doneness_verdict(u.get('wiring_class'), u.get('status'), u.get('kind'))
ns=[u for u in cf if v(u)=='not-started']
books=set(u.get('book') for u in ns)
index={}
for book in books:
    root=os.path.join('data/corpus', book, 'class_feature')
    if not os.path.isdir(root): continue
    for dp,_,fns in os.walk(root):
        for fn in fns:
            if fn.endswith('.json'):
                p=os.path.join(dp,fn)
                rec=json.load(open(p))
                k=rec.get('data',{}).get('key')
                if k: index[(book,k)]=rec
buckets=collections.Counter()
for u in ns:
    key=(u.get('book'), u.get('corpus_key'))
    if key not in index:
        buckets['no_corpus_record']+=1; continue
    rec=index[key]
    desc=rec.get('data',{}).get('description')
    wc=u.get('wiring_class')
    if desc is None or (isinstance(desc,str) and desc.strip()==''):
        buckets[f'null_desc|{wc}']+=1; continue
    tag='has_pctN' if re.search(r'%\d',desc) else 'clean_desc'
    buckets[f'{tag}|{wc}']+=1
for k,c in sorted(buckets.items(), key=lambda kv:-kv[1]): print(c,k)
print('SUM', sum(buckets.values()))
"
```

| Bucket | Count |
|---|---:|
| `clean_desc \| display` — real prose, no `%N`, zero-magnitude classifier | **4,435** |
| `no_corpus_record` — closure scan found the row, no JSON was ever written | **2,381** |
| `clean_desc \| computed` — real prose but a real magnitude lives elsewhere in the record | 1,322 |
| `null_desc \| display` — no description field at all, zero-magnitude classifier | 1,081 |
| `has_pctN \| computed` | 477 |
| `null_desc \| computed` | 474 |
| `has_pctN \| derived` | 472 |
| `clean_desc \| derived` | 328 |
| `clean_desc \| static` | 310 |
| `has_pctN \| display` | 237 |
| `clean_desc \| ambiguous` | 173 |
| `null_desc \| static` | 156 |
| `has_pctN \| static` | 68 |
| `null_desc \| derived` | 53 |
| `has_pctN \| ambiguous` | 4 |
| **SUM** | **11,971** |

Sums exact. **7,826 of 11,971 (65.4%) have real, printable prose already sitting in the corpus JSON.
1,764 (14.7%) have a corpus record but a genuinely null description field. 2,381 (19.9%) have no
corpus JSON at all.**

## 3. The findings, ranked, each with a general fix and a tool evaluation

### 3.1 — F1 (the headline): a generic `class_feature` render catalog already exists and is proven — it just isn't trusted as proof-of-holds

**3,536 units** (G1's 1,656 + G2's 1,880 — computed by re-running §2's join filtered to those two
evidence codes) are `wiring_class: display`, carry a real, clean (no dropped `%N`) description, and
belong to either a real class or a real option pool. **Nothing about these records is unknown or
unresolved — the content exists, is PI-screened, and the render path to show it already exists in
the tree.**

`apps/desktop/src-tauri/src/class_feature_descriptions.rs` (494 lines, `SD31-D7-PROSE-003`) is a
**real, working, generic catalog** — its own doc comment says it "walks 12,000+ live corpus records
at process-start time", is PI-screened (SD-30 `§52.3`/`§53.5`), and runs the same leak guard
(`leaked_pcgen_syntax`) `class_feature_pool_catalog.rs`/`monster_catalog.rs`/`companion_catalog.rs`
all share. This is the exact shape `class_feature_pool_catalog.rs`'s own module doc names as missing
("no generic class_feature catalog exists anywhere in this engine, unlike feat/spell/equipment")
— **except it does exist now**, built one cycle later, and this pass is the first to notice the two
haven't been connected.

**Why these 3,536 units still read `not-started`:** `class_feature_descriptions.rs` only *serves*
a description once the frontend already has an `ExplanationDto.id` to join against
(`classFeaturesModel.ts`'s suffix match) — i.e. it rides the SAME explanation-id join `classify()`
uses to decide `grounded`. `classify()`'s own eligibility gate for `text-complete` requires **either**
an explanation-id match (G2's population fails this — that's the whole reason it's G2) **or**
membership in `class_feature_pool_catalog::REGISTERED_POOL_GROUPS`, currently 2 pools wide
(G1's population fails this too, for 815 of its 817 pool names). Neither test asks the question
Decision 7 actually asks: does the engine hold a real, render-safe description for this exact
record? `class_feature_descriptions.rs` already answers that question for the whole corpus; `classify()`
just never asks it.

**General fix, named specifically:** widen `Kind::ClassFeature`'s `text_only` eligibility in
`v06_work_inventory.rs` (the `if text_only { ... }` branches at lines 6719 and 6905) to accept a third
proof-of-holds: `class_feature_descriptions.rs`'s own render succeeding without a dropped `%N` or a
leaked-syntax finding, for ANY record — not gated on `REGISTERED_POOL_GROUPS` membership or an owner
match at all. This is a genuine widening of an already-proven mechanism, not new machinery.

**Tool evaluation.** This is a tool question with an unusually cheap answer: the tool (the catalog)
is **already built**. The remaining work is a classify()-side eligibility change plus re-running the
SAME Decision-7 safety checks (`universal_sheet_modifier`, dropped-`%N` refusal) `class_feature_pool_catalog.rs`
already implements once, generically, instead of once per registered pool. **Corpus-wide ceiling:
up to 3,536 units in this pile alone**, before even considering `race_trait`/`monster_ability`
records that might hit the identical shape (a sweep question, not answered here — see §5).
**What must be checked before trusting it at scale:** the render-and-refuse gate's `universal_sheet_modifier`
check must run over the SAME raw closure text Decision 7 REFINED's discriminator reads
(`closure_states_universal_sheet_modifier`) for every one of the 3,536 — a hand-verified sample, not
an assumption, per the standing proxy-validation rule (`decisions.md §7`'s PROXY WARNING).

### 3.2 — F2: the bonus-feat cross-reference bridge — real content sitting unused under a different `kind`

A distinct, provable sub-shape: a `class_feature` record whose entire content is a grant of an
**already-separately-modelled `feat`**. Detected via `type_facet` containing "BonusFeat" in any
casing/spacing (`Warpriest Bonus Feat`, `MonkBonusFeat`, `MasterOfManyStylesBonusFeat`,
`RangerBonusFeat`, and 26 smaller variants):

```
python3 -c "
import json, sys, collections
sys.path.insert(0,'scripts/observer')
import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
def v(u): return P.doneness_verdict(u.get('wiring_class'), u.get('status'), u.get('kind'))
feat_by_name=collections.defaultdict(list)
for u in d['units']:
    if u.get('kind')=='feat': feat_by_name[u.get('name')].append(u)
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
cf=[u for u in U if u.get('kind')=='class_feature']
ns=[u for u in cf if v(u)=='not-started']
bf=[u for u in ns if u.get('type_facet') and 'bonusfeat' in u.get('type_facet').lower().replace(' ','')]
matched=sum(1 for u in bf if feat_by_name.get((u.get('corpus_key') or '').split(' ~ ')[-1]))
matched_done=sum(1 for u in bf if any(v(c)=='done' for c in feat_by_name.get((u.get('corpus_key') or '').split(' ~ ')[-1],[])))
print(len(bf), matched, matched_done)
"
# -> 1378 1028 708
```

**1,378 units total** across 11 books (`ultimate_combat` 588, `advanced_class_guide` 477,
`inner_sea_combat` 115, …). **1,028 (74.6%) exact-name-match a real, independently-modelled `feat`
unit. 708 (51.4% of the whole 1,378) match a feat that is already `done`** — content that already
exists, already verified, already rendering somewhere else on the sheet, just not cross-referenced
into this record.

Concrete example, read from the corpus JSON directly (`data/corpus/advanced_class_guide/class_feature/daring_infiltrator/daring_infiltrator_alertness.json`):

```json
"description": null,
"raw_tokens": [
  ...
  {"key": "ABILITY", "value": "FEAT|AUTOMATIC|Alertness"}
]
```

`description` is genuinely `null` — the record's whole job is the grant, not the content. A player
reading this class feature is owed the real Alertness feat text, which exists, verified, elsewhere
in the corpus (`feat:alertness`, `done`).

**Refined by presence of local content** (same script family):

| Sub-bucket | Count | Fix |
|---|---:|---|
| Null local description, feat-name matched | **431** | Needs the bridge specifically — nothing local to render otherwise. |
| Real local description, feat-name matched | 463 | Already coverable by F1's generic-catalog widening; the bridge is a second, independent path to the same answer. |
| Real local description, no feat match | 40 | F1 only. |
| Null local description, no feat match | 31 | Neither fix reaches these — needs per-record content sourcing or a fuzzy-match pass (see below). |
| No corpus record at all | 413 | Needs ingest first (§3.3/§3.4). |

**Tool evaluation.** A general fix, precisely scoped: for any `class_feature` record whose only
`ABILITY` token is `FEAT|AUTOMATIC|<name>` or `FEAT|VIRTUAL|<name>` (a single, unconditional grant,
no formula), render the target feat's own already-verified description through
`feat_catalog.rs`'s existing render path instead of the class_feature's own (null) one. **Closes 431
units outright** (the null+matched sub-bucket) and, combined with F1, the other 463. **What I could
not determine:** the 274-unit unmatched tail (25.4% of 1,028's complement) needs name normalization —
sampled examples show weapon-parenthetical suffixes (`Improved Critical (rapier)`), case variants
(`Exotic Weapon Proficiency Whip` vs. feat `Exotic Weapon Proficiency (whip)`), and compound-choice
names (`Impaling Critical ~ unarmed strike`) that a bare exact-name join cannot resolve — a fuzzy
matcher would need to be built and hand-verified, not assumed to close cleanly.

### 3.3 — F3: Ultimate Psionics has zero `class_feature` corpus records — a whole-book ingest gap, not a per-record one

```
find data/corpus/ultimate_psionics -maxdepth 1 -type d
# -> equipment, monster, monster_ability. No class_feature/ directory at all.
```

**All 994 of Ultimate Psionics' not-started `class_feature` units are missing from the corpus
JSON store entirely** — confirmed both by the directory listing above and by the join in §2 (994
of 994 in the `no_corpus_record` bucket come from this one book). This is distinct from G4
(`no_compiled_rule_set_for_book`) — Ultimate Psionics is NOT in that bucket; its `class` records ARE
declared (10 real classes, per §3.6), but no `class_feature` ingest step has ever run for this book
at all, at any scale.

**Tool evaluation.** Not a tool question — a missing ingest run. **994 units, one book, one
pipeline invocation** (`scripts/book-ingest-workflow.py` or the equivalent `cache_gen::class_feature`
generator pointed at `ultimate_psionics/pu_abilities_class.lst`, following the same playbook every
other book's class_feature ingest already used). Cheapest-per-unit fix in this whole census by a wide
margin, but it only produces corpus JSON — it does **not** by itself move any unit past G5's
"class itself isn't modelled" gate (10 of Ultimate Psionics' classes have zero chassis; see §3.6),
so ingest alone would move these units from `no_corpus_record` to `class_feature_of_unmodelled_corpus_class`,
not to `done`.

### 3.4 — F4: `data/class_feature_grants/` covers less than a quarter of the pile

```
python3 -c "
import json, sys, glob, collections
sys.path.insert(0,'scripts/observer')
import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
def v(u): return P.doneness_verdict(u.get('wiring_class'), u.get('status'), u.get('kind'))
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
cf=[u for u in U if u.get('kind')=='class_feature']
ns=[u for u in cf if v(u)=='not-started']
grants={}
for f in glob.glob('data/class_feature_grants/**/*.json', recursive=True):
    book=f.split('/')[2]
    for rec in json.load(open(f)): grants[(book, rec['key'])]=rec
with_g=sum(1 for u in ns if (u.get('book'),u.get('corpus_key')) in grants)
print(with_g, len(ns)-with_g)
"
# -> 2887 9084
```

**Only 2,887 of 11,971 (24.1%) have a known grant fact** (which class grants this feature, at what
level). The primary driver: `data/class_feature_grants/` covers **16 books**; the not-started
population spans **23**. `ultimate_psionics` (994 units), `pathfinder_unchained`,
`inner_sea_world_guide`, both `book_of_the_damned_volume_*`, `bestiary_6`, and `bestiary_4` have
**no grant file at all**, regardless of any individual unit's own content. Even inside the 16 covered
books, coverage is partial — the sampled files (e.g. `advanced_class_guide/hunter.json`) mark every
row `"granted_via_archetype": true`, suggesting the grants pipeline may have been scoped to
archetype-substitution features specifically rather than the full class roster. **What I could not
determine:** whether that scoping is deliberate (archetype grants were the original ask) or an
accidental narrowing — the generating script was not read this pass; a future cycle should check
`scripts/` for the grants generator before assuming either.

### 3.5 — F5: the magnitude-bearing gap (G3, 2,583 units) — needs real computation, not text

G3 is exactly the population §2's cross-tab shows as `computed`/`derived`/`static` with no
explanation id: **1,751 `computed`, 526 `derived`, 302 `static`, 4 `ambiguous`**. These carry a real
formula the engine has never modelled for this feature specifically, owned by a real, buildable
class. This is the population Ruling §20 (interpreter authorised) is most directly aimed at.

A shallow signal (regex over `BONUS`/`SA`/`ABILITY`/`DESC` raw-token text for the four known-refused
shapes named in `defects.md` D3, run over the `computed`+`derived` slice, 3,745 units before removing
619 with no corpus record):

```
python3 -c "... (see full script in §2's family; scans for PREVARGTEQ / && / skillinfo() vs. an
    LVL/classlevel()/CL signal) ..."
# -> refused-shape signal: 554 (14.8%); readable LVL/classlevel signal: 882 (23.6%);
#    no clear formula token in the fields scanned: 1,690 (45.1%); no corpus record: 619
```

**This is a rough proxy, explicitly flagged as such — a substring scan over four raw-token keys, not
a dry run of the actual interpreter.** It should not be quoted as a formula-shape census; it says
only that the refused shapes are a real, double-digit-percent presence in this population and that a
large plurality (1,690) needs a closer per-record look before any interpreter-readiness number can be
trusted. **What I could not determine:** true interpreter readability requires running the actual
tokenizer against each formula, which this pass did not do.

### 3.6 — F6: the 80-class unmodelled-corpus-class tail (G5, 2,194 units) — and a correction to `sweeps.md`

The 80 named classes (see §1 command for the full list) are the `class_feature`-kind shadow of the
already-known `class`-kind census (`sweeps.md`, wave 27: 157 not-done classes, 77 prestige + 48
structurally-non-PC + 18 real-base-zero-table + 28 no-ruleset-book + 5 CRB NPC + 2 Ninja/Samurai
near-miss). No new class-existence work belongs to this document — that census stands. **One
correction is owed to it**, found incidentally while tracing G5's owner-resolution: `sweeps.md`'s
"real base classes with zero table: 18" names Antipaladin + the 6 Occult Adventures classes + the 10
Ultimate Psionics classes — **1 + 6 + 10 = 17 named classes against a stated count of 18.** Cross-
checking the `class`-kind `class_absent_from_ClassId_ALL_and_book_class_id_enums` evidence bucket
directly:

```
python3 -c "
import json
d=json.load(open('docs/work-inventory.json'))
for u in d['units']:
    if u.get('kind')=='class' and u.get('name')=='Magus':
        print(u['book'], u['evidence'])
"
# -> ultimate_magic class_absent_from_ClassId_ALL_and_book_class_id_enums
```

**Magus (`ultimate_magic`) is a real PF1e base class with zero engine chassis**, carrying the exact
same evidence tag as Antipaladin and the 16 others — and it accounts for **123 `class_feature`
not-started units** in G5 (`class_feature_of_unmodelled_corpus_class:magus`). This resolves the
17-vs-18 arithmetic gap in `sweeps.md`'s own table; it should be added to that row's named list
rather than left as an unnamed 18th.

### 3.7 — F7 (small, but the S9-shaped population this wave was told to look for): G6's 29 near-misses

G6 is the one bucket small enough to name every unit and check by hand against the S9 sweep
(`sweeps.md`): "the Ninja/Samurai shape — complete chassis blocked by one missing row in a downstream
table. How many other units corpus-wide are one table row from working?"

```
python3 -c "
import json, sys, collections
sys.path.insert(0,'scripts/observer')
import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
def v(u): return P.doneness_verdict(u.get('wiring_class'), u.get('status'), u.get('kind'))
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
cf=[u for u in U if u.get('kind')=='class_feature']
ns=[u for u in cf if v(u)=='not-started']
g6=[u for u in ns if u.get('evidence')=='class_feature_no_dedicated_magnitude_id_matched_the_record_slug']
print(collections.Counter(u.get('book') for u in g6).most_common())
for u in g6[:6]: print(u.get('corpus_key'))
"
```

29 units, `advanced_class_guide` (17), `pathfinder_unchained` (7), `advanced_players_guide` (3),
`ultimate_wilderness` (2). **I could not confirm the S9 shape (one missing table row) for these 29
without reading each one's specific magnitude-suffix-strip failure in `id_matches_feature_slug_after_known_magnitude_suffix_strip`
against the live explanation-id list — that requires a running engine build (the `--bin
v06_work_inventory` `facts.explanation_ids` set), which this read-only pass did not build.** Flagging
this as the single most promising unexamined lead this document did not close: 29 is small enough for
one cycle to fully hand-trace, and if even a handful match the Ninja/Samurai shape, that is a very
cheap close per the operator's own framing.

### 3.8 — G1's own long tail: 817 distinct option-pool names, and the open-vs-exclusive question ruling §18 requires

G1 (3,347 units) is not one pool problem. **817 distinct group-prefix names**; the top 30 cover only
44.3% of the population (`Master of Many Styles` 238, `Wild Talent` 166, `Skill unlock` 131, `Combat
Trick` 102, `Discovery` 93, `Domain Power` 66, …). Only **161 units across 6 group names** (`Rage
Power`, `Discovery`, `Focused Arcane School`, `Mercy`, `Unchained Rage Power`, `Judgment`) overlap
`v06_work_inventory.rs`'s own `CLASS_FEATURE_POOLS` list (27 pools with a real consumer-delta probe)
— **the other 3,186 units, across 811 group names, have never been examined by any existing
mechanism**, consumer-delta probe or pool-catalog registration alike.

Ruling §18 makes this population's real gate OPEN vs. EXCLUSIVE, not a text/magnitude question F1
already answers: an open, repeatable-pick pool (Rogue Talent, Rage Power, Discovery, Hex) may use the
reference-catalog pattern F1 proposes; an exclusive, once-per-character pool (Domain Power, Bloodline,
Mystery, Arcane School, Order, Spirit, Curse) may not and needs real per-choice grounding instead —
Bestiary 6's Domain Power units are the paradigm case already ruled on.

**What I could not determine:** which of the 817 group names are OPEN vs. EXCLUSIVE at scale.
`CLASS_FEATURE_POOLS`'s own 27-entry list is a strong partial answer by precedent (its shape names
are recognizable PF1e categories), but extending that judgment to 790 more names, most represented by
single-digit-to-low-double-digit unit counts, needs either a PCGen-token-level heuristic (does the
granting feature's `CHOOSE:`/level table show a repeatable per-level pick vs. a one-time selection) or
per-name domain knowledge — neither was built or applied here. **This is the honest edge of this
pass**: G1's total (3,347) and its top-30 concentration are solid; its open/exclusive split is not
sized and should not be assumed either way.

## 4. Defect worth filing: a classifier miss inside G1/clean data

§2's cross-tab surfaced **237 `has_pctN | display` units** — records the wiring classifier calls
zero-magnitude, that nonetheless carry an unresolved `%N` formula placeholder in their own
description. Example, read directly from the corpus JSON
(`advanced_class_guide:class_feature:arcanist_school_void_aura_of_prescience`):

> "You can emit a 30-foot aura of void energy for **%1** rounds. Allies within this aura gain a +2
> insight bonus…"

This is the SAME miss class `decisions.md §7`'s own correction already named at small scale ("9 of
121 hand-checked, a single named failure mode — flat, non-scaling numerics in `SPROP:`/`BENEFIT:`
fields the wiring-class classifier does not read", `OPEN-ISSUES.md` row 69) — **237 is a much larger
corpus-wide instance of that exact defect**, found here because this pass joined every unit against
its own description text rather than trusting the `wiring_class` field alone. **These 237 must be
excluded from F1's promotion path** (the render-and-refuse gate would correctly catch the dropped
`%1` and refuse them anyway — this is not a double-count risk, only a classifier-accuracy finding
worth its own line in `defects.md`).

## 5. Summary — tool evaluation table, ranked by corpus-wide ceiling

| # | Group | Units | Hand or tool? | What it costs | What it closes, corpus-wide |
|---|---|---:|---|---|---|
| F1 | Generic render-catalog eligibility widening | up to 3,536 | Tool — the catalog exists; widen `classify()`'s eligibility gate | Small: one `classify()` branch change + re-verify the render-and-refuse gate over the population | This pile alone up to 3,536; likely more if `race_trait`/`monster_ability` share the shape (unmeasured — a sweep, not answered here) |
| F2 | Feat cross-reference bridge | 431 (+463 overlapping F1) | Tool — new, narrow bridge keyed on `ABILITY:FEAT\|` tokens | Small: one join + the existing feat catalog's render path | 431 outright; scales to any book with the same bonus-feat-list shape, not just this pile's 4 named type_facets |
| F3 | Ultimate Psionics `class_feature` ingest | 994 | Book-scale ingest run, not per-unit | One pipeline invocation | 994 units move off `no_corpus_record`; does NOT reach `done` alone (blocked by G5's class-chassis gap next) |
| G4 | `no_compiled_rule_set_for_book` | 928 | Already-known lever (`sweeps.md` L1) | Book-scale ingest (`adventurers_guide`, `inner_sea_magic`) | Confirms existing lever's size at this kind's grain |
| G5 | Unmodelled corpus class | 2,194 | Structural — L0 (prestige gating) / L1 (chassis) | Large, already tracked | Gated entirely behind class-chassis work; not this document's to size further |
| F5 | Magnitude-bearing, no consumer (G3) | 2,583 | Hand-modelled function or interpreter (Ruling §20) | Per-feature, linear-to-constant depending on interpreter adoption | Rough signal only — 554 likely-refused-shape, 882 likely-readable, 1,690 unclassified |
| §3.8 | Option-pool long tail (G1 minus F1's slice) | ~3,186 remaining after F1 | Unclear until open/exclusive is sized | Unknown — the open/exclusive question is unresolved | Unknown |
| §3.7 | G6 near-misses | 29 | Hand — small enough to trace fully | One cycle, full trace | Unmeasured but potentially very cheap per-unit (S9 shape) |

**The biggest single lever by corpus-wide ceiling is F1**: it costs one eligibility-gate widening
against an already-built, already-guarded render surface, and its ceiling (3,536 units, 29.5% of
this entire pile) dwarfs F2 and F3 combined. It is also the safest of the three — it reuses machinery
Decision 7 already proved out for 2 pools and for `race_trait`/`monster_ability`, rather than
building anything new.

## 6. What this document could not determine (stated plainly, per the wave's own bar)

- **G1's open-vs-exclusive split** (817 group names, only 6 cross-checked against the known-pool
  registry) — the single largest unresolved question in this census.
- **Real interpreter-readiness for G3's 3,745 `computed`/`derived` units** — §3.5's numbers are a
  substring-regex proxy over four raw-token keys, not a dry run of the actual formula tokenizer.
- **Whether the 29 G6 near-misses are S9-shaped** (Ninja/Samurai: complete chassis, one missing
  downstream table row) — requires a live engine build to inspect `facts.explanation_ids`, which this
  read-only pass did not perform.
- **Why `data/class_feature_grants/` scopes every sampled row `granted_via_archetype: true`** —
  whether that's the pipeline's deliberate scope or an accidental narrowing was not checked against
  the generating script.
- **F2's 274-unit unmatched tail** — needs a fuzzy name matcher (weapon parentheticals, case
  variants, compound choice names) that was not built here.
- **Whether F1's mechanism generalizes to `race_trait`/`monster_ability`** — both kinds have their
  own `*_descriptions.rs`-shaped render surfaces per the Bestiary 6 ledger and Decision 7's own
  history (`SD31-D7-PROSE-001`/`002`), but whether they hit the identical "catalog exists, eligibility
  gate doesn't trust it" shape was not checked corpus-wide. **Named as a new sweep candidate below.**

## 7. Filed to the TODO directory

- **New sweep, `sweeps.md`:** does F1's shape (a proven, generic render catalog whose output
  `classify()`'s eligibility gate does not trust as proof-of-holds) recur for `race_trait` and
  `monster_ability`? `SD31-D7-PROSE-001`/`002` built the equivalent render surfaces for both kinds;
  neither was checked against this exact gap this pass.
- **Correction to `sweeps.md`'s L1/wave-27 table:** "real base classes with zero table: 18" names 17
  (Antipaladin + 6 OA + 10 PU); Magus (`ultimate_magic`) is the 18th, confirmed by the same
  `class_absent_from_ClassId_ALL_and_book_class_id_enums` evidence tag, and accounts for 123
  `class_feature` units in this pile's G5.
- **Correction/expansion to `levers.md` L6** ("36 grant facts with no DESC at all... likely many more
  corpus-wide"): confirmed much larger within `class_feature` alone — 1,764 not-started units carry a
  genuinely null description field (§2), not 36.
- **New defect for `defects.md`:** 237 `class_feature` units read `wiring_class: display` (zero-
  magnitude) while their own description carries an unresolved `%N` placeholder — the same miss shape
  `decisions.md §7`'s correction named at n=9; this is a corpus-wide instance at n=237, found by
  joining the classifier's output against the record's own text rather than trusting the field.
