# Cycle t9-onboarding-equipment-ue-gap-routing — Gate 3 closure invariant / `decisions.md §20`

- **Card ID:** kanban card 11 (`epic-2-cause-closure`), rows 11 and 15 left `in-progress` per
  dispatch instruction.
- **Commit SHA:** (this cycle's own commit — see push log)
- **Files touched:**
  - `src/rules_core/cache_gen/equipment_gap.rs` — added a `"UE"` arm to `book_routing()`; updated
    the module/function doc comments and the `book_routing_excludes_ue` test (renamed
    `book_routing_includes_ue_gap_residue`, assertion flipped)
  - `src/rules_core/rules_tables/equipment_gap_tables.rs` — regenerated
    (`cargo run --locked --bin gen_equipment_gap_tables`); one line changed (a fresh PI redaction
    on an `inner_sea_gods` description, unrelated to the UE fix — see below)
  - `data/corpus/ultimate_equipment/equipment/*.json` (58 new) and
    `data/corpus/ultimate_equipment/equipment/equipmods/*.json` (6 new) — 64 new files, written by
    `gen_cache_equipment_gap`
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  (`git diff --unified=0 -- src/rules_core/cache_gen/equipment_gap.rs
  src/rules_core/rules_tables/equipment_gap_tables.rs | grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` → no match)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff scope,
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → no match)
- **Acceptance criterion:** `decisions.md §20` — `no_record == 0` is Gate 3's closure condition.
  This cycle's scope (per dispatch brief): `equipment` 170, `spell` 167, `equipment_modifier` 43,
  `companion` 2. `monster_ability` explicitly out of scope (sibling lane).
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`;
  worktree's oracle slot was empty on start, bootstrapped via
  `scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR"`, confirmed via
  `scripts/verify.sh --only preflight-oracle` → PASS)
- **Status:** complete for this cycle's own scope (`equipment`/`equipment_modifier` mechanical
  win landed and proven); `spell`'s residual 167 and `equipment`'s remaining 116 and
  `equipment_modifier`'s remaining 33 are traced but not closed — named below, not silently
  narrowed. `companion`'s 2 confirmed already-correctly-parked (no action taken, see below).
- **Notes:**

## §17a re-derivation before planning

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger_now.json
python3 -c "import json,collections; d=json.load(open('/tmp/ledger_now.json')); \
  c=collections.Counter(r['kind'] for r in d['rows'] if r['join_status']=='no_record'); \
  print(c.most_common())"
