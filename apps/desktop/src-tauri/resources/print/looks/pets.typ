// The `pets` look (SD-40 E3.3, decisions.md §23.4: section 2): the companion's line (the row of kind
// `companion`: its role and species, its notes under it), its stats (rows of kind `stat`) as a strip
// of small labelled boxes, six to a line, then its notes (the row of kind `notes`); with no
// companion, the block's one line.
// It formats strings it is given; it computes nothing (NR-2).

// The heading stays on the page of the block's first line (`sticky`): never alone at a page's foot.
#let heading-of(b) = block(sticky: true, below: 0.5em, {
  text(size: 12pt, weight: "bold", b.heading)
  v(-0.4em)
  line(length: 100%, stroke: 0.6pt + luma(60))
})

#let kind(r) = r.at("kind", default: "")

#let line-of(r) = block(below: 0.4em, {
  text(size: 9pt, weight: "bold", (r.label, ..r.values).filter(t => t != "").join(" "))
  for n in r.notes { linebreak(); text(size: 8pt, fill: luma(40), n) }
})

#let look(b) = {
  let stats = b.rows.filter(r => kind(r) == "stat")
  block(breakable: true, width: 100%, above: 1em, below: 0.6em)[
    #heading-of(b)
    #for r in b.rows.filter(r => kind(r) != "stat" and kind(r) != "notes") { line-of(r) }
    #if stats.len() > 0 {
      grid(
        columns: (1fr,) * 6,
        column-gutter: 4pt,
        row-gutter: 4pt,
        ..stats.map(r => box(width: 100%, stroke: 0.6pt + luma(60), inset: 4pt, stack(spacing: 0.35em, text(size: 7.5pt, fill: luma(60), r.label), text(size: 10pt, weight: "bold", r.values.at(0, default: ""))))),
      )
    }
    #for r in b.rows.filter(r => kind(r) == "notes") { v(0.3em); line-of(r) }
  ]
}
