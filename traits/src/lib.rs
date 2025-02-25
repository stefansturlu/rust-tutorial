pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize_with_default(&self) -> String {
        String::from("(Read more...)")
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
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// impl Trait is syntactic sugar, equivalent to this trait bound:
pub fn notify_2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/* 
// See differences. Arguments in latter def must have same type.:
pub fn notify(item1: &impl Summary, item2: &impl Summary) { ... }
pub fn notify<T: Summary>(item1: &T, item2: &T) { ... }

// For multiple traits:
pub fn notify(item: &(impl Summary + Display)) {...}
pub fn notify<T: Summary + Display>(item: &T) {...}
*/


// where clauses for clearer trait bounds
use std::fmt::{Display, Debug};
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    42
}

fn some_function_2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{ 
    42 
}



// Conditionally implement methods

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}