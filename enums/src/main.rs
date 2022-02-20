


enum IpAddressKind {
	V4(u8, u8, u8, u8),
	V6(String),
}

enum Message {
	Quit,
	Move {x: i32, y: i32},
	Write(String),
	ChangeColor(i32, i32, i32)
}

impl Message {
	fn call(&self) {
		// 
	}
}

fn main() {

	let four = IpAddressKind::V4;
	let six = IpAddressKind::V6;

	let home = IpAddressKind::V4(127, 0, 0, 1);
	let loopback = IpAddressKind::V6(String::from("::1"));


	let m = Message::Write(String::from("hello"));
	m.call();


	let some_number = Some(5);
	let some_string = Some("a string");
	let absent_number: Option<i32> = None;

}

