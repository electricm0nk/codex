---
canonical: true
owner: operator
bundle_id: SD-38
slug: cyberpunk-red
canonical_branch: tranche-18
target_version: 0.18.0
date: 2026-10-13
status: draft — authored under operator-away defaults; operator rulings on licence and scope are on record
---

# SD-38 Risks and Open Questions

## 1. Risks, ranked

| ID | Risk | Self-healable? | Control (a mechanism, not a warning) |
|---|---|---|---|
| R-1 | **Transcription error with no external oracle.** There is no PCGen data; the page text is garbled on many pages (CUI §4), and the inventories already caught column-placement hazards a sum check cannot see (`creation.md §2d`: the 86-point check proves sums, not placement). A wrong value looks plausible on the sheet. | yes, inside E1 | Hand transcription from the page image for every CUI §4 page; E1.V's 10% image sample per kind with 0 `disagree` left; book-internal parity fixtures (HP table 126 cells, 10 templates = 86, 10 cyberware packs = F-66, p121); counts pinned by two implementations |
| R-2 | **The proof is narrower than it looks** (AGENTS.md rule 7). A 10% sample and four parity tables leave most records checked only by the lane that wrote them. | partly | E1.V reports the unsampled share by kind; each card's "Does not cover" field; the operator may raise the sample rate (change E1.V only) |
| R-3 | **PF/SF regression through the shared seam.** E0.1 touches every `match` and `PerSystem` literal (24 sites) and the `tauri-resources-tracked` parser. | yes | G-1 byte-identical gate at E0.6 and every later merge check; never re-baseline (SD-e) |
| R-4 | **Shared-file collisions between parallel lanes** (`main.rs`, `CharacterHubPage.tsx`, `spec.json`, `cpr/mod.rs`, `cpr_adapter.rs`). | yes | Writer order by `Depends on` (`epic-breakdown.md §0`); the script refuses a card whose dependencies are not `complete` (R8); lane anchors in `cpr/mod.rs` (A-10, SD-p) |
| R-5 | **More than 3 cargo lanes** (E2's three lanes plus E3/E6). | yes | 3-slot semaphore in the script; `Cargo?` column; E1 lanes run no cargo |
| R-6 | **The PF-typed adapter trait** (`rule_system_adapter.rs:72–133`) does not fit CPR. | yes | `decisions.md §9`/`technical-design §2`: PF-typed methods follow SD-37's precedent; CPR actions are their own commands |
| R-7 | **Resolved at authoring audit (2026-10-13).** A `Populated at closure` notes body would have kept `resolve_release_notes.py` (CI step `release-tooling-tests.yml:49`) red from C1 to E7.8, and leaving the pointer on SD-37 fails the resolver's tranche check. | — | `release-notes.md` ships an interim body without the resolver's markers, opened by the sentinel `SD-38 INTERIM NOTES`; C1 asserts the resolver exits 0; E7.8 and E7.11 fail while the sentinel exists (`decisions.md §2`). **Residual, ACCEPTED by the lead (2026-10-13):** the publish workflow does not check the sentinel. The accepted control is the E7.8 → E7.11 order (both fail while `grep -c 'SD-38 INTERIM NOTES'` is not 0) plus the operator's merge |
| R-8 | **Gate PR #398 not merged.** C1 cannot cut. | no (operator) | SD-k: the run writes a resume receipt and stops; a new run starts with `START=C1` |
| R-9 | **Quota exhaustion while unattended** (SD-37 ran 5 Workflow runs). | partly | `decisions.md §16.3` stop rule; resume receipts; never downgrade. **Residual:** resumption needs a session |
| R-10 | **VM stop / OOM** mid-run. | partly | crash-resume recipe (`§16.4`); MEMORY GUARD; ≤ 3 cargo lanes. **Residual:** nothing restarts the orchestrator |
| R-11 | **Licence posture.** The whole book text ships under the operator's ruling; it is not OGL. | n/a (operator ruling) | LICENSE.json + DEVIATION row; every record has `source_page` so a later rewording can be scoped (`decisions.md §4`) |
| R-12 | **GM tables without an inventory** (Mooks & Grunts, Encounters, Beat Charts). Their counts are unknown at authoring. | yes | E1.8 inventories them first with two implementations; MANIFEST `null` until E1.V pins it; `--all` fails on `null` |
| R-13 | **Fresh-install check may not be runnable on this box.** | no, if the box cannot install | SD-m: E7.3 `blocked-escalated` with the operator action named; the bundle waits at E7.5 |
| R-14 | **Author defaults the operator did not see** (A-1…A-11). | n/a | Listed in `README.md §6` and `decisions.md §21` with revisit lines; each is one card's change |
| R-15 | **Peer sessions on the lead's checkout** (`/home/ubuntu/workspace/repos/codex`). | yes | SD-38 writes only in `.worktrees/sd38-*`; the wrong-base control refuses the main checkout; `git reflog`/`git worktree list` before each dispatch; SD-f |

## 2. Open questions → defaults taken

Every open question has a default already taken (`README.md §6`).

| Q | Question | Default | Where |
|---|---|---|---|
| Q-1 | `docs/release` vs the overlay's `docs/stc` | `docs/release` (flagged) | `decisions.md §1` |
| Q-2 | Cut before #398? | No; wait | `§2`, SD-k |
| Q-3 | Does CPR get a sheet-rule package? | No; `Option` | `§8` |
| Q-4 | Where do GM tables go? | print-only reference pages | `§5` |
| Q-5 | How is transcription checked? | 10% page-image sample + book-internal parity | `§11` |
| Q-6 | In-app dice? | creation-time table rolls only; in-play dice deferred | `§22` DEF-2 |
| Q-7 | Inventory UNCLEARs | D-1…D-12 (brief); A-1…A-11 (author) | `§20`, `§21`; CUI §5 |
| Q-8 | Review card for the package? | none carded; the lead may run one | `§15` |
| Q-9 | Watchdog | one long Workflow run; new run with `START` on restart | `§16.2` |
| Q-10 | Quota threshold | 85% shown, or 10 M reported tokens (estimate), or a usage-limit error | `§16.3` |

## 3. Override flags

None are set. The no-stub doctrine's audit is waivable only by an explicit operator override recorded
here, and no override exists.
