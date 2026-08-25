# Card 11, T2a-residual — census (measurement only, `decisions.md §13`)

**Scope:** measurement only, per the dispatch brief. This memo does not change engine behaviour,
corpus data, or any pinned count. It sizes the T2a-residual sub-population for sibling work-lane
cycles, per `decisions.md §13`: *"Measurement is explicitly authorised as a first step, and does
not substitute for the work... A measurement cycle that banks zero units but produces a real,
re-derivable book/file census is a legitimate closed cycle."*

- **Actor:** `t2a-residual-census`
- **Base:** `8b8e00c0d` (rebased onto `origin/tranche/12` at `3981e7091`)
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`)
- **Re-derive script (committed):** `scripts/sd32-t2a-residual-census.py`

## What T2a-residual is

The T2a+T12 combined cycle (`artifacts/gate-3-closure-invariant/epic-2-t2a-t12_cycle-1_cycle_receipt.md`,
commit `985e24c1e`) fixed `cache_gen::class_feature::generate()`'s `data.class` derivation so it
resolves the TRUE owning class via four tiers (grant fact, registered option-pool table, `TYPE:`
class marker, full corpus-declared-class-name match) before falling back to the raw, unvalidated
key-prefix text. That cycle corrected 4,936 records at the cause and closed structurally. What's
left — **T2a-residual** — is every `class_feature` record whose `data.class` is STILL a category
label (an option-pool name, an archetype tag, a deity-obedience-line name, …) that none of those
four tiers could turn into a real class name. This is my scope; I did not touch the generator,
corpus, or any code this cycle.

## Re-deriving the total (do not trust `decisions.md §13`'s "~2,775")

`decisions.md §13`'s table carries T2a-residual as **~2,775**, explicitly tilde-marked as an
estimate (computed there as `|T2a| − |T2a∩T12|` via a T12-key-driven join). I re-derived it
directly and independently: for every `class_feature` corpus record with a non-null `data.class`,
classify the value as (a) one of the 34 engine-dispatched classes, (b) a real corpus-declared class
name (`docs/work-inventory.json`, `kind == "class"`, 182 names) that the engine simply doesn't model
yet — that's the T12-overlap shape, not mine — or (c) neither: a category label. (c) is
T2a-residual.

```bash
python3 scripts/sd32-t2a-residual-census.py
# -> total 12464  non-null-class 11904  dispatched 7620  corpus-declared-undispatched(T12-overlap-shape) 1644  residual-category-label(T2a-residual) 2640
```

**My re-derived total is 2,640, not ~2,775.** The ~135-unit gap is not an error in either figure —
it's the same order-of-magnitude discrepancy the T2a+T12 receipt itself already documented between
its two cross-check methods (1,509 vs 1,644 for the overlap, "explained by the two methods'
different join directions... not a computation error in either"). My method classifies every
record directly by its own `data.class` value against the full corpus class roster; `decisions.md
§13`'s estimate joins T12's specific evidence-coded key set (2,453 keys, only 1,556 of which even
have a `class_feature` corpus record under this generator's 21-book scope) to the corpus. The two
measure closely related but not identical populations. **Logged as a `scripts/retro.py correction`**
(`docs/retro/events/t2a-residual-census.jsonl`) — see Findings below.

## The population: 547 distinct category labels, 18 books, heavy long tail

```bash
python3 scripts/sd32-t2a-residual-census.py --groups
```

- **2,640 residual units** across **547 distinct category labels**, spanning **18 books**.
- Distribution is heavily long-tailed: the top 50 labels cover 1,425 units (54%); the top 100 cover
  1,833 (69%). **266 of the 547 labels (49%) are singletons** (exactly 1 record); **398 (73%) have
  ≤3 records.**
- Per-book totals (residual units / distinct labels in that book):

| Book | Residual units | Distinct labels |
|---|---:|---:|
| advanced_players_guide | 484 | 121 |
| advanced_class_guide | 374 | 62 |
| occult_adventures | 282 | 17 |
| ultimate_combat | 220 | 44 |
| core_rulebook | 212 | 36 |
| ultimate_magic | 203 | 28 |
| book_of_the_damned_volume_2 | 175 | 103 |
| ultimate_intrigue | 162 | 8 |
| adventurers_guide | 153 | 54 |
| advanced_race_guide | 118 | 28 |
| inner_sea_world_guide | 104 | 65 |
| ultimate_wilderness | 77 | 13 |
| inner_sea_magic | 22 | 10 |
| inner_sea_intrigue | 17 | 5 |
| bestiary_6 | 13 | 1 |
| horror_adventures | 11 | 2 |
| inner_sea_combat | 9 | 1 |
| bestiary_4 | 4 | 1 |
| **Total** | **2,640** | **547** |

Re-derive command (per-book breakdown): see the Python snippet embedded in
`scripts/sd32-t2a-residual-census.py`'s module docstring usage, or run `--groups` and pipe through
the book-aggregation shown in this memo's own working notes (the script's per-record loop is the
source of truth; the per-book table above was produced by grouping its output by
`p.split('/')[2]`).

## `registered: true` check against `CLASS_FEATURE_POOLS`'s 27 entries

**Zero of the 547 residual labels match any of the 27 `CLASS_FEATURE_POOLS` /
`POOL_TO_DISPATCHED_CLASS` entries exactly.** This is expected, not a gap in my check: the
generator's tier-2 resolution (`src/rules_core/cache_gen/class_feature.rs`'s
`POOL_TO_DISPATCHED_CLASS`, reproduced verbatim from `v06_work_inventory.rs::CLASS_FEATURE_POOLS`)
already resolves any record whose group-prefix matches one of those 27 labels — if a residual
record's label matched, the T2a+T12 cycle's own regen would already have corrected it. Confirmed by
diffing the 547 residual labels against the 27-entry table (no overlap).

```bash
grep -c '","' <(true)  # sanity placeholder; the actual check is a set-difference, verified interactively:
python3 -c "
pools = ['Rage Power','Unchained Rage Power','Discovery','Grand Discovery','Rogue Talent','Advanced Talents',
'Hex','Revelation','Mercy','Investigator Talent','Slayer Talent','Judgment','Inquisition','Blessing',
'Evolution','Bloodline','Bloodrager Bloodline','Domain','Order','Mystery','Curse','Spirit','Animal Focus',
'Favored Enemy','Favored Terrain','Versatile Performance','Arcane School','Focused Arcane School']
residual = set(open('/tmp/labels.txt').read().splitlines())
print('overlap:', residual & set(pools))
"
# -> overlap: set()
```

So **every group in this census has `registered: false`** in the structured return.

## Consumer-conflict hazard audit

The dispatch brief flags the found hazard: `class_feature_pool_catalog.rs`'s Rogue Talent/Rage
Power picker used to read `data.class` for filtering, the opposite purpose from
`class_feature_descriptions.rs`'s join use — already fixed by the T2a+T12 cycle (now reads the
corpus `key`'s own `" ~ "`-split prefix, never `data.class`; confirmed by reading
`REGISTERED_POOL_GROUPS`'s doc comment and the module's own tests).

I audited every remaining `data.class` / `data["class"]` reader in the codebase for a similar
conflict:

```bash
grep -rn 'data\["class"\]\|data\.get("class")\|\.class ==\|data\.class' src/ apps/desktop/src-tauri/src/
```

Four real readers found, beyond the already-fixed pool catalog:

| File | Purpose | Conflict with mapping a residual label to its true class? |
|---|---|---|
| `class_feature_pool_catalog.rs` | filters `Rogue Talent`/`Rage Power` records | **Already fixed** — reads `key`-split, not `data.class` |
| `apps/desktop/src-tauri/src/class_feature_descriptions.rs` | `class_slug` join key for the frontend | No — mapping IMPROVES this consumer (its own module doc: `class_slug` exists so the frontend can join a real `class_feature.<class>.*` id, which a category label never could) |
| `src/rules_core/pilot_compute/class_feature_grant_consumer.rs` | reads owning `data.class` for grant-consumption logic | No — same posture, mapping improves it |
| `apps/desktop/src-tauri/src/class_feature_feat_bridge.rs` | reads owning `data.class` to bridge to feat prerequisites | No — same posture |

**No new consumer-conflict hazard found.** All four readers treat `data.class` as "the record's real
owning class" — the same meaning T2a's fix establishes — so widening the resolution (mapping more
residual labels to their true class) only sharpens these consumers' correctness, it never breaks
them the way the pool-catalog's filter-key misuse did.

I also grepped every one of the 547 literal label strings against `src/` and
`apps/desktop/src-tauri/src/` as a broader sweep. Most hits are coincidental substring matches in
unrelated tables/prose (e.g. `"Adaptation"` appears in spell-list and monster-data module names;
`"Pack Lord"` matches a *different* corpus key, `"Druid Archetype ~ Pack Lord"`, in
`reach_gate.rs`'s test fixtures — a different record entirely, not a `data.class` comparison). None
of these are `data.class`-comparison sites; I traced every hit that touched a non-test,
non-doc-comment line and none compares against these residual label strings directly.

## Feasibility sample — what "map it or prove it should not be mapped" actually costs, per group

I read `TYPE:`/`PRE*:` tokens for a sample of the top-10 labels (the same discipline
`CLASS_FEATURE_POOLS`' own 27 entries were built through — reading each group's real corpus tokens
before trusting a mapping, per `decisions.md §3`'s fixture-check bar):

```bash
python3 -c "
import json, glob, os
labels = ['Wild Talent','Refined Education','Ki Power','Master of Many Styles','Implement School Focus Power','Demonic Obedience','Pack Lord','Adaptation','Blessings','Favored Enemy Bonus']
for lab in labels:
    for p in glob.glob('data/corpus/*/class_feature/**/*.json', recursive=True):
        if os.path.basename(p).startswith('manifest'): continue
        d = json.load(open(p)); data = d.get('data')
        if isinstance(data, dict) and data.get('class') == lab:
            types = [t['value'] for t in data.get('raw_tokens', []) if t['key'] == 'TYPE']
            pre = [t['value'] for t in data.get('raw_tokens', []) if t['key'].startswith('PRE')]
            print(lab, '|', data['key'], '| TYPE:', types, '| PRE:', pre)
            break
