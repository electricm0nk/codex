// The `attack` look (SD-40 E2.1; E2.3 two to a line): Base Attack Bonus, Melee, CMB and CMD,
// each a labelled box, the label first. It formats strings it is given; it computes nothing (NR-2).

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

// Two labelled boxes to a line (Base Attack Bonus and Melee, then CMB and CMD), so the block sits
// low beside the Armor Class on page 1.
#let look(b) = {
  let n = b.value_columns
  let pair = (1fr,) + (1.1cm,) * n
  block(breakable: false, width: 100%, above: 1em, below: 0.6em)[
    #heading-of(b)
    #grid(
      columns: pair + (0.3cm,) + pair,
      column-gutter: 0.12cm,
      row-gutter: 0.12cm,
      ..b.rows.chunks(2).map(two => {
        let cells = ()
        for (i, r) in two.enumerate() {
          if i > 0 { cells.push([]) }
          cells.push(align(horizon, text(size: 9pt, weight: "bold", r.label)))
          cells += r.values.map(v => value-box(v, height: 0.6cm))
        }
        if two.len() == 1 { cells += ([],) * (1 + 1 + n) }
        cells
      }).flatten(),
    )
    #notes-of(b)
  ]
}
