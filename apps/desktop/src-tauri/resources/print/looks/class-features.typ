// The `class-features` look (SD-40 E3.1, decisions.md §23.4: section 2, Abilities and Feats): per
// held class one line (the rows of kind `class`): its name and level in bold, then the class's own
// words (its hit die, its skill ranks per level); then the feature lines two to a line: each its
// label, its value in bold at the right, and its condition and words under it. It formats strings
// it is given; it computes nothing (NR-2).

#let heading-of(b) = {
  text(size: 12pt, weight: "bold", b.heading)
  v(-0.4em)
  line(length: 100%, stroke: 0.6pt + luma(60))
  v(-0.2em)
}

#let kind(r) = r.at("kind", default: "")

#let entry(r) = block(width: 100%, {
  grid(
    columns: (1fr, auto),
    column-gutter: 0.3cm,
    text(size: 9pt, r.label),
    text(size: 9pt, weight: "bold", r.values.filter(v => v != "").join(", ")),
  )
  if r.notes.len() > 0 {
    v(-0.5em)
    pad(left: 0.3cm, stack(spacing: 0.35em, ..r.notes.map(n => text(size: 8pt, fill: luma(40), n))))
  }
  v(-0.3em)
  line(length: 100%, stroke: 0.4pt + luma(190))
})

#let look(b) = {
  let classes = b.rows.filter(r => kind(r) == "class")
  let lines = b.rows.filter(r => kind(r) != "class")
  block(breakable: true, width: 100%, above: 1em, below: 0.6em)[
    #heading-of(b)
    #for r in classes [
      #text(size: 10pt, weight: "bold", r.label)#for v in r.values.filter(v => v != "") [#h(1.2em)#text(size: 9.5pt, v)]
      #v(0.1em)
    ]
    #grid(columns: (1fr, 1fr), column-gutter: 0.6cm, row-gutter: 0.5em, ..lines.map(entry))
    #metadata((columns: 2)) <class-features-look>
  ]
}
