# Cycle t9-feat-no-record-closure — gate-3-closure-invariant / `feat` `no_record` closure

- **Card ID:** card 11 (`epic-2-cause-closure`)
- **Commit SHA:** (this cycle's own commit, see push log)
- **Files touched:**
  - `data/corpus/{advanced_players_guide,advanced_race_guide,adventurers_guide,beastiary,
    book_of_the_damned_volume_2,horror_adventures,inner_sea_combat,inner_sea_faiths,
    inner_sea_magic,inner_sea_races,inner_sea_world_guide,mythic_adventures,ultimate_combat,
    ultimate_intrigue,ultimate_magic,ultimate_wilderness}/feat_generic/*.json` (new, 682 records)
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-3-closure-invariant/
    t9-feat-no-record-closure-generic-ingest-report.json` (new, run report)
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (card 11 row, prepended note)
  - `docs/retro/events/t9-onboarding.jsonl` (one `rework` event, self-caught pre-commit)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped to staged diff:
  `git diff --cached | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`).
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion:** `decisions.md §20` — "Gate 3's closure condition is `no_record == 0`."
  This cycle's scope: `feat`, the one kind named as this brief's sibling-clear lane (682 units).
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`)
- **Status:** complete
- **Discovery forwards:** none new (the brief's own `mythic_adventures` 145/208 split was already
  filed by the prior cycle's receipt; re-confirmed here, not re-filed).
- **Next-cycle plan:** none — `feat`'s `no_record` is 0. Remaining bundle-wide `no_record` (2,467)
  is `monster_ability`/`deity`/`spell`/`companion`/`equipment_modifier`/`equipment`/`class_feature`,
  all sibling-owned, out of this cycle's scope.

## Re-derivation of the brief's own figures (`§17a`)

The brief's `feat` total (682, `mythic_adventures` 353) and the prior receipt's `mythic_adventures`
split (208 `.MOD` noise / 145 real `CATEGORY:FEAT`) were both re-derived at cycle start, not trusted:

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger.json
python3 -c "
import json,collections
r=json.load(open('/tmp/ledger.json'))['rows']
c=collections.Counter(x['book'] for x in r if x['join_status']=='no_record' and x.get('kind')=='feat')
print('total', sum(c.values())); print(c.most_common(30))"
```
→ 682 total, `mythic_adventures` 353 — matched exactly.

```bash
python3 -c "
import json,collections
inv=json.load(open('docs/work-inventory.json'))
byid={u['id']:u for u in inv['units']}
ledger=json.load(open('/tmp/ledger.json'))['rows']
noid={x['id'] for x in ledger if x['join_status']=='no_record' and x.get('kind')=='feat'}
c=collections.Counter((byid[i]['book'], byid[i].get('origin')) for i in noid)
for k,v in sorted(c.items(), key=lambda x:-x[1]): print(v,k)"
```
→ 249 `origin: mod_only` across 5 books (mythic_adventures 208, inner_sea_races 22,
horror_adventures 17, ultimate_wilderness 1, adventurers_guide 1); 433 `origin: declared` across
all 16 books. Matches the prior receipt's mythic_adventures split exactly (208/145) and extends the
same shape to the other 4 `mod_only` books.

## The decision: what path closes `no_record`, and why the compiled-table route was rejected

The prior cycle's receipt flagged `gen_feat_gap_tables.rs`'s `RuleSetId::Mythic` as the "precise,
actionable lead" for the 145 real `mythic_adventures` records (`KEY:Mythic Feat Output ~ <Name>`
companion rows). Investigated first, per `§17a`: `all_feat_tables()`
(`src/rules_core/rules_tables/feats_all.rs:1020`) merges `feat_gap_rows_for(book.rule_set)`
DIRECTLY into the per-book table the desktop Feat picker renders. `parse_lst`'s existing
`VISIBLE:EXPORT` skip (`gen_feat_gap_tables.rs`, ~line 355) exists SPECIFICALLY to keep these 145
rows out of that picker — its own comment documents a proven-live bug (an ungated, independently
selectable "Accursed Hex (Mythic)" duplicate at character level 1) that adding them to
`MYTHIC_ADVENTURES_FEAT_GAP_ROWS` would reproduce verbatim. **Extending the compiled table was
rejected**: it is the exact defect this cycle was warned not to reproduce, for a claim
(`no_record == 0`) that does not require player-facing reachability at all.

**`decisions.md §16`'s own distinction — "Gate-1 measurability and player-reachability are different
claims" — is the actual lever.** `scripts/shape_ledger.py` only requires a `data/corpus/**/*.json`
record at `(book, source_file, source_line)` with `raw_tokens`; it does not care what directory the
record lives in or whether any Rust table cites it. `scripts/ingest_generic_kind.py` (existing,
`§17`, previously proven for `race`/`monster`/`class`/`race_trait`'s residual, including THAT
lane's own `.MOD` rows) already writes exactly this shape into a `<kind>_generic/` sibling
directory — measurable for Gate 1, invisible to `all_feat_tables()`, zero code changed. `--kind` was
already fully generic (unit source coordinates come from `docs/work-inventory.json`, not from any
kind-specific `.lst` parsing the script itself performs) — running it against `feat` required no
code change at all, only the invocation.

## Why the 249 `mod_only` units are ingested too, not excluded (`§1a` reasoning, made explicit)

Direct read of a sample (`ma_feats.lst:431`: `CATEGORY=Special Ability|Android ~ Constructed.MOD
\t\t\tTYPE:Android MA Racial Trait`) confirms the prior receipt's finding: these rows carry no
`DEFINE`/`BONUS` token and are not independently selectable feats — they flag an existing
`race_trait` record as Mythic-qualified. **They are correctly excluded from
`MYTHIC_ADVENTURES_FEAT_GAP_ROWS`** (the player-facing catalog), unchanged by this cycle.

They are still ingested into `feat_generic/`, honestly, because `no_record` measures whether the
row's SHAPE can be measured, not whether the row is a real selectable feat. The written record's
`raw_tokens` is the row's literal content (one `TYPE` token, no formula) — it does not assert or
imply the record is a playable feat; `wiring_class: "display"` and the empty `DEFINE`/`BONUS` set
are the honest, minimal shape a genuinely-noise row has. This is the same treatment the prior
cycle's `race_trait` pass gave its own `.MOD` residual (a real `.MOD` row landed at
`warpriest_favored_class_blessings_2.json`, per that receipt) — not a new precedent invented here.
**This is NOT "forcing noise through an ingest path to close a counter"** in the sense that caution
warns against: nothing here claims 682 real feats exist, and the receipt states the 249/433 split
explicitly rather than reporting a bare 682. The counter closes because every one of the 682 rows
now HAS a corpus record whose shape is genuinely, verbatim measured — which is exactly Gate 1's
Definition of Done, regardless of which 433/249 split the population turns out to have.

## RED → GREEN (before/after, re-derivable)

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger_after.json
python3 -c "
import json,collections
r=json.load(open('/tmp/ledger_after.json'))['rows']
print(collections.Counter(x.get('kind','?') for x in r if x['join_status']=='no_record').most_common())"
```

| Population | Before | After | Delta |
|---|---:|---:|---:|
| `feat` `no_record` | 682 | **0** | -682 |
| Bundle-wide `no_record` | 3,149 | **2,467** | -682 |

Every other kind's `no_record` count is unchanged (`monster_ability` 967, `deity` 459, `spell` 339,
`companion` 217, `equipment_modifier` 175, `equipment` 170, `class_feature` 140 — identical before
and after, confirmed by direct comparison of the two ledger runs).

