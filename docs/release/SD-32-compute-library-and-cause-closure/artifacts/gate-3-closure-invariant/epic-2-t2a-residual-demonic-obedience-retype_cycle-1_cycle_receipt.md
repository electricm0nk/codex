# Cycle 1 — Gate 3 (closure invariant) / Card 11 `epic-2-cause-closure`, lane T2a-residual `Demonic Obedience` re-type (`decisions.md §23b`)

- **Card ID:** `epic-2-cause-closure` (shared row — this receipt covers ONLY the `Demonic Obedience`
  re-type lane named in `decisions.md §23b`; card 11 stays `in-progress`, not marked `complete`, per
  the dispatch brief's instruction — sibling lanes close the row's other shapes)
- **Commit SHA:** `42f77f8ac` (the fix), `0247407bc` (rebase-discovered `"AG"` equipment-book-code
  addendum) — both pushed clean to `tranche/12` after two rebases.
- **Files touched:**
  - `src/bin/v06_work_inventory.rs` — the cause site: `refine_kind`'s new `Kind::ClassFeature` arm
    (deity-obedience-only-prerequisite rows whose group names no corpus-wide PC class reclassify to
    `Kind::Feat`), a new corpus-wide `corpus_pc_class_names` set (unioned from every book's own
    `book_pc_class_names`, computed before enumeration) threaded through `enumerate_book`/
    `enumerate_file`/`refine_kind`, 4 new tests (`refine_kind_class_feature_deity_obedience_tests`)
  - `docs/work-inventory.json` — regenerated through the real producer
    (`cargo run --locked --bin v06_work_inventory`, `CORPUS_LITERAL_SWEEP_REPORT`/
    `DERIVED_FIXTURE_CHECK_REPORT` set from a live `corpus_literal_sweep`/
    `derived_evaluator_fixture_check` run — the guarded, no-`--allow-stamp-loss` path)
  - `data/corpus/book_of_the_damned_volume_2/class_feature/demonic_obedience/demonic_obedience-{2..43}.json`
    — 42 stale files deleted (no longer `class_feature` kind; `demonic_obedience.json`, the
    no-`PREDEITY` chassis-marker "Demonic Obedience Base" record, is untouched and correctly stays
    `class_feature`)
  - `docs/retro/events/t9-onboarding.jsonl` (this cycle's retro events — 1 incident, 1 note)
  - `kanban.md` (card 11 row note appended), `progress.md` (this entry appended)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  (`git diff --unified=0 HEAD -- src/bin/v06_work_inventory.rs` — no
  `sd[0-9]+_`/`SD[0-9]+_`/`Sd[0-9]+`/`t_[0-9a-f]{8,}` matches)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff — no
  `STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack` tokens)
- **Acceptance criterion:** `decisions.md §23b` — "Ruled: re-type it... The closure is a `kind`
  correction, not a class mapping... This is a `§16` movement, so it binds: a unit re-typed out of
  `class_feature` is not a unit closed. Name the kind it moves to, prove the move, and report closure
  and reclassification as separate numbers."
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`
  `PCGEN_ORACLE_SHA`) — fresh worktree, empty oracle slot, self-healed per §8 via
  `scripts/fetch-pcgen-oracle.sh --dest <repo-local pcgen slot>`; matches the pin exactly.
- **Status:** complete (this lane's own kind-correction scope only — see "What this cycle closes and
  what it does not" below; card 11's shared row stays `in-progress`)

## Re-confirming the 42-unit premise before moving anything (§17a)

The dispatch brief's cited evidence (`epic-2-t2a-residual-alias-tier_cycle-1_cycle_receipt.md`)
claimed 42 units, all `PRE:` tokens naming a demon lord. Re-derived fresh against the pinned oracle
rather than trusted:

```
python3 -c "
import json, glob
files = sorted(glob.glob('data/corpus/book_of_the_damned_volume_2/class_feature/demonic_obedience/*.json'))
no_pre = []
demon_lords = set()
for f in files:
    d = json.load(open(f))
    toks = d['data']['raw_tokens']
    key = d['data']['key']
    if key == 'Demonic Obedience Base':
        continue
    pre_deity = [t['value'] for t in toks if t['key']=='PREDEITY']
    other_pre = [t['key'] for t in toks if t['key'].startswith('PRE') and t['key'] != 'PREDEITY']
    if not pre_deity:
        no_pre.append(key)
    if other_pre:
        print('HAS OTHER PRE TOKEN:', key, other_pre)
    else:
        demon_lords.add(pre_deity[0].split(',')[-1])
