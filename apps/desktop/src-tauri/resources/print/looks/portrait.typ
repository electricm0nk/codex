// The `portrait` look (SD-40 E2.5, decisions.md §23.1 item 1): page 1's top-left box. A framed
// box with its small caption (the block's heading, then any row's words) at the top; the saved
// portrait, when the block carries one, scaled inside it to fit, aspect kept, centred; else the
// box stays empty for the player to draw in. It formats what it is given; it computes nothing.

#let caption-of(b) = text(size: 7pt, fill: luma(60), (b.heading, ..b.rows.map(r => (r.label, ..r.values).join(" "))).join(" · "))

#let look(b) = block(breakable: false, width: 100%, above: 0em, below: 0.6em,
  box(width: 100%, height: 3.4cm, stroke: 0.8pt + luma(60), inset: 0.12cm, grid(
    rows: (auto, 1fr),
    row-gutter: 0.08cm,
    caption-of(b),
    if b.at("image", default: none) != none {
      align(center + horizon, image(b.image.path, width: 100%, height: 100%, fit: "contain"))
    },
  )),
)
