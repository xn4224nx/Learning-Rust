pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


pub fn add_two(a: i32) -> i32 {
    return a + 2
}


pub fn greeting(name: &str) -> String {
    format!("Hello!")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    #[should_panic(expected = "Different panic state.")]
    fn another() {
        panic!("Make this test fail!");
        println!("Not panicing here.");
    }
    
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        
        assert!(larger.can_hold(&smaller));
    }
    
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        assert_ne!(5, add_two(4));
    }
    
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, returned value was \"{}\".", result
        );
    }
    
    #[test]
    fn it_works() -> Result<(), String> {
        if add(2, 3) == 4 {
            Ok(())
        } else {
            Err(String::from("Result does not equal four"))
        }
    }
}
