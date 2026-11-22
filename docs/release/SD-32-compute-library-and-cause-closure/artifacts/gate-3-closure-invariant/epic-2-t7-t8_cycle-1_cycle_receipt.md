# Cycle epic-2-t7-t8/1 — Gate 3 closure invariant / Epic 2 (cause closure) / Card 11 `epic-2-cause-closure`, lane T7+T8

- **Card ID:** `epic-2-cause-closure` (shared row; this is one of six concurrent lanes closing its
  sibling shapes on disjoint files — see `decisions.md §10` for why the card cannot be marked
  `complete` from a single lane).
- **Commit SHA:** (filled in at push — see this file's own commit in `git log`)
- **Files touched:**
  `src/rules_core/pilot_compute/class_feature_grant_consumer.rs` (T7 fix: `RawGrantFact.gate`
  field added, `resolvable_grants()` widened to refuse a bare-`PRECLASS:`-only pair with no
  `mod_row_*` corroboration, one new test, one pinned-count test updated with a dated note),
  `docs/retro/events/epic-2-t7-t8.jsonl` (new — 1 correction, 1 deferral),
  `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (card 11 row: `returned-to-backlog`
  → `in-progress`, T7/T8 status appended, **not** `complete`),
  `docs/release/SD-32-compute-library-and-cause-closure/progress.md` (this receipt cited).
  **`scripts/observer/pf1e_dashboard_producer.py` is NOT touched** — see T8 below.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  (`git diff --unified=0 1bb523773d32705d1b7387fd4c494861523f55ba...HEAD -- src/rules_core/pilot_compute/class_feature_grant_consumer.rs | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — no matches).
- **Wired-integration audit result:** `OK_NO_TOKENS`
  (same diff, `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` — no
  matches).
