#[derive(Debug)]
enum Cereal {
    Barley,
    Millet,
    Rice,
    Rye,
    Spelt,
    Wheat,
}

fn main() {
    /* Create an empty vector of Cereal type. */
    let mut grains: Vec<Cereal> = vec![];

    grains.push(Cereal::Rye);
    drop(grains);

    /* Try and access data that isn't there. */
    println!("{:?}", grains);
}
