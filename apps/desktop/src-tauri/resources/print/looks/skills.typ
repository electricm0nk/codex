// The `skills` look (SD-40 E2.2): the skills in two narrow columns, the first half of the rows
// down the left and the rest down the right. Each row is the class-skill box, the name with its
// armor-check-penalty mark, the total, the key ability and its modifier, the ranks and the misc
// write-in box the player fills. Any other row (what has no total, what a feat states and no
// total adds) prints its label, its values and its notes under the columns. It formats; it adds nothing up (NR-2).
//
// A skill row's values, as `print/blocks/skills.rs` sends them: 0 class-skill mark, 1 total,
// 2 ability, 3 ranks, 4 misc (a write-in), 5 armor-check mark.

#let heading-of(b) = {
  text(size: 12pt, weight: "bold", b.heading)
  v(-0.4em)
  line(length: 100%, stroke: 0.6pt + luma(60))
  v(-0.2em)
}

#let caption(t) = text(size: 8pt, fill: luma(60), t)

// A value the engine filled, or (an empty string) a ruled write-in box the player fills.
#let value-box(v, weight: "regular") = box(
  width: 100%, height: 0.42cm, stroke: 0.5pt + luma(60),
  align(center + horizon, text(size: 8pt, weight: weight, v)),
)

#let mark-box(v) = box(
  width: 0.3cm, height: 0.3cm, stroke: 0.5pt + luma(60),
  align(center + horizon, text(size: 8pt, weight: "bold", v)),
)

#let is-skill(r) = r.values.at(1, default: "") != ""

#let column-of(rows) = grid(
  columns: (0.32cm, 1fr, 0.55cm, 0.95cm, 0.46cm, 0.5cm),
  column-gutter: 0.03cm,
  row-gutter: 0.08cm,
  align: (x, y) => if x == 1 { left + horizon } else { center + horizon },
  ..rows.map(r => (
    mark-box(r.values.at(0)),
    { set par(leading: 0.25em); text(size: 8pt, r.label + r.values.at(5)) },
    value-box(r.values.at(1), weight: "bold"),
    // The key ability and its modifier on one line.
    text(size: 8pt, r.values.at(2).replace(" ", "\u{a0}")),
    value-box(r.values.at(3)),
    value-box(r.values.at(4)),
  )).flatten(),
  // The captions under the columns, as the other number-box looks print them: the block's text
  // reads heading, then its first skill.
  [], caption[Skill], caption[Tot.], caption[Abil.], caption[Rk.], caption[Misc],
)

// The skills print whole, one block beside the Weapons: two columns split across a page break
// would read out of order.
#let look(b) = {
  let skills = b.rows.filter(is-skill)
  let words = b.rows.filter(r => not is-skill(r))
  let half = int((skills.len() + 1) / 2)
  block(breakable: false, width: 100%, above: 1em, below: 0.6em)[
    #heading-of(b)
    #grid(
      columns: (1fr, 1fr),
      column-gutter: 0.2cm,
      column-of(skills.slice(0, half)),
      column-of(skills.slice(half)),
    )
    #v(0.1em)
    #caption[Tot. total = Abil. key ability modifier + Rk. ranks + Misc (write in) · X class skill (+3 once it has a rank) · \* armor check penalty applies]
    #for r in words {
      v(0.2em)
      text(size: 8.5pt, weight: "bold", r.label)
      for v in r.values.filter(v => v != "") { h(0.4em); text(size: 8.5pt, v) }
      if r.notes.len() > 0 {
        v(-0.3em)
        stack(spacing: 0.45em, ..r.notes.map(n => text(size: 8.5pt, fill: luma(40), n)))
      }
    }
  ]
}
