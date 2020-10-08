// File to be submitted to spoj.com.


mod kingdom {
    #[allow(non_camel_case_types)]
    pub type size_k = usize;
    pub type CityId = size_k;

    // A city is a collection of links to other cities.
    pub type City = Vec<CityId>;

    // A kingdom is a collection of cities.
    pub struct Kingdom {
        cities: Vec<City>
    }

    impl Kingdom {
        // Preallocates memory for up to 200_000 cities.
        // TODO: replace with "transmute" code.
        pub fn new(number_of_cities: size_k) -> Kingdom {
            let mut cities = Vec::new();
            for _ in 0..number_of_cities {
                cities.push(Vec::new());
            }

            Kingdom {cities}
        }

        // Adds a two-way link between two cities.
        pub fn link(&mut self, mut city_id_1: CityId, mut city_id_2: CityId) {
            city_id_1 -= 1;
            city_id_2 -= 1;
            self.cities[city_id_1].push(city_id_2);
            self.cities[city_id_2].push(city_id_1);
        }

        // Returns the answer for a query.
        pub fn solve(&self, mut query: Vec<CityId>) -> size_k {
            if query.len() == 1 {return 1;}

            for city_id in &mut query {*city_id -= 1}

            42
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
