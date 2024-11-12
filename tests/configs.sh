# Tests that will check that all configuration defined in the configs/ directory can load.
# Easy simple runs, typically use to compare between runs, code modification that should not impact the simulation
ERRORS=0
SEED=0
STEPS=100
cargo build
export CARGO_MANIFEST_DIR=`pwd`/exelixi
echo "-----------"
for CONFIG in `ls configs/`
do
    echo "-- Running with ${CONFIG}"
    cargo run -- --seed=${SEED} --config=configs/${CONFIG} --run-for=${STEPS} --exit >& /dev/null
    if [ $? != "0" ]
    then
        echo "  ##### ERROR #####"
        ERRORS=$((ERRORS + 1))
    fi
done
if [ ${ERRORS} != 0 ]; then
echo "-----------"
echo "--> ERRORS: ${ERRORS}"
else
echo "--> OK"
fi