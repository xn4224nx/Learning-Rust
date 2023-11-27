fn greet_world() {

    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let england = "Hello, world!";
    
    let regions = [southern_germany, japan, england];
    
    println!("\nIn many places:");
    for region_greet in regions.iter() {
        println!("\t{}", &region_greet);
    };
}

fn main() {
    greet_world();
}
