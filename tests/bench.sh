# This file store run results to check performance and determinism.
cargo build --release
echo "" > saves/bench.txt
CMD="cargo run --release -- --config configs/bench_A.ron --seed 0 --run-for 50000 --exit"
echo ${CMD} >> saves/bench.txt
{ time ${CMD} &>/dev/null ; } 2>> saves/bench.txt
CMD="cargo run --release -- --config configs/bench_B.ron --seed 0 --run-for 50000 --exit"
echo ${CMD} >> saves/bench.txt
{ time ${CMD} &>/dev/null ; } 2>> saves/bench.txt
CMD="cargo run --release -- --config configs/bench_C.ron --seed 0 --run-for 50000 --exit"
echo ${CMD} >> saves/bench.txt
{ time ${CMD} &>/dev/null ; } 2>> saves/bench.txt
CMD="cargo run --release -- --config configs/bench_D.ron --seed 0 --run-for 50000 --exit"
echo ${CMD} >> saves/bench.txt
{ time ${CMD} &>/dev/null ; } 2>> saves/bench.txt