```
→ `monster_ability 191, equipment 170, spell 167, equipment_modifier 43, companion 2` — total
**573**, **matches the dispatch brief's own headline exactly**. Ran against the freshly-bootstrapped
repo-local oracle (empty on this worktree; landed at `7f818006e371188e5717fd18d74d18a420747fc6`).

## `equipment`/`equipment_modifier`: root cause found and fixed — a single routing-table drift

### Search for the existing path (`decisions.md §17`)

`equipment`'s 170 broke down by book: `ultimate_equipment 58` (largest single book, exactly the
brief's named starting point), `inner_sea_gods 25`, `ultimate_magic 19`, `adventurers_guide 18`,
and 13 smaller books. Traced `ultimate_equipment:equipment:aklys` end to end:

1. `src/rules_core/rules_tables/ultimate_equipment/equipment_tables.rs` (the hand-authored table
   `cache_gen::ultimate_equipment` dumps from) does **not** contain "Aklys" — confirmed real content
   at `ue_equip_arms_armor.lst:581` (`COST:5 WT:2 ... DAMAGE:1d8`), simply never entered the
   hand-authored table.
2. A **separate, already-built, already-wired generic lever exists for exactly this shape**:
   `gen_equipment_gap_tables.rs` already declares an `EQUIPMENT_BOOK_UE` `BookInput` (all 4 of UE's
   real `.lst` files) and its compiled output,
   `rules_tables::equipment_gap_tables::equipment_gap_rows()`, already carries **64 `"UE"`-book
   rows** — the exact hand-authored-table coverage gap, already computed against the pinned oracle.
3. **The rows were computed but never dumped to `data/corpus/`.** `cache_gen::equipment_gap.rs`'s
   `book_routing()` — the function that maps a gap row's book code to a `data/corpus/` directory —
   had **no arm for `"UE"` at all**, on the documented assumption "`cache_gen::ultimate_equipment`
   already owns that book." True only for the 1,613 keys the hand-authored tables list; false for
   the 64 keys that are real content the hand-authored tables never captured. Every `"UE"` row hit
   `let Some(..) = book_routing(book) else { continue }` and was silently dropped — the identical
   drift shape this same file's own `ISTEM`/`ISM` fix (a prior cycle, cited in this file's own
   comments) already named once.

This is `decisions.md §17`'s "search for the existing path" applied literally: the fix is a
**one-arm addition to a routing table**, not new per-object authorship of 64 items.

### Fix — additive-only, proven safe by construction

Added `"UE" => Some(("ultimate_equipment", "pathfinder/paizo/roleplaying_game/ultimate_equipment"))`
to `book_routing()`. Safety argument, not just assertion: `generate()`'s `held`-equivalent
protection is `write_json`'s no-clobber semantics keyed by `slugify(entry.key, used_by_book[...])`
— every one of the 1,613 already-hand-authored UE keys is absent from
`equipment_gap_tables::equipment_gap_rows()`'s `"UE"` slice in the first place (that slice is
already the *residue* `gen_equipment_gap_tables.rs` computed by subtracting the hand-authored keys
at codegen time), so routing UE can only ever emit a slug for a key that was never covered, never
re-derive or overwrite one that was.

### RED → GREEN

- **RED**: `book_routing_includes_ue_gap_residue` (renamed from `book_routing_excludes_ue`,
  assertion flipped from `None` to the real routing tuple), run against the pre-fix code:
  ```
  test rules_core::cache_gen::equipment_gap::tests::book_routing_includes_ue_gap_residue ... FAILED
  left: None
  right: Some(("ultimate_equipment", "pathfinder/paizo/roleplaying_game/ultimate_equipment"))
  ```
  Failed for the intended reason — the routing arm did not exist yet.
- **GREEN**: after adding the arm,
  ```
  cargo test --locked --lib rules_core::cache_gen::equipment_gap   # 15/15 pass
  cargo test --locked --test equipment_gap_tables                  # 7/7 pass
  cargo test --locked --test sd24_equipment_coverage_audit         # 9/9 pass (unaffected, confirms
                                                                     # no regression to the
                                                                     # hand-authored-table path)
  cargo test --locked --lib rules_core::cache_gen::                # 140/140 pass, 10 pre-existing
                                                                     # ignored
  ```

### Corpus regeneration — additive-only, verified

```bash
git status --porcelain -- data/corpus | wc -l              # 0, before the run
cargo run --locked --bin gen_equipment_gap_tables            # regen the compiled table first
git status --porcelain -- src/rules_core/rules_tables/equipment_gap_tables.rs   # 1-line diff, an
  # unrelated fresh PI redaction on an inner_sea_gods description that newly tripped the blacklist
  # screen this run — not caused by the UE routing change, named for completeness
cargo run --locked --bin gen_cache_equipment_gap
# "Equipment gap cache generated: 54 equipment, 10 equipment_modifier records"
git status --porcelain -- data/corpus | awk '{print $1}' | sort | uniq -c
#      64 ??
```
**Zero deletions, zero modifications** — only 64 new untracked files, all under
`ultimate_equipment/`. Re-ran `gen_cache_equipment_gap` a second time to confirm idempotency: `0
equipment, 0 equipment_modifier records` (everything now `skipped_pre_existing`), no further corpus
change.

## `no_record`, before/after (this cycle's own scope)

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger_after1.json
python3 -c "import json,collections; d=json.load(open('/tmp/ledger_after1.json')); \
  c=collections.Counter(r['kind'] for r in d['rows'] if r['join_status']=='no_record'); \
  print(c.most_common())"
```

