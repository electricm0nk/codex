# Cycle t2b-w1-a/1 — Gate 3 closure invariant / Epic 2, shape T2b, book `bestiary_6`

- **Card ID:** `epic-2-cause-closure` (row 11)
- **Actor:** `t2b-w1-a`
- **Commit SHA:** (this book lands in the same commit as `monster_codex`; see that book's receipt
  for the SHA)
- **Files touched:** none for `bestiary_6` itself (see finding below) —
  `docs/retro/events/t2b-w1-a.jsonl` (correction logged)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion (verbatim):** AT-32-E2-001 — T2b closed corpus-wide, by class.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`)
- **Status:** complete
- **Notes:**

`bestiary_6`'s one residual T2b unit is `Adopted Race ~ Rougarou`
(`rougarou_abilities_race.lst:29`). Re-derived directly against the pinned oracle: the row is
`KEY:Adopted Race ~ Rougarou CATEGORY:Special Ability TYPE:AdoptiveRace MULT:YES
CHOOSE:ABILITYSELECTION|Special Ability|TYPE=Rougarou Race Trait ABILITY:Traits|VIRTUAL|%LIST`,
immediately followed by `CATEGORY=Special Ability|No Race Trait Available.MOD
TYPE:Rougarou Race Trait` — the CHOOSE pool's *only* member is the literal placeholder. This is
the identical browse-only-stub shape `ingest_races.rs`'s own `IN_SCOPE_RACES` doc comment already
investigated and named for this exact row ("it offers nothing and gates nothing"). Writing a
record for it would ship a selectable menu option that changes nothing — the defect
`decisions.md 44.2`'s browse-only-stub guard (and `ingest_race_traits.rs`'s own
`assert!(!found.is_empty(), ...)` heritage-selector guard) exists to refuse.

**Finding — correction to the census memo**, logged via `scripts/retro.py correction`
(`docs/retro/events/t2b-w1-a.jsonl`): `card11-t2b-census-census.md` counted this row as real open
T2b work. Verified against the pinned oracle it is by-design not-work, same disposition as the
147 category-header rows already excluded. Cross-checked the identical shape against all 7 of
`bestiary_2`'s "Adopted Race ~ <X>" rows (Fetchling/Grippli/Ifrit/Oread/Sylph/Undine/Dhampir) —
every one pairs `CHOOSE:ABILITYSELECTION|Special Ability|TYPE=<Race> Race Trait` with only
`No Race Trait Available.MOD TYPE:<Race> Race Trait` as the pool. All 8 "Adopted Race" units
across the two books are this same stub, not open work.

**`bestiary_6` is fully closed** — its one residual unit is correctly not-work; there is no
transcribable content left to ingest for this book under T2b.

- **Discovery forwards:** none.
- **Next-cycle plan:** none — this book needs no further T2b work.
