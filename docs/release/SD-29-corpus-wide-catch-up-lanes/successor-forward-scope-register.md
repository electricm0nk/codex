# Successor Forward-Scope Register — SD-29

> **Renamed at package consolidation (2026-08-01).** This file originated in the
> `SD-29-bestiary-2-3-4-5-content-ingestion` package under the name
> `forward-scope-register.md`. During consolidation into this directory it collided
> with a differently-scoped, differently-authored file of the same name that
> originated in this package (`./forward-scope-register.md`, authored by the
> `tranche/7-1` debt cycle) — that file routes work **into** SD-29 from the
> predecessor's retro log and is the operator sign-off gate; this file instead
> tracks work **downstream of** SD-29 (successor/retrofit dependencies). Both are
> real and neither supersedes the other; this one was renamed, not deleted, so
> both survive under distinct names. See `README.md` for the reconciliation note.

This register captures work downstream of SD-29. SD-29's successor bundle
(SD-30) is recorded as **Class 1** (named successor). Bundles that depend
on SD-29's bestiary outputs but aren't yet named land in **Class 2**.
SD-29-specific retrofits land in **Class 3**.

> **Note, 2026-08-10 (`decisions.md §38.5`).** The "successor" framing above predates the
> corpus-wide re-scope. SD-30 is no longer cleanly downstream of SD-29's output — its sixteen-book
> list is now a *subset* of SD-29's corpus-wide lane scope, a collision rather than a
> dependency. See `risks-and-open-questions.md` R-29-009/OQ-29-004; not resolved here.

## Ownership audit at SD-29's closure (2026-08-13, `decisions.md §70`)

Decision 27 requires every deferred finding to name an owner; an unowned deferral is not a valid
disposition. Audited entry by entry at closure run 3, and every deferral in this register carries
one:

