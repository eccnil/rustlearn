fn main () {
    println!("advanced functions and traits");

    //function pointers -> fn is a *function pointer*
    function_pointers();
    returning_closures();
}

fn function_pointers() {
    fn add_one(x:i32) -> i32 { x +1 }
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 { f(f(arg)) } // fn is a type not a trait
    println!("doing twice {} (should be 5)", do_twice(add_one, 3));

    //function type (fn) implments all closure traits -> Fn, FnMut, FnOnce
    //extenrnal (C) functions does not.

    //cada rama de un enum, genera una función con su nombre como
    //constructor y la podemos usar
    enum Status {
        Value(u32),
        _Stop,
    }

    let _list_of_status: Vec<Status> =
        (0u32..20).map(Status::Value).collect();
}

fn returning_closures(){
    //los traits no existen, necesitamos punteros
    fn _returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x +1 )
    }
}
