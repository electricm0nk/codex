---
canonical: true
owner: operator
bundle_id: SD-39
slug: ui-print-corrections
canonical_branch: tranche-19
target_version: 0.19.0
date: 2026-11-01
status: planning-ready — R1–R16 are operator answers given 2026-11-01; "DEFAULT" rows are the author's and name how to revisit
---

# SD-39 Decisions

Every entry says who decided (OPERATOR ruling, verbatim where it matters, or author DEFAULT), what
enforces it, and how to revisit it. "R<n>" below is a grill ruling; `workflow-instruction.md §12` uses
"R<n>" for standing rules, so the two are told apart by file.

## §1 — Package location

**DEFAULT.** `docs/release/SD-39-ui-print-corrections/`. The overlay names `docs/stc/stc-<##>-<description>/`
as the chassis path; this repo has no `docs/stc/` (`ls docs/stc` → no such directory, 2026-11-01), every
sibling bundle lives under `docs/release/`, and `scripts/architecture_truth_up.py:54` treats `^docs/release/`
as architecture-relevant. SD-38 recorded the same deviation (`../SD-38-cyberpunk-red/decisions.md §1`).
**Enforced by:** C0. **Revisit:** the operator says "move to docs/stc"; one `git mv` before C1.

## §2 — Branch, board, version

