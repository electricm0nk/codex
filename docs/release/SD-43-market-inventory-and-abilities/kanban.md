---
canonical: true
bundle_id: SD-43
slug: market-inventory-and-abilities
canonical_branch: tranche-43
target_version: 0.43.0
date: 2026-11-22
board: local-file ./kanban.md
status: planning — every card `todo` at authoring
---

# SD-43 Kanban

One row per card of `epic-breakdown.md §0` (one criterion per card). The dispatched agent that closes a card edits
that card's row in its own closing commit; a row moves to `complete` only with its receipt path and commit SHA in
the last column.

**Status vocabulary:** `todo` · `in-progress` · `complete` · `partial` (blocks closure) · `blocked-escalated`
(an operator ruling is needed; pauses the card and its dependants; blocks closure) · `declined` (premise refuted or
dependency not complete; re-dispatched after the lead's correction).

**Column order is load-bearing:** the closure scan reads ID as awk field `$2`, Tier as `$4`, Status as `$5`.
**`Depends on` is load-bearing:** the script dispatches a card only when every dependency is `complete`.
**Tier** is the `model` the script sets (`opus` = Opus 5.5, `sonnet` = Sonnet 5.5, `haiku` = Haiku 4.5); `lead` and
`operator` are never dispatched as work. **Closure rule:** no PR while any row other than E7.4 is anything but
`complete`; a filed blocker never satisfies it. **One operator wait:** E4.G.

**Count and same-ID checks** (run from the package directory):

```bash
cd docs/release/SD-43-market-inventory-and-abilities
awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'          # 56 at authoring
diff <(awk -F'|' '$2 ~ /^ (C|E)[0-9]/ {gsub(/ /,"",$2); print $2}' kanban.md | sort) \
     <(awk -F'|' '$2 ~ /^ (C|E)[0-9]/ {gsub(/ /,"",$2); print $2}' epic-breakdown.md | sort) && echo SAME_IDS
awk -F'|' '$2 ~ /^ (C|E)[0-9]/ {print $5}' kanban.md | sort | uniq -c   # 56 todo at authoring
```

| ID | Title | Tier | Status | Depends on | Lane | Cargo | hub | Receipt / notes |
|---|---|---|---|---|---|---|---|---|
| C0 | This package (front + back half) committed; placeholder gate run | lead | complete | — | lead | no | no | front half `75e95b30c2`; back half = this commit; the lead marks `complete` after reviewing both — reviewed 2026-11-22 at `64432acc84`; receipt `progress.md` Cycle 0 |
| C1 | Interim `release-notes.md` + `current-release.json` → tranche 43, one commit; branch pushed | haiku | complete | C0 | main | no | no | RED: `resolve_release_notes.py` exit 2 (2026-11-22); receipt `artifacts/cycle_0/C1_cycle_receipt.md`; SHA `c422d005ee4c39a9e144dc038da9cee8703bf6c5` |
| C2 | Incremental graphify after SD-42 (no `--force`, no `graphify update`); receipt | sonnet | complete | C1 | main | no | no | OUT OF SCOPE — operator ruling 2026-11-22: "the graph refresh is out of scope"; nothing written to graphify-out; attempt receipt `artifacts/cycle_0/C2-graphify-receipt.md` (SHA `597ff805dc2beecf1097ec87e9069cdd122733d1`) |
| E0.1 | Abilities census: seven sub-tabs, every class-granted choice by mechanism (§22c), pools, saved fields, budgets, prereqs, pool sizes per level, PCGen mapping, data-gap list | opus | complete | C1 | census | no | no | settles Q-22.2; RED `abilities_census.py --check` exit 2 (no such file); GREEN exit 0; 1,329 class-granted choice sites (A=B); 34 data gaps (30 ingest → E3.12); receipt `artifacts/epic_0/E0.1_cycle_receipt.md`; SHA `d9c84afeaf037a84a04700c640447e1a1f935c14` |
| E0.2 | Market count reconciliation test over `built_catalog()` | opus | complete | C1 | market | yes (desktop) | no | settles Q-11.1 (Weapons 480, Armor 63, Shields 31, Gear 1,624, Magic 3,680; sum 8,119); receipt `artifacts/epic_0/E0.2_cycle_receipt.md`; SHA `29132b1d71d77d03d13966d107aaa2fe1099c4ea` |
| E0.3 | Placeholder-name scan widened to every kind and surface | sonnet | complete | C1 | census | no | no | 111 of 3,805 / 201 of 51,474 reproduced; receipt `artifacts/epic_0/E0.3_cycle_receipt.md`; SHA `b13403a9610f8e877f893701add64bb5b97234d4` |
| E0.4 | Reproduce items 5, 7, 12 on a fresh Brother Tomas Vane | opus | complete | C1 | gear | yes (app build) | no | RED: `test -f …/gear-repro.md` exit 1 (2026-11-22); receipt `artifacts/epic_0/E0.4_cycle_receipt.md`; SHA `952d79c7e1af9e2fca2868391d8cad4288e4d059`; discovered E1.12 |
| E0.5 | Rest-rule census (classes, races); House Rules per system (PF, SF, CPR) | sonnet | complete | C1 | census | no | no | settles Q-2.1 input, Q-3.5; receipt `artifacts/epic_0/E0.5_cycle_receipt.md`; SHA `e0c96c5ea6bb7ec0d698c6c893d7b30e04fb1ac6` |
| E1.1 | Container truth: no `#N`, custom names, filled container never stacked, split on put-into-stack | opus | complete | E0.4 | gear | yes (desktop) | hub | receipt `artifacts/epic_1/E1.1_cycle_receipt.md`; SHA `210eeccc3e6f39c5772488ba9c0235d8b95ede62` |
| E1.2 | One-column gear list: order, natural sort, nested boxes, Not Carried section | opus | complete | E1.1 | gear | no | no | receipt `artifacts/epic_1/E1.2_cycle_receipt.md`; SHA `adebea8a631b8b9ef47e011107acdf5c9488c7a0` (r3 receipt repair; work `56e28e711f78ba190ebdc5390a7b167c374d48b0`) |
| E1.3 | Whole container box is a drop target (nearest container) | sonnet | complete | E1.2 | gear | no | no | receipt `artifacts/epic_1/E1.3_cycle_receipt.md`; SHA `1a72bf730baad31aad1b8a5d4d3d3af4dc774b88`; repair SHA `f1090131dfea7c3af48c2346682bcc864219cfe9` |
| E1.4 | Double-click / Enter: first free slot, else swap; to Carried; one piece | opus | complete | E1.3 | gear | no | no | receipt `artifacts/epic_1/E1.4_cycle_receipt.md`; SHA `e761ee7072a94a26dc50766c7872d1adf20ea207` |
| E1.5 | Selling a container moves its direct contents to Carried | opus | complete | E1.4 | gear | yes (desktop) | hub | receipt `artifacts/epic_1/E1.5_cycle_receipt.md`; SHA `d72cf744c7d897e1316c2b29239ad29902ee8026` |
| E1.6 | Sell window: Market Rate / Build / Negotiate | opus | complete | E1.5 | gear | no | no | receipt `artifacts/epic_1/E1.6_cycle_receipt.md`; SHA `fffb58ccf515ed741f3c8ce15abf4739fda47669`; discovered E1.13 |
| E1.7 | One `MarketButton` on every panel (PF, SF, CPR); Print and pricing control leave the panels | sonnet | complete | E1.6 | sheet | no | hub | receipt `artifacts/epic_1/E1.7_cycle_receipt.md`; SHA `917043576d7e5735797e496c416812f34fccbc7a` |
| E1.8 | Market inventory sell value column (`sellNegotiateGp`) | sonnet | complete | E0.2 | market | no | no | receipt `artifacts/epic_1/E1.8_cycle_receipt.md`; SHA `7adf7e494d1b376c76ba01a0ee41e66721bf6d79` |
| E1.9 | Item stats in the Market description box (engine-stated, shared renderer) | opus | complete | E1.8 | market | yes (desktop) | hub | receipt `artifacts/epic_1/E1.9_cycle_receipt.md`; SHA `7701cae42e4cfd50140833f8ad06c82bf45913a1` |
| E1.10 | Filter model: per-list columns and values, `Other`, counts cover every row | opus | complete | E1.9, E0.2 | market | no | no | receipt `artifacts/epic_1/E1.10_cycle_receipt.md`; SHA `a1b361d7df3ce84762680108ffcc8a9c752d8164` |
| E1.11 | Filter column; 200 cap removed; windowed list; first paint measured | opus | complete | E1.10 | market | no | no | receipt `artifacts/epic_1/E1.11_cycle_receipt.md`; SHA `d171f57676ecf6ef1810ab9da6fecd90cae76525` |
| E1.12 | Sell removes the copy the player clicked (copy key, not item id + state) — discovered by E0.4 | opus | complete | E1.5 | gear | yes (desktop) | hub | discovered E0.4 (`artifacts/epic_0/gear-repro.md` §12.2); receipt `artifacts/epic_1/E1.12_cycle_receipt.md`; SHA `52d1ff8f0fd3f707768867154e59d61c9393ced6` |
| E1.13 | Gear sell window's choice holds under a Cashless session (Q-9.1): `handleGearSell` sends a non-cashless mode when the window sends a total — discovered by E1.6 | sonnet | complete | E1.6 | gear | no | hub | receipt `artifacts/epic_1/E1.13_cycle_receipt.md`; SHA `7d4d8cf3fe925ce770b1b13662221bd6f754b4c0`; discovered E1.6 |
| E1.14 | Market inventory sell value and Negotiate prefill read every held copy's catalog row, not only the lists already opened (`rowOfCopy` `MarketDialog.tsx:702`); a priced item never shows `—` — discovered by E1.MC | sonnet | complete | E1.8 | market | no | no | receipt `artifacts/epic_1/E1.14_cycle_receipt.md`; SHA `d3c2ca789afefbbfbcf6f09cee50202596f922f1` (code 8b063d7a42; live look r6); discovered E1.MC |
| E1.15 | Gear Sell opens its window on the first click after launch (E1.6 and E1.MC both saw the first Sell clicks open nothing while Customize opened) — discovered by E1.MC | opus | complete | E1.6 | gear | yes (desktop) | hub | receipt `artifacts/epic_1/E1.15_cycle_receipt.md`; SHA `841b0e721287d7fae2220054590bc91a02f7cebb`; RE-SCOPED by lead 2026-11-23 (hub=yes, cargo=desktop); `describe_held_equipment` → `#[tauri::command(async)]`; discovered E1.MC; retro key `gear-sell-first-click-no-window` |
| E1.MC | E1 merge check + epic report | opus | complete | E1.7, E1.11 | gear | yes (base build) | no | receipt `artifacts/epic_1/E1.MC_cycle_receipt.md`; SHA `b76a410c4d5e00df57da5f4fbad2daf9e62f8757`; discovered E1.14, E1.15 |
| E2.1 | Fixed layout on the PF sheet; Show Progression visible (items 17 + 21) | opus | complete | E1.7 | sheet | no | hub | receipt `artifacts/epic_2/E2.1_cycle_receipt.md`; SHA `10aefca95f0cec754665263d2028fa2fd49a4367` |
| E2.2 | The shared layout on the SF and CPR sheets | sonnet | complete | E2.1 | sheet | no | no | receipt `artifacts/epic_2/E2.2_cycle_receipt.md`; SHA `79f355f04ba4e4373fb5b7cc3f49be3bd86752f2`; settles Q-8.2 |
| E2.3 | Open campaign button opens the folder; one banner per campaign; Manager banner smaller | opus | complete | E2.2 | sheet | no | hub | receipt `artifacts/epic_2/E2.3_cycle_receipt.md`; SHA `85af984e595df395a86f2a12047e39f8e4861170` |
| E2.4 | Rest and recovery: cited table, engine command, print block, override field | opus | complete | E0.5, E2.3 | sheet | yes (root + desktop) | hub | receipt `artifacts/epic_2/E2.4_cycle_receipt.md`; SHA `4caf5d7df78eb51fb7e62a6d2bbfa7c165d8fc5e` |
| E2.5 | Rest text read-only under the Biography text | sonnet | complete | E2.4 | sheet | no | hub | receipt `artifacts/epic_2/E2.5_cycle_receipt.md`; SHA `74cb141f8877f85038de2c769aebf0808a75a4e0` |
| E2.6 | Rest-and-recovery override editor: the Custom dialog writes `custom.json` `restText` (decisions.md §8 "in the custom rules menu"); clearing it restores the table text — discovered by E2.MC | sonnet | complete | E2.5 | sheet | no | hub | receipt `artifacts/epic_2/E2.6_cycle_receipt.md`; SHA `7ad94c33bed994c9311f86cbe60d706126bfbaa9` (code a437accada; screens repair r6); discovered E2.MC (`artifacts/epic_2/E2.MC_cycle_receipt.md`, "Discovered cards"); retro key `dod-clause-without-owner-card` |
| E2.MC | E2 merge check + epic report | opus | complete | E2.5 | sheet | yes (base build) | no | receipt `artifacts/epic_2/E2.MC_cycle_receipt.md`; SHA `8b84bc240323d132d5eb996f41a0297b67455e3a`; discovered E2.6 |
| E3.1 | Rename the Feats tab to Abilities (operator ruling Q-22.3, 2026-11-22): seven sub-tabs with Feats as one of them, existing surfaces moved, no behaviour change | opus | complete | E0.1, E2.5 | sheet | no | hub | receipt `artifacts/epic_3/E3.1_cycle_receipt.md`; SHA `23f5984466610573b3ada4acda5e3a45f72b98b1` |
| E3.2 | Engine: generic ability-budget module, saved selections, list/add/remove commands; proved on Traits | opus | complete | E3.1 | sheet | yes (root + desktop) | hub | receipt `artifacts/epic_3/E3.2_cycle_receipt.md`; SHA `110abeaa104f325c42ae70d1588782daac520ab9` |
| E3.3 | Generic sub-tab UI; Traits sub-tab on it | opus | complete | E3.2 | sheet | no | hub | receipt `artifacts/epic_3/E3.3_cycle_receipt.md`; SHA `260275938f1bdb14f141f0db3bc30d7f002e9517` |
| E3.4 | Class Features: grants per level for every catalog pool | opus | complete | E3.3 | sheet | yes (root + desktop) | hub | receipt `artifacts/epic_3/E3.4_cycle_receipt.md`; SHA `1391c72f1ea5b393f3b750c83f0e222fe5afe8d0`; discovered E3.13, E3.14 (split rule) |
| E3.5 | Class Features: pool-member prerequisite evaluator and refusal reasons | opus | complete | E3.4 | sheet | yes (root + desktop) | hub | receipt `artifacts/epic_3/E3.5_cycle_receipt.md`; SHA `9ee4fe96de5db045a6260a80afe4dcd6723ef697`; coverage 7,657 members evaluated, unknown token shape 0; discovered E3.15 |
| E3.6 | Racial Abilities: alternate racial traits on the sheet; replaces-conflict refused | opus | complete | E3.5 | sheet | yes (root + desktop) | hub | receipt `artifacts/epic_3/E3.6_cycle_receipt.md`; SHA `f014e02fc7795f319e0a949453368a901fdd9816` |
| E3.7 | Archetype: select per class; slot conflicts refused; grants feed pools | opus | complete | E3.6 | sheet | yes (root + desktop) | hub | receipt `artifacts/epic_3/E3.7_cycle_receipt.md`; SHA `a85085a55ed73e07bb9318e0fb3ab73d57cc1c8c`; discovered E3.16 |
| E3.8 | Favored class bonus: class pick, per-level HP / skill / race alternates; engine totals | opus | complete | E3.7 | sheet | yes (root + desktop) | hub | receipt `artifacts/epic_3/E3.8_cycle_receipt.md`; SHA `c6a8b9f3fd98b1caa9fe930c550368e5b2d0d3f8` |
| E3.9 | Background and Misc sub-tabs per E0.1's mapping | opus | complete | E3.8 | sheet | yes (root + desktop) | hub | receipt `artifacts/epic_3/E3.9_cycle_receipt.md`; SHA `bb7d887eeb8f568fc4f932cdee45b74384b0a7f9`; discovered E3.17 |
| E3.10 | Overrides on the six new sub-tabs (two flags) | sonnet | complete | E3.9 | sheet | yes (desktop) | hub | receipt `artifacts/epic_3/E3.10_cycle_receipt.md`; SHA `9be9340ad3e5392c2f583aa6652440e21d9b0871`; repair `5ea723a192d688547953493183245d6fd0124e16` |
| E3.11 | Printout: chosen abilities per sub-tab and the favored class bonus | opus | complete | E3.10 | sheet | yes (desktop) | no | receipt `artifacts/epic_3/E3.11_cycle_receipt.md`; SHA `5c0fcbfbe8b6f2330d41b7a6fd73ff3b0d4b3776` |
| E3.12 | DISCOVERED (E0.1): ingest the oracle's member typing (`<record>.MOD TYPE:` lines) so the 30 ingest-gap pools in E0.1's data-gap list get members; add the missing `Ninja ~ Ninja Trick` grant row (level 2) to `data/class_feature_grants`; resolve the 7 pool names with no ABILITYCATEGORY | opus | complete | E0.1 | sheet | yes (root) | no | receipt `artifacts/epic_3/E3.12_cycle_receipt.md`; SHA `233dcf906fcd25209648ccc1f00471eebf839706`; data_gap_list 34 -> 3, all book gaps (Canny Foe, Swift Powder Choice, Viking Bonus Feat; the card's '4' and '30' were E0.1 census errors, corrected); E3.MC's notice count = 3; discovered E3.18, E3.19 |
| E3.13 | DISCOVERED (E3.4): level-1 `ABILITYSELECTION` picks — the 5 bloodline pools whose size is an `offers` count with no `Pool` target — get `abilities.budget.class_features.*` rows and picks | opus | complete | E3.4 | sheet | yes (root + desktop) | hub | receipt `artifacts/epic_3/E3.13_cycle_receipt.md`; SHA `b75ebe66d4f149cd4cee6d10d4499e2a9186d851`; discovered E3.20 |
| E3.14 | DISCOVERED (E3.4): the 31 class-feature pools whose converted grants carry no `offers` list their members (converter carries the category's member filter) | opus | complete | E3.4 | sheet | yes (root + desktop) | no | receipt `artifacts/epic_3/E3.14_cycle_receipt.md`; SHA `5f42b51cd708c91945b7c255338156abb29e7614`; members listed 148 of 148 granted class-owned pools, 31 of 31 no-offer pools |
| E3.15 | DISCOVERED (E3.5): Class Features picks held by the sheet — `HeldSeed::from_record` seeds saved `ability_selections` (Class Features) so a pick's own rules reach the sheet and its print, not only the options and the add path | opus | complete | E3.5 | sheet | yes (root + desktop) | no | receipt `artifacts/epic_3/E3.15_cycle_receipt.md`; SHA `fa2ee038e3537a7bfd74f8be96b02220d8f28778` |
| E3.16 | DISCOVERED (E3.7): a held archetype held by the sheet — `HeldSeed::from_record` seeds the saved `choice:archetype` records so a replaced feature's sheet lines and print follow (Two-Handed Fighter still shows the Bravery note) | opus | complete | E3.7 | sheet | yes (root + desktop) | no | receipt `artifacts/epic_3/E3.16_cycle_receipt.md`; SHA `3e6b68938c535e3032ffb005d060f86dd05af9f8` |
| E3.17 | DISCOVERED (E3.9): Background and Misc picks held by the sheet — `HeldSeed::from_record` seeds the saved `ability_selections` of the `background` and `misc` sub-tabs so a pick's own rules reach the sheet and its print (a Fleshforge Enhancement's +2, a corruption's text), not only the options and the grants | opus | todo | E3.9 | sheet | yes (root + desktop) | no | must be complete before E3.MC; E3.9 receipt "Does not cover" (b), `ability_budget::picked_rules` |
| E3.18 | DISCOVERED (E3.12): the E3.12 pools whose converted grants carry no offer selecting a member (proxy `artifacts/epic_3/E3.12/offer_probe.py` → 10 of 29) list their members on the sheet; engine test per pool first | opus | todo | E3.12 | sheet | yes (root + desktop) | no | must be complete before E3.MC |
| E3.19 | DISCOVERED (E3.12): `gen_class_feature_grants` reads `uc_abilities_globalvar.lst` (Ninja.MOD 64, Samurai.MOD 99 grant rows; 1 Ninja row hand-added by E3.12) and re-runs without rewriting the 58 committed files it drifts today | opus | todo | E3.12 | sheet | yes (root) | no | must be complete before E3.MC |
| E3.20 | DISCOVERED (E3.13): the Class Features `Bloodline` pool (granted 1 by `Sorcerer ~ Standard Bloodline`) reads `0 of 1 spent, 1 remaining` for a Sorcerer whose bloodline is held, and its member Standard Bloodline is struck `requires Bloodline` although the engine holds that record (it gates the Sorcerer bloodline pool E3.13 sizes); the level-1 bloodline kind pick is reconciled with the held bloodline and its qualification verdict agrees with the gate | opus | todo | E3.13 | sheet | yes (root + desktop) | hub | must be complete before E3.MC |
| E3.MC | E3 merge check: every sub-tab on three seeds; every class-granted choice reachable or a discovered card (intake §22c); notices = E0.1's data-gap list; no engine-gap notice | opus | todo | E3.11 | sheet | yes (base build) | no | |
| E4.1 | One display-name derivation; the all-kinds test; proposed names from book text | opus | todo | E0.3, E3.MC | sheet | yes (root + desktop) | no | |
| E4.G | Operator wait: approve the proposed names; confirm the reading of "1" (Q-20.1) | operator | todo | E4.1 | lead | no | no | the operator's reply quoted in `progress.md` |
| E4.2 | Ship the approved names; Market and Inventory show them | sonnet | todo | E4.G | sheet | yes (root + desktop) | no | |
| E5.1 | Report API client in Rust; key stamped at build; every reply handled | opus | complete | C1 | settings | yes (desktop) | hub | precondition P-8 (CI secret) is checked at E7.2, not waived; receipt `artifacts/epic_5/E5.1_cycle_receipt.md`; SHA `f9416a5d0712d3931d209ad4dfb8b87405f1cf9f` |
| E5.2 | Anonymous selector + optional Handle; GitHub-unavailable message | opus | complete | E5.1 | settings | no | no | fence exit App.tsx (composers live there; retro incident); receipt `artifacts/epic_5/E5.2_cycle_receipt.md`; SHA `96e241f7310ae4ec3728a46f649857e150e5afff` |
| E5.3 | Update menu: channel selector beside `Channel:`; wider text blocks | sonnet | complete | E5.2 | settings | no | no | receipt `artifacts/epic_5/E5.3_cycle_receipt.md`; SHA `7c82126eeb0a612d786000a9ac15201b58e8eb29` |
| E5.4 | Developer tab slimmed + Corpus Ingest Diagnostic embedded | sonnet | complete | E5.3, E2.3 | settings | no | no | receipt `artifacts/epic_5/E5.4_cycle_receipt.md`; SHA `152ca44ab809a8398e9619b149466a61cb98e4a7` |
| E5.MC | E5 merge check + epic report | opus | complete | E5.4 | settings | yes (base build) | no | widest scope green on `e5b75764ac` (frontend 227/227; desktop 6 suites 1,759 ok; root 301 results 6,909 ok; 0 FAILED); audits all OK; 8 PNGs looked at; receipt `artifacts/epic_5/E5.MC_cycle_receipt.md`; SHA `c33c636f736ede95f5e07c7a32b89c179dec2359` |
| E6.1 | Campaign folder relocation: copy, verify, switch, skeleton | opus | complete | C1 | campaign | yes (desktop) | hub | RED: 0 tests matched `relocate_campaign` (exit 1); GREEN 3 passed; receipt `artifacts/epic_6/E6.1_cycle_receipt.md`; SHA `a2530d82d746d8efeb568dfa209636dcce8af2a7` |
| E6.2 | House Rules ☰ menu per character and system | opus | todo | E3.MC | sheet | yes (desktop) | hub | |
| E6.3 | House rule: HP at level-up, seven methods | opus | todo | E6.2 | sheet | yes (root + desktop) | hub | |
| E6.4 | House rule: Ambidextrous removes the off-hand penalty | opus | todo | E6.3 | sheet | yes (root + desktop) | hub | |
| E6.5 | Custom items export/import | opus | complete | E6.1 | campaign | yes (desktop) | hub | RED: receipt absent, 0 export/import fns (exit 1); GREEN `custom_items_exchange` 3 passed; receipt `artifacts/epic_6/E6.5_cycle_receipt.md`; SHA `f5a12da36086c5abd1d02685fc3bbff24e30b4c1` |
| E6.MC | E6 merge check + epic report | opus | todo | E6.4, E6.5 | campaign | yes (base build) | no | |
| E7.1 | ui-smoke rows for every changed screen; inventory regenerated | sonnet | todo | E1.MC, E2.MC, E3.MC, E4.2, E5.MC, E6.MC | main | no | no | sole `spec.json` writer |
| E7.2 | Final-acceptance scan (widest verify, packaged resources, fresh install, P-8) → retrospective written and cited → sweep | opus | todo | E7.1 | main | no | no | stops with no retro/sweep/PR if anything is short |
| E7.3 | Release notes (final) + `current-release.json` check → architecture truth-up | sonnet | todo | E7.2 | main | no | no | |
| E7.4 | PR LAST → lead merges when green → watch the develop publish (graphify out of scope, operator 2026-11-22) | sonnet | todo | E7.3 | main | no | no | a red publish is a blocker |

Count at authoring, by epic (`awk -F'|' '$2 ~ /^ (C|E)[0-9]/ {gsub(/ /,"",$2); split($2,a,"."); g=(a[1] ~ /^C/)?"C":a[1]; n[g]++} END {for (e in n) print e, n[e]}' kanban.md | sort`):
C 3, E0 5, E1 12, E2 6, E3 12, E4 3, E5 5, E6 6, E7 4 = 56. Discovered: E1.12 (E0.4, 2026-11-22) → E1 13; E1.13 (E1.6, 2026-11-22) → E1 14; E1.14 and E1.15 (E1.MC, 2026-11-22) → E1 16; E2.6 (E2.MC, 2026-11-23) → E2 7; with E0.1's E3.12 the total is 62; E3.13 and E3.14 (E3.4, 2026-11-23) → E3 15, total 64; E3.15 (E3.5, 2026-11-23) → E3 16, total 65; E3.16 (E3.7, 2026-11-23) → E3 17, total 66; E3.17 (E3.9, 2026-11-23) → E3 18, total 67; E3.18 and E3.19 (E3.12, 2026-11-24) → E3 20, total 69; E3.20 (E3.13, 2026-11-24) → E3 21, total **70**.

`Depends on` equals `epic-breakdown.md §0`'s `After` column except three rows, where it carries the card sections'
own wording: E1.10 adds E0.2 ("Depends: E1.9 (lane order), E0.2 (data)"), E5.4 adds E2.3 ("Depends: E5.3, E2.3"),
and E7.1 spells out "all E*.MC, E4.2" as the six cards.
