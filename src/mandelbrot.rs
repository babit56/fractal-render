use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
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

pub struct FractalState {
    pub bounds: ((f64, f64), (f64, f64)),
    pub max_steps: usize,
    pub func: fn(Complex) -> Complex,
}

impl FractalState {
    pub fn steps_out(&self, c: Complex) -> usize {
        let mut z = Complex::new(0., 0.);
        let (xlim, ylim) = self.bounds;
        let out_of_bounds = |z: Complex| z.real > xlim.1 || z.real < xlim.0 || z.imag > ylim.1 || z.imag < ylim.0;

        let mut steps = 0;
        while !out_of_bounds(z) && steps < self.max_steps {
            z = z*z + c;
            steps += 1;
        }
        steps
    }

    pub fn create_bytes(&self, points: usize) -> Vec<usize> {
        // (0..points).map(|y| (0..points))
    }
}
