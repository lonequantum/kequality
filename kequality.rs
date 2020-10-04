// spoj.com requires the solution to be one file only,
// so we are not using any external definition.


mod kingdom {
    use std::collections::HashMap;
    use self::kingdom_browser::*;

    #[allow(non_camel_case_types)]
    pub type size_k = usize;
    pub type CityId = size_k;
        type TreeId = size_k;

    // A road is a one-way link between two cities and is owned by the city it starts from.
    // It holds the number of cities that are reachable by taking it.
    pub struct Road {
        reachable_cities_count: size_k
    }

    // A city is a node in a tree.
    // It's a list of roads that start from it, plus an identifier of the tree the city belongs to.
    pub struct City {
        id: CityId, // convenient, but redundant with cities indexation at the kingdom level
        tree_id: TreeId,
        roads_to: HashMap<CityId, Road>
    }

    impl City {
        // Creates or updates a road, basing on destination's data.
        fn auto_update_road_to(&mut self, destination: &City) -> size_k {
            let count = destination.count_reachable_cities(self.id) + 1;
            self.roads_to.insert(
                destination.id,
                Road {
                    reachable_cities_count: count
                }
            );

            count
        }

        // Updates a road with the given increase value.
        fn update_road_to(&mut self, dest_id: CityId, increase_value: size_k) {
            self.roads_to.entry(dest_id).and_modify(
                |road| road.reachable_cities_count += increase_value
            );
        }

        // Takes all roads (but not the excluded one) from our city and gets values from immediate next cities,
        // so we have the total number of cities in the kingdom that are reachable from our city.
        fn count_reachable_cities(&self, exclude: CityId) -> size_k {
            let mut count = 0;
            let mut ri = RoadIterator::new(&self, exclude);
            while let Some((_, road)) = ri.next() {
                count += road.reachable_cities_count;
            }

            count
        }
    }

    // A kingdom is a set of trees.
    pub struct Kingdom {
        cities: Vec<City> // indexes all nodes of all trees
    }

    impl Kingdom {
        // Preallocates memory for up to 200_000 cities.
        // TODO: replace with "transmute" code.
        pub fn new(number_of_cities: size_k) -> Kingdom {
            let mut cities = Vec::new();
            for i in 0..number_of_cities {
                cities.push(
                    City {
                        id: i + 1,
                        tree_id: i,
                        roads_to: HashMap::new()
                    }
                );
            }

            Kingdom {
                cities
            }
        }

        // Returns mutable references to 2 cities in the same vector.
        fn select_cities_mut(&mut self, city_id_1: CityId, city_id_2: CityId) -> (&mut City, &mut City) {
            if city_id_1 > city_id_2 { // probably useless (if we trust input data) but who knows?
                return self.select_cities_mut(city_id_2, city_id_1);
            }

            let (part_1, part_2) = self.cities.split_at_mut(city_id_1); // should be safe, since the vector's length never changes
            let part_1_len = part_1.len();

            (&mut part_1[city_id_1 - 1],
             &mut part_2[city_id_2 - 1 - part_1_len])
        }

        // Adds a two-way link between two cities
        pub fn link(&mut self, city_id_left: CityId, city_id_right: CityId) {
            let (count_right, count_left) = {
                let (city_left, city_right) = self.select_cities_mut(city_id_left, city_id_right);

                (city_left.auto_update_road_to(city_right),
                 city_right.auto_update_road_to(city_left))
            };

            // TODO
        }

        // Returns the answer for a query.
        pub fn solve(&self, query: Vec<CityId>) -> size_k {
            match query.len() {
                1 => 1,
                _ => 42 // TODO: remove placeholder and compute real solution ^^
            }
        }
    }


    mod kingdom_browser {
        use super::*;

        pub struct RoadIterator<'a> {
            owner_city_id: CityId,
            exclude_dest_id: CityId,
            iterator: std::collections::hash_map::Iter<'a, CityId, Road>
        }

        impl RoadIterator<'_> {
            pub fn new(city: &City, exclude_dest_id: CityId) -> RoadIterator {
                RoadIterator {
                    owner_city_id: city.id,
                    exclude_dest_id,
                    iterator: city.roads_to.iter()
                }
            }

            pub fn next(&mut self) -> Option<(&CityId, &Road)> {
                let ret = self.iterator.next();
                if let Some((&road_to, _)) = ret {
                    if road_to == self.exclude_dest_id {
                        return self.next();
                    }
                }

                ret
            }
        }

        // Another convenient way to define a road.
        // It's the PointingTreeBrowser's I/O format.
        pub struct Link {
            from: CityId,
            to: CityId
        }

        pub struct PointingTreeBrowser<'a> {
            kingdom: &'a Kingdom,
            current_chain: Vec<RoadIterator<'a>>
        }

        impl PointingTreeBrowser<'_> {
            pub fn new(kingdom: &Kingdom, start_link: Link) -> PointingTreeBrowser {
                PointingTreeBrowser {
                    kingdom,
                    current_chain: {
                        vec![
                            RoadIterator::new(&kingdom.cities[start_link.to - 1], 0),
                            RoadIterator::new(&kingdom.cities[start_link.from - 1], start_link.to - 1)
                        ]
                    }
                }
            }
        }

        impl Iterator for PointingTreeBrowser<'_> {
            type Item = Link;

            fn next(&mut self) -> Option<Self::Item> {
                let mut len = self.current_chain.len();
                if len == 1 {
                    None
                } else {
                    let current_from = &mut self.current_chain[len - 1];
                    let current_from_owner_id = current_from.owner_city_id;

                    if let Some ((&road_to, _)) = current_from.next() {
                        self.current_chain.push(
                            RoadIterator::new(&self.kingdom.cities[road_to - 1], current_from_owner_id)
                        );
                    } else {
                        len -= 1;
                        self.current_chain.truncate(len);
                        if len == 1 {
                            return None;
                        }
                    }

                    let current_from = &self.current_chain[len - 1];
                    let current_to   = &self.current_chain[len - 2];

                    Some(Link{
                        from: current_from.owner_city_id,
                        to: current_to.owner_city_id
                    })
                }
            }
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
        
        if let Some(1) = road_definition.next() {
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
