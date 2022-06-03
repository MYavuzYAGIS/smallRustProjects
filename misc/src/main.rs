// smart pointers
use std::rc::Rc;
fn main() {
    let pointer = Rc::new(1);
    {
        let second_pointer = pointer.clone(); // or Rc::clone(&pointer)
        println!("{}", *second_pointer);
    }
    println!("{}", *pointer);

    let pointer_a = Rc::new(2);

    let pointer_b = pointer_a.clone();

    println!("{} , {}", *pointer_a, *pointer_b);
}
