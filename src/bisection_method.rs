pub fn bisection(f: &dyn Fn(f64) -> f64, mut a: f64, mut b: f64, tolerance: f64) -> f64 {
  if f(a)*f(b) >= 0.0 { 
    panic!("There might be several, or no, roots. I don't know what to do.");
  }
  while (b-a)/2.0 > tolerance {
    let c: f64 = (a+b)/2.0;
    if f(c) == 0.0 {
      return c;
    }
    if f(a)*f(c) < 0.0 { 
      b = c; 
    } else { 
      a = c; 
    }
  }
  return (a+b)/2.0;
}