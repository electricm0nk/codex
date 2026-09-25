"""Doneness classification: the (wiring_class, status) -> doneness verdict
table, and EXCLUDED_BOOKS -- extracted from `pf1e_dashboard_producer.py`
(SD-36 D3, 2026-09-15) so `scripts/coverage_ledger.py`, the one live
caller that needs only this piece, does not need to import the whole
5,000+-line producer module, most of which nothing calls any more now that
`scripts/publish-site-dashboard.sh` and `v06_work_inventory.rs` are retired.

This is a verbatim extraction, not a rewrite: every doc comment, every dated
QA-round citation, and every operator-ruling reference below is preserved
exactly as it stood in the producer, because the reasoning behind each rule
is exactly what stopped this classification table from drifting silently in
the past (see each comment's own history). `pf1e_dashboard_producer.py`
itself is deliberately left UNCHANGED (not rewired to import from here) --
its own kept test suite (`scripts/tests/test_pf1e_dashboard_producer.py`,
the `producer-selftest` verify.sh stage) exercises its copy directly, and
touching a 5,000+-line file with its own 1,000+-line test suite is a
bigger, riskier change than this extraction's own scope. This module is
therefore a second, independent copy of the same logic, proved identical to
the producer's own `doneness_verdict` across every (wiring_class, status,
kind) combination both modules define -- 1,300 of 1,300 match (5
`WIRING_CLASS_VALUES` x the 13 status words this function's own branches
resolve without raising x the 19 `kind` values in `docs/work-inventory.json`
plus `None`), re-derived with:

    python3 -c "
    import sys,json,itertools
    sys.path.insert(0,'scripts/observer')
    import doneness as D, pf1e_dashboard_producer as P
    inv=json.load(open('docs/work-inventory.json'))
    kinds=sorted(set(u.get('kind') for u in inv['units']))+[None]
    ws=list(P.WIRING_CLASS_VALUES)
    ss=['deferred-with-reason','engine-does-not-hold','not-started','sheet-complete',
        'unknown','unmeasurable','grounded','text-complete','ingested-magnitude',
        'literal-verified','fixture-verified','oracle-agree','oracle-unverifiable']
    combos=list(itertools.product(ws,ss,kinds))
    mism=[c for c in combos if D.doneness_verdict(*c)!=P.doneness_verdict(*c)]
    print('combos=%d mismatches=%d excluded_books_equal=%s' % (len(combos), len(mism), D.EXCLUDED_BOOKS==P.EXCLUDED_BOOKS))
    "
    # -> combos=1300 mismatches=0 excluded_books_equal=True

a future cycle that actually deletes the producer should treat this module
as the surviving, canonical one.
"""

# CLOSED 2026-08-24 (`decisions.md §27b`, operator ruling 2026-08-23:
# "EVERYTHING" -- no carve-outs survive). This constant used to read
# `{"beginner_box"}` on a 2026-08-02 operator directive ("genuinely
# simplified intro subset ... ruled out of scope"). That directive did not
# survive §27b: the only admissible reasons a unit may sit outside every
# closure figure are a hard impossibility -- the source data does not
# exist, or licensing forbids shipping it -- and neither holds for
# `beginner_box`. Its 19 equipment units are real, declared records sourced
# from the pinned PCGen oracle's own `bbox_equip_magic_items.lst` /
# `bbox_equip_arms_armor.lst` (`data/pathfinder/paizo/roleplaying_game/
# beginner_box/`, verified present at `PCGEN_ORACLE_SHA`
# 7f818006e371188e5717fd18d74d18a420747fc6) -- "genuinely simplified" is a
# cost/awkwardness judgment, exactly what §27b names as inadmissible. They
# were already flowing into `docs/work-inventory.json` as `not-started`
# units (evidence `no_compiled_rule_set_for_book`); the carve-out lived
# ONLY in this dashboard-reporting layer, hiding them from every
# denominator rather than reporting them honestly as not-done. Kept empty
# now rather than deleted so the mechanism survives for a FUTURE
# genuinely-admissible exclusion -- but any future entry here must carry a
# paired, admissible reason in `EXCLUDED_BOOKS_REASONS` below, checked at
# import time, so the next carve-out cannot hide silently in code the way
# this one did (`decisions.md §27b`'s own diagnosis: "it survived every
# prose sweep because it lives in Python rather than in a document").
#
# Historical note (why this constant exists at all): every corpus-wide
# figure this producer emits reads from this one constant -- by_status,
# by_kind, by_wiring_class, by_doneness, by_doneness_kind, cross_tab, and
# the unit-search shard index -- so a book cannot silently stay excluded
# from some rollups and not others the way `cross_tab` and
# `build_unit_shards()` did in round 1 (SD-29 QA findings #7/#8, round 2,
# 2026-08-12: both drifted from this set and their totals stopped matching
# the by-lane figures by exactly beginner_box's count).
EXCLUDED_BOOKS: frozenset[str] = frozenset()

