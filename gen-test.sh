#!/bin/bash
output_file="test.txt"

number_of_cities=$(shuf -i 1-200000 -n 1)
echo $number_of_cities > "$output_file"

echo "1 2 1" >> "$output_file"
for city_id in $(seq 3 $number_of_cities); do
        limit=$((city_id - 1))
        start=$(shuf -i 1-${limit} -n 1)
        open=$RANDOM
        if [ $open -lt 1000 ]; then
                open=0
        else
                open=1
        fi
        echo "$start $city_id $open" >> "$output_file"
done

number_of_queries=$(shuf -i 1-200000 -n 1)
echo $number_of_queries >> "$output_file"

