#[derive(Debug)] // so we can inspect the state in a minute
enum USState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn value_in_cents(coin: Coin) -> u8 {

    match coin {
        Coin::Penny => {
        
            println!("Lucky penny!!");
            return 1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}.", state);
            return 25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {

    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    
    let my_coin = Coin::Penny;
    let other_coin = Coin::Quarter(USState::Alaska);
    
    println!("A coin is {} cents.", value_in_cents(my_coin));
    println!("A coin is {} cents.", value_in_cents(other_coin));
    
    let five = Some(5);
    let six  = plus_one(five);
    let none = plus_one(None);
    
    let dice_roll = 9;
    
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        //other => move_player(other),  // Create a value to capture non accounted for values
        //_ => reroll() {},    // Ignore the value and call a function
        _ => (),    // Do nothing with other values
    }
    
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}
}


