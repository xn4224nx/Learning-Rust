use rand::random;

fn mock_rand(n: u8) -> f32 {
	return (n as f32) / 255.0;
}

fn main() {
    let sourced = random::<u8>();
    let prob = mock_rand(sourced);
    
    println!("u8 = {}, f32 = {}", sourced, prob);
}
