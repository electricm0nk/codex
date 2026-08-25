# Cycle epic-2-t2b-w1b-bestiary_5 — Gate 3 closure invariant / Card 11 T2b (Epic 2)

- **Card ID:** `epic-2-cause-closure` (row 11; shape T2b, book `bestiary_5`)
- **Actor:** `t2b-w1-b`
- **Commit SHA:** (this docs-only commit — see push log; no production code touched)
- **Files touched:** none in `src/`, `apps/`, or `data/corpus/`. Receipt + `progress.md` only.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (no production diff exists)
- **Wired-integration audit result:** `OK_NO_TOKENS` (no production diff exists)
- **Acceptance criterion:** AT-32-E2-001 / AT-32-E4-001 — `acceptance-and-verification.md`.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6`
- **Status:** measurement cycle — **0 units banked**, per `decisions.md §13`'s explicit
  authorization ("measurement... does not substitute for the work... a precursor to it"). Escalated
  below.
- **Notes:**

  **Correction to the census memo and dispatch brief for `bestiary_5` — this book was
  mis-scoped as "ingest-tool extension only... no chassis-load wiring needed."** Re-derived,
  by class, not by instance:

  ```
  python3 -c "
  import json
  d = json.load(open('docs/work-inventory.json'))
  u = [x for x in d['units'] if x.get('kind')=='race_trait' and x.get('evidence')==
       'race_trait_race_not_modelled' and x.get('book')=='bestiary_5']
  from collections import Counter
  print(Counter(k.split(' ~ ')[0] for k in (x['corpus_key'] for x in u)))
  print(len(u))
  "
  ```
  → 137 (136 real work + 1 `Race Subtype ~ Sahkil` header row, already confirmed by-design
  excluded — matches the census memo exactly). The 136 real units split into three classes, none
  of which is a simple ingest-tool extension:

  **Class A — 8 races/entities with no chassis this project has ever built (61 units):**
  Shabti 12, Reptoid 10, Deep One Hybrid 9, Orang-Pendak 9, Astomoi 8, Caligni 7, Clockwork
  Familiar 5, Esipil 1. `ingest_races.rs`'s `IN_SCOPE_RACES` for `bestiary_5` names only
  `skinwalker` (`RaceSpec { dir: "skinwalker", book: "bestiary_5" }`) — none of these 8 have a
  `RaceSpec` entry, and adding one is chassis registration, not extension. Confirmed no
  stale-regen shortcut exists here (unlike `inner_sea_races`): re-ran
  `cargo run --bin ingest_races` against the pinned oracle; the only `bestiary_5` files it
  touched were Skinwalker's 9 already-committed standard-trait records, and the diff was
  `ingested_at`-only (byte-identical `data`), reverted (`git checkout -- data/corpus/bestiary_5`).
  Nothing here was silently stale.

  **Class B — Skinwalker's own heritage-selector shape (72 units): 36 `Skinwalker ~ Change
  Shape (*)`/`~ Default`/`~ Were*-Kin` rows + 9 `Were*-Kin ~ *` subrace rows × 4 fields each.**
  `ingest_races.rs`'s own `skinwalker` `RaceSpec` doc comment (this cycle re-read it, not
  transcribed) states this explicitly: "Skinwalker's heritage rows themselves are NOT ingested by
  this batch... each heritage alternate sets its `Skinwalker_Replace*` FACT flags directly on its
  OWN constituent trait rows (via a `PREMULT` gate on the selector), a structurally different
  shape `subrace_grants()` cannot parse without new code. That is a genuinely new mechanism,
  deferred (not stubbed) to a follow-on batch." This cycle did not attempt it — building it here
  would be exactly the "genuinely new mechanism" the prior batch already, correctly, deferred.

  **Class C — `Adopted Race ~ Skinwalker` (1 unit): a selector-mechanism gap shared across
  4 books, not local to `bestiary_5`.** The census memo (`card11-t2b-census-census.md §3`) and
  a sibling T2b lane's own `epic-2-t2b-w1-c` cycle (`progress.md` DISCOVERED, above) both already
  found this spans `bestiary_2`/`bestiary_5`/`bestiary_6`'s 9 `Adopted Race ~ <X>` units plus
  `advanced_race_guide`'s 7 `CATEGORY:Adoptive Parentage` units — the sibling's own recommendation
  is "one follow-up cycle builds the selector once, scoped across all four books together, rather
  than two T2b lanes race-conditioning the same shared surface independently." Building it here,
  alone, for `bestiary_5`'s single instance would be exactly that race condition.

  **2 further units checked and NOT counted as real work above, matching the disposition-by-class
  bar (`decisions.md §12b`):**
  - `Favored Enemy ~ Humanoid (Skinwalker)` (`skinwalker_abilities_race.lst:27`) —
    `TYPE:RangerClassFeatures.FavoredEnemy.SpecialAttack.Extraordinary.AttackOption`. A Ranger
    class-feature-shaped grant, not a `<Race> Racial Trait`/`Racial Default` TYPE; the same
    TYPE-suffix gate this program's own `ingest_race_traits.rs` uses elsewhere would never match
    it. Real content, but not this lane's ingest-tool shape — it belongs with whatever surface
    models class-granted racial favored enemies, not the race-trait picker.
  - `Psychic Magic` (`b5_abilities_race_oa.lst:23`) — sourced from the book's
    `_abilities_race_oa.lst` support file. The identical hazard `ingest_race_traits.rs`'s
    `horror_adventures` `BookSource` doc comment already names for a sibling book's own `_oa.lst`
    file: PCGen loads it conditionally on owning Occult Adventures
    (`grep -l PRECAMPAIGN` pattern), a book this project has not ingested; the gate is on the pcc
    load line, not inside the `.lst` itself. Not counted as open `bestiary_5` work pending
    confirmation of the same conditional-load gate for this book's `.pcc`.

  **No stub, no fabricated chassis, no fake completion was produced to close this book's
  count.** All 136 units remain open on disk and in `docs/work-inventory.json`, honestly.

  **This is a blocker per `AGENTS.md` Blocker Discipline, not a deferral**: `bestiary_5` was
  inside AT-32-E2-001's Definition of Done at launch, scoped by the dispatch brief as "roughly 3
  files... no chassis-load wiring." That scoping was wrong for 135 of its 136 units.  Escalated
  in `progress.md`'s Open blockers: the exact ruling needed is which of Class A (8-race chassis
  batch, largest single win — 61 units), Class B (Skinwalker heritage mechanism — 72 units, the
  single largest block, benefits every book with a heritage-selector shape once built), or Class
  C (cross-book Adopted Race selector — 1 unit here, 9 corpus-wide) gets sequenced first, and by
  which lane.

- **Discovery forwards:** none new — Class B and Class C are already named as open findings by
  sibling T2b cycles (`epic-2-t2b-w1-c`'s DISCOVERED entries, above); this cycle corroborates both
  independently from `bestiary_5`'s own side rather than duplicating the finding.
- **Next-cycle plan:** none on `bestiary_5` under this lane's granted scope (ingest-tool extension
  only). This lane's three assigned books are now dispositioned: `inner_sea_races` (12 closed, 45
  escalated), `horror_adventures` (0 real units, census corrected), `bestiary_5` (0 bankable
  within scope, fully escalated by class). See lane summary in the dispatch return.
