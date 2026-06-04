use rand::RngExt;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

struct Gen {
    x: u8,
    y: u8,
}

impl Gen {
    fn get_distance(&self, point: &Gen) -> f64 {
        let dx = (point.x as f64) - (self.x as f64);
        let dy = (point.y as f64) - (self.y as f64);

        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, Copy)]
struct Population {
    chormosomes: [Chromosome; 100],
}

impl Population {
    fn generate_random(points: &HashMap<u8, Gen>) -> Population {
        Population {
            chormosomes: [(); 100].map(|_| Chromosome::generate_random(points)),
        }
    }

    fn get_five_percent_random(&self) -> [&Chromosome; 5] {
        let mut rng = rand::rng();

        [(); 5].map(|_| &self.chormosomes[rng.random_range(1..100)])
    }

    fn tournament(&self, points: &HashMap<u8, Gen>) -> Population {
        let child_population: [Chromosome; 100] = [(); 100].map(|_| {
            let choosen: [&Chromosome; 5] = self.get_five_percent_random();

            let current_best = choosen
                .iter()
                .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

            let best = *current_best.copied().unwrap();

            best.reproduction(points);

            match best.log_route(points) {
                Ok(_) => println!("Success written"),
                Err(e) => eprintln!("Failed: {}", e),
            }

            best
        });

        Population {
            chormosomes: child_population,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Chromosome {
    gens: [u8; 20],
    distance: f64,
}

impl Chromosome {
    fn generate_random(points: &HashMap<u8, Gen>) -> Chromosome {
        let mut rng = rand::rng();

        let mut chromosome = Chromosome {
            gens: [(); 20].map(|_| rng.random_range(1..20)),
            distance: 0.0,
        };

        let distance: f64 = chromosome.calculate_gen_distances(points);

        chromosome.distance = distance;

        chromosome
    }

    fn calculate_gen_distances(self, points: &HashMap<u8, Gen>) -> f64 {
        let mut row_distance = 0.0;
        let mut last_gen: Option<&Gen> = None;

        for rgen in self.gens {
            if let Some(current_gen) = points.get(&rgen) {
                if let Some(prev) = last_gen {
                    row_distance += prev.get_distance(current_gen);
                }

                last_gen = Some(current_gen);
            }
        }

        row_distance
    }

    fn reproduction(mut self, points: &HashMap<u8, Gen>) {
        let mut rng = rand::rng();

        if rng.random_bool(0.5) {
            //"Swap algorithm"

            // 1. Get random start point
            let start = rng.random_range(0..20);

            // 2. Get random length that fits remaining space
            let max_len = 20 - start;
            if max_len <= 1 {
                return;
            } // Too small to swap within itself
            let len = rng.random_range(1..=max_len);

            // 3. Find another non-overlapping spot to swap with, or swap with neighbor
            // Genetic algorithm mutation style: reverse the chosen segment
            let mut left = start;
            let mut right = start + len - 1;

            while left < right {
                self.gens.swap(left, right);
                left += 1;
                right -= 1;
            }
        } else {
            // 1. Pick random start point
            let start = rng.random_range(0..20);

            // 2. Pick random length that fits remaining space
            let max_len = 20 - start;
            if max_len <= 1 {
                return;
            } // Need at least 2 elements to invert
            let len = rng.random_range(2..=max_len);

            // 3. Invert the selected slice inside bounds
            let end = start + len;
            let slice = &mut self.gens[start..end];
            slice.reverse();
        }

        self.distance = self.calculate_gen_distances(points);
    }

    fn log_route(self, points: &HashMap<u8, Gen>) -> std::io::Result<()> {
        let mut points_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("/points.csv")?;

        let mut distance_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("distance.csv")?;

        for rgen in self.gens {
            if let Some(current_gen) = points.get(&rgen) {
                writeln!(points_file, "{},{}", current_gen.x, current_gen.y)?;
            }
        }

        writeln!(distance_file, "{}", self.distance)?;
        Ok(())
    }
}

fn main() {
    let mut point_map: HashMap<u8, Gen> = HashMap::new();

    point_map.insert(1, Gen { x: 1, y: 3 });
    point_map.insert(2, Gen { x: 2, y: 5 });
    point_map.insert(3, Gen { x: 2, y: 7 });
    point_map.insert(4, Gen { x: 4, y: 2 });
    point_map.insert(5, Gen { x: 4, y: 4 });
    point_map.insert(6, Gen { x: 4, y: 7 });
    point_map.insert(7, Gen { x: 4, y: 8 });
    point_map.insert(8, Gen { x: 5, y: 3 });
    point_map.insert(9, Gen { x: 6, y: 1 });
    point_map.insert(10, Gen { x: 6, y: 6 });
    point_map.insert(11, Gen { x: 7, y: 8 });
    point_map.insert(12, Gen { x: 8, y: 2 });
    point_map.insert(13, Gen { x: 8, y: 7 });
    point_map.insert(14, Gen { x: 9, y: 3 });
    point_map.insert(15, Gen { x: 10, y: 7 });
    point_map.insert(16, Gen { x: 11, y: 1 });
    point_map.insert(17, Gen { x: 11, y: 4 });
    point_map.insert(18, Gen { x: 11, y: 6 });
    point_map.insert(19, Gen { x: 12, y: 7 });
    point_map.insert(20, Gen { x: 13, y: 5 });

    let asd = Population::generate_random(&point_map);

    asd.tournament(&point_map);

    // println!("{:?}", childs)
}
