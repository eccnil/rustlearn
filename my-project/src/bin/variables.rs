
fn main() {
    variables();
    tipos();
    funciones();
    control_flow();
    ownership();
}

fn variables(){
    const Y: usize = 3*9;
    let mut x = 5;
    println!("the value of x is {x}");
    x = 6+Y;
    println!("the value of x is {x}");
    let x = 4;
    println!("the value of x is {x}");
    {
        let x=x+1;
        println!("the value of x is {x}");
    }
    println!("the value of x is {x}");
}

fn tipos(){
    let _a : usize = 1_200;
    let _b : u128 = 2_200;
    let _c = b'a';
    let letra: char = '🤓';
    //let abc = a+b+c;
    let _d: f64 =  3_000.02;
    println!("division {} y modulo {} de enteros (5 y 2) {}", 5/2, 5%2, letra);

    let tupla: (i32, f64, u8)= (500, 5.5 ,1);
    let (_x,_y,z) = tupla;
    println!("z en la tupla: {} o {}", z, tupla.2);
    let _unit = ();

    let mi_array: [i32; 5] = [1,2,3,4,5];
    let mis_ceros = [0; 5];
    println!("mi array empieza con {}",mi_array[mis_ceros[0]]);

}

fn funciones() -> () {
    let r = function1(22, 't');
    println!("me dicen que {r}");
}

fn function1(x:i32, unit_label: char) -> i32 {
    println!("me dices que {}? {}??", x, unit_label);
    let y = {
        let z = 3;
        z + 1
    };
    println!("y si te digo que {y}?");
    x/2
}

fn control_flow(){
    let mut x = 0;
    'labeled: loop{
        x = x+1; 
        if x<5 {
            println!("otro mas {}", x);
            continue;
            //println!("no salgo por pantall");
        } else {
            println!("ya no mas {}", x);
            break 'labeled;
            //println!("no salgo por pantalla");
        };
    }

    while x>1 {
        x = x-1;
        println!("otro menos {}", x);
    }

    for i in (1..=10).rev(){
        println!("mas facil {}", i);
    }

}

fn ownership(){
    let hola= String::from("hola");
    let hola2 = &hola;
    //let _hola5 = hola;
    let hola3 = hola2.clone();
    println!("{hola} {hola2} {hola3}");

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("{r1}");
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;
    println!("{r2}");

    let s = String::from("hello world");
    let hello = &s[..5]; //slice
    let world = &s[6..s.len()];
    let todo = &s[..] ;
    println!("{} {} -> {} {}!", world, hello, todo, &s[4..=5]);
}