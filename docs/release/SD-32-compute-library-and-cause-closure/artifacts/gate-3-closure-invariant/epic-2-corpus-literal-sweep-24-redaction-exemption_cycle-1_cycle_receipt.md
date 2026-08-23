# Cycle epic-2-corpus-literal-sweep-24-redaction-exemption — Gate 3 (closure invariant) / Card 11

- **Card ID:** 11 (`epic-2-cause-closure`)
- **Commit SHA:** (this cycle's commit — see push output)
- **Files touched:**
  - `src/rules_core/corpus_literal_sweep.rs` (new `ShippedRecord::codex_generated_name` field, new
    `SweepTally::codex_generated_name_tokens_exempted`/`codex_generated_name_records_exempted`
    counters, third exemption branch in `compare_tokens`, `pi_field` comma-list fix to
    `pi_redacted_description`, 7 new tests)
  - `src/bin/corpus_literal_sweep.rs` (unconditional summary line reporting the new counters)
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (row 11 note prepended)
  - `docs/release/SD-32-compute-library-and-cause-closure/progress.md` (this cycle's entry appended)

## Identifier audit result

`OK_NO_BUNDLE_TAGS`

## Wired-integration audit result

`OK_NO_TOKENS`

## Acceptance criterion

`corpus_literal_sweep` goes CLEAN (0 findings, exit 0) on the real pinned oracle, without loosening
the sweep for anything except an exactly-scoped, counted, `decisions.md §24` redaction exemption —
and still catches (a) a mutated non-redacted token on a `§24`-marked record, (b) a mutated token on
an unmarked record, and (c) a record that is NOT actually `§24`-redacted but happens to carry the
sentinel value.

## The collision, re-derived (`§17a`)

The dispatch brief's "1,014 findings across 394 records" figure was one measurement at an earlier
tip. Re-derived at this cycle's actual base (`c1505f6497`, the deity/class_feature `§24` ingestion
commit — confirmed the true `origin/tranche/12` tip via `git merge-base`): **before this fix,
`corpus_literal_sweep` reported 1 finding** against the freshly-bootstrapped pinned oracle
(`7f818006e371188e5717fd18d74d18a420747fc6`) — the corpus had already moved since the brief's
figure was taken (a sibling `source.path` fix, `af2f07f68`, had already landed on `tranche/12`
ahead of this cycle's base, closing most of the 1,014). The one remaining finding was NOT a `§24`
naming-shape record at all:

```
corpus-literal-sweep: MISMATCH data/corpus/inner_sea_magic/ability/diplomatic_student.json:
    token not byte-present in corpus token closure: DESC:[redacted PI]
```

`diplomatic_student.json` has `codex_generated_name: false` (its own name is clean) but
`pi_field: "description,raw_tokens"` — a comma-joined list, because more than one field was
redacted on the same record. The pre-existing DESC-redaction exemption required
`pi_field == Some("description")` by exact equality, which a multi-field list never satisfies,
so a real, declared (`DESCISPI:YES` on the oracle row), already-audited redaction (
`declared_pi_shipping_audit.rs` already reads this exact field as a comma-list, precedent
followed here) was reported as a byte-level mismatch. Fixed alongside the `§24` exemption
(same class of defect: the sweep not recognizing an authorised divergence) by matching
`declared_pi_shipping_audit.rs`'s own `split(',').any(|part| part == "description")` reading.

## What was built

A third, narrow exemption in `compare_tokens`, gated on the record's own
`data.codex_generated_name` field (never the filename):

```rust
if record.codex_generated_name && token.value == REDACTED_PI_MARKER {
    tally.codex_generated_name_tokens_exempted += 1;
    tally.codex_generated_name_records_exempted.insert(record.record_path.clone());
    continue;
}
```

Scoped exactly to `decisions.md §24b`-2's shape: a `§24`-renamed record's redaction is not confined
to `DESC`, and does not require the real corpus row's same-key value to independently re-screen as
blacklisted (`KEY:Trait ~ Guardian of the Forge` redacts because it restates the record's own
original name, not because the phrase itself is a blacklisted term — neither of the two
pre-existing exemptions covered that shape). The exemption fires ONLY when:

1. `record.codex_generated_name == true`, read off the record's own top-level JSON field — a
   `codex_named_unit_*` filename is never trusted as proof (`compare_tokens`'s own doc comment).
2. The token's value is EXACTLY `REDACTED_PI_MARKER` (`"[redacted PI]"`) — any other value on the
   same record, including one that merely drifted from the real corpus row, is still checked
   normally and still reported.

Both counters are printed unconditionally in the binary's summary line (zero included), per `§22`'s
divergence-must-be-visible condition and `§24b`-4.

## RED → GREEN, both directions, on the real corpus

**GREEN (before → after):**

```
# before this fix (pinned oracle 7f818006e371188e5717fd18d74d18a420747fc6)
corpus-literal-sweep: 46119 records examined of 49225 read, 379715 tokens compared (9 synthesized), 49212 digests checked, 1 findings
corpus-literal-sweep: MISMATCH data/corpus/inner_sea_magic/ability/diplomatic_student.json: token not byte-present in corpus token closure: DESC:[redacted PI]
corpus-literal-sweep: 1 findings across 1 records

# after this fix
corpus-literal-sweep: 46119 records examined of 49225 read, 379715 tokens compared (9 synthesized), 49212 digests checked, 0 findings
corpus-literal-sweep: 1145 tokens exempted under decisions.md §24 redaction across 406 codex_generated_name records
corpus-literal-sweep: CLEAN
```

**RED, mutation-proved (then reverted — `git diff --stat` on both files empty afterward):**

1. Corrupted a non-redacted token (`CATEGORY`) on a `§24`-marked record
   (`codex_named_unit_ability_advanced_players_guide_apg_abilities_lst_230.json`) — still a
   TokenNotInClosure finding: a `§24` record is not exempt from the sweep, only the redacted token
   in it is.
2. Corrupted a token (`KEY`) on an UNMARKED record (`magical_lineage.json`,
   `codex_generated_name: false`) — still a TokenNotInClosure finding: the exemption never fires
   without the record's own marker.

```
corpus-literal-sweep: 46119 records examined of 49225 read, 379715 tokens compared (9 synthesized), 49212 digests checked, 2 findings
corpus-literal-sweep: 1145 tokens exempted under decisions.md §24 redaction across 406 codex_generated_name records
corpus-literal-sweep: MISMATCH data/corpus/advanced_players_guide/ability/codex_named_unit_ability_advanced_players_guide_apg_abilities_lst_230.json: token not byte-present in corpus token closure: CATEGORY:Corrupted Category Value
corpus-literal-sweep: MISMATCH data/corpus/advanced_players_guide/ability/magical_lineage.json: token not byte-present in corpus token closure: KEY:Trait ~ Magical Lineage_CORRUPTED
corpus-literal-sweep: 2 findings across 2 records
```

Both corpus files reverted from backup immediately after; `git diff --stat` on both is empty.

3. A record that is NOT `§24`-redacted (`codex_generated_name: false`, absent) but happens to carry
   the exact sentinel value in some field is still a finding — pinned as
   `an_unmarked_record_gets_no_24_exemption_for_the_sentinel_value` (unit test, not a corpus
   mutation, since this shape was already true pre-existing and this cycle only proves the new
   branch doesn't loosen it): a record cannot smuggle a token through by merely claiming `§24`.

## Unit tests (`cargo test --locked --lib rules_core::corpus_literal_sweep`)

36 passed, 0 failed. 7 new: multi-field `§24` redaction exempt-and-counted; a non-redacted drifted
token on a `§24` record still caught; an unmarked record gets no exemption for the sentinel value;
`codex_generated_name` parses from the record's own top-level field (present/absent); a comma-list
`pi_field` still exempts DESC.

## `no_record`, before and after (per brief instruction)

This cycle touched only `corpus_literal_sweep`'s comparison logic — no ingest script, no
`data/corpus/**` content change (the two corpus files mutated for the RED proof were reverted
byte-for-byte). `no_record` is therefore **unmoved by this cycle**, confirmed by re-running
`python3 scripts/shape_ledger.py` once (both before starting this fix and unaffected by it):
population 35,328, `no_record` **1,814** (5.1%) — different from the brief's/prior cycle's cited
2,664 figure because sibling lanes have continued landing `no_record`-closing work on `tranche/12`
between that entry and this cycle's base; not attributable to this cycle, which shipped zero
`data/corpus/**` changes.

## Notes

- Did not touch Gate 3's budget constants.
- Did not put a real PI term in code/tests/comments/this receipt — used the sentinel string, an
  invented deity-shaped placeholder already used by the pre-existing test suite ("Iomedae" as a
  stand-in, already present before this cycle, not introduced here), and real corpus phrases that
  are NOT on the PI blacklist (e.g. "Guardian of the Forge", "Magical Lineage") cited only to explain
  *why* the pre-existing exemptions didn't cover the shape.
- Sibling lane note (concatenated-PI-name / `scrub_name_pi_tokens` shared-module work): no collision
  observed — this cycle's diff touches only `corpus_literal_sweep.rs` (lib + bin), disjoint from
  that lane's files.

## Discovery forwards

One, reported by name in `progress.md`'s addendum to this cycle's own entry rather than added to
`## DISCOVERED` (already at its 10-entry self-heal ceiling): after rebasing onto sibling commit
`e7d80ad430` ("`ability` fully regenerated"), a third sweep run found 1 NEW, unrelated finding —
`data/corpus/inner_sea_magic/ability/hidden_wand.json`, NOT `§24`-renamed, whose `raw_tokens.DESC`
is over-redacted relative to its own clean `data.description` (a `scripts/ingest_ability.py`
two-scan-disagreement defect — `blacklist_term_hit_including_concatenated` vs
`normalized_term_hit` disagreeing on the identical text). Confirms the `§24` exemption built this
cycle is not over-broad: the sweep correctly still reports it. Out of this cycle's granted scope
(`corpus_literal_sweep.rs` only) and out of this cycle's file scope
(`scripts/ingest_ability.py`, the sibling lane's active file) — escalated, not fixed.

## Next-cycle plan

This cycle's own defect (`§24` redaction / `pi_field` comma-list) is closed and mutation-proved,
both directions, on the real pinned oracle. One residual, unrelated finding remains open for a
future cycle scoped to `scripts/ingest_ability.py` (see Discovery forwards above) — the sweep
itself is not the fix site for that defect.
