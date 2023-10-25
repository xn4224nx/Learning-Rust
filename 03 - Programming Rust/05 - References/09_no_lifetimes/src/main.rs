struct S<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

struct StringTable {
    elements: Vec<String>,
}

fn sum_r_xy(r: &i32, s: &S) -> i32 {
    return r + s.x + s.y;
}

impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0..self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i]);
            }
        }
        return None;
    }
}

fn main() {
    let height = 3;
    let width = 7;
    let point = S {
        x: &height,
        y: &width,
    };
    println!("{}", sum_r_xy(&1, &point));

    let test = StringTable {
        elements: vec![
            String::from("a3f3fsdfs"),
            String::from("a2f3fxfd_hrtr"),
            String::from("a232ewerrt"),
        ],
    };

    /* Search for the first string by a prefix. */
    let first_find = test.find_by_prefix("a3");

    /* If a element has been found print it. */
    if let Some(full_value) = first_find {
        println!("The first element with the prefix is '{}'", full_value);
    } else {
        println!("No element has been found with that prefix.");
    }
}
