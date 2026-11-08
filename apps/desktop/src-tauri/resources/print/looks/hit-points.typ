// The `hit-points` look (SD-40 E2.1, decisions.md §11, SD-m): the total the engine holds and the
// current and nonlethal damage write-ins, each a 2 cm box the player pencils at the table, then
// Damage Reduction when the screen shows one. It formats strings it is given; it computes nothing.

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

// One labelled box: the label above (so it reads before its value), the box under it.
#let labelled(r, height: 0.75cm, size: 11pt) = stack(
  spacing: 0.12cm,
  text(size: 8pt, weight: "bold", r.label),
  ..r.values.map(v => value-box(v, height: height, size: size)),
)

#let look(b) = block(breakable: false, width: 100%, above: 1em, below: 0.6em)[
  #heading-of(b)
  #grid(
    columns: (1fr,) * (if b.rows.len() > 0 { b.rows.len() } else { 1 }),
    column-gutter: 0.25cm,
    ..b.rows.map(r => labelled(r, height: 2cm, size: 16pt)),
  )
  #notes-of(b)
]
