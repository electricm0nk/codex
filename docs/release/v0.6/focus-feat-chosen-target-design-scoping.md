# Focus-Feat Chosen-Target Mechanism — Design Scoping (task #16, candidate 2)

> Task #16's candidate (1) — the 8 two-skill General feats + Persuasive's
> Diplomacy half — is **built and reported** (`standalone_skill_facts_from_feats`
> in `feat_effects.rs`, 29/29 module tests, 540/540 lib). This doc scopes the
> remaining candidate (2): the chosen-target mechanism for the Focus feats
> (Skill Focus, Weapon Focus, Spell Focus / Greater Spell Focus), which the lead
> flagged as "the single biggest locked door in feat-effects… a real design
> conversation, not a quick widening." Per standing discipline: **scoping only,
> no code** — and the load-bearing pieces cross my lane, so this is a proposal
> for the lead/operator to decide, not something to default into building.

## The one fact that reframes this whole question

My original scoping doc deferred all four Focus feats for the same reason:
their effect targets a **player-chosen** skill/weapon/school (corpus `%LIST`),
which `selected_feats` has no slot to record. That framing carried a second,
now-obsolete assumption: that even if the target *were* recorded, the bonus
would have "no live consumer" (skill/weapon/spell-DC totals aren't computed).

**The methodology correction (scout + lead, 31+ shipped precedents) kills that
second assumption.** A Focus bonus grounds fine as a *standalone explanation
record* — `+3 to <chosen skill>`, `+1 attack with <chosen weapon>`, `+1 save DC
for <chosen school> spells` — exactly like Track / Bardic Knowledge / the 17
two-skill facts I just landed, whether or not that dimension is integrated into
a running total. So **the consumer problem is gone. The ONLY remaining blocker
is recording the chosen target.** That is a bounded, answerable design
question, not the open-ended one the original "biggest locked door" framing
implied.

## Verified magnitudes (against `feat_data/`, real CRB text)

- **Skill Focus** (`general.rs`, `["SKILL","%LIST","3","TYPE=SkillFocus"]`):
  +3 to one chosen skill ("+3 bonus on checks with that skill; +6 at 10+
  ranks"). Provable +3 at every level the engine represents (ranks < 10) — same
  floor reasoning as the two-skill feats. **Cleanest first target.**
- **Weapon Focus** (`combat.rs`, `["WEAPONPROF=%LIST","TOHIT","WeaponFocusToHit"]`):
  +1 attack with one chosen weapon. (The fixed loadout already hardcodes the
  Longsword instance as `WEAPON_FOCUS_LONGSWORD_SELECTION` — a catalog-picked
  Weapon Focus on any other weapon is the gap.)
- **Spell Focus** (`general.rs`, `["DC","SCHOOL.%LIST","1","TYPE=SpellFocus"]`):
  +1 to save DCs of one chosen school. **Greater Spell Focus** (`"2"`): +2,
  stacks.

## Recording the chosen target — two real mechanisms

### Mechanism A — compound key in `selected_feats`

Mirror the existing hardcoded `feat:weapon_focus:weapon:longsword` shape: a
catalog-picked Skill Focus arrives as `"Skill Focus:skill:Stealth"` (key +
target), and `feat_effects.rs` splits the target off. Handles multiplicity
naturally (two Skill Focus picks = two distinct compound strings).

**Real cost — a cross-cutting ripple, not local.** `selected_feats: Vec<String>`
is read by more than the effects engine: `feat_prereqs::evaluate_feat_prerequisites`
matches on the exact catalog `key` (a compound `"Skill Focus:skill:Stealth"`
would **not** match a `"Skill Focus"` prerequisite lookup — silently breaking
prereq evaluation for any feat that requires Skill Focus), `feat_catalog`
dedup/display logic, and the frontend feat list (`CharacterSheet.tsx:1058`
renders `featId` verbatim — it would show the raw compound string to the user).
Every one of those readers would need a strip-the-suffix pass. That is exactly
the kind of shared-contract overload this repo's discipline exists to avoid.

### Mechanism B — a parallel `SelectedChoice` (recommended)

Record the target as a `SelectedChoice { choice_set_id: "choice:skill_focus_target",
selection_id: "skill:Stealth" }` on `chosen.selected_choices` — the **already-
shipped, proven** choice mechanism (`character_input.rs:148`; the same field
Wizard's school specialization and `LevelUpCharacterRequest.additional_choices`
use). `selected_feats` stays clean (`"Skill Focus"` unchanged), so prereqs,
catalog, and display are untouched. `feat_effects.rs` reads both: it grounds a
fact only when the feat is in `selected_feats` **and** a matching target choice
is present (an orphan choice with no feat, or a feat with no chosen target,
grounds nothing — honest, not fabricated).

**Multiplicity is a non-issue for standalone facts:** two Skill Focus picks
produce two target choices → two independent "+3 to <skill>" facts; the fact
stands alone, so pairing a target to a *specific* feat instance is cosmetic, not
load-bearing. (If a future integrated-total consumer needs strict instance
pairing, that's a later concern, not this slice's.)

**Recommendation: Mechanism B.** It reuses shipped infrastructure, ripples
nothing, and matches the choice-picker doc's own conclusion that
`selected_choices` is the right home for player picks.

## What I can own vs. what crosses my lane

**My lane (`feat_effects.rs`), buildable now once the mechanism is chosen:** a
producer that reads `(selected_feats, selected_choices)` and returns the
standalone Focus facts — the same shape as the two producers already landed.
For **Skill Focus** this reuses the existing `StandaloneSkillFeatFact` shape
almost verbatim (a `+3` skill fact whose `skill_name` comes from the chosen
target instead of a fixed table). Fully unit-testable in isolation with
synthetic `selected_choices`, RED-then-GREEN, no cross-lane dependency to
*test*.

**Crosses my lane (backend / frontend / adapter), needs the lead's Path call —
same Path A / Path B split as `choice-picker-ui-gap-scoping.md`:**
- **Path A (cheap, canonical default):** no picker; the adapter silently applies
  one canonical target (e.g. every Skill Focus → a fixed skill), mirroring
  Wizard's silent school default. *Honesty caveat, stated plainly:* a silent
  canonical target is a weaker fit here than Wizard's school — Skill Focus's
  entire mechanical identity **is** the player's choice, so defaulting it is
  more visibly a false claim. If taken, it must be named in the support-level
  copy, not shipped quietly.
- **Path B (real picker):** a target sub-choice UI when a `%LIST` feat is picked
  (`CharacterSheet.tsx` feat picker → a follow-on target dropdown), the choice
  flows through the existing `selected_choices` wire path. Genuinely new
  frontend surface; the "most targets are honestly ungrounded" UX problem is
  *smaller* here than the choice-picker classes (under the standalone-fact bar,
  **every** skill/weapon/school target grounds a real fact — there's no
  "unsupported option" list to gray out), which is a point in Path B's favor.

## Recommended slice sequence

1. **Skill Focus, Mechanism B, Path A first** — one canonical target, adapter
   applies it (backend), `feat_effects.rs` grounds the `+3` standalone fact
   (mine), surfaced as an explanation record (backend). Proves the
   target-recording pattern end-to-end the same way Toughness proved the whole
   engine — smallest real slice.
2. **Weapon Focus / Spell Focus** — same mechanism, but each needs its own fact
   *dimension* (attack-bonus fact, spell-DC fact) beyond the skill-fact shape;
   fold in once the skill pilot lands.
3. **Path B (real picker)** — separate, larger, product-decision work, exactly
   the "separate, larger, out-of-scope future work" framing the Wizard-school
   code comment and the choice-picker doc already use.

## Open questions for the lead

1. **Mechanism A vs B** — I recommend B (parallel `SelectedChoice`, zero ripple
   on `selected_feats` readers). Concur, or is there a reason to prefer the
   compound-key shape (A) despite the prereq/catalog/display ripple?
2. **Path A vs B for the first slice** — canonical silent default (fast, but the
   honesty caveat above is sharper than Wizard's), or hold Focus feats until a
   real target picker (Path B) is greenlit? This is the same product call the
   choice-picker doc surfaced, now for feats.
3. **Coordination** — the buildable-now piece is mine (`feat_effects.rs`
   producer + tests), but it's inert until backend adds the adapter target-append
   and the explanation-record surfacing. Do you want me to build the
   `feat_effects.rs` producer now (tested against synthetic `selected_choices`,
   handed off like the last two slices), or hold until the mechanism/Path is
   decided so I don't build to a contract you might change?
