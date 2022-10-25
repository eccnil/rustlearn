fn main (){
    println!("lets start with threads");
    threads();
    //message
    //shared state
    //sinc & send traits
}

fn threads() {
    use std::thread;
    use std::time::Duration;

    let v: i32 = 5;

    let handle = thread::spawn(move || {
        println!("enthering in thread with {}", v);
        for i in 1..10 {
            println!("hi number {} from the spawned threas!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
