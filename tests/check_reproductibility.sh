CONFIG=bench_A
SEED=0
STEPS_A=10000
STEPS_B=10000
STEPS_C=20000
cargo run -- --seed=${SEED} --config=configs/${CONFIG}.ron --run-for=${STEPS_A} --save=saves/test_A.sim --exit > saves/res_A.txt
cargo run --  --config=configs/${CONFIG}.ron --load=saves/test_A.sim --run-for=${STEPS_B} --save=saves/test_B.sim --exit > saves/res_B.txt
cargo run -- --seed=${SEED} --config=configs/${CONFIG}.ron --run-for=${STEPS_C} --save=saves/test_C.sim --exit > saves/res_C.txt

tail -n 5 saves/res_B.txt
tail -n 5 saves/res_C.txt


# Failure:
CONFIG=bench_D
SEED=0
STEPS_A=100000
STEPS_B=100000
STEPS_C=200000

#Steps: 200000
#  Herbivore  - Size:   64 Energy:  1410470 Deaths:    0 Generation:40   Mean_Pos:(-1296.904,  312.705)
#  Carnivore  - Size:    1 Energy:    24796 Deaths:    0 Generation:29   Mean_Pos:(-2313.954,-2624.951)
#  Plant      - Size: 9979 Energy:124807520 Deaths:    0 Generation:0    Mean_Pos:(   39.159,   -6.965)
#
#Steps: 200000
#  Herbivore  - Size:   64 Energy:  1410470 Deaths:    0 Generation:40   Mean_Pos:(-1296.904,  312.705)
#  Carnivore  - Size:    1 Energy:    24796 Deaths:    0 Generation:29   Mean_Pos:(-2313.954,-2624.951)
#  Plant      - Size: 9979 Energy:124807664 Deaths:    0 Generation:0    Mean_Pos:(   39.159,   -6.965)

CONFIG=HerbivoreCarnivoreFullEye
SEED=0
STEPS_A=100000
STEPS_B=100000
STEPS_C=200000

#Steps: 200000
#  Herbivore  - Size:  198 Energy:  6092556 Deaths:    0 Generation:40   Mean_Pos:( -913.663, -452.010)
#  Carnivore  - Size:    6 Energy:   147062 Deaths:    0 Generation:29   Mean_Pos:(-2405.319,  884.338)
#  Plant      - Size: 1613 Energy:  9105700 Deaths:    0 Generation:0    Mean_Pos:(  380.873,  256.546)
#
#Steps: 200000
#  Herbivore  - Size:  234 Energy:  6850926 Deaths:    0 Generation:40   Mean_Pos:( -967.016,  754.221)
#  Carnivore  - Size:    5 Energy:   153181 Deaths:    0 Generation:29   Mean_Pos:(-1531.895,-1713.462)
#  Plant      - Size: 1482 Energy:  8372520 Deaths:    0 Generation:0    Mean_Pos:(  427.196, -437.472)