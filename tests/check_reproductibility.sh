CONFIG=bench_A
SEED=0
STEPS_A=33000
STEPS_B=6000
STEPS_C=39000
cargo run -- --seed=${SEED} --config=configs/${CONFIG}.ron --run-for=${STEPS_A} --save=saves/test_A.sim > saves/res_A.txt
cargo run --  --config=configs/${CONFIG}.ron --load=saves/test_A.sim --run-for=${STEPS_B} --save=saves/test_B.sim > saves/res_B.txt
cargo run -- --seed=${SEED} --config=configs/${CONFIG}.ron --run-for=${STEPS_C} --save=saves/test_C.sim > saves/res_C.txt

tail -n 5 saves/res_B.txt
tail -n 5 saves/res_C.txt

CONFIG=bench_A
SEED=0
STEPS_A=33000
STEPS_B=6000
STEPS_C=39000