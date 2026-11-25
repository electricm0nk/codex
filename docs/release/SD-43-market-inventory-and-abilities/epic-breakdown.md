---
canonical: false
owner: operator
bundle_id: SD-43
slug: market-inventory-and-abilities
canonical_branch: tranche-43
target_version: 0.43.0
date: 2026-11-22
status: planning — front half; acceptance commands are authored, not yet run against a tree that has the change; the lead runs each on the pre-change tree before dispatch and requires a failure (TDD), and the card records that RED output
---

# SD-43 Epic Breakdown — Market, Inventory and the Abilities Tab

## §0 Parallel / serial map

This table leads the file because the Workflow script reads it. **Tier** is the `model` each `agent()` sets
(user's global tiering, 2026-09-29: Opus 5.5 default; Sonnet 5.5 for well-specified mechanical work; Haiku 4.5
for housekeeping). **Lane** names the worktree: `.worktrees/sd43-<lane>` with
`CARGO_TARGET_DIR=/home/ubuntu/cargo-target/sd43-<lane>` (never under `/tmp`; deleted at lane end). At most **3**
cargo cards in flight on the box (`decisions.md §29`). **Token `hub`**: a card marked `hub` writes one of the
three shared files — `apps/desktop/src-tauri/src/character_hub.rs`, `apps/desktop/src-tauri/src/main.rs`,
`apps/desktop/src/characterHub/CharacterSheet.tsx` — and at most one `hub` card runs at a time across all lanes
(the Workflow script holds the token; this is the control for the shared-tree collisions of tranche/7, not a
warning). A card that only *adds registration lines* to `main.rs` still holds `hub`.

> Paths are repo-root-relative unless a command `cd`s first. `ui/` = `apps/desktop/src/`, `desktop/` =
> `apps/desktop/src-tauri/src/`. TS tests are self-executing: `cd apps/desktop && npx tsx src/characterHub/gearScreenModel.test.ts` (one file per run); the
> full UI suite is `cd apps/desktop && npm test && npm run typecheck`. Cargo filters run in the workspace that
> holds the test: root (`src/`, `crates/`) or `apps/desktop/src-tauri`. A filter that matches 0 tests is a FAIL.
> In a table cell `\|` is the escape for `|`.

