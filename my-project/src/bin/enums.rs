enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
    None,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Self::Penny => 1,
            Self::Nickel => 5,
            Self::Dime => 10,
            Self::Quarter => 25,
        }
    }
}


fn main() {
    let _ip = IpAddrKind::V6(String::from("a.a.a.b"));
    let _ip = IpAddrKind::None;
    let _ip = IpAddrKind::V4(192,10,0,1);
    match _ip {
        IpAddrKind::V4(192, _, _, _) => println!("local ip"),
        IpAddrKind::V4(10, _ , _, _) => (),
        IpAddrKind::V4(_, _, _, _) | IpAddrKind::None => todo!() ,
        IpAddrKind::V6(x) => println!("your v6 ip is {x}"), 
    }
    
    let _optional = Option::Some(4);
    let _optional: Option<i32> = None;
    let _optional = Some(6);
    dbg!(may_add_1(_optional));
    println!("tengo {:?}?",_optional);
    if let Some (n) = _optional {
        println!("tenemos un {n}")
    } else {
        println!("no tenemos nada")
    }

    let _my_coin = Coin::Dime; 
    let _my_coin = Coin::Penny;
    let my_coin = Coin::Nickel;
    let _my_coin = Coin::Quarter;
    println!(" this {:?} equivalent to {} cents", my_coin, my_coin.value_in_cents());
}

fn may_add_1 (x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(n) => Some(n+1)
    }
}