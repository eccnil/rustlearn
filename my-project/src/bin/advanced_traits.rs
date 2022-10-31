fn main() {
    println!("advanced traits");

    pub trait Iterator2 {
        type Item;

        fn next(&mut self) -> Option <Self::Item>;
    }

    trait Pilot {
        fn fly(&self);
        fn sound();
    }

    trait Wizard {
        fn fly(&self);
        fn sound();
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self){
            println!("This is your captain speaking");
        }
        fn sound() {
            println!("zoooom");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("up!");
        }
        fn sound() {
            println!("ding");
        }
    }

    impl Human {
        fn fly(&self){
            println!("*waving arms furiously*");
        }
    }

    let person = Human;

    person.fly();
    Pilot::fly(&person);
    <Human as Wizard>::sound();
}
