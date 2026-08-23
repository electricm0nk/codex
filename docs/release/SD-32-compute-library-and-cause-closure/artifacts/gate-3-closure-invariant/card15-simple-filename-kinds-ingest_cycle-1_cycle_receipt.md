# Cycle card15-simple-filename-kinds-ingest — Gate 3 closure invariant / `decisions.md §20`

- **Card ID:** kanban.md card 15 (census-scope-closure), `decisions.md §20` no_record-to-zero mandate
- **Commit SHA:** (recorded after push)
- **Files touched:**
  - `scripts/ingest_simple_filename_kinds.py` (new — generic ingest, one mechanism for 5 kinds)
  - `scripts/tests/test_ingest_simple_filename_kinds.py` (new — 10 unit tests, unittest)
  - `data/corpus/<book>/{template,domain,power,language,skill}/*.json` (new — 3,124 records across
    ~35 books; regenerated through this cycle's own script, never hand-edited)
  - `docs/retro/events/t9-onboarding.jsonl` (append — 1 deferral event for `deity`, 1 derived
    verify.sh event from oracle preflight)
  - `docs/release/SD-32-compute-library-and-cause-closure/progress.md` (append)
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (rows 11, 15 left `in-progress`
    per dispatch instruction; no status field changed)

## Scope and disposition

Brief's six kinds: `template` 2,248, `deity` 459, `power` 421, `domain` 183, `language` 136,
`skill` 149 — 3,596 units, all `no_record` at baseline (re-derived, matches brief exactly):

```
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/l.json
python3 -c "import json,collections; r=json.load(open('/tmp/l.json'))['rows']; nr=[x for x in r if x['join_status']=='no_record']; print(collections.Counter(x.get('kind','?') for x in nr).most_common())"
# BEFORE: [('template',2248),('deity',459),('power',421),('domain',183),('skill',149),('language',136)]
```

**Investigated first per the brief's own instruction ("establish whether one mechanism serves
several kinds before splitting").** All six kinds share `SIMPLE_FILENAME_KINDS`
(`src/bin/v06_work_inventory.rs`, `decisions.md §17`), and `template`'s modifier-vs-object question
is **already settled** by a prior cycle's memo (`artifacts/gate-0-census-closure/
15-card-15-other-kinds-memo.md` §1, "disposition (A) — all 2,343 units are objects, not modifiers on
objects already counted, verified by 0 `.COPY=` derivations and a 0-hit join against every existing
kind"). One generic ingest mechanism serves **five** of the six kinds
(`scripts/ingest_simple_filename_kinds.py`); `deity` is a genuine sixth case — split out and
escalated, not ingested (see "What did not close" below).

## Method

1. `census_independent.py`'s own `discover_book_dirs`/`classify_scope` resolve each unit's book
   directory (reused, not re-derived) — with a `pcc_includes` fallback added (`ce_templates.lst` and
   similar shared-dependency files live in `core_essentials`'s own directory, not the including
   book's; without the fallback 1,200 of 2,248 `template` units resolved `no_file` — found and fixed
   before the real run, dry-run evidence in this receipt's history).
2. For every not-yet-ingested unit of `template`/`power`/`domain`/`language`/`skill`, re-reads the
   cited `(source_file, source_line)` from the pinned oracle and verifies the row's own leading field
   against the unit's `corpus_key` — accepting the `<group header> ~ <leaf>` composition
   `v06_work_inventory.rs` already uses elsewhere (same shape as the shipped
   `air_domain/lightning_arc.json`'s `record_key: "Air Domain ~ Lightning Arc"`). A genuine mismatch
   is **skipped and reported**, never silently written — 13 total (see below).
3. Parses the row's tab-delimited fields into `raw_tokens` — this is what makes
   `scripts/shape_ledger.py`'s join succeed.
4. PI-screens every row: PCGen's own declared `NAMEISPI:YES`/`DESCISPI:YES` tokens, plus the shared
   `PI_BLACKLIST_TERMS`/`normalized_term_hit` scan imported directly from
   `scripts/sd32_t9_pi_review_feat_equipment.py` (no second copy of the term list). A hit redacts the
   field to `"[redacted PI]"` and stamps `license: "PI-REDACTED"`; no hit stamps `"OGL"`.
5. Writes `data/corpus/<book>/<kind>/<slug>.json` in Shape B v1
   (`src/rules_core/shape_b_v1.rs::CorpusRecordV1` schema — same shape `gen_book_cache.rs`'s Rust
   generators emit; verified byte-compatible against a live shipped record,
   `data/corpus/core_rulebook/class_feature/air_domain/lightning_arc.json`, before writing).

## Result — re-derived after the write

```
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/l_after.json
python3 -c "import json,collections; r=json.load(open('/tmp/l_after.json'))['rows']; nr=[x for x in r if x['join_status']=='no_record']; print(collections.Counter(x.get('kind','?') for x in nr).most_common())"
```

| Kind | Before | After | Closed | Notes |
|---|---:|---:|---:|---|
| `template` | 2,248 | 12 | 2,236 | 12 citation mismatches, named below — inventory-vs-LST naming drift, not silently dropped |
| `domain` | 183 | 0 | 183 | full closure |
| `power` | 421 | 0 | 421 | full closure |
| `language` | 136 | 1 | 135 | 1 citation mismatch (`bestiary_2`, `D'ziriak (cannot speak)` vs `D'ziriak (understanding only, cannot speak)`) |
| `skill` | 149 | 0 | 149 | full closure |
| `deity` | 459 | 459 | 0 | **not ingested this cycle** — escalated, see below |

**Bundle-wide `no_record`: 20,889 → 17,765 (−3,124)**, re-derived at the top of this receipt's own
command against the pinned oracle
(`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`).

**The 13 named citation mismatches (0.4% of the 3,137-unit ingested scope), skipped honestly rather
than force-matched:**
```
D'ziriak (cannot speak)                        bestiary_2 / fetchling_languages.lst:6
Familiar ~ Augmented Animal                    core_rulebook / ce_templates_familiar_cr.lst:4
Kyton (Apostle)                                horror_adventures / ha_templates.lst:14
Climbing Master ~ Has Climb                    mythic_adventures / ma_templates.lst:18
Climbing Master ~ No Climb                     mythic_adventures / ma_templates.lst:19
Mythic Arugment Summoning                      mythic_adventures / ma_templates.lst:30
Mythic Simple Template ~ Agile                 mythic_adventures / ma_templates.lst:5
Mythic Simple Template ~ Arcane                mythic_adventures / ma_templates.lst:6
Mythic Simple Template ~ Divine                mythic_adventures / ma_templates.lst:7
Mythic Simple Template ~ Invincible            mythic_adventures / ma_templates.lst:8
Mythic Simple Template ~ Savage                mythic_adventures / ma_templates.lst:9
Swimming Master ~ Has Swim                     mythic_adventures / ma_templates.lst:15
Swimming Master ~ No Swim                      mythic_adventures / ma_templates.lst:16
```
Each is the inventory's own `corpus_key` diverging from the LST row's own leading field by more than
the `<group> ~ <leaf>` composition rule accounts for (e.g. inventory drops a parenthetical
qualifier, or orders "Has Climb"/"Climbing Master" the opposite way from the row). This is a
`v06_work_inventory.rs` naming-derivation question, not a PI or ingest-mechanism question — named by
exact shape and count per `decisions.md §16`, not rounded into "done."

**PI redactions applied (real, not simulated):** `template` 39, `language` 19, `domain` 2, `skill` 1
— 61 records shipped `license: "PI-REDACTED"` with the offending field replaced by the standing
`"[redacted PI]"` marker (`src/rules_core/shape_b_v1.rs::REDACTED_PI_MARKER`), `pi_field`/`pi_marker`
set. Sample verified by hand: `data/corpus/inner_sea_world_guide/language/jistka.json` — PCGen's own
`NAMEISPI:YES` on the `Jistka` language row, `data.key`/`data.name` redacted, `raw_tokens` preserved
(including the `NAMEISPI:YES` token itself, so the redaction reason stays auditable on the record).

## What did not close — `deity`, 459 units, escalated per `decisions.md §15`

**Not ingested. Land-everything-else, stop-and-report per `decisions.md §15` disposition 2, not a
silent skip and not a unilateral ruling.**

`ogl-pi-blacklist.md §2.1` names `deity`/`deity_name` as "Product Identity in CRB" **as a field
category** — but the only mechanized screen this repo has (`PI_BLACKLIST_TERMS`, 60 terms) is a
closed list covering the 20 core Golarion deities (Iomedae, Abadar, …) plus a handful of
incident-driven additions (`Jarn`, `Cayden CaiLean`, `lrori`). A `deity` record's **own row identity
is a deity's proper name in every one of the 459 units** — unlike `template`/`power`/`domain`/
`language`/`skill`, whose identity strings are game-mechanical labels the blacklist's own §2.2 table
already classifies OGL-inlinable. `deity` has **no `ogl-pi-blacklist.md §2.3` per-field judgment
entry at all** — the identical gap `decisions.md §19a` amendment 3a closed for `companion`/
`monster_ability` (802 units, "no rule exists" was the finding, not "safe") — by operator ruling, not
by an ingesting cycle's own authority.

Running this cycle's own term-list scan against the 459 deity identities, as a measurement only (no
write):
```
python3 -c "
import json,sys
sys.path.insert(0,'scripts')
from sd32_t9_pi_review_feat_equipment import normalized_term_hit
inv=json.load(open('docs/work-inventory.json'))
units=[u for u in inv['units'] if u.get('kind')=='deity']
hits=sum(1 for u in units if normalized_term_hit(u['name']))
print(len(units), 'total,', hits, 'hit the 60-term list,', len(units)-hits, 'would ship un-redacted under the mechanized screen alone')
"
```
Result: **459 total, 24 hit the term list (the core-20 names plus 4 recurrences), 435 (94.8%) would
ship un-redacted under the mechanized screen alone** — the exact exposure a per-record or per-book
review (`decisions.md §18`/`§19`'s own precedent) exists to resolve, not a number to write past.

**Retro-logged:** `docs/retro/events/t9-onboarding.jsonl`, deferral event `id
1787484957833-t9-onboarding-7c3605` — `--what`/`--reason`/`--revisit`/`--blocked-by`/`--scope`
recorded per `AGENTS.md`'s retro-logging instruction so a later cycle does not re-pay the discovery
cost.

## Verification

**Unit tests (RED→GREEN proved live):**
```
python3 -m unittest scripts.tests.test_ingest_simple_filename_kinds -v
# 10/10 OK
```
RED proof: mutated `parse_row`'s `raw_line.split("\t")` to `raw_line.split(" ")` — 4 of 10 tests
failed for the intended reason (tab-delimited fields no longer split; `KEYSTAT`/`TYPE`/`BONUS`
collapsed into one malformed token). Reverted; re-ran GREEN (10/10).

**Identifier audit** (own diff, `scripts/ingest_simple_filename_kinds.py` +
`scripts/tests/test_ingest_simple_filename_kinds.py`, against
`BASE_BRANCH=$(git merge-base HEAD origin/develop)` = `1bb523773d32705d1b7387fd4c494861523f55ba`):
`OK_NO_BUNDLE_TAGS`.

**Wired-integration audit** (same diff): `OK_NO_TOKENS`.

**Corpus SHA:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
(`scripts/pcgen-oracle-pin.env`), re-fetched fresh in this worktree via
`scripts/fetch-pcgen-oracle.sh` (a fresh worktree's oracle slot is git-ignored and starts empty, per
this cycle's dispatch instructions).

**Regeneration discipline:** every `data/corpus/**` file this cycle touches was written by
`scripts/ingest_simple_filename_kinds.py`, never hand-edited. No existing corpus file was modified
(`git status --porcelain` before commit shows only new files under `data/corpus/<book>/{template,
domain,power,language,skill}/`, confirmed no `M` lines under `data/corpus/`).

**Not run this cycle (scoped out, named honestly):** the reach-gate/reachability proof
(`apps/desktop/src-tauri/src/reach_gate.rs`). All five ingested kinds are, as of this cycle, `wiring_
class: "display"` reference data with **zero existing engine or UI consumer** — no Gate-2 engine, no
`rules_core` module, no desktop-app surface reads `template`/`power`/`domain`/`language`/`skill`
corpus records today. Reachability is a claim about a player-facing path that does not exist yet for
this content; claiming it here would be exactly the "success: true from a fake operation" shape
`workflow-instruction.md §8` names as non-self-healable. This cycle's scope, per `decisions.md §20`,
is Gate 1 shape-measurability (`no_record` → 0), which it closes for 5 of 6 kinds; Gate-2 engine work
for these kinds is a distinct, unscoped follow-on.

**Full unscoped `cargo test` NOT run** (dispatch instruction: "the full unscoped `cargo test --locked
--no-fail-fast` may never finish on this box — do not run it"). This cycle adds no Rust code, so no
Rust test suite is affected by it; the existing `cargo test --locked --bin v06_work_inventory` suite
is untouched (no `.rs` file in this cycle's diff).

- **Status:** complete (5 of 6 kinds; `deity` escalated per `decisions.md §15`, not closed — card 15
  stays `in-progress` per dispatch instruction, this is one sub-shape of it)
- **Notes:** the `pcc_includes` file-resolution fix (step 1 above) generalizes beyond this cycle's own
  scope — any future ingest of a `core_essentials`-shared file needs the same fallback; flagging so a
  sibling lane does not rediscover it.
- **Discovery forwards:** `## DISCOVERED` — none opened; the `deity` gap is filed as a `deferral`
  retro event + this receipt's own escalation, not a new backlog item (it is already card 15's own
  scope, per `decisions.md §13`).
- **Next-cycle plan:** (a) operator ruling on `deity`'s PI exposure, then re-run this same mechanism
  restricted to `--kind deity` once a blacklist §2.3 entry exists; (b) the 13 named citation
  mismatches need a `v06_work_inventory.rs` corpus_key-derivation fix, not a re-run of this script;
  (c) `class_feature`/`ability`/`race_trait`/`monster_ability`/`feat`/`spell`/`companion`/
  `equipment`/`equipment_modifier`/`class`/`monster`/`race` remain at their own `no_record` counts —
  out of this cycle's assigned scope (six named kinds), reported here only per `decisions.md §12c`'s
  "state every population, never a bare total" rule: `no_record` is still 17,765 bundle-wide.