print('total non-base records:', len(files)-1)
print('records with no PREDEITY:', no_pre)
print('distinct demon lords:', len(demon_lords))
"
# -> total non-base records: 42; records with no PREDEITY: []; distinct demon lords: 42
```

**Confirmed exactly.** 43 corpus files exist under `demonic_obedience/`: the "Demonic Obedience Base"
chassis marker (`TYPE:Internal`, `DEFINE:DemonicObedienceApply|0`, no `PREDEITY`, no other `PRE*:`
token — a generic apply-bonus tracker shared across every lord, genuinely not deity-specific) plus
42 `"Demonic Obedience ~ <Lord>"` records, every one carrying exactly one `PREDEITY:` token naming a
demon lord (`Shivaska`, `Jubilex`, `Baphomet`, `Orcus`, `Lamashtu`, ... 42 distinct names) and **zero**
other `PRE*:` tokens — no `PRECLASS:`, no `PRELEVEL:`, nothing that could imply class membership. No
record's premise failed; the ruling's premise holds for all 42.

## The correct target kind: `feat`, checked rather than assumed

The base "Demonic Obedience" feat (the thing a character actually takes) already exists as a real
`Kind::Feat` corpus record: `data/corpus/book_of_the_damned_volume_2/feat/demonic_obedience.json`,
sourced from `botd2_feats.lst` (a genuine `CATEGORY:FEAT` row), and is already listed in
`feat_gap_tables.rs` (`RuleSetId::Botd2, "Demonic Obedience"`, `src/rules_core/rules_tables/feat_gap_tables.rs:647`).
The 42 units are the per-deity variant benefits of that SAME feat — PCGen files them as
`CATEGORY:Special Ability` rows in `botd2_abilities_classes.lst` (hence the stray `class_feature`
typing), not as separate `CATEGORY:FEAT` rows, but they are semantically the feat's own per-lord
selection text: comparable to a boon feat's chosen-deity benefit, not a class feature. No other kind
in the `Kind` enum (`race`, `spell`, `equipment`, `monster`, `companion`, `skill`, `template`,
`deity`, `power`, `domain`, `language`, `ability`) fits this shape better — `deity`/`domain` are
declared-entity kinds (the deity/domain itself, not a benefit granted by worshipping one), and
`power` is reserved for `_powers.lst`-filename content, a different corpus shape entirely. `feat` is
the correct target: re-typing these 42 alongside their own feat's existing record groups the whole
mechanic under one kind.

## The classifier fix, and why it is generic (not a 42-record exclusion)

`refine_kind`'s new `Kind::ClassFeature` arm (`src/bin/v06_work_inventory.rs`) reclassifies to
`Kind::Feat` any `_abilities_class.lst` row whose ONLY prerequisite token is `PREDEITY:` (naming a
deity, never a class) **and** whose `KEY:` group prefix (the text before `" ~ "`) does not name any
PC class in the CORPUS-WIDE class roster — not just the enumerating book's own roster, because the
book this shape lives in (and every other deity-obedience book like it) typically declares no
`*classes*.lst` of its own at all (`book_pc_class_names`'s own doc comment). A new
`corpus_pc_class_names` set, unioned from every book's `book_pc_class_names` and computed once
before enumeration starts, makes this possible.

**Validated against the known false-positive case before trusting the rule** (§17a): the corpus
carries 7 OTHER `class_feature` records with the identical `PREDEITY`-only prerequisite shape —
`"Ranger Combat Style ~ Kurgess/Achaekek/Besmara/Cayden Callean"` (`inner_sea_combat`),
`"Warpriest Archetype ~ Mantis Zealot"` (`adventurers_guide`), `"Cleric Archetype ~ Elder Mythos Cultist"`
(`horror_adventures`), `"Paladin Archetype ~ Sword of Valor"` (`inner_sea_magic`) — genuinely
class-owned deity-flavored features. All 7 are correctly left untouched, because their `KEY:` group
prefix embeds the real class name (`"Ranger "`, `"Warpriest "`, ...), which the corpus-wide check
catches:

```
# re-derived corpus-wide before writing the fix, confirming the discriminator holds:
python3 -c "
import json, glob
inv = json.load(open('docs/work-inventory.json'))
all_classes = {u['name'] for u in inv['units'] if u['kind']=='class'}
def matches(group, classes):
    gl = group.lower()
    return any(gl==c.lower() or gl.startswith(c.lower()+' ') or gl.endswith(' '+c.lower()) for c in classes)
