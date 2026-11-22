// The `languages` look (SD-40 E2.13, decisions.md §23.1 item 6; E2.16, §24.3.3): the Skills panel's
// Languages line under the portrait box: the small bold label (as the Character Name and Campaign
// labels beside it), then the known languages as one comma list, wrapping inside the cell when
// long (seven names fit), with no write-in line. It formats; it computes nothing.

#let look(b) = {
  let known = b.rows.map(r => (r.label, ..r.values).filter(t => t != "").join(" ")).filter(t => t != "")
  block(breakable: false, width: 100%, above: 0.6em, below: 0.2em,
    par(hanging-indent: 0.4cm, [#text(size: 8pt, weight: "bold", b.heading) #h(0.2cm) #text(size: 9pt, known.join(", "))]))
}
