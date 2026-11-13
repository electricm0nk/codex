// The `feats` look (SD-40 E3.1, decisions.md §23.4: section 2, Abilities and Feats): the feat budget
// as one line under the heading (the row of kind `budget`, `<pool> <spent> of <granted>` per pool, as
// `print/blocks/feats.rs` joins the engine's `feats.budget.*` rows), then the feats two to a line, each
// its name and target in bold over its override tags and its rule's whole text; then two blank
// ruled rows for a feat the player writes in. It formats strings it is given; it computes nothing
// (NR-2).

#let blank = 2

#let heading-of(b) = {
  text(size: 12pt, weight: "bold", b.heading)
  v(-0.4em)
  line(length: 100%, stroke: 0.6pt + luma(60))
  v(-0.2em)
}

#let kind(r) = r.at("kind", default: "")

// One feat: its name (and any value it is given) over its words.
#let entry(r) = {
  text(size: 9.5pt, weight: "bold", (r.label, ..r.values).filter(t => t != "").join(" "))
  if r.notes.len() > 0 {
    v(-0.45em)
    stack(spacing: 0.4em, ..r.notes.map(n => text(size: 8.5pt, n)))
  }
}

#let write-line = box(width: 100%, height: 0.5cm, stroke: (bottom: 0.5pt + luma(60)))

#let look(b) = {
  let budget = b.rows.filter(r => kind(r) == "budget")
  let feats = b.rows.filter(r => kind(r) != "budget")
  block(breakable: true, width: 100%, above: 1em, below: 0.6em)[
    #heading-of(b)
    #for r in budget [
      #text(size: 9pt)[*#r.label:* #r.values.at(0, default: "")]
      #v(0.1em)
    ]
    #grid(
      columns: (1fr, 1fr),
      column-gutter: 0.6cm,
      row-gutter: 0.8em,
      ..feats.map(entry),
      // The write-in rows: two lines across both columns, on rows of their own after the feats (an
      // empty cell closes an odd feat's line).
      ..if calc.odd(feats.len()) { ([],) } else { () },
      ..range(blank).map(_ => grid.cell(colspan: 2, write-line)),
    )
    #metadata((columns: 2, blank: blank)) <feats-look>
  ]
}
