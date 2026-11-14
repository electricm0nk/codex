// The `math` look (SD-40 E3.5, decisions.md §23.1 item 4, §17): the Math section's derivation lines,
// after its breakdown tables. The heading ("Show the Math"), then each printed number in two columns
// at 8 pt: its label (`block · label`) and value in bold, then the engine's rows behind it, verbatim
// (`(no derivation recorded)` where it recorded none). It formats strings it is given; it computes
// nothing (NR-2).

#let entry(r) = block(above: 0.55em, below: 0em, breakable: true, {
  text(weight: "bold", r.label)
  let value = r.values.filter(v => v != "").join(" ", default: "")
  if value != "" { h(0.5em); text(weight: "bold", value) }
  for n in r.notes {
    block(above: 0.25em, below: 0em, inset: (left: 6pt), text(fill: luma(40), n))
  }
})

#let look(b) = {
  block(sticky: true, above: 1em, below: 0.5em, {
    text(size: 11pt, weight: "bold", b.heading)
    v(-0.4em)
    line(length: 100%, stroke: 0.6pt + luma(60))
  })
  set text(size: 8pt)
  set par(justify: false)
  columns(2, gutter: 1.4em, for r in b.rows { entry(r) })
}
