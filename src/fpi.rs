pub fn fpi(f: &dyn Fn(f64) -> f64, mut x: f64, k: i32) -> f64 {
  for _ in 0..k {
    x = f(x);
  }
  return f(x);
}