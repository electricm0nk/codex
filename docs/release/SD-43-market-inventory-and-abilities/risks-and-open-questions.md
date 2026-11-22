---
canonical: true
owner: operator
bundle_id: SD-43
slug: market-inventory-and-abilities
canonical_branch: tranche-43
target_version: 0.43.0
date: 2026-11-22
status: planning — mirrors decisions.md §30 (open questions) and §31 (corrections); decisions.md wins where they differ
---

# SD-43 Risks and Open Questions

## 1. Ranked risks

| # | Risk | Likelihood | Impact | Control | Self-healable? |
|---|---|---|---|---|---|
| R-1 | **The CI secret `CODEX_REPORT_KEY` does not exist** (P-8, Q-13.2): `gh secret list -R electricm0nk/codex \| awk '$1=="CODEX_REPORT_KEY"' \| awk 'END{print NR}'` → 0 on 2026-11-22. Without it the release ships Send disabled under Anonymous, so DoD row 19 is not met | certain today | high (an intake item does not work for testers) | **Escalation, not deferral.** E5.1 lands and proves the dev-build "unavailable" path; G-R's secret line is part of E7.2's final-acceptance scan; if it prints 0 then, E7.2 files an `## Open blockers` entry naming the one precondition ("the operator creates the `CODEX_REPORT_KEY` repository secret") and the bundle pauses before the retrospective and the PR. The lead asks for it at launch so it is not discovered at closure | no — operator action |
| R-2 | **E4.G operator wait** (names approval and the reading of "1", Q-20.1) stalls E4.2 and therefore E7 | expected | medium (wall time) | the script ends the run cleanly with `START=E4.2` after every non-dependent card finishes (`workflow-instruction.md §2.4` step 6); E4.1 runs as early as its dependencies allow so the question reaches the operator while E6 still runs | n/a — a planned wait |
| R-3 | **The abilities engine gap is larger than one epic can hold.** The size is unknown until E0.1: how pool sizes per level are stated (token shapes), how many `PRE*` shapes pool members use, what Background and Misc hold. 1,913 pool groups / 16,348 records are in scope for E3.4/E3.5 (`python3 scripts/census_class_feature_pool_population.py`) | medium | high | E0.1 measures before E3 starts; E3.4's split rule files a discovered card per extra token shape; E3.5's unknown-shape count must be 0 at E3.MC or each shape is a discovered card; discovered cards run before E3.MC. A gap is cleared or escalated, never shipped as a notice (`decisions.md §18`) | yes — by decomposition |
| R-3a | **The Class Features scope is wider than the pool catalog** (intake §22c, 2026-11-22: every class- or archetype-granted choice, the named pools are examples). Open check: choices not modeled as a catalog pool today (bloodline, domain, school, mystery, order picked at level 1) may sit outside the 1,913-group count | high | high | E0.1 enumerates every class-granted choice by mechanism (two implementations) and states in/out of the 1,913 groups; E3.MC proves each mechanism reachable (selectable + recorded) or names a discovered card; "not reachable" is a number closed before E7.2, never an exemption (`acceptance-and-verification.md` G-E3 E3.MC (22c)) | yes — by decomposition |
| R-4 | **Q-22.3 is answered "a new tab beside Feats" after E3.1 has landed the rename** (the operator's original words: "a tab next to feats, maybe call it abilities") | medium | medium (rework on E3.1 and the sub-tab host) | the lead asks Q-22.3 at launch, before E3.1 can dispatch (E3.1 waits on E0.1 and E2.5); E3.1's criterion reads correctly under both answers (`acceptance-and-verification.md` G-E3); an overrule after landing is a rework card | yes — lead action |
| R-5 | **Shared-checkout collisions** (the largest incident class of tranche/7: 10 of 34, per `AGENTS.md` Concurrency and Measurement, citing `docs/retro/tranche-7-retrospective.md` §6.1) — seven lanes, a peer session on the same box | medium | high | one writer per tree; every lane its own worktree + `CARGO_TARGET_DIR`; the `hub` token for the three shared files; `spec.json` only by E7.1; `git status --porcelain`, `git reflog -3`, `git worktree list` before every dispatch; the prefix's wrong-base command refuses the lead's checkout; never `git stash`; never `git add -A` | no — a second writer on one file is `blocked-escalated` |
| R-6 | **`docs/release/current-release.json` still names tranche 22** and `resolve_release_notes.py` exits 2, so any PR or develop publish goes red (`check-release-manifest.yml`, `release-tooling-tests.yml`, `publish-tester-release.yml` call it) | certain until C1 | high | **C1 lands before any push other than C0 and before the first PR**; C1's acceptance is the resolver's exit 0; every lane's tree is cut from `origin/tranche-43` after C1's push | yes |
| R-7 | **OOM on the box** (Proxmox stops the VM on guest OOM) with up to six cargo-capable lanes | medium | high | the script's cargo semaphore = 3; the prefix's `pgrep -x cargo` < 3 wait; `-j 8`, `--test-threads=8`; one cargo process per lane | yes |
| R-8 | **An acceptance command passes on the base** and proves nothing (SD-39…SD-42) | high | medium | RED before dispatch, recorded in the dispatch log; the script refuses a card whose RED exited 0 | yes |
| R-9 | **E0.2 cannot reproduce the operator's Market figures** (Magic 3,680; Gear 1,624; Weapons 480, read off the screen) | medium | medium | the card stops and reports — the test is wrong, not the screen; the filter work (E1.10) waits on reconciled counts | yes |
| R-10 | **The data-gap notice count disagrees with E0.1's list at E3.MC** (a hidden gap, or an engine gap dressed as a data gap) | medium | high (a faked tab) | the two numbers side by side in E3.MC's receipt; the `engine card` grep → 0; `decisions.md §18`'s two kinds kept apart | no — a mismatch is a defect card |
| R-11 | **A new runtime data read is missing from the package** (the rest table E2.4; the name table E4.2) — the 2026-10 class-roster defect shape | medium | high | FILES YOU OWN includes `tauri.conf.json` `bundle.resources` + `tests/packaged_resources.rs`; G-R runs `packaged_resources` and `no_baked_manifest_dir_in_production`; fresh-install check at E7.2 | yes |
| R-12 | **The report key leaks** (repo, log, receipt, web view JS) | low | high (public repo) | key read in Rust only; NR-8 greps; secrets never printed (prefix); a leak is an incident and the key is rotated by the operator | no — incident |
| R-13 | **A key in a shipped app can be extracted** (intake) | certain in principle | medium | codex-web's rate limits are the protection; stated in the release notes' Known Issues | n/a — accepted |
| R-14 | **Graphify `--force` or `graphify update` replaces the semantic graph** (memory: 648k → 52k nodes) | low | high | C2 and E7.4 never pass either; node counts before/after in the receipt; a guard exit 1 = file the receipt and stop; restore from `graphify-out/<date>/` | yes |
| R-15 | **Fresh-install acceptance cannot run on this box** (root for `dpkg -i`, a display for the app) | medium | medium | the run-desktop harness provides the display; `dpkg-deb -x` fallback stated with what it does not prove; if neither works, escalate — the run may not move the check to the post-merge alpha on its own (that was an operator ruling in SD-42) | partly |
| R-16 | **The windowed Market list breaks keyboard selection, sort or the description box** | medium | medium | E1.11's acceptance names each behaviour as a test; no new dependency unless the fixed-height slice fails (licence read first) | yes |
| R-17 | **A red develop publish after the merge** | low | high | E7.4 watches it; a red publish is a blocker (`blocked-escalated` with the failing job), not a follow-up | no — fix forward on a new branch with the operator told |

## 2. Open questions → defaults taken (mirror of `decisions.md §30`)

The run takes each default and logs it; the operator may overrule any before or during the run. An overrule after
the card has landed is a rework card.

| ID | Question | Default taken | Settled by |
|---|---|---|---|
| Q-1.1 | Move or copy on relocation? | copy, verify bytes, switch; old folder kept | operator |
| Q-1.2 | New folder already holds content? | refuse unless its `campaign.json` has the same campaign id; then open it, copy nothing | operator |
| Q-1.3 | Keep or remove the old folder? | keep; the screen names it | operator |
| Q-2.1 | Source of the rest text | a committed table, one cited row per class/race rule, text from the book; E0.5 census first | E0.5 + operator review in SD-44 |
| Q-2.2 | Override shape | one free-text field in `custom.json` replacing the whole text | operator |
| Q-3.1 | House-rule HP at level 1? | no; level 1 stays maximum | operator |
| Q-3.2 | Rounding for Average and Percentage | down; "round up" is its own option; Percentage floor 1 | operator |
| Q-3.3 | Rolled result kept or re-rolled on regeneration? | kept, stored at level-up | operator |
| Q-3.4 | The four existing flags also in the Custom dialog? | no; House Rules is their one home | operator |
| Q-3.5 | Which House Rules show for Starfinder | those whose rule exists in SF, listed by E0.5 | E0.5 |
| Q-4.1 | One file per item or many? | one file, many items | operator |
| Q-4.2 | Where in the campaign folder? | `Custom Items/` at the root, added to the skeleton | operator |
| Q-4.3 | Cross-system import refused? | yes, with a message | operator |
| Q-5.1 | Item into a stacked empty container | the copy splits off first | E1.1 test |
| Q-5.2 | Emptied container re-merges? | no | operator |
| Q-6.1 | No free slot: first `WORN_SLOTS` slot it fits? | yes | operator |
| Q-8.1 | Weapons pricing control removed too? | yes | operator |
| Q-8.2 | SF/CPR Print button placement | moves into a ☰ menu only if E2.2's shared layout gives them one; else stays | E2.2 |
| Q-9.1 | Sell window returns money when the Market is Cashless? | yes; its choice is its own | operator |
| Q-10.1 | Stats order | stats first, then prose | operator |
| Q-10.2 | Missing stat hidden or `—`? | `—` | operator |
| Q-11.1 | App counts for Weapons, Armor, Shields | E0.2's test produces exact values | E0.2 |
| Q-12.1 | Drop / Discard of a container move its contents too? | unchanged in SD-43; a yes from the operator becomes a card | operator |
| Q-12.2 | Cashless value in the Market inventory | `—` | operator |
| Q-12.3 | Not Carried contents of a sold container | stay Not Carried | operator |
| Q-13.2 | Key into release builds; dev builds | CI secret `CODEX_REPORT_KEY` stamped at build; dev builds disable Send with a reason; **the operator creates the secret** (R-1) | operator |
| Q-13.3 | Diagnostics attached? | version, OS, install kind, channel | operator |
| Q-17.1 | Side columns scroll on their own? | yes | operator |
| Q-19.1 | Embed the diagnostic or open it from a button? | embed as a section | operator |
| Q-20.1 | Confirm the reading of "1" | operator proposes-and-approves as written | operator, at E4.G |
| Q-22.1 | "SD23"/"SD24" read as SD-43/SD-44 | yes | settled |
| Q-22.2 | PCGen sub-tab → data category mapping; Background and Misc | E0.1 reads PCGen's UI strings at the pin | E0.1 |
| **Q-22.3** | **One Abilities tab replacing Feats, or a tab beside it? — PENDING operator ruling.** The operator's words were "a tab next to feats, maybe call it abilities"; part 1 took the rename as default, which the operator has not approved | one tab, Feats as sub-tab 1 (default, used only if no reply before E3.1 dispatches); the alternative keeps `Feats` and adds `Abilities` beside it with the six other sub-tabs | **operator — asked at launch** (R-4) |
| Q-22.4 | Overrides on the six new sub-tabs? | yes, two new flags | operator |
| Q-22.5 | Retraining and replace-a-pick excluded? | excluded (`forward-scope-register.md` DEF-1, DEF-2); advanced/master tricks still selectable as members | operator |
| Q-22.6 | Abilities tab on SF and CPR sheets? | no; Pathfinder only (DEF-3) | operator |
| Q-22.7 | Where the favored class section sits | Class Features sub-tab | operator |

## 3. Corrections to the intake from repo evidence (mirror of `decisions.md §31`)

| ID | Intake said | Repo shows |
|---|---|---|
| C-1 | Only two pools are registered (Rogue Talent, Rage Power) | the backend catalog no longer filters by that list; the two-pool limit is the UI's `POOL_REFERENCE_SECTIONS`; 1,913 groups / 16,348 records / 7,328 text-only |
| C-2 | Ninja Trick lives at `ultimate_campaign/ability/ninja_trick.json` | that file is the retraining record; the pool is `ultimate_combat/class_feature/ninja_trick/`, 45 records |
| C-3 | No menu entry opens the print dialog | the PF sheet's ☰ menu has Print (`CharacterSheet.tsx:4153`) |
| C-4 | SD-43 is built on `tranche-23`; Cargo.lock unchecked | branch `tranche-43`; the lock reads `0.43.0` |
| C-5 | The signed-manifest work is a build inside SD-43 | the brief places itself outside SD-43 and is merged (PR #419) |
| C-6 | Where custom options are stored is unverified | per character, `custom.json` `Overrides`; four of six house rules already exist |
| C-7 | "18 distinct patterns" | 18 distinct name strings; same figure |
| C-8 | `ABILITYCATEGORY` 2,616 lines / 2,430 names | not re-derived (oracle not resolved in the authoring shell); E0.1 re-runs it |

One further precision this back half adds (not a contradiction): the all-kinds denominator 51,474 is **records
with a `data` object** (`content-unit-inventory.md §3`); 48,941 of them carry a string `data.name`.

## 4. Non-self-healable conditions (card → `blocked-escalated`; a filed blocker pauses, never closes)

Two live writers on one file; RED → GREEN not preserved; a stub, mock or "Would …" string in shipping code; an
engine-gap notice at E3.MC; data-gap notices ≠ E0.1's list; the CI secret absent at E7.2 (R-1); a red publish at
E7.4; fresh install not runnable after both forms were tried (R-15); `CLAIM-EXISTS`; an unattributed
`test result: FAILED`.
