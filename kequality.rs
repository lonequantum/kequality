// SPOJ.com requires the solution to be one file only,
// so we are not using any external definition.


// A kingdom is a set of trees
// (there can be more than one tree if a road between two cities is closed).
mod kingdom {

    // A city is a node in a tree.
    mod city {
        use std::collections::HashMap;

        pub type Id = u32; // for values between 1 and 200_000
        pub type IdSized = u32; // for counting cities or roads - can't exceed the maximum Id

        pub struct City {
            //id: Id, // can be removed - redundant with cities indexation
            roads_to: HashMap<Id, IdSized>, // links to other cities
        }

        impl City {
            pub fn new() -> City {
                City {
                    roads_to: HashMap::new(),
                }
            }

            // Adds a one-way road from our city to another city.
            pub fn add_road_to(&mut self, other_city_id: Id) -> &mut City {
                self.roads_to.entry(other_city_id).or_insert(0);

                self
            }
        }
    }


    use kingdom::city::City;
    pub use kingdom::city::Id as CityId;
    pub use kingdom::city::IdSized as CityIdSized;
    use std::collections::HashMap;

    pub struct Kingdom {
        cities: HashMap<CityId, City>, // indexes all nodes of all trees
    }

    impl Kingdom {
        pub fn new() -> Kingdom {
            Kingdom {
                cities: HashMap::new(),
            }
        }

        // Adds a two-ways road between two cities
        // (we automatically add cities if they don't exist).
        pub fn add_road(&mut self, city_id_1: CityId, city_id_2: CityId) -> &mut Kingdom {
            self.insert_city(city_id_1).add_road_to(city_id_2);
            self.insert_city(city_id_2).add_road_to(city_id_1);

            self
        }

        pub fn add_city(&mut self, city_id: CityId) -> &mut Kingdom {
            self.insert_city(city_id);

            self
        }

        fn insert_city(&mut self, city_id: CityId) -> &mut City {
            self.cities.entry(city_id).or_insert(City::new())
        }

        pub fn solve(&self, query: Vec<CityId>) -> CityIdSized {
            match query.len() {
                1 => 1,
                _ => 42,
            }
        }
    }
}


// Parses input data and solves each query.
fn main() {
    use kingdom::*;
    use std::io;

    let mut domki = Kingdom::new();

    let number_of_cities: CityIdSized = {
        let mut l = String::new();
        io::stdin().read_line(&mut l).unwrap();

        l.trim().parse().unwrap()
    };

    for _ in 1..number_of_cities {
        let mut l = String::new();
        let mut road_definition = {
            io::stdin().read_line(&mut l).unwrap();

            l.split_whitespace()
             .map(|v| v.parse().unwrap())
        };

        let city_id_1: CityId = road_definition.next().unwrap();
        let city_id_2: CityId = road_definition.next().unwrap();

        if let Some(1) = road_definition.next() {
            domki.add_road(city_id_1, city_id_2);
        } else {
            domki.add_city(city_id_1)
                 .add_city(city_id_2);
        }
    }

    let number_of_queries: u32 = { // between 1 and 200_000
        let mut l = String::new();
        io::stdin().read_line(&mut l).unwrap();

        l.trim().parse().unwrap()
    };

    for _ in 0..number_of_queries {
        let query: Vec<CityId> = {
            let mut l = String::new();
            io::stdin().read_line(&mut l).unwrap();

            l.split_whitespace()
             .skip(1) // first number on each line is useless
             .map(|v| v.parse().unwrap())
             .collect()
        };

        println!("{}", domki.solve(query));
    }
}
