

fn main() {

	let s: String;
	s = "contents".to_string();

	println!("{}", &s);

	// let hello = String::from("السلام عليكم");
	// let hello = String::from("Dobrý den");
	// let hello = String::from("Hello");
	// let hello = String::from("שָׁלוֹם");
	// let hello = String::from("नमस्ते");
	let hello = String::from("こんにちは");
	// let hello = String::from("안녕하세요");
	// let hello = String::from("你好");
	// let hello = String::from("Olá");
	// let hello = String::from("Здравствуйте");
	// let hello = String::from("Hola");

	println!("{}", &hello);


	let mut s2 = String::from("foo");
	s2.push_str("bar");
	// s2.push_str(String::from("bar")); // not allowed


	let s3 = String::from("Hello, ");
	let s4 = String::from("world!");
	let s5 = s3 + &s4;

}


