fn main() {
    println!(" tipos avanzados ");

    new_type_pattern();
    sized_trait();
}

fn new_type_pattern(){
    type Kilometers = i32; //type alias
    type Thunk = Box<dyn Fn() + Send+ 'static>; //type alias

    let x:i32 = 5;
    let y:Kilometers = 5;

    println!("x + y = {}", x + y);

    let _f: Thunk = Box::new(|| println!("hi"));

    //never type
    fn _infinite_loop () -> ! {
        loop {
            print!("la has cagado ");
        }
    }
}

fn sized_trait() {
    fn generic<T: ?Sized>(t: &T) { // <<-- funcion no limitada a tipos de tamaño conocido por el compilador.
        println!("noop");
    }
}
