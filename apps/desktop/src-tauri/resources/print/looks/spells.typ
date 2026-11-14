// The `spells` look (SD-40 E3.3, decisions.md §23.4: section 2, the daily spells list). Per class (a
// row of kind `class`: the class, its caster level, any note): a table of its spell levels (rows of
// kind `level`: the save DC, the spells per day, known / prepared) with one **used** write-in box per
// spell per day, or a write-in line where the engine states no count; then the class's spell names
// (rows of kind `spell`: name, level) by level in two columns. It formats strings it is given; the
// boxes are drawn as many as the per-day string says; it computes nothing (NR-2).

#let columns = 2

// The heading stays on the page of the block's first line (`sticky`): never alone at a page's foot.
#let heading-of(b) = block(sticky: true, below: 0.5em, {
  text(size: 12pt, weight: "bold", b.heading)
  v(-0.4em)
  line(length: 100%, stroke: 0.6pt + luma(60))
})

#let caption(t) = text(size: 8pt, weight: "bold", fill: luma(60), t)

#let kind(r) = r.at("kind", default: "")

// How many used boxes a level gets: its per-day count when that is a whole number, else none.
#let box-count(r) = {
  let per-day = r.values.at(1, default: "")
  if per-day.match(regex("^\d+$")) != none { int(per-day) } else { 0 }
}

#let used-box = box(width: 0.34cm, height: 0.34cm, stroke: 0.5pt + luma(60))

#let used(r) = {
  let n = box-count(r)
  if n == 0 { box(width: 100%, height: 0.34cm, stroke: (bottom: 0.5pt + luma(60))) } else { range(n).map(_ => used-box).join(h(2pt)) }
}

// One class: its line, its level table, its names by level.
#let class-look(rows) = {
  let class = rows.first()
  let levels = rows.filter(r => kind(r) == "level")
  let spells = rows.filter(r => kind(r) == "spell")
  block(sticky: true, below: 0.4em, grid(
    columns: (1fr, auto),
    text(size: 10.5pt, weight: "bold", class.label),
    if class.values.at(0, default: "") != "" { text(size: 9pt)[Caster level *#class.values.at(0)*] },
  ))
  for n in class.notes { block(above: 0.2em, below: 0.3em, text(size: 8pt, fill: luma(40), n)) }
  if levels.len() > 0 {
    table(
      columns: (1.6cm, 1.3cm, 1.5cm, 2.6cm, 1fr),
      stroke: (x, y) => if y == 0 { (bottom: 0.6pt + luma(60)) } else { (bottom: 0.4pt + luma(190)) },
      inset: (x: 4pt, y: 3pt),
      align: (x, y) => if x in (1, 2, 3) { center + horizon } else { left + horizon },
      table.header(..("Level", "DC", "Per day", "Known / Prepared", "Used").map(caption)),
      ..levels.map(r => {
        let line = (text(size: 9pt, weight: "bold", r.label), text(size: 9pt, r.values.at(0)), text(size: 9pt, weight: "bold", r.values.at(1)), text(size: 9pt, r.values.at(2)), used(r))
        if r.notes.len() == 0 { line } else {
          line + (table.cell(colspan: 5, inset: (left: 10pt, right: 4pt, top: 0pt, bottom: 3pt), stack(spacing: 0.4em, ..r.notes.map(n => text(size: 8pt, fill: luma(40), n)))),)
        }
      }).flatten(),
    )
  }
  if spells.len() > 0 {
    // The spell levels in the order the names are given (lowest first), each its names.
    let at = ()
    for r in spells { if not at.contains(r.values.at(0)) { at.push(r.values.at(0)) } }
    block(above: 0.5em, grid(
      columns: (1fr,) * columns,
      column-gutter: 0.6cm,
      row-gutter: 0.5em,
      ..at.map(level => {
        let names = spells.filter(r => r.values.at(0) == level).map(r => if r.notes.len() == 0 { r.label } else { r.label + " (" + r.notes.join(", ") + ")" })
        text(size: 9pt)[*Level #level:* #names.join(", ")]
      }),
    ))
  }
}

#let look(b) = {
  let starts = b.rows.enumerate().filter(((i, r)) => kind(r) == "class").map(((i, r)) => i)
  let ends = starts.slice(calc.min(1, starts.len())) + (b.rows.len(),)
  let classes = starts.zip(ends).map(((s, e)) => b.rows.slice(s, e))
  let boxes = b.rows.filter(r => kind(r) == "level").map(box-count).sum(default: 0)
  block(breakable: true, width: 100%, above: 1em, below: 0.6em)[
    #heading-of(b)
    #for r in b.rows.filter(r => kind(r) == "") { block(below: 0.4em, text(size: 9pt, (r.label, ..r.values).filter(t => t != "").join(" "))) }
    #for (i, rows) in classes.enumerate() {
      if i > 0 { v(0.6em) }
      class-look(rows)
    }
    #metadata((boxes: boxes, columns: columns)) <spells-look>
  ]
}
