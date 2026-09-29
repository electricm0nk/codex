#!/usr/bin/env bash
set -u
export PATH="$HOME/.cargo/bin:$PATH"
BIN="$1"
OUT="$2"
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

echo RENDER_VARIANT_DONE