| Card | Objective (one line) | Lane | After | Tier | Cargo? | hub? |
|---|---|---|---|---|---|---|
| C0 | This package (front + back half) committed; placeholder gate run | lead | — | lead session | no | no |
| C1 | Interim `release-notes.md` + `current-release.json` → tranche 43, one commit; branch pushed | main | C0 | haiku | no | no |
| C2 | Incremental graphify after SD-42 (no `--force`, no `graphify update`); receipt | main | C1 | sonnet | no | no |
| E0.1 | Abilities census: per sub-tab records, saved field, budget, prerequisite check, pool sizes per level; PCGen sub-tab mapping; data-gap list | census | C1 | opus | no | no |
| E0.2 | Market count reconciliation test over `built_catalog()`: shown + dropped-with-reason = source, per list | market | C1 | opus | yes (desktop) | no |
| E0.3 | Placeholder-name scan widened: every kind, `label`/`OUTPUTNAME`, `_settled`, `rules_tables`, bundle copy | census | C1 | sonnet | no | no |
| E0.4 | Reproduce item 5's three symptoms, item 7's drops and item 12's container sale on a fresh Brother Tomas Vane | gear | C1 | opus | yes (app build) | no |
| E0.5 | Rest-rule census (classes, races) and House-Rules-per-system census (PF, SF, CPR) | census | C1 | sonnet | no | no |
| E1.1 | Container truth: no `#N`, custom names on both panels, a container with items never stacked, put-into-stack splits | gear | E0.4 | opus | yes (desktop) | hub |
| E1.2 | One-column gear list: order, natural sort, indent, nested boxes, buttons on the line, Not Carried section | gear | E1.1 | opus | no | no |
| E1.3 | The whole container box is a drop target (nearest container) | gear | E1.2 | sonnet | no | no |
| E1.4 | Double-click / Enter: to a slot (first free, else swap) or to Carried; one piece | gear | E1.3 | opus | no | no |
| E1.5 | Selling a container moves its direct contents to Carried (backend, both screens) | gear | E1.4 | opus | yes (desktop) | hub |
| E1.6 | Sell window: Market Rate / Build / Negotiate; "the pricing above" gone | gear | E1.5 | opus | no | no |
| E1.7 | One `MarketButton` top-left, 2×, accent, on every panel (PF, SF, CPR); Print and pricing control leave the panels | sheet | E1.6 | sonnet | no | hub |
| E1.8 | Market inventory sell value column from `sellNegotiateGp` | market | E0.2 | sonnet | no | no |
| E1.9 | Item stats in the Market description box, engine-stated, shared renderer | market | E1.8 | opus | yes (desktop) | hub |
| E1.10 | Filter model: per-list columns and values, `Other`, counts account for every row | market | E1.9 | opus | no | no |
| E1.11 | Filter column in the dialog; no 200 cap; windowed list; first-paint measured | market | E1.10 | opus | no | no |
| E1.12 | Sell removes the copy the player clicked (copy key, not item id + state); discovered by E0.4 | gear | E1.5 | opus | yes (desktop) | hub |
| E1.13 | Gear sell window's choice holds under a Cashless session (Q-9.1); discovered by E1.6 | gear | E1.6 | sonnet | no | hub |
| E1.14 | Market inventory sell value / Negotiate prefill resolve every held copy, not only opened lists; discovered by E1.MC | market | E1.8 | sonnet | no | no |
| E1.15 | Gear Sell opens its window on the first click after launch; discovered by E1.MC | gear | E1.6 | opus | no | no |
| E1.MC | E1 merge check + epic report (adversarial; screenshots of gear screen, Market, sell window) | gear | E1.7, E1.11 | opus | yes (base build) | no |
| E2.1 | Fixed layout on the PF sheet: bar, rail and right column fixed, centre scrolls, `100dvh`; Show Progression visible (17 + 21) | sheet | E1.7 | opus | no | hub |
| E2.2 | The same shared layout on the SF and CPR sheets | sheet | E2.1 | sonnet | no | no |
| E2.3 | Open campaign button opens the folder; one full-width banner per campaign; Manager banner smaller | sheet | E2.2 | opus | no | hub |
| E2.4 | Rest-and-recovery: cited rules-text table, engine command, print block under the bio page, override field | sheet | E0.5, E2.3 | opus | yes (root + desktop) | hub |
| E2.5 | Rest text read-only under the Biography text; never saved into `bio.biography` | sheet | E2.4 | sonnet | no | hub |
| E2.6 | Rest-and-recovery override editor in the Custom dialog (`custom.json` `restText`); discovered by E2.MC | sheet | E2.5 | sonnet | no | hub |
| E2.MC | E2 merge check + epic report (screenshots at a short window, all three sheets) | sheet | E2.5 | opus | yes (base build) | no |
| E3.1 | Abilities tab shell: `Feats` → `Abilities`, seven sub-tabs, today's Feats/Traits/racial/pool surfaces moved, no behaviour change | sheet | E0.1, E2.5 | opus | no | hub |
| E3.2 | Engine: generic ability-budget module (`abilities.budget.<subtab>.<pool>.*` rows + `refusal`), saved selections, list/add/remove commands; proved on Traits | sheet | E3.1 | opus | yes (root + desktop) | hub |
| E3.3 | Generic sub-tab UI (Available / Selected / Info, search, Qualified filter, remaining per pool, Add/Remove with reason); Traits sub-tab on it | sheet | E3.2 | opus | no | hub |
| E3.4 | Class Features: every catalog pool's grants per level from the class records; budget rows | sheet | E3.3 | opus | yes (root + desktop) | hub |
| E3.5 | Class Features: pool-member prerequisite evaluator (`PRE*` tokens) and refusal reasons; sub-tab wired | sheet | E3.4 | opus | yes (root + desktop) | hub |
| E3.6 | Racial Abilities: alternate racial traits selectable on the sheet; replaces-conflict refused | sheet | E3.5 | opus | yes (root + desktop) | hub |
| E3.7 | Archetype: select per class via `choice:archetype`; slot conflicts refused; archetype grants feed pools | sheet | E3.6 | opus | yes (root + desktop) | hub |
| E3.8 | Favored class bonus: class pick, per-level HP / skill rank / race alternates; totals in the engine | sheet | E3.7 | opus | yes (root + desktop) | hub |
| E3.9 | Background and Misc sub-tabs per E0.1's mapping; engine budget/prereq where records exist; data-gap notice where none | sheet | E3.8 | opus | yes (root + desktop) | hub |
| E3.10 | Overrides on the six new sub-tabs: two flags, admitted flags shown | sheet | E3.9 | sonnet | yes (desktop) | hub |
| E3.11 | Printout: chosen abilities per sub-tab and the favored class bonus, as text | sheet | E3.10 | opus | yes (desktop) | no |
| E3.12 | DISCOVERED (E0.1): ingest `.MOD TYPE:` member typing for the 30 ingest-gap pools; `Ninja ~ Ninja Trick` grant row; 7 unresolved pool names | sheet | E0.1 | opus | yes (root) | no |
| E3.13 | DISCOVERED (E3.4): level-1 `ABILITYSELECTION` picks (5 bloodline pools: an `offers` count with no `Pool` target) get grant rows and picks on Class Features | sheet | E3.4 | opus | yes (root + desktop) | hub |
| E3.14 | DISCOVERED (E3.4): the 31 class-feature pools whose converted grants carry no `offers` (oracle mystery revelations, kineticist elements, shaman hex, …) list their members | sheet | E3.4 | opus | yes (root + desktop) | no |
| E3.15 | DISCOVERED (E3.5): saved Class Features picks seeded into the sheet's held set (`HeldSeed::from_record`) so a pick's rules reach the sheet and the print | sheet | E3.5 | opus | yes (root + desktop) | no |
| E3.16 | DISCOVERED (E3.7): a held archetype's record seeded into the sheet's held set (`HeldSeed::from_record`) so a replaced feature's sheet lines and print follow | sheet | E3.7 | opus | yes (root + desktop) | no |
| E3.17 | DISCOVERED (E3.9): Background and Misc picks seeded into the sheet's held set (`HeldSeed::from_record`) so a pick's own rules reach the sheet and print | sheet | E3.9 | opus | yes (root + desktop) | no |
| E3.18 | DISCOVERED (E3.12): the E3.12 pools whose converted grants carry no offer selecting a member list members on the sheet (engine test per pool) | sheet | E3.12 | opus | yes (root + desktop) | no |
| E3.19 | DISCOVERED (E3.12): `gen_class_feature_grants` reads `*_abilities_globalvar.lst` (UC Ninja 64 / Samurai 99 class grant rows) and re-runs without drift | sheet | E3.12 | opus | yes (root) | no |
| E3.20 | DISCOVERED (E3.13): the `Bloodline` pool (Standard Bloodline) is reconciled with the held level-1 bloodline, and Standard Bloodline's qualification agrees with its gate | sheet | E3.13 | opus | yes (root + desktop) | hub |
| E3.21 | DISCOVERED (E3.18): the Feats-sub-tab pools whose member filter names only `ABILITYLIST:` rules take only those feats (`feat_budget` reads the filter's `rules`) | sheet | E3.18 | opus | yes (root + desktop) | no |
| E3.22 | DISCOVERED (E3.19): E3.12's `data.member_types` on 28 corpus records carries live PI blacklist terms; `enrich_member_typing` screens the field, declared-pi-audit passes | sheet | E3.12 | opus | yes (root) | no |
| E3.23 | DISCOVERED (E3.21): a gate naming its own record's KEY is converted to `MissingRule` (never met); the converter resolves it, so Blue ~ Psionic Prodigy grants `unlocked_talent_feat` | sheet | E3.21 | opus | yes (root + desktop) | no |
| E3.MC | E3 merge check + epic report: every sub-tab on three seeds; notice count = E0.1's data-gap list | sheet | E3.11 | opus | yes (base build) | no |
| E4.1 | One display-name derivation for every item kind; the all-kinds test; proposed names list from book text | sheet | E0.3, E3.MC | opus | yes (root + desktop) | no |
| E4.G | **Operator wait:** the operator approves the proposed names list (and confirms the reading of "1") | lead | E4.1 | operator | no | no |
| E4.2 | Ship the approved names; Market and Inventory show them; epic report | sheet | E4.G | sonnet | yes (root + desktop) | no |
| E5.1 | Report API client in Rust: key stamped at build, byte limit, every reply handled, "sent for review" | settings | C1 | opus | yes (desktop) | hub |
| E5.2 | Anonymous selector + optional Handle on the bug and enhancement screens; GitHub-unavailable message | settings | E5.1 | opus | no | no |
| E5.3 | Update menu: channel selector beside `Channel:`; text blocks widened; word-boundary wrap | settings | E5.2 | sonnet | no | no |
| E5.4 | Developer tab slimmed (ruling 16) + Corpus Ingest Diagnostic embedded, landing entry and mode removed (19) | settings | E5.3 | sonnet | no | no |
| E5.MC | E5 merge check + epic report | settings | E5.4 | opus | yes (base build) | no |
| E6.1 | Campaign folder relocation: copy, verify, switch, skeleton; refuse a foreign non-empty target | campaign | C1 | opus | yes (desktop) | hub |
| E6.2 | House Rules ☰ menu per character and system over `custom.json` `Overrides`; four existing flags moved there | sheet | E3.MC | opus | yes (desktop) | hub |
| E6.3 | House rule: HP at level-up, seven methods, stored once | sheet | E6.2 | opus | yes (root + desktop) | hub |
| E6.4 | House rule: Ambidextrous removes the off-hand penalty in the engine | sheet | E6.3 | opus | yes (root + desktop) | hub |
| E6.5 | Custom items export/import: local file or `Custom Items/` in the campaign folder; same name edits | campaign | E6.1 | opus | yes (desktop) | hub |
| E6.MC | E6 merge check + epic report | campaign | E6.4, E6.5 | opus | yes (base build) | no |
| E7.1 | ui-smoke rows for every changed screen (the one `spec.json` writer); inventory regenerated | main | all E*.MC, E4.2 | sonnet | no | no |
| E7.2 | Final-acceptance scan → retrospective written and cited → full worktree/branch sweep | main | E7.1 | opus | no | no |
| E7.3 | Release notes (final) + `current-release.json` check → architecture truth-up | main | E7.2 | sonnet | no | no |
| E7.4 | PR `tranche-43 → develop` LAST (graphify out of scope) → merge when green → watch the develop publish | main | E7.3 | sonnet | no | no |

Card count: `awk -F'|' '$2 ~ /^ (C|E)[0-9]/' docs/release/SD-43-market-inventory-and-abilities/epic-breakdown.md | awk 'END{print NR}'`
→ 73 (this table only; 56 at authoring + E3.12 discovered by E0.1 + E1.12 discovered by E0.4 + E1.13 discovered by E1.6 + E1.14 and E1.15 discovered by E1.MC + E2.6 discovered by E2.MC + E3.13 and E3.14 discovered by E3.4 + E3.15 discovered by E3.5 + E3.16 discovered by E3.7 + E3.17 discovered by E3.9 + E3.18 and E3.19 discovered by E3.12 + E3.20 discovered by E3.13 + E3.21 discovered by E3.18 + E3.22 discovered by E3.19 + E3.23 discovered by E3.21; the per-card headings below use `###`).

**Lanes and their files.** `gear`: `ui/characterHub/GearScreen.tsx`, `gearScreenModel.ts`, `GearDetail.tsx`,
the sell/move paths of `desktop/character_hub.rs` (under `hub`). `market`: `ui/characterHub/MarketDialog.tsx`,
`itemPickerFilter.ts`, `marketFilters.ts` (new), `ui/boundary/listMarket.ts`, `desktop/equipment_catalog.rs`.
`sheet`: `CharacterSheet.tsx`, the SF/CPR sheets, `desktop/print/**`, new rules_core abilities modules,
`desktop/character_custom.rs`. `settings`: `ui/feedback/**`, `ui/testerWorkbench/feedback/**`, `ui/update/**`,
`ui/App.tsx`, `ui/settings/SettingsModal.tsx`, `ui/characterHub/LandingScreen.tsx`, `CharacterHubPage.tsx`,
`CorpusIngestDiagnosticPanel.tsx`, `.github/workflows/publish-tester-release.yml` (E5.1's secret line only).
`campaign`: `ui/campaign/**`, `desktop/campaign_folder.rs`. `census`: read-only plus its own
`artifacts/epic_0/` outputs. Each lane is serial inside. Lanes run in parallel when their next cards hold no
`hub` conflict. `CharacterHubPage.tsx` is touched by E2.3 (sheet) and E5.4 (settings): E5.4 runs after E2.3
(the script orders them; listed here so the conflict is visible).

## Card format

Each card: **Objective**; **FILES YOU OWN** (closed under the change: command registration in `main.rs`, the
DTO producer, the boundary file, second call sites, `tauri.conf.json` `bundle.resources` +
`tests/packaged_resources.rs` for any new runtime data read; a file outside the list is a fence exit, logged as a
retro `incident`); **Failing test first** (written and run RED before production code; the receipt quotes the
RED line); **Acceptance** (commands; each names a test the card creates or a count that changes only after the
change); **Does not cover** (rule 7). Every code-bearing card also runs the four-check audit of
`docs/governance/no-stub-mvp-doctrine.md` §"Per-cycle audit" and pastes its output in the receipt.
`kanban.md` and `progress.md` rows are owned by every card.

## Epic C — package, pointer, graph

### C0 — this package
- **Objective:** front half (this file and five siblings) and back half committed on `tranche-43`; the
  placeholder gate run over all `*.md`.
- **FILES YOU OWN:** `docs/release/SD-43-market-inventory-and-abilities/**`.
- **Acceptance:** `grep -rn '<[a-z_-]*>' docs/release/SD-43-market-inventory-and-abilities/*.md` → every hit is
  in README §8's list; `ls docs/release/SD-43-market-inventory-and-abilities` lists the 13 chassis files plus
  `technical-requirements.md` and `technical-design.md`.
- **Tier:** lead session.

### C1 — release pointer
- **Objective:** `release-notes.md` (interim, sentinel `SD-43 INTERIM NOTES`) and
  `docs/release/current-release.json` = `{"tranche": 43, "notes_path":
  "docs/release/SD-43-market-inventory-and-abilities/release-notes.md"}` in one commit; push `tranche-43`.
- **FILES YOU OWN:** `docs/release/current-release.json`; this package's `release-notes.md`.
- **Failing test first:** `python3 tools/release/resolve_release_notes.py --repo-root .` exits 2 today (recorded
  2026-11-22).
- **Acceptance:** that command exits 0 and prints this package's notes path;
  `python3 -m pytest tools/release -q` green; `git ls-remote --heads origin tranche-43` prints one line.
- **Depends:** C0. **Tier:** haiku.
- **Does not cover:** the final notes (E7.3).

### C2 — graphify after SD-42 (intake item 0 step 4)
- **OUT OF SCOPE** (operator ruling 2026-11-22: "the graph refresh is out of scope"). Not run again; the run-2 attempt and its restore are in the receipt. The text below is the original card.
- **Objective:** run graphify incrementally against the current tree, as the operator named (no `--force`, no
  `graphify update`); a node-count guard exit 1 means file the receipt and stop (memory).
- **FILES YOU OWN:** `graphify-out/**` (as the tool writes it); `artifacts/cycle_0/C2-graphify-receipt.md`.
- **Acceptance:** the receipt holds the command, exit code and node counts before/after (`python3 -c "import
  json;print(len(json.load(open('graphify-out/graph.json'))['nodes']))"`, before and after; the after count is not
  lower than 90 % of the before count, else stop and restore from the dated snapshot).
- **Depends:** C1. **Tier:** sonnet.
- **Does not cover:** the closure graphify (E7.4).

## Epic E0 — census and engine-gap measurement (answers every "not yet checked" with a command)

### E0.1 — Abilities census
- **Objective:** for each of the seven sub-tabs and for every Class Features pool, write
  `artifacts/epic_0/abilities-census.json` and `.md` with: records in the corpus (count, by book); where a
  selection is saved today (field on `ChosenCharacterState`, `src/rules_core/character_input.rs:17`, or none);
  whether the engine has a budget (`grep` for budget rows) and a prerequisite evaluator; how pool size per level is
  stated in the class records (the tokens, with three worked classes: Rogue, Barbarian, Ninja); the PCGen
  sub-tab → category mapping read from PCGen's UI strings and `ABILITYCATEGORY` lines at the pin; the count of
  visible, player-editable categories (`VISIBLE:QUALIFY|YES` and `EDITABLE:YES`), not the raw 2,616 lines; and
  the **data-gap list** (sub-tabs/pools with zero selectable records).
- **FILES YOU OWN:** `artifacts/epic_0/abilities_census.py` (new, read-only over the corpus and the oracle),
  its two outputs.
- **Failing test first:** n/a (read-only census); the script exits non-zero when an input is missing.
- **Acceptance:** `python3 -I artifacts/epic_0/abilities_census.py --check` exits 0 and its JSON has a non-null
  entry for all seven sub-tabs; the Class Features section reproduces `python3
  scripts/census_class_feature_pool_population.py`'s 1,913 groups / 16,348 records; the oracle is resolved via
  `scripts/fetch-pcgen-oracle.sh --print-env` and its HEAD equals `PCGEN_ORACLE_SHA`.
- **Depends:** C1. **Tier:** opus.
- **Does not cover:** Starfinder and CPR data (out of scope, `scope-draft.md §5`); whether a member's rule text
  is correct.

### E0.2 — Market count reconciliation
- **Objective:** a committed Rust test that walks `built_catalog()` (`desktop/equipment_catalog.rs:834`) and
  prints, per Market list, the count shown and every dropped row's reason (duplicate key, template, `.FORGET`,
  `Codex-Named Unit`, `Equipmods`, no tags, book not ingested); writes `artifacts/epic_0/market-counts.json`.
- **FILES YOU OWN:** `apps/desktop/src-tauri/src/equipment_catalog.rs` (a `#[cfg(test)]` module, or
  `apps/desktop/src-tauri/tests/market_counts.rs` new); the artifact.
- **Failing test first:** the test asserts shown + dropped = source for each list and that no row is dropped
  without a reason; run before any reason is named, it fails on the unreasoned rows.
- **Acceptance:** `cd apps/desktop/src-tauri && cargo test --test market_counts -- --nocapture` green; its output
  prints Magic 3,680, Gear 1,624, Weapons 480 (the operator's screen figures) and exact Armor and Shields values,
  and the sum equals `built_catalog().entries.len()`. If the three figures do not reproduce, the card stops and
  reports — the test is wrong, not the screen.
- **Depends:** C1. **Tier:** opus.
- **Does not cover:** whether a dropped row *should* be on a list (a reason is a fact, not a ruling; a reason
  that looks wrong becomes a discovered card).

### E0.3 — placeholder names, widened
- **Objective:** scan every record kind's `data.name`, `label` and `OUTPUTNAME`-derived fields, plus `_settled`,
  `rules_tables` and `apps/desktop/src-tauri/resources/corpus_bundle/` copies, for `[`, `]`, `%`, `NAME` and
  other placeholder shapes (`{…}`, `%N`); write `artifacts/epic_0/placeholder-names.json` (record, field, raw
  name, key, book).
- **FILES YOU OWN:** `artifacts/epic_0/placeholder_names.py` (new) and its output.
- **Acceptance:** the script's equipment `data.name` count reproduces 111 of 3,805 and its all-kinds count
  reproduces 201 of 51,474 (`decisions.md §22`); every other surface's count is printed, zero or not.
- **Depends:** C1. **Tier:** sonnet.
- **Does not cover:** names that are wrong without a placeholder shape.

### E0.4 — reproduce the gear defects
- **Objective:** on a fresh Brother Tomas Vane seed in an isolated app-data root, reproduce item 5's symptoms 1–3
  (and the renamed copy showing `Sack #2`), item 7's three drop places, and today's container sale; record per
  symptom the steps, the stored `equipment_sets.json` rows, and display-bug vs data-bug.
- **FILES YOU OWN:** `artifacts/epic_0/gear-repro.md`, screenshots under `artifacts/epic_0/gear-repro/`.
- **Acceptance:** each symptom has a classification and the stored rows quoted; a symptom that does not reproduce
  says so with the steps tried.
- **Depends:** C1. **Tier:** opus.
- **Does not cover:** the operator's own save (not on this machine).

### E0.5 — rest rules and house rules per system
- **Objective:** list every class's and race's rest/recovery rule that the books state (spell preparation,
  resource refresh, natural healing, rest-related traits) with the record or book page it comes from; list
  which of the six House Rules have a counterpart in the SF engine and which apply to CPR (expected none).
- **FILES YOU OWN:** `artifacts/epic_0/rest-and-house-rules.md`.
- **Acceptance:** every PF base class in the class roster has a row or an explicit "no rest rule stated"
  (`ls data/corpus/*/class/*.json | awk 'END{print NR}'` is the denominator, printed); every row names its
  source.
- **Depends:** C1. **Tier:** sonnet.
- **Does not cover:** writing the text (E2.4).

## Epic E1 — Market and inventory (items 5–12)

### E1.1 — container truth (item 5 data and naming)
- **Objective:** remove `#N` (`gearScreenModel.ts:324–329` `copyTitle`); a renamed copy's `CopyFacts.name` shows
  on both panels; a container holding items is never in a stack; the move target and the table bind to the copy
  actually holding items, not `copies[0]` (`:312`, `:342–343`); putting an item into a stacked empty container
  splits that copy off first (backend if E0.4 classed it a data bug).
- **FILES YOU OWN:** `ui/characterHub/gearScreenModel.ts`, `gearScreenModel.test.ts`, `GearScreen.tsx` (labels
  only); `desktop/character_hub.rs` move/split path if E0.4 says data bug, with its test (`hub`).
- **Failing test first:** `gearScreenModel.test.ts` cases from E0.4: one Sack with 2 items + three empty Sacks →
  labels `Sack` and `Sack x3`, no `#`; renamed copy → its custom name; item into the stack → a new single row.
- **Acceptance:** `cd apps/desktop && npx tsx src/characterHub/gearScreenModel.test.ts`; `grep -n "#\${"
  apps/desktop/src/characterHub/gearScreenModel.ts | awk 'END{print NR}'` → 0; if backend: `cd
  apps/desktop/src-tauri && cargo test split_into_stacked_container` (≥1 test).
- **Depends:** E0.4. **Tier:** opus.
- **Does not cover:** the layout (E1.2).

### E1.2 — one-column list (item 5 redesign)
- **Objective:** one column under the equipment set; buttons on the item's line; non-containers A–Z then
  containers A–Z, recursively, natural sort; indent; nested light boxes (today's styling re-derived from
  `GearScreen.tsx` first); Not Carried as its own section, excluded from carried weight.
- **FILES YOU OWN:** `ui/characterHub/GearScreen.tsx`, `gearScreenModel.ts` (order function),
  `gearScreenModel.test.ts`.
- **Failing test first:** an order test: `Sack #10`, `Sack #2`, `Belt Pouch`, `Backpack` (holding loose A–Z and a
  container) → expected order at every depth.
- **Acceptance:** that test; a test that the screen's carried weight equals `totalCarriedWeightLbs` with a Not
  Carried item present; `npm test && npm run typecheck`; one screenshot per section shape.
- **Depends:** E1.1. **Tier:** opus.
- **Does not cover:** drop targets (E1.3).

### E1.3 — whole-box drop target (item 7)
- **Objective:** header, empty space and every child row of a container box target that container (nearest
  when nested); same-container drop is a no-op; self/descendant drops refused.
- **FILES YOU OWN:** `ui/characterHub/GearScreen.tsx`, `gearScreenModel.ts` (`dropTarget` `:363`),
  `gearScreenModel.test.ts`.
- **Failing test first:** a model test that a drop on a child row's zone resolves to its container (fails today:
  child rows carry no zone).
- **Acceptance:** the test; a refusal test for container-into-own-descendant over the widened area.
- **Depends:** E1.2. **Tier:** sonnet.
- **Does not cover:** drag between screens.

### E1.4 — double-click and Enter (item 6)
- **Objective:** per `decisions.md §6`; reuses the Move-to write.
- **FILES YOU OWN:** `ui/characterHub/gearScreenModel.ts` (a pure `doubleClickTarget` function),
  `gearScreenModel.test.ts`, `GearScreen.tsx`.
- **Failing test first:** `doubleClickTarget` cases: ring with one ring slot free; both full → swap first; worn →
  Carried; stack of 3 → moves 1; slot-less → null with reason.
- **Acceptance:** the test; `grep -c "onDoubleClick" apps/desktop/src/characterHub/GearScreen.tsx` ≥ 1; a test
  that a double-click inside a row button does not fire; Enter on a focused row calls the same function.
- **Depends:** E1.3. **Tier:** opus.
- **Does not cover:** drag.

### E1.5 — selling a container (item 12 ask 2)
- **Objective:** `sell_held_pieces_at_root` (`desktop/character_hub.rs:5635`) moves a sold container's direct
  contents to Carried; nested containers keep theirs; Not Carried contents stay Not Carried.
- **FILES YOU OWN:** `desktop/character_hub.rs` (sell path and its tests) (`hub`).
- **Failing test first:** Backpack ⊃ {Sack ⊃ Bedroll, loose items}; sell Backpack → expected locations and money.
- **Acceptance:** `cd apps/desktop/src-tauri && cargo test sell_container` (≥ 6 tests: empty, loose, nested, in a
  Bag of Holding, worn belt pouch + encumbrance, Not Carried contents).
- **Depends:** E1.4. **Tier:** opus.
- **Does not cover:** Drop and Discard (Q-12.1).

### E1.6 — sell window (item 9)
- **Objective:** Sell opens `SellDialog` (new): Market Rate (default) / Build / Negotiate, default amount from
  `sellNegotiateGp` (`ui/characterHub/marketModel.ts:115`), How many?, Sell/Cancel, Escape/Enter, focus return;
  the existing negative/non-number check kept; "the pricing above" removed (`GearScreen.tsx:31`, `:188`, `:205`,
  `:572`).
- **FILES YOU OWN:** `ui/characterHub/SellDialog.tsx` + `sellDialogModel.ts` + test (new), `GearScreen.tsx`
  (`PendingForm` sell branch).
- **Failing test first:** `sellDialogModel.test.ts`: option list is exactly `Market Rate`, `Build`, `Negotiate`;
  default `Market Rate`; amount for a stack of 3 scales; null price → blank.
- **Acceptance:** the test; `grep -rn "pricing above\|Normal" apps/desktop/src/characterHub/GearScreen.tsx
  apps/desktop/src/characterHub/SellDialog.tsx | awk 'END{print NR}'` → 0; `git diff --stat
  apps/desktop/src/characterHub/priceMode.ts` empty (PRICE_MODE_OPTIONS untouched).
- **Depends:** E1.5. **Tier:** opus.
- **Does not cover:** the Market window's own Sell (unchanged).

### E1.7 — the Market button (item 8)
- **Objective:** `MarketButton` (new) top-left on Weapons, Defense, Gear (PF) and on the SF and CPR sheets; 2×
  today's size, accent colour; panel Print removed (`CharacterSheet.tsx:854`); `PriceModeControl` removed from
  Gear (`:1615`) and Weapons (`:849`, DEFAULT); the ☰ Print (`:4153`) still opens `PrintDialog`.
- **FILES YOU OWN:** `ui/characterHub/MarketButton.tsx` + test (new), `CharacterSheet.tsx` (`hub`),
  `StarfinderCharacterSheet.tsx`, `CyberpunkCharacterSheet.tsx`.
- **Failing test first:** a test that the three PF panels render one `MarketButton` each and no `PriceModeControl`.
- **Acceptance:** the test; `grep -n "props.onPrint\|PriceModeControl" apps/desktop/src/characterHub/CharacterSheet.tsx
  | awk 'END{print NR}'` → 1 (the import line only, if kept; else 0, stated in the receipt);
  `grep -c PriceModeControl apps/desktop/src/characterHub/MarketDialog.tsx` ≥ 1 (kept there); the receipt states
  old and new height and font size.
- **Depends:** E1.6 (serial on the sheet lane). **Tier:** sonnet.
- **Does not cover:** SF/CPR Print placement beyond Q-8.2.

### E1.8 — sell values in the Market inventory (item 12 ask 1)
- **Objective:** a value cell between the name/quantity and Negotiate (`MarketDialog.tsx` ~316–336), from
  `sellNegotiateGp` under the Market's live price option, `formatMarketPrice`; `—` for no price and Cashless.
- **FILES YOU OWN:** `ui/characterHub/MarketDialog.tsx`, `MarketDialog.test.ts`.
- **Failing test first:** inventory row of a 3-stack under `standard` shows the 50 % total; under `cashless` `—`.
- **Acceptance:** the test; `grep -c "sellNegotiateGp" apps/desktop/src/characterHub/MarketDialog.tsx` ≥ 2 (the
  existing Negotiate use and the new cell — one function, two uses).
- **Depends:** E0.2 (lane order). **Tier:** sonnet.

### E1.9 — stats in the Market (item 10)
- **Objective:** per `decisions.md §13`.
- **FILES YOU OWN:** `desktop/equipment_catalog.rs` or `desktop/character_hub.rs` (whichever holds the shared
  describe function, `hub`), `desktop/main.rs` (registration if a new command, `hub`), `ui/boundary/`
  `describeMarketEntry.ts` (new if a new command), `ui/characterHub/MarketDialog.tsx`, `GearDetail.tsx` (shared
  renderer), tests on both sides.
- **Failing test first:** bastard sword shows damage and critical; a full plate shows armor bonus, max Dex, check
  penalty, spell failure; a missing stat renders `—`.
- **Acceptance:** those tests; a sampling test over every category and `TYPE` shape that prints, per stat, the
  count of `—` (the receipt pastes the table).
- **Depends:** E1.8. **Tier:** opus.
- **Does not cover:** the printed sheet.

### E1.10 — filter model (item 11)
- **Objective:** `marketFilters.ts` (new): per list, columns and values, `Other`, a two-tag rule; values confirmed
  against E0.2's tag counts.
- **FILES YOU OWN:** `ui/characterHub/marketFilters.ts`, `marketFilters.test.ts` (new); `equipmentCategories.ts`
  (reuse only).
- **Failing test first:** for each list and column, sum of value counts + `Other` covers every row of E0.2's
  per-list rows; a piercing-and-slashing weapon appears under both.
- **Acceptance:** the test.
- **Depends:** E1.9 (lane order), E0.2 (data). **Tier:** opus.

### E1.11 — filter column, no cap, windowed list (item 11)
- **Objective:** the dialog's filter column; `MARKET_ROW_LIMIT` (`itemPickerFilter.ts:362`) and the notice
  (`MarketDialog.tsx:248`) removed; a windowed list; header "N of N".
- **FILES YOU OWN:** `ui/characterHub/MarketDialog.tsx`, `itemPickerFilter.ts`, their tests.
- **Failing test first:** `marketShownRows` returns every row (fails today at 200).
- **Acceptance:** `grep -rn "MARKET_ROW_LIMIT" apps/desktop/src | awk 'END{print NR}'` → 0; keyboard selection,
  description, sort, Only Show Affordable and `+N` tests green; time to first paint on Magic items measured and
  stated "on this machine".
- **Depends:** E1.10. **Tier:** opus.

### E1.12 — Sell names the copy (discovered by E0.4)
- **Objective:** the Gear screen's Sell (and the Market inventory's) removes the copy the player clicked. Today
  `handleGearSell` (`ui/characterHub/CharacterSheet.tsx`) sends `itemId` + `bonus` + `fromState`, and
  `sell_held_pieces_at_root` (`desktop/character_hub.rs:5635`) takes `first_copy_in_hand` (`desktop/pf1_adapter.rs:1349`),
  so with several Sacks it sells a different one (E0.4 `artifacts/epic_0/gear-repro.md` §12.2: clicked `Sack|0|2`,
  removed `Sack|0|1` and its custom name). Drop, Discard and Move already send `copies[0]`.
