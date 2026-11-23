---
canonical: true
owner: operator
bundle_id: SD-43
slug: market-inventory-and-abilities
canonical_branch: tranche-43
target_version: 0.43.0
date: 2026-11-22
status: planning — commands authored, not yet run against a tree that has the change; the lead runs each on the pre-change tree before dispatch and requires a failure (workflow-instruction.md §2.2)
---

# SD-43 Acceptance and Verification

Every card has at least one criterion below with a runnable command or a named manual check. Root-crate filters
run from the repo root; desktop filters from `apps/desktop/src-tauri`; TS tests from `apps/desktop`
(`npx tsx <file>`, one file per run). A cargo filter that matches 0 tests is a FAIL; a receipt quotes the
`N passed` line. "≥ N" means the card's new tests, counted by name in the receipt. Every figure in a receipt
carries its command and denominator. `epic-breakdown.md` holds each card's full text; this file is the gate the
closure scan reads.

## G-W Wired integration and identifiers — every code-bearing card (C1, C2, E0.x and E4.G are exempt: docs, census or operator)

The six greps of `workflow-instruction.md §6` step 5 on the card's added lines → `OK_NO_BUNDLE_TAGS`,
`OK_NO_TOKENS`, `OK_NO_NOOP_HANDLERS`, `OK_NO_MOCK_LEAKS`, `OK_NO_WOULD_STRINGS`, pasted in the receipt
(`receipts.md §2`). For a card touching a `.rs` file also: `cargo clippy --locked --tests -j 8` clean in each
touched workspace and `bash scripts/verify.sh --only pcgen-residue-gate -j 8` → PASS.
**Does not cover:** a stub that uses none of the forbidden words (the MC's adversarial read is the control).

## G-C Package, pointer, graph

| Card | Criterion | Command / check |
|---|---|---|
| C0 | All package files exist; the placeholder gate passes | `find docs/release/SD-43-market-inventory-and-abilities -type f -name '*.md' \| sort` lists the 13 chassis files + `technical-requirements.md`, `technical-design.md`, `kanban.md`, `forward-scope-register.md`, `receipts.md`; `grep -rn '<[a-z_-]*>' docs/release/SD-43-market-inventory-and-abilities/ --include=*.md` → only `workflow-instruction.md §9` tokens |
| C1 | Pointer resolves; version and notes agree; branch on origin | `python3 tools/release/resolve_release_notes.py --repo-root .` exit 0 and prints `docs/release/SD-43-market-inventory-and-abilities/release-notes.md` (exit 2 on 2026-11-22); `python3 -c "import json;d=json.load(open('docs/release/current-release.json'));assert d['tranche']==43,d"`; `grep -c 'SD-43 INTERIM NOTES' docs/release/SD-43-market-inventory-and-abilities/release-notes.md` → 1; `python3 -m pytest tools/release -q` green; `git ls-remote --heads origin tranche-43 \| awk 'END{print NR}'` → 1 |
| C2 | **OUT OF SCOPE (operator ruling 2026-11-22: "the graph refresh is out of scope")** — Incremental graphify ran without `--force` and kept the graph | `artifacts/cycle_0/C2-graphify-receipt.md` holds the command, exit code, node counts before/after (`python3 -c "import json;print(len(json.load(open('graphify-out/graph.json'))['nodes']))"`); after ≥ 90 % of before; `grep -c -- '--force\|graphify update' artifacts/cycle_0/C2-graphify-receipt.md` → 0 outside quoted prohibitions |

## G-E0 Census (read-only; each output reproduces its stated figure or the card stops)

| Card | Criterion | Command / check |
|---|---|---|
| E0.1 (22c) | Every class- or archetype-granted choice enumerated by mechanism | `abilities-census.json` has `class_granted_choices` with per-mechanism counts and a total, produced by two independent implementations that agree; it states explicitly whether choices not modeled as a catalog pool today (e.g. a bloodline or domain picked at level 1) are inside or outside the 1,913-group count of `scripts/census_class_feature_pool_population.py` (open check, intake §22c) and counts each such mechanism separately |
| E0.1 | Seven sub-tabs and every pool censused; PCGen mapping at the pin; data-gap list | `python3 -I docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_0/abilities_census.py --check` exit 0; its JSON has a non-null entry per sub-tab (`python3 -c "import json;d=json.load(open('docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_0/abilities-census.json'));assert all(d['subtabs'].get(k) is not None for k in ['Feats','Racial Abilities','Traits','Archetype','Background','Class Features','Misc'])"`); the Class Features section reproduces 1,913 groups / 16,348 records (`python3 scripts/census_class_feature_pool_population.py`); `git -C "$PCGEN_REPO_DIR" rev-parse HEAD` = `PCGEN_ORACLE_SHA` |
| E0.2 | Market counts reconciled with a reason per dropped row | `cd apps/desktop/src-tauri && cargo test --locked -j 8 --test market_counts -- --nocapture --test-threads=8` green; output prints Magic 3,680, Gear 1,624, Weapons 480 and exact Armor, Shields; shown + dropped-with-reason = source per list; the sum = `built_catalog().entries.len()` |
| E0.3 | Placeholder scan over every kind and surface | `python3 -I docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_0/placeholder_names.py` reproduces 111 of 3,805 (equipment `data.name`) and 201 of 51,474 (all kinds, records with a `data` object); prints a count for `label`/`OUTPUTNAME`, `_settled`, `rules_tables`, `resources/corpus_bundle/` (zero or not) |
| E0.4 | Gear defects reproduced and classified | `artifacts/epic_0/gear-repro.md`: one section per symptom (item 5 × 3 + the `Sack #2` rename, item 7 × 3 drop places, the container sale) with steps, the stored `equipment_sets.json` rows quoted, and `display bug` / `data bug` / `does not reproduce (steps tried)`; screenshots under `artifacts/epic_0/gear-repro/` |
| E0.5 | Rest rules per class/race; House Rules per system | `artifacts/epic_0/rest-and-house-rules.md`: every PF base class has a row or "no rest rule stated" (denominator printed: `ls data/corpus/*/class/*.json \| awk 'END{print NR}'`); every row names its source; a per-system table of the six House Rules for PF / SF / CPR |

## G-E1 Market and inventory (items 5–12)

| Card | Criterion | Command / check |
|---|---|---|
| E1.1 | No `#N`; custom names both panels; a filled container never stacked; put-into-stack splits | `cd apps/desktop && npx tsx src/characterHub/gearScreenModel.test.ts` (the E0.4-derived cases); `grep -n '#\${' apps/desktop/src/characterHub/gearScreenModel.ts \| awk 'END{print NR}'` → 0; if E0.4 classed it a data bug: `cd apps/desktop/src-tauri && cargo test --locked -j 8 split_into_stacked_container -- --test-threads=8` ≥ 1 |
| E1.2 | One column; order at every depth; natural sort; Not Carried excluded from weight | the order test (`Sack #2` before `Sack #10`; non-containers then containers at every depth); the carried-weight test with a Not Carried item (= `encumbrance.totalCarriedWeightLbs`); `npm run typecheck && npm test`; one screenshot per section shape, looked at |
| E1.3 | Whole box drops into the nearest container | model test: a drop on a child row resolves to its container; refusal test for container-into-own-descendant over the widened area |
| E1.4 | Double-click / Enter per ruling 6 | `doubleClickTarget` cases (free slot; both full → swap first `WORN_SLOTS`; worn → Carried; stack of 3 moves 1; slot-less → refused with reason); `grep -c onDoubleClick apps/desktop/src/characterHub/GearScreen.tsx` ≥ 1; a row-button double-click does not fire; Enter calls the same function |
| E1.5 | Selling a container moves direct contents to Carried | `cd apps/desktop/src-tauri && cargo test --locked -j 8 sell_container -- --test-threads=8` ≥ 6 (empty, loose, nested, inside a Bag of Holding, worn belt pouch + encumbrance, Not Carried contents) |
| E1.6 | Sell window | `npx tsx src/characterHub/sellDialogModel.test.ts` (options exactly `Market Rate`, `Build`, `Negotiate`; default `Market Rate`; stack scales; null price → blank); `grep -rn "pricing above\|Normal" apps/desktop/src/characterHub/GearScreen.tsx apps/desktop/src/characterHub/SellDialog.tsx \| awk 'END{print NR}'` → 0; `git diff --stat origin/develop -- apps/desktop/src/characterHub/priceMode.ts` empty |
| E1.7 | One `MarketButton` per panel; no panel Print; no pricing control on Gear/Weapons; ☰ Print still opens `PrintDialog` | the MarketButton test (three PF panels, one button each, no `PriceModeControl`); `grep -n "props.onPrint\|PriceModeControl" apps/desktop/src/characterHub/CharacterSheet.tsx \| awk 'END{print NR}'` → 0 or 1 (the import, stated); `grep -c PriceModeControl apps/desktop/src/characterHub/MarketDialog.tsx` ≥ 1; the receipt states old and new height and font size; SF and CPR sheets render the same button (test) |
| E1.8 | Sell value column | MarketDialog test: 3-stack under `standard` → 50 % total; `cashless` → `—`; `grep -c sellNegotiateGp apps/desktop/src/characterHub/MarketDialog.tsx` ≥ 2 |
| E1.9 | Engine-stated stats in the Market box | Rust + TS tests: bastard sword damage and critical; full plate armor bonus, max Dex, check penalty, spell failure; missing stat → `—`; the sampling test prints per stat the count of `—` over every category and `TYPE` shape (table pasted); `GearDetail` tests unchanged and green |
| E1.10 | Filter model covers every row | `npx tsx src/characterHub/marketFilters.test.ts`: per list and column, value counts + `Other` ≥ E0.2's row count, every row reachable through `All`; a piercing-and-slashing weapon under both |
| E1.11 | No cap; windowed list | `grep -rn MARKET_ROW_LIMIT apps/desktop/src \| awk 'END{print NR}'` → 0; `marketShownRows` returns every row; keyboard selection, description, sort, Only Show Affordable, `+N` tests green; first paint on Magic items stated "on this machine" |
| E1.MC | Epic verified at the widest scope | `cd apps/desktop && npm run typecheck && npm test`; `cd apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8`; root `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8`; every `test result: FAILED` attributed to its `Running` line; screenshots of gear (every section), Market (each list, a filter), sell window — looked at; G-W over the epic |

## G-E2 Sheet layout, campaign button, rest text (items 17, 21, 18, 2)

| Card | Criterion | Command / check |
|---|---|---|
| E2.1 | Fixed bar, rail, right column; centre scrolls; `100dvh`; Show Progression visible | the SheetLayout test (root `100dvh`, no page scroll, centre the only main scroller, side columns scroll per Q-17.1); screenshots at full height, at −40 px and at the smallest window with Show Progression fully visible, looked at |
| E2.2 | SF and CPR on the same layout | test: each sheet renders inside `SheetLayout`; screenshots of both; `grep -n AppMenu apps/desktop/src/characterHub/StarfinderCharacterSheet.tsx apps/desktop/src/characterHub/CyberpunkCharacterSheet.tsx` recorded (settles Q-8.2) |
| E2.3 | Open campaign opens the folder | test: Open with a real membership row passes an absolute path (fails today: the UUID); one full-width banner per membership; the Manager banner's height before/after in the receipt; the receipt names which test exercised the handler and with what argument |
| E2.4 | Cited rest table; one engine command; print block after the bio page; override | Wizard 5 / Elf prints its rows under the bio page; with the override set the override prints instead; every table row has a citation (a test); **packaged resources:** the table's path in `apps/desktop/src-tauri/tauri.conf.json` `bundle.resources` and asserted in `tests/packaged_resources.rs` — `cd apps/desktop/src-tauri && cargo test --locked -j 8 --test packaged_resources -- --test-threads=8` green; root `cargo test --locked -j 8 --test no_baked_manifest_dir_in_production` green |
| E2.5 | Rest text read-only under the Biography text | test: after a blur, saved `bio.biography` does not contain the rest text; the rest text renders below the field in print order |
| E2.MC | Epic verified | widest scope as E1.MC; screenshots of all three sheets at a short window; the PDF bio page with the rest block (`pdftotext` excerpt pasted); G-W over the epic |

## G-E3 The Abilities tab (items 22, 22a, 22b)

| Card | Criterion | Command / check |
|---|---|---|
| E3.1 | The Feats tab renamed to Abilities (operator ruling Q-22.3, 2026-11-22: rename the Feats tab to Abilities; Feats is one of its seven sub-tabs) | `npx tsx src/characterHub/AbilitiesTab.test.ts`: seven sub-tab labels in PCGen's order (Feats, Racial Abilities, Traits, Archetype, Background, Class Features, Misc); the Feats sub-tab renders the feat budget line; `SHEET_TABS` holds `Abilities` and no `Feats` entry (test); the existing FeatsTab tests unchanged and green |
| E3.2 | Generic budget module, saved selections, commands; Traits | root `cargo test --locked -j 8 --lib ability_budget -- --test-threads=8` ≥ 1 (two traits at level 1 → `remaining 0`; a third refused naming the pool; two of one category refused naming the category); `cd apps/desktop/src-tauri && cargo test --locked -j 8 --test old_save_loads_unchanged -- --test-threads=8` green with the new case |
| E3.3 | Generic sub-tab UI renders rows only | `abilitySubTabModel` tests: `granted/spent/remaining` from rows; no number computed from anything but rows; `npm run typecheck && npm test` |
| E3.4 | Grants per level for every catalog pool | tests: Rogue 6 → Rogue Talent granted 3; Barbarian 4 → Rage Power 2; Ninja 2 → Ninja Trick 1 (expected values transcribed from the book text, not the engine's read path); a census test: every catalog pool with a class owner yields a grant row or is in E0.1's data-gap list |
| E3.5 | Pool-member prerequisites in the engine | test: Silent Kill without Assassinate → refused, reason names Assassinate; coverage count printed: evaluated / unknown token shape over every pool member (unknown = 0 by E3.MC, or a discovered card per shape) |
| E3.6 | Racial Abilities alternates | tests: a Human alternate replacing the bonus feat; a second alternate replacing it refused; the feat budget's racial pool follows |
| E3.7 | Archetype | test: Rogue with two archetypes both replacing Trapfinding → refused, reason names the slot |
| E3.8 | Favored class bonus | tests: Fighter 3 favored Fighter, HP at 1–3 → HP total +3; skill at level 2 → skill pool +1; an imported character with `favored_hit_points` grants is not counted twice |
| E3.9 | Background and Misc | each sub-tab renders engine rows or the data-gap sentence matching E0.1's list (test written from E0.1's mapping; the receipt states it) |
| E3.10 | Overrides | test: flag on → unqualified pick admitted and flagged; off → refused; `custom.json` without the flags loads with them off |
| E3.11 | Printout | `cd apps/desktop/src-tauri && cargo test --locked -j 8 print_pf_blocks_abilities -- --test-threads=8` ≥ 1 (Rogue 6, three talents, a favored class, each under its heading) |
| E3.MC (22c) | **Every class- or archetype-granted choice is reachable** (intake §22c, 2026-11-22: ninja tricks and rage powers are examples only; the scope is every choice a class or archetype grants — talents, tricks, powers, discoveries, hexes, arcana, revelations, mercies, bloodlines, domains, schools, mysteries, orders and every other shape) | E0.1's by-mechanism table (`artifacts/epic_0/abilities-census.json` `class_granted_choices`: one row per mechanism — catalog pool group, level-1 pick such as bloodline/domain/school/mystery/order, archetype-granted pool, other — with its record count, summing to E0.1's total; two implementations agree). E3.MC re-counts on the built app/engine: per mechanism, **reachable** (selectable in the Class Features sub-tab **and** recorded on the character after reload) + **discovered card** (id in `kanban.md`) = the mechanism's E0.1 total; the receipt pastes the table. **"Not reachable" is a number to close, never an exemption:** E3.MC cannot be `complete` while any mechanism's unreachable count is non-zero and not covered by a discovered card, and every such discovered card is itself `complete` before E7.2 (G-CL). Rogue Talent / Rage Power / Ninja Trick tests (E3.4) prove three mechanisms only and do not stand in for the count |
| E3.MC | Every sub-tab works on three seeds; no engine-gap notice; data-gap notices = E0.1 | screenshots on a Rogue/Ninja, a Barbarian, a Human Fighter with an archetype, looked at; `grep -rn "engine card" apps/desktop/src/characterHub/AbilitiesTab.tsx \| awk 'END{print NR}'` → 0; on-screen data-gap notices counted in the built app = E0.1's data-gap list length (both numbers in the receipt); the E3.5 unknown-shape count = 0; widest scope as E1.MC; G-W |

