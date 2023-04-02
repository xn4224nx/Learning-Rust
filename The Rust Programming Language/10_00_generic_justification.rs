fn max(list: &[i32]) -> &i32 {
    let mut curr_max = &list[0];
    
    for item in list {
        if item > curr_max {
            curr_max = item;
        }
    }
    
    return curr_max
}


fn main() {
    
    let number_list = vec![34, 50, 25, 100, 65];
    
    let mut largest = &number_list[0];
    
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    
    println!("The largest number is {largest}");
    
    println!("The largest value is {}", max(&vec![1, 2, 3, 4, 5, 6, 7]))
}
