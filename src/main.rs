mod bisection_method;
use bisection_method::bisection;

mod fpi;
use fpi::fpi;

fn main() {
    let f1 = | x: f64 | x.powi(3) + x - 1.0;
    let g1 = | x: f64 | (1.0 + 2.0*x.powi(3)) / (1.0 + 3.0*x.powi(2));
    println!("root is {}", bisection(&f1, 0.0, 1.0, 0.000000001));
    println!("root is {}", fpi(&g1, 0.0, 10));
}
