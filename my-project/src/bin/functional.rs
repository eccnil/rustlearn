fn main() {
    closures();
    iterators();
}

fn closures() {
    let y = 2;
    let closure1 = |x| x + y;

    println!("closure1: {}", closure1(9));
}

fn iterators() {
    //iterators for indexed structures: vectors
    let v1 = vec![1, 2, 3, 4];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let mut v2_iter = v1.iter();
    let v21 = v2_iter.next();
    match v21 {
        Some(x) => println!("element {}", x),
        None => println!("no element"),
    }
    let v21 = v2_iter.next();
    match v21 {
        Some(x) => println!("element {}", x),
        None => println!("no element"),
    }
    let v21 = v2_iter.next();
    match v21 {
        Some(x) => println!("element {}", x),
        None => println!("no element"),
    }
    let v21 = v2_iter.next();
    match v21 {
        Some(x) => println!("element {}", x),
        None => println!("no element"),
    }
    let v21 = v2_iter.next();
    match v21 {
        Some(x) => println!("element {}", x),
        None => println!("no element"),
    }
    let v21 = v2_iter.next();
    match v21 {
        Some(x) => println!("element {}", x),
        None => println!("no element"),
    }

    let suma: i32 = v1.iter().sum();
    println!("el valor de la suma es {}", suma);

    //iterator adaptors -> no consume el iterador y produce otro como: map
    let v2:Vec<_> = v1.iter().map(|x| x+1).collect();
    for i in v2 {
        print!("{i}, ");
    }
    println!();

    let vias:i32 = 2;
    for i in v1.iter().filter(|x| x > &&vias){
        println!("sobre sesgo: {i}");
    }
}
