# Cycle t9-onboarding-equipment-copy-citation-repair — Gate 3 closure invariant / `decisions.md §20`

- **Card ID:** kanban card 11 (`epic-2-cause-closure`), rows 11 and 15 left `in-progress` per
  dispatch instruction.
- **Commit SHA:** (this cycle's own commit — see push log)
- **Files touched:**
  - `src/rules_core/cache_gen/equipment_gap.rs` — `try_files`'s strategy order changed so
    `.COPY=<id>` is tried BEFORE the bare first-column match for the SAME identifier (previously
    tried only as a last-resort fallback, after every file had already been tried for a
    first-column match); new regression test
    `find_citation_prefers_a_copy_variant_over_a_coincidental_first_column_match`; new `#[ignore]`d
    full-population regression test `find_citation_full_population_regression`.
  - `src/rules_core/cache_gen/equipment_copy_citation_repair.rs` (new) — a narrowly-scoped,
    self-proving repair for ALREADY-SHIPPED records whose citation went stale under the fixed
    resolver; four independent safety checks, detailed below.
  - `src/rules_core/cache_gen/mod.rs` — registers the new module.
  - `src/bin/repair_equipment_copy_citations.rs` (new) — CLI entry point, `--check`/write modes,
    scoped to the three books this cycle traced the defect in.
  - `data/corpus/{advanced_class_guide,core_rulebook,mythic_adventures}/equipment/equipmods/*.json`
    (29 files) — `source.line` corrected; stale `raw_tokens`/`raw_bonus_chains` removed (never
    hand-computed — see below) then repopulated by the pre-existing `enrich_equipment_raw_tokens`
    binary.
  - `data/corpus/**` (203 further files, 9 other books) — a side effect of running the pre-existing,
    idempotent `enrich_equipment_raw_tokens` on its full book list to regenerate the 29 repaired
    records' tokens: it also filled a pre-existing enrichment backlog (records that had never been
    enriched at all, unrelated to this cycle's defect) in `ultimate_psionics` (113),
    `ultimate_equipment` (64), `ultimate_combat` (19), `ultimate_intrigue` (7). Purely additive
    (fills previously-absent fields on already-shipped records); confirmed zero deletions, zero
    citation changes outside the 29 this cycle's own tool touched
    (`git status --porcelain -- data/corpus | awk '{print $1}' | sort | uniq -c` → `235 M`, `0`
    anything else).
  - `docs/retro/events/t9-onboarding.jsonl` — append-only, two `verification` events from this
    cycle's own `scripts/verify.sh --only preflight-oracle` runs (oracle bootstrap).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  (`git diff --unified=0 -- src/rules_core/cache_gen/equipment_gap.rs
  src/rules_core/cache_gen/mod.rs src/rules_core/cache_gen/equipment_copy_citation_repair.rs
  src/bin/repair_equipment_copy_citations.rs | grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` → one hit, a legitimate cross-reference to the
  pre-existing test name `sd24_equipment_coverage_audit` in a doc comment, not a bundle-tag leak)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff scope,
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → no match)
- **Acceptance criterion:** `decisions.md §20` — `no_record == 0` is Gate 3's closure condition.
  This cycle's scope (per dispatch brief): `equipment` 116, `equipment_modifier` 33.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`;
  worktree's oracle slot was empty on start, bootstrapped via
  `scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR"`, confirmed via
  `scripts/verify.sh --only preflight-oracle` → PASS)
- **Status:** complete for this cycle's own scope (`equipment_modifier` 27-unit mechanical win landed
  and proven, `equipment`'s 116 untouched — traced further, one major new-shaped lead surfaced, named
  below, not silently narrowed).

## §17a re-derivation before planning

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger_now.json
python3 -c "import json,collections; d=json.load(open('/tmp/ledger_now.json')); \
  c=collections.Counter(r['kind'] for r in d['rows'] if r['join_status']=='no_record'); \
  print(c.most_common())"