- **FILES YOU OWN:** `desktop/character_hub.rs` (sell path and its tests), the remove/sell request DTO and its
  command registration if it changes, `ui/boundary/removeEquipmentSelection` (or its file), `ui/characterHub/CharacterSheet.tsx`
  (`handleGearSell`, `handleMarketSell`) (`hub`). Closure check first: `git grep -n "removeEquipmentSelection\|sell_held_pieces_at_root"`.
- **Failing test first:** two Sack lines Carried, the second holding a Bedroll; sell the second by its copy key →
  the second line is gone, the first stays, the Bedroll is Carried.
- **Acceptance:** `cd apps/desktop/src-tauri && cargo test sell_names_the_copy` (≥ 1 test, 0 filtered = FAIL); a
  frontend test that `handleGearSell`'s request carries `row.held.copies[0]`.
- **Depends:** E1.5. **Tier:** opus.
- **Does not cover:** what the sale pays (E1.6) and Drop/Discard (already by copy key).

### E1.13 — the sell window's choice under Cashless (discovered by E1.6)
- **Objective:** `decisions.md §10` / §30 Q-9.1: the Gear sell window returns money per its own option even when
  the session Market mode is Cashless. Today `sell_held_pieces_at_root` (`desktop/character_hub.rs`, `(PriceMode::Cashless, _) => 0`)
  ignores `negotiated_copper` under Cashless, and `handleGearSell` (`ui/characterHub/CharacterSheet.tsx`) sends the
  session `priceMode`. Fix: `handleGearSell` sends `priceMode: 'standard'` whenever `negotiatedGp` is given (the
  window always sends its total when a price is listed), so the backend pays the window's total; the Market's own Sell is unchanged.
