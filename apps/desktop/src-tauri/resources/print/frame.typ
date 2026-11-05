// The shared page frame (SD-39 operator ruling R2). Every system's sheet prints through it: the
// paper, the one bundled font family, the title, the page footer, then the layout file's blocks
// in order. The frame names no inch for any block; Typst flows them onto pages.
//
// `doc` is the `SheetDocument` the Rust layout file built (`src/print/model.rs`), passed as JSON.
// It carries text only: no number is computed here (NR-2).

#import "/blocks.typ": sheet-block

#let sheet(doc) = {
  set document(title: doc.title, date: none)
  set page(
    paper: doc.paper,
    margin: (x: 1.6cm, y: 1.8cm),
    footer: context align(center, text(size: 8pt, fill: luma(90))[
      #doc.title · page #counter(page).display() of #counter(page).final().first()
    ]),
  )
  set text(font: "Liberation Sans", size: 10pt, lang: "en")
  set par(justify: false)

  align(left, text(size: 18pt, weight: "bold", doc.title))
  v(0.4em)
  for b in doc.blocks {
    sheet-block(b)
  }
}
