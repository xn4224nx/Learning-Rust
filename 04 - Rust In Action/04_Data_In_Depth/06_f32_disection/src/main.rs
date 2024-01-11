/* Definitions for decoding f32 */
const BIAS: i32 = 127;
const BASE: f32 = 2.0;

fn main() {
	let n: f32 = 42.42;
	
	let (sign, exp, frac) = to_parts(n);
	
	println!("sign:     {:01b}", sign);
	println!("exponent: {:08b}", exp);
	println!("mantissa: {:023b}", frac); 
}

fn to_parts(n: f32) -> (u32, u32, u32) {
		
	let bits = n.to_bits();
	
	/* Remove 31 bits leaving only the sign bit. */
	let sign = (bits >> 31) & 1;
	
	/* Filter out the top bit with a logical AND mask then remove 23 bits. */
	let exponent = (bits >> 23) & 0xf;
	
	/* Retain the 23 least significant bits via an AND mask. */
	let fraction = bits & 0x7fffff;
	
	return (sign, exponent, fraction);
}
