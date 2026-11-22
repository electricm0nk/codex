// The `table` look (SD-40 decisions.md §4): the generic table every block printed in before
// SD-40, and the look a block with none of its own still prints in. A block is a heading and
// labelled rows; each row prints its label, then its values as given, then its notes (a rule's
// words, a condition, an override tag) the width of the block. It formats; it computes nothing.

#let sheet-row(r, columns) = {
  let line = (r.label, ..r.values.map(v => strong(v)))
  if r.notes.len() == 0 {
    line
  } else {
    line + (table.cell(colspan: columns + 1, inset: (left: 10pt, right: 4pt, y: 2pt),
      stack(spacing: 0.45em, ..r.notes.map(n => text(size: 8.5pt, fill: luma(40), n)))),)
  }
}

#let look(b) = {
  block(breakable: true, width: 100%, above: 1em, below: 0.6em)[
    #text(size: 12pt, weight: "bold", b.heading)
    #v(-0.4em)
    #line(length: 100%, stroke: 0.6pt + luma(60))
    #v(-0.2em)
    #table(
      columns: (1fr,) + (auto,) * b.value_columns,
      stroke: (x, y) => (bottom: 0.4pt + luma(190)),
      inset: (x: 4pt, y: 3pt),
      align: (x, y) => if x == 0 { left } else { right },
      ..b.rows.map(r => sheet-row(r, b.value_columns)).flatten(),
    )
  ]
}
