fn main() {
    println!("macros");
    //declarative macros
    // - macro rules!
    call_mi_macro();
    //procedural macros
    // - #[derive]
    // - attribute like macros
    // - function-like macros

}

#[macro_export]
macro_rules! mi_macro {
    ( $($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    }
}

fn call_mi_macro(){
    let _v = mi_macro!(2,3,4,5);
}


