use std::{rc::Rc, cell::RefCell, i32};

fn main () {
    println!("pointers in the way");
    boxes();
    reference_counted();
    interior_mutability();
}

fn boxes(){
    let b = Box::new(5);
    println!("b = {}", b);

    use crate::List::{Cons,Nil};
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    {
        let _list2 =  Cons(99, Box::new(Nil));
    }
    let _list3 =  Cons(77, Box::new(Nil));

    std::mem::drop(_list3); // o solamente drop(_list3)

    let name = Box::new("Lucio");
    hello(&name);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl Drop for List {
    fn drop(&mut self) {
        match self {
            List::Nil => println!("dropping NIL"),
            List::Cons(n,_) => println!("drop {n}"),
        }
    }
}

enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}

fn hello(name: &str){
    println!("I am command to say Hello to {name}!");
}

fn reference_counted(){
    //Rc<T> keeps track of the number of references to.
    //Rc is intended to be used only in mono-thread scenarios.
    //Rc allows multiple owners of same data, but all inmutable
    use crate::List2::{Cons,Nil};

    let a = Rc::new(Cons(1, Rc::new(Cons(0, Rc::new(Nil)))));
    println!("usage of a --> {}", Rc::strong_count(&a));
    let _b = Cons(1, Rc::clone(&a));
    {
        let _c = Cons(2, Rc::clone(&a)); //clone, wroten in this mode, not a.clone(), only incrementes usage counter.
        println!("usage of a --> {}", Rc::strong_count(&a));
    }
    println!("usage of a --> {}", Rc::strong_count(&a));
}

fn interior_mutability () {
    // unsafe code indecates to the compiler manual checking of the rules of take-borrow
    // unsafe code needs to be wrapped in a safe API
    // with RefCell<T> if you break the rules the compiler is ok, but runtime panics!
    // RefCell<T> is only for use in single-threaded secenarios, RefCell<T> has single owners, but multiple mutable borrows and inmutables can be modified

    //mock
    let mierda_a = Mierdainsegura::new("hola");
    mierda_a.add('p');
    let _one_imut=mierda_a.b.borrow();
    // let _one_mut=mierda_a.b.borrow_mut(); // <- fallaría
    eprintln!("mierda de 2+2={}", mierda_a.b.borrow());

}

struct Mierdainsegura {
    b : RefCell<String>
}

impl Mierdainsegura {
    fn new (b:&str) -> Mierdainsegura {
        Mierdainsegura { b: RefCell::new(String::from(b)) }
    }
    fn add(&self, cadena: char) {
        self.b.borrow_mut().push(cadena);
    }
}
