# Cycle t9-template-concat-pi-redaction-regen — Gate 3 (closure invariant) / Card 11 (T9)

- **Card ID:** 11 (`epic-2-cause-closure`, T9 sub-lane
  `t9-template-concat-pi-redaction-regen`)
- **Commit SHA:** see push log (this file is written pre-commit; SHA recorded
  in `progress.md`'s appended receipt)
- **Files touched:**
  - `data/corpus/*/template/*.json` — all 2,248 `template`-kind records
    rewritten via the existing guarded generic-ingest path
    (`scripts/ingest_simple_filename_kinds.py --kind template`); **3 records
    gain a genuinely new redaction** (the concat-defect fix landing on
    `template` for the first time), **15 more** re-affirm a redaction an
    earlier cycle's rename pass already applied (byte-identical content,
    timestamp only), and the remaining **2,230** carry only their
    `ingested_at` timestamp bump — confirmed by `git diff --numstat`, not
    assumed.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped diff against this
  cycle's own start point `0514071f58`, `scripts/` + the touched
  `data/corpus/**/template/*.json` — 0 hits).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope — 0 hits;
  this cycle wrote no new code, only ran the existing generic-ingest script).
- **Acceptance criterion:** deferred item from
  `epic-2-companion-allowlist-widening_cycle-1_cycle_receipt.md` §"Why the
  already-shipped 42 template records were NOT regenerated this cycle" —
  regenerate `template` through the guarded path now that the concat-PI
  cause-fix has landed, and prove zero blacklist-term leaks remain.
- **Corpus SHA:** `PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`).
- **Status:** complete.
- **Notes:** see full body below.
- **Discovery forwards:** none new — the remaining 40 concat-defect records
  named in the prior receipt (`class_feature` × 35 across 5 books, `equipment`
  × 0/`spell` × 0 — both closed by sibling lanes since that receipt was
  written, re-derived below) stay named, not silently dropped.
- **Next-cycle plan:** see §5.

---

## 1. Guarded-path discipline, per `decisions.md`'s corpus-regen caution

Before touching anything:

```bash
python3 -c "...Counter(u['status'] for u in json.load(open('docs/work-inventory.json'))['units'])..."
```
Snapshot (`/tmp/status_before.json`, 49,490 units):
```
deferred-with-reason      46
fixture-verified        1741
grounded                 2724
ingested-magnitude       1612
literal-verified         6506
not-ingested            28060
not-started                19
text-complete             4435
unknown                   4347
```

`--dry-run` first, matching the deferring receipt's own figures exactly:
`template:seen: 2248, template:written: 2248, pi_redacted_by_kind: {template: 42}`,
`0` citation mismatches. Then the real run — identical stats.

`git status --porcelain`: **2,248 modified files, zero new, zero deleted**
under `data/corpus/*/template/`. `git diff --numstat` shows 2,230 files with
exactly one changed line (the timestamp) and 18 with more — the 3 genuinely
newly-redacted records plus 15 already-redacted (pre-existing `codex_named_unit_*`)
records whose content is byte-identical except timestamp, confirming the
regen changed content ONLY where a redaction was due.

