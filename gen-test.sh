#!/bin/bash
# Generates random input data for performance testing

output_file="test.txt"

number_of_cities=$(shuf -i 1-200000 -n 1)
echo $number_of_cities > "$output_file"

echo -n "Generating $((number_of_cities - 1)) links… "

echo "1 2 1" >> "$output_file"
for city_id in $(seq 3 $number_of_cities); do
        limit=$((city_id - 1))
        start=$(shuf -i 1-${limit} -n 1)
        open=$RANDOM
        if [ $open -lt 2048 ]; then
                open=0
        else
                open=1
        fi
        echo "$start $city_id $open" >> "$output_file"
done
echo -e "\e[92mDone\e[0m"

echo -n "Generating queries… "

total_people=$(shuf -i 1-200000 -n 1)
declare -a queries

while [ $total_people -gt 0 ]; do
        people=$(shuf -i 1-${total_people} -n 1)
        cities=""

        limit=$((people > number_of_cities ? number_of_cities : people))
        length_of_query=$(shuf -i 1-${limit} -n 1)
        while [ $length_of_query -gt 0 ]; do
                cities="$cities $(shuf -i 1-${number_of_cities} -n 1)"
                length_of_query=$((length_of_query - 1))
        done

        cities=$(echo $cities | tr ' ' '\n' | sort -u | tr '\n' ' ')
        queries+=("$people ${cities:0:-1}")

        total_people=$((total_people - people))
done

number_of_queries=${#queries[@]}
echo $number_of_queries >> "$output_file"

for i in "${queries[@]}"; do
        echo "$i" >> "$output_file"
done

echo -e "\e[92m${number_of_queries}\e[0m"
