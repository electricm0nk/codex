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

**Measured (`python3` over `data/sheet_rules/*/equipment/*.json`, 2026-11-01):** 7,386 records, 2,906 with a slot
tag, 1,617 with a `Slot:` line, 27 where the two differ (E1.0; the 19 first written here left out the 8 `Slot: none` weapons); the prose wins. Items with neither have no slot.

**Slot vocabulary.** The operator said a ring takes "a hands slot". The book and `equipmentslots.lst` give rings
their own slot, two of them. SD-39 follows the book; a change is one table row.

**Does not cover:** whether every tag maps to the right slot on every book. E1.0's test names the shapes it checks and
lists the rest as counts.
