use num::Complex;


fn main() {
    println!("{:?}", escape_time(Complex{re: 0.0, im: 0.0}, 20));
}


/// Determine if `c` is in the Mandelbrot set. 
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
   
    let mut z = Complex {re: 0.0, im: 0.0};
    
    for i in 0..limit {
        
        /* Return the number of iterations that it took to prove it wasn't in */
        if z.norm_sqr() > 4.0 {return Some(i);}
        z = z * z + c;
    }
    
    /* If it can't be proved to be outside the set. */
    return None;
}
