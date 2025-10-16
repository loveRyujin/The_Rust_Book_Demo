use std::rc::Rc;

#[derive(Debug)]
enum List {
    #[allow(dead_code)]
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(List::Cons(3, Rc::new(List::Cons(4, Rc::new(List::Cons(5, Rc::new(List::Nil)))))));
    let b = List::Cons(1, Rc::clone(&a));
    let c = List::Cons(2, Rc::clone(&a));
    println!("{:#?}", b);
    println!("{:#?}", c);
}
