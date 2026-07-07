use std::f64::consts::E;

fn attacker_success_prob(q: f64, z: i32) -> f64 {
    let p = 1.0 - q;
    let lambda = z as f64 * (q / p);
    let mut sum = 1.0;
    for k in 0..=z {
        let mut poisson = E.powf(-lambda);
        for i in 1..=k {
            poisson *= lambda / i as f64;
        }
        sum -= poisson * (1.0 - (q / p).powi(z - k));
    }
    sum
}

fn main() {
    let prob = attacker_success_prob(0.1, 0);

    println!("THe Prob is  {}", prob);
}
