//! 2.3 期待値と積分 (p.30-31)
//!
//! 半径が１の3次元旧の体積V3を計算します
//!
//! 高さ z = \sqrt(1 - x^2 - y^2) を x > 0, y > 0, x^2 + y^2 < 1 の範囲で平均値を求めると V3/8が得られるので
//! V3 = 2π x <\sqrt(1 - x^2 - y^2)>

use rand::Rng;
use std::f64::consts::PI;

fn main() {
    let mut rng = rand::rng();
    let num_iter = 10000;

    let mut sum_z = 0.0;
    let mut num_in = 0_usize;
    for _ in 0..num_iter {
        let x = rng.random_range(0.0..1.0);
        let y = rng.random_range(0.0..1.0);
        if x * x + y * y < 1.0 {
            num_in += 1;
            sum_z += (1_f64 - x * x - y * y).sqrt();
        }
    }

    // V3 = 2π <z>
    println!("Expected value = {}", 2.0 * PI * sum_z / num_in as f64);
    // Exact value = 4π/3
    println!("Exact          = {}", 4.0 * PI / 3.0);
}
