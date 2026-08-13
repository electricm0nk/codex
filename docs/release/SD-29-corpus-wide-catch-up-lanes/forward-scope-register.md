# SD-29 — Forward-Scope Register

> **RE-SCOPED CORPUS-WIDE, 2026-08-10 (`decisions.md §38`).** This register's seven-book scope
> (§0/§1.1) is preserved below as the historical predecessor-deferral analysis it performed — it
> remains the authoritative source for the seven-book figures it derived (§1.3). It is **no longer
> SD-29's current book-list boundary**: SD-29's lanes now run corpus-wide, across all 37 in-scope
> books (`../corpus-work-channels.md §10.2`), per `decisions.md §38`.
>
> **Status:** scope pass, **signed off 2026-08-02**. The operator pinned this register's own §0/§1.1 seven-book scope (`decisions.md §34`); loader may proceed per that decision.
> Nothing downstream (`scope-draft.md`, epics, criteria) exists yet, deliberately — per
> `spec-domain-bundle-authoring` v1.2.0 and SD-27's own recorded pitfall, the register is the
> disagreement surface the operator signs, and the committed scope is authored *after*.
>
> **Authored:** 2026-08-01, by the `tranche/7-1` debt cycle, from
> `docs/retro/events/*.jsonl` + the PCGen source checkout + the SD-27 bundle docs.
> **Predecessor:** SD-27 — Future-State Book Content Ingestion (branch `tranche/7`).
> **Workchannel:** `SD-29 (Bestiary)`, 7 books, per
> `../SD-27-future-state-book-content-ingestion/epic-breakdown.md:150`.

This register separates inherited work into three classes, matching SD-27's register exactly.
The class determines whether SD-29 **owns** it (class 1), **sequences** it (class 2), or **stays
clear** of it (class 3). The three-class separation is load-bearing; do not collapse it into a
flat backlog.

---

## 0. How this register was derived, and the one number that matters

Every figure below was produced by running a command on 2026-08-01 against this working tree and
the PCGen checkout at `/home/todd/workspace/repos/pcgen`. Nothing is copied from SD-27's docs
without re-derivation; two SD-27 figures did not survive that and are corrected in §1.3.

```bash
# The deferral corpus this register routes.
ls docs/retro/events/*.jsonl | wc -l                    # 52 shard files
python3 - <<'PY'                                        # 241 events, of which:
import json,glob,collections
c=collections.Counter()
for p in glob.glob('docs/retro/events/*.jsonl'):
    for line in open(p):
        if line.strip(): c[json.loads(line)['type']]+=1
print(c)
PY
# -> deferral 74, correction 90, verification 40, incident 28, near_miss 4, rework 4, note 1
```

| measure | value |
|---|---:|
| retro shard files | 52 |
| shard files containing at least one deferral | 40 |
| deferrals | **74** |
| deferrals whose own text names **SD-29** | **1** |
| deferrals whose own text names **SD-28** | 0 |
| deferrals containing the word "unscheduled" | 0 |
| deferrals whose own text names SD-27 (as a citation, not a routing) | 11 |

**The honest headline: exactly one deferral routes itself to SD-29.** The dispatching brief
estimated "~72 deferrals across ~51 actor shards"; the real figures are 74 across 40 (52 shards
exist, 12 carry no deferral). That correction is logged.

Everything else in class 1 comes from `epic-breakdown.md:150` (the book list) or from the corpus
itself (§1.3). Everything in class 2 is an *inference* with its evidence stated. Class 3 is
explicitly **not claimed**. Two deferrals are blocked on Ultimate-line books and are routed to
SD-28, not silently absorbed here — §4.1.

---

## Class 1 — Committed payloads (with documentary citations)

The strongest authority. Non-negotiable.

### 1.1 The 7 Bestiary-line `book_stub` entries

**Source:** `../SD-27-future-state-book-content-ingestion/epic-breakdown.md:150` (the operator's
dashboard workchannel table); SD-27 `decisions.md §9` (all 7 are Tier-1);
`docs/governance/wired-integration-stubs-registry.md`; `data/stubs/*.json`.

| # | Book | Stub manifest | Registry entry |
|---|------|---------------|----------------|
| 1 | `bestiary_2` | `data/stubs/bestiary_2.json` | `#0006` |
| 2 | `bestiary_3` | `data/stubs/bestiary_3.json` | `#0007` |
| 3 | `bestiary_4` | `data/stubs/bestiary_4.json` | `#0008` |
| 4 | `bestiary_5` | `data/stubs/bestiary_5.json` | `#0009` |
| 5 | `bestiary_6` | `data/stubs/bestiary_6.json` | `#0010` |
| 6 | `bonus_bestiary` | `data/stubs/bonus_bestiary.json` | `#0011` |
| 7 | `monster_codex` | `data/stubs/monster_codex.json` | `#0014` |

**SD-29's obligation.** Resolve each stub into a real Shape B v1 JSON cache at
`data/corpus/<book>/`, through the 4-stage per-book cycle SD-27 proved (license prep +
pre-build → verify → parity). `content_kind_counts` gets a real value rather than `null`; the
registry `Status` flips from "Registered stub" to "Resolved" with a receipt pointer.

**Out of scope within class 1.** Designing new rules or new chassis for these books. SD-27
`decisions.md §24` — no formula interpreter, every feature hand-modelled as a corpus-verified
pure function — stands and is inherited.

### 1.2 Monster Codex ingestion unblocks the project's last reach NO

**This is the one deferral that names SD-29**, at
`docs/retro/events/record-gaps.jsonl` (id `1785554760789-record-gaps-d9bdf9`, 2026-08-01):

> `blocked_by`: *"monster_codex ingestion (Tier-1 but deferred: decisions.md §9; assigned to
> SD-29's Bestiary bundle by epic-breakdown.md:150)"*
> `revisit`: *"The day Monster Codex is ingested — `tests/sd27_duergar_invisibility_sla_is_upstream_blocked.rs`
> goes RED by design at that point"*

**The record.** `Duergar ~ Spell-Like Ability ~ Invisibility` is ingested, on disk, and carries
its real prose. No player can reach it. It is the **only** entry left in
`reach_gate::OPEN_FINDINGS` — count derived, not assumed:

```bash
awk '/const OPEN_FINDINGS/,/^\];/' apps/desktop/src-tauri/src/reach_gate.rs | grep -c '^    ('
# -> 1
```