## G-E4 Item names (item 20)

| Card | Criterion | Command / check |
|---|---|---|
| E4.1 | One display-name function; the all-kinds test; the proposal | the all-kinds test (`[`, `]`, `NAME` in any display name of any item kind) fails today on 201 hits and, after E4.1, only on the hits `artifacts/epic_4/proposed-names.md` covers; every proposed name cites its book text |
| E4.G | Operator approval (manual) | the operator's reply (approve / edits, and the confirmation of the reading of "1", Q-20.1) quoted with its date in `progress.md ## Operator replies` |
| E4.2 | Approved names ship | the all-kinds test green; if the table is read at runtime: `bundle.resources` + `packaged_resources.rs` assertion and `cargo test --test packaged_resources` green; Market screenshot shows `Hill Giant-hide`; E4 epic report in `progress.md` |

## G-E5 Reports, Update menu, Developer tab (items 13, 15, 14, 16, 19)

| Card | Criterion | Command / check |
|---|---|---|
| E5.1 | Rust report client; every reply handled; key never in JS | `cd apps/desktop/src-tauri && cargo test --locked -j 8 report_api -- --test-threads=8` ≥ 7 (201, 400, 403, 413, timeout, no network, 9 KB body refused before sending); `grep -rn "x-codex-key" apps/desktop/src \| awk 'END{print NR}'` → 0; a build without the key reports "unavailable" with the reason (test) |
| E5.2 | Anonymous + Handle; no silent fallback | tests: Anonymous on + Handle "Gus" → body's first line is the handle and the API is called; off → browser handoff; GitHub unavailable → message, no API call; `grep -rn "issue created" apps/desktop/src/feedback \| awk 'END{print NR}'` → 0 |
| E5.3 | Update menu layout | `npx tsx src/update/Ui.test.ts`: the selector renders in the top block next to `Channel:`; screenshot of a long release note at the narrowest window, word-boundary wrap, looked at |
| E5.4 | Developer tab slimmed; diagnostic embedded; landing entry gone | test: the Developer tab renders exactly the kept sections (ruling 16) and the diagnostic section; `grep -rn "corpusIngestDiagnostic\|onCorpusIngestDiagnostic" apps/desktop/src --include=*.tsx --include=*.ts \| awk 'END{print NR}'` → 0 |
| E5.MC | Epic verified | widest scope as E1.MC; screenshots of both report screens and the Developer tab, looked at; G-W |

