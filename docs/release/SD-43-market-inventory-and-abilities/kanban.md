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
| C2 | Incremental graphify after SD-42 (no `--force`, no `graphify update`); receipt | sonnet | todo | C1 | main | no | no | intake item 0 step 4 |
| E0.1 | Abilities census: seven sub-tabs, every class-granted choice by mechanism (§22c), pools, saved fields, budgets, prereqs, pool sizes per level, PCGen mapping, data-gap list | opus | complete | C1 | census | no | no | settles Q-22.2; RED `abilities_census.py --check` exit 2 (no such file); GREEN exit 0; 1,329 class-granted choice sites (A=B); 34 data gaps (30 ingest → E3.12); receipt `artifacts/epic_0/E0.1_cycle_receipt.md`; SHA `d9c84afeaf037a84a04700c640447e1a1f935c14` |
| E0.2 | Market count reconciliation test over `built_catalog()` | opus | todo | C1 | market | yes (desktop) | no | settles Q-11.1 |
| E0.3 | Placeholder-name scan widened to every kind and surface | sonnet | todo | C1 | census | no | no | must reproduce 111 / 201 |
| E0.4 | Reproduce items 5, 7, 12 on a fresh Brother Tomas Vane | opus | todo | C1 | gear | yes (app build) | no | isolated app-data root |
| E0.5 | Rest-rule census (classes, races); House Rules per system (PF, SF, CPR) | sonnet | todo | C1 | census | no | no | settles Q-2.1 input, Q-3.5 |
| E1.1 | Container truth: no `#N`, custom names, filled container never stacked, split on put-into-stack | opus | todo | E0.4 | gear | yes (desktop) | hub | |
| E1.2 | One-column gear list: order, natural sort, nested boxes, Not Carried section | opus | todo | E1.1 | gear | no | no | |
| E1.3 | Whole container box is a drop target (nearest container) | sonnet | todo | E1.2 | gear | no | no | |
| E1.4 | Double-click / Enter: first free slot, else swap; to Carried; one piece | opus | todo | E1.3 | gear | no | no | |
| E1.5 | Selling a container moves its direct contents to Carried | opus | todo | E1.4 | gear | yes (desktop) | hub | |
| E1.6 | Sell window: Market Rate / Build / Negotiate | opus | todo | E1.5 | gear | no | no | |
| E1.7 | One `MarketButton` on every panel (PF, SF, CPR); Print and pricing control leave the panels | sonnet | todo | E1.6 | sheet | no | hub | |
| E1.8 | Market inventory sell value column (`sellNegotiateGp`) | sonnet | todo | E0.2 | market | no | no | |
| E1.9 | Item stats in the Market description box (engine-stated, shared renderer) | opus | todo | E1.8 | market | yes (desktop) | hub | |
| E1.10 | Filter model: per-list columns and values, `Other`, counts cover every row | opus | todo | E1.9, E0.2 | market | no | no | |
| E1.11 | Filter column; 200 cap removed; windowed list; first paint measured | opus | todo | E1.10 | market | no | no | |
| E1.MC | E1 merge check + epic report | opus | todo | E1.7, E1.11 | gear | yes (base build) | no | |
| E2.1 | Fixed layout on the PF sheet; Show Progression visible (items 17 + 21) | opus | todo | E1.7 | sheet | no | hub | |
| E2.2 | The shared layout on the SF and CPR sheets | sonnet | todo | E2.1 | sheet | no | no | settles Q-8.2 |
| E2.3 | Open campaign button opens the folder; one banner per campaign; Manager banner smaller | opus | todo | E2.2 | sheet | no | hub | |
| E2.4 | Rest and recovery: cited table, engine command, print block, override field | opus | todo | E0.5, E2.3 | sheet | yes (root + desktop) | hub | new runtime data read: `bundle.resources` + `packaged_resources.rs` |
| E2.5 | Rest text read-only under the Biography text | sonnet | todo | E2.4 | sheet | no | hub | |
| E2.MC | E2 merge check + epic report | opus | todo | E2.5 | sheet | yes (base build) | no | |
| E3.1 | Rename the Feats tab to Abilities (operator ruling Q-22.3, 2026-11-22): seven sub-tabs with Feats as one of them, existing surfaces moved, no behaviour change | opus | todo | E0.1, E2.5 | sheet | no | hub | Q-22.3 settled: rename |
| E3.2 | Engine: generic ability-budget module, saved selections, list/add/remove commands; proved on Traits | opus | todo | E3.1 | sheet | yes (root + desktop) | hub | |
| E3.3 | Generic sub-tab UI; Traits sub-tab on it | opus | todo | E3.2 | sheet | no | hub | |
| E3.4 | Class Features: grants per level for every catalog pool | opus | todo | E3.3 | sheet | yes (root + desktop) | hub | split rule per E0.1 |
| E3.5 | Class Features: pool-member prerequisite evaluator and refusal reasons | opus | todo | E3.4 | sheet | yes (root + desktop) | hub | |
| E3.6 | Racial Abilities: alternate racial traits on the sheet; replaces-conflict refused | opus | todo | E3.5 | sheet | yes (root + desktop) | hub | |
| E3.7 | Archetype: select per class; slot conflicts refused; grants feed pools | opus | todo | E3.6 | sheet | yes (root + desktop) | hub | |
| E3.8 | Favored class bonus: class pick, per-level HP / skill / race alternates; engine totals | opus | todo | E3.7 | sheet | yes (root + desktop) | hub | |
| E3.9 | Background and Misc sub-tabs per E0.1's mapping | opus | todo | E3.8 | sheet | yes (root + desktop) | hub | |
| E3.10 | Overrides on the six new sub-tabs (two flags) | sonnet | todo | E3.9 | sheet | yes (desktop) | hub | |
| E3.11 | Printout: chosen abilities per sub-tab and the favored class bonus | opus | todo | E3.10 | sheet | yes (desktop) | no | |
| E3.12 | DISCOVERED (E0.1): ingest the oracle's member typing (`<record>.MOD TYPE:` lines) so the 30 ingest-gap pools in E0.1's data-gap list get members; add the missing `Ninja ~ Ninja Trick` grant row (level 2) to `data/class_feature_grants`; resolve the 7 pool names with no ABILITYCATEGORY | opus | todo | E0.1 | sheet | yes (root) | no | must be complete before E3.MC; E3.MC's notice count then = the 4 book gaps; `artifacts/epic_0/abilities-census.json` `data_gap_list[].cause` |
| E3.MC | E3 merge check: every sub-tab on three seeds; every class-granted choice reachable or a discovered card (intake §22c); notices = E0.1's data-gap list; no engine-gap notice | opus | todo | E3.11 | sheet | yes (base build) | no | |
| E4.1 | One display-name derivation; the all-kinds test; proposed names from book text | opus | todo | E0.3, E3.MC | sheet | yes (root + desktop) | no | |
| E4.G | Operator wait: approve the proposed names; confirm the reading of "1" (Q-20.1) | operator | todo | E4.1 | lead | no | no | the operator's reply quoted in `progress.md` |
| E4.2 | Ship the approved names; Market and Inventory show them | sonnet | todo | E4.G | sheet | yes (root + desktop) | no | |
| E5.1 | Report API client in Rust; key stamped at build; every reply handled | opus | todo | C1 | settings | yes (desktop) | hub | precondition P-8 (CI secret) is checked at E7.2, not waived |
| E5.2 | Anonymous selector + optional Handle; GitHub-unavailable message | opus | todo | E5.1 | settings | no | no | |
| E5.3 | Update menu: channel selector beside `Channel:`; wider text blocks | sonnet | todo | E5.2 | settings | no | no | |
| E5.4 | Developer tab slimmed + Corpus Ingest Diagnostic embedded | sonnet | todo | E5.3, E2.3 | settings | no | no | `CharacterHubPage.tsx` shared with E2.3 |
| E5.MC | E5 merge check + epic report | opus | todo | E5.4 | settings | yes (base build) | no | |
| E6.1 | Campaign folder relocation: copy, verify, switch, skeleton | opus | todo | C1 | campaign | yes (desktop) | hub | |
| E6.2 | House Rules ☰ menu per character and system | opus | todo | E3.MC | sheet | yes (desktop) | hub | |
| E6.3 | House rule: HP at level-up, seven methods | opus | todo | E6.2 | sheet | yes (root + desktop) | hub | |
| E6.4 | House rule: Ambidextrous removes the off-hand penalty | opus | todo | E6.3 | sheet | yes (root + desktop) | hub | |
| E6.5 | Custom items export/import | opus | todo | E6.1 | campaign | yes (desktop) | hub | |
| E6.MC | E6 merge check + epic report | opus | todo | E6.4, E6.5 | campaign | yes (base build) | no | |
| E7.1 | ui-smoke rows for every changed screen; inventory regenerated | sonnet | todo | E1.MC, E2.MC, E3.MC, E4.2, E5.MC, E6.MC | main | no | no | sole `spec.json` writer |
| E7.2 | Final-acceptance scan (widest verify, packaged resources, fresh install, P-8) → retrospective written and cited → sweep | opus | todo | E7.1 | main | no | no | stops with no retro/sweep/PR if anything is short |
| E7.3 | Release notes (final) + `current-release.json` check → architecture truth-up | sonnet | todo | E7.2 | main | no | no | |
| E7.4 | Graphify LAST → PR LAST → lead merges when green → watch the develop publish | sonnet | todo | E7.3 | main | no | no | a red publish is a blocker |

Count at authoring, by epic (`awk -F'|' '$2 ~ /^ (C|E)[0-9]/ {gsub(/ /,"",$2); split($2,a,"."); g=(a[1] ~ /^C/)?"C":a[1]; n[g]++} END {for (e in n) print e, n[e]}' kanban.md | sort`):
C 3, E0 5, E1 12, E2 6, E3 12, E4 3, E5 5, E6 6, E7 4 = 56.

`Depends on` equals `epic-breakdown.md §0`'s `After` column except three rows, where it carries the card sections'
own wording: E1.10 adds E0.2 ("Depends: E1.9 (lane order), E0.2 (data)"), E5.4 adds E2.3 ("Depends: E5.3, E2.3"),
and E7.1 spells out "all E*.MC, E4.2" as the six cards.
