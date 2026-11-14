// The `bio` look (SD-40 E3.4, decisions.md §23.1 item 5, §24.9): the Bio section's page. The heading
// ("Details") over the details as a two-column form, each label beside its value on a ruled line
// (an empty value is the bare line the player writes on); then each text row (the Biography, then
// each campaign) under its name, a paragraph per line; then blank ruled lines of 0.5 cm (SD-m) to
// the foot of the page. It formats strings it is given; it computes nothing (NR-2).
//
// The rows, as `print/blocks/core.rs` `character_details` sends them: kind `detail` (label, value 0)
// and kind `text` (label = the name, notes = the paragraphs); a row of another kind prints as a field.

#let slot = 0.5cm
// The page's bottom margin (frame.typ's `margin.y`): the ruled lines stop where the footer's band
// starts. A look cannot import the frame (the frame imports the looks).
#let foot = 1.8cm

#let heading-of(b) = block(sticky: true, below: 0.5em, {
  text(size: 12pt, weight: "bold", b.heading)
  v(-0.4em)
  line(length: 100%, stroke: 0.6pt + luma(60))
})

#let write-line = box(width: 100%, height: slot, stroke: (bottom: 0.5pt + luma(150)))

#let field(r) = grid(
  columns: (auto, 1fr),
  column-gutter: 0.5em,
  align: left + bottom,
  text(size: 8pt, weight: "bold", fill: luma(60), r.label),
  box(width: 100%, height: slot, stroke: (bottom: 0.6pt + luma(60)), inset: (x: 2pt, bottom: 2pt), align(left + bottom, text(size: 10pt, r.values.at(0, default: "")))),
)

#let look(b) = {
  let details = b.rows.filter(r => r.at("kind", default: "") != "text")
  let texts = b.rows.filter(r => r.at("kind", default: "") == "text")
  heading-of(b)
  grid(columns: (1fr, 1fr), column-gutter: 1.4em, row-gutter: 0.3em, ..details.map(field))
  for r in texts {
    block(sticky: true, above: 1.2em, below: 0.5em, text(size: 11pt, weight: "bold", r.label))
    for n in r.notes {
      block(above: 0.5em, below: 0.5em, text(size: 10pt, n))
    }
  }
  v(0.8em)
  // The lines that fit between here and the page's foot, whole: none spills onto a page of its own.
  context {
    let room = page.height - foot - here().position().y
    let lines = calc.max(0, calc.floor(room / slot))
    stack(..range(lines).map(_ => write-line))
    [#metadata((lines: lines, left: (room - lines * slot) / 1pt, slot: slot / 1pt)) <bio-look>]
  }
}
