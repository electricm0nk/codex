// The `actions` look (SD-40 E3.1, decisions.md §23.4: section 2, Abilities and Feats): the Actions
// tab's lines as one table under its captions (the row of kind `captions`: Name, Kind, Value): each
// line its name, its kind (the row's `kind`: Racial trait, Race) and its value as given, then its
// condition and words the width of the table. It formats strings it is given; it computes nothing
// (NR-2).

// The heading stays on the page of the block's first line (`sticky`): never alone at a page's foot.
#let heading-of(b) = block(sticky: true, below: 0.5em, {
  text(size: 12pt, weight: "bold", b.heading)
  v(-0.4em)
  line(length: 100%, stroke: 0.6pt + luma(60))
})

#let caption(t) = text(size: 8pt, weight: "bold", fill: luma(60), t)

#let line-of(r) = {
  let cells = (text(size: 9pt, weight: "bold", r.label), text(size: 8.5pt, r.at("kind", default: "")), text(size: 9pt, weight: "bold", r.values.filter(v => v != "").join(", ")))
  if r.notes.len() == 0 {
    cells
  } else {
    cells + (table.cell(colspan: 3, inset: (left: 10pt, right: 4pt, top: 0pt, bottom: 4pt),
      stack(spacing: 0.4em, ..r.notes.map(n => text(size: 8.5pt, fill: luma(40), n)))),)
  }
}

#let look(b) = {
  let captions = b.rows.filter(r => r.at("kind", default: "") == "captions")
  let rows = b.rows.filter(r => r.at("kind", default: "") != "captions")
  let head = if captions.len() > 0 { (captions.first().label, ..captions.first().values).map(caption) } else { ([], [], []) }
  // Which table rows end a line (its words row, or the line itself when it has none): a rule under each.
  let ends = (true,) + rows.map(r => if r.notes.len() == 0 { (true,) } else { (false, true) }).flatten()
  block(breakable: true, width: 100%, above: 1em, below: 0.6em)[
    #heading-of(b)
    #table(
      columns: (1fr, 2.6cm, auto),
      stroke: (x, y) => if y == 0 { (bottom: 0.6pt + luma(60)) } else if ends.at(y, default: false) { (bottom: 0.4pt + luma(190)) } else { none },
      inset: (x: 4pt, y: 3pt),
      align: (x, y) => if x == 2 { right } else { left },
      table.header(..head),
      ..rows.map(line-of).flatten(),
    )
  ]
}