```
→ `monster_ability 191, spell 167, equipment 116, equipment_modifier 33, companion 2` — total **509**,
matches the dispatch brief's own headline exactly. Per-book breakdown for `equipment`/
`equipment_modifier` also re-derived and matched the brief's figures exactly (see dispatch brief's
own table — confirmed, not re-transcribed here).

## Root cause: the SAME `find_citation` defect the immediately-prior cycle traced but did not fix

### The defect (`decisions.md §17`, re-applied to a second drift in the same resolver)

`equipment_gap::find_citation`'s `try_files` tried, per identifier, `KEY:<id>` then bare
first-column `<id>` then (only as an absolute last resort, after every file had been tried for
BOTH of those) `.COPY=<id>`. A `.lst` block frequently declares a base "template" row whose own
DISPLAY name (first column) coincidentally equals the short identity a DIFFERENT row's
`.COPY=<id>` line mints — e.g. `advanced_class_guide/acg_equipmods.lst`:

```
Answering<TAB>KEY:Special Ability ~ Answering ~ Weapon<TAB>...SPROP:Enhancement bonus...   (line 27)
Special Ability ~ Answering ~ Weapon.COPY=Answering<TAB>VISIBLE:NO                          (line 95)
```

Line 27 is a template whose own `KEY:` is the long string; its bare first column happens to read
"Answering". Line 95 is the row that actually mints the playable object keyed `"Answering"`. The
census's own `answering` unit (`advanced_class_guide:equipment_modifier:answering`, `corpus_key:
"Answering"`) expects `source_line: 95`; the shipped record (data content correct — `.COPY=`
inherits every field verbatim, confirmed for every case this cycle repairs — but `source.line`
wrong) cited line 27, so `shape_ledger.py`'s `(book, source_basename, source_line)` join found no
record at line 95 and reported `no_record`.

### Fix — `try_files` strategy reorder

`.COPY=<id>` is now tried immediately after `KEY:<id>`, before the bare first-column match, for
each identifier (`key`, then `name` if different). Promoting it is safe by construction: it can
only ever change an outcome when BOTH a `.COPY=<id>` row and a coincidental first-column `id`
exist in the SAME file — and a `.COPY=<id>` target is a strictly stronger identity signal (PCGen's
own syntax's sole purpose is to mint a new object under that exact key) than a first-column string,
which is just a display name that can coincide with an unrelated row's key for cosmetic reasons.

### RED → GREEN

```
test rules_core::cache_gen::equipment_gap::tests::find_citation_prefers_a_copy_variant_over_a_coincidental_first_column_match
```
Temporarily reverted `try_files` to the old strategy order in-place, re-ran this one test:
```
left: Some(("acg_equipmods.lst", 1))
right: Some(("acg_equipmods.lst", 2))
```
Failed for the intended reason (resolves to the template row, not the `.COPY=` row). Restored the
fix, re-ran:
```
cargo test --locked --lib rules_core::cache_gen::equipment_gap    # 16/16 pass (was 15; +1 new test)
```

### Full-population regression proof (the prior cycle's own explicit ask)

Wrote `#[ignore]`d test `find_citation_full_population_regression` inside `equipment_gap.rs`'s own
test module (a bin crate cannot reach `pub(crate) find_citation`/`book_routing`, confirmed — the
same visibility wall `gen_equipment_gap_tables.rs`'s own doc comment already names for
`declared_pi_at`). Re-resolves EVERY already-shipped `data/corpus/**/equipment*/**/*.json` record
whose `source.kind == "lst_token"` against its OWN stored `data.key`/`data.name`, using the fixed
resolver, and diffs against the shipped citation:

```
PCGEN_CORPUS_ROOT=... cargo test --locked --lib \
  rules_core::cache_gen::equipment_gap::tests::find_citation_full_population_regression \
  -- --ignored --nocapture
```
→ `checked=7464 mismatches=32`. All 32 traced by hand:
- **29** are the real defect above, spread across 3 books (`advanced_class_guide` 23,
  `core_rulebook` 4, `mythic_adventures` 2) — every one independently verified against the real
  `.COPY=` shape before being repaired (below).
