# This file store run results to check performance and determinism.
PFX=${1}
TIME=`date +%y-%m-%d-%H:%M:%S`
COMMIT=`git log --format="%h" -n 1`
SAVE="saves/bench-${PFX}-${TIME}-${COMMIT}.txt"
ERRORS=0
cargo build --release
echo "-----------"
echo "" > ${SAVE}
for BENCH in `ls configs/bench*.ron`
do
    echo "-- Running with ${BENCH}"
    CMD="cargo run --release -- --config ${BENCH} --seed 0 --run-for 50000 --exit"
    echo ${CMD} >> ${SAVE}
    { time ${CMD} >/dev/null ;
        if [ $? != "0" ]
        then
            echo "  ##### ERROR #####"
            ERRORS=$((ERRORS + 1))
        fi
    } 2>> ${SAVE}
done
echo "-----------"
echo "Bench result saved to ${SAVE}"
if [ ${ERRORS} != 0 ]; then
echo "--> ERRORS: ${ERRORS}, see save"
else
echo "--> OK"
fi