## G-E6 Campaign folder, House Rules, custom items (items 1, 3, 4)

| Card | Criterion | Command / check |
|---|---|---|
| E6.1 | Relocation carries content, writes the skeleton, keeps the old folder | `cd apps/desktop/src-tauri && cargo test --locked -j 8 relocate_campaign -- --test-threads=8` ≥ 3 (all bytes at the target + skeleton + old intact + known-folders updated; foreign non-empty target refused; same-campaign target opened) |
| E6.2 | House Rules menu per character and system | tests: a CPR character shows none; a PF character shows six (SF per E0.5); a new character all off; A's setting leaves B off; the four flags no longer in the Custom dialog |
| E6.3 | HP at level-up, seven methods | tests: each method on a d10 at level 2; Random 50 % floor never below 5; regeneration keeps the stored result; level 1 stays maximum |
| E6.4 | Ambidextrous | test: flag on → off-hand attack row has no off-hand penalty; off → it does |
| E6.5 | Custom item exchange | tests: export two, import into another character → two; import again with an edited description → edited, not duplicated; SF file into a PF character → refused |
| E6.MC | Epic verified | widest scope as E1.MC; screenshots of relocation, House Rules (PF, SF, CPR), export/import, looked at; G-W |

## G-E7 ui-smoke

| Card | Criterion | Command / check |
|---|---|---|
| E7.1 | Rows for every changed screen; old rows rewritten | `cd apps/desktop && npm run ui-smoke` green; row count before/after printed (`python3 -I -c "import json;print(len(json.load(open('apps/desktop/scripts/ui-smoke/spec.json'))['rows']))"`); `npm run ui-smoke:doc` regenerated and `git status --porcelain docs/testing/ui-smoke-inventory.md` empty after the commit |

