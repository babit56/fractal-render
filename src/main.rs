use fractals::{run, create_image};

fn main() {
    // println!("{:?}", is_divergent(Complex::new(0.2505, 0.), 200));
    // println!("{:?}", is_divergent(Complex::new(0.250, 0.), 200));
    let state = fractals::FractalState {
        bounds: ((-2., 1.), (-1., 1.)),
        max_steps: 255,
    };
    // create_image(state);
    pollster::block_on(run());
}
