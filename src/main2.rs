fn main() {
    println!("GM!");
}
pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub traint Summary{
    fn summarize(&self) -> String{
        String::from("(Read more...)")
    }
}

// These are kind of lik eobjects in a way
pub trait Summary {
    fn summarize(&self) -> String;    
}

// WE don't have a method body, we just want to say that each type to implement traint it should return a string.

