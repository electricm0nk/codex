// The `custom` look (SD-40 E3.3, decisions.md §23.4: section 2): the Custom tab's grants, records,
// conditions and override flags as one compact table, a line per row (its label and its values as
// given), its notes under it in small type. It formats strings it is given; it computes nothing
// (NR-2).

// The heading stays on the page of the block's first line (`sticky`): never alone at a page's foot.
#let heading-of(b) = block(sticky: true, below: 0.5em, {
  text(size: 12pt, weight: "bold", b.heading)
  v(-0.4em)
  line(length: 100%, stroke: 0.6pt + luma(60))
})

#let look(b) = {
  let width = b.value_columns + 1
  block(breakable: true, width: 100%, above: 1em, below: 0.6em)[
    #heading-of(b)
    #table(
      columns: (1fr,) + (auto,) * b.value_columns,
      stroke: (x, y) => (bottom: 0.4pt + luma(190)),
      inset: (x: 4pt, y: 2.5pt),
      ..b.rows.map(r => {
        let line = (text(size: 8.5pt, weight: "bold", r.label), ..r.values.map(v => text(size: 8.5pt, v)))
        if r.notes.len() == 0 { line } else {
          line + (table.cell(colspan: width, inset: (left: 10pt, right: 4pt, top: 0pt, bottom: 2.5pt), stack(spacing: 0.35em, ..r.notes.map(n => text(size: 8pt, fill: luma(40), n)))),)
        }
      }).flatten(),
    )
  ]
}