**The protocol, read off the corpus.** Duergar's two spell-like-ability rows are mutually
exclusive alternatives, each granted by the flag that removes *the other one*
(`core_essentials/races/duergar/duergar_abilities_race.lst:27-28`). ARG supplies one half —
`Duergar ~ Blood Enmity` sets `Duergar_ReplaceSLAInvisibility`, which grants the Enlarge Person
row, and that really works today. The other half has no setter in any ingested book.

**Re-derived against the whole PCGen checkout on 2026-08-01, not inherited from SD-27's prose:**

```bash
grep -rc 'FACT:Duergar_ReplaceSLAEnlargePerson|True' /home/todd/workspace/repos/pcgen | awk -F: '$2>0'
# -> .../roleplaying_game/monster_codex/mc_abilities_race.lst:1
```

**Exactly one row, in one file, in one book: `Duergar ~ Ironskinned`,
`monster_codex/mc_abilities_race.lst:16`.** Every other checkout-wide mention of that identifier
is a `FACTDEF:`, a `DEFINE:`, a `PREVAREQ:`/`PREFACT:` gate, or a `BONUS:VAR` — a *reader* of the
flag, never a setter. The mirror flag `FACT:Duergar_ReplaceSLAInvisibility|True` appears in three
files, one of which (`advanced_race_guide/arg_abilities_race.lst`) is ingested — which is
precisely why the mirror row reaches a player and this one does not.

**The executable proof, cited by path as required:**
`tests/sd27_duergar_invisibility_sla_is_upstream_blocked.rs`. Four tests: the record is really on
disk with its real prose; **no ingested book sets the flag**; no Duergar alternate selection —
one at a time *or all at once* — brings the row in; and the symmetric half of the protocol *does*
reach a player, which is what makes "blocked" the right word instead of "broken".

**How this finding closes.** `no_ingested_book_sets_the_flag_that_grants_duergar_invisibility`
fails the moment a corpus record sets that flag — i.e. the day SD-29 ingests Monster Codex. The
finding closes by a test going red and being read, not by anyone remembering. **SD-29 must expect
that red and treat it as the deliverable**, then delete the `OPEN_FINDINGS` and
`UNREACHED_RECORD_FINDINGS` entries. Do not close it by hiding the record.

### 1.3 Corpus-shape corrections SD-29 must not inherit blind

**Two of the assumptions this bundle was sized on are false.** Both were re-derived here rather
than taken from `cross-bundle-findings-2026-07-30.md` §5, which flagged the first.

Base race/monster declarations per book (`SOURCELONG` header rows excluded), 2026-08-01:

| book | `.lst` files | total lines | base `races.lst` rows |
|---|---:|---:|---|
| `bestiary_2` | 23 | 8,965 | **322** |
| `bestiary_3` | 22 | 7,359 | **261** |
| `bestiary_4` | 30 | 7,108 | **220** |
| `bestiary_5` | 17 | 807 | **no base `races.lst` at all** (9 rows in `b5_races_pc.lst`) |
| `bestiary_6` | 12 | 321 | **no base `races.lst` at all** (1 row in `b6_races_pc.lst`) |
| `bonus_bestiary` | 4 | 279 | **14** |
| `monster_codex` | 21 | 591 | **2** |

**Correction 1 — `bestiary_5` and `bestiary_6` are not monster books.** They ship only `_pc` and
`_companion` race files. A per-monster-block epic over them yields **zero** monster cycles. This
invalidates the "~250-300 monsters each, total ~1,000-1,200" figure SD-29 was sized on. The real
total across all seven books is **819** base declarations, and 803 of them are in three books.

**Correction 2 — `monster_codex` is not a monster book either, and this is new.**
`mc_races.lst` holds **2** race declarations (Seru; Sootwing Bat). Its actual payload is
player-and-NPC options for races that already exist: 34 feats, 26 spells, 53 equipment rows
across three files, 74 class abilities, 21 racial-ability rows (touching exactly three races —
Duergar, Ratfolk, Seru), 20 race kits and 34 companion kits. **Sizing Monster Codex as a
Bestiary-style monster ingest will produce a cycle plan that does not match the book.** Its value
to this project is the racial-ability file — which is exactly where the Duergar flag lives.

**Consequence for epic authoring:** the 7 books are **not** uniform, so a single templated
per-book cycle count is wrong. Three monster-block books (b2/b3/b4), one small monster book
(bonus_bestiary, 14), one racial-options book (monster_codex), and two player-options datasets
(b5/b6) that need a different cycle shape or an explicit carve-out. Derive book shape from the
corpus before writing any epic — this is the third time that instruction has had to be written
down.

---

## Class 2 — Structurally implied by the payload (ingestion-pipeline debt a 7-book fan-out multiplies)

These are open deferrals that **no document routes to SD-29**, but whose own `revisit` field or
subject matter makes them SD-29's problem the moment a book is ingested. SD-29 must decide to own
or sequence each; none may be assumed closed. Evidence is stated so the operator can disagree.

### 2.1 `records_processed` is written from the wrong source and will silently regress

**Retro:** `book-coverage-truth-agent.jsonl`, deferral 06. **Revisit:** *"next cycle that owns
`src/bin/sd27_gen_book_cache.rs`"*.

`sd27_gen_book_cache.rs` writes each book's `LICENSE.json` `records_processed` from **what that
one binary itself emitted** (for PU, `feat_written + equipment_written`). Any other binary that
adds records to the same book directory — and there are three — leaves the compliance number
behind. Re-running the generator *resets* corrected values.

**This has now bitten four times**, and the fourth was closed on 2026-08-01 by the same cycle
that authored this register: `pathfinder_unchained` said 59 against 127; `advanced_race_guide`
479 against 635; `core_rulebook` **3326 against 3400**; `beastiary` **45 against 164**.
`tests/sd27_book_license_record_counts.rs` now derives its own book coverage from the filesystem,
so the drift fails loudly — but the **source** is untouched and SD-29 ingests 7 more books
through it. Fix at source, per `AGENTS.md` rule 5.

### 2.2 The reach gate is structurally blind to function-wrapped tables

**Retro:** `closeout.jsonl`, deferral 11. **Revisit, verbatim:** *"the next book ingest that
emits function-wrapped tables, or any cycle touching `corpus_ingest_diagnostic`"* — which is
SD-29 by definition.

