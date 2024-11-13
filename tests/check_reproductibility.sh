ERRORS=0
SEED=0
STEPS_A=50000
STEPS_B=50000
STEPS_C=100000
PROFILE="--release"
export CARGO_MANIFEST_DIR=`pwd`/exelixi
mkdir saves/reproductibility
echo "-----------"
for CONFIG in `ls configs/*.ron`
do
    echo "---------------"
    echo "-- Running with ${CONFIG}"
    echo "---------------"
    BASENAME=$(basename $CONFIG .ron)
    cargo run ${PROFILE} -- --seed=${SEED} --config ${CONFIG} --run-for=${STEPS_A} --save=saves/reproductibility/${BASENAME}_A.sim --exit > saves/reproductibility/${BASENAME}_A.txt
    cargo run ${PROFILE} --  --config ${CONFIG} --load=saves/reproductibility/${BASENAME}_A.sim --run-for=${STEPS_B} --save=saves/reproductibility/${BASENAME}_B.sim --exit > saves/reproductibility/${BASENAME}_B.txt
    cargo run ${PROFILE} -- --seed=${SEED} --config ${CONFIG} --run-for=${STEPS_C} --save=saves/reproductibility/${BASENAME}_C.sim --exit > saves/reproductibility/${BASENAME}_C.txt

    OUT_B=$(tail -n 10 saves/reproductibility/${BASENAME}_B.txt)
    OUT_C=$(tail -n 10 saves/reproductibility/${BASENAME}_C.txt) 
    if [[ ${OUT_C} != ${OUT_B} ]]
    then
        echo "  ##### Differences for ${CONFIG} #####"
        ERRORS=$((ERRORS + 1))
    fi
done
if [ ${ERRORS} != 0 ]; then
echo "-----------"
echo "--> ERRORS: ${ERRORS}"
else
echo "--> OK"
fi