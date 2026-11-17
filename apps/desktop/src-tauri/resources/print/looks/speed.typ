// The `speed` look (SD-40 E2.1; E2.18, decisions.md §24.3.2; SD-41 decisions.md §34 (6)): a 2×2 grid
// of the modes in the order the block sends them (Land, Fly over Swim, Climb), each label beside a
// box holding what the engine states ("30 ft.", "—"), or an empty box for the pencil; a value's
// words in brackets (the fly maneuverability, "60 ft. (average)") print under its box. It formats;
// it computes nothing (NR-2).

#import "/looks/section-title.typ": section-title

#let heading-of(b) = section-title(b.heading)

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

// A mode's box, at 9 pt so a three-digit speed fits half the block's width; the words in brackets after its feet ("60 ft. (average)") print under the box.
#let mode-box(v) = {
  let at = v.position(" (")
  if at == none { value-box(v, height: 0.7cm, size: 9pt) } else {
    stack(spacing: 0.08cm, value-box(v.slice(0, at), height: 0.7cm, size: 9pt), align(center, caption(v.slice(at + 1))))
  }
}

// Two modes to a row, each a label and its box.
#let modes-per-row = 2

#let look(b) = block(breakable: false, width: 100%, above: 1em, below: 0.6em)[
  #heading-of(b)
  #grid(
    columns: (auto, 1fr) * modes-per-row,
    column-gutter: 0.15cm,
    row-gutter: 0.12cm,
    align: (left + horizon, left) * modes-per-row,
    ..b.rows.map(r => (text(size: 8pt, weight: "bold", r.label), ..r.values.map(mode-box))).flatten(),
  )
  #metadata((columns: modes-per-row, rows: calc.ceil(b.rows.len() / modes-per-row))) <speed-look>
  #notes-of(b)
]