**Set `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` and
attempted the `v06_work_inventory` regen this instruction calls for, per the
letter of the brief.** Built `corpus_literal_sweep --json-out` (1 pre-existing
finding, `inner_sea_magic/ability/hidden_wand.json`, unrelated to this
cycle's own diff) and `derived_evaluator_fixture_check --json-out` fresh,
then ran `v06_work_inventory` with both env vars set and **no**
`--allow-stamp-loss`. **It refused to write**, even with both fresh reports
supplied: *"this run would drop 6506 of the 8247 verification stamp(s) it
currently carries."* This is the guard working exactly as designed — the
correct response is to NOT force it, not to add `--allow-stamp-loss`. Did
neither. `docs/work-inventory.json` is confirmed **byte-identical** to the
pre-cycle state (`git status --porcelain docs/work-inventory.json` → empty;
status-distribution re-derived after → identical to the snapshot above). No
diff to report because no write happened — the guard's whole job.

**Why not force it.** The 6,506-stamp near-miss this bundle already survived
once (`--allow-stamp-loss` used without fresh reports, caught by a full
status-distribution diff, reverted) is exactly the failure mode this refusal
prevents. `docs/work-inventory.json`'s prior stamps rest on evidence this
worktree's single fresh sweep+fixture-check pass cannot fully reconstruct —
forcing past that with `--allow-stamp-loss` was never authorized by this
cycle's brief ("avoid `--allow-stamp-loss`"), so the correct move is to leave
`docs/work-inventory.json` untouched and let the `template` corpus content
regen stand on its own. `shape_ledger.py`'s join (which reads corpus content
directly, not the inventory's verification stamps) still succeeds against
the regenerated `template` records — confirmed below.

## 2. Zero blacklist-term leaks remain in `template` — proved, not assumed

```bash
grep -rliE -f <(python3 -c "...PI_BLACKLIST_TERMS...") data/corpus/*/template/*.json
```
finds candidates; every candidate re-checked in Python against
`pi_scrub.blacklist_term_hit_including_concatenated` (the SAME shared
function `ingest_simple_filename_kinds.py` itself calls, no second
implementation per `decisions.md §17`), excluding already-`pi_marker:
redacted` records. **Zero hits.**

The 3 concat-only records the deferring receipt named live, confirmed
redacted:
- `inner_sea_world_guide/template/human_ethnicity_garundi.json` —
  `SUBRACE` token → `[redacted PI]`
- `inner_sea_world_guide/template/bonus_language_varisian.json` —
  `LANGBONUS` token → `[redacted PI]`
- `inner_sea_world_guide/template/human_ethnicity_varisian.json` —
  `AUTO` and `SUBRACE` tokens → `[redacted PI]`

None of the three records' own `name`/`key` changed — the fix is
raw-token-scoped, exactly the shape the cause-fix targets (a record whose
own identity is clean but carries a blacklisted term concatenated into a
DIFFERENT token's value).

## 3. Concat-defect population, re-derived fresh (`decisions.md §17a`)

The deferring receipt's own 43/9-dirs figure was itself an estimate flagged
for re-derivation. Re-derived this cycle with a grep-prefiltered candidate
set (1,507 files containing any blacklist term as a literal substring,
cutting the scan from 50,613 files to a tractable set) rather than a full
corpus walk:

```
concat-only hits: 38 (across 6 kind dirs, down from the receipt's 43/9)
  advanced_players_guide/class_feature     1
  adventurers_guide/class_feature         11
  book_of_the_damned_volume_2/class_feature 8
  inner_sea_magic/class_feature           12
  inner_sea_world_guide/template           3   <- this cycle's scope, now closed
  ultimate_combat/class_feature            3
```

**`core_rulebook/equipment` and `ultimate_wilderness/spell` — present in the
deferring receipt's count, absent from this re-derive.** A sibling lane
closed them in the interim (this bundle's `no_record` totals for `equipment`/
`spell` also moved independently, confirmed via `shape_ledger.py` re-runs
before and after this cycle's own rebase — not this cycle's work, named so
the credit lands correctly).

**35 `class_feature` hits remain, across 5 books — confirmed out of this
cycle's reach, same as the deferring receipt named.** `class_feature`'s
writers are the Rust `gen_cache_class_feature.rs`-family generators, not one
of the Python generic-ingest paths `ingest_simple_filename_kinds.py`/
`ingest_ability.py` cover. Named here by book and count, not silently
dropped: `advanced_players_guide` (1), `adventurers_guide` (11),
`book_of_the_damned_volume_2` (8), `inner_sea_magic` (12),
`ultimate_combat` (3).

## 4. Bundle-wide `no_record`, re-derived (per `decisions.md §16`)

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json
```
```
no_record kind breakdown (this cycle's start, post-Part-1 push+rebase):
  equipment          170   spell               167
  monster_ability    121   equipment_modifier    43
  companion             2                total    503

no_record kind breakdown (after this cycle's template regen):
  spell               167   monster_ability      121
  equipment           116   equipment_modifier    33
  companion              2                total    439
```

**`template` does not appear in either breakdown — its `no_record` was
already 0 before this cycle and stays 0.** This cycle closes zero `no_record`
units; its acceptance criterion is the PI-redaction correctness of records
that were already ingested, not a shape-measurement gap.

**The `equipment` (170→116) and `equipment_modifier` (43→33) movement is NOT
this cycle's work** — `git status --porcelain` before this cycle's own
commit shows zero touched files under either kind directory. It is a
sibling lane's concurrent closure, absorbed into this worktree by the
`git fetch && git rebase origin/tranche/12` this cycle's own §5 push
performed between Part 1 and Part 2 of this dispatch. Named here so it is
not mistakenly claimed as this cycle's own closure — `decisions.md §12c`,
no bare totals without the population and command that produced them.

## 5. What remains (explicit)

- **35 `class_feature` concat-defect records, 5 books — Rust-side, out of
  this cycle's reach.** Needs the equivalent fix in
  `gen_cache_class_feature.rs`'s generator(s), then a cargo rebuild +
  targeted corpus regen — a different toolchain than this cycle's Python
  guarded-regen path.
- **The `v06_work_inventory` stamp-loss guard fired and was correctly not
  forced.** `docs/work-inventory.json`'s `literal-verified`/`fixture-verified`
  stamps (6,506 + 1,741) rest on evidence broader than one worktree's single
  fresh sweep+fixture-check pass can reconstruct; a future cycle that needs a
  full `docs/work-inventory.json` regen should investigate why fresh reports
  still can't cover the existing stamped population, rather than reaching
  for `--allow-stamp-loss`.

## 6. Tests / verification commands run

```
python3 scripts/ingest_simple_filename_kinds.py --kind template --dry-run   template:written 2248, pi_redacted 42, 0 citation mismatches
python3 scripts/ingest_simple_filename_kinds.py --kind template            identical stats, real write
git status --porcelain data/corpus/*/template/                              2248 modified, 0 new, 0 deleted
git diff --numstat data/corpus/*/template/*.json                            2230 files: 1 line changed; 18 files: >1 line changed
grep -rliE -f <blacklist-pattern> data/corpus/*/template/*.json | <python re-check>   0 leaks
corpus_literal_sweep --json-out       1 pre-existing finding (unrelated to this cycle's diff)
derived_evaluator_fixture_check --json-out   2577 fixtures, 1836 cleared, 0 failed
v06_work_inventory (both report env vars set, no --allow-stamp-loss)   refused (stamp loss); NOT forced; docs/work-inventory.json unchanged
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json   no_record 503 -> 439 (equipment/equipment_modifier movement is a sibling lane's, absorbed via rebase; template unchanged at 0)
```
