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
        id: CityId,
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

        // Adds a two-way link between two cities.
        // Merges trees and updates all roads.
        pub fn link(&mut self, city_id_left: CityId, city_id_right: CityId) {
            let (count_right, count_left, tree_id_left) = {
                let (city_left, city_right) = self.select_cities_mut(city_id_left, city_id_right);

                city_right.tree_id = city_left.tree_id;

                (city_left.auto_update_road_to(city_right),
                 city_right.auto_update_road_to(city_left),
                 city_left.tree_id)
            };

            let pointing_tree_left: Vec<Link> = PointingTreeBrowser::new(&self, Link {
                from: city_id_left,
                to: city_id_right
            })
            .collect();

            let pointing_tree_right: Vec<Link> = PointingTreeBrowser::new(&self, Link {
                from: city_id_right,
                to: city_id_left
            })
            .collect();

            for link in pointing_tree_left {
                self.cities[link.from - 1].update_road_to(link.to, count_right);
            }

            for link in pointing_tree_right {
                let city = &mut self.cities[link.from - 1];
                city.tree_id = tree_id_left;
                city.update_road_to(link.to, count_left);
            }
        }

        // Returns the answer for a query.
        pub fn solve(&self, query: Vec<CityId>) -> size_k {
            if query.len() == 1 {
                1
            } else {
                // Quick checking: is every city reachable by others?
                let first_city = &self.cities[query[0] - 1];
                for city_id in &query[1..] {
                    if self.cities[city_id - 1].tree_id != first_city.tree_id {return 0;} // not the same tree
                }

                let paths: Vec<Vec<CityId>> = Vec::new();
                for (i, city_id_from) in query.iter().enumerate() {
                    for city_id_to in &query[(i + 1)..] {
                        // TODO: find the longest path
                    }
                }

                42
            }
        }
    }


    mod kingdom_browser {
        use super::*;

        // To iterate over roads that start from a city.
        // Typically we want one road to be ignored.
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
                let mut ret = self.iterator.next();
                if let Some((&road_to, _)) = ret {
                    if road_to == self.exclude_dest_id {
                        ret = self.iterator.next();
                    }
                }

                ret
            }
        }

        // Another convenient way to define a road.
        // It's the PointingTreeBrowser's I/O format.
        pub struct Link {
            pub from: CityId,
            pub to: CityId
        }

        // To iterate over a (sub)tree behind a particular city/road.
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
                            RoadIterator::new(&kingdom.cities[start_link.from - 1], start_link.to)
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
                        len += 1;
                        self.current_chain.push(
                            RoadIterator::new(&self.kingdom.cities[road_to - 1], current_from_owner_id)
                        );
                    } else {
                        len -= 1;
                        self.current_chain.truncate(len);
                        return self.next();
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

        print!("{:?} -> ", query); // to be removed before submitting to spoj.com
        println!("{}", domki.solve(query));
    }
}
