#!/bin/bash
# Generates random input data for performance testing

OUTPUT_FILE="test-perf.txt"
NUMBER_OF_CITIES=200000
TOTAL_PEOPLE_INIT=200000
MAX_CLOSED_ROADS=2

echo $NUMBER_OF_CITIES > "$OUTPUT_FILE"

closed_roads=0
echo "1 2 1" >> "$OUTPUT_FILE"

for city_id in $(seq 3 $NUMBER_OF_CITIES); do
        limit=$((city_id - 1))
        start=$(shuf -i 1-${limit} -n 1)

        open=$RANDOM
        if [[ $open < 16 && $closed_roads < $MAX_CLOSED_ROADS ]]; then
                open=0
                ((closed_roads++))
        else
                open=1
        fi

        echo "$start $city_id $open" >> "$OUTPUT_FILE"
        echo -en "\rGenerating $((NUMBER_OF_CITIES - 1)) links… $((city_id * 100 / NUMBER_OF_CITIES))%"
done

echo

total_people=$TOTAL_PEOPLE_INIT
declare -a queries

while [[ $total_people > 0 ]]; do
        people=$(shuf -i 1-${total_people} -n 1)

        limit=$((people > NUMBER_OF_CITIES ? NUMBER_OF_CITIES : people))
        length_of_query=$(shuf -i 1-${limit} -n 1)

        cities=$(shuf -i 1-${NUMBER_OF_CITIES} -n ${length_of_query} | tr '\n' ' ')

        queries+=("$people ${cities:0:-1}")
        ((total_people -= people))

        echo -en "\rGenerating queries… $((100 - total_people * 100 / TOTAL_PEOPLE_INIT))%"
done

number_of_queries=${#queries[@]}
echo $number_of_queries >> "$OUTPUT_FILE"
echo " ($number_of_queries)"

for i in "${queries[@]}"; do
        echo "$i" >> "$OUTPUT_FILE"
done

echo "Written to $OUTPUT_FILE"
