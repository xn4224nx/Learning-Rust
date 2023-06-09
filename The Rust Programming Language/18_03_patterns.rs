fn main() {
    /* Matching literal values */
    let x = -1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    /* Matching Named Variables */
    let a = Some(5);
    let b = 10;

    match a {
        Some(50) => println!("Got 50"),

        /* The `b` is a shadowed variable not the original */
        Some(b) => println!("Matched, b = {b}"),
        _ => println!("Default case, a = {:?}", a),
    }

    println!("At the end: a = {:?}, b = {b}", a);

    /* Multiple Match Patterns */
    let c = 2;

    match c {
        1 | 2 => println!("One or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    /* Matching Ranges of Integers */
    let d = 6;

    match d {
        1..=5 => println!("One through five"),
        _ => println!("Something else"),
    }

    /* Matching Ranges of Chars */
    let e = 'J';

    match e {
        'A'..='J' => println!("Early ASCII letter."),
        'K'..='Z' => println!("Late ASCII letter."),
        _ => println!("Something else"),
    }

    /* Destructuring Structures into Variables */
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 0 };

    let Point { x: a, y: b } = p;
    println!("{a}");
    println!("{b}");

    let Point { x, y } = p;
    println!("{x}");
    println!("{y}");

    /* Matching Structures */
    match p {
        Point { x: 0, y: 0 } => println!("Point on the origin"),
        Point { x: 0, y } => println!("Point on the y-axis at {y}"),
        Point { x, y: 0 } => println!("Point on the x-axis at {x}"),
        Point { x, y } => println!("Point on neither axis"),
    }

    /* Matching Enums */
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("No data to remove");
        }
        Message::Move { x, y } => {
            println!("Move box to ({x}:{y})");
        }
        Message::Write(text) => {
            println!("Text message: '{text}'");
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to Red: {r}, Green: {g}, Blue: {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to Hue: {h}, Saturation: {s}, Value: {v}")
        }
        _ => (),
    }

    /* Destructuring Structs and Tuples */
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    println!("{feet}");
    println!("{inches}");

    /* Ignoring a value with an `_` */
    fn foo(_: i32, y: i32) {
        println!("y = {y}");
    }

    foo(3, 4);

    /* Ignoring Parts of a Value with a nested `_` */
    let mut setting_value = Some(3);
    let new_setting_value = Some(8);

    match (setting_value, new_setting_value) {
        /* If the value is already set don't overwrite it */
        (Some(_), _) => {
            println!("Can't overwrite an existing value!");
        }

        _ => {
            setting_value = new_setting_value;
        }
    }

    if setting_value.is_some() {
        println!("Setting: is {}", setting_value.unwrap());
    } else {
        println!("Setting: is {:?}", setting_value);
    }

    /* Ignoring multiple parts of a tuple */
    let numbers = (1, 2, 3, 4, 5, 6);

    match numbers {
        (first, _, third, _, fifth, _) => {
            let sum = first + third + fifth;
            println!("sum of odd positioned numbers = {}", sum);
        }
    }

    /* Using `_` to not bind a value */
    let tst_string: Option<String> = Some(String::from("Hello!"));

    if let Some(_) = tst_string {
        println!("`tst_string` contains a '{}'", tst_string.unwrap());
    } else {
        println!("tst_string = {:?}", tst_string);
    }

    /* Ignoring remaining values */
    match p {
        Point { x, .. } => println!("x = {}", x),
    }

    match numbers {
        (first, .., last) => {
            println!("first = {first}, last = {last}");
        }
    }

    /* Match Guards */
    let num = Some(0);
    let ext_var = 2;

    match num {
        Some(n) if n == ext_var => println!("{n} = {ext_var}"),
        Some(x) if x % 2 == 0 => println!("{x} is even"),
        Some(x) => println!("{x} is odd"),
        None => (),
    }

    /* Combining Multiple Patterns and Guards */
    let x = 4;
    let y = true;

    match x {
        (4 | 5 | 6) if y => println!("yes"),
        _ => println!("no"),
    }

    /* Bindings with `@` */
    enum Message2 {
        Hello { id: i32 },
    }

    let msg2 = Message2::Hello { id: 1 };

    match msg2 {
        Message2::Hello {
            id: id_variable @ 3..=7,
        } => println!("id in range 3 to 7: {id_variable}"),

        Message2::Hello {
            id: id_variable @ 10..=12,
        } => println!("id in range 10 to 12:  {id_variable}"),

        Message2::Hello { id } => println!("Found another id: {id}"),
    }
}
