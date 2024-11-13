# Easy simple runs, typically use to compare between runs, code modification that should not impact the simulation
CONFIG=default
SEED=0
STEPS=10000

TIME=`date +%y-%m-%d-%H:%M:%S`
COMMIT=`git log --format="%h" -n 1`
SAVE="saves/simple-${CONFIG}-SEED_${SEED}-STEPS_${STEPS}-${TIME}-${COMMIT}"
cargo run -- --seed=${SEED} --config=configs/${CONFIG}.ron --run-for=${STEPS} --save=${SAVE}.sim --exit > ${SAVE}.txt
echo "Output saved to ${SAVE}.txt"

tail -n 5 ${SAVE}.txt