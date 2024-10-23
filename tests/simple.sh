# Easy simple runs, typically use to compare between runs, code modification that should not impact the simulation
CONFIG=bench_A
SEED=0
STEPS=10
cargo run -- --seed=${SEED} --config=configs/${CONFIG}.ron --run-for=${STEPS} --save=saves/simple.sim --exit > saves/simple.txt

tail -n 5 saves/simple.txt