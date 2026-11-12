// The `name-campaign` look (SD-40 E2.5, decisions.md §23.1 item 2): the screen's IdentityPanel
// beside the portrait. Each field is a small bold label over its value on a ruled line: the
// heading ("Character Name") over the first row's value (the name), then each later row's label
// ("Campaign") over its value; an empty value is the bare ruled line the player writes on. It
// formats; it computes nothing.

// A value wider than the cell (a long name, several campaigns) prints at 10 pt, wrapped over as
// many lines as it needs (E2.4): every word prints, nothing runs past the cell.
#let field(label, value, size: 10pt) = stack(
  spacing: 0.08cm,
  text(size: 7.5pt, weight: "bold", label),
  layout(cell => {
    let one-line = text(size: size, weight: "bold", value)
    if measure(one-line).width <= cell.width {
      box(width: 100%, height: size * 1.35, align(left + bottom, one-line))
    } else {
      block(width: 100%, above: 0em, below: 0em, inset: (top: 0.1cm), text(size: calc.min(size, 10pt), weight: "bold", value))
    }
  }),
  line(length: 100%, stroke: 0.6pt + luma(60)),
)

#let look(b) = {
  let first = b.rows.at(0, default: (label: "", values: ("",)))
  block(breakable: false, width: 100%, above: 0em, below: 0.6em, stack(
    spacing: 0.22cm,
    field((b.heading, first.label).filter(t => t != "").join(" · "), first.values.at(0, default: ""), size: 14pt),
    ..b.rows.slice(calc.min(1, b.rows.len())).map(r => field(r.label, r.values.at(0, default: ""))),
  ))
}