- **FILES YOU OWN:** `ui/characterHub/CharacterSheet.tsx` (`handleGearSell`) (`hub`) + a frontend test.
- **Failing test first:** with the session mode `cashless`, the gear sell request for a window total of 7.5 gp carries a
  non-cashless `priceMode` and `negotiatedPriceCopper: 750`.
- **Depends:** E1.6. **Tier:** sonnet.
- **Does not cover:** the Market window's own Sell (Cashless there still pays nothing, Q-12.2).

### E1.14 — Market inventory sell value for every held copy (discovered by E1.MC)
- **Objective:** `decisions.md §10`: the Market inventory's value cell and its Negotiate prefill show
  `sellNegotiateGp` for every held copy with a listed price, whatever list is open. Today `rowOfCopy`
  (`ui/characterHub/MarketDialog.tsx:702`) searches only the lists already loaded (`loaded`), so on a fresh Market
  opened on Gear a Longsword (15 gp) and a Chain Shirt (100 gp) both read `—` under Buy 100% / Sell 50%; each shows
  its value only after its own list (Weapons, Armor) has been opened (E1.MC images `03`, `04`, `05`).
- **FILES YOU OWN:** `ui/characterHub/MarketDialog.tsx` (`rowOfCopy`, list loading), `MarketDialog.test.ts`; a
  backend read only if the dialog cannot resolve a held copy's row without one (then the command, its registration
  and boundary file are named in the receipt first).
- **Failing test first:** with only the Gear list loaded, a held Longsword's sell value under `standard` is 7.5 gp,
  not `null`.
- **Depends:** E1.8. **Tier:** sonnet.
- **Does not cover:** Cashless (`—` by Q-12.2).

### E1.15 — Gear Sell opens on the first click (discovered by E1.MC)
- **Objective:** a Gear-screen Sell click always opens the sell window. E1.6 (retro incident, "first two Sell clicks
  after a fresh launch opened no window while Customize opened") and E1.MC (images `13`, `14`, `16`: two Sell clicks
  on a Bedroll in a Backpack opened nothing; Customize opened; the next Sell click opened the window) saw it on a
  debug build under Xvfb. Cause not established; diagnose first (state of `pending` / `props.busy` at the click).
- **FILES YOU OWN:** `ui/characterHub/GearScreen.tsx`, `SellDialog.tsx`, their tests; others named after diagnosis.
- **Failing test first:** the reproduced mechanism as a test (a Sell click on a fresh screen sets the sell window open).
- **Depends:** E1.6. **Tier:** opus.
- **Does not cover:** the Market window's own Sell.

### E1.MC — E1 merge check
- **Objective:** adversarial review of E1's diff against the rulings; screenshots of gear (all sections),
  Market (each list, a filter), sell window; the four-check audit over the epic; epic retro summary.
- **Acceptance:** `npm test && npm run typecheck`; `cd apps/desktop/src-tauri && cargo test` (whole crate),
  every `test result:` attributed to its `Running` line. **Tier:** opus.

## Epic E2 — sheet layout, campaign button, rest text (items 17, 21, 18, 2)

### E2.1 — fixed layout, PF sheet (items 17 + 21, one cycle)
- **Objective:** per `decisions.md §24`; one shared layout component the SF/CPR sheets adopt in E2.2.
- **FILES YOU OWN:** `ui/characterHub/SheetLayout.tsx` + test (new), `CharacterSheet.tsx` (`hub`).
- **Failing test first:** a layout model test that the root is `100dvh` with no page scroll and the centre is the
  only `overflow: auto` region (plus side columns per Q-17.1).
- **Acceptance:** the test; screenshots at full height, at −40 px and at the smallest allowed window with Show
  Progression fully visible; dialogs and the progression drawer above the bar.
- **Depends:** E1.7. **Tier:** opus.

### E2.2 — SF and CPR sheets
- **FILES YOU OWN:** `StarfinderCharacterSheet.tsx`, `CyberpunkCharacterSheet.tsx`, `SheetLayout.tsx`.
- **Failing test first:** each sheet renders inside `SheetLayout`.
- **Acceptance:** the test; screenshots of both. **Depends:** E2.1. **Tier:** sonnet.