| Entry | What it is | Owner |
|---|---|---|
| C1.4a | Frontend preview fixtures are hand-authored, not corpus-derived (`companionCatalogRuntime.ts`) | **SD-31** |
| C1.4b | `wired-integration-audit.sh` ↔ `sd24_wired_integration_audit.rs` `placeholder` parity — the bundle's one RED instrument | **SD-31** |
| C1.4c | Decision 41 does not say whether NEW `tests/` files may carry a bundle tag — needs a ruling | **SD-31** |
| C1.4d | `equipment` and `spell` `SEARCH_Y` constants are uncalibrated in `verify-on-screen.sh` | **SD-31** |
| C1.5 | The `ABILITY:Internal\|AUTOMATIC\|` bundle hop — 229 monster rows across six books | **SD-31** |
| C1.6 | `ASPECT:` is modelled by no chassis — 34 grounded companion rows diminished, 1 emptied | **SD-31** |
| C1.3 | `class_feature`, 15,472 units | **SD-30** |
| C3.1 | DM Toolkit extension as retrofit (Epic 8's `decision-blocked` ruling) | **operator-on-request** |
| C3.3 | Retroactive magnitude-fidelity sweep over already-landed `static` units | routed 2026-08-12 |

**One ceiling is deliberately NOT in this register, and that is the honest disposition.** The
**race-variant chassis** — the mechanism the 3 `bestiary` Drow Noble rows and the 2 `core_essentials`
`PREABILITY`-gated subrace selectors need — has **no owner**. It is not a review finding; it is a
structural ceiling outside any SD-29 or SD-30 epic, and it needs an operator scope decision before it
has one. Adding it here under SD-31 to make this table look complete would be the same taxonomy abuse
that produced the premature 2026-08-11 closure. It is stated as a real, measured ceiling in
`release-notes.md` §Known issues 2 with `Owner: unassigned` written out (`decisions.md §70.3`).

## Class 0 — Doctrinal anchors (always-on)

| Anchor | Path | Note |
|--------|------|------|
| Per-book ingest pipeline | `docs/governance/book-ingestion-playbook.md` | Doctrine-of-record; pre-cycle trap-report is mandatory |
| Reach gate | `apps/desktop/src-tauri/src/reach_gate.rs` | Definition-of-done per `decisions.md §19`; gate's `OPEN_FINDINGS` carries the Bestiary-1-monster-surface prerequisite |
| Identifier discipline | `~/workspace/governance/identifier-discipline.md` | SD-29 inherits; Epic 1 enforces |
| Build-version scheme | `<major>.<tranche-base>.<build>` (2026-07-17 amendment) | SD-29 first concrete value `0.9.<build>` |
| Source STC chassis | `spec-domain-bundle-authoring` skill | 15-file shape per the modern chassis (SD-22 through SD-28) |
| Move-not-copy publish | `release-package-promotion` skill | Workspace tree removed on publish commit |
| Hermes board retirement | SD-28 `decisions.md §15a` (2026-08-01) | All post-2026-08-01 bundles are local-file only |

## Class 1 — Predecessor-deferred (named successor owns)

### C1.1 — Ingest cycle consumes `data/corpus/beastiary1/` from SD-22

**Owner:** SD-29 itself.

**What depends on SD-29:** no upstream dependency. SD-29 reads
`data/corpus/beastiary1/` as a reference shape for its own monster slices
but doesn't need it as a cycle dependency — Bestiary 1 records are
already canonical.

### C1.2 — DM Toolkit extension (consume SD-29's monster records)

**Owner:** SD-29 itself (Epic 8; was Epic 7 under the retired per-book numbering — `decisions.md §37`).

**What depends on SD-29:** the extension consumes Epic 5's monster/monster-ability chassis records
(was Epic 4 before the 2026-08-10 corpus-wide re-scope, `decisions.md §38`) for the encounter
builder + party-CR math. Epic 8 is gated on Epic 5's pilot cycle-batch (Bonus Bestiary) landing, and
can consume the remaining books incrementally as Epic 5's extension cycle-batches close — it does
not wait for every lane.

**Status: SUPERSEDED BY C3.1 — ruled 2026-08-11.** The safe default this entry anticipated was
taken: `epic-8-toolkit` is `decision-blocked`, Epic 8 does not land inside SD-29, and ownership
moves out of this bundle to the Class 3 retrofit at **§C3.1** (now ACTIVE), which carries the
evidence. SD-29 is no longer the owner of the DM Toolkit extension. Preserved above as the record
of what was planned, per `decisions.md §19`.

### C1.3 — `class_feature` (15,472 units corpus-wide) inherits `corpus-work-channels.md §9.1`'s per-class archetype funding

**Owner: ASSIGNED 2026-08-10 — SD-30** (`docs/release/SD-30-class-feature-archetype-bundle/`, renamed
via `git mv` from `SD-30-occult-and-companion-content-ingestion` the same day). Not SD-29. Previously
recorded as "not yet assigned an SD number"; the operator's 2026-08-10 directive closed that gap by
re-scoping SD-30 (whose old sixteen-book list this same directive dissolved, see OQ-29-004/R-29-009
above) into the `class_feature` bundle — the exact assignment this entry anticipated. **Widened from
90 units to 15,472 units by the 2026-08-10 corpus-wide re-scope** (`decisions.md §38.4`); originally
added by the kind-lane re-cut (`decisions.md §37.4`) at the retired seven-book, 90-unit figure.

**What depends on SD-29:** nothing — this is the reverse relationship. `class_feature` (15,472
units, 40.2% of the corpus) is Channel D per `../corpus-work-channels.md §3`/`§5.4`: blocked behind
the archetype mechanism and per-class chassis (SD-28 `§60`/`§63`), corpus-wide sizing funded
(`§9.1`) but not yet measured per-class. Explicitly excluded from every SD-29 lane, including the
now-corpus-wide Epic 4 (proven-path lane scopes only the settled-method kinds). Ingests once the
`§9.1` measurement reaches the relevant classes — tracked here so it is not silently dropped, and
not silently folded into a lane whose method does not fit it.

### C1.4 — Deferred findings from Epic 10, Bundle Code Review RUN 2 (2026-08-13, `decisions.md §66`)

Decision 27 requires every deferred review finding to name an owner; an unowned deferral is not a
valid disposition. Four items from the run-2 review are deferred here. Two of them are **fixed in
this bundle** (`§66.2`, commit `4d22ecbb`) and are not listed.

**Owner: SD-31** (`docs/release/SD-31-pcgen-character-import/`), as the next bundle in the program;
re-assign here if a different successor is cut first. Naming an owner is what makes a deferral a valid
disposition under Decision 27 — an unowned deferral is not one.

**What depends on SD-29:** nothing. These are defects and debts SD-29 surfaced and chose not to
close; none of them blocks Epic 8 (Closure Epilogue), and none of them is a shipped-path defect.

#### C1.4a — Frontend preview fixtures are hand-authored rules data with nothing pinning them to the corpus

`apps/desktop/src/companionCatalog/companionCatalogRuntime.ts` and
`apps/desktop/src/monsterCatalog/monsterCatalogRuntime.ts` both build a browser-preview catalog by
hand and both declare full transcription fidelity to the corpus. The companion one is **not**
faithful: `Familiar (Clockwork Spy)` serves 1 of 6 stat adjustments (omitting `CHA −10`) and 1 of 3
abilities. No test references `buildPreviewCatalog` in either file.

**Why it is deferred and not fixed:** transcribing the missing values by hand moves the drift rather
than removing it. The fix `§54.5` and `§65.8` both name — derive from the served response — needs a
fixture pipeline the frontend does not have, and standing that up is a design decision outside a
review card's scope.

**This is the fourth instance of one root cause in this bundle** (`§54.5`, `§54.6`, `§65.8`, `§66.3`)
and the first whose consequence is rendered rules content rather than a test roster. A successor that
touches either file should treat "derive it, don't transcribe it" as the entry condition, not as a
follow-on.

**Not shipped-path:** the branch is behind `if (!hasTauriRuntime())` and is never taken in the
desktop product. Severity is medium because of the pattern, not because a player sees it today.

#### C1.4b — `scripts/wired-integration-audit.sh` and `tests/sd24_wired_integration_audit.rs` disagree about `placeholder`

The Rust repo-wide sweep encodes three reviewed exclusion filters for the `placeholder` token and is
green in the gate. The shell script carries none of them and is therefore red at bundle scope on 13
hits the Rust gate has already adjudicated as not-stubs (2 JSX `placeholder=` attributes, 10 doc
comments about upstream corpus placeholders, 1 `#[cfg(test)]` assertion message).

**The remedy is parity, not leniency:** port the Rust gate's three documented filters into the shell
script, with cases added to a self-test for the shell script (which has none today — only
`identifier-discipline-audit.sh` has one, at `scripts/tests/`). **Do not fix this by dropping
`placeholder` from the token list.**

Until it lands, Decision 27's bundle-scope wired-integration run cannot be reported clean.

#### C1.4c — Decision 41 does not say whether NEW `tests/` files may carry a bundle tag

`§41` binds Epics 3-11 to function-based naming including "test module names", and separately exempts
`tests/` file names because 531 existing ones are load-bearing citation targets. The audit's self-test
encodes the exemption unconditionally (`scripts/tests/test_identifier_discipline_audit.sh:115`). Epic
6 then added `tests/sd29_declared_product_identity_in_shipped_race_traits.rs` — permitted by the gate,
against the convention's stated intent.

**Two valid resolutions, and this is a ruling rather than an edit:** (a) `§41` grows an explicit
"newly added `tests/` files too" clause and case 115 splits into *existing* (pass) vs *added* (fail),
with the four existing citations moved; or (b) `§41` concedes `tests/` entirely and drops the "test
module names" phrase. Severity low. A review cycle should not pick one by rewriting a tested gate.

#### C1.4d — Equipment and spell have never been verified on screen by the harness

Not a defect in anything shipped; a calibration debt. Epic 4 predates `verify-on-screen.sh`, so the
`equipment` and `spell` families' `SEARCH_Y` constants in the harness are by-analogy and have never
been exercised. **The first equipment or spell cycle after this bundle must calibrate them before
citing a PASS** — an uncalibrated `SEARCH_Y` is exactly the silent-plausible-screen failure mode
`§65.7` describes.

### C1.5 — The `ABILITY:Internal|AUTOMATIC|` bundle-ownership hop: 229 units across six books

**Owner: SD-31** (`docs/release/SD-31-pcgen-character-import/`), as the next bundle in the program;
re-assign here if a different successor is cut first. Same owner and same reasoning as C1.4.

**Status at handoff: fully scanned, counted, checked in and pinned by two executing tests.** This is
not a discovery a successor has to fund. It is a mechanism change a successor has to *execute*, and
everything needed to start it is in the repo.

#### The number, and the command that produces it

```
python3 scripts/scan_monster_ability_bundle_rows.py
book                        orphans  bundle-reachable
bestiary                        146                63
bestiary_2                       65                15
bestiary_3                       13                 9
bestiary_4                      225                61
inner_sea_gods                   81                79
ultimate_psionics                66                 2

orphan rows the `ABILITY:Internal|AUTOMATIC|` hop would reach: 229
```

Reproduced unchanged by SD-29 Epic 5's final round, on a tree that had just ingested nine further
units — the class is stable under ingest and does not decay.

#### What the mechanism is

A monster row may state its abilities **indirectly**, through a `CATEGORY:Internal` bundle row:

```
support/isg_races_b4.lst:6              The First Blade
    ABILITY:Internal|AUTOMATIC|Race Traits ~ First Blade
support/isg_abilities_races_b4.lst:8    Race Traits ~ First Blade   CATEGORY:Internal
    ABILITY:Special Ability|AUTOMATIC|…|First Blade ~ Powerful Blows (Slam)|…
```

The monster row names the bundle; the bundle names the abilities. **Neither of the repo's two
ownership passes follows that hop.** The row-named pass reads
`ABILITY:Special Ability|AUTOMATIC|` tokens *on monster rows* and never sees the bundle. The
namespaced-prefix pass matches an ability's `KEY:` namespace against a monster **key**, and here the
namespace is the creature's short name while the key is longer. Both passes therefore call these
rows orphans, and an orphan is deliberately not transcribed.

#### Why it is a ceiling correction rather than a backlog line

These are **records already-registered books do not ship** — five of the six books are registered —
not books the lane never reached. That is the more expensive kind of gap: `bestiary`, `bestiary_2`,
`bestiary_3`, `bestiary_4`, `inner_sea_gods` and `ultimate_psionics` are all live in
`monster_chassis::MONSTER_BOOKS` today and each is under-shipping.

#### Exactly what a successor must change

Following the hop widens an **ownership** pass, and ownership decides which records six registered
books emit. That is the `count-change-needs-a-sweep` hazard at its worst, and it is the reason two
consecutive SD-29 rounds derived the number and deliberately did not close it.

1. `scripts/transcribe_monster_tables.py` — the ownership pass. It must, for each monster row,
   follow every `ABILITY:Internal|AUTOMATIC|<bundle-key>` token to the `CATEGORY:Internal` row whose
   first column is `<bundle-key>` (stripping a `CATEGORY=…|` prefix and a `.MOD` suffix), and read
   that row's `ABILITY:…|AUTOMATIC|` key list as abilities of the monster.
   `scan_monster_ability_bundle_rows.py::scan_book` already implements exactly this traversal and is
   the reference; port it, do not re-derive it.
2. `scripts/classify_monster_ability_rows.py` — `classify_book` must gain the same pass in the same
   commit. Its module doc states the invariant that makes it worth having at all: *"a classification
   that used a looser rule than the transcriber would over-report reachability, which is the
   direction that ships stubs."* The two must move together or the lane's instrument silently
   diverges from the lane's output.
3. Re-run the transcriber for all six books, then `gen_book_cache` for each, then
   `v06_work_inventory`.
4. **Sweep the count pins.** Every one of the six books' `rules_tables::<book>::tests` carries an
   assertion on `monsters().len()` / `monster_abilities().len()`; `reach_gate`'s per-book expectation
   tables carry on-disk record counts; `corpus_ingest_diagnostic` and `monster_catalog` carry more.
   Grep the OLD and the NEW count for every book before committing.
5. **Two tests are designed to go red and must be updated, not weakened.**
   `rules_tables::ultimate_psionics::monster_tests::no_internal_bundle_ability_ships_yet` and
   `rules_tables::inner_sea_gods::…::no_support_directory_ability_ships_yet` exist precisely to tell
   the round that closes this that the surrounding arithmetic is stale. A red there is the mechanism
   working.

#### The split, as one command pair

```
python3 scripts/classify_monster_ability_rows.py        # workable now, by the shipped screens
python3 scripts/scan_monster_ability_bundle_rows.py     # what the hop would additionally reach
```

The second script imports the first and reuses its `classify_book` orphan set rather than defining
`orphan` a second time, so the two cannot drift and the finding stays falsifiable.

#### One caution about the classifier's own summary

`classify_monster_ability_rows.py`'s `reachable remainder` line is an **upper bound**, not an
equality, for any book with a Product-Identity cascade (`rules_tables::inner_sea_bestiary` records
why). And two of its summary lines were corrected by SD-29 Epic 5's final round — the zero-monster
bucket now distinguishes *no monster row at all* (703 units / 10 books, structurally unreachable)
from *every monster row already ships* (228 units / 4 books, which is mostly this class). A successor
reading a pre-2026-08-13 receipt will find the two conflated.

## Class 2 — Future-acquired (deferred)

### C2.1 — Bestiary 6 + Bonus Bestiary drop-in [SUPERSEDED — decisions.md §34, 2026-08-02]

**Superseded.** Bestiary 6 and Bonus Bestiary are no longer contingent
swap-in candidates for Bestiary 5 — `decisions.md §34` (operator directive
2026-08-02) commits all seven books, including these two, as in-scope
alongside Bestiary 5, not as a replacement for it. **Further superseded,
2026-08-10 (`decisions.md §37`):** the "Epics 11/12" per-book epics this
note pointed to are themselves retired. Bestiary 6's and Bonus Bestiary's
units are now distributed across the kind lanes (`epic-breakdown.md`
Epics 4-7) the same as every other book's.

**Original text, preserved as historical record:** The 07-30 scope-draft
flagged that Bestiary 5 has no `monster` records (player-options dataset).
Bestiary 6 + Bonus Bestiary are listed in the 07-30 scope-draft as drop-in
replacements for Bestiary 5. Cycle-0 trap-report + inventory runs first;
the swap fires only if operator prefers B6 + Bonus over B5's
player-options cycles.

### C2.2 — Monster catalog command and browser [RETIRED — 2026-08-01]

**Retired.** The monster catalog command and browser this item deferred
have shipped: the `("beastiary1", "monsters")` arm of `reach_gate.rs`
(`:986` as of 2026-08-10; was `:840`) carries an executed reach claim
in place of the old `OPEN_FINDINGS` entry;
`apps/desktop/src-tauri/src/monster_catalog.rs`'s `list_monster_catalog`
command is registered (`main.rs:57,197`); `MonsterCatalogScreen.tsx` is
routed via `CharacterHubPage.tsx:104-105`, reachable from a "Browse Monster
Catalog" button at `LandingScreen.tsx:353`. The deferred work this item
named no longer exists as an open item.

The surviving related item is the `beastiary1/race_traits` Duergar
`Spell-Like Ability ~ Invisibility` record — upstream-blocked on
`monster_codex`. *(Corrected 2026-08-10: it is no longer the sole
`OPEN_FINDINGS` entry — seven `<book>/archetypes` gaps recorded at SD-28
closure sit alongside it (SD-28 `decisions.md §60`/`§63`); those belong to
SD-30's class_feature/archetype bundle.)* That record is
now expected to be addressed by Epic 5's Monster Codex cycle-batch
(Race-Trait Lane; was Epic 13 under the retired per-book numbering —
`decisions.md §37`; `monster_codex` is in scope per `decisions.md §34`) —
see `epic-breakdown.md` Epic 5 and `forward-scope-register.md §1.2`.

### C2.3 — Bulk-modification retrofit

If operator requests a bulk-modification pass across ingested records
(per `decisions.md §17` — "bulk modifications deferred"), that pass is a
separate bundle. SD-29 preserves the per-cycle one-record-at-a-time
discipline.

## Class 3 — Retrofit (operator-on-request)

### C3.1 — DM Toolkit extension as retrofit — **ACTIVE (ruled 2026-08-11)**

**Status: ACTIVE.** This is no longer conditional. Card `epic-8-toolkit` was ruled
`decision-blocked` on 2026-08-11 under `loop-instruction.md` UNATTENDED MODE item 4 (the bundle's
one sanctioned instance). Epic 8 does **not** land inside SD-29; the DM Toolkit extension is a
Class 3 retrofit owned by a successor bundle. Full evidence in `progress.md`, cycle
`SD29-E8-F1-001`.

**The criterion and why it was not met.** `epic-breakdown.md` Epic 8 and `loop-instruction.md`
"Epic ordering" make Epic 8 in-scope only if a lane cycle *needed* the consumer surface to satisfy
its reach claim. It did not. Both reach claims Epic 5's pilot landed assess an
already-shipped surface:

```
git show origin/worktree-wf_3516060a-756-9:apps/desktop/src-tauri/src/reach_gate.rs \
  | awk '/^fn bonus_bestiary_(monsters|monster_abilities)_reach/,/^}/' \
  | grep -o 'assess("[a-z_]*"' | sort -u
```
→ `assess("list_monster_catalog"` — one distinct surface, and it is the monster catalog that
shipped under SD-22/C2.2, not a toolkit. Zero of the pilot's claims route through an encounter
builder or party-CR screen. No `OPEN_FINDINGS` entry names a DM-toolkit surface as its remedy
either; the seven standing entries all name an archetype picker (SD-30's). `epic-breakdown.md`
Epic 8 makes the same point from the other side: the `OPEN_FINDINGS` Bestiary-1-monster-surface
prerequisite Epic 8 would have satisfied "already [was] independently satisfied", so deferring
costs the reach gate nothing.

**What the retrofit inherits, and what it must build.** The engine half already exists and is
untouched by SD-29 — `src/rules_core/encounters.rs` (DM-toolkit encounter-difficulty computation,
landed by SD-22 Epic 6, criterion 18) and `src/rules_core/party_cr.rs`. What is missing is the
entire consumer path, which is why this is a bundle and not a cycle:

- **No IPC command.** `grep -n 'invoke_handler' -A 60 apps/desktop/src-tauri/src/main.rs | grep -icE 'encounter|party_cr'` → **0**. Neither module is reachable from the front end at all.
- **No screen.** `CharacterHubPage.tsx:112-120` renders the `dm-toolkit` mode as a `StubScreen`
  reading "Encounter building, initiative tracking, and other GM-side tools. Not built yet." That
  is an honest labelled placeholder, not a no-stub-doctrine violation — but it is also the whole of
  the surface today.

So landing Epic 8 inside SD-29 would mean building a new command, a new screen, and new reach
claims to satisfy no lane's requirement — scope no cycle needs, taken on at the cost of the lanes
that do. The safe default is the retrofit.

**Consequences to honour downstream.** `epic-10-review`'s dependency on `epic-8-toolkit` is
written "(COMPLETE or `decision-blocked`)" and is therefore satisfied; Epic 10 is not held. Class 1
entry **C1.2** ("Owner: SD-29 itself (Epic 8)") is superseded by this ruling — see the note added
there. Nothing in Epic 5's chassis is wasted: the monster/monster_ability records the retrofit
consumes land regardless, and the retrofit can start whenever a successor bundle is named.

### C3.2 — Bestiary 6 + Bonus Bestiary ingestion [SUPERSEDED — decisions.md §34, 2026-08-02]

**Superseded.** This is no longer an operator-on-request retrofit —
`decisions.md §34` commits Bestiary 6 and Bonus Bestiary as in-scope
content inside SD-29 itself, not a separate bundle. **Further superseded,
2026-08-10:** their units land via the kind lanes (`epic-breakdown.md`
Epics 4-7), not the "Epics 11/12" per-book epics this note previously
named.

**Original text, preserved as historical record:** If operator prefers
Bestiary 6 + Bonus Bestiary over Bestiary 5 (per `decisions.md §18`), a
retrofit bundle adds them. Cycle-0 trap-report + inventory produces the
per-book shape finding.

### C3.3 — Retroactive magnitude-fidelity sweep over already-landed `static` units — **OPEN (routed 2026-08-12)**

**Routed by `decisions.md §46`, which split one instrument in two.** The per-cycle half is in SD-29
now: `epic-breakdown.md` Epic 4's three feature seeds each carry a magnitude-fidelity acceptance
criterion, so no *newly landed* `static` record escapes an exact round-trip check against its corpus
literal. This entry is the other half — the sweep over records that landed **before** that criterion
existed — and it is deliberately not SD-29's work.

**What it owns.** The `static` + `ingested-magnitude` backlog: **4,582 units** against the committed
inventory (`generated_at 2026-08-11T22:28:28Z`), all six of them in Epic 4's kinds
(`equipment`, `equipment_modifier`, `spell`, `feat`, `race`, `class`) — the command is in
`epic-breakdown.md` Epic 4. Re-derive before starting; Epic 4's own cycles will have moved the
figure, downward for records that land under the new criterion and upward for ones they add.

**Why it is a bundle and not a cycle.** The per-cycle criterion is nearly free because the cycle
already holds both the corpus row and the record it just wrote. The sweep holds neither, so it must
build the corpus-side reader standalone across every magnitude-bearing field (`COST`, `WT`, `AC`,
`DAMAGE`, `RANGE`, and `BONUS` literals) over ~7,487 `static` units. Two constraints make it real
work rather than a rollup:

- **`.MOD` base-name resolution MUST mirror the determinator's** (`CATEGORY=<x>|<Base>.MOD` →
  `<Base>`; `CLASS:<Base>.MOD` → `<Base>`) or the sweep and the inventory will disagree about which
  record a row belongs to and the check will silently skip rows — the identical hazard
  `wiring-class-determination.md` records for the closure pass.
- **It is a new test binary in the `reach_gate.rs` family**, not an extension of it. The reach gate
  asserts presence in an IPC response; this asserts textual fidelity to a corpus line. Sharing the
  inventory-construction machinery is welcome, sharing the assertion is not.

**What it unblocks, and what it does not.** It is the missing `DONE` verdict word for the `static`
class — `wiring-class-determination.md`'s table records the status column for `static` as
"(none — currently `ingested-magnitude`)". Until it exists, those units cannot be reported as
complete by any instrument, which is the dashboard defect the operator raised on 2026-08-12: a
finished `static` unit renders identically to unfinished `computed` work. It does **not** address
`derived`, whose bar is an evaluator-vs-fixture check over a further 1,000 `ingested-magnitude`
units and whose status column is equally "(none)" — a sibling gap, not this entry's scope, and
unrouted as of this writing.

**Not blocking anything.** No lane, epic, or reach claim in SD-29 waits on this. Deferring it costs
the bundle nothing; running it inside the bundle would cost the lanes cycles they need.

### C1.6 — `ASPECT:` is modelled by no chassis in this program: 34 grounded companion rows are diminished by it and 1 is emptied

**Owner: SD-31** (`docs/release/SD-31-pcgen-character-import/`), as the next bundle in the program;
re-assign here if a different successor is cut first. Same owner and same reasoning as C1.4 and C1.5.

**Status at handoff: measured, named, and blocking exactly one unit.** Recorded by SD-29 Epic 7
round 9, the companion lane's final pass (`decisions.md §69.5`). It is the reason the companion lane
is `DRY` at 922 of a 923 ceiling rather than at 923 of 923.

#### The number, and the command that produces it

```
python3 - <<'PY'   # every GROUNDED companion unit, re-read from its own corpus line
import json, sys, collections
sys.path.insert(0,'scripts')
from classify_companion_rows import read_row, resolve_source_file, book_dirs
inv=json.load(open('docs/work-inventory.json')); dirs=book_dirs()
n=0; tot=0; per=collections.Counter()
for u in inv['units']:
    if u['kind']!='companion' or u['status']!='grounded': continue
    d=dirs.get(u['book'])
    if not d: continue
    row=read_row(resolve_source_file(d,u['source_file']),u['source_line'])
    tot+=1
    if any(t.startswith('ASPECT:') for t in row): n+=1; per[u['book']]+=1
print('grounded companion rows read:',tot); print('of which carry ASPECT: ',n); print(dict(per))
PY

  grounded companion rows read: 922
  of which carry ASPECT:  34
  {'bestiary': 5, 'bestiary_4': 1, 'bestiary_5': 1, 'book_of_the_damned_volume_1': 1,
   'core_essentials': 14, 'core_rulebook': 2, 'ultimate_magic': 5, 'ultimate_wilderness': 5}
```

Re-derived on the round-9 tip. `decisions.md §63.3` measured 27 on the twelve-book tree; the figure
moves with each registered book, so a successor should re-run the command rather than cite either
number.

#### What the mechanism is

`ASPECT:` is a PCGen token that states what a record DOES in a form meant for display —
`ASPECT:ReachAttack|5 ft.`, `ASPECT:RacialSkillModifier|+4 Stealth (improves to +8 in forests)`. No
chassis in this program models it:

```
grep -rn -i aspect src/rules_core/rules_tables/companion_chassis.rs \
                   src/rules_core/rules_tables/monster_chassis.rs \
                   scripts/transcribe_monster_tables.py
  -> only prose about it being unmodelled; no field, no parse
```

#### Why it is a ceiling correction rather than a backlog line

**33 of the 34 rows are diminished by the omission; exactly one is emptied by it.** The 33 also carry
a `TYPE:` and usually a `DESC:`, so they reach a player with real content and merely lose one
statement. `core_essentials`' `Pseudodragon ~ Tail` (`ce_abilities_familiar_race_cr.lst:215`) carries
`KEY:`, `CATEGORY:Special Ability`, `SOURCEPAGE:p.229` and `ASPECT:ReachAttack|5 ft.` — no `TYPE:`,
no `DESC:`, no `BONUS:` — so transcribing it under today's chassis emits a card that reads "Tail"
over a page number.

That is the stub `docs/governance/no-stub-mvp-doctrine.md` forbids, and the transcriber's
empty-payload screen (`decisions.md §63.3`) correctly refuses it. **The reach gate would have counted
it as reached**, which is the twin problem `AGENTS.md` names: the screen and the gate would have
agreed while the player saw nothing. Screening at the generator is the fix at the source; modelling
`ASPECT:` is the fix at the cause.

#### What executing it costs

A field on `CompanionAbilityRecord` and on `MonsterAbilityRecord`, a parse in both transcribers, a
DTO field in `companion_catalog` and `monster_catalog`, a render in both screens, and a regeneration
of all sixteen registered companion books' and every registered monster book's generated tables —
with the count pins that move with them (`AGENTS.md`: a record-count change compiles clean and leaves
OTHER files' hardcoded assertions red).

**Not blocking anything in SD-29.** One companion unit stays honestly `not-ingested`; the lane's
`DRY` ruling accounts for it explicitly rather than rounding it away.

#### The classifier over-reports this row, and that is worth carrying forward

`scripts/classify_companion_rows.py` asks "is this ability row owned by a creature of its own book?"
and `Pseudodragon ~ Tail` is owned, so the classifier calls it reachable. The transcriber's
empty-payload screen is a *different* predicate and drops it. A successor reading `reachable
remainder − grounded` will therefore compute `1` workable companion unit and find zero. Same
proxy-defect class as `decisions.md §68`'s two: a screen making a confident claim about something
other than what it names.

## Review trigger

Reopen SD-29's forward-scope register when:

- A successor bundle reaches into SD-29's ingest outputs.
- A new bestiary arrives in the corpus.
- The bulk-modification retrofit is operator-authorized.
- The post-`tranche/9` consumer is operator-named.
- Operator requests Class 3.x retrofits.
- The retroactive magnitude-fidelity sweep is operator-authorized, or the `derived` class's
  evaluator-vs-fixture bar is routed to an owner (C3.3).
- SD-28's per-class archetype measurement (`§9.1`) reaches the classes behind SD-29's deferred
  `class_feature` units (C1.3).
- A cycle proposes to model PCGen's `ASPECT:` token on either chassis (C1.6) — the companion lane's
  last unit unblocks with it and 33 already-shipped rows get richer.

Closed-form: the bundle closes when Epic 11 (Closure Epilogue) fires.
