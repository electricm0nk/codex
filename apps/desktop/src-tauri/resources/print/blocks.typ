// The look dispatcher (SD-40 decisions.md §4, operator ruling R1): each block prints in the look
// it names, one Typst template per look under /looks/ (`typst.rs`'s `LOOKS`). `table` is the
// generic table every block printed in before SD-40. A block marked `new_page` starts a page of
// its own.

#import "/looks/table.typ": look as table-look
#import "/looks/abilities.typ": look as abilities-look
#import "/looks/square-block.typ": look as square-block-look
#import "/looks/armor-class.typ": look as armor-class-look
#import "/looks/attack.typ": look as attack-look
#import "/looks/saves.typ": look as saves-look
#import "/looks/speed.typ": look as speed-look
#import "/looks/skills.typ": look as skills-look, items as skills-items
#import "/looks/weapons.typ": look as weapons-look, items as weapons-items
#import "/looks/portrait.typ": look as portrait-look
#import "/looks/name-campaign.typ": look as name-campaign-look
#import "/looks/languages.typ": look as languages-look
#import "/looks/conditional-bonuses.typ": look as conditional-bonuses-look, fit as conditional-bonuses-fit
#import "/looks/resistances.typ": look as resistances-look
#import "/looks/section-title.typ": look as section-title-look
#import "/looks/feats.typ": look as feats-look
#import "/looks/class-features.typ": look as class-features-look
#import "/looks/actions.typ": look as actions-look
#import "/looks/defense.typ": look as defense-look
#import "/looks/gear.typ": look as gear-look
#import "/looks/money.typ": look as money-look
#import "/looks/spells.typ": look as spells-look
#import "/looks/spellbook.typ": look as spellbook-look
#import "/looks/custom.typ": look as custom-look
#import "/looks/pets.typ": look as pets-look
#import "/looks/bio.typ": look as bio-look
#import "/looks/breakdown.typ": look as breakdown-look
#import "/looks/math.typ": look as math-look
#import "/looks/progression.typ": look as progression-look

#let looks = (
  table: table-look,
  abilities: abilities-look,
  "square-block": square-block-look,
  "armor-class": armor-class-look,
  attack: attack-look,
  saves: saves-look,
  speed: speed-look,
  skills: skills-look,
  weapons: weapons-look,
  portrait: portrait-look,
  "name-campaign": name-campaign-look,
  languages: languages-look,
  "conditional-bonuses": conditional-bonuses-look,
  resistances: resistances-look,
  "section-title": section-title-look,
  feats: feats-look,
  "class-features": class-features-look,
  actions: actions-look,
  defense: defense-look,
  gear: gear-look,
  money: money-look,
  spells: spells-look,
  spellbook: spellbook-look,
  custom: custom-look,
  pets: pets-look,
  bio: bio-look,
  breakdown: breakdown-look,
  math: math-look,
  progression: progression-look,
)

// The looks whose list page 1's overflow rule may shrink and split (E2.4, decisions.md §23.2
// SD-b): each look's `items(b)`, the lines it prints that can continue on page 2.
#let lists = (
  skills: skills-items,
  weapons: weapons-items,
)

// The looks of fixed height (E2.20): each look's `fit(b, width)` keeps what fills its box and hands
// the rest (`rest`) to page 2, whatever room page 1 has.
#let boxes = (
  "conditional-bonuses": conditional-bonuses-fit,
)

// One block in its own look, without the page break: what a grid cell holds.
#let block-look(b) = looks.at(b.look)(b)

#let sheet-block(b) = {
  if b.new_page { pagebreak(weak: true) }
  block-look(b)
}