# The only reasons `decisions.md §27b` admits for a book to sit in
# EXCLUDED_BOOKS. Anything else is a cost/awkwardness/novelty judgment and
# must be escalated for an operator ruling instead, per the same decision.
ADMISSIBLE_EXCLUSION_REASONS = frozenset({
    "source_data_absent",
    "licensing_forbids_shipping",
})

# book -> admissible reason. Every key of EXCLUDED_BOOKS must appear here
# with a value drawn from ADMISSIBLE_EXCLUSION_REASONS; the assertion right
# below enforces it at import time so a future carve-out cannot be added to
# EXCLUDED_BOOKS alone without also declaring, in writing, why it qualifies.
EXCLUDED_BOOKS_REASONS: dict[str, str] = {}

assert set(EXCLUDED_BOOKS) <= set(EXCLUDED_BOOKS_REASONS), (
    "EXCLUDED_BOOKS entries missing a declared reason in "
    f"EXCLUDED_BOOKS_REASONS: {sorted(set(EXCLUDED_BOOKS) - set(EXCLUDED_BOOKS_REASONS))}"
)
assert all(reason in ADMISSIBLE_EXCLUSION_REASONS for reason in EXCLUDED_BOOKS_REASONS.values()), (
    "EXCLUDED_BOOKS_REASONS carries a reason outside "
    f"ADMISSIBLE_EXCLUSION_REASONS: {EXCLUDED_BOOKS_REASONS}"
)


# --- Doneness ladder -------------------------------------------------------

DONENESS_DONE = "done"
DONENESS_HELD = "held"
DONENESS_IN_PROGRESS = "in-progress"
DONENESS_NOT_STARTED = "not-started"
DONENESS_UNMEASURABLE = "unmeasurable"
DONENESS_DEFERRED = "deferred"

# Ladder order: best-evidence first, then the two buckets that are not on the
# ladder at all (`unmeasurable`, `deferred`) last. The viewer renders in this
# order; do not re-sort by count.
DONENESS_VALUES = (
    DONENESS_DONE,
    DONENESS_HELD,
    DONENESS_IN_PROGRESS,
    DONENESS_NOT_STARTED,
    DONENESS_UNMEASURABLE,
    DONENESS_DEFERRED,
)

