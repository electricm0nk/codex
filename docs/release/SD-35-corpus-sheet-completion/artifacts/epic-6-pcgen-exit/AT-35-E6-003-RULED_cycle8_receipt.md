# Cycle AT-35-E6-003-RULED cycle 8 — Epic 6 (PCGen exit) / AT-35-E6-003-RULED

- **Commit SHA:** `bfd82ec0aa` (the code, the census script and its JSON, the retro
  events), cycle start `eb73f54255`

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units by design;
  decisions.md §2, workflow-instruction.md §6 step 1)`

  It ran anyway, at the cycle's start tree `eb73f54255`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  The residue check, which is **not** exempt, ran first at the same tree and passed at exactly
  cycle 7's closing figure — nothing drifted between the two cycles:
  ```
  live_files=16 live_hits=34 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```

- **Files touched:**
  - `src/rules_core/spell_record.rs` — **new, 145 lines.** `CorpusSpellRecord`: the live
    side's own converted spell record. Field-for-field the content of a parsed spell row,
    minus the ingest format's own vocabulary — `school` is `"Transmutation"`,
    `casting_time` is `"1 standard action"`; the `SCHOOL:`/`CASTTIME:` column tags never
    reach it, because the parser already stripped them. One constructor,
    `from_corpus_json_fields`, for the corpus-JSON path, which invents nothing the JSON does
    not carry.
  - `src/pcgen_import/ir_converter.rs` — **+50.** New `spell_record_to_corpus`: the total,
    lossless, one-way map `LstSpellRecord → CorpusSpellRecord`, converter-side, and the only
    reader of the parser row left on this path. `convert_spell_record` keeps its signature
    and now returns an envelope whose payload is the converted record, interned for the
    process lifetime — the same `Box::leak` every caller of it already did one allocation
    earlier with the parser row itself.
  - `src/pcgen_import/source_content_payload.rs` — **±12.** `SourceContentPayload::Spell`
    borrows `&CorpusSpellRecord`, not `&LstSpellRecord`. The file stops importing
    `lst_parser::spell` entirely.
  - `src/rules_core/source_content.rs` — **+21.** New **live** envelope constructor
    `SourceContentRecord::spell`, so a caller holding already-converted data does not go
    through the converter to wrap it.
  - `src/rules_core/corpus_loader.rs` — **−48, +25.** `load_spell_corpus` reads
    `data/corpus/<book>/spell/*.json` — already-converted corpus data — into a
    `CorpusSpellRecord` and builds the envelope with the live constructor. It used to
    reconstruct a 21-field ingest-format parser struct out of that JSON and hand it to
    `ir_converter::convert_spell_record` to be re-converted: the live side running the
    converter over the converter's own output. Plus one new corpus-backed test.
  - `src/rules_core/spell_resolver.rs` — **±12.** `spell_id_resolve` returns
    `&CorpusSpellRecord`. Its `SourceContentPayload` import stays pointed at `pcgen_import`
    **on purpose** — see *Notes*.
  - `src/rules_core/mod.rs` — **+1.** `pub mod spell_record;`.
  - `tests/sd17_e_source_ir_shape.rs` — **±35.** The `std::ptr::eq(p, inner)` zero-copy
    assertion is replaced, deliberately and with the reason written into the test, by a
    field-by-field equality assertion over all 21 fields plus three real values. The spell
    payload is no longer a borrow of the parser row; what the round trip must still
    guarantee is that the projection is **total and lossless**, which is a strictly stronger
    claim than pointer identity.
  - `…/AT-35-E6-003-RULED_cycle8_runtime_import_census.py` / `.json` — **new.** Imports cycle
    7's census whole (which imports cycle 6's, … back to cycle 1's) and rewrites three
    groups' reasons with this cycle's measurements. It asserts its own total against the
    gate's (`gate_agreement=OK (32 == 32)`) and asserts the two specific closures by row, so
    the claim and the instrument cannot disagree silently.
  - `docs/retro/events/at-35-e6-003-ruled.jsonl` — 1 `correction`, 1 `deferral`.
  - `progress.md`, `kanban.md`, this receipt.
  - **Folded from this cycle's own instruments, not authored work:**
    `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (one
    field, `derived_at`, restamped by this cycle's `completion_atlas.py --check`). Committed
    rather than filtered away, per the standing "clean tree = unfiltered `git status` empty"
    rule.

  **No `data/` file and no corpus record was changed**, so `data/sheet_rules/` and
  `docs/work-inventory.json` are byte-identical to the cycle's start tree. **`apps/` was NOT
  touched**, so the desktop crate and the frontend run at the epic wrap-up (`§6` step 3), not
  here.

- **Identifier audit result:** OK_NO_BUNDLE_TAGS. Over this cycle's own added lines
  (`git diff eb73f54255 -- src/ tests/ | grep -E '^\+'`),
  `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` returns one line: the diff
  header `+++ b/tests/sd17_e_source_ir_shape.rs`, a pre-existing **filename**, not an
  identifier in shipping code. Same disposition every Epic 6 receipt has recorded.

- **Wired-integration audit result:** OK_NO_TOKENS, first run, no self-heal. This cycle's
  added Rust lines and the new Python instrument under
  `grep -nEi '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` return
  nothing. No `"Would …"` string, no inline mock, no fixture-only data path: the new record
  type is proved over the **real** `data/corpus/core_rulebook` spell corpus and the converter
  map is proved field-for-field against a real parsed row.

- **Acceptance criterion** (verbatim, `epic-breakdown.md` `### AT-35-E6-003`):

  > The 17 `apps/desktop/src-tauri/src/*_catalog.rs` / picker / bridge / `reach_gate.rs` readers
  > of `raw_tokens` read `SheetRule.applies` and `SheetRule.prose` instead. `render_pcgen_desc`
  > is deleted from the live side; its `%N` substitution already happened in the converter.
  >
  > **Evidence:** `pcgen_residue_gate.py --check` shows zero hits under `apps/desktop/`; desktop
  > crate and frontend suites green; the 19 on-screen tests still pass.

  Plus the `-RULED` dispatch's own bar, which is the two rulings applied **and the call sites the
  corrected gate now sees cleared**.

  The Evidence sentence's `apps/desktop` clause stays met (`root apps/desktop files=0 hits=0`,
  first met in cycle 4, not regressed here — this cycle wrote no `apps/` file). The criterion as
  a whole is **not** met: 32 hits across 16 files remain under `src/rules_core/`, and
  `render_pcgen_desc_with_values` is still called there.

- **Receipt rows (mechanical):**
  ```
  since=eb73f542554cf9a10070e564fc7eea35d18989bf target_dir=/tmp/cargo-sd35-AT-35-E6-003-RULED residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=270 ratio=n/a builds_recorded=3 pcgen_live_files=16
  ```
  `closed=0` is correct and expected: Epic 6 closes zero corpus units by design and no `data/`
  file changed, so `docs/work-inventory.json` is byte-identical before and after.

- **PCGen residue:** `live_files=16 live_hits=32 baseline_files=260 baseline_hits=12736
  verdict=PASS` — down from cycle 7's `16 / 34` on hits, **and the instrument was not touched
  this cycle** (`git diff --name-only eb73f54255..HEAD -- scripts/` is empty), so the `−2` is
  entirely code. Per-root:
  ```
  root src/rules_core         files=16 hits=34  ->  files=16 hits=32
  root src/saved_character    files=0  hits=0
  root src/campaign           files=0  hits=0
  root src/homebrew_authoring files=0  hits=0
  root apps/desktop           files=0  hits=0   (unchanged — no apps/ file written)
  ```
  **Two hits, and they are the two the census has been blocked on since cycle 1.** Not
  gate-gaming: nothing renamed to duck a regex, no path exempted, no rebaseline, and the one
  `use`-collapse available here was **refused for the second cycle running** (see *Notes*).

- **Oracle parity:** N/A for the pinned PCGen oracle — no `Number` mapping was added, no
  converter mapping row changed, and `data/sheet_rules/` is byte-identical. The parity that
  **was** required for this change is the converter map's own, and it is pinned in
  `tests/sd17_e_source_ir_shape.rs::v1_spell_record_round_trips_into_spell_payload`:
  all 21 fields of a real parsed `Magic Missile` row arrive on the payload unchanged.

- **Movement, four buckets:**
  - **closure:** 2 live `pcgen_import` hits, named by row —
    `spell_resolver.rs:15` (the `LstSpellRecord` type-ownership hit: the live side owns its
    spell record shape now) and `corpus_loader.rs:140` (the run-time `convert_spell_record`
    call on the corpus-JSON path: the conversion does not happen, because the data was
    already converted). **Zero corpus units**, as Epic 6 closes none.
  - **relabel:** none. No hit moved from one file or group to another.
  - **reachability:** none.
  - **instrument-correction:** none. The gate script and
    `scripts/pcgen-residue-baseline.env` are absent from this cycle's diff.

- **Refused tokens:** `renderer=5, lst_parser_types=11, ingest_record_tokens=5,
  trait_and_pool_tokens=3, ir_converter=3, source_content_payload=5` — **32 hits / 16 files,
  summing, all under `src/rules_core/`.** Six groups, under this cycle's flag-cap of 10.

- **Discoveries:**
  - **The "one piece of work" the census has named since cycle 1 was two pieces, and one of
    them fit in a cycle.** Cycles 1–7 recorded all 21 hits in `lst_parser_types` /
    `ir_converter` / `source_content_payload` as blocked on a single undone thing — *"a
    converted equipment/spell record shape the live side owns, which does not exist yet"*.
    The spell kind was separable and took one cycle, because its live consumers
    (`spell_resolver`, `spellbook`, `pilot_compute_corpus`) read exactly **two settled
    values**: `name` and `school`.
    `correction 1789274759824-at-35-e6-003-ruled-d48149`.
  - **And the equipment kind is not the same job — stated now so cycle 9 does not discover it
    the expensive way.** `EquipmentRecord`'s nine live consumers read `record.tokens` and
    `record.bonus_chains` **directly**. A `CorpusEquipmentRecord` carrying those arrays would
    move PCGen token structures *under a live root*: a lower gate number and a worse repo.
    The equipment half clears when its consumers read converted `SheetRule` rows, which is
    this epic's remaining piece — not a relabelling of this one.

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=16 live_hits=32`, was `16 / 34` | every non-comment, non-`#[cfg(test)]` line under the five live roots | `python3 scripts/pcgen_residue_gate.py --check` |
  | 32 hits split `renderer=5, lst_parser_types=11, ingest_record_tokens=5, trait_and_pool_tokens=3, ir_converter=3, source_content_payload=5`; `gate_agreement=OK (32 == 32)` | the same 32 hits, classified | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle8_runtime_import_census.py` |
  | 2 hits closed, by row | the 34 hits at cycle start vs the 32 at HEAD | the census's own `live_LstSpellRecord_type_hits=0` / `corpus_json_spell_conversion_hits=0` assertions, same command |
  | 9 live consumers of `EquipmentRecord` | every live file naming the type | `grep -rln 'lst_parser::equipment::EquipmentRecord' --include=*.rs src/rules_core/` |
  | the spell path's live read surface is 2 fields | every live use of a resolved spell record | `grep -rn 'spell_id_resolve' --include=*.rs src/rules_core/ \| grep -v '^src/rules_core/spell_resolver.rs'`, then the two `record.school` / `spell.name` reads at `spellbook.rs:191` and `pilot_compute_corpus.rs:197` |
  | 21 fields map total and lossless | one real parsed `Magic Missile` row | `cargo test --locked --test sd17_e_source_ir_shape v1_spell_record_round_trips_into_spell_payload` |
  | 7 prior receipts, so this is cycle 8 | this criterion's receipts on disk | `ls docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle*_receipt.md \| wc -l` |

- **Build scope verified:** root workspace in `/tmp/cargo-sd35-AT-35-E6-003-RULED`, once, at
  the final tree. `apps/` was not touched, so the desktop crate and the frontend run at the
  epic wrap-up per `§6` step 3.
  ```
  NO_RUN_EXIT=0
  cargo test --locked --lib -j 6      -> test result: ok. 3348 passed; 0 failed; 16 ignored
                                         (cycle 7's 3346 + this cycle's 2 new tests)
  cargo test --locked --no-fail-fast  -> FULL_EXIT=0; 418 targets; 8,877 passed; 0 failed;
                                         69 ignored; zero `test result: FAILED` lines
                                         (cycle 7 recorded 8,875; the +2 is exactly this
                                         cycle's two new tests)
  cargo clippy --locked --tests -j 6  -> CLIPPY_EXIT=0, 0 warnings, first run
  cargo run --bin sheet_rule_convert  -> CONVERT_EXIT=0
  ... -- --check                      -> records=49438 converted=49296 refused=142
                                         rules=70135 var_tables=5293 verdict=PASS
  grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l   -> 0
  python3 scripts/pcgen_residue_gate.py --check   -> live_files=16 live_hits=32 verdict=PASS
  python3 scripts/completion_atlas.py --check     -> citation_failures=0 stale_derived_at=False
  python3 scripts/token_coverage.py --check       -> non_done=0 refused=142 token_types=233 PASS
  python3 scripts/shape_engine_boundary.py --check-> magnitude_bearing=26396 not_held_by_engine=0
  python3 scripts/missing_engine_tables.py --check-> population=0 citation_failures=0
  python3 scripts/denominator_gate.py --check ... -> files_checked=137 violations=0
  scripts/verify.sh --only pi-sweep               -> RESULT: PASS
  corpus_literal_sweep                            -> not run; no corpus record changed
                                                     (`git status --porcelain data/` empty)
  desktop crate + frontend                        -> epic cadence; `apps/` not touched
  ```

- **Sweep population:** N/A — `corpus_literal_sweep` runs only when corpus records changed, and
  no `data/` file was written this cycle.

- **Oracle pin:** N/A — no figure in this receipt came from the pinned PCGen corpus.

- **Status:** `partial`.

- **Notes:** One trim was available and **refused for the second cycle running**:
  `spell_resolver.rs`'s `SourceContentPayload` import could be repointed at
  `rules_core::source_content`'s own re-export of the same enum, for `−1` and **zero** change
  in what the module depends on. Cycle 7's kanban row named that trim and refused it; this
  cycle does not take it either, and the import now carries a comment saying why, so the next
  cycle does not have to re-decide it. The enum moves to the live side when its remaining
  variants stop borrowing parser entry types — the same equipment-shape work.

- **Next-cycle scope:** **AT-35-E6-003-RULED cycle 9**, `SCOPE_GATE: EXEMPT (Epic 6 cycle)`,
  on the 32-hit remainder, whose largest and only genuinely-unblocked piece is the equipment
  half of `lst_parser_types` + `ir_converter` + `source_content_payload` — **19 hits across 11
  files**, which needs `equipment_effects`'s nine consumers to read converted `SheetRule` rows
  rather than `EquipmentRecord.tokens`. The `renderer` group (5) stays a converter-parity
  cycle (cycle 2's `disagree=97,332 of 660,320`), and `trait_and_pool_tokens` (3) stays
  refused on cycle 7's corrected number.
