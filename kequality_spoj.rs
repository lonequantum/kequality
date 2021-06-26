// File to be submitted to spoj.com.


mod kingdom {
    use std::collections::HashMap;
    use std::cmp::Ordering;

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

    impl City {
        fn parent_id(&self) -> CityId {
            self.roads[0].destination
        }

        // Augments vec_dst with new elements found in vec_src.
        // I guess this signature is not idiomatic in the rust world, perhaps I should use slices
        fn union_vec_ids(vec_dst: &mut Vec<CityId>, vec_src: &Vec<CityId>) {
            for element in vec_src {
                if !vec_dst.contains(element) {
                    vec_dst.push(*element);
                }
            }
        }
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
        trees_sizes: HashMap<TreeId, size_k>
    }

    impl Kingdom {
        // Preallocates memory for up to 200_000 cities.
        // TODO: replace with "transmute" code.
        pub fn new(number_of_cities: size_k) -> Kingdom {
            let mut cities = Vec::new();

            for tree_id in 0..number_of_cities {
                cities.push(City{
                    tree_id,
                    depth: 0,
                    roads: Vec::new()
                });
            }

            Kingdom {
                cities,
                trees_sizes: HashMap::new()
            }
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

            let mut current_meeting_point;
            let mut merged_meeting_point = MeetingPoint {
                city_id: 0,
                traveled_distance: 0,
                dont_go_back_to: Vec::new()
            };

            for (i, &id_i) in query.iter().enumerate() {
                let depth_i = self.cities[id_i].depth;

                for &id_j in &query[(i + 1)..] {
                    let depth_j = self.cities[id_j].depth;

                    if depth_i != depth_j {
                        if (depth_i + depth_j) % 2 == 1 {
                            return 0; // distance between two queried cities is an odd number of roads
                        }

                        current_meeting_point = if depth_i < depth_j {
                            self.find_meeting_point_from_two_depths(id_j, id_i, depth_j, depth_i)
                        } else {
                            self.find_meeting_point_from_two_depths(id_i, id_j, depth_i, depth_j)
                        };
                    } else {
                        current_meeting_point = self.find_meeting_point_from_one_depth(id_i, id_j);
                    }

                    // Merges results to determine the final meeting point.

                    if merged_meeting_point.traveled_distance == 0 { // first iteration
                        merged_meeting_point = MeetingPoint {
                            ..current_meeting_point
                        };

                        continue;
                    }

                    if merged_meeting_point.city_id == current_meeting_point.city_id {
                        if merged_meeting_point.traveled_distance != current_meeting_point.traveled_distance {
                            return 0; // the MP involves different traveled distances
                        } else {
                            City::union_vec_ids(&mut merged_meeting_point.dont_go_back_to, &current_meeting_point.dont_go_back_to);
                        }
                    } else {
                        // The most complex part.

                        let merged_city_id  = merged_meeting_point.city_id;
                        let current_city_id = current_meeting_point.city_id;
                        let merged_depth  = self.cities[merged_city_id].depth;
                        let current_depth = self.cities[current_city_id].depth;
                        let merged_traveled  = merged_meeting_point.traveled_distance;
                        let current_traveled = current_meeting_point.traveled_distance;

                        let path = self.collect_cities_between(merged_city_id, current_city_id, merged_depth, current_depth);
                        let last_index = path.len() - 1;

                        if merged_traveled > (last_index + current_traveled) {
                            return 0;
                        }
                        let mut pos = last_index + current_traveled - merged_traveled;
                        if pos % 2 != 0 {
                            return 0;
                        }
                        pos /= 2;
                        if pos > last_index {
                            return 0;
                        }

                        if          pos > 0 && merged_meeting_point.dont_go_back_to.contains(&path[1])
                        || pos < last_index && current_meeting_point.dont_go_back_to.contains(&path[last_index - 1]) {
                            return 0; // at least one people would move to its inital MP then go backward to join another MP
                        }

                        merged_meeting_point = MeetingPoint {
                            city_id: path[pos],
                            traveled_distance: merged_traveled + pos,
                            dont_go_back_to: {
                                match pos {
                                    0 => {
                                        City::union_vec_ids(&mut merged_meeting_point.dont_go_back_to, &vec![path[1]]); // maybe not very idiomatic; path[1] is copied
                                        merged_meeting_point.dont_go_back_to
                                    },

                                    v if v == last_index => {
                                        City::union_vec_ids(&mut current_meeting_point.dont_go_back_to, &vec![path[last_index - 1]]);
                                        current_meeting_point.dont_go_back_to
                                    },

                                    _ => vec![path[pos - 1], path[pos + 1]]
                                }
                            }
                        };
                    }
                }
            }

            42
        }

