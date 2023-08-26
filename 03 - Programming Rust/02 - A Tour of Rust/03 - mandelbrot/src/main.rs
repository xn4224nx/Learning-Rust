use mandelbrot::*;
use std::env;

fn main() {
    /* Collect the program arguments. */
    let args: Vec<String> = env::args().collect();

    /* If not enough arguments are given print the correct usage. */
    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        std::process::exit(1)
    }

    /* Extract the image resolution, upper_left and lower_right. */
    let bounds = match parse_img_res(&args[2], 'x') {
        Some((width, height)) => (width, height),
        None => panic!("Can't parse: {} with {}", &args[2], 'x')
    };
    
    let upper_left = match parse_complex_res(&args[3]) {
        Some(complex) => complex,
        None => panic!("Can't parse: {}", &args[3])
    };
    
    let lower_right= match parse_complex_res(&args[4]) {
        Some(complex) => complex,
        None => panic!("Can't parse: {}", &args[4])
    };
    

    /* Create the pixel array with all zeros. */
    let mut pixels = vec![0; bounds.0 * bounds.1];

    /* Fill the pixel array. */
    render(&mut pixels, bounds, upper_left, lower_right);

    /* Save the pixel array as an image. */
    write_img(&args[1], &pixels, bounds).expect("Error writing PNG file.");
}
