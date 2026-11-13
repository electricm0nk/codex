#!/usr/bin/env bash
# Rasterize every PDF in <pdf-dir> to 60 dpi PNGs in <png-dir>; prints one line per PNG.
# Exits 2 when <pdf-dir> holds no PDF. Used by the print_proof harness (SD-40 E1.2).
set -euo pipefail
if [ "$#" -ne 2 ]; then echo "usage: $0 <pdf-dir> <png-dir>" >&2; exit 64; fi
pdf_dir=$1; png_dir=$2
shopt -s nullglob
pdfs=("$pdf_dir"/*.pdf)
if [ "${#pdfs[@]}" -eq 0 ]; then echo "no PDFs in $pdf_dir" >&2; exit 2; fi
mkdir -p "$png_dir"
for pdf in "${pdfs[@]}"; do
  stem=$(basename "$pdf" .pdf)
  pdftoppm -r 60 -png "$pdf" "$png_dir/$stem"
done
for png in "$png_dir"/*.png; do echo "$png"; done
