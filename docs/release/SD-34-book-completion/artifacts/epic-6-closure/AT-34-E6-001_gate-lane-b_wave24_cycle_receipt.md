# Cycle wave-24-gate-lane-b — Epic 6 (gate remediation) / AT-34-E6-001 (Lane B)

**Filename note (same convention wave-23's lane receipts and wave-24's own `_gate-lane-a_wave24_`
receipt set).** This cycle's dispatch again reused the tracking label `AT-34-E6-001` for a
gate-remediation lane, distinct from `kanban.md` row 26's canonical `AT-34-E6-001`
(`final-acceptance-scan`, still `not-started`, untouched by this cycle). Writing to the literal
path (`AT-34-E6-001_cycle_receipt.md`) would overwrite the real 2026-08-29 final-acceptance-scan
FAIL-verdict receipt; writing to `AT-34-E6-001_gate-lane-b_cycle_receipt.md` would overwrite
wave-23's own lane-B receipt (a real prior-cycle record: frontend-test/site-public-status-check
closed, site-dashboard-check left open — still the correct, valuable history of that different set
of stages). Filed here instead, wave-tagged, so all records stay on disk. `kanban.md` is untouched.

- **Commit SHAs:** `170c9219c4` (race_trait_picker + reach_gate: the 9-record cascade),
  `e36eacb224` (feat_catalog: the render-defect count re-derive) — both already pushed to
  `tranche/14` before this receipt; see `commit_sha` in the structured return for the final SHA
  including this receipt's own commit.
- **Files touched:** `apps/desktop/src-tauri/src/race_trait_picker.rs`,
  `apps/desktop/src-tauri/src/reach_gate.rs`, `apps/desktop/src-tauri/src/companion_catalog.rs`,
  `apps/desktop/src-tauri/src/feat_catalog.rs`. No `src/`, no `tests/`, no `data/corpus/**` —
  entirely inside this lane's `apps/desktop/` + `site/` territory.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` —
  `git diff --unified=0 aee9c78234...e36eacb224 -- apps/desktop/ site/ ':!**/__tests__/**'
  ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` → no hits.
- **Wired-integration audit result:** two hits of the word "placeholder", both self-healed inline
  rather than a real stub: `apps/desktop/src-tauri/src/reach_gate.rs` describes
  `Human Ethnicity ~ {None,Unknown}` as "placeholder rows" — the exact, precedented term
  `src/rules_core/race_resolver.rs`'s own comment already uses for these identical two records
  ("pure flavor placeholders for 'no ethnicity chosen'"). Real, shipped corpus content correctly
  described, not unfinished code. `OK_NO_TOKENS` otherwise.
- **Acceptance criterion (verbatim from this cycle's dispatch brief — a wave-24
  gate-remediation lane, not the canonical `epic-breakdown.md` AT-34-E6-001 final-acceptance
  scan):** "GATE LANE B — desktop, reach, and site-dashboard-check. Wave 23 took the gate from
  14 red stages to 5. Yours are `desktop`, `reach` and `site-dashboard-check`... The 7 left are
  named in lane A's receipt... Four of them (~30 records) are the SAME 9 already-diagnosed
  core_rulebook Adopted-Race / Human-Ethnicity records... The other three are `companion_catalog`
  (2, one apparently a large pre-existing reach gap spanning hundreds of ids) and `feat_catalog`
  (1, uninvestigated)."

## What this cycle found and fixed — all 7 named desktop-crate failures, closed

Starting population (re-derived live, not copied from the brief): `cd apps/desktop/src-tauri &&
cargo test --locked` → **565 passed / 7 failed**, matching the brief's own figure exactly.

### 1–4. race_trait_picker + 3× reach_gate — the 9-record cascade (closed, but the population
### assumption in the brief was itself wrong and needed re-deriving, not just applying)

