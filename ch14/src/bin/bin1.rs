
use rustbook_ch14_02 as ch14;
use rustbook_ch14_02::Art::mix;
use rustbook_ch14_02::Art::PrimaryColor;


fn main() {
	println!("Hello, world!");
	println!("{:?}", ch14::add_one(1.5));

	let red = PrimaryColor::Red;
	let yellow = PrimaryColor::Yellow;
	mix(red, yellow);
}