### E2.3 — the campaign button (item 18)
- **FILES YOU OWN:** `CharacterSheet.tsx` (`hub`), `CharacterHubPage.tsx`, `ui/campaign/CampaignManagerScreen.tsx`,
  the membership DTO producer if the path is not on it (to verify in the card), tests.
- **Failing test first:** clicking Open with a real membership row calls the handler with an absolute path (fails
  today: passes the UUID).
- **Acceptance:** the test; one banner per membership; the Manager banner's height stated before/after.
- **Depends:** E2.2. **Tier:** opus.

### E2.4 — rest and recovery text (item 2)
- **Objective:** per `decisions.md §8`: the cited table, one engine command, the print block after the bio page,
  the `custom.json` override field.
- **FILES YOU OWN:** the table file (a new JSON under `data/` read through `repo_root()`) **plus** its
  `tauri.conf.json` `bundle.resources` entry **and** an assertion in `apps/desktop/src-tauri/tests/packaged_resources.rs`;
  `src/rules_core/rest_text.rs` (new); `desktop/print/blocks/rest.rs` (new) and `print/layout/mod.rs:312`
  region; `desktop/character_custom.rs` (one field); `desktop/main.rs` (`hub`).
- **Failing test first:** a Wizard 5 / Elf prints its rows under the bio page; with the override set, the
  override prints instead.
- **Acceptance:** those tests; `cd apps/desktop/src-tauri && cargo test --test packaged_resources`; every table
  row has a citation (a test).
- **Depends:** E0.5, E2.3. **Tier:** opus.

### E2.5 — rest text on the Biography tab (item 2 adjustment)
- **FILES YOU OWN:** `CharacterSheet.tsx` (`hub`, the Biography branch at `:4532`), a boundary file for the
  command.
- **Failing test first:** after a blur on the biography field, the saved `bio.biography` does not contain the rest
  text; the rest text renders below the field in the print order.
- **Acceptance:** the test. **Depends:** E2.4. **Tier:** sonnet.

### E2.6 — rest-and-recovery override editor (discovered by E2.MC)
- **Objective:** `decisions.md §8`: the override is set "in the custom rules menu". E2.4 added the
  `custom.json` field (`character_custom.rs:170` `rest_text`, length-checked `:273`) and both surfaces honour it,
  but no screen writes it: `git grep -n restText -- apps/desktop/src` hits only `boundary/loadRestText.ts` (a
  reader); `boundary/characterCustom.ts` carries no `restText`. A player can set it only by editing `custom.json`.
- **FILES YOU OWN:** `CharacterSheet.tsx` (`hub`, the Custom dialog), `boundary/characterCustom.ts` (the DTO
  field), their tests; `character_custom.rs` only if the save path drops the field (named in the receipt first).
- **Failing test first:** saving the Custom dialog with rest text "X" writes `restText: "X"`; the Biography tab and
  `load_rest_text` then return the override; clearing it (empty) removes the field and the table rows return.
- **Acceptance:** the test; a screenshot of the field and of the Biography tab with the override. **Depends:** E2.5.
  **Tier:** sonnet.
- **Does not cover:** per-line override (Q-2.2 whole-text replace); the House Rules menu (E6.2).

### E2.MC — E2 merge check
- **Acceptance:** screenshots of all three sheets at a short window; the PDF's bio page with the rest block
  (`pdftotext` excerpt); the four-check audit. **Tier:** opus.

## Epic E3 — the Abilities tab (items 22, 22a, 22b)

Design: `technical-design.md §3`. Every sub-tab card is vertical (engine → command → UI → saved → reload test),
so no card ships a picker the engine does not check.

### E3.1 — the tab shell
- **Objective:** `SHEET_TABS` `Feats` → `Abilities` (OPERATOR 2026-11-22, Q-22.3: rename); seven sub-tabs; Feats sub-tab = `FeatsTab`
  unchanged; `TraitsSection`, `RacialTraitsSection`, the pool reference lists and
  `ClassFeatureDescriptionReferenceSection` moved to their sub-tabs. Sub-tabs whose engine card has not landed
  show a notice naming that card; E3.MC asserts none remain.
- **FILES YOU OWN:** `CharacterSheet.tsx` (`hub`), `ui/characterHub/AbilitiesTab.tsx` (new) + test.
- **Failing test first:** `AbilitiesTab.test.ts`: seven sub-tab labels in PCGen's order; Feats sub-tab renders
  the feat budget line.
- **Acceptance:** the test; the existing FeatsTab tests unchanged and green. **Depends:** E0.1, E2.5.
  **Tier:** opus.

### E3.2 — the generic engine, proved on Traits
- **Objective:** `src/rules_core/ability_budget.rs` (new) in `feat_budget.rs`'s shape: pools per sub-tab with
  sources per level, `explanation_rows()` → `abilities.budget.<subtab>.<pool>.{granted,spent,remaining}`,
  `refusal()`; a saved selections field on `ChosenCharacterState` with `#[serde(default)]`; commands
  `list_ability_options` (each option with `qualified` and the unmet reasons), `add_ability_selection`,
  `remove_ability_selection`, registered in `main.rs`. Traits: the budget and the one-per-category rule as the
  books state them (E0.1 cites the source).
- **FILES YOU OWN:** `src/rules_core/ability_budget.rs`, `src/rules_core/mod.rs`, `src/rules_core/character_input.rs`
  (field), the saved-character serializer, `desktop/abilities.rs` (new), `desktop/main.rs` and
  `desktop/character_hub.rs` (rows into the load response) (`hub`), `ui/boundary/abilities.ts` (new),
  `apps/desktop/src-tauri/tests/old_save_loads_unchanged.rs` (a new case).
- **Failing test first:** a Traits case: a level-1 character with two traits → `remaining 0`; a third is refused
  with the pool named; two traits of one category → refused with the category named.
- **Acceptance:** `cargo test --lib ability_budget` (root) ≥ 1; `cd apps/desktop/src-tauri && cargo test --test
  old_save_loads_unchanged` green.
- **Depends:** E3.1. **Tier:** opus.

### E3.3 — the generic sub-tab UI on Traits
- **Objective:** `AbilitySubTab.tsx` (new): Available (search, Qualified filter, unqualified drawn differently),
  Selected (with Choices), Info pane, remaining per pool, Add/Remove disabled with the engine's reason. Traits
  sub-tab on it.
- **FILES YOU OWN:** `ui/characterHub/AbilitySubTab.tsx` + `abilitySubTabModel.ts` + tests (new),
  `AbilitiesTab.tsx`, `CharacterSheet.tsx` (`hub`).
- **Failing test first:** the model renders `granted/spent/remaining` from rows only; a test that no number in the
  model is computed from anything but rows.
- **Acceptance:** the tests; `npm test && npm run typecheck`. **Depends:** E3.2. **Tier:** opus.

### E3.4 — Class Features: grants per level
- **Objective:** pool sizes per level for every Class Features pool in the catalog (not two), from the class
  records' tokens E0.1 named; rows per pool with the class level that granted each pick.
- **FILES YOU OWN:** `src/rules_core/ability_budget.rs`, `src/rules_core/class_feature_pool_catalog.rs` (read
  API only), `desktop/abilities.rs`, `AbilitiesTab.tsx`; `POOL_REFERENCE_SECTIONS` removed from
  `CharacterSheet.tsx` (`hub`).
- **Failing test first:** Rogue 6 → Rogue Talent granted 3; Barbarian 4 → Rage Power 2; Ninja 2 → Ninja Trick 1
  (the expected values transcribed from the book text, not from the engine's read path).
- **Acceptance:** those tests; a census test that every pool in the catalog with a class owner yields a grant row
  or appears in E0.1's data-gap list. **Depends:** E3.3. **Tier:** opus.
- **Split rule:** if E0.1 shows more than one token shape for pool size, this card covers the shapes for the
  pools named in the test and files a discovered card per further shape before E3.5 starts.

### E3.5 — Class Features: prerequisites
- **Objective:** evaluate pool members' `PRE*` tokens (the shapes E0.1 counted) in the engine; options carry
  `qualified` and the unmet reasons; refusal names them.
- **FILES YOU OWN:** `src/rules_core/ability_prereqs.rs` (new, reusing `feat_prereqs/` evaluators where the
  token shape matches), `ability_budget.rs`, `desktop/abilities.rs`.
- **Failing test first:** Silent Kill without Assassinate → refused, reason names Assassinate.
- **Acceptance:** that test; a coverage count over every pool member: evaluated / unknown token shape (the
  unknown count must be 0 at E3.MC, or each unknown shape is a discovered card). **Depends:** E3.4. **Tier:** opus.

### E3.6 — Racial Abilities
- **Objective:** alternate racial traits selectable on the sheet, reusing `list_alternate_racial_traits` /
  `resolve_race_alternate_selection` (`desktop/race_trait_picker.rs:1067`, `:1075`); two alternates replacing
  the same trait refused.
- **FILES YOU OWN:** `ability_budget.rs`, `desktop/abilities.rs`, `desktop/race_trait_picker.rs` (if a shared
  function is extracted), `AbilitiesTab.tsx`.
- **Failing test first:** a Human with an alternate that replaces the bonus feat; a second alternate replacing it
  refused; the feat budget's racial pool follows (`feat_budget.rs` reads replaced traits).
- **Acceptance:** those tests. **Depends:** E3.5. **Tier:** opus.

### E3.7 — Archetype
- **Objective:** select an archetype per held class as `SelectedChoice{choice:archetype}`
  (`src/rules_core/archetype_resolver.rs:41`); two archetypes claiming the same slot refused (`archetype_claims_slot`);
  archetype-granted pools feed E3.4.
- **FILES YOU OWN:** `ability_budget.rs`, `archetype_resolver.rs` (read API), `desktop/abilities.rs`,
  `AbilitiesTab.tsx`.
- **Failing test first:** Rogue with two archetypes that both replace Trapfinding → refused, reason names the
  slot.
- **Acceptance:** the test. **Depends:** E3.6. **Tier:** opus.

### E3.8 — favored class bonus
- **Objective:** per `decisions.md §19`.
- **FILES YOU OWN:** `ability_budget.rs`, the HP and skill-point producers that read it (to verify:
  `grep -rn "favored_hit_points\|skill_points" src/rules_core apps/desktop/src-tauri/src --include=*.rs`),
  `desktop/abilities.rs`, `AbilitiesTab.tsx`, the PCGen import mapping only if a double count is found.
- **Failing test first:** Fighter 3 favored Fighter with HP at 1–3 → HP total +3; skill at level 2 → skill pool +1;
  an imported character with `favored_hit_points` grants is not counted twice.
- **Acceptance:** those tests. **Depends:** E3.7. **Tier:** opus.

### E3.9 — Background and Misc
- **Objective:** per E0.1's mapping: engine budget and prerequisites for whatever records exist; a data-gap
  sentence where E0.1 found none.
- **FILES YOU OWN:** `ability_budget.rs`, `desktop/abilities.rs`, `AbilitiesTab.tsx`.
- **Failing test first:** written from E0.1's mapping (the card states it).
- **Acceptance:** each sub-tab either renders engine rows or the data-gap sentence matching E0.1's list.
  **Depends:** E3.8. **Tier:** opus.

### E3.10 — overrides
- **Objective:** `abilities_without_prerequisites`, `ignore_ability_budget` in `Overrides`
  (`desktop/character_custom.rs:131`); admitted picks show their flag.
- **FILES YOU OWN:** `desktop/character_custom.rs`, `desktop/character_ledger.rs` (admitted records),
  `desktop/abilities.rs`, `AbilitySubTab.tsx`, `ui/characterHub/customModel.ts`.
- **Failing test first:** with the flag on, an unqualified pick is admitted and flagged; off, refused.
- **Acceptance:** the test. **Depends:** E3.9. **Tier:** sonnet.

### E3.11 — printout
- **Objective:** per `decisions.md §20`.
- **FILES YOU OWN:** `desktop/print/blocks/abilities.rs` (new), `print/layout/mod.rs`, print golden files if the
  golden test covers the page.
- **Failing test first:** a Rogue 6 with three talents and a favored class prints each under its heading.
- **Acceptance:** `cd apps/desktop/src-tauri && cargo test print_pf_blocks_chosen_abilities_per_subtab` ≥ 1 (a new test; the old filter `print_pf_blocks_abilities` matched the existing six-boxes test, run 6 RED exit 0). **Depends:** E3.10.
  **Tier:** opus.

### E3.12 — ingest-gap pools get members (discovered by E0.1, 2026-11-22)
- **Objective:** close the 30 ingest-cause entries of E0.1's data-gap list (`artifacts/epic_0/abilities-census.json`
  `data_gap_list[]`, 34 entries: 30 `cause` starting `ingest:`, 4 `the books in the corpus define no members` —
  `python3 -c "import json,collections;g=json.load(open('artifacts/epic_0/abilities-census.json'))['data_gap_list'];print(len(g),sum(x['cause'].startswith('ingest:') for x in g))"`
  → `34 30`): apply the oracle's member typing (`<record>.MOD TYPE:` lines) in the ingest so those pools get members;
  add the missing `Ninja ~ Ninja Trick` grant row (level 2) to `data/class_feature_grants`; resolve the 7 pool names with
  no ABILITYCATEGORY (E0.1 receipt, "Discovered cards").