`reach_gate::full_inventory()` unions three discovery sources. Two of them cannot see a modern
book: `scanned_inventory()` matches column-zero `pub const NAME: &[Type]` declarations only,
while Pathfinder Unchained emits its records inside accessor **function bodies**; and
`corpus_ingest_diagnostic` reports a hardcoded four-book list. A book the gate cannot see is a
book whose reach claims are vacuously green. See SD-27 `decisions.md §29.3`.

### 2.3 `ingest_races.rs` has no `%%` handling

**Retro:** `arg-desc-render.jsonl`, deferral 04. **Revisit:** *"any re-ingest of a book whose
`DESC` rows use the escape"*.

`substitute_placeholders` does not collapse PCGen's literal-percent escape and
`leaked_pcgen_syntax` does not flag it. Nothing leaks today (0 of the shipped race-trait
descriptions carry `%%` or `%N` — re-derived 2026-08-01), and `ingest_races.rs` is the binary
that will ingest every Bestiary-line race. A Bestiary row carrying `50%%` ships `50%%` to the
player.

### 2.4 One ingest binary classifies records without running the PI screen

**Retro:** `book-coverage-truth-agent.jsonl`, deferral 07.

`src/bin/ingest_race_traits_arg.rs` sets `License::Ogl` on all 156 ARG racial-trait records
**without running any screen**, where `ingest_pu_classes.rs` and `ingest_races.rs` both run the
54-term PI-blacklist scan and abort the run on a hit. The classification is in fact correct (the
identical scan returns 0 hits), so this is an *unchecked-but-true* claim rather than a wrong one
— which is exactly the state that survives review. SD-29 runs a 7-book license pass; an
unscreened classification path is a redistribution liability multiplied by seven.

### 2.5 Height/weight profiles are read from no book's ingest

**Retro:** `race-creation-widening.jsonl`, deferral 41. **Revisit:** *"a cycle that ingests
`<race>_biosettings.lst`"*.

PCGen keeps them in `<race>/<race>_biosettings.lst`, which **no** book's ingest reads for **any**
book. Every Bestiary-line PC race SD-29 makes creatable inherits the same hole.

### 2.6 A duplicate `#[path]` include inflates the clippy baseline

**Retro:** `spellgate-agent.jsonl`, deferral 59. **Revisit:** *"next cycle owning
`src/bin/sd27_gen_book_cache.rs`"* — the same file as §2.1, so the two close together.

It `#[path]`-includes `rules_tables/advanced_race_guide/mod.rs` a second time into its own crate
though the module is now registered normally. Deleting the include should drop several warnings
off the root clippy baseline of 75.

### 2.7 `SpellCatalogScreen`'s `BOOK_ORDER` is a hardcoded literal pinned against another literal

**Retro:** `verify.jsonl`, deferral 72.

Not a current falsehood — and that is the point. The screen's book list and its test's book list
are two hand-maintained constants that agree with each other, so a book added on the Rust side
leaves the screen silently short and the test green. **This is the identical defect shape that
kept `core_rulebook`'s 3326 stale**: a guard whose coverage was a hand-maintained constant could
not see the two books that were drifting. SD-29 adds up to 7 books. Derive the list, do not
extend it.

### 2.8 (conditional) `rule_set_for()` still maps only four books

**Retro:** `closeout.jsonl`, deferral 13. **Routed by its own text to an SD-27 cycle**, not to
SD-29 — recorded here as **conditional inheritance only**. `v06_work_inventory.rs`'s
`rule_set_for()` maps `core_rulebook`/`advanced_players_guide`/`advanced_class_guide`/`beastiary`,
so ARG and PU corpus units keep scope `future_state` and evidence `no_compiled_rule_set_for_book`.
If SD-27 closes it, SD-29 inherits nothing. If SD-27 does not, SD-29's 7 books land in the same
false state. **Do not claim this without checking whether SD-27 closed it first.**

---

## Class 3 — Candidate only (no documentary backing)

**31 of the 74 deferrals are real, open, and belong to nobody.** They are engine and UI work:
size-modifier follow-ups, spell-picker gating, feat-effect grounding for magnitudes the engine has
no total for, post-creation racial-trait editing, archetype suppression, the Class Progression
browser's coverage gap, and so on. Each is enumerated in the ledger below.

**None of them is SD-29's**, and this register does not claim them. Carrying an engine backlog
into a content-ingestion bundle on no authority is the "scope takeover" SD-27's register warns
against by name. If the operator wants any of them, they need an explicit greenlight and their own
cycle — not a line in this file.

Two class-3 items are worth flagging as *adjacent* without being claimed, because a wide book
bundle will make them more visible rather than less:

- **The Add Item picker shows no source book** (`equipment-surface.jsonl` 17, `verify.jsonl` 71 —
  the same defect logged twice). Seven more books make an unlabelled row worse. It is an omission,
  not a false statement.
- **Character creation offers 7 races while the corpus supplies 18** (`verify-agent-t71.jsonl` 65,
  `race-creation-widening-agent.jsonl` 39). SD-29 adds PC-playable races from `b5_races_pc.lst`
  and `b6_races_pc.lst`; if the creation roster is still a 7-entry literal, they arrive unreachable.

---

## 4. Explicitly routed elsewhere — do not absorb

### 4.1 Blocked on the Ultimate line → **SD-28**, not SD-29

Two deferrals name Ultimate Magic and Ultimate Combat as their blocker. `epic-breakdown.md:149`
routes both books to SD-28. They are listed here **only** so a future reader can see they were
considered and rejected:

- `pu-class-wiring-agent.jsonl` (30) — the Unchained Summoner's own 202-spell list.
  `blocked_by`: *"Ultimate Magic and Ultimate Combat ingestion (46 of the 202 spells)"*.
- `pu-rogue-summoner-features.jsonl` (37) — the `summoner_spell_list.rs` slice.
  `blocked_by`: *"Ultimate Magic and Ultimate Combat are unregistered; 46/202 spells resolve to
  neither of the 4 ingested books"*.

### 4.2 Routed to a named non-SD-29 owner by their own `revisit`

10 deferrals name a specific file-owning cycle (`race_trait_picker.rs`, `pilot_compute.rs`,
`previewData.ts`, `cache_gen/apg.rs`, `race_resolver.rs`, …) or an SD-27 reachability cycle. They
are in the ledger, marked `ELSEWHERE`. SD-29 does not inherit them.

