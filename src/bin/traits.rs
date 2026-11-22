use std::fmt::Display;

pub struct NewsArticle {
    pub author: String,
    pub content: String,
    pub headline: String,
}

pub struct Tweet {
    pub reply: bool,
    pub retweet: bool,
    pub content: String,
    pub username: String,
}

pub trait Summary {
    // no body: REQUIRED
    fn summarize_author(&self) -> String;

    // body: DEFAULT
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// accepts anything that implements the summary trait
// and calls summary
pub fn notify(item: &impl Summary) {
    println!("Breaking News! {}", item.summarize())
}

// longer version of notify
pub fn notify_<T: Summary>(item: &T) {
    println!("Breaking News! {}", item.summarize())
}

// this will have 2 summary types
pub fn notify2(item1: &impl Summary, item2: &impl Summary) {
    // ...
}

// this will have 1 type only which is summary
// we can also add more traits using '+'
pub fn notify_2<T: Summary + Display>(item1: &T, item2: &T) {
    // ...
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
    hockey team in the NHL.",
        ),
    };

    let post = Tweet {
        reply: false,
        retweet: false,
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    };

    println!("New TWEET: {}", post.summarize());
    println!("New ARTICLE: {}", article.summarize());

    notify(&article);
}
