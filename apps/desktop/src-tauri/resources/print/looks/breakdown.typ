// The `breakdown` look (SD-40 E3.5, decisions.md §23.1 item 4): one of the Math section's breakdown
// tables, the cells page 1 leaves out (Initiative, Saving Throws, Skills, Weapons). The heading, then
// a table whose header is the row of kind `captions` (label empty, a caption per value column), then
// a line per figure: its label, its cells as given, and its notes (a feat's damage) under it. It
// formats strings it is given; it computes nothing (NR-2).

#let caption(c) = text(size: 7.5pt, weight: "bold", fill: luma(60), c)

#let figure-row(r, columns) = {
  let line = (text(size: 9pt, r.label), ..r.values.map(v => text(size: 9pt, weight: "bold", v)))
  if r.notes.len() == 0 {
    line
  } else {
    line + (table.cell(colspan: columns + 1, inset: (left: 10pt, right: 4pt, y: 1.5pt),
      stack(spacing: 0.4em, ..r.notes.map(n => text(size: 8pt, fill: luma(40), n)))),)
  }
}

#let look(b) = {
  let captions = b.rows.filter(r => r.at("kind", default: "") == "captions")
  let figures = b.rows.filter(r => r.at("kind", default: "") != "captions")
  block(breakable: true, width: 100%, above: 1em, below: 0.6em)[
    #block(sticky: true, below: 0.3em, {
      text(size: 11pt, weight: "bold", b.heading)
      v(-0.4em)
      line(length: 100%, stroke: 0.6pt + luma(60))
    })
    #table(
      columns: (1fr,) + (auto,) * b.value_columns,
      stroke: (x, y) => (bottom: 0.4pt + luma(190)),
      inset: (x: 4pt, y: 2.5pt),
      align: (x, y) => if x == 0 { left } else { right },
      ..captions.map(r => table.header(caption(r.label), ..r.values.map(caption))),
      ..figures.map(r => figure-row(r, b.value_columns)).flatten(),
    )
  ]
}
