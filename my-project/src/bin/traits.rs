use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;
    fn label(&self) -> String{
        String::from("undisclosed")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub reteet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn print_label (item: &impl Summary){
    println!("the label for {} is --> {}", item.summarize(), item.label());
}


fn print_label_sugar<T: Summary>(item: &T){
    println!("sugar bayby, fuking {} sugar", item.label());
}


fn _print_label_sugar2<T>(item: &T)
where
    T: Summary + Summary
{
    println!("sugar bayby, fuking {} sugar", item.label());
}

fn main () {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        reteet: false,
    };

    println!("1 new teet {}", tweet.summarize());
    print_label(&tweet);
    print_label_sugar(&tweet);

    //Borrow checker & lifetime
    let string1 = String::from("abcd");
    let string2 :&'static str = "xyz";

    let _result = longest(&string1.as_str(), string2);
    let _result = longuest_with_an_announcement(&string2, &string1, "anuncio!");
}

fn longest<'a>(x: &'a str, y:&'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longuest_with_an_announcement <'a,T> (
    x: &'a str,
    y: &'a str,
    ann: T,
    ) -> &'a str
where
    T: Display
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
