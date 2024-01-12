/* Q7 is a tuple stuct whose fields are not meant to be accessed. */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q7(i8);

impl From<f64> for Q7 {
    fn from(n: f64) -> Self {
        if n >= 1.0 {
            return Q7(127);
        } else if n <= -1.0 {
            return Q7(-128);
        } else {
            return Q7((n * 128.0) as i8);
        }
    }
}

impl From<f32> for Q7 {
    fn from(n: f32) -> Self {
        return Q7::from(n as f64);
    }
}

impl From<Q7> for f64 {
    fn from(n: Q7) -> f64 {
        return (n.0 as f64) * 2_f64.powf(-7.0);
    }
}

impl From<Q7> for f32 {
    fn from(n: Q7) -> f32 {
        return f64::from(n) as f32;
    }
}

fn main() {
    let test_val = Q7(40);
    let test_val2: f64 = 40.0;

    println!("{:?} {}", test_val, f64::from(test_val));
    println!("{} {:?}", test_val2, Q7::from(test_val2));
}