- **FILES YOU OWN:** the ingest step that applies `.MOD` lines (located by the card: `git grep -n 'MOD' crates/codex-ingest/src/pcgen_import`),
  its test, the regenerated corpus records it writes, `data/class_feature_grants`; second call sites by `git grep` per the prefix.
  Nothing from the oracle is read at runtime (prefix: THE ORACLE).
- **Failing test first:** a test that the 30 ingest-gap pools have ≥ 1 member each fails today (E0.1: 0 members).
- **Acceptance:** `python3 -I artifacts/epic_0/abilities_census.py --check` exits 0 and the re-run census reports
  `data_gap_list` = 4 entries, every one `the books in the corpus define no members`; the new test green; root
  `cargo test --locked -j 8 --lib -- --test-threads=8` and the desktop workspace green. **Depends:** E0.1 (lane order:
  after E3.11, before E3.MC). **Tier:** opus.
- **Does not cover:** the 4 book gaps (they stay as on-screen data-gap notices, E3.MC).

### E3.13 — level-1 ABILITYSELECTION picks on Class Features (discovered by E3.4, 2026-11-23)
- **Objective:** E3.4's split rule: the pool-size shape that is not a `Pool` grant. E0.1 counts 5 class-owned pools whose
  only mechanism is `abilityselection` (Blood of Dragons Bloodline, Bloodrager Bloodline, Crossblooded Bloodline,
  Crossblooded Rager Bloodline, Sorcerer Bloodline); the converter writes them as an `offers` with a count and no
  `Pool` target (`core_rulebook:class_feature:sorcerer_standard_bloodline_selection`, count 1, from pool
  `sorcerer_bloodline`). Give them `abilities.budget.class_features.<pool>.*` rows sized by the offer's count, members
  from the offer, and picks saved in `ability_selections`; reconcile with the per-class choice ids creation already
  records (`selected_choices`) so a bloodline is not held twice.
- **Figure:** `python3 -c "import json;p=json.load(open('docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_0/abilities-census.json'))['class_feature_pools']['pools'];print(sorted(n for n,v in p.items() if 'class' in v['granted_by'] and v['mechanisms']==['abilityselection']))"` → the 5 names above.
- **FILES YOU OWN:** `src/rules_core/ability_budget.rs`, `apps/desktop/src-tauri/src/abilities.rs`; second call sites per the prefix.
- **Failing test first:** Sorcerer 1 → `abilities.budget.class_features.sorcerer_bloodline.granted` = 1 (CRB: a sorcerer
  chooses one bloodline at 1st level); E3.4's census test then drops `ABILITY_SELECTION_POOLS_SPLIT_OFF` to empty.
- **Acceptance:** that test; E3.4's census test green with no split-off list. **Depends:** E3.4. **Tier:** opus.

### E3.14 — members for pools whose grants carry no offer (discovered by E3.4, 2026-11-23)
- **Objective:** 31 of the 148 class-owned class-feature pools (E0.1 census, class among `granted_by`, a converted
  `Pool` grant, not a feat pool) have no `offers` on any grant rule, so E3.4's sub-tab shows their pool line with no
  members: the ABILITYCATEGORY's member filter (`TYPE`) never reached the package. The converter carries each pool's
  member filter (an `offers` or a pool → tags table in the package) so `class_feature_options` lists them; until then
  the sub-tab names this card for a pool with no members.
- **Figure:** `python3 -I docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_3/E3.4/pool_shapes.py .` printed `class-owned 181 no Pool target 5 feat pools 28 class-feature pools 148 of which no offer 31`
  (the list: 20 oracle mystery revelation pools, 5 kineticist element pools, Amateur Gunslinger, Craft Skill Bonus,
  Guileful Polyglot Additional Language, Gun Training Choice, Gunslinger Battered Gun, Samurai Expertise, Shaman Hex —
  the script prints it).
- **FILES YOU OWN:** the sheet-rule converter's offer emission (located by the card: `git grep -n "offers" crates/ src/rules_core/sheet_rule*.rs`),
  the regenerated `data/sheet_rules` records, `src/rules_core/ability_budget.rs`.
- **Failing test first:** an Oracle 1 with the Flame mystery lists ≥ 1 Flame revelation option (0 today).
- **Acceptance:** that test; a census that every class-owned class-feature pool with a grant lists ≥ 1 member or is on
  E0.1's data-gap list. **Depends:** E3.4. **Tier:** opus.
### E3.15 — Class Features picks held by the sheet (discovered by E3.5, 2026-11-23)
- **Objective:** a saved Class Features pick (`ChosenCharacterState.ability_selections`, subtab `class_features`) is
  held wherever the character's held set is built, so the pick's own rules (a bonus, a granted pool, a printed
  line) reach the sheet and the print. E3.5 holds picks only where the Class Features options and grants are
  evaluated (`ability_budget::picked_rules`); `HeldSeed::from_record` (`src/rules_core/sheet_rule.rs`) still omits them.
- **FILES YOU OWN:** `src/rules_core/sheet_rule.rs` (`HeldSeed::from_record`), `src/rules_core/ability_budget.rs`
  (drop `picked_rules` once the seed carries the picks), the sheet's load path in `desktop/character_hub.rs` if a test needs it.
- **Failing test first:** a Rogue 4 holding the Minor Magic pick: the sheet's held set (`held_set` over
  `HeldSeed::from_character`) holds `core_rulebook:class_feature:rogue_talent_minor_magic` (absent today).
- **Acceptance:** that test; root and desktop suites green. **Depends:** E3.5. **Tier:** opus.
### E3.16 — Archetypes held by the sheet (discovered by E3.7, 2026-11-23)
- **Objective:** a saved archetype (`selected_choices`, `choice:archetype`, `archetype:<KEY>` or a record id) is held
  wherever the character's held set is built, so the class features its record replaces leave the sheet and the print
  (their converted gates read `RuleTag archetype` / the archetype record). E3.7 holds the record only where the
  Class Features options and grants are read (`ability_budget::picked_rules`); `HeldSeed::from_record`
  (`src/rules_core/sheet_rule.rs`) still omits it, so Brother Tomas Vane with Two-Handed Fighter keeps the
  "Will: Bravery: +1 Will vs. fear" conditional note (`artifacts/epic_3/E3.7/07-sheet-reloaded.png`).
- **FILES YOU OWN:** `src/rules_core/sheet_rule.rs` (`HeldSeed`), `src/rules_core/ability_budget.rs` (the selection →
  record resolver it can reuse; drop the archetype half of `picked_rules` once the seed carries it), the sheet's load
  path in `desktop/character_hub.rs` if a test needs it.
- **Failing test first:** a Fighter 2 holding `archetype:Fighter Archetype ~ Two-Handed Fighter`: the sheet's held set
  (`held_set` over `HeldSeed::from_character`) holds `advanced_players_guide:class_feature:fighter_archetype_two_handed_fighter`
  and no Bravery conditional note (both absent / present today).
- **Acceptance:** that test; root and desktop suites green. **Depends:** E3.7. **Tier:** opus.
### E3.17 — Background and Misc picks held by the sheet (discovered by E3.9, 2026-11-23)
- **Objective:** a saved Background or Misc pick (`ability_selections` with sub-tab `background` or `misc`) is held
  wherever the character's held set is built, so the pick's own rules reach the sheet and the print (Fleshforge
  Enhancement's +2 to an ability score; a corruption's rule text). E3.9 holds the picks only where the Background and
  Misc pools and options are read (`ability_budget::picked_rules`); `HeldSeed::from_record`
  (`src/rules_core/sheet_rule.rs`) still omits them. Same mechanism as E3.15 (Class Features picks); one change may close both.