### 4.3 Already closed at `tranche/7-1` HEAD

**22 of the 74 are no longer open.** Each was verified by command on 2026-08-01, not assumed:

| deferral | closed by | verification |
|---|---|---|
| 05, 46 — `core_rulebook`/`beastiary` `LICENSE.json` counts | this cycle | `cargo test --test sd27_book_license_record_counts` → 6/6 |
| 14 — `monster_catalog.rs` raw `\|` in `race_subtype` | this cycle | `cargo test monster_catalog` → 14/14 |
| 15 — equipment description reaching no screen | this cycle | `npm test` → 97/97; `tsc --noEmit` clean |
| 18, 62 — `reach_gate` `OPEN_FINDINGS` for equipment/ARG spells | earlier tranche/7 cycle | `OPEN_FINDINGS` now holds exactly 1 entry |
| 03, 64 — raw `%%`/`%N` in ARG alternate-trait descriptions | ARG desc-render cycle | 0 shipped `race_trait` descriptions carry either |
| 40, 50, 54, 70 — size modifiers to touch AC / CMB / CMD | size-modifier cycle | all three now computed in `pilot_compute.rs` |
| 73 — flat-footed AC computed in React | `decisions.md §29.2` cycle | `flat_footed_armor_class` present in **both** compute twins |
| 74 — equipment descriptions not run through `render_pcgen_desc` | desc-sweep cycle | `equipment_catalog::serve_description` exists |
| 32, 69 — PU `LICENSE.json` `records_processed: 59` | PU cycle | states 127, 127 on disk |
| 12, 19 — TS `EquipmentCatalogEntryDto` missing `book` | frontend cycle | field + filter chips present |
| 24, 63 — root integration tests uncompilable | Phase-1 follow-up | root suite runs; 5,895 tests |
| 34 — `barbarian_features`/`monk_features` unwired | PU class-features cycle | consumed by `ground_unchained_*` |
| 66 — ARG feat magnitudes not in `pilot_compute_corpus` | `decisions.md §29.1` seam | both twins read `feat_derived_pillar_contributions` |

They are left in the ledger rather than deleted: the retro log is append-only, and a reader
returning to a shard needs to find its disposition here.

---

## 5. The full routing ledger — all 74 deferrals

Generated from `docs/retro/events/*.jsonl` on 2026-08-01, in shard-then-file order. `what` text
is truncated; the shard is the authority.

**Legend:** `C1` class 1 (SD-29 owns) · `C2` class 2 (SD-29 sequences) · `C2*` conditional ·
`C3` candidate only, not claimed · `SD-28` routed to the Ultimate bundle · `ELSEWHERE` routed to a
named non-SD-29 owner · `CLOSED` verified resolved at HEAD.

