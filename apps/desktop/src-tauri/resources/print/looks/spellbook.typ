// The `spellbook` look (SD-40 E3.3, decisions.md §22 "2: A": section 4, spells only; R3: PCGen's
// 3x5 spell cards as the picture). One card per spell (rows of kind `spell`), two to a line: its
// name over a rule, then each field under the caption the row of kind `captions` gives it (School,
// Level, Casting time, Components, Range, Target or area, Duration, Saving throw, Spell resistance),
// then the spell's text. It formats strings it is given; it computes nothing (NR-2).

#let columns = 2

// The heading stays on the page of the block's first line (`sticky`): never alone at a page's foot.
#let heading-of(b) = block(sticky: true, below: 0.5em, {
  text(size: 12pt, weight: "bold", b.heading)
  v(-0.4em)
  line(length: 100%, stroke: 0.6pt + luma(60))
})

#let kind(r) = r.at("kind", default: "")

#let card(captions, r) = block(width: 100%, stroke: 0.6pt + luma(60), inset: 6pt, {
  block(sticky: true, below: 0.35em, text(size: 10.5pt, weight: "bold", r.label))
  v(-0.2em)
  line(length: 100%, stroke: 0.4pt + luma(120))
  v(-0.2em)
  stack(spacing: 0.45em, ..captions.zip(r.values).map(((c, v)) => text(size: 8.5pt)[*#c:* #v]))
  if r.notes.len() > 0 {
    v(0.2em)
    stack(spacing: 0.45em, ..r.notes.map(n => text(size: 8.5pt, n)))
  }
})

#let look(b) = {
  let captions = b.rows.filter(r => kind(r) == "captions").map(r => r.values).at(0, default: ())
  let spells = b.rows.filter(r => kind(r) == "spell")
  block(breakable: true, width: 100%, above: 1em, below: 0.6em)[
    #heading-of(b)
    #for r in b.rows.filter(r => kind(r) == "") { block(below: 0.4em, text(size: 9pt, (r.label, ..r.values).filter(t => t != "").join(" "))) }
    #grid(
      columns: (1fr,) * columns,
      column-gutter: 0.4cm,
      row-gutter: 0.4cm,
      ..spells.map(r => card(captions, r)),
    )
    #metadata((entries: spells.len(), columns: columns)) <spellbook-look>
  ]
}
