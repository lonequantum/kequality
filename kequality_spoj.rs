// File to be submitted to spoj.com.


mod kingdom {
    #[allow(non_camel_case_types)]
    pub type size_k = usize;
    pub type CityId = size_k;
        type TreeId = size_k;

    // A road is a one-way direct link to a city.
    // It holds the number of cities that are reachable (destination and beyond) by taking it.
    struct Road {
        destination: CityId,
        reachable_cities_count: size_k // a value of 0 means uninitialized
    }

    // A city is a collection of roads to immediate next cities.
    // It also holds the ID of the tree it belongs to, and its depth in this tree.
    struct City {
        tree_id: TreeId,
        depth: size_k,
        roads: Vec<Road>
    }

    // A meeting point indicates the city where people in other cities are likely to meet.
    // Then people can also meet in the rest of the tree, only by taking roads that didn't lead them to the MP.
    struct MeetingPoint {
        city_id: CityId,
        traveled_distance: size_k,
        dont_go_back_to: Vec<CityId>
    }

    // A kingdom is a collection of (trees of) cities.
    // It also memoizes some internal results.
    pub struct Kingdom {
        cities: Vec<City>,
        trees_sizes: Vec<Option<size_k>>
    }

    impl Kingdom {
        // Preallocates memory for up to 200_000 cities.
        // TODO: replace with "transmute" code.
        pub fn new(number_of_cities: size_k) -> Kingdom {
            let mut cities = Vec::new();
            let mut trees_sizes = Vec::new();

            for tree_id in 0..number_of_cities {
                cities.push(City{
                    tree_id,
                    depth: 0,
                    roads: Vec::new()
                });
                trees_sizes.push(None);
            }

            Kingdom {cities, trees_sizes}
        }

        // Adds a two-way link between two cities.
        pub fn link(&mut self, mut city_id_1: CityId, mut city_id_2: CityId) {
            city_id_1 -= 1;
            city_id_2 -= 1;

            let city_1 = &mut self.cities[city_id_1];
            city_1.roads.push(Road{
                destination: city_id_2,
                reachable_cities_count: 0
            });
            let city_1_tree_id = city_1.tree_id;
            let city_1_depth = city_1.depth;

            let city_2 = &mut self.cities[city_id_2];
            city_2.roads.push(Road{
                destination: city_id_1,
                reachable_cities_count: 0
            });

            // Warning!: we assume input data is safely ordered.
            city_2.tree_id = city_1_tree_id;
            city_2.depth = city_1_depth + 1;
        }

        // Returns the answer for a query.
        pub fn solve(&mut self, mut query: Vec<CityId>) -> size_k {
            query[0] -= 1;

            // People in one city only can move anywhere in the tree.
            if query.len() == 1 {
                return self.tree_size(self.cities[query[0]].tree_id);
            }

            // People in different trees can't meet.
            let first_tree_id = self.cities[query[0]].tree_id;
            for city_id in &mut query[1..] {
                *city_id -= 1;
                if self.cities[*city_id].tree_id != first_tree_id {
                    return 0;
                }
            }

            let mut meeting_point_candidates = Vec::new();

            for (i, &city_id_i) in query.iter().enumerate() {
                let city_i = &self.cities[city_id_i];

                for &city_id_j in &query[(i + 1)..] {
                    let city_j = &self.cities[city_id_j];

                    if city_i.depth != city_j.depth {
                        if (city_i.depth + city_j.depth) % 2 == 1 {
                            return 0; // distance between two queried cities is an odd number of roads
                        }
                    } else {
                        meeting_point_candidates.push(
                            self.find_meeting_point_from_one_depth(city_id_i, city_id_j)
                        );
                    }
                }
            }

            42
        }

        // Finds the city that is the common ancestor of two cities that have the same depth.
        fn find_meeting_point_from_one_depth(&self, mut city_id_1: CityId, mut city_id_2: CityId) -> MeetingPoint {
            let mut old_city_id_1;
            let mut old_city_id_2;

            let mut traveled_distance = 0;

            loop {
                old_city_id_1 = city_id_1;
                old_city_id_2 = city_id_2;

                city_id_1 = self.cities[city_id_1].roads[0].destination;
                city_id_2 = self.cities[city_id_2].roads[0].destination;
                traveled_distance += 1;

                if city_id_1 == city_id_2 {break;}
            }

            MeetingPoint {
                city_id: city_id_1,
                traveled_distance,
                dont_go_back_to: vec![old_city_id_1, old_city_id_2]
            }
        }

        // Returns the number of cities of a given tree.
        fn tree_size(&mut self, tree_id: TreeId) -> size_k {
            let size = &mut self.trees_sizes[tree_id];

            if *size == None {
                *size = Some(self.cities.iter()
                                        .map(|city| city.tree_id)
                                        .filter(|id| *id == tree_id)
                                        .count());
            }

            size.unwrap()
        }
    }
}


// Parses input data and prints the solution of each query.
fn main() {
    use kingdom::*;
    use std::io;

    let number_of_cities: size_k = {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        line.trim().parse().unwrap()
    };

    let mut domki = Kingdom::new(number_of_cities);

    for _ in 1..number_of_cities {
        let mut line = String::new();
        let mut road_definition = {
            io::stdin().read_line(&mut line).unwrap();

            line.split_whitespace()
                .map(|value| value.parse().unwrap())
        };

        let city_id_1: CityId = road_definition.next().unwrap();
        let city_id_2: CityId = road_definition.next().unwrap();

        if road_definition.next().unwrap() == 1 { // closed roads can be ignored
            domki.link(city_id_1, city_id_2);
        }
    }

    let number_of_queries: u32 = {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        line.trim().parse().unwrap()
    };

    let mut answers = Vec::new();

    for _ in 0..number_of_queries {
        let query: Vec<CityId> = {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();

            line.split_whitespace()
                .skip(1) // first number on each line is useless
                .map(|value| value.parse().unwrap())
                .collect()
        };

        answers.push(domki.solve(query));
    }

    let mut expected_answers: Vec<Option<size_k>> = Vec::new();

    for _ in 0..number_of_queries {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                match line.trim().parse() {
                    Ok(value) => expected_answers.push(Some(value)),
                    Err(_) => expected_answers.push(None)
                }
            },

            Err(_) => expected_answers.push(None)
        }
    }

    for (&answer, &expected) in answers.iter().zip(expected_answers.iter()) {
        if expected == None {
            println!("{}", answer);
        } else {
            if expected.unwrap() != answer {
                print!("[ERR] ");
            } else {
                print!("[OK ] ");
            }
            println!("expected: {}\tcomputed: {}", expected.unwrap(), answer);
        }
    }
}