The brief characterized the 7 new `core_rulebook` Adopted-Race selectors as resolving
identity-only (Rougarou's "proven empty" shape). **Re-derived and found wrong**: `grep -rl
'"Elf Race Trait"' data/corpus/*/trait_generic/*.json` (the brief's own suggested check) returns
nothing because of a JSON-formatting false negative — the real per-token grep
(`grep -rl 'RaceTrait.<Race> Race Trait'`, restricted to the 11 books `RACE_CORPUS_BOOKS` actually
loads) finds real, multi-member `<Race> Race Trait` pools for 6 of the 7: Dwarf 4, Elf 4, Gnome 4,
Half-Elf 4, Half-Orc 4, Human 4, Halfling 3 — real PF1e chargen Traits from
`advanced_players_guide`/`inner_sea_races`. Only Rougarou is genuinely empty.

`race_trait_picker.rs`'s test now pins the real 21-key menu, per-race grant counts, and the mixed
`ISR`/`APG` book attribution for the 7 CRB selectors' multi-member pools (previously every
non-Rougarou grant was assumed single-member and `ISR`-only, true only for the original 14).

`reach_gate.rs`: the only genuine gap is the 2 `Human Ethnicity ~ {None,Unknown}` placeholder
rows (`TYPE:HumanEthnicity`, no readable gate, `TraitRole::Unclassified` on the root side already
— `race_resolver.rs`'s own `unclassified_traits()` census pins them by name). Added a
`("crb", "race_traits")` `OPEN_FINDINGS` entry (remedy: a mechanically-inert `HumanEthnicity`
picker, out of this cycle's scope) and an `UNREACHED_RECORD_FINDINGS` entry naming the 2 keys
exactly — the same two-list pinning shape every other `race_traits` family in this file already
uses.

### 5. `companion_catalog::an_unmodelled_facet_reaches_the_wire_with_its_type_segments` — a real,
### large, but entirely legitimate re-derive (141→2193 wire rows / 39→93 records)

Traced to `AT-34-E3-001`'s 54 new `core_rulebook` companion-ability records (`6aad5b0f7a`, lane
A's own wave-23 work): the book-wide-granted generic Animal Companion progression table (31
`AnimalCompanionFeat` + 14 `AnimalTrick` + 6 `CompStatChoice` + 2 `CompChoice` + 1 `Special`).
Each names no single creature — upstream grants the whole table to every companion a
core_rulebook class can have — so `companion_chassis`'s ownership graph attaches each of the 54
records to every one of core_rulebook's own companion-eligible creatures, producing +2052 wire
rows from +54 records (≈38 average owners per record). `93` cross-checks exactly against
`companion_chassis.rs`'s own root-lib pin
(`an_ability_with_no_modelled_facet_still_states_its_type_segments`, `39 + 54 = 93`) — re-derived
independently on the wire side, not copied. 5 new unmodelled `TYPE:` shapes named and pinned
(none is a feat/quality/attack `CompanionAbilityFacet` models — each is a progression-table
CHOICE, the same reason every earlier round's unmodelled shapes ship unmapped).

### 6. `companion_catalog::every_registered_ability_reaches_the_wire_under_an_owner` — a REAL
### code fix, not a re-pin (the brief's "hundreds of ids" characterization was stale)

Instrumented before fixing: only **14** missing, not "hundreds" — the brief's characterization
was lane A's own wave-23-in-progress snapshot, already shrunk by lane A's subsequent fixes before
this lane started (**instrument-correction**, logged below). All 14 are `core_rulebook`'s generic
`Familiar ~ <X>` rows (Alertness, Deliver Touch Spells, Empathic Link, Improved Evasion, ...) —
PF1e's standard familiar-ability list, book-wide-granted to whichever familiar a Wizard/Sorcerer
picks.

**Root cause, verified by reading the code, not guessing:** each of these 14 records declares
ownership only via `cross_book_owners` (`&[("beastiary", "Bat"), ("beastiary", "Cat"), ...]` — 11
`beastiary` familiars), never via same-book `owners`. `CompanionBook::abilities_of`
(`src/rules_core/rules_tables/companion_chassis.rs`) resolves a creature's `ability_keys`
**only against its own book's `companion_abilities` table** — so `beastiary`'s Bat creature (which
does NOT list `"Familiar ~ Alertness"` in its own `ability_keys` at all) never picks it up, and
`core_rulebook`'s own creatures never claim it either (it is not in their `owners` list). The
field exists, is `pub`, and is read by `v06_work_inventory.rs` (accounting) and by
`companion_chassis.rs`'s own consistency test — but by **nothing on the actual wire path**. The
data-side ownership was already proven consistent (`the_chassis_link_resolves_in_both_directions_
for_every_book`); nothing on the *screen* side ever consumed it.

**Fix, entirely inside this lane's territory (`apps/desktop/`), no `src/` touch:** added
`cross_book_abilities_of(book_id, companion_key)` to `companion_catalog.rs` — scans every
registered `CompanionBook`'s `companion_abilities` for a `cross_book_owners` entry naming this
creature, returning `(owning_book, ability)` pairs. Wired into `map_companion`'s `abilities:`
field via `.chain(...)`, using the ability's OWN book (not the creature's) for its wire `key`,
matching the rule same-book abilities already follow.

### 7. `feat_catalog::feat_descriptions_are_rendered_and_otherwise_byte_identical` — the
### "uninvestigated" one, traced to an already-landed sibling-lane fix (201→187, -14)

`changed.len()` (records whose rendered description differs from raw) moved 201→187.
Root-caused to `AT-34-E3-003` bucket-U cycle 2 (`36db23a053`, 2026-08-28, **not this cycle's own
work** — landed by lane A's predecessor earlier in the bundle): `render_pcgen_desc` gained an
exemption for a bare `%` immediately preceded by a digit ("75% chance..."), mirroring
`leaked_pcgen_syntax`'s own pre-existing exemption. Before that fix, `render_pcgen_desc` silently
dropped the literal `%`, making `served != raw` for any feat whose corpus text carries a clean
`N%` and nothing else needing a rewrite — wrongly counting it as "rewritten". 14 such feats named
exactly (diffed against the test's own exact-name pin, not guessed): `Arcane Armor Training`,
`Empower Spell-Like Ability ~ Ability`/`~ Spell`, `Greater Mesmerizing Feint`, `Hover`,
`Lingering Spell-Like Ability`, `Messenger Of Fate`, `Mirror Kin`, `Phantom Fortification`,
`Protector of the People`, `Reject Poison`, `Seeping Darkness`, `Spirit Sense`,
`Tavern Regular`. `raw_leaks` (187) is unaffected — none of these 14 was ever a real leak.

## Figures + their re-derive commands

| Figure | Old | New | Command / denominator |
|---|---:|---:|---|
| desktop crate suite | 565 passed / 7 failed | **572 passed / 0 failed** | `cd apps/desktop/src-tauri && CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001 cargo test --locked` |
| `race_trait_picker` adopted-race menu | 14 keys, `["B2","B3","B5","B6"]` books | 21 keys, `["B2","B3","B5","B6","CRB"]` | same command, `race_trait_picker::tests::*` |
| Dwarf/Elf/Gnome/Half-Elf/Half-Orc/Human `<Race> Race Trait` pool size | assumed 0 (identity-only) | **4 each** | `grep -rl 'RaceTrait.<Race> Race Trait' data/corpus/{core_rulebook,beastiary,advanced_race_guide,advanced_players_guide,monster_codex,inner_sea_races,horror_adventures,bestiary_2,bestiary_5,bestiary_6,bestiary_3}/trait_generic/*.json \| wc -l` per race |
| Halfling `<Race> Race Trait` pool size | assumed 0 | **3** | same command, Halfling |
| `crb/race_traits` unreached count | 0 | 2 (`Human Ethnicity ~ None`/`~ Unknown`) | `reach_gate::tests::unreached_records_are_exactly_the_recorded_findings` panic output before fix |
| `companion_catalog` unmodelled-facet wire rows / distinct records | 141 / 39 | **2193 / 93** | `companion_catalog::tests::an_unmodelled_facet_reaches_the_wire_with_its_type_segments`, cross-checked against `companion_chassis.rs`'s own `an_ability_with_no_modelled_facet_still_states_its_type_segments` (93) |
| `companion_catalog` unowned-ability gap | brief's "hundreds of ids" | **14** (instrument-correction: stale snapshot) | `every_registered_ability_reaches_the_wire_under_an_owner`, temporary `expected.difference(&served)` instrumentation, removed before commit |
| `feat_catalog` `changed` (rewritten records) | 201 | **187** | `feat_catalog::tests::feat_descriptions_are_rendered_and_otherwise_byte_identical` |
| denominator gate on this package | — | `files_checked=16 violations=0` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` |
| corpus_literal_sweep baseline | 48708 (post wave-23 sweep) | **unmoved** — this cycle touched no `data/corpus/**` | N/A, no corpus write this cycle |

## Row-count command output (this cycle's own artifact — the 3 assigned stages)

```
$ cd apps/desktop/src-tauri && CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001 cargo test --locked 2>&1 | tail -1
test result: ok. 572 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 96.59s
```
- **desktop: CLOSED.** 572/0, verified live, re-run twice (once mid-cycle, once at final HEAD
  after the root-workspace `--no-run` build) with identical result.
- **reach: CLOSED.** Same binary, same run — `reach` and `desktop` are the same `cargo test`
  invocation over this crate; all `reach_gate::tests::*` are in the passing 572.
- **site-dashboard-check: NOT verified green this cycle, deliberately not attempted — see below.**

## site-dashboard-check — deliberately not attempted, root cause confirmed by read-only means

The brief's hazard note is explicit: *"do NOT run the inventory regenerator or the dashboard
producer from a lane — both silently drop stamps."* Reading `scripts/publish-site-dashboard.sh`
confirms `--check` mode never writes the committed file (it seeds a `mktemp -d` scratch copy and
diffs against that) — but it still **invokes the real producer** (`python3 "$PRODUCER" --out
"$TMP"`), which internally shells out to `cargo run --bin v06_work_inventory --summary`
(`scripts/observer/pf1e_dashboard_producer.py`). No `target/debug/v06_work_inventory` binary
exists at this worktree's default `target/` (`ls target/debug/v06_work_inventory` → no such
file), so a real attempt would trigger a fresh workspace-adjacent build inside a python subprocess
this lane cannot bound the same way its own `cargo test` invocations are — exactly the shape the
brief's second hazard note names: *"the producer's own `v06_work_inventory --summary` has been
timing out at its 600s cap under load; that is a contention symptom, not a bug to paper over."*
Per the brief's own instruction — *"If the feed can only be refreshed by running the producer, say
so in your receipt and leave it for the closing sweep"* — this cycle did not attempt it.

**Staleness confirmed by cheap, read-only means instead** (no subprocess, no cargo):

```
$ python3 -c "import json; print(json.load(open('site/dashboard/PF1e-dashboard.json'))['generated_at'])"
2026-08-24T22:17:30Z
$ git log -1 --format=%cd --date=iso -- docs/work-inventory.json
2026-08-31 20:15:47 -0400
```

The committed feed predates the last `docs/work-inventory.json` regeneration by **7 days** —
unchanged from wave-23 lane B's own finding (`AT-34-E6-001_gate-lane-b_cycle_receipt.md`), and
from the wave-23 gate-sweep's own live `verify.sh` run naming the same stage red for the same
reason. Nothing this lane could safely do moves this number without either the forbidden
regeneration or an unbounded, resource-risky build.

## Build scope verified

- `cd apps/desktop/src-tauri && CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001 cargo test --locked`
  (separate cargo workspace, explicit per `workflow-instruction.md §2.5`): **572 passed / 0
  failed**, re-run after the last commit that could move a figure (`e36eacb224`).
- Root workspace `cargo test --locked --no-run`, isolated `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001`,
  run at `e36eacb224` (this cycle touched zero root-workspace files, but this proves nothing
  cross-crate broke): **exit 0.**
- `cargo clippy` was NOT run — Lane C's territory, per this cycle's own dispatch brief.
- Full untargeted root-workspace `cargo test --locked --no-fail-fast` was **not run** this cycle —
  out of this lane's assigned population (`root-full` is Lane A's), and re-running the full
  ~600-suite root workspace on top of this cycle's own root-workspace `--no-run` build would be
  redundant work this lane's changes cannot affect (zero `src/`/`tests/` files touched).

## Sweep population

N/A — this cycle touched no `data/corpus/**` file (all 4 changed files are under
`apps/desktop/src-tauri/src/`). `corpus_literal_sweep`'s examined population is unmoved from the
wave-23 gate-sweep's own measured `48708`.

## Oracle pin

Not load-bearing for any figure in this receipt. The `<Race> Race Trait` pool-size figures came
from a live corpus grep, not the pinned PCGen oracle.

- **Status:** partial. 2 of this lane's 3 assigned stages (`desktop`, `reach`) reached the bar,
  verified live, 572/0. The third (`site-dashboard-check`) is not closed this cycle, deliberately,
  per the brief's own explicit instruction — named exactly below, a sequencing/authority question
  for the closing sweep, not an escalation.

## Movement, four buckets

- **Closure:** 0 inventory-bucket units moved — this cycle never touched `docs/work-inventory.json`
  or `data/corpus/**`. Movement is entirely test/gate re-derivation against already-landed engine
  and data changes, plus one real code fix (`cross_book_abilities_of`).
- **Reclassification:** N/A.
- **Reachability:** one real reachability improvement, inside `apps/desktop/`'s own wire path: the
  14 `core_rulebook` generic `Familiar ~ <X>` records (cross-book-owned by `beastiary`'s familiars)
  now actually reach a player's companion-catalog screen for the first time, via
  `cross_book_abilities_of`. Nothing in `docs/work-inventory.json`/`completion-atlas.json` moves
  from this — those track corpus-wide DONE-bucket status, not desktop-wire reachability, and this
  cycle did not touch either generated file.
- **Instrument-correction:** four, all retro-logged (`docs/retro/events/sd34-at-34-e6-001.jsonl`):
  (1) the brief's "identity-only, 7 empty pools" premise for the CRB Adopted-Race selectors was
  wrong — 6 of 7 have real, multi-member pools, re-derived from the live corpus rather than
  applied as given. (2) the brief's "hundreds of ids" characterization of the companion
  unowned-ability gap was a stale snapshot from mid-wave-23 — the live population, instrumented
  this cycle, was 14. (3) `companion_catalog`'s unmodelled-facet wire-row/record counts (141/39)
  needed re-deriving to 2193/93 against `AT-34-E3-001`'s already-landed 54-record cascade, cross-
  checked against `companion_chassis.rs`'s own independent pin rather than trusted alone. (4)
  `feat_catalog`'s `changed` count (201) needed re-deriving to 187 against `AT-34-E3-003` bucket-U
  cycle 2's already-landed `render_pcgen_desc` fix (`36db23a053`), with the exact 14 records named
  from the test's own exact-name pin rather than left as a bare count.

## Notes (judgment calls)

- **The dispatch brief itself carried two wrong premises, both corrected by re-deriving rather
  than trusting.** Per `decisions.md §12 L2` ("never carry your own number forward") and the
  standing lesson that a wave's own dispatch text can go stale between being written and being
  executed (concurrent lanes keep landing commits): the "7 empty Adopted-Race pools" and the
  "hundreds of ids" companion gap were both checked against the live repo and found smaller/
  different than stated, before writing any fix.
- **`companion_catalog::every_registered_ability_reaches_the_wire_under_an_owner` was fixed with
  real code, not a count re-pin**, because the underlying defect (a cross-book ownership
  declaration nothing on the wire path consumed) is a genuine player-facing gap: familiar
  abilities PF1e's own rules grant were never shown on any familiar's companion-catalog entry.
  The fix lives entirely inside this lane's own territory (`apps/desktop/src-tauri/src/
  companion_catalog.rs`) — no `src/rules_core/rules_tables/companion_chassis.rs` edit was needed,
  since `cross_book_owners` was already `pub` and already correctly populated on the data side.
- **`site-dashboard-check` was investigated only by read-only means** (a JSON field read and a
  `git log` date), explicitly avoiding both hazards this cycle's own brief named twice. This is a
  deliberate, bounded finding, not an incomplete investigation — the remedy (run the producer for
  real) is unambiguous and was already established by wave-23's own lane-B receipt; re-confirming
  the same staleness with a cheaper instrument adds evidence without adding risk.

## Next-cycle plan (named remainder, by sub-cause, population summing to what remains)

**site-dashboard-check — 1 stage, exactly one remedy, no further investigation needed:**

1. Run `./scripts/publish-site-dashboard.sh` for real (not `--check`), from a tree confirmed to be
   at or past this cycle's own HEAD, then re-run `scripts/verify.sh --only site-dashboard-check`
   to confirm PASS. This is explicitly the "closing sweep" step both this receipt's brief and
   wave-23's own lane-B receipt route it to — not a fresh diagnosis. Population: 1 stage, 2 files
   (`site/dashboard/PF1e-dashboard.json` + `site/dashboard/units/*.json`'s shard cache).
2. If the underlying `v06_work_inventory --summary` subprocess again exceeds its 600s cap even
   with this lane's cargo processes long finished, the brief's own diagnosis stands: this is a
   genuine single-threaded corpus-walk wall-clock cost at the corpus's current size, not a
   contention artifact alone, and the closing sweep should run it with headroom (no concurrent
   lane cargo activity) rather than raise the timeout to paper over it.

**clippy — untouched this cycle, Lane C's territory.**

**root-full — untouched this cycle, Lane A's territory (10 of 13 named tests remain per that
lane's own wave-24 receipt).**
