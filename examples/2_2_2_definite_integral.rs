//! 2.2.2 一様乱数を用いた定積分 (p.23-24)
//!
//! 区間 [0, 1]の一葉乱数xを生成し、f(x) = \sqrt(1-x^2) の平均値を求める

use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    let num_iter = 10000;

    let mut sum_y = 0.0;
    for _ in 0..num_iter {
        let x = rng.random_range(0.0..1.0);
        let y = (1.0_f64 - x * x).sqrt();
        sum_y += y;
    }
    println!("{}", sum_y / num_iter as f64);
}
