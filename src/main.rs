use rand::Rng;

#[allow(dead_code)]
fn monte_carlo() -> f64 {
    let mut rng = rand::thread_rng();
    let total_tries = 1_000_000;
    let mut num_sucesses = 0;

    for _ in 0..total_tries {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        if (x.powi(2) + y.powi(2)) < 1.0 {
            num_sucesses = num_sucesses + 1;
        }
    }

    return (num_sucesses as f64) / (total_tries as f64) * 4.0;
}

#[allow(dead_code)]
fn riemann(num_rects: i32) -> f64 {
    let mut area = 0.;
    let dx = 1.0 / (num_rects as f64);

    for i in 0..num_rects {
        let x = (i as f64) / (num_rects as f64) + dx / 2.0;
        let y = (1.0 - x.powi(2)).sqrt();

        area = area + y * dx;
    }

    return area * 4.0;
}

fn main() {
    println!("pi estimate: {}", riemann(100_000));
}
