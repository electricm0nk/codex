# Proposal for review — partition remaining ingestion work by *kind*, not by *book*

**Status:** proposal, not a decision. Written on `tranche/8`, 2026-08-09, from the SD-28 session log
and live measurements against `docs/work-inventory.json` at `160f5e31`.
**Author:** SD-28 orchestrator. **Audience:** whoever scopes SD-29 and SD-30.
**Prompted by:** operator question — *"given the remaining books, do we need to refactor our
approach; not go by book but instead by the type of work and parallelization."*

The short answer is yes, and the evidence below is stronger than I expected when I started
checking. But the recommendation depends on one prerequisite refactor (§4) without which per-kind
parallelism does not actually work.

---

## 0. How to reproduce every number here

```bash
cd ~/workspace/repos/codex

# unit totals by kind and by book
python3 - <<'PY'
import json, collections
U = json.load(open('docs/work-inventory.json'))['units']
print(collections.Counter(u.get('kind') for u in U).most_common())
PY

# the four shared chokepoint files
wc -l src/bin/v06_work_inventory.rs src/bin/v06_content_state_dump.rs \
      apps/desktop/src-tauri/src/reach_gate.rs \
      apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs

# kind-scoped clusters
ls src/rules_core/feat_*.rs src/rules_core/feat_prereqs/
ls src/rules_core/rules_tables/crb/
```

All figures below are from that inventory at `160f5e31` unless stated.

---

## 1. Why "book" was never a real boundary

Every book slice this session reached into the same count-pinning files. Book looked like a
territory because each book has its *own* data table, but the data table was never the contested
resource — the sweep files were. Two agents working different books collide on the seventh file
they both have to touch.

The measured cost of that boundary:

| slice | records landed | fixed sweep cost |
|---|---|---|
| UM equipment | 26 | full per-kind tax (~7 files) |
| UPsi equipment | 439 | full per-kind tax (~7 files) |

**The tax is per-file per-kind and constant regardless of record count.** UM paid the same price for
26 records that UPsi paid for 439. Across the ~23 books remaining in SD-29 (7) and SD-30 (16), a
per-book partition pays that tax up to 23 times per kind. A per-kind partition pays it **once**.

This is not a new observation — it is recorded as
`book-onboarding-tax-is-per-file-not-per-record` — but it was treated as a costing note rather than
as an argument about how to partition the work. It is the latter.

---

## 2. Kind *is* a real boundary

The file structure already partitions by kind almost cleanly:

```
KIND-SCOPED — naturally disjoint, safe for concurrent writers in different lanes
  feat        feat_effects.rs · feat_identity.rs · feat_prereqs/ · rules_tables/*/feats_all.rs
  equipment   equipment_resolver.rs · rules_tables/*/equipment_tables.rs
  race_trait  rules_tables/*/race_tables.rs
  spell       rules_tables/*/[class]_spell_list.rs

SHARED — the only genuine chokepoints, four files
  v06_work_inventory.rs         3,004 lines
  reach_gate.rs                 2,194
  corpus_ingest_diagnostic.rs     933
  v06_content_state_dump.rs       836
```

Two agents on feats collide. One agent on feats and one on equipment share **only those four
files**. That is a materially better partition than book, which shares all seven.

---

## 3. The learning also transfers by kind, not by book

Every corpus hazard found this session was kind-shaped, and each was discovered once and then
*rediscovered* when the next kind was attempted:

- `.MOD` unconditional recovery (UC `Revelation Strike`) — §46
- `.MOD` conditional variant (APG `Deadly Aim`, `PREABILITY:`-gated) — §48
- never-join (UM raw `BONUS:` tokens leaking into descriptions) — §49
- `.COPY=` aliasing, two sub-shapes (genuine variant vs legacy alias) — §58
- the `ABILITY:` grant grammar — 29 category families, ruled individually

A per-kind pass front-loads that discovery **once per kind** instead of paying re-discovery per
book. The per-book cost model already predicted "one unplanned corpus-shape finding per book" and
held for four books before UC produced three; per-kind, most of those findings collapse into a
single grammar pass at the front of the lane.

---

## 4. Prerequisite: the four chokepoint files must derive their counts

**Per-kind lanes do not actually parallelize until this is done.** All four shared files carry
hand-pinned aggregate counts, which is why every slice serializes through them.

The fix is the one already proven in this bundle. `646aea2b` changed `equipment_keys` from a
hand-maintained four-book list to a derivation from `equipment_resolver`'s own chain, with a guard
test — and it **caught a real regression automatically within the hour** when UM's table landed.

Applying the same treatment to the four files removes the last serialization point. Until then, the
honest shape is *fan out generation, serialize the sweep* — which is still a large win, but not the
full one.

This also retires the pattern the session logged **nineteen** times: a hand-maintained list beside a
derivable one, diverging silently. The four chokepoints are the largest remaining instances.

---

## 5. Recommended shape

1. Finish the current SD-28 queue (`file_kind()`, dashboard justification table, archetype wiring,
   picker + UE, SD-27 ownership).
2. Land the four-file derivation refactor (§4) as its own item, with guard tests.
3. Re-scope SD-29 and SD-30 as **kind lanes**, not book lists. One writer per lane; lanes run
   concurrently. Within a lane, fan out per-book extraction and serialize only the table landing.
4. Run a grammar/hazard pass at the head of each lane before any ingestion, producing the
   enumeration up front rather than discovering it book by book.

---

## 6. Honest costs and open questions

- **Provenance.** Per-book receipts currently carry the OGL/licensing story. Per-kind lanes need a
  different provenance record, and licensing is not a place to improvise — this needs a deliberate
  answer before the first lane runs.
- **Cross-book KEY collisions** become *easier* to catch (all books in one kind, examined together)
  but the check has to move out of the per-book slice. Do not lose it in the move; it caught real
  duplication in three books.
- **`class_feature` is unaffected.** 18,057 units, 42% of the program, blocked behind archetypes and
  class chassis regardless of partitioning. Kind lanes do not unblock it.
- **`companion` — 1,915 units, zero progress of any kind.** No lane currently owns it.
- **Sequencing against SD-29.** SD-29 is the Bestiary line, and Bestiary is exactly where filename
  mis-typing put ~620 monster abilities in the `race_trait` bucket. `file_kind()` should land before
  SD-29 starts, or SD-29 will be scoped off known-bad figures.

---

## 7. What I am least sure about

The per-kind economics are measured and I stand behind them. Two things are argued rather than
measured:

- **That the four-file refactor is tractable.** It is the same shape as `646aea2b`, but those files
  are larger and their pins are more varied. Someone should cost it before it is committed to.
- **That concurrent lanes stay collision-free in practice.** The partition is clean on paper. This
  checkout has burned cycles on boundary violations before, and the mitigation that worked was
  file-level territory lists derived from `git status`, not directory globs
  (`agent-territories-must-be-file-level`). Lanes should be expressed the same way.

A note on the evidence standard: this session produced **seven instrument failures**, three of them
mine — proxies that measured a naming convention rather than the thing itself. Every figure in this
document is derived from the live inventory or from `wc -l`, not from recollection, and §0 exists so
the next reader can re-derive rather than inherit.
