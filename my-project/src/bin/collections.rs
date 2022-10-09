enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main(){
    vectors();
    strings();
    hashmaps();
}

fn vectors(){
    let mut vacio: Vec<i32> = Vec::new();
    let _v123 = vec![1,2,3];
    vacio.push(5);
    dbg!(vacio.push(3));


    //let elemento = vacio[100];
    let elemento = match vacio.get(1) {
        None => {print!("no element"); let default: i32 = 0; default}
        Some(elem) => *elem
    };
    println!("imprimiendo un vector {:?}, un elemento {}", vacio, elemento);

    //iterating
    let mut v = vec![100, 32, 57, 44, 55];
    println!("{:?}", v);
    for i in &mut v {
        *i += 20;
    }
    println!("{:?}", v);

    //with enums
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("hello")),
        SpreadsheetCell::Float(33.3),
    ];
}

fn strings(){
    //creaciones
    let mut _s = String::new();
    let _a = "initial content".to_string();
    let mut b = String::from("data");

    //cosa curiosa del ownership
    let my_slice = "!!.!!";
    b.push_str(my_slice);
    b=dbg!(b);
    println!("se supone que no podría usar {my_slice}, pero puedo!");

    //concat
    let s1 = String::from("Hello, ");
    let s2 = String::from("world !");
    let s3 = s1 + &s2 + "" + &my_slice; //note s1 has been moved here and cannot be used any further
    let s4 = format!("+ {b}, +"); //note format macro does not take anything
    println!("usando add {s3}, y la macro format!: {s4}");

    //acceso a las letras, 1) con indices no se puede, 2) con slices mala idea -> iterating
    let mut cn = 0;
    let mut bn = 0;
    let guevon = "Güevón 🤬";
    for c in guevon.chars(){
        print!("{},",c);
        cn += 1;
    }
    for b in guevon.bytes(){
        print!("{},",b);
        bn += 1;
    }
    println!("la cadena tenía {cn} letras en {bn} bytes");
}

fn hashmaps(){
    //creation
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10); //los strings son movidos al hashmap
    scores.insert(String::from("Yellow"), 20);

    //acceso
    let team_name=String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score = {score}");

    //iterating
    for (k,v) in &scores {
        println!("{k} <- {v}");
    }

    //updates
    scores.insert(String::from("Blue"), 22);
    scores.entry(String::from("Orange")).or_insert(33);
    scores.entry(String::from("Yellow")).or_insert(4);
    let azules = scores.entry(String::from("Blue")).or_insert(0);
    *azules += 1; 
    println!("{:?}", scores)
}
