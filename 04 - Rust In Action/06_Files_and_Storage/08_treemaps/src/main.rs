use std::collections::BTreeMap;

fn main() {
    let mut voc = BTreeMap::new();

    voc.insert(3_697_915, "Amsterdam");
    voc.insert(1_300_405, "Middelburg");
    voc.insert(540_00, "Enkhuizen");
    voc.insert(469_400, "Delft");
    voc.insert(266_868, "Hoorn");
    voc.insert(173_000, "Rotterdam");

    println!("\nAll Chambers Investments");
    for (guilder, kamer) in &voc {
        println!("\t{:<20} {:>20}", kamer, guilder);
    }

    println!("\nSmall Chamber Investments");
    for (small_guilder, kamer) in voc.range(0..500_000) {
        println!("\t{:<20} {:>20}", kamer, small_guilder);
    }
}