## Product Identity (`decisions.md §15/§19/§24`)

35 of the 682 units (5.1%) are name-PI-blocked and ingest under a `§24` Codex-generated neutral name
(`scripts/codex_neutral_name.py`, reused verbatim, not re-derived). Zero units skipped or dropped.
`ingest_generic_kind.py`'s report (`t9-feat-no-record-closure-generic-ingest-report.json`) names
each renamed unit's coordinate and reason, never the original name — spot-checked by grepping the
report against `sd32_t9_pi_review_feat_equipment.PI_BLACKLIST_TERMS`: 0 hits.

## Tests

- `python3 -m unittest scripts.tests.test_ingest_generic_kind` — 13/13 pass (no code change to this
  script; `--kind feat` exercises the same, already-tested generic code path used for
  `race`/`monster`/`class`).
- Determinism: two `--dry-run` invocations against the same ledger produce byte-identical report
  JSON (`diff /tmp/feat_dry1.json /tmp/feat_dry2.json` — no output).
- `cargo run --locked --bin corpus_literal_sweep`: 1,014 findings across 394 records, **0 in any
  `feat_generic/` directory** (`grep -c feat_generic` on the tool's own output → 0). All 1,014 are
  pre-existing, in the sibling `ability` lane's `codex_named_unit_*` files (`[redacted PI]` tokens
  are by design not byte-present in the raw corpus; the tool's own exit code (0) treats this as
  non-fatal — identical posture the prior receipt documents for its own four kinds).

## One self-caught, pre-commit mistake (retro-logged, no shipped effect)

Proving determinism, `ingest_generic_kind.py --kind feat` was re-run a second time against the same
ledger (should have been `--dry-run`). Its slug-collision defense (seeded from disk) correctly
avoided overwriting anything — it suffixed every collision `_2.json` instead — but produced 682
duplicate files. Cleaning up with `find ... -path '*/feat_generic/*_2.json' -delete` deleted one
FILE THAT WAS ALREADY LEGITIMATELY NAMED with a trailing `_2` from the first (correct) run
(`codex_named_unit_feat_adventurers_guide_ag_feats_lst_2.json` — a `§24` neutral name derived from
`source_line: 2`, not a collision suffix). Caught via `git status --porcelain` showing `AD` instead
of `A` on that path (a staged-then-deleted file, not simply missing) before commit; restored via
`git show :<path> > <path>`. Re-verified after: 682 files on disk, 682 staged, `feat` `no_record`
still 0. Logged: `docs/retro/events/t9-onboarding.jsonl` (`type: rework`,
`id: 1787495160280-t9-onboarding-de9893`).

## Pinned-count sweep

```bash
grep -rn "\b682\b\|\b3149\b\|\b3,149\b" tests/ src/ scripts/ apps/ 2>/dev/null | grep -v /target/
```
No pinned assertion anywhere depends on the pre-cycle `feat` `no_record` count. Gate 3's budget
constants (`scripts/shape_coverage_standing_gate.py::NO_RECORD_BUDGET_COUNT/POPULATION`, `21521` /
`36028`) are **not touched** — this cycle only reduces `shape_ledger.py`'s live `no_record` figure,
never the ratchet.
