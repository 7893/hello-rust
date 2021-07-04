use traits::NewArticle;
use traits::Summary; //trait
use traits::Tweet; //类型 //类型

fn main() {
    println!("Hello, world!");

    let tweet = Tweet {
        username: String::from("a_name"),
        content: String::from("of course, as you probably already know , people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let news = NewArticle {
        headline: String::from("headlines"),
        location: String::from("the_location"),
        author: String::from("the_author"),
        content: String::from("the_content"),
    };

    println!("1 new article: {}", news.summarize());
}
