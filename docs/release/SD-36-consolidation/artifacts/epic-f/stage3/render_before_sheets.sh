#!/usr/bin/env bash
# SD-36 epic-f1b, R2 gate re-run: the BASELINE half of render_after_sheets.sh's own build
# manifest, rendered against data/sheet_rules AS CURRENTLY TRACKED (no swap) -- so both halves
# of blast-radius-classify.json's before/after diff reflect the SAME (corrected, R2-joined)
# class_census binary, isolating the F1 converter's link-repair effect from the join fix's own
# effect (the join fix changes BOTH before/after equally; re-running both sides with the same
# binary keeps the diff meaningful).
set -u
export PATH="$HOME/.cargo/bin:$PATH"
BIN=/home/ubuntu/workspace/worktrees/codex-epic-f1-target/debug/class_census
OUT=/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/6badc5b8-ae3b-4359-80c5-cd0b1598973e/scratchpad/sd36/f1/sheets/before
cd /home/ubuntu/workspace/worktrees/codex-epic-f1

run_one() {
  local name="$1" build="$2" race="${3:-}"
  local file="$OUT/${name}.txt"
  if [ -n "$race" ]; then
    "$BIN" --sheet-dump "$build" --with-sheet-rules --race "$race" > "$file" 2>"${file}.stderr"
  else
    "$BIN" --sheet-dump "$build" --with-sheet-rules > "$file" 2>"${file}.stderr"
  fi
}
export -f run_one
export BIN OUT

{
echo "wizard_L1 wizard:1 -"
echo "wizard_L5 wizard:5 -"
echo "wizard_L11 wizard:11 -"
echo "wizard_L20 wizard:20 -"
for fam in barbarian cleric rogue oracle summoner arcanist brawler unchained_monk gunslinger kineticist vigilante magus samurai warrior paladin; do
  for lvl in 1 7 14 20; do
    echo "${fam}_L${lvl} ${fam}:${lvl} -"
  done
done
echo "elf_wizard_L5 wizard:5 elf"
echo "dwarf_fighter_L5 fighter:5 dwarf"
echo "gnome_wizard_L5_ARG_alt_traits wizard:5 gnome"
echo "mix_barbarian12_fighter1 barbarian:12+fighter:1 -"
echo "mix_fighter6_wizard4 fighter:6+wizard:4 -"
echo "mix_cleric5_rogue3 cleric:5+rogue:3 -"
} | xargs -P 8 -L 1 bash -c 'run_one "$1" "$2" "$( [ "$3" = "-" ] && echo "" || echo "$3" )"' _

echo PAR_RENDER_BEFORE_DONE