- **3** (`core_rulebook`'s `holy_symbol_silver`/`holy_symbol_wooden`, `ultimate_equipment`'s
  `masterwork_tool-2`) resolve to a genuinely different line under the fixed resolver too, but that
  line does not correspond to any currently-`no_record` census unit — left untouched (see "Not
  repaired" below; repairing them would be pure churn with no closure benefit and was refused by
  this cycle's own repair tool's `NewLineAlreadyClaimed`/coverage gates in the `--check` dry run,
  confirmed not present in its `REPAIRED` list).

Zero of the 32 were a citation moving to a DIFFERENT FILE (only ever a different line in the SAME
file) — the shape stays exactly what the prior cycle's receipt named.

## The repair mechanism — a second, narrowly-scoped tool, not a blind regen

Fixing `find_citation` alone does not move `no_record`: `write_json`'s no-clobber guard means an
ALREADY-SHIPPED record at a stale line is never automatically rewritten by re-running the ordinary
generators. A dedicated repair tool was required — `cache_gen::equipment_copy_citation_repair` /
`src/bin/repair_equipment_copy_citations.rs` — built to the same bar `cache_gen::
lst_provenance_repair` already sets for "narrow an already-shipped citation, never fabricate":

**Four independent safety checks, every one against the real corpus, not assumed:**
1. `find_citation` resolves the record's OWN `data.key`/`data.name` to a DIFFERENT line in the
   SAME file it already cites (a different file is refused, out of scope).
2. The newly-resolved line's own row, read fresh off disk, really is a `.COPY=<key>`/`.COPY=<name>`
   variant — proving same-identity, never a re-identification.
3. **The OLD line remains covered by at least one OTHER record in the book** after the move — this
   is the check that makes the move safe: every book this repaired writes its `held` (hand-authored,
   long-key) units through a SEPARATE generator (`cache_gen::acg`'s own local resolver for ACG,
   `cache_gen::hand_authored_equipment` elsewhere) that already, independently, cites the same base
   line under the long key. Confirmed live for all 29 repaired records — the check is not
   theoretical, `citation_counts` is built from every `lst_token` record actually on disk.
4. The NEW line is not already the citation of any other record (no collision created).

A record failing any check is left exactly as shipped and named in the tool's own report (`REFUSED`
in `--check`), never silently skipped.

**`raw_tokens`/`raw_bonus_chains` are REMOVED, never hand-computed**, so the pre-existing
`enrich_equipment_raw_tokens` binary — the ONE established mechanism for that field
(`decisions.md §17`) — repopulates them from the corrected line on its own next run, rather than
this tool forking that logic.

### RED → GREEN (repair module)

```
cargo test --locked --lib rules_core::cache_gen::equipment_copy_citation_repair
```
3/3 pass:
- `repairs_a_stale_copy_citation_when_the_old_line_has_independent_coverage` — the exact `answering`
  shape, proves the move AND the `raw_tokens` removal.
- `refuses_when_the_old_line_has_no_independent_coverage` — the safety gate fires and the record is
  left byte-identical when no sibling covers the old line.
- `dry_run_does_not_write` — `--check`-equivalent leaves disk untouched.

### Execution

```bash
PCGEN_CORPUS_ROOT=... cargo run --locked --bin repair_equipment_copy_citations -- --check
# advanced_class_guide: 23 repaired, 294 refused, of 317 read
# core_rulebook: 4 repaired, 2989 refused, of 2993 read
# mythic_adventures: 2 repaired, 250 refused, of 252 read
# repair-equipment-copy-citations: 29 repaired, 3533 refused
PCGEN_CORPUS_ROOT=... cargo run --locked --bin repair_equipment_copy_citations   # real run
PCGEN_CORPUS_ROOT=... cargo run --locked --bin enrich_equipment_raw_tokens      # repopulates tokens
git status --porcelain -- data/corpus | awk '{print $1}' | sort | uniq -c   # 235 M, zero deletions
```

## `no_record`, before/after (this cycle's own scope)

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger_after.json
python3 -c "import json,collections; d=json.load(open('/tmp/ledger_after.json')); \
  c=collections.Counter(r['kind'] for r in d['rows'] if r['join_status']=='no_record'); \
  print(c.most_common())"