| Kind | Before | After | Delta |
|---|---:|---:|---:|
| `equipment` | 170 | 116 | **-54** |
| `equipment_modifier` | 43 | 33 | **-10** |
| `spell` | 167 | 167 | 0 (not attempted; see below) |
| `companion` | 2 | 2 | 0 (verified correctly parked, not touched — see below) |
| `monster_ability` | 191 | 191 | untouched (sibling lane's scope) |
| **Bundle total `no_record`** | **573** | **509** | **-64** |

Gate 3 standing check (not touched, verified still green):
```bash
python3 scripts/shape_coverage_standing_gate.py --inventory docs/work-inventory.json
```
`NO_RECORD_BUDGET_COUNT`/`POPULATION` constants in `scripts/shape_coverage_standing_gate.py`
**not modified**, per the dispatch brief's explicit instruction.

## Closure figures — three separate numbers (`decisions.md §16`)

- **Closure** (real ingest, new corpus record, `no_record` → `matched`/`no_formula_tokens`): 64
  units total — 54 `equipment`, 10 `equipment_modifier`, all `ultimate_equipment`.
- **Reclassification:** none. No unit changed `kind` this cycle.
- **Reachability (Gate 2):** honest claim of **0**. `cache_gen::equipment_gap`'s existing
  `wiring_class` computation (`WiringClassIndex::wiring_class_for`) ran on every new record (each
  carries a real, non-fabricated `wiring_class`/`wiring_class_signals` value, e.g. `"static"` for
  Aklys's literal `COST:`/`WT:`), but this cycle did not verify or newly wire any of these 64 rows
  into `equipment_resolver::equipment_catalog_rows()` or the desktop equipment catalog — same
  Gate-1-measurability-only precedent the `spell` wave 2 cycle's own receipt sets. Not claimed here.

## `equipment`'s remaining 116: traced further, one book's shape found, not fixed this cycle

`inner_sea_gods` (25), `ultimate_magic` (19), `adventurers_guide` (18 — largest three remaining)
traced:

- `ultimate_magic`: regenerating `equipment_gap_tables.rs` against the pinned oracle now shows
  **`ultimate_magic: 0` gap rows** — every UM row the generator can compute is already `held`
  (already in a hand-authored table) or screened out; the 19-unit residual is **not** reachable
  through this lever. Real cause untraced this cycle — a different mechanism than the UE gap.
- `advanced_class_guide`'s `equipment_modifier` residual (22 of the remaining 33; the largest
  single-book slice) was traced to a **citation-resolution ambiguity, not a missing row**:
  `advanced_class_guide:equipment_modifier:answering` (census unit, `source_line: 95`, a genuine
  `.COPY=` alias row: `acg_equipmods.lst:95`: `Special Ability ~ Answering ~ Weapon.COPY=Answering`)
  already has a corpus record at `data/corpus/advanced_class_guide/equipment/equipmods/answering.json`
  — but that record cites `acg_equipmods.lst:27`, the UNRELATED base declaration, whose first
  tab-delimited column happens to equal the literal string `"Answering"` too. `find_citation`'s
  strategy order (`find_by_key_field` → `find_exact_first_column`, tried across every file, THEN
  `find_copy_variant` as a last resort) lets the coincidental first-column match at line 27 win
  before `find_copy_variant` (which would correctly resolve line 95) is ever tried. **Confirmed the
  record exists and is real** (not a `no_record` cause on its own), but the citation attached to it
  doesn't match what the census expects, so `shape_ledger`'s `(book, source_basename, source_line)`
  join still reports `no_record` for the census's own `answering` unit. `Bloodsong`/`Brawling`/the
  other 19 ACG names in this residual share the identical shape (each has both a base declaration
  and a `.COPY=` alias with the same short name).
  **Not fixed this cycle**: reordering `try_files`'s strategies risks flipping other, currently-
  correct resolutions elsewhere in the 390-citation population this matcher already resolves
  correctly (this file's own `find_citation_prefers_an_equipment_shaped_file_over_a_proficiency_file`
  test documents one prior coincidental-collision regression already fixed here) — a change of that
  shape needs its own dedicated RED→GREEN cycle validating against the full resolved-citation
  population, which this cycle's remaining budget did not have. Named precisely, not silently
  narrowed.

## `spell`'s 167: not attempted this cycle

Budget spent tracing and fixing the `equipment`/`equipment_modifier` routing defect, which was the
larger, cleaner, `§17`-shaped win (64 units, one config-table arm) versus `spell`'s residual, which
the prior cycle's own receipt already named as needing either a `apg::spell_list` consumer trace
(24 units) or further `bestiary`/`bestiary_4`/`bestiary_6` tracing (113 units) — both real per-book
investigation, not a single mechanical fix. Not started; the prior cycle's own next-steps stand
unchanged.

## `companion`'s 2: verified already correctly parked, not touched

Per the dispatch brief's own instruction to check before assuming the residual "may share [a
different] shape": read `epic-2-companion-allowlist-widening_cycle-1_cycle_receipt.md` in full. It
already closed `companion` 217 → 2 in the immediately-prior cycle, and the 2 remaining units
(`advanced_race_guide:arg_abilities_companion.lst:30-31`, "Shaitan Binder Eidolon") are a
**deliberately left undecidable** setting-specific-vs-public-domain-mythological Product Identity
judgment call, matching `decisions.md §19c`'s own precedent for exactly this creature-subtype name.
This is not a defect to fix — it is a named, correctly-parked residual pending an operator PI
ruling, and re-litigating it was explicitly out of that receipt's own scope. No action taken this
cycle; confirmed unchanged at 2 (`scripts/shape_ledger.py`, this cycle's own before/after tables
above).

