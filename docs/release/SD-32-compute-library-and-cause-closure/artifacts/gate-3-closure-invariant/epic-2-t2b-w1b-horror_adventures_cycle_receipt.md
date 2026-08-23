# Cycle epic-2-t2b-w1b-horror_adventures — Gate 3 closure invariant / Card 11 T2b (Epic 2)

- **Card ID:** `epic-2-cause-closure` (row 11; shape T2b, book `horror_adventures`)
- **Actor:** `t2b-w1-b`
- **Commit SHA:** (this cycle's commit — see push log; docs-only, no production code touched)
- **Files touched:** none in `src/`, `apps/`, or `data/corpus/` — see Notes. Receipt + `progress.md`
  only.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (no production diff exists)
- **Wired-integration audit result:** `OK_NO_TOKENS` (no production diff exists)
- **Acceptance criterion:** AT-32-E2-001 (cause closure closes by class) — `acceptance-and-
  verification.md`.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`
  `PCGEN_ORACLE_SHA`)
- **Status:** complete
- **Notes:**

  **Correction to the census memo (`card11-t2b-census-census.md §4`, "registered books"
  table): `horror_adventures`'s real open work is 0, not 4.** The memo counted 6 residual units,
  confirmed 2 as by-design `Race Subtype ~ *` category-header exclusions, and assumed the
  remaining 4 were "never-transcribed per-record content" without checking their shape. Direct
  inspection of `docs/work-inventory.json` and the pinned oracle shows all 4 are creature-
  **template** ability rows, never player-facing race traits:

  | Unit | `corpus_key` | Oracle row shape (`ha_abilities_race.lst`) | `origin` field |
  |---|---|---|---|
  | `lich_rejuvenation` | `Lich ~ Rejuvenation` | `:267`, `CATEGORY=Special Ability\|Lich ~ Rejuvenation.MOD` — a `.MOD` row modifying an ability defined elsewhere for the Lich creature template | `mod_only` |
  | `lycanthrope_change_shape` | `Lycanthrope ~ Change Shape` | `:248`, `CATEGORY=Special Ability\|Lycanthrope ~ Change Shape.MOD` — same shape, Lycanthrope template | `mod_only` |
  | `ghoulish_creature_paralysis` | `Ghoulish Creature Paralysis` | `:270`, `CATEGORY=Special Ability\|Ghoul ~ Paralysis.COPY=Ghoulish Creature Paralysis`, `TYPE:GhoulishCreature` — a `.COPY=` rename of a monster ability for the Ghoulish Creature template | `copy` |
  | `vampiric_creature_energy_drain` | `Vampiric Creature Energy Drain` | `:351`, `CATEGORY=Special Ability\|Vampire ~ Energy Drain.COPY=Vampiric Creature Energy Drain`, `TYPE:VampiricCreature` — same shape, Vampiric Creature template | `copy` |

  None carries a `<Race> Racial Trait`/`<Race> Racial Default` TYPE component — `ingest_race_
  traits.rs`'s existing `is_mod_row` guard (line ~974, explicitly documented against Inner Sea
  Races' 618 `.MOD` rows) and its TYPE-suffix gate (`racial_trait_race`, line ~1007) already,
  correctly, permanently exclude all four. **This is why `horror_adventures`'s reach_gate.rs test
  already asserts full 43/43 reach with zero shortfall** — re-run to confirm no regression:
  `cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml
  horror_adventures_alternate_racial_traits_reach_a_player` → `ok`. **No code change was needed
  or made.**

  **This is not a T2b census-tool defect I have write scope to fix.** The 4 units remain visible
  in `docs/work-inventory.json` with `kind: "race_trait"`, `evidence:
  "race_trait_race_not_modelled"` because the upstream inventory/census tool
  (`src/bin/v06_work_inventory.rs`) classifies every non-header row in a book's
  `*_abilities_race.lst` as a `race_trait` unit by filename, the same "not-ingested figures are
  classifier noise" shape this program has hit before for other kinds. Correcting that
  classifier is out of this lane's granted scope (`ingest_races.rs`/`ingest_race_traits.rs`
  extension only) and belongs with Card 15's disposition-by-class work
  (`decisions.md §12b`). Logged, not silently fixed:
  `scripts/retro.py correction --subject t2b-census --claimed "horror_adventures: 4 real work
  units" --actual "horror_adventures: 0 real work units -- all 4 are creature-template .MOD/.COPY=
  rows the ingest tool already correctly excludes" --verified-by "grep -n 'Lich ~ Rejuvenation\|
  Lycanthrope ~ Change Shape\|Ghoulish Creature Paralysis\|Vampiric Creature Energy Drain'
  <oracle path>/ha_abilities_race.lst"` (recorded 2026-08-23,
  `docs/retro/events/sd31-transcribe.jsonl` — actor mislabelled by an inherited env var on this
  worktree; content is accurate).

  **`horror_adventures` needs no further code work from this lane.** Its true remaining T2b
  population is the 2 confirmed-excluded header rows (not work) + 4 confirmed-excluded template
  rows (not work) = 0 open real units. The book was already registered in `RACE_CORPUS_BOOKS`
  and its `BookSource` in `ingest_race_traits.rs`; nothing else to wire.

- **Discovery forwards:** the `class_feature`/`race_trait`-by-filename classifier noise pattern
  (`docs/retro/` "not-ingested figures are classifier noise") likely affects other T2b/T9 books
  whose `*_abilities_race.lst` mixes real race content with creature-template `.MOD`/`.COPY=`
  rows. Worth a sweep before any sibling lane trusts a book's raw residual-unit count without
  checking `origin` field shape first.
- **Next-cycle plan:** `bestiary_5` next (this lane).
