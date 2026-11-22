# C2 graphify receipt — BLOCKED (acceptance guard tripped; stopped as the card instructs)

Card C2 (lane main, sonnet). Status: blocked-escalated. Lead's `graphify-out/` was NOT modified.

## RED
`test -f docs/release/SD-43-market-inventory-and-abilities/artifacts/cycle_0/C2-graphify-receipt.md` → exit 1 before this commit (matches the lead's RED).

## Tree
`graphify-out` is gitignored (`.gitignore:10`) and exists only in the lead checkout. To avoid writing there, it was copied (`cp -a`) into the lane tree, a snapshot taken (`graphify-out/2026-11-22-pre-c2/`), the run done on the copy, then the copy removed. Lead copy re-counted afterwards: unchanged.

## Command, exit, counts
- Command: `graphify extract . --code-only --exclude node_modules,target,dist,build,.git,out,dist-ssr,.next,coverage` (cwd = lane tree; no `--force`, not `graphify update`; the manifest-gated incremental form of `extract`; AST only, no LLM, no API key present in env).
- Exit: 0. Log: `/home/ubuntu/c2_extract.log` (outside repo).
- Tool summary: `117974 re-extracted, 12692 cached/unchanged, 731 deleted`; "Replaced 361406 node(s)"; "Pruned 10457 node(s)"; wrote 322949 nodes, 378281 edges, 14744 communities.
- Before: `python3 -c "import json;print(len(json.load(open('graphify-out/graph.json'))['nodes']))"` → 648328 (lead copy and snapshot agree).
- After: same command → 322949 (49.8 % of before; `awk 'BEGIN{print 322949/648328*100}'` → 49.8). Acceptance floor 90 % FAILED.
- Restore: lane `graph.json` replaced from the dated snapshot → 648328; lead copy → 648328.

## closure check
FILES YOU OWN: `graphify-out/**` and this receipt. No code producers or second call sites (lead's grep: docs/release/SD-43* only). No file outside the list written. Tool also wrote `graphify-out/2026-11-22/` backup, in the discarded copy.

## Mechanism (why it fell)
`--code-only` skips 5232 non-code files (4459 docs, 51 papers, 722 images), so the semantic nodes derived from docs are absent; it also reported 117366 "uncached" files because the AST cache key differs from the one that built the 2026-09 graph. The result is an AST-only graph, the same hazard class as memory "graphify --force replaces the semantic graph". The tool's own node-count guard did not fire (it ran in extract mode and wrote).

## Needed from the operator
A ruling on how to refresh the semantic graph without LLM-less loss: either (a) run `graphify extract` WITH a semantic backend (`graphify provider list` shows omniroute and llamacpp; the corpus would go to those endpoints; token cost unbounded, last run used ~1.8M of a 2M quota), run in the lead checkout; or (b) accept `cluster-only` as C2 (last done 2026-11-17, refused by the guard at -1 node) which does not add SD-42/43 files. Closure graphify (E7.4) remains the final run.

## Does not cover
The closure graphify (E7.4); a semantic (LLM) incremental run (not attempted: no key in env, endpoint choice is an operator ruling); the lead checkout's graph freshness (still the 2026-11-17 graph).
