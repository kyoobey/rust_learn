use std::fmt::{Display, Debug};



pub trait Summary {
	// fn summarize(&self) -> String;

	fn summarize_author(&self) -> String;

	fn summarize(&self) -> String {
		format!("(Read more from {} ...)", self.summarize_author())
	}
}


pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String
}

impl Summary for NewsArticle {
	fn summarize_author(&self) -> String {
	    format!("{}", self.author)
	}

	// fn summarize(&self) -> String {
	// 	format!("{}, by {} ({})", self.headline, self.author, self.location)
	// }

}


pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool
}

impl Summary for Tweet {
	fn summarize_author(&self) -> String {
	    format!("@{}", self.username)
	}

	// fn summarize(&self) -> String {
	//     format!("{}: {}", self.username, self.content)
	// }
}


// pub fn notify(item: &impl Summary) {
// 	println!("Breaking news! {}", item.summarize());
// }
// pub fn notify<T: Summary>(item: &T) {
// 	println!("Breaking news! {}", item.summarize());
// }
// pub fn notify(item: &impl Summary + Display) {
pub fn notify<T: Summary + Display>(item: &T) {
	println!("Breaking news! {}", item.summarize());
}
/*
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
the types of item1 and item2 can be different

pub fn notify<T: Summary>(item1: &T, item2: &T) {
the types of item1 and item2 is always same
*/

fn some_function<T, U>(t: &T, u: &U) -> i32
	where	T: Display + Clone,
			U: Clone + Debug
{
	0
}


fn return_summarizable() -> impl Summary {
	Tweet {
		username: String::from("horse_ebooks"),
		content: String::from("of course, as you probably already know, people"),
		reply: false,
		retweet: false
	}
}

