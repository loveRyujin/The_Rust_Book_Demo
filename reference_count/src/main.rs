use std::rc::Rc;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    #[allow(dead_code)]
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(3, Rc::new(Cons(4, Rc::new(Cons(5, Rc::new(Nil)))))));
    println!("reference count after creating a: {}", Rc::strong_count(&a));
    let b = Cons(1, Rc::clone(&a));
    println!("reference count after creating b: {}", Rc::strong_count(&a));
    println!("{:#?}", b);
    {
        let c = Cons(2, Rc::clone(&a));
        println!("reference count after creating c: {}", Rc::strong_count(&a));
        println!("{:#?}", c);
    }
    println!("reference count after releasing c: {}", Rc::strong_count(&a));
}