| # | retro shard | deferral (truncated) | routing |
|---:|---|---|---|
| 01 | `alternate-racial-trait-agent` | 142 of ARG's 153 alternate racial traits still change no computed number; their declared bonuses are… | **C3** |
| 02 | `alternate-racial-trait-agent` | The engine gates standard-trait suppression only for Dwarf's 6 modelled records; Elf/Gnome/Half-Elf/H… | **ELSEWHERE** |
| 03 | `arg-desc-render` | Halfling ~ Adaptable Luck's reduced-bonus magnitude (DESC arg 2 = Halfling_AdaptableLuck_Bonus-1) is… | **CLOSED** |
| 04 | `arg-desc-render` | src/bin/ingest_races.rs still lacks '%%' handling: substitute_placeholders does not collapse the lite… | **C2** |
| 05 | `book-coverage-truth-agent` | Correct core_rulebook + beastiary LICENSE.json record counts and extend tests/sd27_book_license_recor… | **CLOSED** |
| 06 | `book-coverage-truth-agent` | Fix records_processed at its source: src/bin/sd27_gen_book_cache.rs writes each LICENSE.json from wha… | **C2** |
| 07 | `book-coverage-truth-agent` | Add the PI-blacklist term scan to src/bin/ingest_race_traits_arg.rs, which classifies all 156 ARG rac… | **C2** |
| 08 | `book-coverage-truth-agent` | Report ARG's 156 racial-trait records on the Corpus Ingest panel | **C3** |
| 09 | `book-coverage-truth-agent` | Declare an executed reach claim for pathfinder_unchained/class_features instead of an OPEN_FINDINGS e… | **C3** |
| 10 | `book-coverage-truth-agent` | 3 ARG racial-trait records reach no player surface: 'Feral ~ Languages' and 'Scion of Humanity ~ Lang… | **ELSEWHERE** |
| 11 | `closeout` | The reach gate is structurally blind to pathfinder_unchained. Neither discovery source sees it: corpu… | **C2** |
| 12 | `closeout` | Frontend/TypeScript follow-ups the widened catalogs opened: (1) apps/desktop/src/boundary/loadEquipme… | **CLOSED** |
| 13 | `closeout` | v06_work_inventory.rs's rule_set_for() still maps only core_rulebook/advanced_players_guide/advanced_… | **C2*** |
| 14 | `desc-sweep` | fixing monster_catalog.rs's race_subtype, which serves the raw PCGen multi-value separator to a playe… | **CLOSED** |
| 15 | `desc-sweep` | wiring the newly-served equipment description through to a screen | **CLOSED** |
| 16 | `desc-sweep` | decoding the PCGen &nl; entity in src/bin/ingest_pu_classes.rs's private description treatment | **ELSEWHERE** |
| 17 | `equipment-surface` | labelling the source book on Add Equipment picker rows: itemPickerFilter.ts's mapEquipmentCatalogEntr… | **C3** |
| 18 | `equipment` | reach_gate.rs's ('apg'\|'acg'\|'beastiary1'\|'advanced_race_guide', 'equipment') OPEN_FINDINGS entries a… | **CLOSED** |
| 19 | `equipment` | adding 'book' to the TypeScript EquipmentCatalogEntryDto and a book filter/chip row to EquipmentCatal… | **CLOSED** |
| 20 | `equipmod-reach-agent` | Two pre-existing pricing defects found while fixing the dead affordance, deliberately NOT fixed: (1)… | **C3** |
| 21 | `featmate-arg-deferrals` | 24 of ARG's 49 unconditionally-bonused feats remain ungrounded (25 of 49 counting the pre-existing Gr… | **C3** |
| 22 | `featprereq` | 14 of the catalog's 31 PRE token kinds are reported as unverifiable rather than evaluated: PREALIGN,… | **C3** |
| 23 | `feats` | Did not add ARG/PU prerequisites, effect or source_page to the book-spanning feat aggregate; feats_al… | **C3** |
| 24 | `feats` | Could not run the root crate's integration tests through cargo (cargo test --test v06_apg_acg_feat_ca… | **CLOSED** |
| 25 | `featwire-agent` | 32 of ARG's 49 unconditionally-bonused feats left unwired, by category rather than one at a time | **C3** |
| 26 | `featwire-agent` | Armor of the Pit's Scaled Skin branch grounds no resistance number | **C3** |
| 27 | `four-surface-truth-agent` | The 3 remaining on-disk spell records carrying an unescaped '%%' PCGen escape: data/corpus/advanced_p… | **ELSEWHERE** |
| 28 | `prereq` | Stale 'not wired into RuleSetId' doc claims left in 4 files outside Phase-1 ownership: src/rules_core… | **ELSEWHERE** |
| 29 | `pu-class-features` | 6 of PU's 23 no-magnitude class features stay uncomputed: Monk Purity of Body, Monk Tongue of the Sun… | **C3** |
| 30 | `pu-class-wiring-agent` | The Unchained Summoner's own 202-spell list (12/35/39/39/27/23/27 at spell levels 0-6) is not transcr… | **SD-28** |
| 31 | `pu-classes-ingest` | PU's selectable-option pools: 54 Unchained Rage Powers, 31 Ki Powers, 27 Rogue Talents, 16 Advanced R… | **C3** |
| 32 | `pu-classes-ingest` | data/corpus/pathfinder_unchained/LICENSE.json records_processed=59, now understating the book at 128… | **CLOSED** |
| 33 | `pu-classmate-barb-monk` | Unchained Monk unarmed-strike damage progression and Stunning Fist save DC / uses-per-day are not mod… | **C3** |
| 34 | `pu-classmate-barb-monk` | barbarian_features.rs / monk_features.rs are not wired into pilot_compute.rs, the desktop app, or any… | **CLOSED** |
| 35 | `pu-classmate-barb-monk` | BONUS:VAR\|FAB_7\|FAB (pu_abilities_class.lst:503) is transcribed in the doc table but flurry_iterative… | **C3** |
| 36 | `pu-classmate-barb-monk` | Archetype suppression (PREVAREQ:<Class>_CF_<Feature>,0 gates on every progression row, plus the nine… | **C3** |
| 37 | `pu-rogue-summoner-features` | summoner_spell_list.rs -- the 202-spell Unchained Summoner spell list from pu_abilities_class.lst:269… | **SD-28** |
| 38 | `pu-rogue-summoner-features` | tests/sd27_pu_class_features.rs \[landed as `tests/sd27_pu_class_features_reach_by_corpus_key.rs` — filename corrected 2026-08-10\] -- a PCGEN_CORPUS_ROOT-gated test pinning both modules' formulas agai… | **ELSEWHERE** |
| 39 | `race-creation-widening-agent` | Widen character creation from 7 races to all 18 corpus races, corpus-driven | **C3** |
| 40 | `race-creation-widening` | PF1 size modifiers to AC, attack rolls, CMB/CMD and Stealth for Small races (Gnome, Halfling, Goblin,… | **CLOSED** |
| 41 | `race-creation-widening` | height/weight profiles for the 11 Bestiary 1 races now creatable | **C2** |
| 42 | `race-size-fix` | race_tables.rs::race_size_for_race_id left in place with zero production callers, and its module test… | **ELSEWHERE** |
| 43 | `record-gaps` | Duergar ~ Spell-Like Ability ~ Invisibility remains the one unreached beastiary1 race-trait record | **C1** |
| 44 | `removal-agent` | Post-creation removal of alternate racial traits | **C3** |
| 45 | `removal-agent` | Boundary shim apps/desktop/src/boundary/removeSelection.ts(+.test.ts) was written outside the brief's… | **C3** |
| 46 | `sd27-closer` | Correct core_rulebook/LICENSE.json (3326 recorded vs 3400 on disk) and beastiary/LICENSE.json (45 vs… | **CLOSED** |
| 47 | `sd27-reach-defects` | The direct-ABILITY grant edge: an ARG alternate that names its replacement row with ABILITY:<Race> Ra… | **ELSEWHERE** |
| 48 | `sd27-reach-verify-agent` | The Wizard prepared-spell level gate covers CRB spells only. A Wizard 1 is correctly refused all 508… | **ELSEWHERE** |
| 49 | `sd27-reach-verify-agent` | The Add Spell picker offers the whole 1185-row catalog with no class or level filter (CharacterSheet.… | **C3** |
| 50 | `sd27-verify-agent` | Small-creature touch AC, CMB and CMD are still wrong on the live sheet: they are computed in React (C… | **CLOSED** |
| 51 | `sd27-verify-agent` | ARG's 153 alternate racial traits can be browsed and their swaps resolved live in the Race Traits scr… | **C3** |
| 52 | `sd27-verify-agent` | The Class Progression browser lists 300 rows across 15 classes (11 CRB + 4 PU) while character creati… | **C3** |
| 53 | `sd27-verify-agent` | Widening attach_equipment_modifier_at_root beyond CRB so the 57 non-CRB equipmods the picker offers c… | **C3** |
| 54 | `size-modifier-agent` | PF1 size modifiers to touch AC, CMB and CMD (Small: touch +1, CMB -1, CMD -1) | **CLOSED** |
| 55 | `size-modifiers-agent` | CMD does not yet include the deflection/dodge/insight/luck/morale/profane/sacred bonuses to AC that P… | **C3** |
| 56 | `size-modifiers-agent` | apps/desktop/src/characterHub/previewData.ts was not given defense.touch_armor_class / combat.combat_… | **ELSEWHERE** |
| 57 | `spell-surface` | SpellCatalogScreen still renders the short wire code (CRB/APG/ACG/ARG) as the visible text on both th… | **C3** |
| 58 | `spellgate-agent` | Gate the Known/spellbook path: the app's Add Spell writes AcquisitionMode::Known with no check that t… | **C3** |
| 59 | `spellgate-agent` | src/bin/sd27_gen_book_cache.rs still #[path]-includes rules_tables/advanced_race_guide/mod.rs a secon… | **C2** |
| 60 | `spellgate` | PF1's minimum-ability rule for LEARNING a spell: CRB Wizard, 'To learn, prepare, or cast a spell, the… | **C3** |
| 61 | `spellgate` | The Add Spell picker still offers all 1185 records to a class with no ingested spell list, which incl… | **C3** |
| 62 | `spells` | Three ARG-spell follow-ups outside this brief's file ownership: (1) reach_gate.rs OPEN_FINDINGS still… | **CLOSED** |
| 63 | `spells` | Could not run tests/spell_cross_book_identity.rs (the root-crate test this module's doc comment cites… | **CLOSED** |
| 64 | `verify-agent-t71` | Fix raw PCGen DESC syntax still reaching the player in ARG alternate-trait descriptions: Halfling ~ A… | **CLOSED** |
| 65 | `verify-agent-t71` | Widen character creation's race list beyond the 7 CRB races. RACE_OPTIONS in apps/desktop/src/charact… | **C3** |
| 66 | `verify-agent-tranche7` | Wire the ARG feat magnitudes into pilot_compute_corpus.rs (armor_of_the_pit natural armor -> compute_… | **CLOSED** |
| 67 | `verify-agent-tranche7` | Make the sheet's Vision cell honour a vision-replacing alternate racial trait (e.g. ARG Halo, which r… | **C3** |
| 68 | `verify-agent` | Pathfinder Unchained's 4 classes and 64 class features are in data/corpus/pathfinder_unchained/ and i… | **C3** |
| 69 | `verify-agent` | data/corpus/pathfinder_unchained/LICENSE.json still reports records_processed: 59; the book now holds… | **CLOSED** |
| 70 | `verify-agent` | Size modifiers to AC, touch AC, CMB and CMD are still not applied for any race. A live Goblin Fighter… | **CLOSED** |
| 71 | `verify` | The Add Item picker (mapEquipmentCatalogEntries in apps/desktop/src/characterHub/itemPickerFilter.ts)… | **C3** |
| 72 | `verify` | SpellCatalogScreen's BOOK_ORDER is a hardcoded literal and its test pins it against another hardcoded… | **C2** |
| 73 | `verify71` | Fix flat-footed AC to drop dodge-typed bonuses, by moving the cell out of CharacterSheet.tsx and into… | **CLOSED** |
| 74 | `verify72` | Route equipment descriptions through render_pcgen_desc in apps/desktop/src-tauri/src/equipment_catalo… | **CLOSED** |


**Totals:** C1 **1** · C2 **7** · C2\* **1** · C3 **31** · SD-28 **2** · ELSEWHERE **10** ·
CLOSED **22** — **74**.

---

## 6. Pick-three summary

The operator's three lever decisions for SD-29:

1. **Class 1 — accept, but re-size first.** The 7 Bestiary-line stubs are the payload, and
   Monster Codex closes the project's last reach NO. But §1.3 shows the books are **not** uniform:
   three real monster books, one small one, one racial-options book, and two that are not monster
   books at all. Accepting class 1 means accepting a **non-templated** cycle plan.
2. **Class 2 — own §2.1–§2.7, check §2.8.** Seven pipeline debts that a 7-book fan-out multiplies,
   each with a `revisit` that names this bundle's own trigger condition. §2.8 belongs to SD-27
   first; verify before claiming.
3. **Class 3 — drop.** 31 engine/UI deferrals with no documentary binding to SD-29. Not this
   bundle's, on no authority. Two are flagged as adjacent in §3 without being claimed.

Bind points the operator must resolve before any cycle dispatches:

- **Book shape.** Does `bestiary_5`/`bestiary_6`/`monster_codex` get a monster-block epic
  (yielding almost nothing), a different cycle shape, or an explicit carve-out? §1.3.
- **Whether §2.1 is a prerequisite.** Ingesting 7 books through a generator that writes the wrong
  compliance number reproduces the defect seven times, and `LICENSE.json` is a redistribution
  artifact. This register's authoring cycle recommends fixing it **before** cycle 1, not after.
- **Predecessor state.** SD-27 must be closed, and §2.8's disposition known, before SD-29's class-2
  list is final.

---

## Pitfalls (inherited from SD-27's register, plus two earned here)

- **Don't inflate class 1 from the retro log.** Exactly **one** of 74 deferrals routes itself to
  SD-29. A register that claimed more would be a scope takeover wearing a citation.
- **Don't absorb the Ultimate-blocked items.** §4.1. They name their blocker explicitly, and it
  is another bundle's book.
- **Don't author `scope-draft.md` until the operator accepts this register.** SD-27's own pitfall
  list, repeated here because the failure it prevents is silent.
- **Don't size a book from its name.** `monster_codex` has 2 monsters and `bestiary_6` has none.
  Both were sized as ~250-monster books by a plan written without opening the corpus. Derive book
  shape from the `.lst` files first — this is the third recorded instance.
- **Don't close the Duergar finding by hiding the record.** The test going red **is** the
  deliverable. A record on disk that no selection can reach is exactly what the reach gate is for.

---

# §7. AMENDMENT — 2026-08-01, tranche/8 retrospective reconciliation

> **Read this before acting on §4.3 or §5.** This amendment is additive. Nothing above is deleted:
> the register is a signed-off surface and the corrections belong beside it, not on top of it. Where
> §7 and an earlier section disagree, §7 is the later measurement and says so per row.
>
> Authored by the tranche/7 retrospective (`docs/retro/tranche-7-retrospective.md`) after the
> deferral corpus was re-derived at `tranche/8` HEAD. Every row was verified by command; the
> commands are given.

## 7.1 The corpus this register routes has grown, and nine deferrals are unrouted

The register's actor multiset matches the first **74** deferral events exactly — it is not in
disagreement with the log, it predates part of it. The tranche/7 log now holds **83**.

```bash
python3 scripts/retro.py query --type deferral --json | python3 -c "
import json,sys
print(len([e for e in json.load(sys.stdin) if e['actor'] not in
  {'retro-corrections-analyst','deferral-mining','tranche8-incident-retro','tranche7-retro-synthesis'}]))"
# -> 83
```

| # | shard | what | disposition |
|---:|---|---|---|
| 75 | `sd29-scope-and-debt` | APG + ACG `LICENSE.json` state no `records_processed` at all (641 and 423 records on disk) | **SD-29 — folds into §2.1.** Confirmed absent by command; ARG 635 ✓, beastiary 164 ✓, CRB 3400 ✓, PU 127 ✓ |
| 76 | `verify-reach-reissue` | ARG racial-trait display-value renderer has zero callers | ELSEWHERE — a frontend-surface cycle |
| 77 | `pu-description-resolution` | Extra Rage / Extra Ki move PU display variables, applied to neither the sentence nor the magnitude row | ELSEWHERE — the cycle that owns `feat_effects.rs` |
| 78 | `traitreach` | Create form renders alternate-trait numbers at racial base, never feat-moved | ELSEWHERE — creation-form cycle |
| 79 | `monkslug` | `classFeaturesModel.test.ts:189` enshrines the `maker_s_call` defect's output | ELSEWHERE — **same root cause as 80; fix 80 first** |
| 80 | `sd27verify` | `ingest_pu_classes::slugify` promotes `'` → `_` into corpus filenames | ELSEWHERE — a corpus re-ingest with `sha256` re-pinning, scheduled alone |
| 81 | `sd27verify` | No stable predicate for "carries a computed magnitude"; four variants give 48/49/51/52 on one tree | **SD-29-adjacent, and see §7.5 — resolve before publishing any coverage ratio** |
| 82 | `vendor-ge05-fixtures` | `DEFAULT_PCGEN_REPO_DIR` hardcoded | **CLOSED** — `pcgen_runner.rs` now joins `DEFAULT_PCGEN_REPO_DIR_REL` to `$HOME` |
| 83 | `home-paths` | 6 remaining `/home/ubuntu` literals outside `SCANNED_DIRS` | ELSEWHERE — **and see §7.4** |

## 7.2 §4.3 closed one row on another row's evidence — ledger row 03 is OPEN

§4.3 groups **rows 03 and 64** as *"raw `%%`/`%N` in ARG alternate-trait descriptions … 0 shipped
`race_trait` descriptions carry either."*

**Row 64 is genuinely closed. Row 03 is a different deferral and it is open.** Its `--what` is:

> *"Halfling ~ Adaptable Luck's reduced-bonus magnitude (DESC arg 2 =
> `Halfling_AdaptableLuck_Bonus-1`) is dropped from the served description, so the row reads 'they
> only gain a bonus' with no number."*

```bash
python3 -c "import json;print(json.load(open(
 'data/corpus/advanced_race_guide/race_trait/halfling/halfling_adaptable_luck.json'
))['data']['description'][-120:])"
# -> "...if they choose to do so afterward, they only gain a bonus. Using adaptive luck in this
#     way is not an action. This racial trait replaces halfling luck."
```

**Still no number.** Row 03 is blocked on one operator ruling and nothing else: does
`<SameRowVar> ± <int literal>` count as *transcription* (allowed) or *interpretation* (forbidden by
`decisions.md §24.1`)? Its own `--revisit` names both exits — a §24.1 hand-modelled pure function, or
that ruling. It is one row of 156, and the deferral calls it *"the only unresolvable DESC arg in the
whole ARG in-scope set."* **Cheap, and currently recorded as done when it is not.**

This is the same failure shape `decisions.md §27.2` already recorded: a general disposition inferred
from one sample. Logged as a correction in `docs/retro/events/deferral-mining.jsonl`.

## 7.3 Seven ledger rows understate progress — they are CLOSED at `tranche/8` HEAD

§4.3's CLOSED column was verified by command. These seven were not, and they are exactly the rows a
successor would waste a cycle re-scoping.

| ledger row | register says | actual | verification |
|---|---|---|---|
| 39, 65 — widen creation from 7 to 18 races | `C3` | **CLOSED** | `raceRoster.ts` is corpus-driven via `list_race_creation_roster` (`:148`); `RACE_OPTIONS` is gone; the module doc names the 18-race roster |
| 47 — direct-`ABILITY` grant edge | `ELSEWHERE` | **CLOSED** | `race_resolver.rs:41` documents `ABILITY:<cat>\|AUTOMATIC\|<key>` as a `FlagGranted` edge; `:57` names `Orc ~ Feral` |
| 48 — Wizard spell-level gate is CRB-only | `ELSEWHERE` | **CLOSED** | commit `f4dcb522`; `class_spell_levels.rs` chains `acg`, `advanced_race_guide`, `apg`, `crb`; `crb/wizard_spell_list.rs:610` = `("Tsunami", 9)` |
| 51 — alternates cannot be applied to a character | `C3` | **CLOSED** | `character_hub.rs:385` `pub selected_alternate_trait_keys: Vec<String>` |
| 10 — 3 ARG records reach no surface | `ELSEWHERE` | **CLOSED** | `reach_gate.rs:1416` `UNREACHED_RECORD_FINDINGS` now holds exactly one entry: `("beastiary1","race_traits",["Duergar ~ Spell-Like Ability ~ Invisibility"])` |
| 11 — reach gate blind to PU | `C2` (§2.2) | **CLOSED** | `slice_element_type` matches `pub fn name() -> &'static [Type]` (tested at `reach_gate.rs:2031–2047`); `corpus_ingest_diagnostic` now lists `advanced_race_guide` and `pathfinder_unchained` |

**Net disposition: 39 of 83 verified closed at `tranche/8` HEAD** (the register's 22, minus row 03,
plus these seven and events 8, 31-partial, 82). **44 open**, deduplicating to **51 survivors**.

**Row 10's closure is also the confirmation of this bundle's headline.** `UNREACHED_RECORD_FINDINGS`
holding exactly one entry, and that entry being Duergar's Invisibility SLA, is the executable form of
`README.md §1`: Monster Codex closes the project's last reach NO. It is now a one-element set, so a
future reader can check the claim in one command instead of trusting the prose.

## 7.4 §2.3 is one third of a defect — F-group re-route

§2.3 records that `src/bin/ingest_races.rs` has no `%%` handling and that `leaked_pcgen_syntax` does
not flag it. Two further deferrals record the **same finding about two other binaries**, and §4.2
routes both `ELSEWHERE`:

- 3 APG spell records on disk carry unescaped `%%` (`chameleon_stride`, `fiery_body`,
  `ghostbane_dirge`), written by `cache_gen/apg.rs`, which has *its own* desc treatment.
- `src/bin/ingest_pu_classes.rs` has *its own copy* of the desc treatment rather than using
  `render_pcgen_desc`, and leaks `&nl;` into one PU record.

**The register currently has three private copies of one description treatment classified into two
different buckets.** Fixing only the `C2` third means SD-29 ships one third of a defect.

> **The readiness statement, and it is the thing SD-27 wished it had known on day one: there is no
> single ingestion pipeline. There are four binaries with four partial copies of the description
> treatment, and only `codex::rules_core::pcgen_desc::render_pcgen_desc` is the sanctioned one.**

**Collapse §2.3 and both `ELSEWHERE` siblings into one readiness task**: route every ingest binary's
description path through `render_pcgen_desc`, and give each a `leaked_pcgen_syntax` production guard.
Done once before book #1, it never recurs. Done per book, it is paid seven more times here, in four
places, by seven different agents — **and six more times in SD-28 and four in SD-30**.

Related, same paragraph of ownership: **`src/bin/sd27_gen_book_cache.rs:73` still `#[path]`-includes
`rules_core/rules_tables/advanced_race_guide/mod.rs` a second time into its own crate**, which is the
sole reason several ARG items report dead code. Confirmed present at line 73. Deleting it should drop
several warnings off the 75-warning root clippy baseline — a §2.6 item that is still live.

## 7.5 Resolve the magnitude predicate before publishing any coverage ratio

Unrouted deferral 81 (§7.1) is a methodological blocker, not a task:

> *"Every ratio published for PU class features so far (23, 32, 46, 49, 51) is a different predicate,
> not a different tree."*

Magnitude rows carry no corpus key, only prose that *usually* repeats the record's name. Four
reasonable variants of the name-substring test return 48/49/51/52 on one tree. The fix is small — an
optional `source_record` on `ComputationExplanation` — and its absence is expensive:

**SD-28, SD-29 and SD-30 will each want to publish a "% of records that reach a player" figure. Without
this, they will publish figures that are not comparable with each other or with SD-27's, and every one
of them will be defensible.** This is `decisions.md §27.1`'s finding — *"625 mentions vs 271 settings;
the arithmetic was never the defect, the label was"* — recurring one layer up.

**Whichever book bundle dispatches first should land it. It is cheaper than any of the three will
spend arguing about their own numbers.**

## 7.6 Shared ownership with SD-28 and SD-30 — pay §2 once

`docs/release/SD-28-ultimate-book-content-ingestion/` and
`docs/release/SD-30-class-feature-archetype-bundle/` are the operator's canonical sibling packages,
landed on `tranche/8` (`6452ef0d`, `26b5155c`, `721c2949`). This amendment defers to their slugs and
scope. Both **claim** §2.1–§2.7 rather than restating it, and both defer to
this register as the canonical statement.

**The ownership rule, agreed across all three registers: whichever bundle dispatches first pays
§2.1–§2.7; the others re-verify rather than re-implement.** Neither of the other two schedules it.

Two genuinely cross-bundle items, named so neither side assumes the other did it:

- **§2.7 (`SpellCatalogScreen`'s `BOOK_ORDER` literal pinned against a literal)** fires hardest for
  SD-28 — Ultimate Magic alone carries 827 spell rows — and again for SD-30's Occult Adventures
  (472 spell keys not in any ingested book). The one-line criterion for all three: **derive the book
  list from the loaded data, the way `EquipmentCatalogScreen` already does.**
- **§4.1's Ultimate-blocked pair is now derived, not asserted.** SD-28's register publishes the exact
  46-spell gap by level and defining book. Its finding for SD-29: *every one of the 46 comes from
  `ultimate_combat` or `ultimate_magic`* — **none requires a Bestiary-line book**, so §4.1's
  "do not absorb" ruling is confirmed by measurement rather than by routing.

## 7.7 Traps SD-29 inherits — name them before cycle 1

`README.md §2` correctly rules out engine work and cites `decisions.md §24`. It does not yet name the
four architectural traps `decisions.md §29` records, and each will bite SD-29 the way it bit SD-27.
Cited, not restated:

| trap | § | shape in this bundle |
|---|---|---|
| **Two compute twins** | §29.1 | `pilot_compute.rs` vs `pilot_compute_corpus.rs`. **A magnitude is not wired until it moves on the twin the player reads.** 15 of SD-27's 115 corrections were this class. |
| **A third twin, in TypeScript** | §29.2 | Any surface re-deriving a rules number instead of rendering an engine `explanations` row. One live instance remains: `CharacterSheet.tsx:2945`. |
| **Reach-gate blind spots, one permanent** | §29.3 | §24-shaped hand-modelled functions emit no slice and are invisible to the source scan by construction. **No family may rest on a single discovery source** — the corpus directory is load-bearing. |
| **`p.xx` is a placeholder** | §29.4 | Checked per row, never per content-kind. Verbatim transcription would have manufactured 143 false citations in SD-27. **Seven books × this decision is where SD-29 fabricates provenance if it inherits nothing else from this table.** |

And the process half, from `docs/retro/tranche-7-retrospective.md §6`: every figure in a dispatching
brief ships with the command that produced it; every ratio ships with its predicate; `FILES YOU OWN`
must be closed under the change it mandates; one writer per tree with its own `CARGO_TARGET_DIR`,
deleted at the end; and a verification stage red for more than one run is a blocker, not a background
condition.

## 7.8 Amendment cross-reference

- `docs/retro/tranche-7-retrospective.md` — the evidence base for §7, including §7 of that document
  on why the ~39 unrouted engine/UI deferrals must not be distributed across book bundles.
- `../SD-28-ultimate-book-content-ingestion/forward-scope-register.md` §1.2 — the derived 46-spell gap
  confirming §4.1.
- `../SD-30-class-feature-archetype-bundle/forward-scope-register.md` §1.2 — three shipped-code
  citations found by grepping source for book names; **SD-29 should run the same grep for its own
  seven books before concluding §3 is complete.**
- `docs/retro/events/deferral-mining.jsonl`, `docs/retro/events/tranche7-retro-synthesis.jsonl` —
  the corrections behind §7.1–§7.4, each with its `verified_by` command.

## 7.9 New owner of record (2026-08-13, pointer only — no scope duplicated here)

SD-29 is closed (`decisions.md §70`). Its per-book ingest lanes have been folded into SD-30 by
operator ruling — see `../SD-30-class-feature-archetype-bundle/decisions.md §44` for the ruling and
the new dispatchable cards.