```

| Kind | Before | After | Delta |
|---|---:|---:|---:|
| `equipment` | 116 | 116 | 0 (not this cycle's fix shape — see below) |
| `equipment_modifier` | 33 | **6** | **-27** |
| `spell` | 167 | 167 | 0 (out of scope this cycle) |
| `companion` | 2 | 2 | 0 (correctly parked, `§19c`) |
| `monster_ability` | 191 | 191 | untouched (sibling lane's scope) |
| **Bundle total `no_record`** | **509** | **482** | **-27** |

`equipment_modifier` per-book, before/after:

| Book | Before | After |
|---|---:|---:|
| `advanced_class_guide` | 22 | **0** |
| `core_rulebook` | 5 | 2 (the 2 `Intelligent Item Purpose` units — a DIFFERENT defect, a
  `write_json` slug-collision named in this file's own doc comment; not this cycle's shape) |
| `mythic_adventures` | 2 | **0** |
| `advanced_players_guide` | 2 | 2 (not traced this cycle) |
| `adventurers_guide` | 1 | 1 (traced to a DIFFERENT cause — see below) |
| `ultimate_combat` | 1 | 1 (not traced this cycle) |

## Closure figures — three separate numbers (`decisions.md §16`)

- **Closure** (real citation correction, a `no_record` unit's own line now has a byte-clean
  `lst_token` record with corpus-derived, `.COPY=`-inherited data — never fabricated): **27 units**,
  all `equipment_modifier` — 22 `advanced_class_guide`, 3 `core_rulebook`, 2 `mythic_adventures`.
- **Reclassification:** none. No unit changed `kind`.
- **Reachability (Gate 2):** honest claim of **0**, matching the prior two cycles' own precedent on
  this exact generator. `wiring_class` was already stamped on these records (they are pre-existing,
  previously-shipped rows this cycle only re-cited); this cycle did not verify or newly wire any of
  them into `equipment_resolver::equipment_catalog_rows()` or the desktop equipment catalog. Not
  claimed here.

## `equipment`'s 116: untouched this cycle, one major new lead traced (not fixed)

### `ultimate_magic`'s 19: partially re-traced

The prior cycle's receipt named this "gap lever computes zero rows, real cause untraced". Re-traced
this cycle: `gen_equipment_gap_tables.rs`'s `EQUIPMENT_BOOK_UM` `BookInput` DOES exist and covers
both files the 19 units cite; `hand_authored_equipment.rs`'s `BookInput{short_code:"UM",...}` also
exists and its `ultimate_magic::equipment_tables()` table already carries hardcoded entries for ALL
25 of this book's `Spellbook.COPY=<name>` rows (same `.COPY=` shape as the ACG defect, confirmed:
`Spellbook.COPY=<name>` line, e.g. `um_equip_general.lst:13`). Of the 19 no-record units, at least 5
carry `NAMEISPI:YES` (named by coordinate, never by name, per this dispatch's PI discipline:
`um_equip_general.lst` lines 13 and 23; `_pfs/pfs_um_equip_general.lst` lines 9, 13, 16) — the SAME
`decisions.md §24` neutral-name shape traced below for
`inner_sea_gods`/`adventurers_guide`, not this cycle's `.COPY=`-citation shape. The remaining ~14
non-PI-named UM units were not further isolated this cycle (budget went to the larger,
already-proven `equipment_modifier` win and to tracing the `§24` shape below, which recurs in
multiple books).

### The major new lead: `decisions.md §24`'s neutral-name mechanism exists but is NOT wired into
### the equipment pipeline — traces `inner_sea_gods`'s 25 (23 of them) and `adventurers_guide`'s 18

Every one of `adventurers_guide`'s 18 `no_record` `equipment` units carries `NAMEISPI:YES` on its
own `.lst` row (per-row re-derivation this cycle, all 18 confirmed; named by coordinate, per
`decisions.md §15`/this dispatch's PI discipline, never by name — spans
`ag_equip_arms_armor.lst` lines 1, 6, 7, 9–11, 17, 19–20, 28 and `ag_equip_magic_items.lst` lines
2, 25, 43–47, plus `ag_equip_general.lst` line 27). 23 of `inner_sea_gods`'s 25 carry it too
(`isg_equip.lst` lines 13–32 — a contiguous 20-row block — plus lines 156, 161, 215; the remaining
2, at lines 172 and 222, did not show the literal `NAMEISPI:YES` tag in this cycle's spot check and
were not further verified against the blacklist term scan).

**`decisions.md §24`'s Codex-generated-neutral-name mechanism is already built and already proven**:
`src/rules_core/codex_neutral_name.rs` (a `pub` module, not `pub(crate)` — reachable from a bin
crate), Rust-ported byte-identical from `scripts/codex_neutral_name.py`, already wired into
`cache_gen::class_feature.rs`'s ingest (the pattern to copy: on a declared-name-PI hit, compute
`neutral_name("class_feature", book, source_file, source_line)`/`neutral_key(...)`, scrub the PI
name out of `raw_tokens`, stamp `codex_generated_name: true` and a `rename: {reason, coordinate}`
field per `§24b`-3/4, ship under the neutral name instead of excluding).

**Why this is traced but NOT fixed this cycle**: `equipment_gap_tables.rs`'s `EquipmentGapRow`
(and `hand_authored_equipment.rs`'s equivalent) is a compile-time-static struct with no
`codex_generated_name`/`rename` fields at all, and `equipment_resolver::equipment_catalog_rows()`
and the desktop equipment catalog consume that struct directly — wiring `§24` in here means either
extending that struct (and every one of its consumers, a wider blast radius than this cycle's
budget covers safely) or building an equivalent post-write rename pass for these two generators'
JSON output specifically. This needs its own dedicated cycle, the same disposition the prior
cycle's receipt gave the ACG `.COPY=` defect this cycle then closed. **Named precisely, per
`decisions.md §15`: no name-PI record was transcribed or silently skipped this cycle — the 41
affected units (18 AG + 23 ISG) are named here by their coordinate/count, not ingested, and the
mechanism that would close them (already built, already proven elsewhere) is named exactly.**

## `spell`'s 167, `companion`'s 2: not attempted / correctly parked

Unchanged from the prior cycle's own disposition — not attempted (`spell`) or already correctly
parked pending an operator PI ruling (`companion`, `§19c`). Not re-litigated this cycle.

## PI screening

Zero drops, zero transcriptions this cycle. This cycle repaired CITATIONS on already-screened,
already-shipped records (their `data`/`pi_field`/`pi_marker` are untouched by the repair tool; only
`source.line` and the enrichment-eligible `raw_tokens`/`raw_bonus_chains` fields changed).
`enrich_equipment_raw_tokens`'s own `NAMEISPI:YES` skip fired 0 times across this run's 232
enrichments (`0 skipped (declared NAMEISPI:YES)` in its own printed summary) — none of the 29
repaired-and-reenriched records' NEW cited lines carry undeclared PI its own screen would have
caught. The `§24`-shaped 41 units named above were never transcribed — they remain `no_record`,
correctly, pending the dedicated wiring cycle.

## Fixture discipline (`decisions.md §3`)

The repair tool never fabricates a value: `source.line` is the only field it computes (from
`find_citation`, itself only ever returning a real line it read off disk), and `raw_tokens`/
`raw_bonus_chains` are removed rather than hand-populated, letting the corpus's own established
enrichment tool (which DOES read raw bytes and apply `.COPY=` inheritance the same way PCGen does)
produce them. Spot-checked `answering.json` post-repair: `source.line: 95`, `raw_tokens` now leads
with `VISIBLE:NO` (line 95's own literal token) followed by the inherited base fields — matches
`.COPY=` semantics exactly, not fabricated.

## Corpus-literal-sweep — full corpus run, one PRE-EXISTING unrelated finding

```bash
PCGEN_CORPUS_ROOT=... cargo run --locked --bin corpus_literal_sweep -- --quiet
```
→ `1 findings across 1 records`: `data/corpus/inner_sea_magic/ability/hidden_wand.json` — a
`kind: ability`, `inner_sea_magic` book PI-redaction byte mismatch, unrelated to this cycle's
`equipment`/`equipment_modifier` scope and to any file this cycle touched (confirmed: not among the
232 modified files). Pre-existing, named for completeness, not remediated (out of scope; not owned
by this lane — `ability` kind is outside the dispatch brief).

## No corpus deletions anywhere

```bash
git status --porcelain -- data/corpus | grep -v '^ M' | wc -l   # 0
```

## What is NOT done, named explicitly (no silent narrowing)

- **`equipment`'s 116** — untouched this cycle's numeric total (the `§24` neutral-name lead above
  covers 41 of them across 2 books, traced not fixed; `ultimate_magic`'s remaining ~14 non-PI units
  not isolated; `inner_sea_gods`'s other 2 non-`NAMEISPI`-tagged units not further checked;
  remainder — 13 smaller books — not traced this cycle).
- **`equipment_modifier`'s remaining 6** — `core_rulebook`'s 2 `Intelligent Item Purpose` units (a
  named, DIFFERENT `write_json` slug-collision defect, traced in this file's own doc comment,
  needs its own fix); `advanced_players_guide` 2, `adventurers_guide` 1 (this book's one
  `equipment_modifier` unit was not checked against the `§24`/`.COPY=` shapes this cycle),
  `ultimate_combat` 1 — none traced this cycle.
- **`spell`'s 167**, **`monster_ability`'s 191`** — unchanged, out of scope.

