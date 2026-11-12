// The `skills` look (SD-40 E2.2; E2.11 one column, totals only, `decisions.md §23.1` item 3c): the
// skills in ONE column, each row on one line: the name and its signed total, nothing else (the
// breakdown prints on the Show-the-math pages). A Knowledge name shortens to `Know. (<Subject>)`
// only when the full name, measured at the row's font, is wider than its cell; every other name
// prints as the block sends it. Any other row (what has no total, what a feat states and no total
// adds) prints its label and its notes under the column. It formats; it adds nothing up (NR-2).
//
// A skill row's values, as `print/blocks/skills.rs` sends them: 0 the signed total.

#let heading-of(b) = {
  text(size: 12pt, weight: "bold", b.heading)
  v(-0.4em)
  line(length: 100%, stroke: 0.6pt + luma(60))
  v(-0.2em)
}

#let caption(t) = text(size: 8pt, fill: luma(60), t)

#let name-size = 8pt
#let total-width = 0.95cm
#let gutter = 0cm

#let is-skill(r) = r.values.len() > 0

// The name as it fits `width`: a Knowledge name too wide for it prints as `Know. (<Subject>)`.
#let fitted(name, width) = {
  let full = text(size: name-size, name)
  if name.starts-with("Knowledge (") and measure(full).width > width {
    text(size: name-size, "Know." + name.slice("Knowledge".len()))
  } else {
    full
  }
}

// One ruled line per skill: the rule under the row carries the eye from the name to its total.
#let column-of(rows) = layout(size => grid(
  columns: (1fr, total-width),
  column-gutter: gutter,
  rows: (0.32cm,) * rows.len() + (auto,),
  align: (x, y) => if x == 0 { left + horizon } else { right + horizon },
  stroke: (x, y) => if y < rows.len() { (bottom: 0.3pt + luma(170)) },
  ..rows.map(r => (
    fitted(r.label, size.width - total-width - gutter),
    text(size: name-size, weight: "bold", r.values.at(0)),
  )).flatten(),
  [], caption[Total],
))

// The skills print whole beside the Weapons, then the lines that are not a panel total.
#let look(b) = {
  let skills = b.rows.filter(is-skill)
  let words = b.rows.filter(r => not is-skill(r))
  block(breakable: false, width: 100%, above: 1em, below: 0.6em)[
    #heading-of(b)
    #column-of(skills)
    #for r in words {
      v(0.2em)
      text(size: 8.5pt, weight: "bold", r.label)
      if r.notes.len() > 0 {
        v(-0.3em)
        stack(spacing: 0.45em, ..r.notes.map(n => text(size: 8.5pt, fill: luma(40), n)))
      }
    }
  ]
}
