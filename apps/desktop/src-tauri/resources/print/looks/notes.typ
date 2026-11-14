// The `notes` look (SD-40 E4.1, decisions.md §24.9): the Notes section's page(s). The Notes tab's
// text, a paragraph per line, then blank ruled lines of 0.5 cm (SD-m) to the foot of the page, as the
// `bio` look ends its page; an empty text prints the ruled lines alone. It formats strings it is
// given; it computes nothing (NR-2).

#import "/looks/bio.typ": slot, foot, write-line

#let look(b) = {
  // The section's title is its heading; a heading, label or value given prints too.
  if b.heading != "" { block(sticky: true, below: 0.5em, text(size: 12pt, weight: "bold", b.heading)) }
  for r in b.rows {
    let line = (r.label, ..r.values).filter(t => t != "")
    if line.len() > 0 { block(above: 0.5em, below: 0.5em, text(size: 10pt, line.join(" "))) }
    for n in r.notes {
      block(above: 0.5em, below: 0.5em, text(size: 10pt, n))
    }
  }
  v(0.8em)
  // The lines that fit between here and the page's foot, whole: none spills onto a page of its own.
  context {
    let room = page.height - foot - here().position().y
    stack(..range(calc.max(0, calc.floor(room / slot))).map(_ => write-line))
  }
}
