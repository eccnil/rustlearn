fn main (){
    println!("lets start with threads");
    threads();
    messages();
    mutex();
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

fn messages(){
    use std::sync::mpsc; // multiple producers sigle consumer
    use std::thread;
    use std::time::Duration;

    let (tx, rx) = mpsc::channel();

    let _sender = thread::spawn(move || {
        let val = String::from("hi");
        //let val:i32 = 3; //para un entero (pila) mantenemos su ownership, no como con el string
        match tx.send(val) {
            Ok(()) => (),
            Err(e) => println!("error sending via channel: {}", e),
        };
        //println!("ya no tengo val, esta linea o compila {}", val);
    });

    let receiver = thread::spawn(move || {
        let received = rx.recv().unwrap(); // try_recv no bloquea el hilo
        println!("received via channel: {received}");
    });

    receiver.join().unwrap();

    // muliple producers
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    let _sender1 = thread::spawn( move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    let _sender2 = thread::spawn( move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rx {
        println!("Got: {received} from a channel of two");
    }
}

fn mutex(){
    use std::sync::{Mutex,Arc};
    use std::thread;

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); //panics if the holder threas paniqued already
        *num = 6; 
    }

    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: for mutex {}", *counter.lock().unwrap());

}
