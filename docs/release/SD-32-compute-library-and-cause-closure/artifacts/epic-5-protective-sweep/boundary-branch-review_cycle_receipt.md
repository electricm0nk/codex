# Cycle boundary-branch-review — Pre-G0 (housekeeping) / Card 2

- **Card ID:** `boundary-branch-review`
- **Commit SHA:** (this cycle's own commit — see git log; git-refs-only scope, no code/doc content diff)
- **Files touched:** none tracked in the working tree except this receipt, `kanban.md`, `progress.md`,
  and the append-only retro log (`docs/retro/events/boundary-branch-review.jsonl`,
  `docs/retro/events/sd31-transcribe.jsonl` auto-emitted by `scripts/verify.sh`). This card's scope
  is git refs only (`workflow-instruction.md §3`, "card 2: git refs only").
- **Identifier audit result:** N/A — no shipping-code diff. `OK_NO_BUNDLE_TAGS` by construction
  (git-ref operations carry no diff).
- **Wired-integration audit result:** N/A — same reason. `OK_NO_TOKENS` by construction.
- **Acceptance criterion:** kanban.md #2 — "Review and disposition the 3 orphaned-but-real branches
  from `artifacts/UNMERGED-BRANCHES.md` § 'Real work orphaned by an orchestrator decision'", plus
  the confirm/merge/discard/disposition items in the card's Notes column.
- **Corpus SHA:** N/A — no corpus-derived figure in this cycle.
- **Status:** complete
- **Notes:** see dispositions below. Re-derivation base: `git branch --list 'worktree-wf_*'
  review-merge-test site-deploy 'fix/*' 'sd31/*'` and `git ls-remote --heads origin`, both re-run at
  cycle start per `UNMERGED-BRANCHES.md`'s own instruction to re-derive rather than trust the
  capture.

## Re-derived branch inventory (2026-08-22, this cycle)

Local: `git branch --list 'worktree-wf_*' review-merge-test site-deploy 'fix/*' 'sd31/*'` returned
**11**, not the 10 `UNMERGED-BRANCHES.md` captured — `worktree-wf_efd6f5fc-a9c-1` is new and is a
**live, locked worktree** (`git worktree list` → `.claude/worktrees/wf_efd6f5fc-a9c-1  locked`),
its tip identical to current HEAD (fully merged, nothing to disposition) — almost certainly card 1's
own in-progress Epic-5-protective-sweep worktree. Out of this card's scope; left untouched.

Origin: `git ls-remote --heads origin` confirmed all 9 branches `UNMERGED-BRANCHES.md` listed as
"unlisted at capture" plus the rescue branch. Dispositioned below.

## Dispositions

### 1. Real work orphaned by the killed wave (UNMERGED-BRANCHES.md §2)

| Branch | Disposition | Evidence |
|---|---|---|
| `worktree-wf_c1156061-e3f-3` | **KEEP — real, verified-still-live defect fix. Forwarded as DISCOVERED, not merged here.** | `git diff $(git merge-base worktree-wf_c1156061-e3f-3 HEAD)..worktree-wf_c1156061-e3f-3 -- src/bin/gen_book_cache.rs` shows a real fix (`index_existing_records_by_key`, extends `gen_monster_book`'s existing `out_path.exists()`-skip guard to `gen_pathfinder_unchained`/`gen_advanced_race_guide`/`gen_companion_book`, deliberately with no stale-record deletion — see the branch's own doc comment). **Confirmed the vulnerability is still live at current HEAD**: `grep -n "remove_dir_all" src/bin/gen_book_cache.rs` still shows the unconditional wipe in all 3 unguarded functions (lines 626, 815, 1580), and `grep -c index_existing_records_by_key src/bin/gen_book_cache.rs` on HEAD is 0. This is genuinely unmerged, still-needed work, matching `sweeps.md` S6's own "OPEN — not yet root-caused into a fix" note and `defects.md` D9 (the current D9, about this exact generalized self-erasure shape). Not merged inline here — a code fix needs its own TDD cycle (RED→GREEN, dual-audit), not a housekeeping card's git-refs-only scope. |
| `worktree-wf_c1156061-e3f-5` | **KEEP — real, unlanded doc content. Forwarded as DISCOVERED, not merged here.** | Diff is 2 lines in `todo/defects.md` (new row, but collides with an already-taken `D9` ID — a different lane's D9, the gen_book_cache one above, landed first) and one row-rewrite in `todo/levers.md` (L3: corrects the filed "28 units, NOT STARTED" to "DEAD — 0 buildable within lane scope, 11 real candidates need a new `MonsterAbilityRecord` field the transcription generator alone can populate"). Checked current HEAD's `levers.md`/`defects.md` for this content under any wording — absent (`grep -rn "same-book\|Constant spell-like\|MonsterAbilityRecord" todo/levers.md todo/defects.md todo/sweeps.md` → no hits), unlike the two branches below whose content DID land through a reviewed/corrected path. Genuinely never folded in. |
| `review-merge-test` | **DISCARD (deleted).** Content fully subsumed. | Its new artifact `VISIBILITY-cross-kind.md` is **byte-identical** already present at current HEAD (`diff <(git show review-merge-test:...) docs/.../VISIBILITY-cross-kind.md` → empty). Its `todo/{blocked,levers,sweeps}.md` edits (B7/B8 scope corrections, S12/S13/S14/L11) are present at HEAD too, but in a **reviewed, re-corrected wording** ("independently reproduced exactly by wave-30 reviewer 7", numbers refined) rather than this branch's raw first-pass text — confirming the substance landed via a different, reviewed integration path, and merging this branch now would reintroduce the pre-review wording. Deleted: `git branch -D review-merge-test`. |

### 2. Late-landing lane work (UNMERGED-BRANCHES.md §3)

| Branch | Disposition | Evidence |
|---|---|---|
| `worktree-wf_cb84ba1e-439-2` | **DISCARD (deleted). Closed-sweep confirmation: it landed, then was correctly reverted.** | Per the card's own instruction ("confirm its closed sweep landed in `todo/sweeps.md`") — it did, in substance: current `sweeps.md` S2 says "STILL PARTIAL — a wave-31 lane attempted to close this at '0 of 3 kinds (equipment/spell/feat) have an IdEnum+table+dispatch construction at all' and was **reviewed GAMED**. That closure and its supporting architectural claim are both false" — this is a direct, matching description of THIS branch's own claim (`git show worktree-wf_cb84ba1e-439-2:.../sweeps.md`, its S2 row: "CLOSED wave 31 — all 7 named kinds accounted for... equipment/spell/feat N/A-by-architecture"). The branch's own claim was reviewed and refuted (a live `EquipmentCategory`/`FeatCategory`/`Pf1SchoolId` counterexample was found). Merging the raw branch would reintroduce an already-refuted claim. Deleted: `git branch -D worktree-wf_cb84ba1e-439-2`. |
| `worktree-wf_be4660f2-72a-3` | **NOT a late-landing lane — this is a THIRD GAMED branch, mis-categorized in `UNMERGED-BRANCHES.md` §3. Left untouched, same disposition as the two named GAMED branches.** Correction logged via `scripts/retro.py`. | Current HEAD's own source carries the tell: `src/rules_core/pilot_compute/mod.rs:9645` — `"wave 26's Undine finding (OPEN-ISSUES.md row 365, GAMED/not merged)"`. `OPEN-ISSUES.md` row 365 (`SD31-W26-INTEGRATE-001`, still `RULING-NEEDED`): "**race_trait lane (`worktree-wf_be4660f2-72a-3`, commits `31fcd11df`/`d93ee6d1a`) marked GAMED by adversarial review and NOT MERGED to tranche/11**" — the reported +14 board gain came entirely from hand-typing `"undine"` into `FORMULA_RACE_TRAIT_RACES`, not from the seam computing anything (only 3 of 20 credited records have a formula the seam reads). `UNMERGED-BRANCHES.md` categorized this branch under "Late-landing lane work... looks mergeable on review" — that categorization is **wrong**, contradicted by the bundle's own predecessor's `OPEN-ISSUES.md`. Not deleted (the reviewer's own note says two remediation paths exist for a *future* wave to cherry-pick the seam+fixtures without the gaming vector — deleting the branch would lose that reference), not merged. |

### 3. Site branches (UNMERGED-BRANCHES.md §4)

| Branch | Disposition | Evidence |
|---|---|---|
| `site-deploy` | **DELETE (done) — merged via PR #361**, not #366-373 as the card's notes guessed. | Single unique commit `4bfc5d7e4 "Add public landing site (campaign-codex.org) under site/"`; `gh pr list --state merged --search "site"` shows PR #361 "Add public landing site (campaign-codex.org) under site/" merged 2026-08-14. `site/` exists at HEAD (content since evolved further through PRs #364/#367-373, which is why a literal file diff isn't byte-identical — expected, not a gap). Deleted: `git branch -D site-deploy`. |
| `fix/site-deploy-page-workflow` | **DELETE (done) — merged via PR #362.** | Single unique commit `e505b2db6 "ci(site): add Cloudflare Pages deploy workflow for site/"`; PR #362 same title, merged 2026-08-14. `diff <(git show fix/site-deploy-page-workflow:.github/workflows/deploy-site.yml) .github/workflows/deploy-site.yml` → **empty (identical)**. Deleted: `git branch -D fix/site-deploy-page-workflow`. |

### 4. Origin-side branches UNMERGED-BRANCHES.md lists as unlisted-at-capture

| Branch | Disposition | Evidence |
|---|---|---|
| `worktree-wf_0628906e-65b-1` | DELETE (done) — 0 unique commits vs `origin/develop` | `git merge-base origin/develop <branch>` == branch tip |
| `worktree-wf_0628906e-65b-3` | DELETE (done) — 0 unique commits vs `origin/develop` | same method |
| `worktree-wf_0628906e-65b-4` | DELETE (done) — 0 unique commits vs `origin/develop` | same method |
| `worktree-wf_0628906e-65b-6` | DELETE (done) — 0 unique commits vs `origin/develop` | same method |
| `worktree-wf_1ad13e3b-085-4` | DELETE (done) — 0 unique commits vs `origin/develop` | same method |
| `worktree-wf_1ad13e3b-085-5` | DELETE (done) — 0 unique commits vs `origin/develop` | same method |
| `worktree-wf_1ad13e3b-085-6` | DELETE (done) — 0 unique commits vs `origin/develop` | same method |
| `test` | **DELETE (done)** — 1 unique commit (PR #366 "Initial test merge with public site payload", merged into `test` itself, not develop/main), superseded by the real site-publish PRs #367-373 that followed it (`diff test..origin/develop -- site` shows +332K lines / far larger current content, not a gap). A disposable one-off integration-test snapshot, not orphaned production work. | `git log origin/develop..test --oneline`; `gh pr list` #366 title |
| `update-index` | **LEAVE ALONE — out of scope, not orphaned corpus work.** Active CI/release-automation branch: 293 commits, all `github-actions[bot]`-authored `channel-index: alpha ...` entries (release-channel publishing), unrelated to the SD-31/32 corpus grind. Deleting a bot-managed automation branch without confirming the publishing pipeline no longer needs it is outside this card's authority. | `git log update-index --oneline \| wc -l` → 293; `git show update-index --stat` → `channels/alpha.json` + `update-manifest.json` bot commits |

Command used for all 9: `git fetch origin <branch>:refs/remotes/origin-tmp/<branch>` then
`git merge-base origin/develop origin-tmp/<branch>` compared to the branch tip; temporary
`origin-tmp/*` remote-tracking refs deleted after use (`git branch -D -r origin-tmp/*`).

### 5. Left untouched (per standing instruction)

- `sd31/racetrait4-SD31-E6-F4-005` — rescue branch, never gated/PI-screened/merged on trust.
- `worktree-wf_13156488-c9b-1`, `worktree-wf_a45ece26-3fc-1` — the two named GAMED branches.
- `worktree-wf_be4660f2-72a-3` — the third, mis-categorized GAMED branch (see §2 above).
- `worktree-wf_efd6f5fc-a9c-1` — live locked worktree, not in `UNMERGED-BRANCHES.md`'s capture,
  out of this card's scope.

## Summary

- **Local branches deleted:** `site-deploy`, `fix/site-deploy-page-workflow`, `review-merge-test`,
  `worktree-wf_cb84ba1e-439-2` (4).
- **Origin branches deleted:** `worktree-wf_0628906e-65b-{1,3,4,6}`,
  `worktree-wf_1ad13e3b-085-{4,5,6}`, `test` (8).
- **Kept, forwarded as DISCOVERED** (real unlanded work, needs its own dispatched cycle):
  `worktree-wf_c1156061-e3f-3` (code, high priority — self-erasure fix), `worktree-wf_c1156061-e3f-5`
  (docs — L3 lever correction + D9-collision finding).
- **Kept, left untouched** (GAMED / rescue / out-of-scope-live): `worktree-wf_be4660f2-72a-3`,
  `worktree-wf_13156488-c9b-1`, `worktree-wf_a45ece26-3fc-1`, `sd31/racetrait4-SD31-E6-F4-005`,
  `update-index`, `worktree-wf_efd6f5fc-a9c-1`.
- **Correction logged:** `UNMERGED-BRANCHES.md` §3's categorization of `worktree-wf_be4660f2-72a-3`
  as mergeable late-landing work is wrong; it is GAMED per `OPEN-ISSUES.md` row 365
  (`scripts/retro.py correction`, actor `boundary-branch-review`).
- **Deferral logged:** the two real-content branches' actual merge, deferred to a future dispatched
  cycle (`scripts/retro.py deferral`, actor `boundary-branch-review`).

## Discovery forwards

- **DISCOVERED-1 (high priority):** `worktree-wf_c1156061-e3f-3`'s `gen_book_cache.rs` self-erasure
  fix is real, unmerged, and the vulnerability it fixes is still live at HEAD. Same defect class as
  Epic 5's protective sweep (card 1) — recommend card 1's dispatch (or its own follow-up cycle)
  cherry-pick/rebuild this fix with a fresh TDD cycle (RED: reproduce the strip on a real corpus
  record per the branch's own doc comment; GREEN: the guard). Do not merge the branch as-is without
  re-verification — it predates SD-32's chassis and needs a fresh base + tests run.
- **DISCOVERED-2:** `worktree-wf_c1156061-e3f-5`'s two doc corrections
  (`todo/levers.md` L3 → DEAD, `todo/defects.md` new-D9-collision re `MonsterAbilityRecord`) never
  landed under any wording. Low risk (doc-only), needs its own ID (current `D9` is taken by a
  different, already-landed finding) — forward to the next cycle touching
  `docs/release/SD-31-corpus-closure-grind/todo/`.

## Next-cycle plan

Card 2 is complete. Two forward items recorded above for a future cycle to pick up (not blocking
Pre-G0 closure — the `boundary-branch-review` card's own AT is satisfied: the 3 named
orphaned-but-real branches are reviewed and dispositioned, the closed-sweep confirmation is done,
the site branches are dispositioned, and the 9 unlisted origin branches are dispositioned). Next
per `kanban.md`'s order: Gate 0 (card 3, `gate-0-census-closure`), gated on Pre-G0 (cards 1+2) both
being met — card 1 (Epic 5 protective sweep) is still in progress in its own worktree
(`worktree-wf_efd6f5fc-a9c-1`, confirmed locked/live this cycle).
