// The look dispatcher (SD-40 decisions.md §4, operator ruling R1): each block prints in the look
// it names, one Typst template per look under /looks/ (`typst.rs`'s `LOOKS`). `table` is the
// generic table every block printed in before SD-40. A block marked `new_page` starts a page of
// its own.

#import "/looks/table.typ": look as table-look
#import "/looks/abilities.typ": look as abilities-look
#import "/looks/hit-points.typ": look as hit-points-look
#import "/looks/initiative.typ": look as initiative-look
#import "/looks/armor-class.typ": look as armor-class-look
#import "/looks/attack.typ": look as attack-look
#import "/looks/saves.typ": look as saves-look
#import "/looks/speed.typ": look as speed-look
#import "/looks/skills.typ": look as skills-look

#let looks = (
  table: table-look,
  abilities: abilities-look,
  "hit-points": hit-points-look,
  initiative: initiative-look,
  "armor-class": armor-class-look,
  attack: attack-look,
  saves: saves-look,
  speed: speed-look,
  skills: skills-look,
)

// One block in its own look, without the page break: what a grid cell holds.
#let block-look(b) = looks.at(b.look)(b)

#let sheet-block(b) = {
  if b.new_page { pagebreak(weak: true) }
  block-look(b)
}
