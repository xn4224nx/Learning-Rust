fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
    return i + j;
}

fn main() {
    println!("{} + {} = {}", 3, 4, add(3, 4));
}