- **FILES YOU OWN:** `src/rules_core/sheet_rule.rs` (`HeldSeed`), `src/rules_core/ability_budget.rs` (drop the
  Background/Misc half of `picked_rules` once the seed carries the picks), the sheet's load path in `desktop/character_hub.rs` if a test needs it.
- **Failing test first:** a Fighter 1 holding the Misc pick `optional_rules_systems:horror_adventures:ability:rules_use_corruption_standard`:
  the sheet's held set (`held_set` over `HeldSeed::from_character`) holds `horror_adventures:ability:rules_use_corruption_standard` (absent today).
- **Acceptance:** that test; root and desktop suites green. **Depends:** E3.9. **Tier:** opus.
### E3.18 — E3.12's pools list their members on the sheet (discovered by E3.12, 2026-11-24)
- **Objective:** E3.12 gave the corpus members for 29 class-granted pools, but the sheet lists a Class Features pool's
  members from a converted grant's `offers` (`ability_budget::pool_offer`), and for 10 of the 29 no converted rule
  granting into the pool carries an offer that selects a member: Advanced Masked Performer Talent, Druid Eagle / Lion /
  Serpent / Wolf Shaman Feat, Finesse / Sniper / Volley Marksman Bonus Feat, Hexenhammer Major Hex Choice (no offer: a
  second `ABILITYPOOL` row on one record, `#bonusN`, carries none) and Lantern Bearer Favored Enemy (offer is a
  `Number` 2–4). Re-derive: `python3 -I artifacts/epic_3/E3.12/offer_probe.py | tail -1` → `10 of 29`. A PROXY: it reads
  the package, not the engine, and feat-parent pools may be listed by `feat_budget` (`feat_list_of`) instead — the
  card's first step is the engine test below, which settles the real count.
- **FILES YOU OWN:** the sheet-rule converter's pool offer (`crates/codex-ingest/src/pcgen_import/sheet_rule/pool_pick.rs`
  and its ABILITYPOOL caller) or the engine's member fallback (`src/rules_core/ability_budget.rs`), whichever the test
  shows; the regenerated `data/sheet_rules` rows if the converter changes.
- **Failing test first:** for each of the 29 pools, a character holding one of its grants is offered ≥ 1 member by
  `ability_options` (Class Features) or the Feats options (feat-parent pools).
- **Acceptance:** that test; root and desktop suites green. **Depends:** E3.12. **Tier:** opus. Must be complete
  before E3.MC (else those pools show a count with nothing to pick).
### E3.19 — Class grant rows from the global-variable files (discovered by E3.12, 2026-11-24)
- **Objective:** `gen_class_feature_grants` reads only `class_feature::BOOK_PRIMARY_FILES`
  (`uc_abilities_class.lst` for Ultimate Combat), but the Ninja and Samurai grant rows live in
  `uc_abilities_globalvar.lst` (`awk -F'\t' '/^CATEGORY=Class\|/ {split($1,a,"|"); c[a[2]]++} END{for(k in c) print c[k], k}'
  "$PCGEN_CORPUS_ROOT/pathfinder/paizo/roleplaying_game/ultimate_combat/uc_abilities_globalvar.lst"` → Ninja.MOD 64,
  Samurai.MOD 99, Gunslinger.MOD 98): `data/class_feature_grants/ultimate_combat/` had no `ninja.json` or `samurai.json`.
  E3.12 hand-added the one row its card named (`Ninja ~ Ninja Trick`, level 2, cited to `uc_abilities_globalvar.lst:21`).
  The generator also cannot be re-run as is: on the E3.12 tree it rewrote 58 committed files
  (`cargo run --locked -p codex-ingest --bin gen_class_feature_grants && git status --porcelain data/class_feature_grants | awk 'END{print NR}'`).
- **FILES YOU OWN:** `crates/codex-ingest/src/pcgen_import/cache_gen/class_feature_grants.rs`, its bin, `data/class_feature_grants`.
- **Failing test first:** the UC Ninja table holds every `Ninja.MOD` grant row of `uc_abilities_globalvar.lst` the
  generator's rules resolve (1 of 64 today); a generator re-run leaves `data/class_feature_grants` unchanged.
