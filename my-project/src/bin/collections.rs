enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main(){
    vectors();
    strings();
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
    
}
