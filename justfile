work day part:
  cargo watch -w {{day}} -x "check -p {{day}}" -s "just test {{day}} {{part}}" -s "just lint {{day}}"
lint day:
  cargo clippy -p {{day}}
test day part:
  cargo nextest run -p {{day}} {{part}}
bench-all:
  cargo bench -q > benchmarks.txt
bench day part:
  cargo bench --bench {{day}}-bench {{part}} >> {{day}}.bench.txt
create day:
  cargo generate --path ./daily-template --name {{day}}
  just get-input {{day}}

get-input day:
  ./scripts/get-aoc-input.rs --day {{day}} --current-working-directory {{justfile_directory()}}
