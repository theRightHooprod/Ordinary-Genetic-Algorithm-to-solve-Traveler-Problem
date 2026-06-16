use rand::RngExt;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn get_distance(&self, point: &Point) -> f64 {
        let dx = (point.x as f64) - (self.x as f64);
        let dy = (point.y as f64) - (self.y as f64);

        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

struct Population<'a> {
    chormosomes: [Chromosome<'a>; 100],
}

impl<'a> Population<'a> {
    fn new(points: &'a HashMap<u8, Point>) -> Self {
        Self {
            chormosomes: [(); 100].map(|_| Chromosome::new(points)),
        }
    }

    fn get_five_percent_random_indices(&self) -> [usize; 5] {
        let mut rng = rand::rng();

        [(); 5].map(|_| rng.random_range(0..100))
    }

    fn best_distance(&self) -> &f64 {
        &self
            .chormosomes
            .iter()
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
            .unwrap()
            .distance
    }

    fn tournament(&mut self) {
        for _ in 0..self.chormosomes.len() {
            let random_indices = self.get_five_percent_random_indices();

            let winner_idx = *random_indices
                .iter()
                .min_by(|&&a, &&b| {
                    self.chormosomes[a]
                        .distance
                        .partial_cmp(&self.chormosomes[b].distance)
                        .unwrap()
                })
                .unwrap();

            let distance_bf_reproduction = self.chormosomes[winner_idx].distance;

            self.chormosomes[winner_idx].reproduce();

            match self.chormosomes[winner_idx].log_route(&distance_bf_reproduction) {
                Ok(_) => (),
                Err(e) => eprintln!("{}", e),
            }

            thread::sleep(Duration::from_millis(100))
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Chromosome<'a> {
    gens: [u8; 20],
    distance: f64,
    points: &'a HashMap<u8, Point>,
}

impl<'a> Chromosome<'a> {
    fn new(points: &'a HashMap<u8, Point>) -> Self {
        let mut chromosome = Chromosome {
            gens: Chromosome::generate_random(),
            distance: 0.0,
            points: points,
        };

        chromosome.distance = chromosome.calculate_gen_distances();

        chromosome
    }

    fn generate_random() -> [u8; 20] {
        let mut rng = rand::rng();

        let chromosome = [(); 20].map(|_| rng.random_range(1..20));

        chromosome
    }

    fn calculate_gen_distances(&self) -> f64 {
        let mut row_distance = 0.0;
        let mut last_gen: Option<&Point> = None;

        for rgen in &self.gens {
            if let Some(current_gen) = &self.points.get(&rgen) {
                if let Some(prev) = last_gen {
                    row_distance += prev.get_distance(current_gen);
                }

                last_gen = Some(current_gen);
            }
        }

        row_distance
    }

    fn reproduce(&mut self) {
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

        self.distance = self.calculate_gen_distances();
    }

    fn log_route(&self, prev_distance: &f64) -> std::io::Result<()> {
        let mut points_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("points.csv")?;

        let mut distance_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("distance.csv")?;

        for rgen in self.gens {
            if let Some(current_gen) = &self.points.get(&rgen) {
                writeln!(points_file, "{},{}", current_gen.x, current_gen.y)?;
            }
        }

        writeln!(distance_file, "{}, {}", prev_distance, self.distance)?;
        Ok(())
    }
}

fn main() {
    let mut point_map: HashMap<u8, Point> = HashMap::new();

    point_map.insert(1, Point { x: 1, y: 3 });
    point_map.insert(2, Point { x: 2, y: 5 });
    point_map.insert(3, Point { x: 2, y: 7 });
    point_map.insert(4, Point { x: 4, y: 2 });
    point_map.insert(5, Point { x: 4, y: 4 });
    point_map.insert(6, Point { x: 4, y: 7 });
    point_map.insert(7, Point { x: 4, y: 8 });
    point_map.insert(8, Point { x: 5, y: 3 });
    point_map.insert(9, Point { x: 6, y: 1 });
    point_map.insert(10, Point { x: 6, y: 6 });
    point_map.insert(11, Point { x: 7, y: 8 });
    point_map.insert(12, Point { x: 8, y: 2 });
    point_map.insert(13, Point { x: 8, y: 7 });
    point_map.insert(14, Point { x: 9, y: 3 });
    point_map.insert(15, Point { x: 10, y: 7 });
    point_map.insert(16, Point { x: 11, y: 1 });
    point_map.insert(17, Point { x: 11, y: 4 });
    point_map.insert(18, Point { x: 11, y: 6 });
    point_map.insert(19, Point { x: 12, y: 7 });
    point_map.insert(20, Point { x: 13, y: 5 });

    let mut asd = Population::new(&point_map);

    println!("Best distance before tournament {}", &asd.best_distance());

    asd.tournament();

    println!("Best distance after tournament {}", &asd.best_distance());
}
