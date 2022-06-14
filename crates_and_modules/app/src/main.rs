


extern crate rename_phrases;
use rename_phrases::{
	en_hello, en_goodbye,
	jp_hello, jp_goodbye
};

#[macro_use]
extern crate log;
use simple_logger;



fn main() {

	simple_logger::init_with_level(log::Level::Debug).expect("Couldn't initialize logger");

	info!("phrases lib version: {}", rename_phrases::internal_phrases_version());
	info!("rename_phrases lib version: {}", rename_phrases::version());

	// println!(
	// 	"{}\n{}\n{}\n{}",
	// 	en_hello(),
	// 	en_goodbye(),
	// 	jp_hello(),
	// 	jp_goodbye(),
	// );

	println!("{}", en_hello());
	println!("{}", en_goodbye());
	println!("{}", jp_hello());
	println!("{}", jp_goodbye());
 
}


