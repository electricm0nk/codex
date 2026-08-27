"""SD-33 remediation wave 5 (`sd33-r5-skillcombat`): the real, root-caused
fix for the `SEVERE Globals:130 Could not find campaign: <name>` failure
class every prior `ultimate_psionics` lane recorded as
`oracle_harness_ultimate_psionics_campaign_load_failure` and left as an
unresolved blocker (`AT-33-E5-remainder-equipment`, `AT-33-E5-last75`,
`AT-33-E5-shape-combat`).

ROOT CAUSE (confirmed live against the pinned oracle, `PCGEN_ORACLE_SHA`
per `scripts/pcgen-oracle-pin.env`), read directly from PCGen's own
`Globals.getCampaignKeyed` (`code/src/java/pcgen/core/Globals.java`):
campaign lookup matches a `.pcg`'s `CAMPAIGN:<name>` line against each
loaded `Campaign`'s own `getKeyName()` -- which is the campaign's `KEY:`
token when its `.pcc` carries one, NOT its `CAMPAIGN:` display name. Every
prior lane's fixture used the DISPLAY name (`CAMPAIGN:Ultimate Psionics`,
matching `ultimate_psionics.pcc`'s own `CAMPAIGN:Ultimate Psionics` line)
-- but that same `.pcc` ALSO carries `KEY:DSP - Ultimate Psionics`, a
SEPARATE internal key other campaigns' own `PRECAMPAIGN:...,INCLUDES=DSP -
Ultimate Psionics` clauses reference. A `.pcg` naming the display name
therefore never resolves: `Globals.getCampaignKeyed("Ultimate Psionics")`
returns `null` (case-insensitive match against `KEY:DSP - Ultimate
Psionics` fails), the SEVERE line prints, and the book silently drops out
of `Loading sources [...]` -- the equipped item never actually equips
("Could not add equipment: <item>. Check loaded campaigns."), so a
downstream AC/SKILL/COMBAT export reflects a bare character, not the
unit's real effect.

Confirmed live (2026-08-25, this wave): a `.pcg` with
`CAMPAIGN:DSP - Ultimate Psionics` (the real KEY, not the display name)
loads with `Loading sources [Core Rulebook, Ultimate Psionics]` -- no
SEVERE line, no equip warning -- and a real SKILL item
(`ultimate_psionics:equipment:crystal_mask_psionic_craft`,
`BONUS:SKILL|Spellcraft|10|TYPE=Competence`) exports `SKILL.MISC=10`,
exactly matching the corpus, where the SAME fixture with the display-name
`CAMPAIGN:` line exported `SKILL.MISC=0` under an identical direct-`java`
`charbuild_remainder_run_one.sh` invocation.

Not every campaign has this KEY/CAMPAIGN divergence -- Core Rulebook's own
`KEY:Core Rulebook` equals its `CAMPAIGN:Core Rulebook`, so it was never
visibly broken. This module is the general fix: use
`campaign_line_value(display_name)` wherever a `.pcg` fixture emits a
`CAMPAIGN:<name>` line, instead of writing the display name literally.
`CAMPAIGN_KEY_OVERRIDES` is deliberately small and grows only when a real,
confirmed-live divergence is found -- never guessed from a `.pcc`'s `KEY:`
token alone (`Core Rulebook`'s own `KEY:` token is present too, and it
equals its display name; the divergence has to be checked per book).

Usage:
    from campaign_key import campaign_line_value
    pcg_text = f"CAMPAIGN:{campaign_line_value(\"Ultimate Psionics\")}\\n"
"""

from __future__ import annotations

# display CAMPAIGN name (as used by every existing CAMPAIGN_CLOSURE table,
# e.g. `combat-shape-work/ac_generate.py`) -> the campaign's real `KEY:`
# token, when the two diverge. Confirmed by reading the pinned oracle's own
# `.pcc` file directly (`grep -n "^KEY:\|^CAMPAIGN:" <book>.pcc`), never
# guessed.
CAMPAIGN_KEY_OVERRIDES: dict[str, str] = {
    # data/pathfinder/dreamscarred_press/ultimate_psionics/ultimate_psionics.pcc:
    #   CAMPAIGN:Ultimate Psionics
    #   KEY:DSP - Ultimate Psionics
    "Ultimate Psionics": "DSP - Ultimate Psionics",
}


def campaign_line_value(display_name: str) -> str:
    """The real value a `.pcg`'s `CAMPAIGN:` line must carry for
    `display_name` to actually load -- the override's real `KEY:` token
    when one is known to diverge, else `display_name` itself unchanged."""
    return CAMPAIGN_KEY_OVERRIDES.get(display_name, display_name)