## G-R Release artifacts (template §6 step 1a) — E7.2 runs it before the PR; E7.4 re-checks after the publish

```bash
# Version surfaces read 0.43.0 (the build is stamped 0.43.<run> at publish)
node -p "require('./apps/desktop/package.json').version"                                             # 0.43.0
python3 -c "import json;print(json.load(open('apps/desktop/src-tauri/tauri.conf.json'))['version'])"  # 0.43.0
awk '/^name = "codex-desktop"/{f=1} f&&/^version/{print;exit}' apps/desktop/src-tauri/Cargo.lock       # version = "0.43.0"
grep -cF 'VERSION="0.43.${GITHUB_RUN_NUMBER}"' .github/workflows/publish-tester-release.yml          # 1 (line 129; -F: a plain grep reads ${ as an anchor and prints 0)
grep -c 'assert_version_stamp.py' .github/workflows/publish-tester-release.yml                       # 3 (one per platform job: lines 323, 580, 667)
grep -c 'verify_release_consistency.py' .github/workflows/publish-tester-release.yml                 # 2 (lines 503, 969)
# Pointer and notes move together
python3 tools/release/resolve_release_notes.py --repo-root .                                         # prints .../SD-43-market-inventory-and-abilities/release-notes.md
python3 scripts/tranche/validate-tranche-notes.py --release-notes-path docs/release/SD-43-market-inventory-and-abilities/release-notes.md
grep -c 'SD-43 INTERIM NOTES' docs/release/SD-43-market-inventory-and-abilities/release-notes.md     # 0 at E7.3+
python3 -m pytest tools/release -q                                                                   # green
# Packaged resources (every new runtime read: the rest table E2.4; the name table E4.2 if read at runtime)
cd apps/desktop/src-tauri && cargo test --locked -j 8 --test packaged_resources -- --test-threads=8   # green, with the new assertions named in the receipt
cargo test --locked -j 8 --test no_baked_manifest_dir_in_production -- --test-threads=8               # root: green
# The publish still signs on 0.43.<run> (signed manifest, PR #419 — not SD-43 work, re-checked only)
grep -n 'UPDATE_SIGNING_KEY' .github/workflows/publish-tester-release.yml | awk 'END{print NR}'        # ≥ 1 (2 on 2026-11-22)
# The report key reaches the release build (P-8; E5.1)
gh secret list -R electricm0nk/codex | awk '$1=="CODEX_REPORT_KEY"' | awk 'END{print NR}'             # 1 (created 2026-11-22)
```

