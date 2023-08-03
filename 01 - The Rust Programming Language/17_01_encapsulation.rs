/* A public structure with private attributes. */
pub struct AveragedCollection {
    
    list: Vec<i32>,
    average: f64,
}


impl AveragedCollection {
    
    /* A public function to add a value to internal list. */
    pub fn add(&mut self, value: i32) {
        
        self.list.push(value);
        self.update_average();
    }
    
    /* A public function to pop the last value from the list. */
    pub fn remove(&mut self) -> Option<i32> {
        
        /* Pop the value last added to the list */
        let result = self.list.pop();
        
        /* Check that a value has actually been found. */
        match result {
            
            /* If a value is returned update the average. */
            Some(value) => {
                
                self.update_average();
                return Some(value)
            }
            
            /* If there is an error return None. */
            None => {return None}
        }
    }
    
    /* Public function to get the value of the list average. */
    pub fn average(&self) -> f64 {
        
        return self.average
    }
    
    
    /* Private function to recalculate the list average. */
    fn update_average(&mut self) {
        
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}


fn main() {
    
    /* Initialise an instance of AveragedCollection */
    let mut my_list = AveragedCollection {
        list: vec![],
        average: 0.0,
    };
    
    /* Add values to the list. */
    for i in 0..100 {
        my_list.add(i)
    }
    
    println!("The list average is {}", my_list.average());
}
