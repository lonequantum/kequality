#!/bin/bash
# Generates random input data for performance testing

OUTPUT_FILE="test-perf.txt"

NUMBER_OF_CITIES=200000
TOTAL_PEOPLE_INIT=200000
MAX_CLOSED_ROADS=2

echo $NUMBER_OF_CITIES > "$OUTPUT_FILE"

echo -n "Generating $((NUMBER_OF_CITIES - 1)) links… "
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
done
echo -e "\e[92mDone\e[0m"

echo -n "Generating queries… "

total_people=$TOTAL_PEOPLE_INIT
declare -a queries

while [[ $total_people > 0 ]]; do
        people=$(shuf -i 1-${total_people} -n 1)
        cities=""

        limit=$((people > NUMBER_OF_CITIES ? NUMBER_OF_CITIES : people))
        length_of_query=$(shuf -i 1-${limit} -n 1)
        while [[ $length_of_query > 0 ]]; do
                cities="$cities $(shuf -i 1-${NUMBER_OF_CITIES} -n 1)"
                ((length_of_query--))
        done

        cities=$(echo $cities | tr ' ' '\n' | sort -u | tr '\n' ' ')
        queries+=("$people ${cities:0:-1}")

        ((total_people -= people))
done

number_of_queries=${#queries[@]}
echo $number_of_queries >> "$OUTPUT_FILE"

for i in "${queries[@]}"; do
        echo "$i" >> "$OUTPUT_FILE"
done

echo -e "\e[92m${number_of_queries}\e[0m"
