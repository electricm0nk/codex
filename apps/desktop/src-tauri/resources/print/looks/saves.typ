// The `saves` look (SD-40 E2.1; E2.10 totals only, decisions.md §23.1 item 4; E2.20 one line):
// Fortitude, Reflex, Will, each its total, side by side. The save notes print in the Conditional
// Bonuses box (E2.20, decisions.md §24.3.4). The look adds nothing up (NR-2).

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

// The three saves to a line (Fortitude, Reflex, Will), each a labelled box, the label first, as the
// Attack prints two to a line: the left column holds the Conditional Bonuses box under them (E2.20).
#let look(b) = {
  let n = b.value_columns
  let pair = (1fr,) + (1.1cm,) * n
  block(breakable: false, width: 100%, above: 1em, below: 0.6em)[
    #heading-of(b)
    #grid(
      columns: (pair + (0.3cm,)) * calc.max(0, b.rows.len() - 1) + pair,
      column-gutter: 0.12cm,
      ..b.rows.enumerate().map(((i, r)) => {
        let cells = if i > 0 { ([],) } else { () }
        cells.push(align(horizon, text(size: 9pt, weight: "bold", r.label)))
        cells + r.values.map(v => value-box(v, height: 0.6cm))
      }).flatten(),
    )
  ]
}
