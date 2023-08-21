use mandelbrot::escape_time;
use num::Complex;

fn main() {
    println!("{:?}", escape_time(Complex { re: 0.0, im: 0.0 }, 20));
}
