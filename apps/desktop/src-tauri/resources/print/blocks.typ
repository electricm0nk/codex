// The look dispatcher (SD-40 decisions.md §4, operator ruling R1): each block prints in the look
// it names, one Typst template per look under /looks/ (`typst.rs`'s `LOOKS`). `table` is the
// generic table every block printed in before SD-40. A block marked `new_page` starts a page of
// its own.

#import "/looks/table.typ": look as table-look
#import "/looks/abilities.typ": look as abilities-look
#import "/looks/square-block.typ": look as square-block-look, items as square-block-items
#import "/looks/armor-class.typ": look as armor-class-look
#import "/looks/attack.typ": look as attack-look
#import "/looks/saves.typ": look as saves-look, items as saves-items
#import "/looks/speed.typ": look as speed-look
#import "/looks/skills.typ": look as skills-look, items as skills-items
#import "/looks/weapons.typ": look as weapons-look, items as weapons-items
#import "/looks/portrait.typ": look as portrait-look
#import "/looks/name-campaign.typ": look as name-campaign-look
#import "/looks/languages.typ": look as languages-look

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
)

// The looks whose list page 1's overflow rule may shrink and split (E2.4, decisions.md §23.2
// SD-b): each look's `items(b)`, the lines it prints that can continue on page 2.
#let lists = (
  "square-block": square-block-items,
  saves: saves-items,
  skills: skills-items,
  weapons: weapons-items,
)

// One block in its own look, without the page break: what a grid cell holds.
#let block-look(b) = looks.at(b.look)(b)

#let sheet-block(b) = {
  if b.new_page { pagebreak(weak: true) }
  block-look(b)
}
