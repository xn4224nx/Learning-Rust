fn main() {
    let penguin_data = "\
    common_name, length
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    for (i, record) in penguin_data.lines().enumerate() {
        /* Skip the header and pure whitespace lines. */
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        /* Extract every element of the line into a vector. */
        let fields: Vec<_> =
            record.split(',').map(|field| field.trim()).collect();

        /* Prints the record if the bin is not release. */
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        /* Try and parse the numeric field and then print the row. */
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", fields[0], length);
        }
    }
}