- **Acceptance:** that test; root and desktop suites green. **Depends:** E3.12. **Tier:** opus. Must be complete before E3.MC.
### E3.20 — the Bloodline kind pick beside the held bloodline (discovered by E3.13, 2026-11-24)
- **Objective:** on a Sorcerer 1 whose creation bloodline is held (E3.13: `Sorcerer bloodline: 1 of 1 spent, 0 remaining`),
  the Class Features sub-tab still reads `Bloodline: 0 of 1 spent, 1 remaining` (the pool `core_rulebook:class_feature:sorcerer_standard_bloodline`
  grants, `Pool(bloodline)` 1; PCGen's `Bloodline` category, `cr_abilitycategories.lst:127`, members typed `Bloodline Selection`), and its member
  Standard Bloodline (`sorcerer_standard_bloodline_selection`) is struck `requires Bloodline` although the record its gate names is held
  (the same gate opens the Sorcerer bloodline pool: `chooser_offered`). Seen on screen:
  `artifacts/epic_3/E3.13/01-sorcerer-1-class-features.png`, `02-standard-bloodline-pool.png`. The bloodline kind a held bloodline implies
  (Standard unless an archetype's selection is held) fills that pool, and the option's verdict (`ability_prereqs::unmet`) agrees with the gate.
- **FILES YOU OWN:** `src/rules_core/ability_budget.rs`, `src/rules_core/ability_prereqs.rs`; second call sites per the prefix.
- **Failing test first:** Sorcerer 1 (canonical seeds) → `abilities.budget.class_features.bloodline.remaining` = 0, and no Class Features
  option of a held record's own gate reads unmet.
- **Acceptance:** that test; root and desktop suites green. **Depends:** E3.13. **Tier:** opus. Must be complete before E3.MC.
### E3.21 — Feats pools filtered by named feats take only those (discovered by E3.18, 2026-11-24)
- **Objective:** E3.18 made a Feats-sub-tab pool whose grant carries no offer read its member filter's `any_tags`
  (`feat_budget::feat_grant_of`), so Druid Eagle Shaman Feat takes only `DruidShamanEagle` feats. A filter that names
  only `ABILITYLIST:` rules is not read yet, so those pools still take any feat. Re-derive the population (a PROXY:
  pools named for feats, `feat_budget::named_for_the_characters_feats`):
  `python3 -I -c "import json; d=json.load(open('data/sheet_rules/_pool_members.json')); print(sum(1 for k,v in d.items() if v['member_pool']=='feat' and not v['any_tags'] and any(w in ('feat','feats') for w in k.split('_')) and 'companion' not in k.split('_')))"`
  → 11 (Amateur Gunslinger Feat, Ape / Bat / Boar Shaman Feat, Canine Ferocity Feat, Contemplative Knowledge Bonus Feat,
  Gendarme Bonus Feat, Healing Surge Feat, Master Craftsman Feat, Rapid Reload Feat, Unlocked Talent Feat).
- **FILES YOU OWN:** `src/rules_core/feat_budget.rs` (`FeatPool` admission, `HeldFeat`, `feat_grant_of`'s filter fallback).
- **Failing test first:** for each such pool, a character holding its grant has the pool, it admits a feat the filter names
  (Ape Shaman Feat: Toughness) and refuses one it does not.
- **Acceptance:** that test; root and desktop suites green. **Depends:** E3.18. **Tier:** opus. Must be complete before E3.MC.
### E3.22 — PI screening of the member typing field (discovered by E3.19, 2026-11-25)
- **Objective:** E3.12's `enrich_member_typing` wrote `data.member_types` on 723 corpus records without the PI screen the
  other generators apply, and 28 of them carry a live blacklist term:
  `bash scripts/verify.sh --only declared-pi-audit -j 8` -> `declared-pi-audit: FAIL — 28 violation(s) across 28 file(s)`,
  every hit `BLACKLIST-TERM-SHIPPED: data.member_types[<i>]`; `git log -1 --format=%h -- <file>` -> `233dcf906f` on each.
  `declared-pi-audit` is in `verify.sh`'s QUICK stages, so the tranche's quick gate is red until this lands.
- **FILES YOU OWN:** `crates/codex-ingest/src/pcgen_import/member_typing.rs`, `crates/codex-ingest/src/bin/enrich_member_typing.rs`,
  the 28 `data/corpus/**` records (and any other record the screened re-run rewrites); the census reads the field, so
  E0.1's `abilities_census.py --check` must stay green (a fence exit if it moves).
- **Failing test first:** a member_typing test that no written `member_types` tag carries a blacklist term (the screen
  `pi_screening::classify_field` applies); then the re-run, `declared-pi-audit` PASS.
- **Acceptance:** that test; `declared-pi-audit` PASS; E3.18's pools still list a member; root and desktop suites green.
  **Depends:** E3.12. **Tier:** opus. Must be complete before E3.MC.

### E3.23 — A gate that names its own key (discovered by E3.21, 2026-11-25)
- **Objective:** PCGen writes `PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,<own KEY>],[...]` so a record applies
  when it is held. The converter writes that self reference as `MissingRule { name: <own KEY> }`, which
  `sheet_rule.rs` reads as false, so the branch can never be met. Blue ~ Psionic Prodigy
  (`ultimate_psionics:monster_ability:blue_psionic_prodigy`, UP p.311, `BONUS:ABILITYPOOL|Unlocked Talent Feat|1`) is
  gated `AtLeast 1 of [MissingRule(self), Not(holds BlueStealthy or BlueRepletion)]`, and it carries both tags itself,
  so no character is granted `unlocked_talent_feat`: the one E3.21 pool (of 11) its test cannot hold. Population
  (a PROXY: a `MissingRule` name whose slug equals the rule's own id slug):
  `python3 -I docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_3/E3.21/self_key_probe.py`
  → 136 of 1260 rules with a `MissingRule` in `applies`.
- **FILES YOU OWN:** `crates/codex-ingest/src/pcgen_import/sheet_rule/prereq.rs` (key resolution) and the regenerated
  `data/sheet_rules/**` records it rewrites; the closure check names the rest.
- **Failing test first:** a converter test that the self-key `PREABILITY` resolves to the record's own id; then a
  `feat_budget` test that a Blue holding Psionic Prodigy has `unlocked_talent_feat` (1) taking only Unlocked Talent,
  which completes E3.21's 11th pool.
- **Acceptance:** those tests; the probe's self-key count 0 or each remaining one named; root and desktop suites green.
  **Depends:** E3.21. **Tier:** opus. Must be complete before E3.MC.
### E3.MC — E3 merge check
- **Acceptance:** every sub-tab exercised on three seeds (a Rogue/Ninja, a Barbarian, a Human Fighter with an
  archetype); the count of on-screen gap notices equals E0.1's data-gap list and no engine-gap notice remains
  (`grep -rn "engine card" apps/desktop/src/characterHub/AbilitiesTab.tsx | awk 'END{print NR}'` → 0); the
  four-check audit; whole-crate tests attributed. **Tier:** opus.

## Epic E4 — item names (item 20)

### E4.1 — one display name; the guard; the proposal
- **Objective:** derive the display name from the variant (`data.key`) in one place every screen reads; one test
  over all item kinds fails on `[`, `]`, `NAME`; a proposed-names list for every hit (E0.3) written from the book
  text to `artifacts/epic_4/proposed-names.md`.
- **FILES YOU OWN:** the name function's file (named in the card after reading the catalog builders:
  `desktop/equipment_catalog.rs` and the other kinds' catalogs), its test, the proposal.
- **Failing test first:** the all-kinds test fails today (201 hits).
- **Acceptance:** the test fails only on the hits the proposal covers. **Depends:** E0.3, E3.MC (lane order).
  **Tier:** opus.

### E4.G — operator approval (the bundle's one wait)
- **Objective:** the operator reads `proposed-names.md` and answers approve / edit; confirms the reading of "1".
- **Acceptance:** the operator's reply quoted in `progress.md`. **Tier:** operator.

### E4.2 — ship the approved names
- **FILES YOU OWN:** the name table (data file + `bundle.resources` + `packaged_resources.rs` if read at runtime),
  the name function.
- **Acceptance:** the all-kinds test green; the Market shows `Hill Giant-hide` (screenshot); E4 epic report.
  **Depends:** E4.G. **Tier:** sonnet.

## Epic E5 — reports, Update menu, Developer tab (items 13, 15, 14, 16, 19)

### E5.1 — report API client (item 13)
- **Objective:** per `decisions.md §23`.
- **FILES YOU OWN:** `desktop/report_api.rs` (new), `desktop/main.rs` (`hub`), the build-time key stamp (a
  `build.rs` env read or `option_env!`, named in the card), `.github/workflows/publish-tester-release.yml` (the
  secret into the build env only), `ui/boundary/sendReport.ts` (new), tests.
- **Failing test first:** a test against a local stub server (test-only) for 201/400/403/413/timeout/no network,
  and a 9 KB body refused before sending.
- **Acceptance:** `cd apps/desktop/src-tauri && cargo test report_api` ≥ 7; `grep -rn "x-codex-key"
  apps/desktop/src | awk 'END{print NR}'` → 0 (the key never in JS); a build without the key reports
  "unavailable" with the reason. **Precondition:** the operator has created the CI secret (Q-13.2); without it the
  card still lands (the dev-build "unavailable" path is proved), and E7.2's final-acceptance scan checks P-8: a
  missing secret then is an escalation to the operator that pauses closure, never a release with Send disabled
  (`risks-and-open-questions.md` R-1).
- **Depends:** C1. **Tier:** opus.
- **Does not cover:** codex-web's side; the key's extractability (stated risk).

### E5.2 — Anonymous and Handle (item 15)
- **FILES YOU OWN:** `ui/feedback/submissionState.ts`, `submissionUiState.ts`, `browserHandoff.ts`, the bug and
  enhancement composers under `ui/testerWorkbench/feedback/`, tests.
- **Failing test first:** Anonymous on + Handle "Gus" → body's first line is the handle and the API is called;
  off → browser handoff; GitHub unavailable → message, no API call.
- **Acceptance:** the tests; success text "sent for review" (`grep -rn "issue created" apps/desktop/src/feedback
  | awk 'END{print NR}'` → 0). **Depends:** E5.1. **Tier:** opus.

### E5.3 — Update menu (item 14)
- **FILES YOU OWN:** `ui/update/ChannelSelector.tsx`, `CheckPanel.tsx`, `installedPanel.tsx`, `lastCheckPanel.tsx`,
  `Ui.tsx`, `Ui.test.ts`.
- **Failing test first:** the selector renders inside the top block next to `Channel:`.
- **Acceptance:** the test; a screenshot of a long release note at the narrowest window. **Depends:** E5.2.
  **Tier:** sonnet.

### E5.4 — Developer tab and the diagnostic (items 16 + 19)
- **FILES YOU OWN:** `ui/App.tsx`, `ui/settings/SettingsModal.tsx`, `ui/characterHub/LandingScreen.tsx`,
  `CharacterHubPage.tsx`, `CorpusIngestDiagnosticPanel.tsx`, their tests; `desktop/authoring_workbench.rs:480` if
  no reader remains; `apps/desktop/scripts/ui-smoke/spec.json` is NOT owned (E7.1).
- **Failing test first:** the Developer tab renders exactly the kept sections and the diagnostic section.
- **Acceptance:** the test; `grep -rn "corpusIngestDiagnostic\|onCorpusIngestDiagnostic"
  apps/desktop/src --include=*.tsx --include=*.ts | awk 'END{print NR}'` → 0. **Depends:** E5.3, E2.3 (shared
  `CharacterHubPage.tsx`). **Tier:** sonnet.

### E5.MC — E5 merge check
- **Tier:** opus. Screenshots of both report screens and the Developer tab; the four-check audit.

## Epic E6 — campaign folder, House Rules, custom items (items 1, 3, 4)

### E6.1 — relocation (item 1)
- **FILES YOU OWN:** `desktop/campaign_folder.rs`, `desktop/main.rs` (`hub`), `ui/campaign/campaignFolderModel.ts`,
  `campaignMigrationModel.ts`, `EditCampaignScreen.tsx`, `ui/boundary/` campaign file, tests.
- **Failing test first:** relocate a folder with files → all bytes present at the target, skeleton complete, old
  folder intact, known-folders entry updated; a foreign non-empty target refused; same-campaign target opened.
- **Acceptance:** `cd apps/desktop/src-tauri && cargo test relocate_campaign` ≥ 3. **Depends:** C1. **Tier:** opus.

### E6.2 — House Rules menu (item 3)
- **FILES YOU OWN:** `ui/characterHub/HouseRulesDialog.tsx` + model + test (new), `CharacterSheet.tsx` (menu
  entry, `hub`), the Custom dialog (the four flags leave it), `desktop/character_custom.rs`.
- **Failing test first:** a CPR character shows no rules; a PF character shows six; a new character has all off;
  setting one on character A leaves B off.
- **Acceptance:** the tests. **Depends:** E3.MC (sheet lane). **Tier:** opus.

### E6.3 — HP at level-up
- **FILES YOU OWN:** `desktop/character_custom.rs` (method field), the level-up HP path that writes
  `hit_points.json` (`character_hub.rs:526` region, `hub`), level-up UI, tests.
- **Failing test first:** each of the seven methods on a d10 at level 2; Random 50 % floor never below 5;
  regeneration keeps the stored result.
- **Acceptance:** the tests. **Depends:** E6.2. **Tier:** opus.

### E6.4 — Ambidextrous
- **FILES YOU OWN:** the rules_core two-weapon attack path (to verify: `grep -rln "off_hand\|offhand\|two_weapon"
  src/rules_core`), `desktop/character_custom.rs`.
- **Failing test first:** with the flag, the off-hand attack row has no off-hand penalty; without, it does.
- **Acceptance:** the test. **Depends:** E6.3. **Tier:** opus.

### E6.5 — custom item exchange (item 4)
- **FILES YOU OWN:** `desktop/character_custom.rs` (export/import functions), `desktop/campaign_folder.rs`
  (skeleton adds `Custom Items/`), `desktop/main.rs` (`hub`), a dialog + boundary + tests.
- **Failing test first:** export two items, import into another character → two items; import again with an
  edited description → edited, not duplicated; an SF file into a PF character → refused.
- **Acceptance:** the tests. **Depends:** E6.1. **Tier:** opus.

### E6.MC — E6 merge check
- **Tier:** opus.

## Epic E7 — closure (`workflow-instruction-template.md §10/§11`; `decisions.md §26`)

### E7.1 — ui-smoke rows
- **Objective:** rows for gear list, sell window, Market filters/stats, Abilities sub-tabs, House Rules, rest
  text, report screens, Developer tab, relocation; old rows for the removed landing entry rewritten; inventory
  regenerated (`npm run ui-smoke:doc`).
- **FILES YOU OWN:** `apps/desktop/scripts/ui-smoke/spec.json` (sole writer), the generated inventory.
- **Acceptance:** `cd apps/desktop && npm run ui-smoke` green; the row count before/after printed. **Tier:** sonnet.

### E7.2 — final-acceptance scan, retrospective, sweep
- **Objective:** every card `complete` (a filed blocker does not satisfy it; if short, stop — no retro, no sweep,
  no PR); the release-artifact checks of `acceptance-and-verification.md` G-R as part of the scan (widest verify
  in both cargo workspaces + frontend, `packaged_resources`, `no_baked_manifest_dir_in_production`,
  `pytest tools/release`, fresh-install of the .deb and the AppImage, P-8 the CI secret);
  `scripts/retro.py summary --since 2026-11-22 --json` → `docs/retro/sd43-market-inventory-and-abilities-retrospective.md`
  cited from `references/README.md`; every `sd43-*` worktree, branch and `CARGO_TARGET_DIR` removed with counts.
- **Acceptance:** `awk -F'|' '$2 ~ /^ (C|E)[0-9]/ && $5 !~ /complete/' docs/release/SD-43-market-inventory-and-abilities/kanban.md | awk 'END{print NR}'`
  → 0 with E7.2–E7.4 exempted (the exact commands: `acceptance-and-verification.md` G-CL; kanban Status is
  `$5`); `git worktree list | grep -c sd43` → 1 (only `sd43-main`, which E7.3–E7.4 still use; removed after the
  merge). **Tier:** opus.

### E7.3 — release notes, pointer, architecture
- **Objective:** final `release-notes.md` (no interim sentinel); `resolve_release_notes.py` and
  `scripts/tranche/validate-tranche-notes.py` pass; architecture truth-up over the touched topics; the publish
  still signs on `0.43.<run>` (`grep -n "sign" .github/workflows/publish-tester-release.yml`).
- **Tier:** sonnet.

### E7.4 — PR last, merge and watch
- **Objective:** (graphify is out of scope, operator ruling 2026-11-22: "the graph refresh is out of scope"; do not run it) PR `tranche-43 → develop` → when green the lead merges → watch the
  develop publish to green on all three OSes; a red publish is a blocker.
- **Acceptance:** `gh pr view --json state` → `MERGED`; the publish run's conclusion `success`. **Tier:** sonnet.
