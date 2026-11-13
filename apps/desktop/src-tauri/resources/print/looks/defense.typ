// The `defense` look (SD-40 E3.2, decisions.md §23.1 item 5: section 3, Inventory): the armor and
// shield table under its captions (the row of kind `captions`: AC, Max Dex, Check, Failure, Weight),
// a line per worn piece as given, one blank ruled row for a piece the player writes in, then the worn
// totals (rows of kind `total`) as one line of label and value. It formats strings it is given; it
// computes nothing (NR-2).

#let blank = 1

// The heading stays on the page of the block's first line (`sticky`): never alone at a page's foot.
#let heading-of(b) = block(sticky: true, below: 0.5em, {
  text(size: 12pt, weight: "bold", b.heading)
  v(-0.4em)
  line(length: 100%, stroke: 0.6pt + luma(60))
})

#let caption(t) = text(size: 8pt, weight: "bold", fill: luma(60), t)

#let kind(r) = r.at("kind", default: "")

#let look(b) = {
  let captions = b.rows.filter(r => kind(r) == "captions")
  let totals = b.rows.filter(r => kind(r) == "total")
  let lines = b.rows.filter(r => kind(r) != "captions" and kind(r) != "total")
  let width = b.value_columns
  let head = if captions.len() > 0 { (captions.first().label, ..captions.first().values).map(caption) } else { ([],) * (width + 1) }
  block(breakable: true, width: 100%, above: 1em, below: 0.6em)[
    #heading-of(b)
    #table(
      columns: (1fr,) + (auto,) * width,
      stroke: (x, y) => if y == 0 { (bottom: 0.6pt + luma(60)) } else { (bottom: 0.4pt + luma(190)) },
      inset: (x: 4pt, y: 3pt),
      align: (x, y) => if x == 0 { left } else { right },
      table.header(..head),
      ..lines.map(r => (text(size: 9pt, weight: "bold", r.label), ..r.values.map(v => text(size: 9pt, v)))).flatten(),
      // The write-in row: as tall as a pencilled line (SD-m), every cell empty.
      ..range(blank).map(_ => (table.cell(colspan: width + 1, box(height: 0.5cm)),)).flatten(),
    )
    #if totals.len() > 0 {
      v(0.2em)
      text(size: 8.5pt, totals.map(r => [#r.label: *#r.values.at(0, default: "")*]).join([ #h(0.8em) · #h(0.8em) ]))
    }
    #metadata((blank: blank)) <defense-look>
  ]
}
