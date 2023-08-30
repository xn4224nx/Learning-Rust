fn build_vector() -> Vec<i16> {
    /* Define an empty vector of 16-bit integers. */
    let mut v: Vec<i16> = Vec::<i16>::new();

    /* Add values to the vector and specify the number type. */
    v.push(10_i16);
    v.push(20_i16);

    return v;
}

fn build_vector_infered() -> Vec<i16> {
    /* Define an empty vector of 16-bit integers. */
    let mut v: Vec<i16> = Vec::<i16>::new();

    /* Add values to the vector and infer their type. */
    v.push(10);
    v.push(20);

    return v;
}

fn main() {
    println!("{:?}", build_vector());
    println!("{:?}", build_vector_infered());
}