- **Acceptance criterion:** `acceptance-and-verification.md` AT-32-E2-001, T7/T8 clause verbatim:
  "T8/T7 (16 units together) close opportunistically" — ranked in `epic-breakdown.md`'s Epic 2
  table as T7 (D12, "Shallow single-hop traversal", 4 units) and T8 (D13, "Status stamp never
  re-examined once written", 12 units).
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`
  `PCGEN_ORACLE_SHA`) — re-fetched fresh into this worktree's repo-local oracle slot
  (`scripts/verify.sh --only preflight-oracle` FAILed empty-slot on a fresh worktree, self-healed
  per §8 via `scripts/fetch-pcgen-oracle.sh --dest <repo-local pcgen slot>`, re-verified PASS at
  the pinned SHA).
- **Status:** T7 **complete** (closed corpus-wide, RED→GREEN proven). T8 **prepared, not
  applied** — scope-boundary finding, operator ruling needed (see below). Card 11 overall stays
  `in-progress` — see `kanban.md`.

## T7 (D12) — closed

**Population re-derivation.** `defects.md` D12 named 4 raw `(class, key)` facts corpus-wide:
`Cleric ~ Channel Energy`, `Gunslinger ~ Gun Training`, `Druid ~ Wild Shape`, `Paladin ~ Smite
Evil`. Re-derived independently with a fresh census script over the live merged
`data/class_feature_grants/**/*.json` tree (every non-`granted_via_archetype` fact whose key group
equals its class, grouped by `(class, key)`, gate values collected per group):

```
python3 - <<'PY'
import json, glob
grants_root = "data/class_feature_grants"
def key_names_base(key, cls):
    return key.split(" ~ ")[0].strip().lower() == cls.strip().lower()
pairs = {}
for f in glob.glob(grants_root + "/**/*.json", recursive=True):
    with open(f) as fh:
        rows = json.load(fh)
    for r in rows:
        key, cls, level, gate, gva = r.get("key"), r.get("class"), r.get("level"), r.get("gate"), r.get("granted_via_archetype")
        if not (key and cls and level is not None) or not key_names_base(key, cls) or gva:
            continue
        pairs.setdefault((cls.lower(), key), []).append((level, gate))
preclass_only = {k: v for k, v in pairs.items() if {g for _, g in v} == {"preclass"}}
print("surviving pairs:", len(pairs), "preclass-only (uncorroborated) pairs:", len(preclass_only))
print(preclass_only)
PY
```

Output: `surviving pairs: 230, preclass-only (uncorroborated) pairs: 1` —
`{('gunslinger', 'Gunslinger ~ Gun Training'): [(1, 'preclass')]}`.

**Finding (logged as a `scripts/retro.py correction`, `docs/retro/events/epic-2-t7-t8.jsonl`):**
D12/T7's stated risk ("4 units... 3 of 4 protected only by an incidental level mismatch, not by
structure") is real for the *raw-fact* layer but does not account for
`class_feature_grant_consumer.rs`'s own `ANTI_FABRICATION_GATE_EXCLUDED_CLASSES` list (line
172-173), which already excludes `cleric`/`druid`/`paladin` (plus `wizard`/`bard`/`sorcerer`/`monk`)
from ever emitting via this module, for an unrelated, pre-existing reason (`OPEN-ISSUES.md` rows
330/338). So 3 of the 4 named pairs were **already double-protected** — by the level-mismatch
cross-book-conflict refusal *and* by class exclusion — before this cycle touched anything.
`gunslinger` is not in that exclusion list, so it was the one pair with live, structural risk.

**Root cause, traced to the actual corpus row (`ultimate_combat/uc_abilities_class.lst:1970`):**
the sole surviving `Gunslinger ~ Gun Training` fact is NOT a real top-level base-class declaration.
It is embedded inside a `CATEGORY:Internal` optional-house-rule row ("Guns Everywhere"):

```
Guns Everywhere ... KEY:Optional Rules Firearms ~ Guns Everywhere  CATEGORY:Internal ...
  ABILITY:Gunslinger Class Feature|AUTOMATIC|Gunslinger ~ Gun Training|PREVAREQ:Gunslinger_CF_GunTraining,0|PRECLASS:1,Gunslinger=1
```

`granted_via_archetype`'s single-hop check reads only the row that OWNS the grant token
(`CATEGORY:Internal`, correctly not `Archetype`), so it cannot see that the grant is nested inside
an unrelated optional-rule toggle rather than being a genuine standalone base-class row — the exact
"multi-hop chain the single-hop check cannot see" shape D12 names, one hop confirmed live (the
other three named pairs are the SAME embedded-in-another-ability shape — `uc_abilities_class.lst
:584`'s Evangelist archetype's "Sermonic Performance" row embeds the `Cleric ~ Channel Energy`
grant; `ui_abilities_class.lst:587`'s Paladin analogue — already protected by class exclusion, so
not re-investigated line-by-line here).

**Fix (closes by class, not by instance).** `resolvable_grants()` now tracks each raw fact's own
`gate` field (`preclass` / `mod_row_gated` / `mod_row_ungated`) alongside its level. A `(class,
key)` pair whose SURVIVING facts are ALL `gate == "preclass"` — i.e. no `mod_row_*` fact anywhere
corroborates it — is refused, exactly like the pre-existing cross-book-level-conflict refusal this
module already applies (`docs/…doc comment, "refuse rather than guess"`). This closes the shape
structurally: it no longer depends on a level coincidence (today's protection for 3 of the 4 named
pairs) or on the current contents of the class-exclusion list (which could change). Any future
corpus edit that produces a NEW bare-`PRECLASS:`-only, key-group-equals-class pair is refused by
the same rule, not merely the four pairs named in 2026.

**Cost check.** Refusing `Gunslinger ~ Gun Training` here changes zero player-visible values: the
real Gun Training magnitude is already served by a dedicated, hand-wired function
(`class_ultimate_combat.rs::gunslinger_gun_training_count`), called before this module's own call
site (`pilot_compute/mod.rs:8297`, "called LAST"); even had this pair still resolved, the
already-computed-slug guard (`push_generic_class_feature_grant_records`) would have suppressed the
duplicate id anyway (same trailing segment, `gun_training`).

**Fixture-check.** No new engine-emitted magnitude is introduced by this fix (it only REFUSES a
fact that was never player-visible to begin with — see cost check above); nothing new requires a
fixture. The refused fact's own oracle correctness was independently confirmed unaffected: the real
Gunslinger Gun Training value at level 1 is unchanged, still served by
`gunslinger_gun_training_count`, unrelated to this module.

**RED → GREEN (proven live, not asserted):**

1. RED: added `a_bare_preclass_only_pair_with_no_mod_row_corroboration_is_refused` (asserts
   `resolvable_grants()` does not contain `("gunslinger", "Gunslinger ~ Gun Training")`) against the
   PRE-fix code — fails (the pair resolves).
2. Implemented the `gates.len() == 1 && gates.contains("preclass")` refusal.
3. GREEN: same test passes.
4. Mutation proof: temporarily changed the refusal condition to `if false && gates.len() == 1 &&
   gates.contains("preclass")`, re-ran the ONE test — **FAILED** for the intended reason
   (`"an uncorroborated bare-PRECLASS: pair must never resolve -- T7/D12 regression"`, `src/rules_core
   /pilot_compute/class_feature_grant_consumer.rs:1100`). Reverted the mutation, re-ran — GREEN
   again.
5. Full suite: `cargo test --lib` (workspace) — **2365 passed, 0 failed, 13 ignored** (was 2364
   passed before this cycle's new test; +1 test, 0 regressions).
   `cargo test --lib rules_core::pilot_compute::` (this module's own package) — **844 passed, 0
   failed** (unchanged pass count minus the one new test, which is included; no other test's
   behavior moved).
6. **Pinned-count test moved, deliberately, and is explained inline where it moved.**
   `the_live_scale_of_this_waves_widening_is_measured_and_pinned`'s `already_admitted` pin moved
   `137 -> 136` — `("gunslinger", "Gunslinger ~ Gun Training")` no longer survives into
   `unambiguous_grants()`. The test's own doc comment ("change these ONLY with a concrete
   corpus/grant-data change that moves them... report them") is satisfied: this cycle is that
   concrete change, and the new pin's own comment cites this cycle and states the zero-player-impact
   finding above.

## T8 (D13) — prepared, NOT applied (scope-boundary finding, per the task brief)

**Population re-derivation**, over the live `docs/work-inventory.json` (25,312,688 bytes,
`generated_at` field unchanged from Gate-0/1's own closure):

```
python3 - <<'PY'
import json
with open("docs/work-inventory.json") as f:
    doc = json.load(f)
units = doc["units"]
cf = [u for u in units
      if u.get("wiring_class") == "display" and u.get("status") == "grounded"
      and u.get("kind") == "class_feature"]
print(len(cf))
for u in cf:
    print(u["id"], u.get("wiring_class_reason"))
PY
```

Output: **12**, all `core_rulebook`, all `wiring_class_reason == "no_magnitude_token"`:

```
core_rulebook:class_feature:barbarian_improved_uncanny_dodge
core_rulebook:class_feature:druid_timeless_body
core_rulebook:class_feature:druid_woodland_stride
core_rulebook:class_feature:monk_evasion
core_rulebook:class_feature:monk_improved_evasion
core_rulebook:class_feature:monk_timeless_body
core_rulebook:class_feature:ranger_evasion
core_rulebook:class_feature:ranger_improved_evasion
core_rulebook:class_feature:ranger_quarry
core_rulebook:class_feature:ranger_woodland_stride
core_rulebook:class_feature:rogue_evasion
core_rulebook:class_feature:rogue_improved_uncanny_dodge
```

Matches `defects.md` D13's own named examples exactly (Evasion, Improved Evasion, Timeless Body,
Woodland Stride, Quarry, Improved Uncanny Dodge and siblings) and the 12-unit count. **No
correction needed here** — D13's figure re-derives clean.

**Root cause.** `pf1e_dashboard_producer.py`'s `wiring_class` field is trusted verbatim from
`docs/work-inventory.json` (`compute_wiring_class_summary()`, the per-unit loop:
`wc = unit.get("wiring_class") or "ambiguous"`, line ~4133) and crossed against `status` in
`doneness_verdict()`'s `wiring_class == "display"` branch (line ~3988): `grounded` under `display`
returns `DONENESS_HELD`, permanently, with the branch's own comment naming the missing instrument
verbatim: "the instrument that would actually resolve this is a wiring-class classifier that checks
the full token closure GE-01 defines, which does not exist yet." All 12 units share
`wiring_class_reason: "no_magnitude_token"` — the classifier's single-hop check (does THIS record's
own row carry a magnitude token?) never considers that `status == "grounded"` is itself real
secondary evidence a live consumer already computes something from these exact records (each is a
genuine binary class feature — Evasion, Timeless Body, etc. — whose "magnitude" IS the flag itself,
which the engine demonstrably reads and acts on, per the `explanation_id_observed_in_a_real_computation`
evidence already stamped on every one of them).

**Why this cycle does not apply the fix.** `technical-design.md`'s own "What this bundle does not
touch" section names `scripts/observer/pf1e_dashboard_producer.py` as SD-30's Epic 0 surface,
read-only from SD-32. `AT-32-E2-001` calls T8 "opportunistic," but closing it as literally written
requires editing a file this bundle is explicitly scoped not to own — this is the scope-boundary
finding the task brief anticipated, not a difficulty deferral. Per this cycle's own instructions,
the diff below is prepared and pinned here, not applied.

**PROPOSED — not applied, pending operator write-scope ruling for
`scripts/observer/pf1e_dashboard_producer.py`:**

```diff
--- a/scripts/observer/pf1e_dashboard_producer.py
+++ b/scripts/observer/pf1e_dashboard_producer.py
@@ compute_wiring_class_summary()
+# T8/D13 (SD-32 card 11, epic-2-t7-t8 lane): 12 corpus-verified CRB
+# class_feature units the single-hop `no_magnitude_token` classifier
+# stamps `display`, even though `status == "grounded"` is itself real
+# evidence a live consumer already reads and acts on the record (the
+# exact "checks the full token closure GE-01 defines" instrument
+# doneness_verdict()'s own `display` branch names as not existing yet).
+# Named, corpus-grounded, closed allowlist -- NOT a general classifier
+# fix (the real one is the GE-01 token-closure instrument this comment
+# cites) -- re-derive with:
+#   python3 -c "import json; d=json.load(open('docs/work-inventory.json'));
+#   print([u['id'] for u in d['units'] if u.get('wiring_class')=='display'
+#   and u.get('status')=='grounded' and u.get('kind')=='class_feature'])"
+# If that command's output ever differs from this tuple, this allowlist is
+# stale and must be re-derived, not silently trusted.
+GROUNDED_DISPLAY_CLASS_FEATURE_RECLASSIFIED_AS_COMPUTED = frozenset({
+    "core_rulebook:class_feature:barbarian_improved_uncanny_dodge",
+    "core_rulebook:class_feature:druid_timeless_body",
+    "core_rulebook:class_feature:druid_woodland_stride",
+    "core_rulebook:class_feature:monk_evasion",
+    "core_rulebook:class_feature:monk_improved_evasion",
+    "core_rulebook:class_feature:monk_timeless_body",
+    "core_rulebook:class_feature:ranger_evasion",
+    "core_rulebook:class_feature:ranger_improved_evasion",
+    "core_rulebook:class_feature:ranger_quarry",
+    "core_rulebook:class_feature:ranger_woodland_stride",
+    "core_rulebook:class_feature:rogue_evasion",
+    "core_rulebook:class_feature:rogue_improved_uncanny_dodge",
+})
+
     for unit in doc.get("units") or []:
         # No wiring_class on a unit is itself a gap, not a zero -- report it
         # under "ambiguous" rather than dropping the unit from the count.
         wc = unit.get("wiring_class") or "ambiguous"
+        if unit.get("id") in GROUNDED_DISPLAY_CLASS_FEATURE_RECLASSIFIED_AS_COMPUTED:
+            wc = "computed"
         corpus_wide[wc] = corpus_wide.get(wc, 0) + 1
```

This reclassifies exactly the 12 named, re-derivable units from `display` to `computed` at
tally-time, before `doneness_verdict()` ever runs — so `computed` + `grounded` -> `DONE` (the
existing, unmodified rule) fires for them with **zero change to `doneness_verdict()`'s own code**.
It does not touch the upstream `wiring_class` field in `docs/work-inventory.json` itself (out of
this fix's reach — that field is GE-01's own determinator's output) and does not attempt to be a
general classifier fix; it is a named, closed, re-derivable allowlist for the corpus-verified 12,
with the real fix (the GE-01 token-closure instrument) still cited as owed follow-on work. A
`WIRING_SUMMARY_SCHEMA` bump would be needed alongside this change (the cache-invalidation
convention this file already uses whenever `doneness_verdict()`'s output for a fixed input changes,
per this file's own precedent at the `WIRING_CLASS_CACHE` schema-version comment) — noted here, not
applied, since the diff itself is not applied.

**Precise ruling needed:** grant SD-32 (or a named successor bundle) write scope to
`scripts/observer/pf1e_dashboard_producer.py` for this one classifier fix.

**Retro events:** `docs/retro/events/epic-2-t7-t8.jsonl` — 1 `correction` (T7's re-derived risk
population), 1 `deferral` (T8, naming the exact ruling needed and the revisit condition).

- **Discovery forwards:** none requiring a new card — T7/T8 are both named, scoped items against
  the existing card 11.
- **Next-cycle plan:** T7 is closed; no further work. T8 is fully prepared (diff above) and
  requires only the named operator ruling before another cycle applies it verbatim (or a successor
  bundle scoped with write access to `scripts/observer/`). Card 11's five other lanes (T2a+T12,
  T2b, T9, T4, and any consolidation cycle) are out of this lane's scope.
