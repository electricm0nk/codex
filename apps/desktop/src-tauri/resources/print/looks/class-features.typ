// The `class-features` look (SD-40 E3.1, decisions.md §23.4: section 2, Abilities and Feats): per
// held class one line (the rows of kind `class`): its name and level in bold, then the class's own
// words (its hit die, its skill ranks per level); then the feature lines two to a line: each its
// label, its value in bold at the right, and its condition and words under it. It formats strings
// it is given; it computes nothing (NR-2).

// The heading stays on the page of the block's first line (`sticky`): never alone at a page's foot.
#let heading-of(b) = block(sticky: true, below: 0.5em, {
  text(size: 12pt, weight: "bold", b.heading)
  v(-0.4em)
  line(length: 100%, stroke: 0.6pt + luma(60))
})

#let kind(r) = r.at("kind", default: "")

// A line stays whole on one page, so its words never break around the cell beside it.
#let entry(r) = block(breakable: false, width: 100%, {
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
    #for r in classes {
      block(sticky: true, below: 0.6em)[#text(size: 10pt, weight: "bold", r.label)#for v in r.values.filter(v => v != "") [#h(1.2em)#text(size: 9.5pt, v)]]
    }
    #grid(columns: (1fr, 1fr), column-gutter: 0.6cm, row-gutter: 0.5em, ..lines.map(entry))
    #metadata((columns: 2)) <class-features-look>
  ]
}