**Fresh-install acceptance (manual check, recorded in E7.2's receipt, completed in E7.4's).**

1. **E7.2, before the PR:** build the Linux artifacts locally from `origin/tranche-43` in the `main` lane tree with
   the version stamped `0.43.1` in a scratch copy (never committed) and
   `python3 tools/release/assert_version_stamp.py --repo-root <tree> --version 0.43.1` passing first:
   `cd apps/desktop && npx tauri build --config src-tauri/tauri.sidecar.json --bundles deb,appimage --ci`.
   Install **the .deb** (`sudo dpkg -i`, or `dpkg-deb -x` into a scratch prefix if the box refuses root — the
   receipt says which and that the extract form does not prove the system install path) **and the AppImage**, each
   on a clean profile (fresh `HOME`/`XDG_*`, no `installed-state.json`). Launch under the run-desktop harness, open
   Settings → Update, press Check. **Nothing reads "unknown"**: the Installed panel shows the stamped version, the
   source commit and the artifact hash; release notes load. Screenshots per artifact, looked at.
2. **E7.4, after the develop publish is green:** relaunch both installs and press Check again: the alpha
   `0.43.<run>` is offered and **Install is enabled**; its notes load. Screenshots per artifact.
   `gh release list -L 1` names `alpha v0.43.<run>`.

