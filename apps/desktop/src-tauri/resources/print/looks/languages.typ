// The `languages` look (SD-40 E2.13, decisions.md §23.1 item 6, §23.7 Q1): the Skills panel's
// Languages line, one line under the portrait box: the small bold label (as the Character Name and
// Campaign labels beside it), the known languages as a comma list (it wraps only when long), then
// the one ruled write-in line for a language learned at the table, running to the cell's edge; when
// less than 1.2 cm is left beside the list, the ruled line takes a line of its own (0.5 cm, SD-m).
// It formats; it computes nothing.

#let min-write-in = 1.2cm

#let ruled(width) = box(width: width, height: 0.5cm, align(bottom, line(length: 100%, stroke: 0.4pt + luma(120))))

#let look(b) = {
  // Each row that holds a value prints as `<label> <value>` (the label empty for the known list);
  // each empty value is one write-in line.
  let known = b.rows.filter(r => r.values.any(v => v != "")).map(r => (r.label, ..r.values).filter(t => t != "").join(" "))
  let blanks = b.rows.map(r => r.values).flatten().filter(v => v == "").len()
  let head = [#text(size: 8pt, weight: "bold", b.heading) #h(0.2cm) #text(size: 9pt, known.join(", "))]
  block(breakable: false, width: 100%, above: 0.6em, below: 0.2em, layout(size => {
    let used = measure(head).width
    if blanks == 0 {
      par(hanging-indent: 0.4cm, head)
    } else if used + min-write-in <= size.width {
      // The list fits on one line with room to write: the first ruled line runs on beside it.
      box(height: 0.5cm, align(bottom, head + h(0.2cm) + ruled(size.width - used - 0.2cm)))
      for _ in range(blanks - 1) { linebreak(); ruled(100%) }
    } else {
      par(hanging-indent: 0.4cm, head)
      for _ in range(blanks) { ruled(100%) }
    }
  }))
}