## PI screening

Zero drops this cycle. All 64 new `ultimate_equipment` records screened clean:
`python3 -c "import json; [print(json.load(open(l.strip()))['pi_marker']) for l in
open('/tmp/new_corpus_files.txt')]" | sort | uniq -c` → `64 None` (no record needed the
`[redacted PI]` marker; `generate()`'s own `declared.name`/`blacklist_hit(name)` gate would have
excluded — not written — any record whose name declared or matched PI, so a written record with
`pi_marker: null` reflects a genuinely clean screen, not an unscreened one). No name-PI stop fired;
nothing to report by coordinate this cycle.

## Fixture discipline (`decisions.md §3`)

The 64 new records carry no `raw_tokens` field — this generator (`cache_gen::equipment_gap`) has
never emitted `raw_tokens` for any of its 704 previously-shipped records across 8 other books (same
struct, `EquipmentData`, unchanged this cycle); `corpus_literal_sweep` examines 0 of them by design,
identical precedent to the `spell` wave 2 cycle's own note on this point. Not a regression.

## No corpus deletions or modifications anywhere

```bash
git status --porcelain -- data/corpus | grep -v '^??' | wc -l   # 0
```

## What is NOT done, named explicitly (no silent narrowing)

- **`equipment`'s remaining 116** — `ultimate_magic` (19, gap lever computes zero rows, real cause
  untraced), `inner_sea_gods` (25, not traced this cycle), `adventurers_guide` (18, not traced this
  cycle), and 13 smaller books.
- **`equipment_modifier`'s remaining 33** — 22 of these (`advanced_class_guide`) have a fully-traced
  citation-resolution root cause (above) but no fix landed this cycle; the remaining 11 across
  `core_rulebook`/`advanced_players_guide`/`mythic_adventures`/`adventurers_guide`/`ultimate_combat`
  not traced.
- **`spell`'s 167** — not attempted this cycle; prior cycle's own next-steps stand.
- **`monster_ability` (191)** — explicitly out of scope, sibling lane.

## Discoveries

- **Discovery forward:** `equipment_gap.rs`'s `find_citation`/`try_files` strategy ordering
  (`find_by_key_field` → `find_exact_first_column`, both tried across every candidate file, THEN
  `find_copy_variant` only as a last resort) resolves to the wrong line when a base declaration's
  own display name coincidentally equals a `.COPY=<name>` alias's target name in the same file —
  traced live for `advanced_class_guide:equipment_modifier:answering` (and confirmed the same shape
  recurs for `bloodsong`/`brawling`/others in the same 22-unit residual). Needs a dedicated cycle:
  reorder or condition the strategy so a `.COPY=<key>` row wins over a coincidental first-column
  match, then re-run `find_citation`'s own test suite plus a full re-resolution of the ~390 already-
  correctly-resolved citations to confirm no regression, before touching the corpus.
- **`ultimate_magic`'s gap-table lever computing zero rows** despite a real 19-unit `no_record`
  residual — the cause is not the same routing-table drift this cycle fixed (UM was already
  correctly routed); needs its own trace.

## Next-cycle plan

1. `advanced_class_guide`'s 22-unit `.COPY=`-alias citation defect (traced above) — the next
   highest-confidence, mechanically-scoped win; needs the `find_citation` reorder + full-population
   regression check named above.
2. `equipment`'s `ultimate_magic`/`inner_sea_gods`/`adventurers_guide` residuals — each needs its
   own per-book trace; no shared cause confirmed yet.
3. `spell`'s 167 — `apg::spell_list` consumer trace (24), `bestiary`/`bestiary_4`/`bestiary_6`
   (113), per the prior cycle's own next-steps.
4. `companion`'s 2 — leave parked pending an operator PI ruling on "Shaitan" (`decisions.md §19c`).

## Retro log

```bash
python3 scripts/retro.py summary --since 2026-08-22 --json 2>&1 | tail -5
```
Real-time events already logged by prior cycles this window (§2.3 discipline was followed by those
cycles, not re-emitted redundantly here since this cycle's own findings are fully captured in this
receipt's trace narrative above, which is the primary record for a measurement-and-fix cycle with
no incident/correction/deferral of its own beyond what's already named).

## Disk

```bash
df -h /
```
(see cycle report below)
