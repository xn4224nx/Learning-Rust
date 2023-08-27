use mandelbrot::*;
use std::env;


fn main() {
    
    /* Collect the program arguments. */
    let args: Vec<String> = env::args().collect();

    /* If not enough arguments are given print the correct usage. */
    if args.len() != 6 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT MULTITHREADED", 
            args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20 T",
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
    if args[5] == "T" {
    
        let threads = 8;
        let rows_per_band = bounds.1 / threads + 1;
        
        {
            let bands: Vec<&mut [u8]> = pixels.chunks_mut(
                rows_per_band * bounds.0).collect();
            
            crossbeam::scope(|spawner| {
                for (i, band) in bands.into_iter().enumerate() {
                    let top = rows_per_band * i;
                    let height = band.len() /bounds.0;
                    let band_bounds = (bounds.0, height);
                    
                    let band_upper_left = pixel_point_to_complex(bounds, (0, top), 
                        upper_left, lower_right);
                    
                    let band_lower_right = pixel_point_to_complex(bounds, 
                        (bounds.0, top + height), upper_left, lower_right);
                    
                    spawner.spawn(move |_| {
                        render(band, band_bounds, band_upper_left, band_lower_right);
                    });
                }
            }).unwrap();
        }
    } else {
        render(&mut pixels, bounds, upper_left, lower_right);
    }
    
    /* Save the pixel array as an image. */
    write_img(&args[1], &pixels, bounds).expect("Error writing PNG file.");
}



