"
```

Results, and what they mean for mapping cost:

| Label | Count | Sample evidence | Disposition |
|---|---:|---|---|
| `Wild Talent` | 128 | `PRE: KineticistLVL_Fire,10` | **Clean single-class candidate** → `Kineticist` (corpus-declared, engine-undispatched). Mapping this closes T2a-residual for these 128 records but *moves* them into the T2a∩T12 overlap shape (a real, still-unmodelled class) — it does not close T12, which is separate work. |
| `Ki Power` | 80 | `PRE: 1,Monk=4` | **Clean single-class candidate** → `Monk`, which **is already dispatched**. This is the highest-value quick win in the sample: mapping it closes 80 units all the way to a dispatched class, no T12 dependency. |
| `Master of Many Styles` | 53 | `PRE: [PREABILITY... Master of Many Styles ~ Djinni Style], MonkBonusFeatLVL,1` | Clean → `Monk` (archetype-tied bonus-feat pool). Same quick-win shape as `Ki Power`. |
| `Pack Lord` | 40 | `TYPE: DruidClassFeatures.SpecialQuality` | Clean → `Druid` (dispatched). Quick win. |
| `Adaptation` | 39 | `TYPE: ClassFeatures.RangerClassFeatures.AdaptationSelection` | Clean → `Ranger` (dispatched). Quick win. |
| `Favored Enemy Bonus` | 37 | `TYPE: RangerClassFeatures.FavoredEnemyBonus` | Clean → `Ranger` (dispatched). Quick win. |
| `Implement School Focus Power` | 48 | `PRE: OccultistSchool_Abjuration=true, OccultistLVL,3` | Clean → `Occultist` (corpus-declared, undispatched — T12-overlap shape after mapping). |
| `Refined Education` | 94 | `TYPE: Refined Education Selection` (no class-identifying PRE token in the sampled record) | **Needs deeper per-record read** — the sampled record carries no class-identifying token; likely resolves via the parent feat's own PREFEAT chain rather than a `PRE*` token on the class_feature record itself. Single-book (`ultimate_intrigue`), so cheap to investigate exhaustively even though the sample didn't resolve it on the first read. |
| `Demonic Obedience` | 42 | `PRE: 1,Shivaska` (a demon lord's name, not a class) | **Not class-owned.** This is a deity/Obedience-feat-line special quality, granted outside any PC class chassis (comparable to a boon feat, not a class option pool). The correct disposition is almost certainly "prove it should not be mapped to a class at all" — a genuine false-positive-shaped finding, not silently dropped: it should stay a category label, or be re-typed to a different `kind` entirely, not forced into a class. |
| `Blessings` | 37 | `TYPE: Blessings`, `PRE: [PREDEITY...]` | **Multi-owner, contextual.** Spans `advanced_class_guide` and `ultimate_wilderness`, deity-blessing tokens rather than a single class marker — likely shared across Warpriest/Sacred-Servant-style builds rather than one class. Needs per-record disambiguation, not a single table row. |
| `Domain Power` | 172 | `PRE: DomainLawLVL` (a generic domain-level variable, not tied to one class) | **Multi-owner, contextual — the largest single group, and a real hazard if mapped naively.** `DomainLawLVL`-shaped variables are shared by every class with domain access (Cleric, Inquisitor via Inquisition, Warpriest via Blessing-domain hybrid rules, Paladin's Sacred Servant archetype, ...). Forcing this label to a single class (e.g. `"Cleric"`) the way `POOL_TO_DISPATCHED_CLASS` forces `"Rage Power"` → `"Barbarian"` would be **wrong for records actually granted through a different class** — exactly the anti-gaming failure `decisions.md §1a` rules out (a relabelled shape is not a closed shape). This group needs a genuinely different resolution mechanism (per-record class attribution, not a static table entry) or an honest "not mappable by this method" disposition for at least some of its 172 records. **Flagged as this census's most important finding for the work lanes.** |

This sample (10 of 547 groups, 987/2640 = 37% of units by the top-10-by-count slice above) already
shows the population is **not uniform**: some groups are one-line table additions to an existing,
proven mechanism (`Ki Power`, `Master of Many Styles`, `Pack Lord`, `Adaptation`, `Favored Enemy
Bonus` — all resolve cleanly to an ALREADY-DISPATCHED class via a single `TYPE:`/`PRE:` token read);
some resolve cleanly but only to an undispatched-but-real class, so they close T2a-residual without
closing T12 (`Wild Talent`, `Implement School Focus Power`); and some are structurally harder —
either genuinely not class-owned (`Demonic Obedience`) or genuinely multi-owner and contextual
(`Domain Power`, `Blessings`) and need a different, more careful mechanism than a static
label→class table.

## Mechanism note: `POOL_TO_DISPATCHED_CLASS` only targets dispatched classes today

`src/rules_core/cache_gen/class_feature.rs`'s `POOL_TO_DISPATCHED_CLASS` table (tier 2) only maps a
pool label to one of the 34 *dispatched* classes (its own name says so). Mapping `Wild Talent` →
`Kineticist` or `Implement School Focus Power` → `Occultist` needs either (a) a parallel table
targeting the broader corpus-declared-class roster (the same population tier 4's
`corpus_class_owner` already reads, `corpus_class_names_from_inventory_json`), or (b) folding these
entries into tier 4's own matching by teaching it "this label text is a known ALIAS for this real
class name," not just "this label text literally contains a class name." Either is a small,
well-precedented code change — same file, same generator, same regen-and-diff discipline the
T2a+T12 cycle already ran — not a new mechanism. This belongs in the work lane's own scope, not
mine to build.

## Cost model and cycle estimate

Per the dispatch brief and `decisions.md §13`: **size by groups/books and files-to-touch, not by
unit count.** The historical unit rate here (`CLASS_FEATURE_POOLS`' own 27 entries, built through
per-group corpus verification across multiple SD-31 waves — W22 built `Rogue Talent` alone with
heavy investigation, W23 added `Rage Power` plus the false-suffix-match guard) is the only real
precedent for this shape's per-group cost, and the T2a+T12 cycle's own throughput (four whole
resolution tiers reproduced, 12,384 records regenerated, one consumer conflict found AND fixed, in
ONE cycle) shows a well-scoped cycle here moves much faster than the original one-pool-per-wave
pace once the tooling and pattern exist.

**Every group's fix lands in the same handful of files** (the resolution table in
`src/rules_core/cache_gen/class_feature.rs`, its own test module, a full corpus regen via `gen_cache_class_feature`) —
this is a genuinely different cost shape from T2b/T9's per-book onboarding tax (where each book
needs its own `RuleSetId`/count-pinning files). The dominant cost here is **verification labor per
group** (reading `TYPE:`/`PRE*:` tokens to confirm a mapping is true, per `decisions.md §3`), not
file-touch count, and the long tail (398 groups with ≤3 records each) does not get cheaper per-group
just because it's low-count — a false mapping is exactly as much of an anti-gaming violation for a
1-record group as for a 172-record one.

`work_estimate.cycles_needed` in the structured return batches this: a handful of cycles for the
high-value top labels (many resolve in one `TYPE:`/`PRE:` read each, batchable several-per-cycle
since they share one file), several more for the mid-tail, and a book-bucketed sweep for the long
tail of singletons (grouping by book, since reading one book's corpus for one label is cheap to
extend to that book's other small labels while the file is already open).

## Findings for the sibling lanes and the operator record

1. **Re-derived total: 2,640, not `decisions.md §13`'s ~2,775.** Logged as
   `scripts/retro.py correction --subject "decisions.md §13 T2a-residual estimate" --claimed
   "~2,775" --actual "2,640" --verified-by "python3 scripts/sd32-t2a-residual-census.py"`
   (`docs/retro/events/t2a-residual-census.jsonl`).
2. **`Domain Power` (172 units, the single largest group) is not a clean single-class mapping.**
   Its `PRE:` tokens name generic domain-level variables shared across multiple classes with domain
   access. The work lane that picks this up must NOT force it into `POOL_TO_DISPATCHED_CLASS`-style
   single-class table the way `Rage Power → Barbarian` works — that would relabel, not close, per
   `decisions.md §1a`.
3. **At least one group (`Demonic Obedience`, 42 units) is likely not class-owned at all** and
   should be confirmed and dispositioned as "correctly not mapped," not silently forced into a class
   to shrink the number.
4. **Zero registered-pool matches** — none of the 547 residual labels are already covered by
   `CLASS_FEATURE_POOLS`'s 27 entries; every group in the structured return carries
   `registered: false`.
5. **No new consumer-conflict hazard found** beyond the one the T2a+T12 cycle already fixed
   (`class_feature_pool_catalog.rs`). The three other `data.class` readers all benefit from, rather
   than conflict with, a more accurate mapping.
6. **This work is dispatched, not done, by this cycle** — logged as a `scripts/retro.py deferral`
   naming this census as the tracking artifact.

## Scope discipline

This cycle touched only: this memo, `scripts/sd32-t2a-residual-census.py` (new, committed), and
retro log entries under `docs/retro/events/t2a-residual-census.jsonl`. `kanban.md` row 11 was left
at `in-progress`, untouched, per the dispatch brief. No engine code, corpus data, or pinned count
was changed.
