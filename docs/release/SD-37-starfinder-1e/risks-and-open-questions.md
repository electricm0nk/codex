---
canonical: true
owner: operator
bundle_id: SD-37
date: 2026-10-02
---

# SD-37 Risks and Open Questions

## 1. Risks, ranked

| ID | Risk | Self-healable? | Control (mechanism, not warning) |
|---|---|---|---|
| R-1 | **Overloaded PCGen fields print confidently wrong numbers.** `HP\|CURRENTMAX` is Stamina, `HP\|ALTHP` is HP, and `CURRENTMAX` sits one per level below the planner's recollection of the book value, probably because PCGen adds the `HD:1` die on top (`decisions.md §8`). | yes, inside E3.3 | SF-only mapping table with one oracle row per field; planted mutations (swap and drop-`HD`) must turn the fixtures red; refusal for any unmapped field |
| R-2 | **No game-system partition today.** One baked `data/sheet_rules` root (`corpus_loader.rs:360`), corpus-wide pools, 19 book consts (CUI F-13). Dropping SF into that tree would offer SF characters PF feats. | yes | E1 lands first; PF byte-identical hash gate; `(kind, slug)` namespacing (SD-h) |
| R-3 | **The compute spine is PF-shaped** (single AC, touch, CMB/CMD, hit die; 82,457 lines of per-PF-class modules, quoted). Porting it repeats SD-27..36. | yes | NR-2's command must print 0; generic chassis reader only |
| R-4 | **Thin oracle.** SF `.lst` is 16,594 lines vs PF's 228,220 (CUI F-3, F-5); `STATUS:BETA` PCCs; the core PCC calls itself "AN ALPHA RELEASE"; there is no Resolve row and no starship data. Whether `scripts/pcgen-run-character.sh` can select the Starfinder game mode is **unverified**. A parity PASS covers fewer shapes than PF's did. | partly | E7.1's explicit "not covered" list; SRD hand values (E0.4) independent of the oracle; E7.1 adds game-mode selection to the runner if missing |
| R-5 | **E4a's blast radius** (180,883 lines, 252 importers; CUI F-10, F-12). It could swallow the bundle. | yes | Parallel track, own worktree and `CARGO_TARGET_DIR`, file fences (`workflow-instruction.md §3`), off SF's critical path; the operator may split it out (`decisions.md §19`) |
| R-6 | **The packaged-app path is baked.** The core `live_sheet_rules()` has no runtime override, so on a tester machine it can fall back to "not verified". A second system doubles this. | yes | E1.1's runtime resolution; E6.1's clean-checkout build |
| R-7 | **Quota exhaustion while unattended.** SD-36 Epic F reached 87% of weekly quota (quoted). | yes | Quota stop rule (`decisions.md §12.3`) with a resume receipt; models are never downgraded |
| R-8 | **VM stop / OOM mid-run.** Two occurred in SD-36. | yes | Crash-resume recipe (`decisions.md §12.4`); at most 3 cargo lanes; MEMORY GUARD |
| R-9 | **Peer session on the shared checkout.** The main checkout `/home/ubuntu/workspace/repos/codex` is another session's tree (`tranche/16`, dirty retro log). | yes | SD-37 runs only in `/home/ubuntu/workspace/worktrees/codex-sd37` and its lane worktrees; `git reflog`/`git worktree list` before each dispatch; SD-j |
| R-10 | **Licence/PI.** SF PCCs use the Paizo Community Use Policy + OGL; the PI term set (Pact Worlds proper nouns) does not exist yet; site PI gates need it too. | partly (operator signs) | E0.2 deliverable; exclude-and-log default (SD-a); `operator_sign_off` stays false until the operator signs |
| R-11 | **Seed hand values need network** (no Starfinder book on the machine). | no, if the SRD is unreachable | `blocked-escalated` on E0.4 only (SD-c); independent cards continue |
| R-12 | **Baselines move.** A second system shifts many count-pinning files in `scripts/verify-baselines.env`. | yes | E7.2 re-derives the baselines from one logged full run; "count change needs a sweep" of old and new counts |

## 2. Open questions → operator-away defaults

Every open question has a default already taken. All of them are listed in `README.md §6`.

| Q | Question | Default taken | Where |
|---|---|---|---|
| Q-1 | `docs/release` vs the overlay's `docs/stc` | `docs/release` (flagged) | `decisions.md §1` |
| Q-2 | Does PR #395 gate the cut? | No; rebase if it lands first | `decisions.md §2` |
| Q-3 | Starship in the DoD? | No; planned deferral with a checked revisit | `decisions.md §17` |
| Q-4 | Alien Archives in scope? | Yes, as the `.lst` carries them; no monster simulation | `decisions.md §4` |
| Q-5 | Resolve formula source | SRD hand transcription + Opus review | `decisions.md §18` |
| Q-6 | E4a in this bundle? | Yes, parallel, fenced; the operator may split it out | `decisions.md §19` |
| Q-7 | Seed builds | Soldier 3, Mystic 5, Technomancer 5, Envoy 3 | `decisions.md §9` |
| Q-8 | Does FS-2 (SD-34 P1s) gate Starfinder? | No; candidate | `decisions.md §10` |
| Q-9 | Watchdog | One long Workflow run | `decisions.md §12.2` |
| Q-10 | Quota threshold | 85% weekly, or 10 M subagent tokens (estimate) | `decisions.md §12.3` |
| Q-11 | PF data layout | Unchanged; SF under `data/starfinder-1e/` | `decisions.md §7` |
| Q-12 | Planning model | Sonnet (not switchable); Opus review C0.2 | `decisions.md §11` |

## 3. Override flags

None are set. The no-stub doctrine's audit is waivable only by an explicit operator override
recorded here, and no override exists.
