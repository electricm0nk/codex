---
canonical: true
purpose: Branches carrying unmerged commits at the tranche/11 → tranche/12 boundary. Preserved
  deliberately rather than cleaned up, with the reason for each. SD-32 decides their disposition.
date: 2026-08-22
---

# Unmerged branches at the tranche/11 close

All fully-merged branches and worktrees were deleted. **Ten branches carry unmerged commits and were
kept.** Three categories, and they need different decisions.

**Where they live (verified 2026-08-22, launch-readiness remediation).** Nine of the ten exist
**only in the operator's local checkout** (`git branch`); only `sd31/racetrait4-SD31-E6-F4-005` is
on origin (`git ls-remote --heads origin`). Consequence for card 2 (`boundary-branch-review`): it
**must run in the primary checkout** — a fresh worktree cut from origin cannot see nine of these
branches. Re-derive before acting: `git branch --list 'worktree-wf_*' review-merge-test site-deploy
'fix/*' 'sd31/*'` and `git ls-remote --heads origin`.

Origin additionally carries branches this capture did not list: `worktree-wf_0628906e-65b-{1,3,4,6}`,
`worktree-wf_1ad13e3b-085-{4,5,6}`, `test`, `update-index`. Card 2 dispositions those too (merged →
delete; unmerged → name it here with a reason), and the §13 closure sweep re-checks.

## 1. Deliberately rejected — the GAMED lanes

Do **not** merge. Their rejections are recorded in `SD-31/artifacts/OPEN-ISSUES.md`; the branches are
kept only so the rejected implementation can be read if a future attempt wants to see what failed.

| Branch | What it is |
|---|---|
| `worktree-wf_13156488-c9b-1` | Wave 20's class-feature roster generalisation. GAMED — 0 of 19 units banked, 9 regressed test binaries. |
| `worktree-wf_a45ece26-3fc-1` | Wave 21's grant-data ingest. GAMED — discarded the true granting class and fabricated a level-1 grant for **73.4%** of its output. |

## 2. Real work orphaned by an orchestrator decision — REVIEW THESE

**These are not rejected. They were stranded**, and at least one is a correctness fix.

| Branch | What it is | Why it never merged |
|---|---|---|
| `worktree-wf_c1156061-e3f-3` | **W30 S6 residual sweep — closes a `gen_book_cache` self-erasure.** | The wave was killed 16 minutes in when the operator redirected to visibility. Same defect class as the generator that was destroying 2,110 fixture entries per run. **Highest priority of the three.** |
| `worktree-wf_c1156061-e3f-5` | W30 monster_ability/companion lane — sizes and kills the L3 bridge | Same killed wave |
| `review-merge-test` | W30 lane 6 — cross-kind visibility checks, no banking | Same killed wave |

The killed wave was the right call — it was building while the operator wanted measuring — but
killing a wave strands whatever its lanes had already finished. **That is a process gap worth naming:
a killed wave needs a salvage step, the way wave 25's three completed lanes were salvaged and merged
by wave 25b.** These three never got one.

## 3. Late-landing lane work

| Branch | What it is |
|---|---|
| `worktree-wf_cb84ba1e-439-2` | W31 lane 2 — prices sweeps S2 and S9; S2 CLOSED, S9 class-scope exhaustive |
| `worktree-wf_be4660f2-72a-3` | W26 retro correction — a `modelled_race_of_race_trait` claim in the dispatch was stale |

Both look mergeable on review. The W31 one carries a **closed sweep**, which by the standing rule
means a corpus-wide count exists for it — worth confirming it reached `todo/sweeps.md`.

## 4. Site branches, and the rescue branch

| Branch | Disposition |
|---|---|
| `site-deploy`, `fix/site-deploy-page-workflow` | Site publishing work, unrelated to the corpus grind. **Queued on card 2** (`kanban.md` #2): check each against `origin/develop` and `origin/main` by content; if already landed via the site-publish PRs (#366-#373), delete; if not, file a one-line disposition here for the operator. These are also the stale local "site" branches HANDOFF footgun 1 refers to — no `site-publish/*` branch exists locally. |
| `sd31/racetrait4-SD31-E6-F4-005` | **RESCUE branch. Never gated, never PI-screened, never merged on trust.** Standing instruction across the whole package — carry it forward untouched. |

---

## Recommended order for SD-32

1. **`worktree-wf_c1156061-e3f-3`** — review and merge if sound. A generator that silently erases its
   own output is the failure class SD-32's Gate 2 depends on not existing.
2. `worktree-wf_cb84ba1e-439-2` — confirm the closed sweep landed in the ledger.
3. The remaining two W30 lanes and the W26 correction — read, merge or discard with a reason.
4. Leave the two GAMED branches and the rescue branch alone.
