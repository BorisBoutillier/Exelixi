# This file store run results to check performance and determinism.
PFX=${1}
TIME=`date +%y-%m-%d-%H:%M:%S`
COMMIT=`git log --format="%h" -n 1`
SAVE="saves/bench-${PFX}-${TIME}-${COMMIT}.txt"
cargo build --release
echo "Bench result saved to ${SAVE}"
echo "" > ${SAVE}
CMD="cargo run --release -- --config configs/bench_A.ron --seed 0 --run-for 50000 --exit"
echo ${CMD} >> ${SAVE}
{ time ${CMD} &>/dev/null ; } 2>> ${SAVE}
CMD="cargo run --release -- --config configs/bench_B.ron --seed 0 --run-for 50000 --exit"
echo ${CMD} >> ${SAVE}
{ time ${CMD} &>/dev/null ; } 2>> ${SAVE}
CMD="cargo run --release -- --config configs/bench_C.ron --seed 0 --run-for 50000 --exit"
echo ${CMD} >> ${SAVE}
{ time ${CMD} &>/dev/null ; } 2>> ${SAVE}
CMD="cargo run --release -- --config configs/bench_D.ron --seed 0 --run-for 50000 --exit"
echo ${CMD} >> ${SAVE}
{ time ${CMD} &>/dev/null ; } 2>> ${SAVE}