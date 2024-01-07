use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64,
}

fn main() {
    let base: Rc<RefCell<GroundStation>> =
        Rc::new(RefCell::new(GroundStation { radio_freq: 87.65 }));

    println!("base: {:?}", base);

    /* A new scope where the base can be mutable borrowed. */
    {
        let mut base2 = base.borrow_mut();
        base2.radio_freq -= 12.34;
        println!("base 2: {:?}", base2);
    }

    println!("base: {:?}", base);

    let mut base3 = base.borrow_mut();
    base3.radio_freq += 43.21;

    println!("base: {:?}", base);
    println!("base 3: {:?}", base3);
}
