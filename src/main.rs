use std::ops::{Add, Mul, Sub};
use fractals::run;

#[derive(Clone, Copy, Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }
}

impl<T> Add<T> for Complex
where
    T: Into<Complex>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::new(self.real + rhs.real, self.imag + rhs.imag)
    }
}

impl<T> Sub<T> for Complex
where
    T: Into<Complex>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::new(self.real - rhs.real, self.imag - rhs.imag)
    }
}

impl<T> Mul<T> for Complex
where
    T: Into<Complex>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::new(
            self.real * rhs.real - self.imag * rhs.imag,
            self.real * rhs.imag + self.imag * rhs.real,
        )
    }
}

impl From<f64> for Complex {
    fn from(value: f64) -> Self {
        Self::new(value, 0.)
    }
}

fn f(x: Complex, c: Complex) -> Complex {
    x * x + c
}

fn steps_out(c: Complex, max_steps: usize) -> usize {
    let mut x = Complex::new(0., 0.);
    let out_of_bounds = |x: Complex| x.real > 100. || x.imag > 100.;

    let mut steps = 0;
    while !out_of_bounds(x) && steps < max_steps {
        x = f(x, c);
        steps += 1;
    }
    steps
}

fn is_divergent(c: Complex, max_steps: usize) -> bool {
    let mut x = Complex::new(0., 0.);
    let out_of_bounds = |x: Complex| x.real > 100. || x.imag > 100.;

    for _ in 0..max_steps {
        if out_of_bounds(x) {
            return true;
        }
        x = f(x, c);
    }
    false
}

fn main() {
    // println!("{:?}", is_divergent(Complex::new(0.2505, 0.), 200));
    // println!("{:?}", is_divergent(Complex::new(0.250, 0.), 200));
    pollster::block_on(run());
}
