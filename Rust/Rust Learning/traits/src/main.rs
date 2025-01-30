mod trait_for_enum;
trait Summary {
    fn get_author(&self) -> &str;

    fn summarize(&self) -> String {
        //Default implementation
        let default = String::from("Default Implementation message from Summarize.");
        default
    }
}

trait Credentials {
    fn credentials_details(&self) -> String;
}

//NewsArticle Structure & it implement to Summary(Trait).
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//Note:-When we're not implem. to summarize in below,it'll return default implem. which is in trait.
impl Summary for NewsArticle {
    /*Here,we're not implem. to summarize,thats why we'll get default msg from trait Summary*/
    // fn summarize(&self) -> String {
    //     let content = format!("News by {}: {}", self.author, self.content);
    //     content
    // }
    fn get_author(&self) -> &str {
        self.author.as_str()
    }
}

//Tweet Structure & it implement to Summary(Trait).
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        let content = format!("Tweet by:- {}: {}", self.username, self.content);
        content
    }
    fn get_author(&self) -> &str {
        self.username.as_str()
    }
}

impl Credentials for Tweet {
    fn credentials_details(&self) -> String {
        let uname = format!("User name is:{}", self.username);
        uname
    }
}
fn main() {
    let tweet = Tweet {
        username: String::from("Jay"),
        content: String::from("Amazing tutorial of Rust."),
        reply: false,
        retweet: false,
    };

    let news_article = NewsArticle {
        headline: String::from("Big News!!"),
        location: String::from("Ahmedabad"),
        author: String::from("Jay Prajapati"),
        content: String::from("Rust is very usefull language in IT."),
    };

    //We're passing borrow reference bec. we don't want to give ownership to func.
    println!("\n ** Below details From NewsArticle structure **");

    news_aggregator_tweet(&news_article);
    println!("\n ** Below details From Tweet structure **");
    news_aggregator_tweet(&tweet);

    /*Trait bound:-If we pass first param as tweet, then second param. should be tweet,
    if we pass news_article in second param then it'll give err.(Both param. should be same type.) */
    mixup(&tweet, &tweet); //We're passing refer. bec. wan't to give ownership

    /*Multiple trait:- If any one struct implements both "Summary" & "Credentials" trait for that structure it'll work.Ahiya "tweet" ae bane trait nu implem. kare chhe atle jo aene pass kariye to ae work karse.
     */
    multiple_trait_for_impl_type(&tweet);

    multiple_trait_for_generic_type(&tweet);

    println!("\n ** Trait implementation by Enum!");
    trait_for_enum::demo();
}

fn news_aggregator_tweet(source: &impl Summary) {
    println!("There is new news in market!!");
    println!("{}", source.summarize());
    println!("{}", source.get_author());
}

//** Trait bound */
fn mixup<T: Summary>(item1: &T, item2: &T) {
    println!("\n** Trait bound **");
    println!("Item 1 of Trait bound:-{}", item1.summarize());
}

//* Multiple Trait bound:-(a)impl type,(b)generic type. */
//Je struct Summary & Credentials banne trait ne implme. kare aej allow chhe.
fn multiple_trait_for_impl_type(item: &(impl Summary + Credentials)) {
    println!("\n** Multiple Trait bound **");
    println!(
        "Item of Multiple Trait bound of impl type:-{}",
        item.credentials_details()
    );
}

pub fn multiple_trait_for_generic_type<T: Summary + Credentials>(item: &T) {
    println!("\n** Multiple Trait bound **");
    println!(
        "Item of Multiple Trait bound of generic type:-{}",
        item.summarize()
    );
}