If the box cannot run either step, that is a blocker escalated to the operator (SD-42's move of this check to the
post-merge alpha was an operator ruling; this run may not take it on its own authority).
**Does not cover:** macOS and Windows installers; an upgrade from an installed 0.22.x build (the check is first
launch only); the self-update download itself.

## G-CL Closure — E7.2 (scan), E7.3, E7.4

The scan **cannot pass** with any card short of `complete` or with any open blocker:

```bash
cd docs/release/SD-43-market-inventory-and-abilities
test -s kanban.md || { echo NO_KANBAN; exit 2; }
awk -F'|' '$2 ~ /^ (C|E)[0-9]/ { n++ } END { if (n != 65) print "ROW_COUNT " n }' kanban.md        # prints nothing (56 at authoring, 57 after E0.1 discovered E3.12, 58 after E0.4 discovered E1.12, 59 after E1.6 discovered E1.13, 61 after E1.MC discovered E1.14 and E1.15, 62 after E2.MC discovered E2.6, 64 after E3.4 discovered E3.13 and E3.14, 65 after E3.5 discovered E3.15; a discovered card updates this number, kanban.md and epic-breakdown.md §0 in one commit)
awk -F'|' '$2 ~ /^ (C|E)[0-9]/ && $2 !~ /^ E7\.[2-4] / && $5 !~ /^ complete *$/ { print $2 "|" $5 }' kanban.md   # prints nothing (E7.2's own scan; E7.4 re-runs it with the exemption narrowed to E7\.4)
awk '/^## Open blockers/{f=1;next} /^## /{f=0} f && /^### /' progress.md | awk 'END{print NR}'        # 0 — a filed blocker is an open card, not a pass
grep -c '^- E4\.G (20[0-9-]*): ' progress.md                                                         # ≥ 1: the operator's E4.G reply quoted under ## Operator replies
grep -L 'closure check' artifacts/epic_*/*_cycle_receipt.md                                          # prints nothing
grep -L 'Does not cover' artifacts/epic_*/*_cycle_receipt.md                                         # prints nothing
grep -L 'Looked at' artifacts/epic_*/E*.MC_cycle_receipt.md                                          # prints nothing (every MC looked)
test -f ../../retro/sd43-market-inventory-and-abilities-retrospective.md && grep -c 'sd43-market-inventory-and-abilities-retrospective' references/README.md   # ≥ 1 (written AND cited, same commit)
git worktree list | grep -c 'sd43-' | awk '{print ($1<=1)?"SWEPT":"NOT_SWEPT " $1}'                  # SWEPT (only sd43-main until E7.4 ends)
```