# Kinds the inventory has NO consumer-delta probe for at all -- `grounded`
# would be unreachable for them by construction, not merely unobserved yet.
#
# EMPTIED (SD30-E0-F2, 2026-08-14). This tuple's own justifying comment
# ("`companion` and `spell` alone read `grounded: 0`") is now FALSE, checked
# against the live `docs/work-inventory.json` this cycle, not transcribed:
# `computed`-wiring-class `companion` reads 416 `grounded` of 793, and
# `computed`-wiring-class `spell` reads 46 `grounded` of 210
# (`v06_work_inventory.rs`'s `Kind::Companion`/`Kind::Spell` verdict arms,
# `facts.holds_key`/`facts.spell_effect_wired` respectively). Both kinds'
# consumer-delta check already exists and already lands nonzero `grounded`,
# so per this card's own acceptance bar ("the cap is removed for a kind once
# its probe lands AND is confirmed reaching a nonzero `grounded` count under
# the `computed` class for that kind") neither belongs in this tuple any
# longer. `companion`'s cap was already inert regardless (its `computed`
# population is a strict `{grounded, engine-does-not-hold}` two-way split with no
# `in-progress`-shaped status to cap -- `build_companion_catalog()` in
# `apps/desktop/src-tauri/src/companion_catalog.rs` is a proven bijection
# over `companion_chassis::COMPANION_BOOKS`, own test
# `the_catalog_serves_every_registered_companion_creature`, so no unit can
# be "in the table but not grounded"). `spell`'s cap DOES move real units:
# emptying it reclassifies 132 `computed`+`ingested-magnitude` spell units
# from `held` to `in-progress` (verified by replaying `doneness_verdict()`
# over `docs/work-inventory.json` with and without the cap, this cycle) --
# a more honest board position, not a `done` gain (spell's `done` count is
# unchanged: `computed`+`grounded` was never subject to this cap, only
# `computed`+non-`grounded` was).
#
# Left as an empty tuple, not deleted, so a FUTURE kind whose probe
# genuinely cannot exist yet (there is none on record as of this cycle) has
# somewhere to be listed without re-deriving this comment's reasoning from
# scratch. Still single-sourced on the payload
# (`work_inventory.no_grounding_probe_kinds`) exactly as before; see
# `doneness_verdict()` below for how it is used, and PF1e-dashboard.html's
# `NO_GROUNDING_PROBE` for the client-side read -- that file's own fallback
# guard (`if (ngp.length) NO_GROUNDING_PROBE = ngp`) was fixed in the SAME
# change (SD30-E0-F2) to honor an explicitly-EMPTY shipped list rather than
# silently keeping its stale `["companion", "spell"]` default, which an
# emptied list would otherwise never be able to override.
NO_GROUNDING_PROBE = ()


def doneness_verdict(wiring_class: str, status: str, kind: str | None = None) -> str:
    """Cross `wiring_class` with `status` into one doneness verdict.

    Raises on any pair it has no rule for. That is the point: a new
    `wiring_class` value or a new status word must force this table to be
    updated rather than silently landing in whichever bucket a default picked,
    which is how `static`'s finished work became invisible in the first place.

    `kind` is optional and, when given, caps the result: `in-progress`'s own
    definition requires "the bar is reachable with an instrument that
    exists", which is false for any `kind` in `NO_GROUNDING_PROBE` -- those
    kinds can never reach `grounded` no matter how complete their data is, so
    "in progress toward a `grounded` verdict" is not an honest description
    for them. Applied structurally at the end of this function (round 8,
    SD-29 QA finding, 2026-08-12) rather than by hand-tuning the individual
    `display`/`computed` cells that produce `in-progress`, so a future new
    wiring_class/status combination that lands in `in-progress` is capped
    automatically rather than needing its own carve-out.
    """
    verdict = _doneness_verdict_uncapped(wiring_class, status)
    if verdict == DONENESS_IN_PROGRESS and kind in NO_GROUNDING_PROBE:
        return DONENESS_HELD
    return verdict


