// The `portrait` look (SD-40 E2.5; SD-41 E4.4, decisions.md §34 (7)): page 1's top-left box. A
// framed box holding the block's image, scaled inside it to fit, aspect kept, centred: the saved
// portrait, else the game system's logo (the caller chooses). It never prints a caption or the
// word "portrait"; with no image the box stays empty. It formats what it is given; it computes nothing.

#let look(b) = block(breakable: false, width: 100%, above: 0em, below: 0.6em,
  box(width: 100%, height: 3.4cm, stroke: 0.8pt + luma(60), inset: 0.12cm,
    if b.at("image", default: none) != none {
      align(center + horizon, image(b.image.path, width: 100%, height: 100%, fit: "contain"))
    },
  ),
)
