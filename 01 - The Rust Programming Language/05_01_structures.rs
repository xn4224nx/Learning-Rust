fn main() {
    
    // Create a new instance of a structure
    let mut new_user = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
    let new_user_2 = build_user(String::from("no@no.com"), 
                                String::from("no_user"));
    
    // Modify a value in the structure
    new_user.email = String::from("anotheremail@example.com");
    
    // Print a structure
    println!("active        = {}", new_user_2.active);
    println!("username      = {}", new_user_2.username);
    println!("email         = {}", new_user_2.email);
    println!("sign in count = {}", new_user_2.sign_in_count);
    
    // Create a structure from some of the value of another
    let new_user_3 = User {
        email: String::from("another@example.com"),
        ..new_user_2
    };
    
    // Print a structure
    println!("active        = {}", new_user_3.active);
    println!("username      = {}", new_user_3.username);
    println!("email         = {}", new_user_3.email);
    println!("sign in count = {}", new_user_3.sign_in_count);
    
    // Use Tuple Structures
    let black = Colour(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    // Use a Unit-Like structure
    let subject = AlwaysEqual;
}

// Unit-Like Structures
struct AlwaysEqual;

// Tuple Structures
struct Colour(i32, i32, i32);
struct Point(i32, i32, i32);


// Define a structure
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


// Function to create a new structure
fn build_user(email: String, username: String) -> User {
    return User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

