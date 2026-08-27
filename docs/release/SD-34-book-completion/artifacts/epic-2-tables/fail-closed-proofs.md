# AT-34-E2-002 — fail-closed proofs, per table

**Criterion (verbatim, `epic-breakdown.md`):** "A table returns a real record or a named
refusal. It never returns a fabricated or defaulted entry. **Evidence:** per table, a
RED→GREEN pair — observed refusing an absent key, and returning a real record for a present
one."

Epic 2 builds 8 tables (`AT-34-E2-001`): 7 new runtime loaders in
`src/rules_core/rules_tables/simple_kind_tables.rs`, plus the pre-existing `companion` table
(`src/rules_core/rules_tables/companion_chassis.rs`, built in SD-29). This criterion formalizes
the fail-closed proof for all 8 as its own deliverable — a dedicated test per table, each
covering both halves, plus the transcript evidence AT-34-E2-001 already produced.

## Per-table proof

Each row below names: the resolver under test, the dedicated test that proves both halves in
one run, and the transcript lines (`AT-34-E2-001_table_transcript.txt`, committed
2026-08-27) showing the same two outcomes against the live corpus.

| Table (kind) | Resolver | Test (both halves) | Transcript: HELD (success) | Transcript: REFUSED (absent key) |
|---|---|---|---|---|
| `ability` | `SimpleKindTable::resolve` | `simple_kind_tables::tests::ability_table_holds_aberrant_bloodline` (+ `an_absent_key_is_refused_not_fabricated`) | `kind=ability ... sample=(advanced_class_guide, "Aberrant Bloodline") -> HELD name="Aberrant Bloodline" source=.../acg_abilities_other.lst:21 raw_tokens=8` | `kind=ability ... sample=(advanced_class_guide, "___a_key_no_corpus_record_carries___") -> REFUSED (absent key)` |
| `template` | `SimpleKindTable::resolve` | `simple_kind_tables::tests::template_table_holds_arcanist_spellbook` | `kind=template ... sample=(advanced_class_guide, "Arcanist SpellBook") -> HELD name="Arcanist SpellBook" source=.../acg_templates.lst:12 raw_tokens=2` | `kind=template ... -> REFUSED (absent key)` |
| `trait` | `SimpleKindTable::resolve` | `simple_kind_tables::tests::trait_table_holds_trait_adopted` | `kind=trait location=data/corpus/*/trait_generic/*.json ... sample=(advanced_players_guide, "Trait ~ Adopted") -> HELD name="Adopted" source=.../apg_abilities.lst:109 raw_tokens=6` | `kind=trait ... -> REFUSED (absent key)` |
| `deity` | `SimpleKindTable::resolve` | `simple_kind_tables::tests::deity_table_holds_a_pi_masked_codex_named_record` | `kind=deity ... sample=(bestiary_6, "Codex-Named Unit (deity_bestiary_6_b6_deities_lst_21)") -> HELD name=... source=.../b6_deities.lst:21 raw_tokens=11` | `kind=deity ... -> REFUSED (absent key)` |
| `domain` | `SimpleKindTable::resolve` | `simple_kind_tables::tests::domain_table_holds_battle_spirit` | `kind=domain ... sample=(advanced_class_guide, "Battle (Spirit)") -> HELD name="Battle (Spirit)" source=.../acg_domains.lst:5 raw_tokens=1` | `kind=domain ... -> REFUSED (absent key)` |
| `skill` | `SimpleKindTable::resolve` | `simple_kind_tables::tests::skill_table_holds_craft_rope` | `kind=skill ... sample=(bestiary_2, "Craft (Rope)") -> HELD name="Craft (Rope)" source=.../b2_skills.lst:8 raw_tokens=7` | `kind=skill ... -> REFUSED (absent key)` |
| `language` | `SimpleKindTable::resolve` | `simple_kind_tables::tests::language_table_holds_xenophobic` | `kind=language ... sample=(advanced_race_guide, "Xenophobic") -> HELD name="Xenophobic" source=.../arg_languages.lst:6 raw_tokens=1` | `kind=language ... -> REFUSED (absent key)` |
| `companion` | `CompanionBook::companion_resolve` | `companion_chassis::tests::companion_resolve_refuses_a_fabricated_key_it_never_defaults` (new this cycle) | `kind=companion location=rules_tables::companion_chassis::COMPANION_BOOKS ... sample=(inner_sea_combat, "Companion (Worg)") -> HELD name="Companion (Worg)"` | `kind=companion ... -> REFUSED (absent key)` |

