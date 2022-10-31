fn main() {
    println!("be unsafe {}", HELLO_WORLD);

    // - defefernece a rww pointer
    // - call an unsafe function
    // - Access or modify a mutable static variable
    // - implementa n unsafe trait
    // - access fields of union(s)

    // Dereference a raw pointer
    let mut num = 5;

    let raw1 = &num as *const i32;
    let raw2 = &mut num as *mut i32;
    let address = 0x19099usize; // << no usar si no queremos un sementfault
    let _raw3 = address as *mut i32;
    unsafe {
        println!("raw1 is {}", *raw1);
        *raw2 = 3;
        println!("raw1 is {}", *raw1);
        // *_raw3 = 0 //Segmentation fault
    }

    //call unsafe function or method
    unsafe fn dangerous(){}

    unsafe{
        dangerous();
    }

    // using extern functions to call external code
    // will use FFI (foreing function interface)

    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Valor absoluto de -3 segun c: {}", abs(-3));
    }

    // - Access or modify a mutable static variable
    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // access unions
    #[repr(C)]
    union MyUnion {
        f1: u32,
        f2: f64,
    }

    let u = MyUnion{ f1: 3};
    let n = unsafe { u.f1 };
    println!(" en la union hay un {n}");
}

static HELLO_WORLD: &str = "hello, world";
static mut COUNTER: u32 = 0;

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
