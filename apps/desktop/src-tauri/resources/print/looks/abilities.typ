// The `abilities` look (SD-40 E2.1; E2.10, decisions.md §23.1 item 3b): six lines, one per
// ability, each the score and the modifier the engine holds, then white space beside them for the
// pencil (no boxes). It formats strings it is given; it computes nothing (NR-2).

#let heading-of(b) = {
  text(size: 12pt, weight: "bold", b.heading)
  v(-0.4em)
  line(length: 100%, stroke: 0.6pt + luma(60))
  v(-0.2em)
}

// A value the engine filled, or (an empty string) a ruled write-in box the player fills.
#let value-box(v, width: 100%, height: 0.75cm, size: 11pt) = box(
  width: width, height: height, stroke: 0.6pt + luma(60),
  align(center + horizon, text(size: size, weight: "bold", v)),
)

#let caption(t) = text(size: 8pt, fill: luma(60), t)

// A row's notes (a rule's words, a condition), under the boxes.
#let notes-of(b) = {
  let notes = b.rows.map(r => r.notes).flatten()
  if notes.len() > 0 {
    v(0.2em)
    stack(spacing: 0.45em, ..notes.map(n => text(size: 8.5pt, fill: luma(40), n)))
  }
}

#let captions = ("Score", "Modifier")
#let label-column = 1fr
#let box-width = 1.1cm
#let operators = false

#let look(b) = {
  let n = b.value_columns
  // Between box i-1 and box i: "=" after the total, "+" after the rest (the rule, printed).
  let op(i) = if not operators { none } else if i == 1 { "=" } else { "+" }
  let columns = (2.4cm,)
  for i in range(n) {
    if i > 0 and operators { columns.push(0.35cm) }
    columns.push(box-width)
  }
  // The white space beside the boxes, for the pencil.
  columns.push(label-column)
  let cells-of(r) = {
    let out = (align(horizon, text(weight: "bold", r.label)),)
    for (i, v) in r.values.enumerate() {
      if i > 0 and operators { out.push(align(center + horizon, caption(op(i)))) }
      out.push(value-box(v, height: 0.6cm))
    }
    out.push([])
    out
  }
  let footer = ([],)
  for i in range(n) {
    if i > 0 and operators { footer.push([]) }
    footer.push(align(center, caption(captions.at(i, default: ""))))
  }
  footer.push([])
  block(breakable: false, width: 100%, above: 1em, below: 0.6em)[
    #heading-of(b)
    #grid(
      columns: columns,
      column-gutter: 0.15cm,
      row-gutter: 0.12cm,
      ..b.rows.map(cells-of).flatten(),
      ..footer,
    )
    #notes-of(b)
  ]
}