**DEFAULT.** Branch `tranche-19`, cut by C1 from `origin/develop` @ `2b14020bc7` (PR #402 merge). Board:
`./kanban.md` + `./progress.md`. Version `0.19.0` stamped by C1's **single commit** on the 14 surfaces
the `0\.18\.` grep names on 2026-11-01 (F-1 in `content-unit-inventory.md`; 14 files, same set SD-38's
C1 bumped) plus `docs/release/current-release.json` → `{"tranche": 19, "notes_path":
"docs/release/SD-39-ui-print-corrections/release-notes.md"}`. Root `Cargo.toml` stays `0.1.0`.
`release-notes.md` ships an interim body with the sentinel `SD-39 INTERIM NOTES` and neither resolver
marker; E9.8 replaces it; E9.11 refuses the PR while the sentinel is present.
**Enforced by:** C1's receipt (A/B grep, 15 files touched); `python3 tools/release/resolve_release_notes.py --repo-root .` exit 0.
**Revisit:** the tranche digit moves only at a new bare cut.

## §3 — Operator rulings on record (2026-11-01)

The grill transcript is `artifacts/cycle_0/defects-and-rulings-2026-11-01.md`. The rulings:

| R | Question | Operator's choice |
|---|---|---|
| R1 | Print engine | **Rust generates the PDF from a per-system template.** Not browser print, not PDF forms |
| R2 | Template shape | **Shared page frame + shared blocks; each system has a short layout file** |
| R3 | Paper size | **App setting, default A4, US Letter option; print dialog shows it and allows a one-off override** |
| R4 | Print modes | **Stacking checkboxes: Spellbook, Show the math, Bio and campaign notes.** basic = none, full = all |
| R5 | Output | **Two buttons: Save PDF and Print** (operator chose 3 over the recommended single Save) |
| R6 | Print proof | **Golden text-extraction tests + one operator eyeball per system as a closure gate** |
| R7 | Flyover kind | **Left side drawer; close by X, click-outside, Escape** |
| R8 | Level ledger | **Stamp the level on every new choice; one-time migration by slot order, rows marked "assigned on upgrade"** |
| R9 | Over-budget legacy saves | **Keep every feat; red "over budget by N" badge; block new adds; never auto-delete** |
| R10 | Hide-unmet checkbox | **Default ticked; unticked shows unmet feats greyed with the missing prerequisite; Add disabled** |
| R10b | Overrides | **Add an Overrides group to the Custom tab**: feats without prerequisites, weapons without proficiency, spells outside class list/level, armor without proficiency, equipment ignoring slot limits (operator's five); author extras in §10 |
| R11 | Market money | **Keep the three price modes (cashless / buy 100% sell 50% / character build) and make Market honour them** |
| R12 | Slot conflicts | **Hard block ("head slot is taken by X"); item stays carried; the slot override allows and tags** |
| R13 | `.pcg` import | **Tool-side converter in `crates/codex-ingest`, shipped as a sidecar; the live app never parses `.pcg`** |
| R14 | Magic variants | **Generate +1…+5 at runtime from base item + enhancement table; special abilities and materials later** |
| R15 | System scope | **PF1e gets everything; SF1e and CPR get print + layout + Market shell** |
| R16 | Epic order | **UI first, print last** (operator chose 2 over the recommended print-first) |
| R17 | Campaign notes (no such field exists) | **Use what exists:** the character bio + the campaign's description. No new field |
| R18 | A newly bought item | **Worn if its slot is free**; else carried, with the reason shown; no-slot items carried |
| R19 | The Print button | **Makes the PDF and opens it in the system viewer**; you print from there |
| R20 | Merged worktree `fix-perform-install` | **Removed** at launch, with its local branch |

## §4 — Print pipeline (R1, R2, R5, R6)

**OPERATOR R1/R2/R5/R6; DEFAULT on the crate.** A Rust module in `apps/desktop/src-tauri/src/print/`
takes the composed sheet (the same `LoadSavedCharacterResponse` the screen renders, plus the engine
`explanations` rows) and a `PrintRequest { paper, spellbook, math, bio }`, builds a Typst document from
a shared frame and shared blocks, and writes a PDF. **Typst as a library** (`typst`, `typst-pdf` crates,
latest stable at E8.1; versions recorded in E8.1's receipt and `technical-design.md §1`), with OFL fonts
bundled under `apps/desktop/src-tauri/resources/fonts/` and listed in `tauri.conf.json bundle.resources`
+ `tests/packaged_resources.rs`. Save PDF = save-as dialog (`tauri-plugin-dialog`) then open with
`tauri-plugin-opener`; Print = same bytes to a temp file, opened in the system PDF viewer (R19). The current CSS print path
(`apps/desktop/src/characterHub/printLayout.ts`, `CharacterSheet.tsx:2854 window.print()`) is removed by E8.4.
**Enforced by:** G-9, G-10; E8.MC. **Revisit:** if `typst` cannot be built offline/CI, E8.1 returns
`blocked-escalated` naming the alternative (`printpdf` with hand layout) and its cost.

## §5 — One converter re-run (DEFAULT, batch-big rule)

Three schema additions need PCGen data: feat `stackable` (PCGen `MULT:YES`), equipment `slot` (PCGen
`SLOTS` + the EQSLOTS vocabulary), and the enhancement-eligibility flags for armor/shield/weapon
(derivable from `TYPE`). The converter already parses `SLOTS`
(`crates/codex-ingest/src/pcgen_import/lst_parser/equipment.rs:87`) and drops it at
`sheet_rule/convert.rs:1936,2557`. **Card E1.0 adds all three fields and runs the converter once**
(`cargo run -p codex-ingest --bin sheet_rule_convert`), first after C1, so no later card re-runs it.
**Enforced by:** E1.0; the `--check` runs in G-1. **Revisit:** a fourth field discovered later is a
discovered card that re-runs the converter once more, never a hand edit of `data/sheet_rules/`.

## §6 — Feat guards (R9, R10)

**OPERATOR.** Budget = the engine's own count of feat slots by pool (level feats at 1, 3, 5 …; racial
bonus; class bonus pools such as Fighter combat feats), exposed as explanations rows
`feats.budget.<pool>.granted|spent|remaining`. `add_feat_selection` refuses at `remaining == 0` for the
pool the feat would consume unless the "ignore feat budget" override is on. Duplicate = same `feat_id`
(and same `target` when the feat takes one) unless `repeatable` (§25). Hide-unmet default ticked;
`FeatPrerequisiteReport` (`apps/desktop/src-tauri/src/feat_catalog.rs:33–69`) already supplies the unmet
list. Legacy over-budget: keep, badge, block adds. **Enforced by:** E1.1–E1.3, G-3. **Revisit:** the
operator may ask for a soft warning instead of a block: one flag in E1.1.

## §7 — Layout (R7 + author calls)

**OPERATOR R7; D2, D8, D9 verbatim.** Target Pathfinder sheet: left rail = portrait, Level/XP, Class,
**Character Details** (moved from the right column), **Show Progression** button; middle = identity,
then one row holding the 2x2 (HP / Initiative / Encumbrance / Conditions) beside a 1x4 (Armor Class /
Attack / Speed / Saving Throws); right column = **Abilities** (moved from the middle) then Skills; tabs
unchanged below. **DEFAULT:** below a breakpoint the 1x4 wraps under the 2x2; the "what the next level
grants" text moves into the drawer. All moves are entries in `sheetLayout.ts` (the registry
tranche-17-ui introduced: "Moving a box is an edit to its entry here, not a JSX move"), extended to
register the left rail and right column. **Enforced by:** E2.1–E2.3, G-4. **Revisit:** edit the registry.

## §8 — Market (R11, R14, D3)

**OPERATOR.** One `Market` button on the Weapons, Defense and Gear tabs (and SF/CPR equivalents)
replaces Add Weapon / Add Armor / Add Gear. Root cause of D3: `buildItemPickerConfig`
(`CharacterSheet.tsx:255–300`) loads the whole `ArmsArmor` category for both pickers and only sets
`initialGroup`. Market filters server-side by a new `MarketCategory` (Weapon / Armor / Shield / Gear /
Magic item) derived from the record's type tags. PF variants: `+N` picker (N = 1…5) on armor, shields and
weapons; price = base + masterwork (150 gp armor/shield, 300 gp weapon) + N² × 1000 gp (armor/shield)
or N² × 2000 gp (weapon); the generated item's id is `<base_id>+<N>`; AC/attack/damage consume the
enhancement bonus (bonus type `enhancement`, so it does not stack with itself). Price modes:
`PriceMode` (`character_hub.rs:4397`) already exists; buy deducts in paid modes and refuses when
`gold < price`; sell credits 50% (Standard) / 100% (CharacterBuild) / 0 (Cashless). Currency label per
system from the adapter (gp / credits / eddies). **Enforced by:** E3.1–E3.5, G-5. **Revisit:** the
operator may add special abilities (DEF-1) in a later bundle.

## §9 — Body slots and worn vs carried (R12, D5)

**OPERATOR.** `BodySlot` enum (armor, shield, belt, body, chest, eyes, feet, hands, head, headband,
neck, ring (×2), shoulders, wrist, wielded main / off / two-handed, none) converted from PCGen's EQSLOTS
by E1.0. Worn/wielded = `ActiveState::EquippedActive`; carried = `ActiveState::SelectedInactive`
(`src/rules_core/character_input.rs:163–170`, already in the engine; the UI sends `EquippedActive` for
everything today). Equip refuses when the slot is held (rings allow two) with "head slot is taken by
<item>"; the slot override allows and tags. Only `EquippedActive` items feed AC, attack, saves, ability
and skill bonuses; weight counts for both states (encumbrance). **Enforced by:** E4.1–E4.3, G-6.
**Revisit:** swap-on-equip (option 3 in the grill) is one flag in E4.1 if the operator changes their mind.

## §10 — Overrides (R10b + author extras)

**OPERATOR five + DEFAULT three.** `CharacterCustomDto` gains `overrides: Overrides` with eight booleans:
`feats_without_prerequisites`, `weapons_without_proficiency`, `spells_outside_class_list`,
`armor_without_proficiency`, `ignore_slot_limits` (operator), `ignore_feat_budget`,
`ignore_alignment_restrictions`, `ignore_encumbrance` (author). Each command that enforces the matching
rule reads the flag. **Every item admitted under an override is tagged**: the selection record carries
`override: Some(<flag>)`, the UI shows a badge, and the "full" print marks it ("admitted without
prerequisites"). The sheet never lies silently. **Enforced by:** E6.1, E6.2, G-7. **Revisit:** drop an
extra by deleting its toggle; the tag rule stays.

## §11 — Level ledger (R8, D9)

**OPERATOR.** Feat, spell, skill-rank and ability-increase selections gain `taken_at_level: Option<u8>`
and `assigned_on_upgrade: bool`. New adds stamp the current character level. Migration (E5.2) runs once
per legacy save on load: feats fill level-feat slots in saved order (1, 3, 5 …; racial/class bonus pools
at their grant level), spells fill spells-known slots per class level, ranks fill per-level rank budgets,
ability increases at 4, 8, 12 …; every migrated row sets `assigned_on_upgrade = true` and the drawer shows
it. The drawer (E5.3) lists per level: class level taken, HP (if recorded), feats with targets, spells
learned, ranks spent, class features gained, ability increase, and unspent slots. Read-only in SD-39
(DEF-6). **Enforced by:** E5.1–E5.3, G-8.

## §12 — `.pcg` import (R13, D12)

**OPERATOR.** `crates/codex-ingest/src/pcg_import/` reads PCGen 6.x `.pcg` (line-based `KEY:value`)
into Codex's saved-character JSON; a binary `pcg_import` is built and shipped as a Tauri sidecar
(`tauri.conf.json bundle.externalBin`). The Import button offers "Codex file | PCGen file"; for PCGen it
runs the sidecar and imports the result like a Codex file. Unknown records become custom items; the
import report lists every mapping and every unknown. Fixtures: `scripts/oracle_harness/eqm-fixtures/*.pcg`
and any `.pcg` under `$PCGEN_REPO_DIR` the card finds (count recorded). The `pcgen-residue-gate` stage
keeps the live crates clean. **Enforced by:** E7.1, E7.2, G-11.

## §13 — Custom items (D11)

**OPERATOR.** `CustomRecordDto` gains optional `bonuses: Vec<CustomBonus { target, bonus_type, value }>`
(targets from a fixed list: attack, damage, ac, each save, each ability, each skill, speed, initiative,
hp, cmb, cmd; bonus types from the engine's stacking vocabulary) and optional `slot: Option<BodySlot>`.
A custom item with neither is the text item of today. Worn custom items apply (§9 rule). **Enforced by:**
E4.4, G-6.

## §14 — Custom chip styling (D10)

**OPERATOR.** The Add Custom dialog's top-row chips (`CustomDialog.tsx`, E1.4) must return to their resting style
on deselect; one selected chip at a time; focus ring distinct from selection; tokens valid in both
themes. The fix is in the shared chip component if one exists, so Market and the feat picker inherit it.
**Enforced by:** E1.4 (ui-smoke row + unit test on the class/style function).

## §15 — Model tiering

Per `~/.claude/CLAUDE.md` (2026-09-29): engine, UI, verify, every merge check → **Opus 5.5**
(`model: 'opus'`); well-specified mechanical work and every long wait → **Sonnet 5.5**; housekeeping
→ **Haiku 4.5**. Every `agent()` sets `model` (`awk '/agent\(/ && !/model:/' <script>` prints nothing).
Planning disclosure: this package was written by Fable 5.1.

## §16 — Run protocol (UNATTENDED)

The operator launched and walked away on 2026-11-01. The run follows the
standing autonomous rules (memory: "operator away — autonomous mode", "run to 100% DoD without asking"):

1. Clear a blocker first; if only an operator ruling can clear it, mark **only that card**
   `blocked-escalated` with the exact ruling under `progress.md ## Open blockers`; continue every
   independent card.
2. Never open the PR while any card is open, partial or blocked-escalated.
3. A choice inside scope with a safe default (§16.1): take it, log it, continue. No `AskUserQuestion`.
4. **Report after each epic** (the merge-check card writes the Epic report block in `progress.md`).
5. **E9.4 is the one planned wait:** the operator prints one sheet per system and records "usable" (or
   defects) in the receipt. The run proceeds to E9.5 only on "usable" ×3. Defects found there are
   discovered cards in E8, not blockers to argue with.

### §16.1 Safe defaults

| # | Decision point | Safe default |
|---|---|---|
| SD-a | A sheet value's derivation has no explanations row | "Show the math" prints the value with "(no derivation recorded)"; log a `retro.py note`; never invent a formula |
| SD-b | Compute vs print | Print unless the value lands on the sheet as a number (paper-sheet rule) |
| SD-c | A count differs from its CUI figure | Re-derive with two implementations; correction via `retro.py` with `--verified-by` |
| SD-d | A PF seed render delta | Allowed only when every changed field is listed in the receipt and is additive or the card's stated intent; otherwise a defect |
| SD-e | An SF or CPR seed delta outside E3.5 / E8.3 | A defect. Fix it. Never re-baseline |
| SD-f | A worktree, branch or file another session may own | Do not touch it; log it |
| SD-g | A PCGen `SLOTS` value with no Codex `BodySlot` | Map to `none`, list it in E1.0's receipt with the count; never guess a slot |
| SD-h | `typst` build fails in CI | E8.1 `blocked-escalated` naming the failing job and the alternative |
| SD-i | Usage-limit stop | Stop dispatching, write a resume receipt (§16.3) |
| SD-j | No CI on `tranche-19` pushes | CI evidence = the closure PR's `pr-tests`, awaited by E9.11 |
| SD-k | Rebase conflict | `kanban.md`: take upstream, re-apply only your own row, run the duplicate-ID check; `progress.md` append-only sections: keep both; any other file: abort, `blocked-escalated` |
| SD-l | Fresh-install check cannot run on this box | E9.2 `blocked-escalated` with the exact operator action |
| SD-m | A `.pcg` key with no Codex mapping | Custom item + import-report line; never dropped |
| SD-n | A feat's stackability is unknown (no `MULT` token) | Not stackable |
| SD-o | The migration finds more saved feats than slots (legacy over-budget) | Assign in saved order until slots run out; the rest get `taken_at_level = None` and feed the over-budget badge (R9) |

### §16.2 Watchdog — one long Workflow run

One long run with a recorded run id; recovery starts a **new** run with `START=<card>`; never resume a
run with parallel lanes. No cron.

### §16.3 Quota stop rule

Stop dispatching when any `agent()` result matches `/usage limit|rate limit|quota|limit reached/i`, or
the harness shows a weekly reading ≥ 85%. Lanes in flight finish; write a resume receipt. Never
downgrade a model.

### §16.4 Crash-resume recipe

Keep every dirty worktree; read the run id from `progress.md` and `journal.jsonl`; start a new run with
`START=` the first unrun card; the prefix must be byte-identical to `artifacts/cycle_0/sd39-workflow.js`;
log a `retro.py incident`.

### §16.5 Environment guards

Carried verbatim into `workflow-instruction.md §2.1` and `§6`.

## §17 — Closure order (STANDING, operator-pinned 2026-09-14)

All cards `complete` → retrospective written and cited → full worktree/branch sweep → **release notes**
→ **architecture truth-up** → **graphify LAST** over a clean tree at `origin/tranche-19` → **PR as the
final action**. The operator merges.

## §18 — Seed deltas in SD-39 (DEFAULT, replaces SD-38's byte-identical rule for PF)

SD-39 changes PF saved-character and sheet DTOs (slots, ledger stamps, overrides, Market ids). The PF
render hashes (`pf_seed_render_hash.rs`) **will** change. Rule: every card renders Aldric (Fighter 3)
and Elowen (Wizard 5) before and after; a delta is accepted only when the receipt lists every changed
JSON path and classes it `additive-field | intended-value-change`; any other delta is a defect. SF and
CPR seed outputs stay byte-identical except in E3.5 (Market shell) and E8.3 (print layout files), whose
receipts list their deltas the same way. **Enforced by:** G-1; R7 (per-character status) in every receipt.

## §19 — Paper-sheet rule (STANDING, 2026-09-07)

Compute only values that land on the sheet as a number; print everything else; dice stay literal. "Show
the math" prints the engine's existing explanations rows, never a new derivation engine.

## §20 — No PCGen in live code (STANDING, 2026-09-07)

`src/`, `apps/desktop/` and `apps/desktop/src-tauri/` parse no `.lst`, no `.pcg`, no PCGen token. The
`.pcg` reader lives in `crates/codex-ingest`; the app calls the sidecar binary. `pcgen-residue-gate` in
`scripts/verify.sh` is the control.

## §21 — Discovered cards

A defect found mid-run that is in the DoD becomes a card appended to `kanban.md` by the card that found
it (with the row-count constant re-counted in the same commit, R18), never a note.

## §22 — Planned capability deferrals

`forward-scope-register.md §4` DEF-1…DEF-6. Each has a revisit command checked at E9.5. None is a card.

## §23 — Fence exits

A lane that needs a file outside its FILES YOU OWN returns `declined` with `ownedBy:<card>` unless that
file's owning epic merge check is `complete`; then it edits under its own criterion and re-runs that
epic's gate, naming both in the receipt.

## §24 — Unattended run

The operator launched SD-39 on 2026-11-01 and said "You will be operating in unattended mode". §16 applies
in full. No live peer: the merged `.worktrees/fix-perform-install` was removed (R20), so SD-f guards only
trees found later. Subagents default to Opus; Sonnet and Haiku where the card's Tier says.

## §25 — Pre-launch corrections (2026-11-01; these win over any older text in this package)

The lead re-read the package against the repo before dispatch (`AGENTS.md`). Each row is a claim the
package made and what the code shows.

| Package said | Code shows | Effect |
|---|---|---|
| PCGen `SLOTS:` holds the body location | It holds a **hand count** (Core Rulebook values 2, 1, 0: `grep -rhE 'SLOTS:' cr_equip_*.lst`). Location comes from `TYPE` + `equipmentslots.lst` `EQSLOT` | E1.0 builds the table over `tags` |
| A converter re-run is needed | `tags`, `repeatable` and the `Slot:` prose line are already in `data/sheet_rules/**` | No re-run unless E1.0's proof fails. §5 is superseded |
| Worn vs carried must be built | `set_equipment_active_state`, the stow control (`CharacterSheet.tsx:1642`) and the engine filter (`pilot_compute_corpus.rs:280`) exist. Missing: slots, refusal, the Equipment view | E4 adds slots and proves the rest. §9's new commands are dropped. Saved items keep their state |
| Market needs `buy_item` / `sell_item` | `purchase_equipment` and `remove_equipment_selection` exist with `PriceMode` | E3.2 extends them. §8's new commands are dropped |
| `+N` needs a new `enhancement.rs` | `equipment_modifier/plus1a`, `plus1w`… and `attach_equipment_modifier` exist | E3.1 and E3.4 reuse them, and change only what a proof shows is missing |
| Selections gain `taken_at_level`, with a schema bump | `selected_feats` is `Vec<String>`, engine input | E5 uses a `ledger.json` beside `custom.json`. No schema bump, and saved characters are not rewritten. §11's fields live in that file |
| Sidecar via `tauri.conf.json externalBin` + `tauri-plugin-shell` | `tauri-build` fails when an `externalBin` is absent, and `publish-tester-release.yml` builds Linux, macOS and Windows | `tauri.sidecar.json` passed only at bundling; `std::process::Command`; all three jobs build the binary. §12 is amended |
| Linux only for print | The app also ships for macOS and Windows | Print opens the viewer (R19), which works on all three |
| "Campaign notes" exist | They do not | R17 |

| C1's acceptance grep may read `Cargo.lock` | The lockfile holds 24 third-party packages at 0.18.x (`awk '/^name = /{n=$3} /^version = "0\.18\./{print n}' apps/desktop/src-tauri/Cargo.lock \| awk 'END{print NR}'`). They must not change | The check reads only the `codex-desktop` entry. C1's receipt said 27, counting by a different predicate. No operator ruling was needed |
| E1.0: `SLOTS:` is a hand count that gives weapons main / off / two-handed | The awk count is right (2: 30, 1: 3, 0: 3), but the 2s are worn pairs (boots, gloves, bracers) and no Core weapon has one; a weapon's hands are `WIELD:`, settled as `wield_category` on the corpus record | E4.1 reads `wield_category`; `slot_of` reads only the `TwoHanded` tag |
| E1.0: 19 tag/prose disagreements | 19 reproduces only by skipping `Slot: none` and comparing spellings: 12 `wrists` vs `wrist`, 7 `neck (does not take up slot)`. 8 weapon-tagged rods and staffs say `Slot: none`. Total 27, of which 15 change the answer | The test lists all 27. The prose wins: the 7 necklaces and 8 rods and staffs have no slot |
| E1.0: `crates/codex-ingest/**` only on a failed proof | `data/equipment_types.json` is written by `gen_equipment_types`, so a table added by hand is lost at the next run. The rule was aimed at `sheet_rule_convert` | E1.0 owns that generator; `data/sheet_rules/**` stays unchanged, `git diff` 0 files |
| G-1: `sf_seed_lines_print_every_resolved_term_and_no_pcgen_residue` writes the SF seed lines | That test writes nothing; the writer is `sf_seed_printed_features_equal_the_features_the_converted_records_grant` (`sf_sheet_print.rs`). The old filter passes and leaves an empty directory, and `diff -r` of two empty directories proves nothing | G-1 names the writing test |

| E1.1's FILES YOU OWN: `feat_budget.rs`, `mod.rs`, `character_hub.rs`, `CharacterSheet.tsx`, `CreateCharacterForm.tsx`, `characterProgression.ts` | The Create count is `creationFeatSlots` in `manageItems.ts`; the draft's rows cross `boundary/draftSheetPreview.ts`; the feat levels need a command (`main.rs`) loaded with the roster (`classRoster.ts`), because the roster's wire is pinned to an SD-36 artifact | The row is amended; these are E1.1's named fence exits |
| A budget is a count against what the character holds | Creation seeds Power Attack, Weapon Focus and (human, monk) Dodge, and form picks were appended on top: Human Fighter 1 with 2 picks saved 5 feats on 3 slots. Weapon Focus is required of every class by `unmet_combat_posture_conditions`, and a slot naming anything but its default is claim-blocked by `validate_fighter_feat_choice_legality` | A pick displaces seeded Power Attack, then Dodge, and the default's slot with it. A non-Fighter's seeded `feat:weapon_focus` token is not charged. Saved characters are not rewritten: R9 applies to them |

| E1.2 and §6: the duplicate guard reads `stackable` | No such field: `grep -c stackable src/rules_core/sheet_rule.rs` is 0. The field is `repeatable`, equal to PCGen `MULT:YES` (E1.0, F-15). Nor does "unless stackable" settle Weapon Focus (longsword) twice: the feat is `repeatable`, and that pick is still a duplicate | E1.2 reads `repeatable`. A repeatable feat with a chooser contract is refused on a target it already holds; one with no contract (Fleet) may always be taken again |
| A Create pick is an add like any other, so a pick equal to a seeded default is a duplicate | The seeds are not shown on Create, so a refusal would refuse a choice the screen offers. Refusal is also wrong in the other direction: if the seed stays, a later pick can displace it, and the player loses the feat they picked | The pick takes the seed's place, with the seed's slot (Power Attack, Dodge). Weapon Focus is not displaceable, so Create refuses it on the seeded target. The same pick twice refuses with `create.feat_duplicate` |
| E1.3's FILES YOU OWN: `CharacterSheet.tsx` picker; technical design: a `FeatPicker` | No `FeatPicker` exists (`grep -rlw FeatPicker apps/desktop/src`: 0 files). The picker is `ItemPickerModal.tsx`, its logic is `itemPickerFilter.ts`, and `CharacterSheet.tsx` renders it. No card owned either file (`grep -n 'ItemPicker\|itemPickerFilter' epic-breakdown.md` names neither in a FILES column) | The row is amended; both files are E1.3's |
| R10: "Settings" as a place | App settings here are `localStorage` keys in `settings/*.ts` (`themeMode.ts`), and the box itself is the control | `settings/featPickerPreference.ts`, key `codex.featPicker.hideUnmet`. No row in the Settings dialog |

| E1.1: a class's feat pools are the rules with `target: Pool(p)` and a Number value; the census and `feat_pool_of` share that predicate | Swashbuckler and Gunslinger grant through an `offers` row with no target, so both read `{}` at every level (`artifacts/epic_1/E1.MC/pool-table.txt`), and an Elf Swashbuckler 4 is refused the bonus feat the book gives. `hunter_teamwork_feats#bonus6..11` repeat `#bonus0..5`, so a Hunter reads twice the book | E1.6. Dragon Disciple and Sentinel hold their grant only as prose, the same miss in another shape (E1.6 (c)). The table's other rows were read against remembered book numbers, not page cites: an estimate, not a proof |
| E1.5: the row "ends ticked" | The harness reads the screen once after the last step, and the assertions need the box unticked. No later row expects the hidden count or clicks Add Feat (`python3` over `spec.json`) | The row text now says unticked |
| E2.1's FILES YOU OWN: `sheetLayout.ts`, `sheetLayout.test.ts`, `CharacterSheet.tsx` | `panels/squareBlock.test.ts` reads `SHEET_COLUMNS.map` from `CharacterSheet.tsx` to pin source order (`grep -rn 'SHEET_COLUMNS' apps/desktop/src`), and a registry-rendered sheet has neither that name nor that order | The row is amended. The check now asserts the name comes before the combat row, and the block's place in it from the registry |
| E2.1 vs E2.3: the rail "holds ... Show Progression button", and the drawer comes later | The button must do something now (no-stub doctrine), and E2.3 owns the drawer | Until E2.3 the button opens the level history in the rail under itself, closed by default; print still shows it. `load-seed-wizard-fireball` expects `Skill points: 7`, printed only there: E2.4's row now names it |
| E2.2's FILES YOU OWN: `sheetLayout.ts`, `CharacterSheet.tsx` | Its acceptance command is `sheetLayout.test.ts`, which the row does not list, so the card could not write the test its own criterion names | The row is amended. The breakpoint is the flex-basis of the row's two children, not a media query: the registry's styles are inline, and inline styles have none. Below 360px + the gap + 320px of row, the 1x4 wraps under |
| E2.3's FILES YOU OWN: `ProgressionDrawer.tsx`, `CharacterSheet.tsx` | Its acceptance is a unit test the row does not list, and `sheetLayout.test.ts` pins `setProgressionOpen`, E2.1's interim button (`grep -n setProgressionOpen apps/desktop/src/characterHub/sheetLayout.test.ts`) | The row is amended. Print hides `[role="dialog"]` (`printLayout.ts`), so while printing the rail draws the history itself, until E8.4 retires that path |
| E2.1's acceptance: `grep -c '>Progression<\|Progression$' CharacterSheet.tsx` → 0 | Prints 1 since E2.3: line 4456 is the button label `Show Progression`, on its own line. The heading itself is gone | The row now anchors the pattern, `^ *Progression$`, which prints 0 (`sheetLayout.test.ts` already checked the heading with `>\s*Progression\s*<`) |

| E3.1: the offer maps to `plus1a`, `plus1w` … (and §25's row above names them) | They are hidden copies with no number (`plus1w.json` `value` is `Text`); the bonus is on `special_ability_<N>_<kind>`, catalog key `Special Ability ~ +N ~ <Kind>`, which `attach_equipment_modifier` already takes (`enhancement_price_table_modifier_keys_name_the_records_that_hold_the_bonus`) | Each offer carries that key as `modifierKey`. The row is amended. E3.2 and E3.4 use the key |
| §8: the generated item's id is `<base_id>+<N>`; technical design: a `+N` is a generated item | §25 reuses the eqmod path, where the bonus sits in the selection's `applied_modifiers` beside a plain `item_id` | No generated id. A row is the base key plus a `variants` list of `{ bonus, name, priceGp, modifierKey }`; E3.2 buys the base key and attaches `modifierKey` |
| Technical design §3: `MarketFilter` has a `system` field | `list_market` reads the PF catalog; SF and CPR have their own (`sf_catalog.rs`, `cpr_catalog.rs`), and a field nothing reads is a stub | No `system` field. E3.5 serves its own catalogs and adds what it needs |
| E3.1's FILES YOU OWN: `boundary/listMarket.ts` (new) | Every boundary has a no-runtime test (`listEquipment.test.ts`), and no card owned this one | The row is amended |
| E3.2's FILES YOU OWN: `character_hub.rs`, `boundary/purchaseEquipment.ts` | Copies of an item could not stay apart. The reader put an `equipment_modifier=` line on the first copy of its id (`character_input.rs`), so a save holding a plain sword and a bought `+1` reloaded with the bonus on the plain one (`left: [1, 0]`, `E3.2/red-reload-root.txt`); and a sale took the first copy, so it could not sell the `+1` it named. No card owned either file (`git show 72bb7894d2:docs/release/SD-39-ui-print-corrections/epic-breakdown.md | grep -c 'removeSelection\|character_input'` → 0) | The row is amended: `character_input.rs` (a modifier belongs to the copy just above it), `boundary/removeSelection.ts` (an optional `bonus`: `0` the plain copy, `N` a `+N`, absent the first), `boundary/purchaseEquipment.test.ts` |
| E3.2: selling is `remove_equipment_selection` at 50% / 100% / 0 of the catalog price | The catalog price of a `+1` is the plain sword's: 15 gp, on a 2,315 gp purchase | A sale pays the mode's share of the copy's own offer |
| E3.2: 9 cases, 3 modes × buy ok / buy refused / sell | Cashless skips the money check, so it cannot refuse for money | Cashless's refused case is a bonus not on offer, which every mode refuses |
| E3.1's receipt: E3.2 reuses `Enhanceable::price_gp` and `modifier_key` | Reusing that table in the buy would be a second path to the number. `list_market` is already public and `equipment_catalog.rs` needs no change | The buy reads the offer off `list_market`, so a hidden row or an unoffered bonus cannot be bought |

| E3.3's FILES YOU OWN: `MarketDialog.tsx`, `CharacterSheet.tsx`, `itemPickerFilter.ts`, `spec.json`, and a Sell list off the character's items | `corpusDerived.equippedItems` resolves against `corpus_fixture_bundle`, which holds 2 equipment records (`corpus_fixture_bundle_has_two_spells_and_two_equipment_records`). A bought Dagger and the `+1` on a Chain Shirt never reach it (`sheet-market-buy` red: `expected string missing: 'Sell +1 Chain Shirt'`), and no read the frontend has carries `applied_modifiers` | A new read, `list_held_equipment`, off the saved selections, with `copy_bonus` naming each copy. The row is amended and `character_hub.rs` and `main.rs` take E3.3 in the writer order: E3.2 is complete, and E3.6 waits on E3.3, so there is one writer. The two `.test.ts` files that pinned the retired picker kinds, `PriceModeControl` (`groupName`, because two controls under one radio name uncheck each other; an `aria-label` for the harness) and the money helper are on the row too. E4.3's lists read the same command |
| E3.2's next-cycle plan: E3.3 shows the engine's refusal on a second wielded weapon | The Longsword is required, so that refusal is every weapon's Buy on a Computed character (`market_buy_sell_a_buy_the_engine_refuses_charges_nothing`, `pilot_compute_corpus.rs` "more than one real weapon"). A list where nothing can be bought is the R-11 failure | Market buys from the Weapons list as `SelectedInactive` and says so on the status line. E4.1 owns wear-or-carry and replaces this |
| The Weapons tab's caption, on the screen E3.3 rebuilds: dropping an item "does not refund what it cost" | Drop sells back under the price mode (`sell_equipment_copy_at_root`) | The caption is corrected. Drop stays beside Market, and keeps the first-copy rule |
| E3.4's FILES YOU OWN: the eqmod consumer path, `equipment_resolver.rs`, `damage_total.rs`, `pilot_compute_corpus.rs` | The bonus could not arrive on that path: `corpus_fixture_bundle` held 2 equipment records and no `+N` eqmod, so a worn `+1 Breastplate` read AC +0 and the eqmod was skipped; only `to_hit_bonus` read a selection's `applied_modifiers`, so AC and damage ignored them on any corpus (`E3.4/red-proof.txt`). No card owned the bundle | The row is amended: `equipment_effects.rs`, `corpus_fixtures.rs`, `fixtures_src/`, `resources/corpus_fixtures/`, `gen_desktop_fixture_corpus.rs` (its list becomes `equip_*.txt`, not a table kept by hand), `encumbranceTabModel.ts` and its test, `boundary/loadCreateCharacter.ts`, `rule_system_adapter.rs`, `character_hub.rs` (DTO field, tests), `equipment_catalog.rs` (below), `spec.json`. Both named files take E3.4 in the writer order: serial main lane, E3.6 waits |
| E3.2, E3.3 and §25: the engine refuses a second wielded weapon, so that refusal is every weapon's Buy | Only a weapon the bundle resolved was refused, which was the Longsword: a wielded Dagger saved and counted for nothing (`set_equipment_active_state_at_root_stows_and_re_equips_a_carried_item`). With the refusal and the `item:longsword` gate together, a bought `+1 Longsword` could never be held: wielding it was refused beside the plain one, and stowing the plain one was refused | The refusal is gone. The Melee number is the required Longsword's own; any copy in hand meets the gate; two copies, the better one; another weapon has its own Weapons tab row. Market still buys a weapon carried, until E4.1. `market_buy_sell_a_buy_the_engine_refuses_charges_nothing` now refuses on no Longsword in hand |
| E3.1: every priced `enhanceable` arm takes `+1…+5` | 7 weapon rows cannot show one: `Net (Base)`, `Snag Net`, `Wrist Launcher`, `Lasso` and `Poisoned Sand Tube` have no damage die to add to, and `Mantis Blade` and `Rending Claw Blades` are `.COPY`/`.MOD` row pairs a one-row fixture cannot hold, each already magic. Selling those a `+N` would charge for a bonus that never arrives. 2 shield rows, `Shield (Light/Steel/Quickdraw)` and `.../Wooden/...`, twin `Quickdraw Shield (Light/...)` and normalize to the seeds' Absent `item:shield`, which would list a shield they do not hold | `market_variants` offers a `+N` only where it reaches the sheet (`bonus_reaches_the_sheet`): 210 weapon and 49 armor and shield rows, each asserted. The rows stay on the shelf at their plain price. The test names the 7 and pins `item:shield` |
| E3.4: "adds an `enhancement` AC row" | No such row existed. The per-item list is `AC by Source` (`buildAcBySourceRows`) | `armorClassEnhancement` on the per-item DTO, and a row of its own after the item, so the rows sum to the panel's total. Attack and damage already had their row: the Weapons tab's enhancement columns and the Melee tile |

| E3.5's acceptance: `grep -c 'Add Weapon\|Add Armor\|Add Gear'` on the two sheets → 0, and "their Add buttons are gone" | It prints 0 on the tree before E3.5 (`E3.5/red-grep.txt`), so it proves nothing. Neither sheet ever had those buttons: Starfinder's add path was `Add equipped` / `Add carried` in the panel of "Feats, spells and gear" and of the level-up dialog, and a Cyberpunk RED sheet had no add path at all (`append_to_character` refuses) | The grep stays and `SystemMarketDialog.test.ts` carries the proof: each sheet renders the Market, and neither sheet-reached dialog offers an equipment list. The creation form keeps its list: no saved character exists there for a Market to charge |
| E3.5's FILES YOU OWN: the two sheets, `sf_adapter.rs`, `cpr_adapter.rs`, `sf_catalog.rs`, `cpr_catalog.rs` | The dialog hard-codes five tabs and Cyberpunk RED has no magic items (`MarketDialog.tsx`); four commands each need registering (`main.rs`); the two sheet-reached panels hold the Add buttons (`StarfinderChoicesPanel.tsx`, `StarfinderChoicesDialog.tsx`, `StarfinderLevelUpDialog.tsx`); five rows drove those buttons (`spec.json`, `docs/testing/ui-smoke-inventory.md`); the engine files below | The row is amended. These are E3.5's named fence exits: E3.3 is complete and neither E3.4 nor E3.6 writes them. New: `boundary/systemMarket.ts`, `SystemMarketDialog.tsx`, their tests. E3.MC's grep for `MarketDialog` matches `SystemMarketDialog` |
| R11 on Starfinder: buy deducts, sell credits | Starfinder holds no balance: Credits remaining is Table 11-5 minus the prices of what is held (`sf_loadout.rs`). Dropping an item always returns 100%, and an item cannot be held free, so two of the three modes have no way to be true | `SfLoadout.credits_adjustment` (`sf_loadout.rs`), saved as one `selected_choices` row, `market.credits_adjustment`, read and removed from the choices by `build_from_input`. Standard sale: minus the half not returned. Cashless buy: plus the price. Cashless sale: minus the price. No row when it is 0, so the seeds are byte-identical. Not taken: hiding the price-mode control on Starfinder, which R11 does not allow, or a `money.json` sidecar, which `compute_sheet` cannot see |
| A Cyberpunk RED sheet's choices are its inventory | `cpr_sheet_choices.txt` has weapons, armour, one shield and ammunition, and no gear. The whole inventory is the kit, `cpr_kit.txt`, written at creation and never again | Market edits both: the kit holds the item, and the one-item reading `kit.rs` already gives a whole kit (`sheet_of_item`) says what reaches the sheet. `CprKitCatalog::price` goes public so a row's price is the shop's. `src/rules_core/cpr/kit.rs` is a named fence exit |
| Cyberpunk RED's catalogs are all on the Market | The shop has 11 kinds. Cyberware needs a mount and a rolled Humanity Loss (Improve's dialog, DEF-2); a deck's programs and hardware install on a deck with a slot count | The Market lists 7 kinds: weapon, armor, gear, attachment, ammo, fashion, drug (`MARKET_KINDS`, `cpr_catalog.rs`). The other 4 are neither listed nor on the Sell list |
| A fixed kit's item prices itself at the shop | 16 of the printed starter names are not the shop's: `Rope` is `Rope (60m/yds)`, `Urbanflash Jacket` is `Urban Flash` + `Jacket` (`market_shell_every_fixed_kit_item_has_a_price` pins the list). Nothing in the data maps them, and R2 bars a hand-kept table | Such an item is on the Sell list, pays nothing, and says `Removed <name>; no money moved.` |
| `equipment_modifier=` splits at the final colon, because "the modifier item id does not" hold one | A Starfinder modifier's id holds three: a saved upgrade reads as an unknown item and the character cannot load (`market_shell_a_copy_sells_with_what_is_on_it`, `E3.5/red-modifier.txt`). The dialog saves through the same writer, so a Starfinder character given an upgrade there could not reload | The item id is the held selection's that prefixes the line; the final colon only words the diagnostic. `src/rules_core/character_input.rs` is a named fence exit, E3.2 being complete |
| E3.6: 3 modes × charged / refused / sold back | Cashless skips the money check, so it cannot refuse for money, as on E3.2's row | Cashless's refused case is a copy the character does not hold, which every mode refuses (`market_attach_price_cashless_refused_when_no_copy_is_held`) |
| `attach_equipment_modifier_at_root`'s doc comment and its boundary: the enhancement records run `+1` through `+10` | `Special Ability ~ +6 ~ Weapon` is refused `equipment.attach_modifier.unknown_item` (`E3.6/red-behaviour.txt`), so a bonus past a row's five cannot attach free | The comment now says what the test shows. E3.6's "no such offer" case is a row with none, `Net (Base)` |

| E3.6: "a row with no such offer, including a legacy `item:*` id, attaches as it does today" | Creation gives every Pathfinder character `item:longsword` and `item:chain_shirt`, the ids the Gear tab's Attach Modifier sends, and since E3.4 a `+5` there moves AC, attack and damage. So the charge E3.6 built was skipped on exactly the gear a character starts with (`E3.MC/red-legacy.txt`: `left: 230000  right: 0`). The id resolves to the record the Market row sells | The row is amended. A legacy id pays by its record's row (`copy_offers`), and the attach, the sale and the Sell list read that one function. A row with no offer still attaches free. The v0.6 free-attach test now holds a Net. Not taken: a discovered card, which nothing would dispatch after this merge check (`E2.MC` found that for E1.6) |
| E3.3's receipt: `applied_modifiers` need not cross the export/import DTO | That DTO is what makes Export then Import a round trip, and it dropped every equipmod: a paid `+3 Longsword` came back plain (`E3.MC/red.txt`, `market_buy_sell_an_export_and_import_keeps_what_each_copy_holds`) | `EquipmentSelectionImportDto.applied_modifiers`, `#[serde(default, skip_serializing_if = "Vec::is_empty")]`, so a file with no equipmod is the file it always was and an old file imports. The frontend reads no such field (`grep -rn equipmentSelections apps/desktop/src --include=*.ts --include=*.tsx`: 0 lines) |
| E3.5, the row above: Cyberpunk RED's other 4 kinds "are neither listed nor on the Sell list" | Refuted twice, by buying every row (`E3.MC/cpr-every-row.txt`; 253 rows). The shop also files 3 `Cyberdeck (...)` records under `gear`: listed, charged, no deck on the sheet. And `market_terms` read a name shared with a cyberware record as installed: `Dartgun` and 10 goods (`Radio Communicator`, `Medscanner`...) could be bought and never sold | `market_rows` lists no `Cyberdeck (`, 250 rows now. `market_terms` reads the Market row first; an installed item says so by its kit `section`, and `market_shell_a_starter_installed_item_is_not_on_the_sell_list` holds that |

| E4.1: weapons are main / off / two-handed "from the settled `wield_category`", and `body_slots.rs`'s header: that field is the answer for a longbow | 60 of 211 damage-stating, non-shield fixture records state no category (`python3` over `resources/corpus_fixtures/_settled/equipment.json`), `Longbow (Base)` among them: the book gives ranged weapons no `WIELD:`. The fixture also holds only what the Market offers a `+N` of, so `Blackaxe` (two-handed in `data/corpus`) was unknown to it, and `Battleaxe` states nothing itself and names `Battleaxe (Base)` | `settled_wield_category` reads the whole corpus and chases one `base_item` hop. A record stating none is two-handed if tagged `Bow`, or `Crossbow` + `Projectile`. Every other weapon stating none is one hand, firearms included (`Rifle` has no tag saying otherwise): permissive, so it never refuses wrongly. Not taken: a table of firearms by hand |
| `body_slots.rs`: the record already holds what is needed | A derived record can hold no slot-naming tag (`Mithral Shirt` `["LesserMinor"]`, `Dagger (Adamantine)` none), and one printing of a name can lack what another states (`Madu`, `Quickdraw Shield`). Over 5,880 Market rows, 6 armor and 2 shield rows read as no slot, and 4 shields as a weapon | `worn_slot` reads the slot over the record's tags and the catalog's `types` together. `slot_conflict_every_market_row_finds_its_record` holds the whole shelf; the only rows with no record are `Dust Knuckles.FORGET` and `False Face.FORGET`. `slot_of` is unchanged, so E1.0's counts hold |
| E4.1's FILES YOU OWN: `body_slots.rs`, `character_hub.rs`, `setEquipmentActiveState.ts`, `CharacterSheet.tsx` | The reason crosses `boundary/purchaseEquipment.ts`; `marketActiveState`, its test and the dialog's `category` argument pinned E3.3's carried-weapon rule (`itemPickerFilter.ts`, `MarketDialog.test.ts`, `MarketDialog.tsx`); 3 market rows read the status line or stacked two armors (`spec.json`, `ui-smoke-inventory.md`) | The row is amended. E3.MC is complete, so these are named fence exits (§23); E3's gate re-ran: 20 `sheet-market-*` rows and the desktop sweep |
| §25, E3.3's row: Market buys a weapon carried until E4.1 | E4.1 replaces it (R18): the Market always asks for worn, and the backend answers where the copy went (`activeState`, `carriedReason`). A Dagger beside a Longsword takes the off hand; a second one is carried with the reason | `sheet-market-buy-weapon-carried` is now `sheet-market-buy-weapon-worn`. `sheet-market-enhancement-ac` sells the plain Chain Shirt first, because a `+1` beside it is now carried |
| `apply_set_equipment_active_state`: the first carried copy, the same first-match rule as a removal | The first copy is the one already worn, so wearing a carried second copy reported `Saved` and moved nothing (`slot_conflict_a_second_copy_of_the_same_helmet_is_refused`, red: `got Saved`) | A state change moves the first copy not already in that state. Which of two different `+N` copies moves is still the first such one: E4.3's panel lists copies and must name one if it offers that choice |
| R18 and E4.1: a two-handed weapon is worn when both hands are free | Refused for every character holding the required Longsword: it fills the main hand and the engine refuses stowing the last one (§25, "any copy in hand meets the gate"). So a Greatsword bought by a seed is always carried, with the reason, and cannot be taken in hand | The check is right and says why. The gate is not this card's; it is the operator's question whether a character may give up the Longsword. Recorded in E4.1's receipt |
| F-16: 5 `.pcg` fixtures in `eqm-fixtures` | 13 (`ls scripts/oracle_harness/eqm-fixtures/*.pcg \| awk 'END{print NR}'`) | F-16 corrected, with the 52 under `$PCGEN_REPO_DIR` |
| E7.1 and NR-13: every fixture converts, and every one loads | Of the 65, 17 are `GAMEMODE:Pathfinder_RPG`, the repo's 13 and the oracle's 4 (`grep -h '^GAMEMODE' \| sort \| uniq -c`: 3e 16, 35e 24, Starfinder 3, Modern 2, zen_test 1). `none_PdfBuster.pcg` and `none_derth.pcg` are PCGen 2.4's positional format with no `PCGVERSION`. 26 name a class or race with no Codex record (`Test Nothing`, `Soldier`, `Dragon`), and no row can compute those (R15: Pathfinder only). A class is never guessed | The 2 are refused by name. The rest convert: the class is a custom feat and named in `report.unmappedClasses`, and `import_character` returns the engine's refusal. 37 save through the real command. NR-13 is amended |
| E7.1: "Reuse any `.pcg` reading `oracle_validation/` holds" | It holds none: `pcgen_runner.rs` hands a path to PCGen and reads its output. `sf_mapping_table.rs` reads two keys in a test | `pcg_import/parse.rs` is new |
| E7.1's FILES YOU OWN | A new module needs `pub mod` in `crates/codex-ingest/src/lib.rs` | The row is amended, one line |
| Technical design §7 and §12: `import_character` is the whole path | Its DTO has no bio, money, language or custom-record field, and serde drops what it does not know (`ImportedCharacterFileDto`). The Pathfinder sheet has no language list (`spokenLanguages` derives it). Money lives in `money.json`, bio in `bio.json`, records in `custom.json`, and a packaged sidecar needs the resource directory | The JSON carries `bio`, `money`, `custom`, `report` beside the two fields it reads. E7.2's row writes them and passes `--data-root`. Languages and the free-text bio are custom feats, so they print. `EQUIPSET` gives worn or carried; no selection field holds a slot |
| E7.1: "ranks" convert to the JSON `import_character` accepts | Both cannot hold: the engine refuses Climb, Intimidate or Swim at anything but 1 rank (`require_selected_skill_rank`; `set_skill_allocations_at_root_does_not_persist_when_resulting_build_is_blocked` pins it), absent included, so `35e_L10(fighter-human).pcg` with 13 in each could not save | The three rows are written at 1, with the file's rank in a warning and on a printed custom feat. E7.3 is the fix, so E7.1 is partial until it lands |
| §12: an import is what the file says | The engine also needs a worn Longsword, the `power_attack` row, Weapon Focus, a Fighter's `choice:fighter_bonus_feat`, each class's seeds; and its class spell lists are narrower than PCGen's, so `pf_Paladin.pcg`'s Compel Hostility blocked the character (`class_spell.paladin.partial_caster.unsupported`) | Added, as Create adds them, each with an `added` report line (R-15). A spell that brings a new `class_spell.*` refusal is a custom spell. 47 of 186 `SPELLNAME` lines across the 63 converted files, none of them refused for a reason but this or an unknown name (`pcg_import_fixtures_a_spell_is_kept_or_custom_and_most_are_kept`) |
| The dispatch prompt: "re-count the 51 constant" | It reads 54 at this card (`awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/' kanban.md \| awk 'END{print NR}'`), 55 with E7.3 | All 7 places re-counted |
| `kanban.md`: E7.2 depends on E7.1 | The script dispatches a card only when every dependency is `complete`, and E7.2's prompt declines otherwise. E7.1 is `partial` until E7.3, E7.3 depends on E7.2, and E7.2 listed E7.1: a cycle no lane can leave. E7.2 reads only E7.1's committed binary and JSON, which the ranks do not change | E7.2's `Depends on` is `E6.MC`, naming E7.1's commit. E7.3's closing commit moves E7.1 to `complete`, and E7.MC still waits on E7.3 |

**Measured (`python3` over `data/sheet_rules/*/equipment/*.json`, 2026-11-01):** 7,386 records, 2,906 with a slot
tag, 1,617 with a `Slot:` line, 27 where the two differ (E1.0; the 19 first written here left out the 8 `Slot: none` weapons); the prose wins. Items with neither have no slot.

**Slot vocabulary.** The operator said a ring takes "a hands slot". The book and `equipmentslots.lst` give rings
their own slot, two of them. SD-39 follows the book; a change is one table row.
| E4.2 and technical design §2: the 5 consumers branch on `ActiveState`, so a proof per consumer is the proof | All 5 filter correctly (`tests/worn_vs_carried.rs`, green before any fix), and the sheet still failed: `corpus_fixture_bundle` held no ring or gear, so a worn Ring of Protection +1 read AC 16 → 16 and a carried Cloak of Resistance weighed 0 with `unresolved: ["power_attack", "Cloak of Resistance +1"]`; and `HeldSeed` holds every selection's rule, so a stowed Chain Shirt printed `Chain Shirt (AC) +4`. `HeldSeed` names no `ActiveState`, so F-11's grep never found it | The bundle takes the whole corpus's records after the fixture, less any whose identity has the modifier shape `Kind ~ Name` (`add_what_a_character_can_hold`), 30 books, already shipped as `resources/corpus_bundle/`. 5,866 of 5,880 Market rows resolve; the 14 are named in `worn_vs_carried_every_market_row_resolves_on_the_sheet_corpus`. A stowed item keeps its words and gives no bonus (`drop_the_bonus_of_an_item_not_worn`). E3.MC is complete, so `corpus_fixtures.rs` is a named fence exit (§23); E3's gate re-ran |
| E4.3's row: `corpusDerived.equippedItems` leaves out what the demo corpus does not hold | It still leaves out those 14 rows and nothing else on the Market. `list_held_equipment` remains the list that holds every copy | E4.3 is unchanged |
| F-11: 5 files name `ActiveState::` | 8 do (`grep -rln 'ActiveState::' src/rules_core/*.rs \| awk 'END{print NR}'`); 3 are test or fixture uses | F-11 restated |
| E4.3's row: the panel shows the override tag | No override exists to tag: `ignore_slot_limits` and the selection's `override` field arrive with E6.1 and E6.2 (`grep -n 'ignore_slot' apps/desktop/src-tauri/src/character_custom.rs` → no line), and E6.1 waits on E5.MC, which waits on E4.MC, which waits on E4.3 (`kanban.md`), so holding E4.3 for the tag is a cycle. A prop nothing can set would be a stub | The tag moves to E6.2, whose row already owns the Gear tab and now names `EquipmentPanel.tsx`, the `list_held_equipment` row and a ui-smoke row for it. DoD 7 is unchanged: only the card that delivers it moved |
| E4.3's FILES YOU OWN: `EquipmentPanel.tsx`, `CharacterSheet.tsx`, `spec.json` | The row needs each copy's worn state and slot, which `list_held_equipment` did not carry, and §25 says the panel must name the copy it moves, which `set_equipment_active_state` could not. Which hand a one-handed weapon is in is `slot_conflict`'s count, so `body_slots.rs` answers it (`worn_places`) | The row is amended: `character_hub.rs`, `body_slots.rs`, `boundary/listHeldEquipment.ts`, `boundary/setEquipmentActiveState.ts`, `EquipmentPanel.test.ts`, `ui-smoke-inventory.md`. `character_hub.rs` takes E4.3 in the writer order: E4.2 is complete and E4.5 waits on E4.4, which does not write it, so there is one writer |
| E3.3: `list_held_equipment` is what the character holds | It listed creation's `power_attack` selection, a feat toggle the engine requires carried (`pilot_compute_corpus.rs`, `require_active_state`), so the Market has offered `Sell power_attack` on every Pathfinder character since E3.3, and the panel offered `Equip power_attack` (`E4.3/red-feat-toggle.txt`) | The list leaves it out (`equipment_panel_the_feat_toggle_is_not_a_held_item`). `market_held_lists_every_copy_and_each_row_sells_itself` counts past it |
| DoD 5: worn items apply, and E4.2's "only `EquippedActive` feed" | The "only" holds, and is nearly vacuous: no worn item feeds a save, an ability score or a skill total, because no consumer reads `var_bonuses`, `ability_score_bonus` or `skill_check_bonus` for those. Touch AC subtracts a deflection bonus, one bonus type stacks, and a derived record such as Mithral Shirt states no AC | Discovered card E4.5 (§21) |
| E4.4's FILES YOU OWN: `character_custom.rs`, `CustomTab.tsx`, "the engine consumer of custom bonuses", `boundary/*.ts` | No such consumer exists, and those files cannot reach a player. The form is `CustomDialog.tsx` and `customModel.ts`; the tiles read `load_saved_character_at_root` and `recompute_character_at_root`; speed and hit points are held in the sheet; a worn item is moved by the Equipment control through `set_equipment_active_state`; a slot must hold against stock items too (R12), which is `item_slot_conflict`; `list_held_equipment` is also the Market's Sell list | The row is amended: `character_hub.rs`, `characterHub/recomputeCharacter.rs`, `CustomDialog.tsx`, `customModel.ts`, its test, `EquipmentPanel.tsx` (its slot list exported), `CharacterSheet.tsx`, `boundary/listHeldEquipment.ts`, `spec.json`, `ui-smoke-inventory.md`. `character_hub.rs` takes E4.4 in the writer order: serial main lane, E4.3 complete, E4.5 waits on E4.4. The consumer is `WornItemBonuses` in `character_custom.rs`, because the file it reads is the desktop's; no root file changed |
| §13: `CustomRecordDto` gains `bonuses` and `slot` | Then nothing says worn or carried, and §9's "worn custom items apply" has no input: `custom.json` holds no `ActiveState` and no selection names a custom item | `worn`, default false. The Equipment control moves it through `set_equipment_active_state` with item id `custom:<id>`, so a refusal reaches the panel word for word. A record with no bonus and no slot is not listed there, and writes none of the three keys, so it stays the text record it was. All three are refused on a custom feat, spell or device: not taken, accepting a number nothing applies |
| §13: "bonus types from the engine's stacking vocabulary" | The engine has no such list. `BonusType.name` is a free word; only `STACKING_TYPES` is fixed (`sheet_rule.rs`) | `BONUS_TYPES`: 17 engine words and `Untyped`. The fold's rule is repeated for the custom items: a penalty, an untyped bonus and a `stacking_types` type always count; otherwise the higher of one type on one target. `Evaluator`'s fold takes a rule package and held rules, not numbers, so it was not callable |
| The sheet's tiles follow a saved change | `recomputed` took precedence over the load for the rest of the sheet's life: `grep -n 'setRecomputed(' CharacterSheet.tsx` printed one setter, so after Recompute no equip moved AC. And `purchaseEquipment` and `attachEquipmentModifier` publish their own response's snapshot, which holds no worn custom item | `recomputed` clears when `props.detail` changes. With a custom item worn, those two handlers publish from the load. E4.4's own criterion needs both, so both are fixed here |
| E4.5: two bonuses of one type take the higher | From E4.4 a custom item has a type and a stock item does not, so a custom `Deflection` adds to a worn Ring of Protection, and a custom `Armor` to a Chain Shirt | E4.5's row now names it: when a stock item settles a type, the two fold together. E6.1 and E6.2's `ignore_slot_limits` must also reach `slot_refusal` and `set_custom_item_state_at_root`, the two new callers of `slot_conflict`; E6.1's row names them |
| E4.5: a derived record reads its `base_item` | `Mithral Shirt`, `Celestial Armor` and `Flame Tongue` settled `base_item: null`; the link was only in `name` (`Chain Shirt (Base)`) | The converter names a `(Base)` template as `base_item` when no `BASEITEM:` token does: 179 records, the only non-additive bundle change (`python3` diff of the old and new bundles). `settled_wield_category` and the damage-dice chase read it too. `resolve_category_effect` and `compute_encumbrance` fill field by field, one hop, and what a record states stands, so a Mithral Shirt keeps its 20 lb and reads Chain Shirt's max Dex and check penalty until DEF-1 |
| E4.5's row and E4.2: `var_bonuses`, `ability_score_bonus` and `skill_check_bonus` are the numbers no consumer reads | `BONUS:SAVE` is settled by none of them (53 chains), none states a type, and `skill_check_bonus` takes one skill of a list | One new settled field, `stated_bonuses`, reads STAT, SKILL, SAVE and the `<Save>Save_*` VARs, with `armor_class_bonus_type` beside it: 448 rows, skill 209, ability 135, save 104, and a second implementation over the raw chains agrees. A chain with a condition (48: `PRECLASS`, `PRERACE`, `PRETYPE`) or a formula (23: `%CHOICE`) settles nothing, and the item's words print it (§19). `PREVAREQ:Disable...,0` is standing. The named row's FILES YOU OWN grew: `equipment_record.rs`, `encumbrance.rs`, `pf1_adapter.rs`, `character_custom.rs`, `recomputeCharacter.rs`, `tests/worn_vs_carried.rs`. Each is a fence exit (§23); `character_custom.rs` and `recomputeCharacter.rs` are E4.4's, complete, in the serial lane |
| E4.4's `custom_item_bonuses_stack_by_the_engines_rule`: a custom `Armor` +5 adds 5 | It wore over Aldric's Chain Shirt because no stock item had a type. The row above E4.5's says the two fold | It adds 1 now, and the test says so |
| DoD 5, §9: E4.5 is the rest of "worn items apply" | §9 names attack. E4.5's row names saves, scores and skills, and a worn item's `COMBAT\|TOHIT`, `COMBAT\|INITIATIVE` and `MOVEADD` still move nothing, nor a Craft, Perform or Profession subtype | Discovered card E4.6 (§21) |
| E4.6's row: a Craft, Perform or Profession subtype bonus reaches the family row with the subtype named | The Skills panel has one number per family, and a bonus on one Perform is on no other, so adding it would print a wrong total for every other Perform. No TSX is on the row | The family row does not move. The number prints under the panel with its subtype, on the line that already prints what no row adds (`feat_skill_bonuses.situational`: `Musical Staff: +5 Competence on Perform (String Instruments)`). The same for a skill with no row at all (`Ring (Self Sufficiency)`, Autohypnosis). Printed on `Not added to a total:`, because that is true |
| E4.6's FILES YOU OWN: `ir_converter.rs`, `equipment_effects.rs`, `pf1_adapter.rs`, `character_custom.rs`, `character_hub.rs`, the settled files | `tests/worn_vs_carried.rs` pins `Robe of Vermin`'s exact rows, and the acceptance filter needs a root test; `equipment_record.rs` documents the target words | The row is amended with both, which E4.5 already took as fence exits (§23). A comment only in `equipment_record.rs`. `resources/corpus_fixtures/_settled/equipment.json` regenerated with no change: none of its 279 records states one of the three |
| DoD 5: E4.6 is the rest of "worn items apply" | E4.5's and E4.6's rows name named skills, attack, initiative and speed. 13 flat standing chains still move nothing (9 on a set of skills, 3 on bare CMB or CMD, Unlucky Figurine's Armor Class), and a Loadstone's speed multiplier | Discovered card E4.7 (§21) |
| E4.7's row: a skill set reaches every row | The Skills panel's set words are the book's `TYPE` words, and the engine holds no table of them; the only key-ability table is private in `skill_allocation.rs`, which E7.3 owns | `equipment_effects::skill_set_rows` holds the sets, and `tests/worn_vs_carried.rs` holds each to the book's own skill records (`TYPE:Dexterity.ACHECK.Base`) |
| E4.7's FILES YOU OWN: closed | `rules_core_equipment_record_via_converter.rs` re-derives the old `COMBAT\|AC` read over the whole corpus, so it failed on the one record whose chain names it in a list (`Unlucky Figurine`) | Its reference read takes a list; row amended |
| E4.7's row: `MOVEMULT` reaches the Speed tile | PCGen's `MovementResultFacet` adds `POSTMOVEADD` after the multiplier and truncates; the sheet has one `custom_item.speed` row for `MOVEADD` and `POSTMOVEADD` (E4.6) | The Speed tile halves race + row, whole feet rounded down. With a Loadstone beside a `POSTMOVEADD` item (3 records) the tile prints 20 where the oracle's formula gives 25 on a 30 ft. race, and the book's halving reads on the total. Not a card: the receipt names it |
| E4.5's row: a worn Ring of Protection raises touch too, so its twins are named | CMD stayed at 18 under a worn Ring of Protection +2, and CRB p.199 counts "circumstance, deflection, dodge, insight, luck, morale, profane, and sacred bonuses to AC" and any penalty to AC (`E4.MC/red.txt`, `left: 18 right: 20`). The formula's comment in `class_shared_core.rs` left feats' bonuses out on purpose; nothing had decided worn items | `WornItemBonuses` lays an Armor Class bonus of those 8 types, stock or custom, on the `cmd` row, and takes the higher of it and a stated `cmd` of its type. A penalty counts once: an item that states `cmd` itself (`Unlucky Figurine`) is not counted twice. Feats' bonuses to CMD stay as that comment says |
| E4.4 and E4.5: what a raised score drives follows from the engine | Not every read should follow. A worn Headband of Vast Intelligence +2 moved Elowen's `class_chassis.skill_points` 30 to 35 and her spellbook 15 to 16, and the sheet's own budget and language slots with them (`E4.MC/probe-output.txt`). The book says a temporary bonus for 24 hours and then ranks in named skills, never a point per level, and stowing it would have put her over | Spell DCs and spells per day still move. `class_chassis.skill_points`, the spellbook and formula book sizes stay on the saved score. `ability_modifier.intelligence.saved` tells `savedIntelligenceModifier` in `customModel.ts` what the budget and languages read; the row exists only when an item moves the score, so seeds do not change |
| E4.4's receipt: export, import and clone "were not examined for `custom.json`" | Both lose it: `clone_character` copied the portrait and not the file, and `export_character_to_json` wrote only the input, so a worn custom item and every Custom tab line were gone | The clone copies it (`copy_character_custom`, an error if it cannot). `ImportedCharacterFileDto.custom` is optional and left out when empty, so a file with no custom data is the file it always was and an old file imports. An import writes it as stated: ability grants are already in the file's scores. Fence exit (§23): E4.4 is complete, serial lane |
| E4.6's and E4.7's `unsettled-chains.py` as the remainder of "worn items apply" | The script counts only the heads it names. `VAR\|LOADSCORE` (Muleback Cords +8, Backpack (Masterwork) +1) and `LOADMULT\|TYPE=SIZE` (Heavyload Belt +3) are flat, standing, and move load limits the sheet prints (`E4.MC/chains.py`) | Settled as `load_score` and `load_multiplier`. `compute_encumbrance` reads what is worn: `load_score` on the Strength its table reads, and the score does not move; `load_multiplier` on the size's multiplier, the pinned oracle's `LoadFacet.getLoadMultForSize`. No book text was read for the belt: the corpus holds no words for it. `Burdenless` (`0.50`, a modifier) stays unread. Fence exits (§23): `encumbrance.rs`, 2 settled files |
| E5.1's row: `ledger.json` beside `custom.json` | Beside `custom.json` is also where a clone and an export lost a file (row above) | E5.1 must carry `ledger.json` through `clone_character` and `ImportedCharacterFileDto`; its row says so |
| E5.1's row: `character_hub.rs` add paths | The add and remove paths are not in `character_hub.rs`: `add_feat_selection_at_root`, the spell and skill paths and level-up are in `pf1_adapter.rs`, and every one of them, with `remove_*` and the appending adapter, passes `mutate_saved_character_at_root` (`grep -n 'SavedCharacterStore::save(' pf1_adapter.rs`: one call before its `mod tests`, in that function). A stamp at each command is a list a new command can miss | One stamp in `mutate_saved_character_at_root`, by what the character holds before and after. `pf1_adapter.rs` is a fence exit (§23; E4.MC complete); row amended |
| §11: an ability increase is a selection | The Pathfinder sheet has no ability-increase selection: `characterProgression.ts` only names the level, and the one writer of a saved score after creation is an `ability:<name>` grant (`grep -n 'ability_scores' character_custom.rs character_hub.rs pf1_adapter.rs`). Nothing else can be stamped | A positive `ability:<name>` grant is the increase: one row a point, `target` the grant's id. A penalty is not, and lifting one is not, so the row comes from the grant and not from the score moving. `character_custom.rs` is a fence exit (§23; E4.MC complete), because a grant can change with the score standing still and `custom.json` is written after the mutation; row amended |
| §11: new adds stamp the current character level | A character made above level 1 picks everything at once | All of it is stamped at the level it was made at, `assigned_on_upgrade` false: that is when it was taken. Slot order (E5.2) is for what nothing recorded |
| E5.2's row: on first load with no `ledger.json` | A change can reach a character with no file before E5.2 lands, or with no load before it, and a file holding only the new row would stop E5.2 ever filling the rest | The change writes the rows already held with `taken_at_level: None`, `assigned_on_upgrade: true` (`rows_of_a_character_with_no_ledger`), which is E5.2's seam. A change to something the ledger does not record writes no file. Starters and old imports have none until E5.2 |
| Technical design §5: a schema bump and `migrate_ledger()`; `workflow-instruction.md §3`'s E5 row: `src/saved_character/*` | The first row of this section already replaced them, and both still said it | Both rewritten to `character_ledger.rs` |
| E5.2's FILES YOU OWN: `character_ledger.rs` alone | The load is `load_saved_character_at_root` in `character_hub.rs`; which slot a feat fills needs `FeatBudget`'s private `admits` and `assign` (`src/rules_core/feat_budget.rs`); the known tables are `pub(super)` in `pilot_compute` (`grep -n 'pub(super) fn .*spells_known' src/rules_core/pilot_compute/*.rs`) | Fence exits (§23; E1.MC and E4.MC complete): one line in the load, `feat_slot_levels`, `spells_known_table`. G-3 and G-6 re-run; row amended |
| §11: spells fill spells-known slots per class level | Three shapes, not one. Seven classes have a known table (sorcerer, oracle, bard, skald, inquisitor, hunter, bloodrager); book casters have `spellbook_size` or `formula_book_size` rows that bound a level and higher; a list caster (cleric, druid) has no limit, only the level its class opens the spell. And the engine states no casting rows for an input holding a spell above that level, so a Wizard 1 holding Fireball reads as no caster | A spell goes in the earliest level its class casts that spell level, inside whichever limit the class has. The levels are read with each class's canonical spells. A spell the class has no list level for, or never reaches, has no level |
| §11: ranks fill per-level rank budgets | Budget alone gives Spellcraft 4 ranks at level 1, which the book forbids (CRB p.87). The human rank is `race.human.trait_bundle.extra_skill_ranks`, 1 a level, no class's term (`multiclass_fold.rs`) | The nth rank of a skill goes no earlier than level n. Budget per level: the rise in `class_chassis.skill_points` or `multiclass.skill_points`, plus that row. Favored-class ranks are in no row, so those go without a level |
| §11: feats fill level-feat slots in saved order (SD-o) | First-come can strand a feat: Fighter 2 holding Power Attack, Weapon Focus, Toughness fills the level slot and leaves Toughness, not a combat feat, nowhere, while the sheet says not over budget. The baseline Weapon Focus takes no slot on a class with no Fighter pool | Earliest free slot first; a feat moves to a later slot only when a later feat cannot otherwise fit, so the rows with no level number exactly `over_by`. The baseline Weapon Focus reads level 1 |
| E5.2: load writes the file | A load that cannot build (an unreadable `custom.json` or rules package, a failed write) has a sheet to show, and the sheet reads no ledger (R-6). An unreadable `custom.json` already reads as empty there, which here would lose the ability rows | A load writes nothing, says so on stderr, and the next load tries again. A change on a character with no file still refuses on the same failure, as a ledger it cannot read does |
| E5.3's FILES YOU OWN: `character_hub.rs`, `main.rs`, `ProgressionDrawer.tsx`, `boundary/listProgressionLedger.ts`, `characterProgression.ts` | The rows need the ledger's private slot work (`input_at_level`, `ranks_each_level`, `SpellSlots`), the drawer's content is handed over in `CharacterSheet.tsx`, the row's ui-smoke row is `spec.json` and its inventory, and the tests are three more files. E5.MC waits on this card, so `character_ledger.rs` cannot wait for it | The row is amended: `character_ledger.rs`, `CharacterSheet.tsx`, `boundary/listProgressionLedger.test.ts`, `ProgressionDrawer.test.ts`, `characterProgression.test.ts`, `spec.json`, `ui-smoke-inventory.md`. `CharacterSheet.tsx` has this card in the writer order |
| Technical design §5: `list_progression_ledger -> Vec<LevelEntry>`, `characterProgression.ts` logic moves server-side | A row with no level (SD-o) has no place in a list of levels and must not be lost (R9), and Create, the level-up preview and print still call that logic | The command returns `{levels, withoutLevel}` of its own rows. `characterProgression.ts` stays, and gains the words the drawer prints. Print keeps its rail history until E8.4 |
| E5.3's row: per level, no hit points | §11 lists "HP (if recorded)", and `hit_points.json` records a die result per class level | The row carries `hitDieResult`, the saved die for that class level, and states nothing when none was saved. It is the die as saved, not the level's HP: the sheet adds Constitution |
| §11: "class features gained" | The engine's `class_feature.*` explanations are mostly `corpus_record` rows read from `data/corpus/*/class_feature`, and the app's bundle holds no such directory (`find apps/desktop/src-tauri/resources/corpus_bundle -maxdepth 3 -type d -name class_feature \| wc -l` → 0): tests saw Cantrips, a Wizard in the app did not | Features are the sheet's own `class_feature` lines at each level, which `data/sheet_rules` ships: a rule when first held, then any of its lines whose printed value moves. A Wizard's bonus feat at 5 prints no such line, so it reads as unspent feat slots, not a feature |
| §11: "unspent slots", with an ability increase among the rows | The Pathfinder sheet has no ability-increase pick (row above on E5.1), so calling every 4th level unspent would be true of every character and spendable by none | Unspent counts feats, spells of a class with a known table, and ranks. A level granting an increase with no row reads `Ability increase: none recorded` |
| §11 and SD-o: which level's slot is unspent | The budget charges a feat to a pool, not a level, and a choice made late is stamped late | Totals are the budget's and the ranks' own; the open slots are the latest ones, since a choice takes the earliest free slot (E5.2). So the drawer's unspent feats always sum to the room the sheet's badge reads, and a level can show a feat and an open slot both |
| E5.3's row: the saved hit die, with E5.1's carry naming only `ledger.json` | The sheet's hit points and the drawer's hit die both read `hit_points.json` (`maxHitPointsWithSavedLevels`, `list_progression_ledger_at_root`), and `clone_character` and `ImportedCharacterFileDto` lost it, so a clone's hit points and die changed from its source's | E5.MC carries it through both, as `custom.json` and `ledger.json` are carried. A file with none is the file it always was |
| E5.1: `ledger.json` written with `std::fs::write` | A ledger that cannot be read refuses every change (this section, E5.2's row), so a write cut short locked a character out of editing with no way back in the app | The write goes to `ledger.json.tmp` and renames. `custom.json` and `hit_points.json` still write in place; neither was in this card's attack |
| §10, E6.1's row: each enforcing command reads the flag; proficiency in `equipment_resolver.rs` | Prerequisites, budget and slots refused; a Known spell refused for a Wizard and a few others. Weapon, armor, alignment and encumbrance refused nothing (`git show c94921873f:src/rules_core/equipment_resolver.rs \| grep -ci proficien` → 0; `converted_gate.rs` reports alignment "not verified"), so a flag alone would have been a toggle nothing read. And the engine blocks a Wizard's off-list spell on every compute, so admitting one at the command alone would break the character | E6.1 builds the five refusals: wear and buy for weapons and armor (a buy stays carried with the reason, R18), a level-up or a feat against the bio's alignment, a load past heavy capacity at the add and the buy, a spell off the class list for every class that casts. A spell the rules refuse under the flag saves with `source_class_id` `override:spells_outside_class_list`, which no per-class rule reads; E6.2's tag for a spell can read that. An unknown never refuses: no stated alignment, a class with no record, a weapon outside the table |
| E6.2's row: the selection record carries `override: Some(flag)`; FILES YOU OWN: `OverrideTag.tsx`, selection DTOs, the tabs, `EquipmentPanel.tsx`, the `list_held_equipment` row, `spec.json` | `selected_feats` is a list of strings, so a feat has no record to carry a field, and `EquipmentSelection` has 102 literal sites (`grep -rn 'EquipmentSelection {' src apps/desktop/src-tauri/src crates tests \| wc -l`). The print module does not exist yet (`ls apps/desktop/src-tauri/src/print` → no such directory); `technical-design.md` names `LoadSavedCharacterResponse` as its input. No tab lists a class level, and the alignment flag admits one. The picker's Add sits behind the Traits section's own `Add`, and `run.mjs` dropped a click's `index` | The record is a row in `ledger.json` (`admitted`, one per record and flag), written with the change and held to what the character holds, so it survives a clone and a file, as the ledger does (`character_ledger.rs`). `admittedUnderOverride` on the load is what E8.2 reads, and the held row carries `override`. A spell's row comes from its source (E6.1's row). The Custom tab's Overrides group lists class levels. Amended row, named fence exits (§23): `character_ledger.rs`, `pf1_adapter.rs`, `character_custom.rs`, `character_hub.rs` (E6.2 after E6.1 in the writer order), `CharacterSheet.tsx`, `CustomTab.tsx`, `boundary/loadSavedCharacterDetail.ts`, `boundary/listHeldEquipment.ts`, the response literal in `cpr_adapter.rs`, `sf_adapter.rs`, `rule_system_adapter.rs` (the field is left out when empty, so no seed moves), `scripts/ui-smoke/run.mjs`, `ui-smoke-inventory.md` |

**Does not cover:** whether every tag maps to the right slot on every book. E1.0's test names the shapes it checks and
lists the rest as counts.