def _doneness_verdict_uncapped(wiring_class: str, status: str) -> str:
    """The (wiring_class, status) table `doneness_verdict()` caps by kind."""
    if status == "deferred-with-reason":
        return DONENESS_DEFERRED
    if status in ("engine-does-not-hold", "not-started"):
        return DONENESS_NOT_STARTED
    # `sheet-complete` (SD-35 AT-35-E2-003; `decisions.md §1`, the sheet
    # rule; `technical-design.md §3`): the record's `SheetRule` renders for a
    # probe character that holds it -- a final number, dice in final form, or
    # the rule's words -- and the kind's on-screen test covers it. That is the
    # terminal state for EVERY wiring class: the sheet rule's bar is the same
    # bar whether the unit is display, static, derived or computed, so the
    # `ambiguous` lower-bound rule below (never more favourable than the
    # least favourable class for the same status) also yields `done` here.
    # Resolved before the per-class dispatch for the same reason
    # `deferred-with-reason`/`not-started` are: the verdict does not depend
    # on the class.
    if status == "sheet-complete":
        return DONENESS_DONE
    # An `unknown`/`unmeasurable` status cannot be measured against any bar,
    # classifiable or not -- checked first, ahead of both the ambiguous check
    # and the per-class rules below. `AT-33-E4-002` (2026-08-25) renamed the
    # STATUS_VOCABULARY word itself from `unknown` to `unmeasurable` (the 318
    # genuinely-irreducible units keep this exact disposition, only the
    # string changed, "so the status string itself stops reading as 'nobody
    # looked'" -- that commit's own message) but this one call site, and only
    # this one, was never updated to match: `unknown` no longer appears
    # anywhere in `STATUS_VOCABULARY`
    # (`src/bin/v06_work_inventory.rs`) or in any real
    # `docs/work-inventory.json` unit, so this branch alone had gone
    # unreachable for every live unit -- including the 11
    # `('ambiguous', 'unmeasurable')` units the `ambiguous` branch below
    # raises `ValueError` on, and the 310 `('display', 'unmeasurable')` units
    # that silently fall through the `display` branch's catch-all into
    # `in-progress` instead of the honest `unmeasurable` -- both populations
    # re-derived live:
    # `python3 -c "import json,collections;u=json.load(open('docs/work-inventory.json'))['units'];print(collections.Counter((x.get('wiring_class'),x.get('status')) for x in u if x.get('status')=='unmeasurable'))"`
    # -> `Counter({('display', 'unmeasurable'): 310, ('ambiguous', 'unmeasurable'): 11})`.
    # `unknown` is kept alongside `unmeasurable` rather than replaced outright
    # -- a frozen/older `work-inventory.json` snapshot generated before this
    # rename (e.g. an archived receipt's embedded fixture) still legitimately
    # carries the old word, and this function's whole design is "never
    # silently reinterpret a status word", not "assume every caller regenerated
    # today".
    if status in ("unknown", "unmeasurable"):
        return DONENESS_UNMEASURABLE
    # `ambiguous` wiring_class is a classifier failure on the CONSUMER side --
    # the determinator could not tell how the unit is wired in, so there is no
    # class-specific bar to check its evidence against. Trace the ACTUAL
    # control flow to see what can still be sitting here: `deferred-with-
    # reason`, `engine-does-not-hold`/`not-started` and `unknown` have all already
    # returned above, so the only statuses that can reach this line are
    # `grounded`, `text-complete` and `ingested-magnitude` -- i.e. every
    # remaining case IS real evidence of some tier, never a status this
    # instrument failed to read at all.
    #
    # Round 2 fix (SD-29 QA finding #4, 2026-08-12) correctly stopped
    # discarding that evidence into `unmeasurable` (the old order threw away
    # 280 demonstrably-proven records -- 175 race_trait `grounded`, 71 feat
    # `text-complete`, 23 equipment `text-complete`, 7 race `grounded`, plus 4
    # others -- purely because the wiring classifier separately failed) but
    # then routed ALL of it to `done`, which is the over-claim SD-29
    # decisions.md §46.4 exists to prevent. Round 3 (SD-29 QA finding #15/#16)
    # overcorrected the other way and routed ALL of it to `held` uniformly --
    # close, but it undersold `display`-shaped evidence elsewhere in this
    # function, so round 4 (SD-29 QA finding F26, 2026-08-12) tried a
    # three-way split by STATUS tier and routed `grounded` specifically to
    # `done`, on the theory that `grounded` (an OBSERVED consumer delta) is
    # strong enough on its own to clear every bar in the lattice regardless
    # of which wiring_class turns out to apply.
    #
    # That theory is FALSE, and it was refuted by this function's own
    # `static`/`derived` branch below: `static`/`derived` + `grounded` ->
    # `held`, because `static`/`derived` carry a magnitude-fidelity bar that
    # `grounded` evidence alone does not clear. So `grounded` does NOT exceed
    # every bar -- it exceeds `display`'s and `computed`'s bars but NOT
    # `static`/`derived`'s. An `ambiguous` record could BE a `static` or
    # `derived` record; routing its `grounded` evidence straight to `done`
    # crediting it with more confidence than a record whose wiring_class is
    # actually KNOWN to be `static`/`derived` is not a hedge, it's a reward
    # for not knowing.
    #
    # Round 5 (SD-29 QA finding #1, 2026-08-12) fixes this with the
    # lower-bound rule that governs the rest of this function everywhere
    # else (e.g. `unknown` -> `unmeasurable`): when wiring_class is
    # unresolved, the verdict must never be MORE FAVORABLE than what the
    # `static`/`derived` fidelity bar -- the specific bar this finding is
    # about -- guarantees for the SAME status. For `grounded` that bar is
    # `held` (the `static`/`derived` branch below returns `held` for it),
    # which is exactly the ceiling `ambiguous`+`grounded` must respect.
    # `text-complete`/`ingested-magnitude` were already `held` here since
    # round 3 (SD-29 QA finding #15/#16) and stay unchanged -- that mapping
    # is not this finding's scope, and re-deriving it from scratch this round
    # (e.g. against `computed`'s stricter `in-progress` for those two
    # statuses, a genuinely different, pre-existing axis untouched across 4
    # rounds of QA) would invent a NEW doctrine change nobody asked for, the
    # exact "re-deciding the core doctrine every round" pattern this round
    # exists to stop. So the entire `ambiguous` bucket collapses to one rule,
    # no per-status special-casing: any evidence-bearing status under an
    # unresolved wiring class is `held`.
    if wiring_class == "ambiguous":
        # `literal-verified`/`fixture-verified` are evidence-bearing statuses too: the stamp
        # proves the LITERAL matched (static's bar) or the EVALUATOR matched (derived's bar),
        # neither of which is known to be THIS unit's bar while its class is unresolved -- the
        # unit could be `computed`, whose bar (an observed consumer delta) neither stamp meets.
        # Same lower-bound rule as `grounded` above: never more favourable than the least
        # favourable class the unit could turn out to be. The generator cannot emit this cell
        # (its stamping loops are gated on Static/Derived and re-derived every run); it can only
        # arise from an in-place `wiring_class` rewrite, and on the next regen the stamp goes
        # away and the unit reads `held` anyway -- so `held` is the one verdict that does not
        # depend on which tool ran last. (Launch-readiness remediation Step 4D, blocker B6.)
        if status in ("grounded", "text-complete", "ingested-magnitude",
                      "literal-verified", "fixture-verified",
                      "oracle-agree", "oracle-unverifiable"):
            return DONENESS_HELD
        raise ValueError(f"doneness: unmapped {wiring_class!r} + {status!r}")
    if wiring_class == "display":
        # `text-complete` is display's actual bar -- the description is
        # present and renders, full stop.
        #
        # `grounded` and `ingested-magnitude` are BOTH disagreement signals,
        # not favorable ones -- but round 7 (SD-29 QA, 2026-08-12) found
        # round 6 collapsed them to the SAME bucket (`in-progress`), which is
        # wrong for `grounded` specifically, for two independent reasons.
        # `display` means the determinator found no magnitude anywhere on
        # the unit. `ingested-magnitude` says "the generator found one
        # anyway, but the consumer-delta probe that would confirm it has not
        # been RUN yet" -- an instrument that exists and is reachable, just
        # not yet exercised. That is a genuine `in-progress`: short of the
        # bar, with a path to close the gap.
        #
        # `grounded` is a different kind of disagreement: the consumer-delta
        # probe WAS run, and it produced evidence that contradicts the
        # `display` classification outright -- a real consumer computed
        # something from this unit, which a magnitude-absent record cannot
        # produce (see e.g.
        # `advanced_class_guide:class_feature:bloodrager_indomitable_will`,
        # classed `display` by a single-row magnitude check even though its
        # `type_facet` shows it inherits magnitude from the Bloodrage/Rage
        # rows -- `computed`-shaped content misclassified as `display`).
        # Routing that contradiction to `done` (round 5) rewarded the
        # classifier's blind spot; routing it to `in-progress` (round 6) is
        # ALSO wrong, because `in-progress`'s own definition below requires
        # "the bar is reachable with an instrument that exists" -- and the
        # instrument that would actually resolve this is a wiring-class
        # classifier that checks the full token closure GE-01 defines, which
        # does not exist yet. That is exactly the same "needs a working
        # classifier before the bar is even known" situation `ambiguous`
        # evidence is in (see the `ambiguous` branch above), so it gets the
        # same verdict: `held` -- as done as the current instruments can
        # prove, and deliberately not counted as done or as a reachable
        # in-progress item.
        return (DONENESS_DONE if status == "text-complete"
                else DONENESS_HELD if status == "grounded"
                else DONENESS_IN_PROGRESS)
    if wiring_class in ("static", "derived"):
        # `literal-verified` / `fixture-verified` are the done rung SD-29
        # decisions.md §46.4 and SD-32 decisions.md §2 both named as missing:
        # the generator emits these ONLY for a unit whose corpus literal was
        # actually byte-compared clean by `corpus_literal_sweep` (`static`) or
        # whose evaluator actually matched its pinned fixture
        # (`derived_evaluator_fixture_check`) -- never for a unit that merely
        # carries `ingested-magnitude`/`grounded`/`text-complete`, which stay
        # `held` exactly as before. The word is new and strictly stronger so
        # it can never be produced by an old inventory or confused with
        # `grounded`, which means something else (a consumer-delta
        # observation, not a literal/evaluator check). Operator directive
        # 2026-08-13 ("add the done rung for static and derived"), answering
        # SD-32 decisions.md §2's open question.
        # `oracle-agree`/`oracle-unverifiable` (SD-34 AT-34-E3-005, 2026-08-31)
        # are REFINEMENTS of these two words, never a separate tier: the
        # inventory's own vocabulary defines each as "a `literal-verified`/
        # `fixture-verified` unit whose id carries a real <verdict> in the
        # consolidated bucket-V oracle ledger", and
        # `src/bin/v06_work_inventory.rs` treats all four as one family in a
        # single list. So the unit had already met static's bar (the literal
        # byte-compared clean) or derived's (the evaluator matched its pinned
        # fixture) BEFORE the oracle looked at it.
        #
        # `oracle-agree` is strictly stronger -- a real PCGen round-trip
        # matched the engine's own computed value exactly.
        #
        # `oracle-unverifiable` is NOT weaker. It means the oracle could not
        # check the unit at all (`no_bonus_chain`, `no_probe_surface`,
        # `oracle_export_no_spellname_line`) -- a limitation of the oracle,
        # not evidence against the unit, whose own literal/evaluator proof is
        # untouched. Downgrading it would punish a record for an instrument's
        # blind spot, which is the same over-correction round 3 made and
        # round 4 undid. A `disagree` verdict is the case that WOULD be
        # evidence against, and it is deliberately never mapped to either
        # word -- it leaves the status alone so the unit stays outstanding.
        if status in ("literal-verified", "fixture-verified",
                      "oracle-agree", "oracle-unverifiable"):
            return DONENESS_DONE
        if status in ("ingested-magnitude", "grounded", "text-complete"):
            return DONENESS_HELD
        raise ValueError(f"doneness: unmapped {wiring_class!r} + {status!r}")
    if wiring_class == "computed":
        return DONENESS_DONE if status == "grounded" else DONENESS_IN_PROGRESS
    raise ValueError(f"doneness: unknown wiring_class {wiring_class!r}")

