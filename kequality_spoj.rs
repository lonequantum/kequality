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

    // A kingdom is a collection of (trees of) cities.
    // It also memoizes some internal results.
    pub struct Kingdom {
        cities: Vec<City>,
        trees_sizes: Vec<size_k> // a value of 0 means uninitialized (0 as "no tree with that index/ID" is never used)
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
                trees_sizes.push(0);
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

            // Quick checking: eliminates trivial impossible cases.

            let first_tree_id = self.cities[query[0]].tree_id;
            for city_id in &mut query[1..] {
                *city_id -= 1;
                if self.cities[*city_id].tree_id != first_tree_id {
                    return 0; // not the same tree
                }
            }

            for (i, &city_id_1) in query.iter().enumerate() {
                let city_1_depth = self.cities[city_id_1].depth;
                for &city_id_2 in &query[(i + 1)..] {
                    if (city_1_depth + self.cities[city_id_2].depth) % 2 == 1 {
                        return 0; // distance between the two cities is an odd number of roads
                    }
                }
            }

            42
        }

        // Returns the number of cities of a given tree.
        fn tree_size(&mut self, tree_id: TreeId) -> size_k {
            let size = &mut self.trees_sizes[tree_id];

            if *size == 0 {
                *size = self.cities.iter()
                                   .map(|city| city.tree_id)
                                   .filter(|id| *id == tree_id)
                                   .count();
            }

            *size
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

        if let Some(1) = road_definition.next() { // closed roads can be ignored
            domki.link(city_id_1, city_id_2);
        }
    }

    let number_of_queries: u32 = {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        line.trim().parse().unwrap()
    };

    for _ in 0..number_of_queries {
        let query: Vec<CityId> = {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();

            line.split_whitespace()
                .skip(1) // first number on each line is useless
                .map(|value| value.parse().unwrap())
                .collect()
        };

        println!("{}", domki.solve(query));
    }
}
