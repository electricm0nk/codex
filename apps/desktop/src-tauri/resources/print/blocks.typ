// The shared blocks (SD-39 operator ruling R2): one look for every block a layout file names.
// A block is a heading and labelled rows; each row prints its label, then its values as given.

#let sheet-block(b) = block(breakable: true, width: 100%, above: 1em, below: 0.6em)[
  #text(size: 12pt, weight: "bold", b.heading)
  #v(-0.4em)
  #line(length: 100%, stroke: 0.6pt + luma(60))
  #v(-0.2em)
  #table(
    columns: (1fr,) + (auto,) * b.value_columns,
    stroke: (x, y) => (bottom: 0.4pt + luma(190)),
    inset: (x: 4pt, y: 3pt),
    align: (x, y) => if x == 0 { left } else { right },
    ..b.rows.map(r => (r.label, ..r.values.map(v => strong(v)))).flatten(),
  )
]
