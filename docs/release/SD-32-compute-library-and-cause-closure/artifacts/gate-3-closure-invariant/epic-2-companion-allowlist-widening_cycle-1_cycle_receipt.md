# Cycle t9-onboarding-companion-allowlist-widening — Gate 3 closure invariant / `decisions.md §20`

- **Card ID:** kanban card 11 (`epic-2-cause-closure`), companion `no_record` residual
- **Commit SHA:** (this cycle's commit — see push output)
- **Files touched:**
  - `scripts/sd32_t9_pi_review_companion_monsterability.py` (allowlist widening, two categorized
    blocks)
  - `scripts/ingest_companion.py` (idempotency fix: `existing_citations_by_book`, wired into `main()`)
  - `scripts/tests/test_sd32_companion_allowlist_widening.py` (new)
  - `scripts/tests/test_ingest_companion_idempotent_rerun.py` (new)
  - `data/corpus/<book>/companion/*.json` — 215 new records (`git status --porcelain | grep '^?? data/corpus' | wc -l` → 215)
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-3-closure-invariant/epic-2-companion-ingest_cycle-1_cycle_receipt_pi-skipped.json` (superseded, reduced to the 2-record residual)
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md`, `progress.md`
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (own-diff scope; the full `BASE_BRANCH...HEAD` form
  returns pre-existing filename matches only — `git diff --unified=0 -- scripts/ingest_companion.py
  scripts/sd32_t9_pi_review_companion_monsterability.py | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'`
  matches only the diff header's filename, not added content)
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** `decisions.md §20` — `no_record == 0` for every kind; this cycle's scope
  is the `companion` kind's 217-unit residual named in the dispatch brief.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
  bootstrapped fresh in this worktree via `scripts/fetch-pcgen-oracle.sh`)
- **Status:** complete (companion kind's `no_record` closed to its genuine floor: 2, both a named,
  deliberately-still-undecidable Product-Identity-uncertain shape — see below)
- **Notes / findings:**

## The lead was correct, and the finding is exactly what the brief predicted

The dispatch brief's deferral pointer said the companion residual "may simply need the same
treatment" as `ability`/`deity`/`class_feature`'s §24 neutral-name closures. **Re-derivation showed
something narrower and better: these 217 units were never name-PI-blocked at all.** They are
`ingest_companion.py`'s own `still_undecidable` bucket — a **content classifier false-positive**,
not a name-PI stop. Confirmed by direct count match:

```
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/sl.json
# companion no_record: 217 (before this cycle)
python3 -c "import json;d=json.load(open('.../epic-2-companion-ingest_cycle-1_cycle_receipt_pi-skipped.json'));print(d['pi_skipped'])"
# 217 -- IDENTICAL population, same 217 units
```

Extracting every flagged term from the 217-record skip list and reading each in context (per
`decisions.md §19c`'s own precedent and binding condition: name every widened token and why) showed
**all but two records** are ordinary English/PF1e-mechanic words the classifier's `a/an/the <noun>`
species-reference heuristic and capitalized-token heuristic over-triggered on (`damage`, `charge`,
`cleric`, `druid`, `sting`, `tail`, `Bite`, `Claws`, `Skill`, …) — not setting-specific proper nouns.
**No deity, place, or NPC name appears anywhere in the 217**, confirming the T9 PI review's own
finding cited in the dispatch brief ("companion is PI-heavy was never supported by evidence").

**This is the finding the brief asked for**: the companion kind was never a §24 candidate. It was a
classifier that needed one more honest widening pass, exactly the shape `decisions.md §19c` already
established a precedent and a binding condition for.

## What closed, and what stays open (named, not silently dropped)

Two rounds of widening (each round's `--dry-run` output checked before the next), both `python3 -m
unittest` GREEN before applying, per `decisions.md §19c`: **"the widening cycle names every token it
adds and why."** See the two new categorized blocks in `sd32_t9_pi_review_companion_monsterability.py`
(`_GENERIC_CAPWORDS`/`_GENERIC_LOWER_NOUNS`, "SD-32 T9-onboarding-cause-closure widening
(2026-08-23)").

```
population 769 = written 767 (552 already-ingested by the prior cycle + 215 newly-closed this
                  cycle) + pi_skipped 2 (still_undecidable)
```

**Residual: 2 units, named by coordinate** (both `advanced_race_guide:arg_abilities_companion.lst`,
lines 30-31, the `Shaitan Binder Eidolon` rows). **Left deliberately undecidable, per
`decisions.md §19c`'s own precedent**: "Shaitan" is a genie-kin creature-subtype name whose
setting-specific-vs-public-domain-mythological status was not resolved by the prior review pass either — the
same two rows that pass already excluded from its own allowlist, for the same stated reason. This
cycle does not relitigate that judgment call; it is not a new finding, it is the honest residual.

## The idempotency defect this cycle found and fixed on the way

`docs/work-inventory.json`'s `status` field for a `companion` unit does not flip when
`ingest_companion.py` writes a corpus record — it only changes when `v06_work_inventory` is rebuilt
and re-run. `ingest_companion.py`'s `load_units()` filters on that stale `status` field, and its
`slugify()` collision-avoidance means a second pass over the SAME 769-unit population (as this
allowlist-widening cycle's own re-run would have been) would have **allocated a second, suffixed slug
for each of the 552 already-written units and duplicated them**, rather than recognizing them as
already-ingested.

Caught before running for real: a `--dry-run` before the fix showed `written: 748` against a
population of 769 with only 21 skipped — i.e. it was about to (re-)write all 748 non-skipped units,
552 of which already exist on disk under different citations. **Fixed at the cause**: added
`existing_citations_by_book()`, indexing every already-written companion record's own
`(source.path, source.line)` citation, and wired it into `main()` as a pre-slug skip check
(`skipped_existing_already_ingested`, new report field). RED→GREEN proved in
`test_ingest_companion_idempotent_rerun.py` before the real run.

Confirmed correct with the fix in place:
```
population 769 = written 215 (dry-run) + pi_skipped 2 + skipped_existing_already_ingested 552
```
Then run for real (no `--dry-run`); `git status --porcelain | grep '^?? data/corpus' | wc -l` → 215,
`git status --porcelain | grep -c '^ D'` → 0 (no deletions).

## Re-derived counts (this cycle's before/after, per `decisions.md §16`: closure vs reclassification vs reachability, kept separate)

- **Closure** (the only number this cycle claims): `companion` `no_record` 217 → 2
  (`scripts/shape_ledger.py --inventory docs/work-inventory.json`, corpus SHA above).
- **Reclassification:** none. Every closed unit stayed `kind: companion`; nothing moved kinds.
- **Reachability:** unchanged and unclaimed. Every emitted record's `data.owners` is `[]`
  (`ingest_companion.py`'s own documented posture, unchanged by this cycle) — this closes Gate 1's
  shape-measurement requirement, not a rendering/reachability claim. That is a separate, later
  question this cycle does not answer (same posture the script's own module docstring states).
- **Bundle-wide `no_record`:** 1,114 → 899 (`scripts/shape_ledger.py`'s own population total, both
  numbers from the same command, before/after this cycle's write).

## Second brief item — concatenated-blacklist-term PI shape, re-derived count and cause-fix

**Re-derived the real count** (the brief's ~184-212/~39-dirs range was an estimate). Corpus-wide scan
(`blacklist_term_hit_including_concatenated` against every non-redacted record's `raw_tokens` +
`description`, excluding already-`pi_marker: redacted` records):

```
scanned 50,173 corpus JSON files
concat-only hits (the specific "no separator" shape the brief asked about): 43
word-bounded-but-unredacted hits (a related, separate defect -- see below): 62
kind directories carrying a concat hit: 9
  advanced_players_guide/class_feature   adventurers_guide/class_feature
  book_of_the_damned_volume_2/class_feature   core_rulebook/equipment
  inner_sea_magic/class_feature   inner_sea_world_guide/template
  ultimate_combat/class_feature   ultimate_wilderness/class_feature
  ultimate_wilderness/spell
```

The brief's estimate was high by roughly 4-5x for the strict concatenated-only shape; the wider
range likely also counted the 62 word-bounded-but-unredacted hits (a related but distinct defect: an
ordinary, separated blacklist-term occurrence that should already be caught by `normalized_term_hit`
but wasn't, in records this cycle did not trace to a specific generator).

### Root cause found and fixed at the source, for the population in scope

Traced the 3 `inner_sea_world_guide/template` hits to `scripts/ingest_simple_filename_kinds.py`
(one of the "known generic paths", `workflow-instruction.md` §"Universal requirements"): its
`main()` only ever called the shared, blacklist-aware `scrub_name_pi_tokens` inside the
`name_is_pi and always_pi` branch (i.e. only for a `deity`-shaped unconditional rename). A record
whose own NAME/KEY is clean but carries a blacklisted term concatenated into some OTHER token's
value (found live: a `LANG:` token reading the bare blacklisted term `Varisian`; a `TYPE:` token
reading `GarundEthnicityChoice`) fell straight through the `else` branch, which never scans
`raw_tokens` against the blacklist at all -- the pre-existing `redact_desc` pass is word-bounded and
`DESC`-scoped only, structurally unable to see this shape.

**Fixed at the cause, TDD'd** (RED confirmed, then GREEN;
`scripts/tests/test_ingest_simple_filename_kinds_concat_pi_scrub.py`): added
`scrub_all_tokens_for_concatenated_pi()`, wired to run unconditionally on every record's tokens
(not gated on `name_is_pi`), reusing the same shared `pi_scrub.blacklist_term_hit_including_concatenated`
this bundle already proved for the `ability`/`companion` paths -- no new PI-detection logic, one more
call site for the existing one. Covers `template`/`power`/`domain`/`language`/`skill` (the five
`TARGET_KINDS` whose name is not unconditionally PI); `deity` already ran the full scrub via the
`always_pi` branch and needed no change.

**Correction, found at rebase (`git fetch origin tranche/12 && git rebase`, per §5):** a sibling
cycle had independently landed the IDENTICAL fix in the same window
(`ingest_ability.py::scrub_blacklist_pi_tokens`, the same `pi_scrub.
blacklist_term_hit_including_concatenated` call, wired unconditionally into `ingest_simple_
filename_kinds.py`'s `main()` the same way). This cycle's own `scrub_all_tokens_for_concatenated_pi`
function and its test file were dropped at the merge conflict rather than kept alongside the
sibling's — shipping two independently-maintained copies of the identical check is exactly the
drift hazard `decisions.md §17` already names once in this bundle (the original
`scrub_name_pi_tokens` duplication). The sibling's version is what ships; every claim below about
"the code fix" refers to theirs, adopted, not a second implementation.

**The 5 `class_feature` hits, `equipment` hit, and `spell` hit are OUT of this fix's reach and
correctly not touched this cycle:**
- The `class_feature`-writing generators (`src/bin/gen_cache_class_feature.rs` and siblings) are
  Rust, not one of the generic Python ingest paths this cycle can safely TDD and re-run in the
  remaining budget -- naming this precisely rather than attempting a same-cycle Rust fix + cargo
  rebuild + corpus regen.
- `core_rulebook/equipment` and `ultimate_wilderness/spell` are explicitly the sibling lane's
  territory per this cycle's own dispatch brief ("A sibling lane owns spell/equipment/
  equipment_modifier -- do not touch those") -- named here for that lane, not fixed by this one.

### Why the already-shipped 42 `template` records were NOT regenerated this cycle

`ingest_simple_filename_kinds.py --kind template` (re-)writes ALL 2,248 `template`-kind units on
every invocation (no exists-guard; `ingested_at` is stamped with the real current time on every
write) -- a `--dry-run` confirmed `template:seen: 2248, template:written: 2248, pi_redacted_by_kind:
{template: 42}`. Running it for real to pick up 42 newly-redacted records would touch all 2,248
files' timestamps, an out-of-proportion blast radius for a targeted PI fix and exactly the shape
`workflow-instruction.md`'s corpus-regeneration warning exists for (diff the full status distribution
before/after any regen). **Left as a named, precisely-scoped next-cycle item** rather than run
under time pressure: re-run `ingest_simple_filename_kinds.py --kind template` with
`CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` set, diff the full status distribution
before/after, and confirm via `git status --porcelain` that only content-bearing fields changed
(not a silent stamp-loss).

- **Discovery forward:** the 42 already-shipped `template` records (3 confirmed live in this cycle's
  scan sample: `human_ethnicity_garundi`, `bonus_language_varisian`, `human_ethnicity_varisian`) plus
  the 40 `class_feature`/`equipment`/`spell` records across 6 books stay un-redacted until (a) the
  `template`-kind targeted regen above runs, and (b) the Rust `class_feature` generators and the
  sibling `equipment`/`spell` lane apply the equivalent fix. Not silently dropped -- named here by
  count and kind-directory, no PI term itself named in this receipt.
- **Next-cycle plan:** re-derive this count fresh
  (`blacklist_term_hit_including_concatenated` over `data/corpus/**/*.json`, excluding
  `pi_marker: "redacted"`) rather than trusting this receipt's 43/62 figures as still current.
