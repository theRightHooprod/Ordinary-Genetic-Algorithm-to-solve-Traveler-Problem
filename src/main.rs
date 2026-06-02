struct Point {
    x: u8,
    y: u8,
}

// struct FixedGrid {
//     matrix: [[Point; 20]; 100],
// }

impl Point {
    fn get_distance(&self, point: &Point) -> f64 {
        (((&point.x - &self.x).pow(2) + (&point.y - &self.y).pow(2)) as f64).sqrt()
    }
}

fn main() {
    let prim = Point { x: 5, y: 10 };

    let hola = prim.get_distance(&Point { x: 15, y: 20 });

    println!("{}", &hola);
}