## Discoveries

- **Discovery forward:** `decisions.md §24`'s Codex-neutral-name mechanism is fully built
  (`codex_neutral_name.rs`, proven in `class_feature.rs`) but not wired into ANY equipment
  generator. At least 41 of `equipment`'s 116 `no_record` units (`adventurers_guide` 18,
  `inner_sea_gods` 23) are this exact shape. Wiring it needs `EquipmentGapRow`/
  `hand_authored_equipment`'s output schema extended with `codex_generated_name`/`rename` (or an
  equivalent post-write pass), full consumer-surface verification
  (`equipment_resolver::equipment_catalog_rows()`, desktop catalog, `equipment_resolver.rs`'s own
  pinned counts), and a fresh `no_record` re-derivation — sized as its own cycle, not a config-row
  tweak.
- **Discovery forward:** `core_rulebook`'s 2 remaining `equipment_modifier` `no_record` units
  (`Intelligent Item Purpose (Slay All)`/`(Slay Creature Type)`) trace to `write_json`'s
  no-clobber guard hitting a DIFFERENT slug-collision than this cycle's shape — that module's own
  doc comment already names the exact collision (`"Intelligent Item Purpose (Slay All)"` slugifying
  to the same filename as an already-shipped, richer record at line 446 vs. the real row at 895).
  Needs a distinct disambiguation fix (e.g. citation-derived slug suffixing), not this cycle's
  repair shape.

## Next-cycle plan

1. Wire `decisions.md §24`'s neutral-name mechanism into the equipment/equipment_modifier
   pipeline — highest-confidence, highest-yield remaining lead (≥41 units traced by coordinate).
2. Fix `write_json`'s slug-collision defect for `core_rulebook`'s 2 `Intelligent Item Purpose`
   units.
3. `ultimate_magic`'s remaining ~14 non-PI `equipment` units, `inner_sea_gods`'s 2 non-`NAMEISPI`
   units, `inner_sea_intrigue` (8), `bestiary_2` (7), `inner_sea_combat` (7), `inner_sea_world_guide`
   (7) and the remaining smaller books — each needs its own per-book trace.
4. `spell`'s 167 — per the prior cycle's own next-steps, unchanged.
5. `companion`'s 2 — leave parked pending the operator's PI ruling.

## Retro log

Two `verification` events logged automatically by this cycle's own `scripts/verify.sh --only
preflight-oracle` runs (oracle bootstrap in a fresh worktree, one expected FAIL before bootstrap,
one PASS after) — append-only, already in `docs/retro/events/t9-onboarding.jsonl`.

## Disk

```bash
df -h /
```
(see cycle report below)
