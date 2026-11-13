// The `gear` look (SD-40 E3.2, decisions.md §23.1 item 5: section 3, Inventory): the owned copies
// under their captions (the row of kind `captions`: Item, Slot, Weight), in three sections (rows of
// kind `section`: Worn, Carried, Left behind), each its item lines as given and then two blank ruled
// rows for the pencil; then the carried weight, the load and the three limits (rows of kind `total`)
// as a strip of labelled boxes. It formats strings it is given; it computes nothing (NR-2).

#let blank = 2

// The heading stays on the page of the block's first line (`sticky`): never alone at a page's foot.
#let heading-of(b) = block(sticky: true, below: 0.5em, {
  text(size: 12pt, weight: "bold", b.heading)
  v(-0.4em)
  line(length: 100%, stroke: 0.6pt + luma(60))
})

#let caption(t) = text(size: 8pt, weight: "bold", fill: luma(60), t)

#let kind(r) = r.at("kind", default: "")

// The table cells of one row: a section's title across the table, closing the section before it
// with its blank rows; an item's name, slot and weight, then its notes the width of the table.
#let cells(r, first) = {
  let blanks = range(blank).map(_ => table.cell(colspan: 3, box(height: 0.5cm)))
  if kind(r) == "section" {
    (if first { () } else { blanks }) + (table.cell(colspan: 3, inset: (x: 4pt, top: 7pt, bottom: 3pt), text(size: 9.5pt, weight: "bold", r.label)),)
  } else {
    let line = (text(size: 9pt, r.label), text(size: 8.5pt, r.values.at(0, default: "")), text(size: 9pt, weight: "bold", r.values.at(1, default: "")))
    if r.notes.len() == 0 { line } else {
      line + (table.cell(colspan: 3, inset: (left: 10pt, right: 4pt, top: 0pt, bottom: 3pt), stack(spacing: 0.4em, ..r.notes.map(n => text(size: 8.5pt, fill: luma(40), n)))),)
    }
  }
}

#let look(b) = {
  let captions = b.rows.filter(r => kind(r) == "captions")
  let totals = b.rows.filter(r => kind(r) == "total")
  let lines = b.rows.filter(r => kind(r) != "captions" and kind(r) != "total")
  let sections = lines.filter(r => kind(r) == "section").len()
  let head = if captions.len() > 0 { (captions.first().label, ..captions.first().values).map(caption) } else { ([], [], []) }
  block(breakable: true, width: 100%, above: 1em, below: 0.6em)[
    #heading-of(b)
    #table(
      columns: (1fr, 3.2cm, 2.2cm),
      stroke: (x, y) => if y == 0 { (bottom: 0.6pt + luma(60)) } else { (bottom: 0.4pt + luma(190)) },
      inset: (x: 4pt, y: 3pt),
      align: (x, y) => if x == 2 { right } else { left },
      table.header(..head),
      ..lines.enumerate().map(((i, r)) => cells(r, i == 0)).flatten(),
      ..if sections > 0 { range(blank).map(_ => table.cell(colspan: 3, box(height: 0.5cm))) } else { () },
    )
    #v(0.4em)
    #grid(
      columns: (1fr,) * calc.max(totals.len(), 1),
      column-gutter: 4pt,
      ..totals.map(r => box(width: 100%, stroke: 0.6pt + luma(60), inset: 4pt, stack(spacing: 0.35em, text(size: 7.5pt, fill: luma(60), r.label), text(size: 10pt, weight: "bold", r.values.at(0, default: ""))))),
    )
    #metadata((sections: sections, blank: blank)) <gear-look>
  ]
}