8 of 8 tables proven. Re-derive the transcript column:
`cat docs/release/SD-34-book-completion/artifacts/epic-2-tables/AT-34-E2-001_table_transcript.txt`
(committed by AT-34-E2-001, unmodified by this cycle — `git log --follow` on that file shows one
commit, `052a9182bf9666d7eaf757104034e040e72f6add`).

## RED → GREEN, this cycle's own evidence

The 7 `simple_kind_tables` resolvers already carried this proof inline from AT-34-E2-001
(each `kind_holds_named_record!` macro invocation asserts a real resolve **and**, in the same
test body, a fabricated-key refusal on the same table). This cycle adds the 8th: `companion`
had no dedicated fail-closed test before now — its resolver was only exercised through
domain-specific tests (e.g. `.COPY=` delta rows) that happen to prove `None` for those specific
keys, not a named "fabricated key never resolves" guarantee.

**RED, confirmed for the intended reason.** `companion_resolve` was temporarily mutated to
fabricate a default instead of refusing:

```rust
// TEMPORARY, reverted before commit:
pub fn companion_resolve(&self, key: &str) -> Option<&'static CompanionRecord> {
    self.companions.iter().find(|c| c.key == key).or_else(|| self.companions.first())
}
```

```
$ cargo test --locked --lib rules_core::rules_tables::companion_chassis::tests::companion_resolve_refuses_a_fabricated_key_it_never_defaults
running 1 test
test rules_core::rules_tables::companion_chassis::tests::companion_resolve_refuses_a_fabricated_key_it_never_defaults ... FAILED

thread '...' panicked at src/rules_core/rules_tables/companion_chassis.rs:1313:9:
a fabricated key must never resolve to a companion record, real or defaulted

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2870 filtered out
```

The failure is the intended one: the assertion on the fabricated-key half fired, not an
unrelated panic (compile error, missing fixture, etc.) — proof the test actually exercises the
fail-closed guarantee rather than passing vacuously.

**GREEN, after reverting the mutation:**

```
$ cargo test --locked --lib rules_core::rules_tables::companion_chassis::tests
running 15 tests
...
test rules_core::rules_tables::companion_chassis::tests::companion_resolve_refuses_a_fabricated_key_it_never_defaults ... ok
...
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 2856 filtered out
```

```
$ cargo test --locked --lib rules_core::rules_tables::simple_kind_tables
running 11 tests
...
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 2860 filtered out
```

## Row-count command output (this artifact's own count)

```
$ grep -c '^| \`' docs/release/SD-34-book-completion/artifacts/epic-2-tables/fail-closed-proofs.md
8
```

8 of 8 Epic 2 tables carry a proven RED→GREEN fail-closed pair. Denominator: the 8 tables
Epic 2 builds (`technical-design.md §4`; `power`, the 9th kind, is Epic 5's and out of this
criterion's population).

## Movement, four buckets

- **Closure:** 0 — this criterion proves an existing property, it does not move a unit between
  buckets.
- **Reclassification:** 0.
- **Reachability:** 0.
- **Instrument-correction:** 0 — no prior figure changes; this closes a gap in test coverage
  (the 8th table lacked a dedicated fail-closed test) without altering any counted population.

## Notes (judgment calls)

- The 7 `simple_kind_tables` resolvers' fail-closed proof was already present inline in
  AT-34-E2-001's tests (each kind's test does both halves in one function). This criterion does
  not duplicate that work with redundant tests; it cites the existing tests by name and adds the
  one table (`companion`) that did not yet carry a dedicated proof.
- `companion_resolve`'s mutation-and-revert (the RED half) touched
  `src/rules_core/rules_tables/companion_chassis.rs` only for the duration of the RED run; the
  file at HEAD (post-revert, plus the new test) is the only change this cycle ships to that
  file.
