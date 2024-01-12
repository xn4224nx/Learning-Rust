use q7_type::Q7;

fn main() {
    let test_val = Q7(40);
    let test_val2: f64 = 40.0;

    println!("{:?} {}", test_val, f64::from(test_val));
    println!("{} {:?}", test_val2, Q7::from(test_val2));
}
