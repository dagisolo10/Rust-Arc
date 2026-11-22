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
}
