// The `speed` look (SD-40 E2.1; E2.18, decisions.md §24.3.2): one line per mode (Land, Swim, Fly),
// its label beside a box holding what the engine states ("30 ft.", "60 ft. (average)"), or an empty
// box for the pencil. It formats; it computes nothing (NR-2).

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

#let look(b) = block(breakable: false, width: 100%, above: 1em, below: 0.6em)[
  #heading-of(b)
  #grid(
    columns: (auto, 1fr),
    column-gutter: 0.15cm,
    row-gutter: 0.12cm,
    align: (left + horizon, left),
    ..b.rows.map(r => (text(size: 8pt, weight: "bold", r.label), ..r.values.map(v => value-box(v, height: 0.7cm, size: 10pt)))).flatten(),
  )
  #notes-of(b)
]
