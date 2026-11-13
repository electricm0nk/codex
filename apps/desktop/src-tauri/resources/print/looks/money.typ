// The `money` look (SD-40 E3.2, decisions.md §23.1 item 5: section 3, Inventory): four coin boxes in
// a row, each its label (PP, GP, SP, CP) over the value it is given (empty: a box for the pencil),
// then a "Ledger" of four ruled write-in lines for what the player spends and earns at the table.
// It formats strings it is given; it computes nothing (NR-2).

#let ledger = 4

// The heading stays on the page of the block's first line (`sticky`): never alone at a page's foot.
#let heading-of(b) = block(sticky: true, below: 0.5em, {
  text(size: 12pt, weight: "bold", b.heading)
  v(-0.4em)
  line(length: 100%, stroke: 0.6pt + luma(60))
})

#let coin(r) = box(width: 100%, height: 1.4cm, stroke: 0.8pt + luma(40), inset: 5pt, stack(
  spacing: 0.45em,
  text(size: 8pt, weight: "bold", fill: luma(60), r.label),
  text(size: 12pt, weight: "bold", r.values.at(0, default: "")),
))

#let write-line = box(width: 100%, height: 0.5cm, stroke: (bottom: 0.5pt + luma(60)))

#let look(b) = {
  block(breakable: false, width: 100%, above: 1em, below: 0.6em)[
    #heading-of(b)
    #grid(columns: (1fr,) * b.rows.len(), column-gutter: 6pt, ..b.rows.map(coin))
    #v(0.5em)
    #text(size: 8pt, weight: "bold", fill: luma(60), "Ledger")
    #v(-0.3em)
    #stack(..range(ledger).map(_ => write-line))
    #metadata((boxes: b.rows.len(), ledger: ledger)) <money-look>
  ]
}
