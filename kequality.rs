// spoj.com requires the solution to be one file only,
// so we are not using any external definition.


mod kingdom {
    use std::collections::HashMap;

    #[allow(non_camel_case_types)]
    pub type size_k = usize;
    pub type CityId = size_k;

    // A road is a one-way link between two cities.
    struct Road {
        reachable_cities_count: size_k
    }

    // A city is a node in a tree.
    struct City {
        id: CityId, // convenient, but redundant with cities indexation at the kingdom level
        roads_to: HashMap<CityId, Road>
    }

    impl City {
        fn update_road_to(&mut self, destination: &City) {
            self.roads_to.insert(
                destination.id,
                Road {
                    reachable_cities_count: destination.count_reachable_cities(self.id) + 1
                }
            );
        }

        fn count_reachable_cities(&self, exclude: CityId) -> size_k {
            let mut count = 0;
            for (&city_id, road) in &self.roads_to {
                if city_id != exclude {
                    count += road.reachable_cities_count
                }
            }

            count
        }
    }

    // A kingdom is a set of trees
    pub struct Kingdom {
        cities: Vec<City> // indexes all nodes of all trees
    }

    impl Kingdom {
        pub fn new(number_of_cities: size_k) -> Kingdom {
            let mut cities = Vec::new();
            for i in 0..number_of_cities {
                cities.push(
                    City {
                        id: i + 1,
                        roads_to: HashMap::new()
                    }
                );
            }

            Kingdom {
                cities
            }
        }

        /*fn get_refmut_city(&mut self, city_id: CityId) -> &mut City {
            &mut self.cities[city_id - 1]
        }*/

        // Adds a two-way link between two cities
        pub fn add_roads(&mut self, city_id_1: CityId, city_id_2: CityId) {
            /*let city_1 = self.get_refmut_city(city_id_1);
            let city_2 = self.get_refmut_city(city_id_2);

            city_1.update_road_to(city_2);*/
        }

        pub fn solve(&self, query: Vec<CityId>) -> size_k {
            match query.len() {
                1 => 1,
                _ => 42
            }
        }
    }
}


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
        
        if let Some(1) = road_definition.next() {
            domki.add_roads(city_id_1, city_id_2);
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
