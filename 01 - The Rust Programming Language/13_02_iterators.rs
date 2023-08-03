#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    
    return shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn main() {
    
    // Define a vector
    let v1 = vec![1, 2, 3];
    
    // Define an iterator for the vector
    let v1_iter = v1.iter();
    
    // Use the iterator
    for val in v1_iter {
        println!("Got: {}", val);
    }
    
    let v2: Vec<i32> = vec![1, 2, 3];
    
    // Collect consumes the iterator
    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect();
    
    println!("{:?}", v3);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn my_iterator() {
        let v1 = vec![4, 5, 6];
        
        let mut v1_iter = v1.iter();
        
        assert_eq!(v1_iter.next(), Some(&4));
        assert_eq!(v1_iter.next(), Some(&5));
        assert_eq!(v1_iter.next(), Some(&6));
        assert_eq!(v1_iter.next(), None);
    } 
    
    #[test]
    fn interator_sum() {
        let v1 = vec![1, 2, 3];
        
        let v1_iter = v1.iter();
        
        // v1_iter is consumed here and cannot be used again
        let total: i32 = v1_iter.sum();
        
        assert_eq!(total, 6);
    }
    
    #[test]
    fn filters_by_size() {
        
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        
        let in_my_size = shoes_in_size(shoes, 13);
        
        assert_eq!(in_my_size, 
            vec![Shoe {size: 13, style: String::from("sandal")},]
        );
        
    }
}