Then G-R in full. **Every acceptance row in this file holds** (the scan re-runs each card's command on the final
tree, or cites the receipt where the check is manual), and every revisit command in
`forward-scope-register.md §1` is run and its output pasted. If anything is short, E7.2 stops: no retrospective,
no sweep, **no PR**; it reports what is short with the command that shows it.

**E7.3:** G-R's notes lines; the architecture truth-up receipt (and the hand-mapped one if the script maps 0 docs)
appended to `receipts.md`; the claims critic's refuted count in the receipt.
**E7.4:** (graphify out of scope, operator ruling 2026-11-22: "the graph refresh is out of scope"; no graphify receipt) `gh pr view <n> --json state` →
`MERGED`; `gh run list -w publish-tester-release.yml -b develop -L 1 --json conclusion` → `success` on all three
OS jobs; fresh-install step 2.

## Traceability — intake item → card(s) → gate

| Intake item (`../SD-43-intake/intake.md`) | Card(s) | Gate |
|---|---|---|
| 0 post-SD-42 cleanup (graphify step) | C2; E7.4 | G-C C2; G-CL |
| 1 campaign folder relocation | E6.1 | G-E6 E6.1 |
| 2 rest and recovery text (print + Biography + override) | E0.5, E2.4, E2.5 | G-E0, G-E2, G-R packaged resources |
| 3 House Rules menu | E0.5, E6.2, E6.3, E6.4 | G-E6 |
| 4 custom items export/import | E6.5 | G-E6 |
| 5 one-column gear list; container contents | E0.4, E1.1, E1.2 | G-E0, G-E1 |
| 6 double-click / Enter | E1.4 | G-E1 |
| 7 whole-box drop target | E1.3 | G-E1 |
| 8 Market button; Print and pricing leave the panels | E1.7 | G-E1 |
| 9 sell window | E1.6 | G-E1 |
| 10 stats in the Market | E1.9 | G-E1 |
| 11 filters, no cap, counts | E0.2, E1.10, E1.11 | G-E0, G-E1 |
| 12 sell values; container sale | E1.5, E1.8 | G-E1 |
| 13 report API | E5.1 | G-E5; G-R P-8 |
| 14 Update menu | E5.3 | G-E5 |
| 15 Anonymous + Handle | E5.2 | G-E5 |
| 16 Developer tab | E5.4 | G-E5 |
| 17 fixed layout, all systems | E2.1, E2.2 | G-E2 |
| 18 Open campaign button | E2.3 | G-E2 |
| 19 Corpus Ingest Diagnostic | E5.4 | G-E5 |
| 20 placeholder names; one all-kinds test; approval | E0.3, E4.1, E4.G, E4.2 | G-E0, G-E4 |
| 21 Show Progression cut off | E2.1 | G-E2 |
| 22 / 22a / 22b Abilities tab, seven sub-tabs, grants and prereqs enforced, printed | E0.1, E3.1–E3.11, E3.MC | G-E0, G-E3 |
| 22c every class- or archetype-granted choice, not the examples (ruling 2026-11-22, `c9994e01bd`) | E0.1 (by-mechanism count), E3.4–E3.9, E3.MC (+ discovered cards) | G-E0 E0.1 (22c), G-E3 E3.MC (22c) |
| 23 minor = SD number; pointer | C1; E7.3 | G-C, G-R |
| Signed-manifest brief | none (merged outside SD-43, PR #419) | G-R signing re-check only |

## Artifact map

| Gate | Receipt(s) | Evidence path |
|---|---|---|
| G-C | C0, C1, C2 | `artifacts/cycle_0/` |
| G-E0 | E0.1–E0.5 | `artifacts/epic_0/` (scripts, JSON, `gear-repro/`) |
| G-E1 … G-E6 | each card; each MC | `artifacts/epic_<n>/<card-id>_cycle_receipt.md`; screenshots under `artifacts/epic_<n>/<card-id>/` |
| G-E4 E4.G | E4.G | `progress.md ## Operator replies`; `artifacts/epic_4/proposed-names.md` |
| G-R, G-CL | E7.2, E7.3, E7.4 | `artifacts/epic_7/`, `receipts.md`, `docs/retro/sd43-market-inventory-and-abilities-retrospective.md` |

## Does not cover

Pixel-exact layout; drag in ui-smoke (the harness has no drag op — drop targets are proved by model tests and
screenshots); macOS and Windows installers; Starfinder and Cyberpunk RED abilities (`scope-draft.md §5`); whether
each pool member's prose is correct (the corpus is the source); codex-web's side of the report API; the key's
extractability from a shipped app (a stated risk, not a check); retraining and replace-a-pick
(`forward-scope-register.md`).
