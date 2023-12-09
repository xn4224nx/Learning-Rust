use num::complex::Complex;

fn calculate_mandelbrot (
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {

    /* Create a container to house the data from all rows. */
    let mut all_rows: Vec<Vec<usize>> = Vec::with_capacity(width);
    for y in 0..height {
        
        /* Create a container to house the data from one row.*/   
        let mut row: Vec<usize> = Vec::with_capacity(height);
        for x in 0..width {
            
            /* Calculate the proportion of space covered in the output. */
            let x_perc = (x as f64) / (width as f64);
            let y_perc = (y as f64) / (height as f64);
            
            /* Convert the proportion to coordinates in the search space. */
            let cx = x_min + (x_max - x_min) * x_perc;
            let cy = y_min + (y_max - y_min) * y_perc;
            
            /* Calculate the escape value. */
            let escaped_at = mandelbrot_at_pnt(cx, cy, max_iters);
            row.push(escaped_at);
        }
        all_rows.push(row);
    }
    return all_rows;
}

fn mandelbrot_at_pnt(
    cx: f64,
    cy: f64,
    max_iters: usize,
) -> usize {

    /* Initalise two complex numbers, one at the origin. */
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);
    
    for i in 0..=max_iters {
        /* If the distance from the origin is greater that two. */
        if z.norm() > 2.0 {
            return i;
        }   
        /* Mutate z to see if its in the mandlebrot set. */
        z = z * z + c;    
    }
    /* If it never escapes return the max iterations */
    return max_iters;
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {

    for row in escape_vals {
    
    let mut line = String::with_capacity(row.len());
        for col in row {
            let val = match col {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '*',
                11..=30 => '+',
                31..=100 => 'x',
                101..=200 => '£',
                201..=400 => '@',
                401..=700 => '#',
                _ => '%'
            };
            
            line.push(val);
        }
        println!("{}", line);
    }
}

fn main() {
    
    let mandelbrot = calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, 100, 24);

    render_mandelbrot(mandelbrot);
}
