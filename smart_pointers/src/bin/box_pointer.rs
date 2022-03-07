


// struct Recursive_test {
// 	item1: i32,
// 	item2: Recursive_test
// }


enum List {
	Cons(i32, Box<List>),
	Nil
}


use crate::List::{Cons, Nil};

fn main() {
	// let b = Box::new(5);
	// println!("b = {}", b);


	let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}