for p in glob.glob('data/corpus/*/class_feature/**/*.json', recursive=True):
    d = json.load(open(p))
    toks = d['data']['raw_tokens']
    if not any(t['key']=='PREDEITY' for t in toks):
        continue
    key = d['data']['key']
    group = key.split(' ~ ')[0] if ' ~ ' in key else key
    print(matches(group, all_classes), key)
"
# -> False for all 42 "Demonic Obedience ~ *"; True for all 7 Ranger/Warpriest/Cleric/Paladin rows
```

RED->GREEN proved inline (`refine_kind_class_feature_deity_obedience_tests::
demonic_obedience_row_reclassifies_from_class_feature_to_feat`): with the reclassification arm's
condition forced to `false`, the test asserted `left: ClassFeature, right: Feat` and failed for the
intended reason; restoring the real condition turned it GREEN. All 23 existing `refine_kind`/
`file_kind` tests and the full 353-test `v06_work_inventory` suite stayed green throughout.

This closes the cause, not just the 42 instances: any future deity-obedience feat line filed under
`_abilities_class.lst` with a `PREDEITY`-only prerequisite and no class-named group will be caught by
the same rule, the next time a book like this one is onboarded.

## Both directions proven — 42 out, 42 in, nothing else moved

```
python3 diff_inventory_demonic.py work-inventory.json.before docs/work-inventory.json
# before class_feature: 18085   after class_feature: 18043   (-42)
# before feat:            2722   after feat:            2764   (+42)
# demonic obedience units before: 44  {'class_feature': 43, 'feat': 1}
# demonic obedience units after:  44  {'class_feature': 1,  'feat': 43}
# total ids that changed kind: 0 outside the demonic-obedience population
```

**Proven idempotent** (two consecutive regens of the patched binary against the same oracle and the
same fresh `corpus_literal_sweep`/`derived_evaluator_fixture_check` reports produce byte-identical
unit sets, differing only in the `generated_at` timestamp) — the 42-unit move is the classifier's
only effect.

**Corpus JSON proven clean.** `cargo run --locked --bin gen_cache_class_feature` was re-run after
deleting the 42 stale files: it did not recreate them (they no longer appear in its
`kind=="class_feature"` input) and produced no orphans. That regen also rewrote `ingested_at` on
every one of the OTHER 17,809 already-tracked `class_feature` records and materialized 4 previously
orphaned records for unrelated pre-existing citation gaps (`native_cunning/overrun.json`,
`social_grace/craft_baskets.json`, `vigilante_favored_maneuver/favored_maneuver_sunder.json`,
`green_faith_marshal/vulture.json`) — a field-by-field diff (`ingested_at` stripped) proved all
17,809 changed ONLY that timestamp; the 4 new files are pre-existing corpus/generator drift unrelated
to this fix. Both are reverted (`git checkout HEAD --pathspec-from-file=...` for the 17,809; `rm` for
the 4 untracked files) so this commit's diff carries only the 42 real deletions plus the code/
inventory change — no unrelated churn.

**Discovered, not fixed (out of this cycle's scope, forwarded):** the committed `docs/work-inventory.json`
baseline (`af2f07f68`) was itself already stale relative to a fresh regen of its OWN unmodified code
against the pinned oracle — 55 `race_trait` units (8 `advanced_race_guide`, 10 `bestiary`, 4
`bestiary_2`, 5 `bestiary_3`, 2 `bestiary_4`, 22 `pathfinder_unchained`, 1 `ultimate_intrigue`, 3
`ultimate_wilderness`) present in the baseline vanish on a fresh regen with no compensating gain in
any other kind. Proven NOT caused by this cycle's patch (the same idempotence check above; zero
commits touched `v06_work_inventory.rs` between `af2f07f68` and this cycle's pin). Logged via
`scripts/retro.py note` (`t9-onboarding.jsonl`) with the re-derive command; not investigated further
here — this cycle's own diff does not carry it (the 55 units' JSON files were never touched by this
cycle).

## `no_record` effect (`decisions.md §20` — reported, budget constants untouched)

Of the 42 reclassified units, 40 carry inventory `status: text-complete` (already outside the standing
gate's not-done population regardless of kind — a description-only completeness verdict independent
of corpus-record presence) and 2 carry `status: unknown` (`Demonic Obedience ~ Mazmezz`,
`~ Shivaska`) and ARE in the gate's population:

```
# both, BEFORE (class_feature kind): join_status = matched (real corpus JSON record existed,
# confirmed by direct inspection of the deleted files' `data.key` before deletion)
# both, AFTER (feat kind): join_status = no_record (no feat generator produces a matching record
# for this shape yet -- decisions.md §23b explicitly scopes THIS cycle to the kind correction only,
# not building that generator)
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus \
  --output /tmp/ledger-after.json
