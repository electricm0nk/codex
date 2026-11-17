---
canonical: true
owner: operator
bundle_id: SD-42
slug: pcgen-import-and-campaign-folder
canonical_branch: tranche-22
target_version: 0.22.0
date: 2026-11-16
status: planning-ready
---

# SD-42 Risks and Open Questions

## 1. Ranked risks

| # | Risk | Likelihood | Impact | Control | Self-healable? |
|---|---|---|---|---|---|
| R-1 | **An SD-42 Phase-1 card writes `character_hub.rs` or another SD-41-owned file before SD-41 merges** | medium | high | the partition (`decisions.md §4`, G-3); E4.1 depends on `SD-41 merged` (R25); the prefix refuses `tranche-21` and `sd41-*` trees | no — `blocked-escalated` + a retro incident |
| R-2 | **A hypothesis in `technical-design.md §1` is wrong and E1.2–E1.4 fix the wrong thing** | medium | high | E1.1 runs first and marks each H proved/refuted on the real files; E1.2+ cite E1.1's rows; a refuted H re-scopes the card by a `§20` correction before dispatch | yes — lead action |
| R-3 | **The operator's file leaks into the repo** (a fixture, a log, a screenshot with the player name, a receipt line) | low | very high (public repo, PII) | NR-4; the prefix's pre-commit `git diff --cached -S'Aonghus' --stat` outside this package must be empty; G-8's `-S"$SD42_PII"` → 0 over the whole history of the branch (the name set locally by the lead, never in a doc); `census.py` prints counts only; the operator's diagnosis and parity outputs go under `/home/ubuntu/sd42-input/` | no — a leak is an incident: the lead rewrites the branch before any push reaches `develop` and records it |
| R-4 | **Two catalogs disagree** (sidecar `equipment_catalog_rows()` vs the app's `corpus_fixture_bundle()`, H5) | medium | medium | E1.1 diffs the sidecar's mapped keys against the app's held list after import; a disagreement is a discovered card; E4.1's test imports the synthetic fixture through the real app path | yes |
| R-5 | **The PCGen engine does not run on this box or in CI** (Java, Gradle, the wrapper jar) | medium | high (Q4 is a closure criterion) | P-9 checked before E1.7; the exports are committed so CI compares without running PCGen; the local run re-derives them; `pcgen_engine_runnable` skips loudly | partly — if the engine never runs here, E1.7 is `blocked-escalated` with the exact missing piece |
| R-6 | **Parity finds many differences the engine does not compute** (SF fields, PF conditional bonuses) and the ledger swallows real defects | high | medium | every row names `why` and a `decision` (a decisions.md § or an SD-37 explained row); E1.MC reads the ledger and challenges each "not computed" row against the engine's `explanations`; the receipt states rows by `why` | yes |
| R-7 | **The SF catalog for the importer duplicates the SF adapter's loader** (a second reader of the SF tables) | medium | medium | E1.6 names and reuses the loader `sf_adapter.rs` / `codex::rules_core` exposes; the MC greps for a second `data/starfinder-1e` reader | yes |
| R-8 | **The eqmod → option-key table drifts from SD-41's Buy Custom keys** | medium | medium | `eqmod_option_key` lives in the root crate beside `CopyFacts`; E4.1 prices through `purchase_custom_equipment_at_root`'s pure core and fails on an unknown key (never silently drops) | yes |
| R-9 | **Byte-stability breaks on a shape the fixtures lack** (mixed EOLs, a BOM plus CRLF, a file without a final newline edited at its end) | medium | medium | the fixtures include each shape; E3.MC re-runs the round trip over every `.md` E2 wrote; `roundTrip` keeps bytes as read and re-encodes only the text | yes |
| R-10 | **CodeMirror live-preview decorations fight the user's cursor** (markers reappearing mid-word, jumping lines) | high in the first pass | medium | the cursor-line rule (raw markup on the active line only); E3.MC and E5.G look at it; defects are discovered cards | yes |
| R-11 | **A sync client (Drive, Dropbox, Syncthing) renames or locks a file during a temp + rename write** | medium | medium | SD-r: one retry after 200 ms, then a warning; every write atomic; the local save never depends on the folder (NR-11) | yes |
| R-12 | **Two devices save the same character's backup** (the two-master trap) | expected | high (data loss) | `master.json` + the counter check (Q21/Q22); a mismatch never overwrites; stand-in never writes; tests `characters_backup_counter_mismatch_does_not_overwrite_and_warns` and `mirror_two_master` | yes |
| R-13 | **The migration loses an old campaign** (localStorage cleared before the files are written) | low | high | the backup JSON is written **first** (`migrate_writes_a_backup_before_anything_else`); localStorage is cleared only after every folder write returned; `.config/<name>.json` is renamed, not deleted; "Later" keeps everything | yes |
| R-14 | **Hand-edited frontmatter (Obsidian) breaks a list** | certain for someone | low | SD-o: the tolerant parser lists the file with a mark; `write_key` touches one key; tests with malformed YAML | yes |
| R-15 | **Path traversal from the UI** (`../` in a wiki link or an attachment name) | low | high | NR-9: the Rust side resolves and refuses `..`, absolute paths and symlink escapes; a test per command | yes |
| R-16 | **`npm` registry unreachable when E3.1 runs** | low | medium | P-10; the versions are named so a lockfile can be authored from a cache; a failure is `blocked-escalated` naming the package | partly |
| R-17 | **Three lanes + SD-41's lanes exceed the 4-cargo cap → OOM / Proxmox stop** | medium | high | the prefix waits on `pgrep -x cargo < 4`; the editor lane runs no cargo; `free -g` before long runs | yes |
| R-18 | **An acceptance command passes on the base (cannot fail)** | high (SD-39 retro) | medium | R21: the lead runs each on the pre-change tree before dispatch | yes |
| R-19 | **FILES YOU OWN misses a second call site** (`after_write` has many callers; `PartyMemberInput` has a TS twin) | medium | low | R22 pre-dispatch `git grep`; E2.10 changes `mirror`'s inside so the callers need no edit; E2.11 names both sides | yes |
| R-20 | **The drone file's ruling (Q-A) is wrong and the operator wanted it imported as a companion** | low | low | the default is logged; the parity ledger explains it; FSR-C4 names the follow-up | n/a |
| R-21 | **`Characters/` vs `characters/` on a case-insensitive sync target** (Windows, macOS Drive clients) | medium | medium | the rename is `characters/` → `Characters/` through a temp name (`.characters-tmp`) so a case-insensitive filesystem completes it; a test on a case-insensitive temp FS when available, else stated | yes |
| R-22 | **The bundle is large (41 cards on 2026-11-17) and the operator sees no result for days** | medium | low | three parallel lanes; each MC leaves screenshots and an Epic report in `progress.md`; the import lane's E1.MC is the first operator-readable result | n/a |
| R-23 | **SD-41's retrospective lands after this package and names a change SD-42 ignores** | medium | low | `workflow-instruction.md §12.2`: the lead reads it before E4.1 and records each change | yes — lead action |

## 2. Open questions → defaults taken

The questions only the operator can settle are `decisions.md §21` (Q-A…Q-M), each with its default; the run
does not wait on them. Recommended answers, for the operator's one-line rulings:

| # | Question | Recommended answer (the default taken) |
|---|---|---|
| Q-A | The drone file | import as its own SF character if the engine computes a Drone, else `Blocked` with the reason |
| Q-B | "Your name" source | a required Create-screen field, remembered locally |
| Q-C | Known-folders list location | `<app data>/campaign-folders.json`, local to the install |
| Q-D | Adventure Log filename | `<YYYY-MM-DD> <author> - <title>.md` |
| Q-E | Pin coordinates | fractions of the image size |
| Q-F | Image link syntax | `![](Attachments/<name>)` |
| Q-G | Same base at two locations | two stacks |
| Q-H | Gates | one, E5.G |
| Q-I | `QUANTITY:0` lines | not held; a Notes line |
| Q-J | `Characters/` capitalisation | rename on first write |
| Q-K | `campaign.json` at the root | yes |
| Q-L | Q10 / Q23 missing from the intake | nothing lost (please confirm) |
| Q-M | Companion lines | gear lines import as gear; companion files are not followed |

## 3. Non-self-healable conditions (card → `blocked-escalated`)

Two live lanes on one file; a Phase-1 write to an SD-41-owned file; RED→GREEN not preserved; a stub, mock or
"Would …" string in shipping code; a `VALUE_MOVED` seed no named key explains; an SF1e/CPR golden text delta a
card cannot remove; the operator's file found in a staged diff outside this package (an incident, not a
card); the PCGen engine not runnable after P-9's exact missing piece is named; the operator's gate verdict
"not usable" (becomes discovered cards, then the gate re-runs); `CLAIM-EXISTS`.

