# Cycle 1 — Gate 3 (closure invariant) / Card 11 `epic-2-cause-closure`, lane T2a+T12 (combined)

- **Card ID:** `epic-2-cause-closure` (shared row, six concurrent lanes — this receipt covers ONLY
  the T2a+T12 combined lane; see `progress.md`'s other Cycle-1-lane receipts for the sibling shapes)
- **Commit SHA:** `985e24c1e` (landed on `tranche/12`; the code+data fix commit. The receipt/
  kanban/progress bookkeeping commit landed as `a255eeba7`.)
- **Files touched:**
  - `src/rules_core/cache_gen/class_feature.rs` (the cause site: `generate()`'s `class`-derivation
    chain, extended with two new resolution tiers before the raw key-prefix fallback; +9 new tests)
  - `src/bin/gen_cache_class_feature.rs` (threads the new `corpus_class_names` argument)
  - `src/rules_core/class_feature_pool_catalog.rs` (consumer fix — see "A load-bearing consumer
    conflict, found and fixed" below)
  - `apps/desktop/src-tauri/src/class_feature_descriptions.rs` (one pre-existing test assertion
    updated to the now-correct value — see same section)
  - `data/corpus/**/class_feature/**/*.json` (12,382 regenerated records — data, not code; see
    "Regeneration scope and discipline" below)
  - `docs/retro/events/epic-2-t2a-t12.jsonl` (new — 1 correction, 1 deferral, 1 incident, 1 note)
  - `kanban.md` (card 11 → `in-progress`, row note appended), `progress.md` (this entry; Open
    blockers section corrected)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  (`git diff --unified=0 1bb523773d32705d1b7387fd4c494861523f55ba -- src/rules_core/cache_gen/class_feature.rs src/bin/gen_cache_class_feature.rs src/rules_core/class_feature_pool_catalog.rs apps/desktop/src-tauri/src/class_feature_descriptions.rs`
  — no `sd[0-9]+_`/`SD[0-9]+_`/`Sd[0-9]+`/`t_[0-9a-f]{8,}` matches; prose references to "SD-32"
  use a hyphen, which the pattern deliberately does not match)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff — no
  `STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack` tokens)
- **Acceptance criterion:** `acceptance-and-verification.md` AT-32-E2-001 — "Cause closure closes
  by class, not by instance... A cycle that closes T2a for a single class and stops is out of
  protocol; the rule is class-closure, not instance-closure." This cycle's scope per the dispatch
  brief: T2a ("`data.class` read from the wrong place") and T12 ("genuine missing engine
  mechanism"), combined per card 11's own cycle-1 receipt ("T2a and T12 need one combined cycle,
  not two independent half-measures").
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`
  `PCGEN_ORACLE_SHA`) — fresh worktree, empty oracle slot, self-healed per §8 via
  `scripts/fetch-pcgen-oracle.sh --dest <repo-local pcgen slot>`; matches the pin exactly.
- **Status:** complete (this lane's own scope — see "What this cycle closes" and "What remains
  open, honestly" below; card 11's shared row itself is left `in-progress`, not marked `complete`,
  per the dispatch brief's explicit instruction — a later consolidation cycle closes the row once
  every lane has landed)

## Re-deriving the population before trusting it (Decision §1a bar)

Card 11's own cycle-1 receipt cited `MEASURE-TWICE.md`'s wave-31 figure of **8,243** for T2a. That
figure was flagged, in this dispatch brief and in `MEASURE-TWICE.md` itself, as needing
re-examination before being credited. Re-deriving it at this cycle's start (before any code change)
found it was already stale:

```
# T2a, pre-fix, re-derived (command run against the corpus as this cycle found it):
python3 -c "
import json, glob, os
DISPATCHED = ['Barbarian','Bard','Cleric','Druid','Fighter','Monk','Paladin','Ranger','Rogue',
  'Sorcerer','Wizard','Arcanist','Bloodrager','Brawler','Hunter','Investigator','Shaman','Skald',
  'Slayer','Swashbuckler','Warpriest','Alchemist','Cavalier','Inquisitor','Oracle','Summoner',
  'Witch','Gunslinger','Ninja','Samurai','Unchained Barbarian','Unchained Monk','Unchained Rogue',
  'Unchained Summoner']
DL=[d.lower() for d in DISPATCHED]
def is_dispatched(v):
    v=v.strip().lower()
    return any(v==d or v.startswith(d+' ') or v.endswith(' '+d) for d in DL)
total=nn=disp=0
for p in glob.glob('data/corpus/*/class_feature/**/*.json', recursive=True):
    if os.path.basename(p).startswith('manifest'): continue
    try: d=json.load(open(p))
    except: continue
    data=d.get('data')
    if not isinstance(data, dict): continue
    total+=1
    c=data.get('class')
    if c is None: continue
    nn+=1
    if is_dispatched(c): disp+=1
print('total', total, 'non-null-class', nn, 'dispatched', disp, 'non-dispatched(T2a)', nn-disp)
"
# -> total 12464  non-null-class 11485  dispatched 5807  non-dispatched(T2a) 5678
```

The 8,243 → 5,678 shrinkage predates this cycle: `cache_gen::class_feature::generate()` already had
a grant-fact-driven correction (`true_class_by_key`, wave 22/23) that landed and was regenerated
into `data/corpus` sometime between wave 31's measurement and this cycle, but its own coverage is
partial (5 of 21 `BOOK_PRIMARY_FILES` books have no grant data at all; even where grant data exists
it only covers keys with an explicit grant fact). **Logged as
`scripts/retro.py correction --subject "epic-2-cause-closure cycle-1 receipt / MEASURE-TWICE.md
wave-31" --claimed "8,243" --actual "5,678 (pre-fix), 4,284 (post-fix)"` — see
`docs/retro/events/epic-2-t2a-t12.jsonl`.**

## Why T2a and T12 cannot close independently (re-confirmed, not just cited)

`MEASURE-TWICE.md` §2 / `sweeps.md` S20 measured the T2a/T12 overlap at "floor 1,354, scaled
~2,124" by joining T12's 2,453-unit population to the corpus on `data.key` and checking which
joined records carried a non-dispatched `data.class`. Re-running that exact join against the live
corpus (pre-fix) reproduced it almost exactly (1,564 joined, 1,354 non-dispatched) — confirming the
cited method, not just trusting the cited number. The overlap's own top values (Psychic, Vigilante
Talent, Medium, Magus Arcana, Vigilante, Magus, Shifter, Kineticist, Spiritualist — all genuinely
unmodelled classes) prove the point directly: these records sit inside T2a's "plumbing" bucket by
evidence-code shape alone, while their real content is T12's "genuine complexity." Fixing T2a's
`data.class` field WITHOUT also correctly routing these records to their real (still-unmodelled)
class name would either (a) leave them mislabeled as a random category string forever, or (b) risk
silently laundering genuine-complexity content into the plumbing bucket if a cruder fix forced every
category label toward a dispatched class regardless of truth. Both are exactly the anti-gaming
failure `decisions.md §1a` rules out. The fix below resolves both correctly, in the same mechanism,
by asking "what is the TRUE class" rather than "is this dispatched" — the dispatched/undispatched
split then falls out as a fact about the answer, not a choice made in advance.

## The cause, and the fix

`cache_gen::class_feature::generate()` (the generator that writes `data/corpus/*/class_feature/*.json`
from raw `.lst` rows) derived `data.class` from `true_class_by_key` (grant facts) when available, and
from **the corpus key's own group-prefix text, verbatim, with no validation** otherwise
(`"Rage Power ~ Ferocity"` → `class: "Rage Power"`). That verbatim fallback is T2a's own name for
itself: "`data.class` read from the wrong place." It is wrong whenever the group prefix is a
category label (an option pool, an archetype tag) rather than the real granting class's own name —
which `v06_work_inventory.rs`'s OWN `Kind::ClassFeature` classify arm already solves correctly, via
three already-tested resolution tiers (`class_feature_owner`, `class_feature_owner_via_type_facet`,
`class_feature_owner_via_pool_catalog`) that this generator never consulted.

This cycle reproduces (not imports — `v06_work_inventory.rs` is a `bin`, not a library; this
package's own disjoint-file-touch convention already establishes reproducing rather than sharing)
two of those three mechanisms locally in `cache_gen::class_feature.rs`, and adds a fourth the
census-side classifier does not need but the corpus DOES: a match against the FULL corpus-declared
class roster, not only the 34 dispatched ones. `generate()`'s resolution chain is now:

1. `true_class_by_key` — grant facts (unchanged, wave 22/23).
2. `pool_catalog_owner` — the 27-entry option-pool → dispatched-class table
   (`v06_work_inventory.rs::CLASS_FEATURE_POOLS`, reproduced verbatim with its two guards:
   cross-class-collision, verified-false-suffix-match).
3. `type_facet_dispatched_owner` — the corpus's own `"<Class> Class Feature(s)"` `TYPE:` marker,
   matched against the 34 dispatched classes.
4. `corpus_class_owner` — the key's group prefix matched against **every** corpus-declared class
   name (`corpus_class_names_from_inventory_json`, the same population `v06_work_inventory.rs`'s
   own `corpus_class_names` fact is built from — read from the already-committed
   `docs/work-inventory.json` rather than re-walking raw `.lst` `*_class.lst` files a second time).
   This is the T2a/T12-overlap fix: a record whose true owner is "Vigilante" (corpus-declared,
   engine-undispatched) now ships `data.class: "Vigilante"`, not the category label
   `"Vigilante Talent"` — correct either way the class is later modelled or not.
5. `type_facet_corpus_owner` — same as (4), tried against `type_facet`'s candidates.
6. The raw key-prefix split (unchanged fallback, for whatever none of 1-5 resolves).

Directory placement is **unchanged** — still keyed on the raw key-owner segment, exactly as before
this cycle and as the existing `generate_writes_the_true_class_not_the_key_prefix_guess` test already
pins. Only the `class` field's VALUE moves.

## RED→GREEN, twice (proving the mechanism, not just describing it)

Two new end-to-end tests (`generate_writes_the_pool_catalog_owner_for_an_unregistered_grant_key`,
`generate_writes_the_corpus_declared_undispatched_owner_for_the_t2a_t12_overlap`) each run the WHOLE
`generate()` pipeline against a real fixture row with no grant fact, proving tier 2 and tier 4
respectively. For each, the load-bearing `.or_else(...)` line was mutated to `.or_else(|| None::<String>)`
and re-run:

```
cargo test --locked --lib cache_gen::class_feature::tests::generate_writes_the_pool_catalog_owner_for_an_unregistered_grant_key
# FAILED: left: Some("Rage Power")  right: Some("Barbarian")   -- failed for the intended reason
cargo test --locked --lib cache_gen::class_feature::tests::generate_writes_the_corpus_declared_undispatched_owner_for_the_t2a_t12_overlap
# FAILED: left: Some("Vigilante Talent")  right: Some("Vigilante")   -- failed for the intended reason
```

Both lines reverted; both tests green again. Full module suite: 27/27 pass (9 new). Full lib suite:
2,376/2,376 pass, 0 failed, 13 ignored (unchanged from before this cycle). Full desktop `--bin
codex-desktop` suite: 516/516 pass, 0 failed (see next section for why this run mattered).

## A load-bearing consumer conflict, found and fixed

Regenerating the live corpus and re-running the full desktop suite (not just the lib suite) surfaced
a real defect this cycle's own change would otherwise have shipped: `class_feature_pool_catalog.rs`
(the `Rogue Talent` / `Rage Power` level-up option picker, `SD31-W22-POOLMEMBER-001`) filtered
records by `data.class == "Rogue Talent"` / `data.class == "Rage Power"` **literally** — the exact
category-label strings T2a's fix removes. Two tests failed with `expected at least 10 real Rogue
Talent options, got 0`. This is the deepest evidence T2a names a real defect: one field
(`data.class`) was being asked to mean two incompatible things — "the real granting class"
(`class_feature_descriptions.rs`'s join target) and "the raw pool category" (this picker's filter
key) — and the ambiguity was silently tolerated only because the OLD, buggy behavior happened to
serve both by accident for these two pools specifically.

Fixed at the actual point of ambiguity, not by reverting the correction: `class_feature_pool_catalog.rs`
now derives its pool-group filter from the corpus `key`'s own `" ~ "`-split prefix (untouched by this
cycle's fix — `"Rogue Talent ~ Ledge Walker"` still splits to `"Rogue Talent"` exactly as before),
never from `data.class`. `REGISTERED_POOL_GROUPS`'s doc comment and `PoolCatalogEntry.pool_group`'s
doc comment updated to say so. Re-ran the full desktop suite: 516/516 pass, including both
previously-failing tests.

A second, narrower breakage: `class_feature_descriptions.rs`'s own
`loads_thousands_of_real_described_class_features_from_the_live_corpus` test hard-asserted
`class_slug == "aberrant_bloodline"` for `core_rulebook`'s `Aberrant Bloodline ~ Aberrant Form` —
the OLD, buggy value (Aberrant is a Sorcerer bloodline; the pool-catalog fix now correctly resolves
`data.class: "Sorcerer"`). This is exactly the fix this consumer exists to benefit from (its own
module doc comment: `class_slug` exists so the frontend can join it against a real
`class_feature.sorcerer.*` `ExplanationDto.id`, which `"aberrant_bloodline"` could never do). The
assertion is updated to the now-correct value, with a comment explaining why, not loosened or
deleted.

No other `data.class` consumer exists in the codebase — confirmed by `grep -rln 'class_slug\|"class":'`
across both crates and by the full desktop suite passing clean afterward (0 failures across 516
tests is strong evidence no third consumer silently broke without a test catching it).

## Regeneration scope and discipline

`cargo run --locked --bin gen_cache_class_feature` (env: `PCGEN_CORPUS_ROOT` pointed at the
repo-local pinned oracle slot, per this cycle's environment block — never
`~/workspace/repos/pcgen`) regenerated 12,384 records across the generator's 21 in-scope books
(`BOOK_PRIMARY_FILES`; `pathfinder_unchained` and `ultimate_psionics` are excluded by that table,
unchanged by this cycle — see the module's own doc comment for why). `corpus_literal_sweep` (the
`raw_tokens`-fidelity self-check) ran clean afterward: `26500 records examined ... 0 findings,
CLEAN`.

**Every regenerated file's diff was checked against its pre-image, field by field, before
committing** (per-file script comparing parsed JSON with `ingested_at`/`data.class` stripped) — not
assumed. **12,382 of 12,384 changed ONLY `class` and/or `ingested_at`** (the `ingested_at` bump is
expected and precedented — the module's own existing comment already anticipates it: "the only
expected diff against the pre-image is `data.class` (plus `ingested_at`)"). **2 files diverged in
OTHER fields too** (`key`, `description`, `raw_tokens` all differed) —
`adventurers_guide/enlightened_bloodrager/bloodline_feat.json` and
`core_rulebook/draconic_bloodline/draconic_bloodline.json`. Investigation showed these are
**pre-existing citation-line drift** (the cited `source_line` now resolves to a different real `.lst`
row than what was last committed), unrelated to this cycle's `class`-derivation fix. Both reverted
to HEAD (`git checkout --`) before committing — this cycle ships ONLY the `data.class` correction for
the other 12,382. Logged as `scripts/retro.py incident --recurrence-key
class-feature-citation-line-drift` for a future cycle to investigate.

Of the 12,382 clean regenerations, **4,936 records' `data.class` value actually changed** — the rest
regenerated byte-identical apart from `ingested_at` (their pre-fix value was already correct, e.g.
from an existing grant fact). Top transitions (`old` → `new`), all independently verifiable against
the corpus:

```
432  'Warpriest Bonus Feat'    -> 'Warpriest'      (tier 4: corpus-class group match)
179  'Ranger Combat Style Feat'-> 'Ranger'          (tier 4)
170  'Rage Power'              -> 'Barbarian'       (tier 2: pool catalog)
133  'Monk Bonus Feat'         -> 'Monk'            (tier 4)
130  'Rogue Talent'            -> 'Rogue'           (tier 2)
124  'Inquisitor Domain'       -> 'Inquisitor'      (tier 4)
109  'Druid Domain'            -> 'Druid'           (tier 4)
 82  None                      -> 'Sorcerer'        (tier 4 -- previously null)
 78  'Vigilante Talent'        -> 'Vigilante'       (tier 4 -- T2a/T12 overlap)
 57  'Magus Arcana'            -> 'Magus'           (tier 4 -- T2a/T12 overlap)
 45  'Shifter Aspect'          -> 'Shifter'         (tier 4 -- T2a/T12 overlap)
 40  None                      -> 'Medium'          (tier 4 -- T2a/T12 overlap, previously null)
```

## Final numbers — |T2a|, |T12|, |T2a ∩ T12|, |T2a ∪ T12|

**|T12|** — unchanged by this cycle, and provably so: `v06_work_inventory.rs`'s `Kind::ClassFeature`
classify arm never reads `data.class` at all (confirmed by `grep -n 'json\["data"\]\["class"\]\|data\.class'
src/bin/v06_work_inventory.rs` — zero hits). T12's own evidence code therefore cannot move from a
corpus regen that only touches `data.class`.

```
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
print(sum(1 for u in d['units'] if (u.get('evidence') or '').startswith('class_feature_of_unmodelled_corpus_class')))
"
# -> 2453
```

**|T2a|** — re-derived post-fix, same command as the pre-fix derivation above, run against the
regenerated corpus:

```
# (same script as above, re-run after cargo run --locked --bin gen_cache_class_feature)
# -> total 12464  non-null-class 11904  dispatched 7620  non-dispatched(T2a) 4284
```

**|T2a ∩ T12|** — the canonical method `sweeps.md` S20 and this card's own cycle-1 receipt cite:
join T12's keys to the live corpus on `data.key`, count how many now carry a non-dispatched
`data.class`:

```
python3 -c "
import json, glob, os
DISPATCHED=['Barbarian','Bard','Cleric','Druid','Fighter','Monk','Paladin','Ranger','Rogue',
  'Sorcerer','Wizard','Arcanist','Bloodrager','Brawler','Hunter','Investigator','Shaman','Skald',
  'Slayer','Swashbuckler','Warpriest','Alchemist','Cavalier','Inquisitor','Oracle','Summoner',
  'Witch','Gunslinger','Ninja','Samurai','Unchained Barbarian','Unchained Monk','Unchained Rogue',
  'Unchained Summoner']
DL=[d.lower() for d in DISPATCHED]
def is_dispatched(v):
    v=v.strip().lower()
    return any(v==d or v.startswith(d+' ') or v.endswith(' '+d) for d in DL)
wi=json.load(open('docs/work-inventory.json'))
t12_keys={u['corpus_key'] for u in wi['units'] if (u.get('evidence') or '').startswith('class_feature_of_unmodelled_corpus_class')}
class_by_key={}
for p in glob.glob('data/corpus/*/class_feature/**/*.json', recursive=True):
    if os.path.basename(p).startswith('manifest'): continue
    try: rec=json.load(open(p))
    except: continue
    data=rec.get('data')
    if isinstance(data, dict) and data.get('key'):
        class_by_key[data['key']]=data.get('class')
joined=non_disp=0
for k in t12_keys:
    if k in class_by_key:
        joined+=1
        c=class_by_key[k]
        if c is None or not is_dispatched(c): non_disp+=1
print('joined', joined, 'of', len(t12_keys), '| non-dispatched (T2a n T12)', non_disp)
"
# -> joined 1556 of 2453 | non-dispatched (T2a n T12) 1509
```

(897 of T12's 2,453 keys have no `class_feature` corpus record at all under this generator's scope
— out of `BOOK_PRIMARY_FILES`'s 21 books, or cited from a nested support/PFS file this generator's
own doc comment already names as excluded. Not this cycle's population to close.)

A second, independent cross-check (starting from T2a's own residual population and matching each
`data.class` value against the corpus-declared class roster directly, rather than starting from
T12's keys) gives **1,644** — the same order of magnitude, confirming the two measurements agree;
the ~135-unit gap is explained by the two methods' different join directions (T12-driven vs.
T2a-value-driven) and is not a computation error in either.

**|T2a ∪ T12| = |T2a| + |T12| − |T2a ∩ T12| = 4,284 + 2,453 − 1,509 = 5,228.**

## What this cycle closes, and what remains open, honestly

**Closed, structurally, corpus-wide, at the cause:** the mechanism that used to ship a raw,
unvalidated category label into `data.class` — T2a's own defect shape — is fixed for every record
any of the four proven, already-tested resolution signals (grant fact, registered pool, `TYPE:`
class marker, corpus-declared class name) can resolve. This is not a sample: it is the SAME
generator that produces every record in scope, so the fix applies corpus-wide by construction, and
`corpus_literal_sweep` + the full lib/desktop test suites confirm nothing else moved. 4,936 records'
`data.class` value corrected this cycle; of those, 1,509 are the T2a/T12 overlap, now honestly
labeled with their real (still-unmodelled) class name instead of a random category string — which is
exactly what "closing T2a and T12 together, not double-counting or relabeling the overlap" means:
the SAME mechanism produces both outcomes, correctly, from the same evidence.

**Not closed — reported, not fabricated:** `|T2a| − |T2a ∩ T12| ≈ 2,775` records still carry a
category label none of the four resolution tiers can turn into a real class name without guessing
(`Domain Power` 172, `Wild Talent` 128, `Refined Education` 94, `Ki Power` 80, `Master of Many
Styles` 53, `Implement School Focus Power` 48, `Demonic Obedience` 42, and ~35 more distinct labels
at smaller counts). Each of these would need the SAME hand-verification discipline
`CLASS_FEATURE_POOLS`' own 27 entries were built through — reading each group's real corpus
`TYPE:`/`PREABILITY:` tokens to confirm a mapping is actually true (e.g. does `Wild Talent` really
always mean Kineticist across every book that prints it) before adding it, per `decisions.md §3`'s
fixture-check bar. That is real, bounded, but non-trivial future work — logged as a
`scripts/retro.py deferral`, not silently dropped and not guessed at to make the number look
smaller.

This is the same shape T1's own cycle-1 closure took: prove the CAUSE mechanism structurally, back
it with a standing test, and be honest about what a mechanism-level proof does and does not cover —
not a claim that literally every unit is individually `grounded`.

- **Discovery forwards:** none requiring a new card — the pool-catalog consumer conflict and the
  citation-line-drift incident are both logged against this cycle's own scope, not new cards.
- **Next-cycle plan:** the ~2,775-unit residual (T2b, T9, T4 remain entirely untouched by this
  lane and are owned by sibling lanes this cycle). A follow-up in this same
  `epic-2-cause-closure`/T2a-T12 lane, working through the residual category labels one group at a
  time with real corpus verification (same method as `CLASS_FEATURE_POOLS`' own construction), is
  the natural continuation if the operator wants T2a driven further toward zero before the row is
  marked `complete`.

`df -h /`: `27%` used (256G/968G), no disk pressure.
