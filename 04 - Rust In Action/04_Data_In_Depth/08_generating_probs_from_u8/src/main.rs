use rand::random;

fn mock_rand(n: u8) -> f32 {
	return (n as f32) / 255.0;
}

fn mock_rand_fast(n: u8) -> f32 {
	
	let base: u32 = 0b0_01111110_00000000000000000000000;
	
	/* Aligns the input byte to 32 bits then increases its values by
	 * shifting 15 places to the left. */
	let large_n = (n as u32) << 15;
	
	/* Take a bitwise OR merging the base with the input byte. */
	let f32_bits = base | large_n;
	
	/* Interprets an u32 as f32. */
	let m = f32::from_bits(f32_bits);
	
	/* Normalises the output range of the value. */
	return 2.0 * (m - 0.5);
}

fn main() {
    let sourced = random::<u8>();
    let prob = mock_rand(sourced);
    let f_prob = mock_rand_fast(sourced);
    
    println!("u8 = {}, f32 = {}, fast_f32 = {}", sourced, prob, f_prob);
}
