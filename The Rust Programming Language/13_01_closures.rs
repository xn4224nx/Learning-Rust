use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColour {
    Red,
    Blue,
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


struct Inventory {
    shirts: Vec<ShirtColour>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColour>) -> ShirtColour {
        return user_preference.unwrap_or_else(|| self.most_stocked())
    }
    
    fn most_stocked(&self) -> ShirtColour {
        
        // Count the number of each coloured shirt in the inventory
        let mut num_red = 0;
        let mut num_blue = 0;
        
        for colour in &self.shirts {
            match colour {
                ShirtColour::Red => num_red += 1,
                ShirtColour::Blue => num_blue += 1,
            }
        }
        
        if num_red > num_blue {
            return ShirtColour::Red
        } else {
            return ShirtColour::Blue
        }
    }
}


fn generate_workout(intensity: u32, random_number:u32) {
    
    // Define a closure with type annotations and a return type
    let expen_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        return num
    };
    
    
    if intensity < 25 {
        println!("Today, do {} pushups!", expen_closure(intensity));
        println!("Next, do {} situps!", expen_closure(intensity)); 
    
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    
    } else {
        println!("Today, run for {} minutes!", expen_closure(intensity)); 
    }
}

fn main() {
    
    // Define the initial shirt inventory
    let store = Inventory {
        shirts: vec![ShirtColour::Blue, ShirtColour::Red, ShirtColour::Blue],
    };
    
    // Make a giveaway to a user with a preference 
    let user_pref1 = Some(ShirtColour::Red);
    let giveaway1 = store.giveaway(user_pref1);
    
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );
    
    // Make a giveaway to a user without a preference 
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
    
    let sim_user_val = 100;
    let sim_ran_num = 3;
    
    generate_workout(sim_user_val, sim_ran_num);
    
    // Function definition
    fn add_one_v1 (x: u32) -> u32 {x + 1}
    
    // Different closure definitions
    let add_one_v2 = |x: u32| -> u32    {x + 1};
    let add_one_v3 = |x|                {x + 1};
    let add_one_v4 = |x|                 x + 1 ;
    
    // Unless v3 and v4 are used their types cannot be infered therfore:
    println!("{}", add_one_v3(3));
    println!("{}", add_one_v4(4));
    
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    
    // This line breaks because the closures type is changed from its intial
    //let n = example_closure(5);
    
    // 
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    
    let only_borrows = || println!("From closure: {:?}", list);
    
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
    
    let mut my_list = vec![1, 2, 3];
    println!("Before defining the closure: {:?}", my_list);
    
    let mut borrows_mutably = || my_list.push(7);
    
    borrows_mutably();
    println!("After defining the closure: {:?}", my_list);
    
    let list_for_thread = vec![1, 2, 3];
    println!("Before defining the thead closure: {:?}", list_for_thread);
    
    thread::spawn(move || println!("From thread: {:?}", list_for_thread))
        .join()
        .unwrap();
        
    
    let mut rec_list = [
        Rectangle{width: 10, height: 1},
        Rectangle{width: 3, height: 5},
        Rectangle{width: 7, height: 12},
    ];
    
    rec_list.sort_by_key(|r| r.width);
    println!("{:#?}", rec_list);
    
    let mut num_sort_operations = 0;
    let value = String::from("by key called");
    
    rec_list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    
    println!("Number of sort operations: {}", num_sort_operations);
}