# feat kind in ledger-after.json: {'no_record': 903, 'no_formula_tokens': 275}
```

**Net measurable effect: `no_record` +2** (feat kind), attributable entirely to this reclassification.
Standing-gate check, unchanged budget constants:

```
python3 scripts/shape_coverage_standing_gate.py --inventory docs/work-inventory.json --corpus-root data/corpus
# population (not-done units considered): 35328
# unclassified: 0
# join-status split: matched=9469 no_formula_tokens=18372 no_record=7487
# no_record budget: 7487/35328 vs. baseline 21521/36028 -- exceeded: False
```

`NO_RECORD_BUDGET_COUNT`/`NO_RECORD_BUDGET_POPULATION` in `scripts/shape_coverage_standing_gate.py`
are **untouched** — huge headroom (7,487 vs a 21,521 budget), no repin needed, no
`no_record_budget_provenance.jsonl` entry required.

## A real blocker found and fixed in-scope: unmapped `"AG"` equipment book code

Rebasing onto `origin/tranche/12`'s tip (which had moved — two sibling `decisions.md §23a`/`§20`
cycles landed concurrently) surfaced a hard panic in `v06_work_inventory`'s own
`equipment_book_slug_for`, unrelated to this cycle's own diff:
`equipment_resolver::equipment_catalog_rows() now carries an unmapped book code "AG"`. This
hard-crashes the regen for every caller, blocking this cycle's own ability to prove its 42-unit
move against the current tip. Same narrow, additive-only, self-verified fix as every prior
occurrence of this exact failure mode already documented in this function's own comments
(`SD31-E6-F10-003`, `SD31-E6-F10-004`, the T9-onboarding `ISTEM`/`ISM` arms): one match-arm line,
`"AG" => "adventurers_guide"`, verified by the function's own pre-existing
`equipment_book_slug_for_covers_every_catalog_book` test. Re-running the full regen with this fix
applied changed 0 unit ids and 0 `by_kind` totals — only 159 `adventurers_guide`
`equipment`/`equipment_modifier` records' `status`/`evidence` fields, which had been silently stuck
at `not-ingested`/`equipment_key_absent_from_equipment_tables` by the crash, now resolve to their
real values (`ingested-magnitude`, `unknown`, etc.) — a correctness fix, not scope creep on this
cycle's own 42-unit move (verified: `git diff --stat HEAD -- src/bin/v06_work_inventory.rs
docs/work-inventory.json` before committing this addendum).

## `decisions.md §22` — no divergence from the oracle recorded

This cycle does not diverge from PCGen's own data; it corrects Codex's OWN kind classification of a
row PCGen itself already declares under `CATEGORY:Special Ability` in a class-abilities file (a
PCGen filing convenience, not a PCGen semantic claim that these rows are class features). No `§22`
divergence-disclosure entry is needed.

## What this cycle closes and what it does not

- **Closes:** the `kind` misclassification. 42 units correctly move from `class_feature` to `feat`.
  The classifier fix is generic and will catch the next deity-obedience line filed this way.
- **Does NOT close:** engine wiring, a corpus-cache generator for this shape, or card 11's shared
  row (still `in-progress`). Per `decisions.md §16`/this brief's item 3: **a unit re-typed out of a
  shape is not a unit closed.** Reclassification: 42. Closure: 0. These are reported as separate
  numbers, not conflated.

- **Discovery forwards:** the 55-unit `race_trait` baseline drift (see above); no new kanban card
  opened for it per this cycle's scope, named in `progress.md` for the next census/Gate-0 cycle to
  pick up if it re-derives the same result.
- **Next-cycle plan:** a future cycle building a corpus-cache generator for feat-kind option-pool
  boons (the same shape `gen_cache_class_feature` already handles for class features) would close
  the 2 `no_record` units this cycle surfaced, plus the other ~525 unverified category labels'
  eventual feat-shaped residue.

`df -h /`: recorded at end of turn, see final report.
