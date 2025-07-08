use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    let mut count = 0.0;
    let num_iter = 1000;
    for _ in 0..num_iter {
        let x = rng.random_range(0.0..1.0);
        let y = rng.random_range(0.0..1.0);
        if x * x + y * y < 1.0 {
            count += 1.0;
        }
    }
    println!("{}", count * 4.0 / num_iter as f64);
}
