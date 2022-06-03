use std::rc::Rc;
fn main() {
    let pointer = Rc::new(1);
    {
        let second_pointer = pointer.clone(); // or Rc::clone(&pointer)
        println!("{}", *second_pointer);
    }
    println!("{}", *pointer);
}