## 4. Bundle size and the split the author did not take (`decisions.md §25`)

37 cards at authoring, 41 on 2026-11-17 (E1.8 discovered; E2.12, E2.13, E4.6 added by the operator, `decisions.md §26`), against SD-41's 56 (`awk -F'|' '$2 ~ /^ (C|E)[0-9]/' ../SD-41-gear-and-equipment-sets/kanban.md | awk 'END{print NR}'`).
The import lane (8 cards) and the campaign + editor lanes (16 cards) are file-disjoint (F-20) and run at the
same time, so the wall time is the longer lane plus Phase 2 (6 cards) and closure (5). The author keeps one
bundle. **If the operator prefers two:** SD-42 = C + E1 + E2.1–E2.5 + E2.10–E2.11 + E3.1 + E4 + E5 (import,
the folder model, member #1, the Drive removal, migration, help, `Characters/`, the DM toolkit, the editor core);
SD-43 = E2.6–E2.9 (the four tabs), E3.2–E3.3 (completion and the extras), with its own closure. The cut is clean
because the four tabs depend on E3.1 only and nothing in E4 depends on them. The cost of the split: a second C1,
a second closure epic and a second catch-up, and an SD-42 whose Campaign Manager ships with a Help screen
describing tabs that are read-only file lists — the author judges that the worse product.
