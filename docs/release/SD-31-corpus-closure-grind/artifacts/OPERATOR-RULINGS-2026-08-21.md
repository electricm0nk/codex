# Operator rulings — 2026-08-21 (UTC)

**Date note, because a lane correctly challenged it:** this file is dated in UTC. A session running
in US local time will see the previous calendar day and may read the date as being in the future.
That is a timezone artifact, not a forged date.

**Provenance note:** wave 25's interpreter-core lane REFUSED to build against §20 because this file
had been written but never committed — it existed only in the orchestrator's working tree, so a lane
in a fresh worktree found nothing for it across every ref and declined to reverse a pinned safety
ruling on the authority of a dispatch prompt alone. That refusal was correct and is the behaviour
this program asks for. The orchestrator's failure was dispatching lanes to read an uncommitted file.
A ruling is not in force until it is committed to the branch.

Recorded by the orchestrator. Fold into `decisions.md` at the next integration cycle.

---

## Ruling §18 — option pools must show ONLY VALID CHOICES

**Operator, verbatim:** *"we need to show only valid choices."*

Answers the exclusive-choice-pool question the Bestiary 6 ledger raised.

**Consequence for Domain Powers.** A Cleric/Inquisitor domain is an EXCLUSIVE, once-per-character
choice: once a character takes the Void domain, Scalykind's powers are not available to them. So
the "browsable reference regardless of selection" pattern is NOT honest here, and the three units
the ledger flagged as bankable that way (Intense Celebration, It Came From Beyond, The Stars Are
Right) may **not** be banked through `REGISTERED_POOL_GROUPS`. They need the same real
domain-power grounding fix as the other 15.

**Consequence for the pools already shipped.** `REGISTERED_POOL_GROUPS` currently registers exactly
two: Rogue Talent and Rage Power. Both are genuinely open, repeatable-pick pools — any rogue can
eventually take any rogue talent — so showing the full list to a character of that class is showing
valid choices, and wave 23's +109 stands. **But this ruling makes prerequisites load-bearing**: a
pool option whose prerequisites the character cannot meet is not a valid choice for them, and the
next pool-touching cycle must confirm the shipped catalog honours that rather than listing a pool
wholesale. That check has not been done.

**The general rule this establishes:** before registering any new pool, establish whether it is OPEN
(repeatable pick from a standing list) or EXCLUSIVE (a once-per-character branch). Open pools may use
the reference-catalog pattern. Exclusive pools may not, and need real per-selection grounding.

---

## Ruling §19 — cross-book verbatim reprints: settled doctrine, close it

**Operator, verbatim:** *"this is settled doctrine. flag it, mark as complete, move on."*

Bestiary 6's 2 spells are byte-identical reprints of spells Ultimate Wilderness already ships. The
ledger raised this as needing a cross-book-reprint crediting design. It does not: `decisions.md §10`
(newest printing wins; the older is superseded and out of scope) already settles it.

Flag the reprints as superseded, mark the units complete, and stop treating this as an open
architectural question. Do not build a cross-book-reprint crediting mechanism.

---

## Ruling §20 — RAISED BY THE OPERATOR, NOT YET RULED: why hand-model what PCGen already encodes?

**Operator, verbatim:** *"can you explain to me why you are unable to green the required logic from
pcgen - the already did the heavy lifting. it feels like you are trying to reinvent the wheel."*

**The answer is that a prior operator ruling forbids it.** `SD-27 decisions.md §24.1` (2026-07-31,
operator-pinned): *"No formula interpreter. Each feature is a hand-written, corpus-verified pure
function."* Its stated rationale:

> an interpreter is the highest-risk option for *silently* wrong answers, and this codebase's own
> history is a list of wrong numbers that survived because nothing failed loudly. A hand-modelled
> formula that is wrong is a failing test; a misinterpreted token is a plausible number nobody
> checks. The cost — linear growth with content — is accepted deliberately in exchange for that
> property.

That ruling is why `ground_or_block_cleric_domain_power` is an allowlist of two domains rather than
an evaluator, and why every class feature is a bespoke function. It is the direct cause of the
program's throughput.

**What has changed since it was made, and why it is worth revisiting:**

1. **The cost is now measured, not estimated.** §24.1 accepted "linear growth with content" when the
   scale was PU's 62 features. The same shape totals roughly 17,000 lines across the deferred books,
   and observed throughput is tens of units per wave against 25,000 remaining.
2. **The checking machinery §24.1 assumed did not exist now does.** Its core fear was that "a
   misinterpreted token is a plausible number nobody checks." Since wave 13 this repo has
   `derived_evaluator_fixture_check`: a seam that pins a computed value against an expected value
   **transcribed from bytes the evaluator never reads**, and which is mutation-proven able to fail.
   That is precisely a mechanism for checking interpreted output. An interpreter gated behind it is
   not the unchecked interpreter §24.1 refused.
3. **Interpretation is already happening at the edges.** `SD-28 decisions.md` draws a
   transcription-versus-interpretation line and records `Halfling ~ Adaptable Luck` as an unresolved
   case sitting exactly on it. The boundary is being litigated case by case rather than decided.

**The trade this ruling is really about:** hand-modelling buys loud failure at linear cost.
A fixture-gated interpreter would buy constant cost, and moves the risk from "silently wrong number"
to "silently wrong *fixture*" — which is a smaller surface, but not zero.

## RULED, 2026-08-21: §24.1 IS OVERTURNED. Build the interpreter.

**Operator, verbatim:** *"I choose thousands. if we need to revisit this, we can do it in the
future. for now we need to get something in front of the user community."*

The operator also noted they did not recall making the §24.1 ruling and could see how it had slipped
by unnoticed — so this is a correction of a decision that was never really examined, not a reversal
of a considered one.

**What is now authorised:** read PCGen's own formula tokens and evaluate them, instead of
hand-writing a pure function per feature. §24.1's "no formula interpreter" is superseded for this
package.

**What is NOT relaxed, and is the condition that makes this safe:** every interpreted value must
clear `derived_evaluator_fixture_check`, whose expected value is transcribed from bytes the
evaluator never reads, and which is mutation-proven able to fail. §24.1's real concern — a
misinterpreted token becoming a plausible number nobody checks — is answered by that gate, not by
abandoning the concern. An interpreted value with no fixture is not done.

**The correctness proof available, and it is strong:** roughly 27 classes' worth of hand-modelled
functions already exist (`warpriest_fervor_uses_per_day`, `slayer_sneak_attack_dice`,
`monk_scorpion_style_dc` and siblings), each verified byte-exact against the corpus and pinned by
tests. The interpreter must REPRODUCE ALL OF THEM. Any disagreement means either the interpreter is
wrong or a hand function is wrong — both are findings worth having. This is far wider than the
4-class proof that let wave 21's parser ship a 73.4% fabrication rate.

**Standing context:** the operator's goal is to get something in front of the user community. Speed
matters now in a way it did not before — but the anti-gaming doctrine is unchanged, and a fast wrong
number is worth less than a slow right one.