        // Finds cities along the shortest path from city 1 to city 2 (included).
        fn collect_cities_between(&self, mut city_id_1: CityId, mut city_id_2: CityId,
                                      mut city_depth_1: size_k, mut city_depth_2: size_k) -> Vec<CityId> {
            let mut ret = vec![city_id_1];
            let mut ter = vec![city_id_2];

            match city_depth_1.cmp(&city_depth_2) {
                Ordering::Equal => {}

                Ordering::Less => {
                    while city_depth_2 > city_depth_1 {
                        city_id_2 = self.cities[city_id_2].parent_id();
                        city_depth_2 -= 1;
                        ter.push(city_id_2);
                    }
                    if city_id_1 == city_id_2 {
                        ter.pop();
                    }
                }

                Ordering::Greater => {
                    while city_depth_1 > city_depth_2 {
                        city_id_1 = self.cities[city_id_1].parent_id();
                        city_depth_1 -= 1;
                        ret.push(city_id_1);
                    }
                    if city_id_1 == city_id_2 {
                        ret.pop();
                    }
                }
            }

            if city_id_1 != city_id_2 {
                while city_id_1 != city_id_2 {
                    city_id_1 = self.cities[city_id_1].parent_id();
                    ret.push(city_id_1);
                    city_id_2 = self.cities[city_id_2].parent_id();
                    ter.push(city_id_2);
                }
                ter.pop();
            }

            ter.reverse();
            ret.append(&mut ter);

            ret
        }

        // Finds the halfway city between two cities that don't have the same depth.
        fn find_meeting_point_from_two_depths(&self, deepest_city_id: CityId, shallowest_city_id: CityId,
                                                  deepest_city_depth: size_k, shallowest_city_depth: size_k) -> MeetingPoint {
            let depth_diff = deepest_city_depth - shallowest_city_depth;
            let traveled_distance;

            let mut city_id = deepest_city_id;
            for _ in 0..depth_diff {
                city_id = self.cities[city_id].parent_id();
            }

            if city_id != shallowest_city_id {
                let mp = self.find_meeting_point_from_one_depth(city_id, shallowest_city_id);
                traveled_distance = depth_diff / 2 + mp.traveled_distance;
            } else {
                traveled_distance = depth_diff / 2;
            }

            city_id = deepest_city_id;
            let mut i = traveled_distance;
            while i > 1 {
                city_id = self.cities[city_id].parent_id();
                i -= 1;
            }

            let child_city_id = city_id;
            city_id = self.cities[city_id].parent_id();

            MeetingPoint {
                city_id,
                traveled_distance,
                dont_go_back_to: vec![child_city_id, self.cities[city_id].parent_id()]
            }
        }

        // Finds the city that is the common ancestor of two cities that have the same depth.
        fn find_meeting_point_from_one_depth(&self, mut city_id_1: CityId, mut city_id_2: CityId) -> MeetingPoint {
            let mut old_city_id_1;
            let mut old_city_id_2;

            let mut traveled_distance = 0;

            loop {
                old_city_id_1 = city_id_1;
                old_city_id_2 = city_id_2;

                city_id_1 = self.cities[city_id_1].parent_id();
                city_id_2 = self.cities[city_id_2].parent_id();
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
            *self.trees_sizes.entry(tree_id).or_insert(
                self.cities.iter()
                           .filter(|city| city.tree_id == tree_id)
                           .count()
            )
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
        match expected {
            None => println!("{}", answer),
            Some(value) if value == answer => println!("[OK ]\t{}", answer),
            Some(value) => eprintln!("[ERR]\t{}\texpected:\t{}", answer, value)
        }
    }
}
