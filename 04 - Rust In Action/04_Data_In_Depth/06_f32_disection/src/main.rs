/* Definitions for decoding f32 */
const BIAS: i32 = 127;
const BASE: f32 = 2.0;

fn main() {
    let n: f32 = 42.42;

    let (sign, exp, frac) = to_parts(n);
    let (sign_d, exp_d, frac_d) = decode(sign, exp, frac);
    let n_recon = reconstitute_f32(sign_d, exp_d, frac_d);

    println!("original = {}\n", n);

    println!("sign:     {:01b}", sign);
    println!("exponent: {:08b}", exp);
    println!("mantissa: {:023b}\n", frac);

    println!("sign:     {}", sign_d);
    println!("exponent: {}", exp_d);
    println!("mantissa: {}\n", frac_d);

    println!("reconstituted = {}\n", n_recon);
}

fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();

    /* Remove 31 bits leaving only the sign bit. */
    let sign = (bits >> 31) & 1;

    /* Filter out the top bit with a logical AND mask then remove 23 bits. */
    let exponent = (bits >> 23) & 0xff;

    /* Retain the 23 least significant bits via an AND mask. */
    let fraction = bits & 0x7fffff;

    return (sign, exponent, fraction);
}

fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    /* Convert to either 1.0 or -1.0 */
    let signed_1 = (-1.0_f32).powf(sign as f32);

    /* Must be converted to i32 incase the result is -VE */
    let exponent = (exponent as i32) - BIAS;
    let exponent = BASE.powf(exponent as f32);

    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        let mask = 1 << i;

        let one_at_bit_i = fraction & mask;

        if one_at_bit_i != 0 {
            let weight = 2_f32.powf((i as f32) - 23.0);
            mantissa += weight;
        }
    }

    return (signed_1, exponent, mantissa);
}

fn reconstitute_f32(sign: f32, exponent: f32, fraction: f32) -> f32 {
    return sign * exponent * fraction;
}
