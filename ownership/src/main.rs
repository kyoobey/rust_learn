

fn main() {

	// let mut s = String::from("hello");
	// s.push_str(", world!");

	// println!("{}", s);

	// let x = 5;
	// let y = x;
	// println!("{} {}", x, y);

	// let s1 = String::from("hello");
	// let s2 = s1.clone();
	// println!("{} {}", s1, s2);

	// let s = String::from("hello");
	// takes_ownership(s);

	// let mut x: u32 = 5;
	// let r = &mut x;
	// make_copy(r);
	// println!("{}", x);

	// let mut s1 = gives_ownership();
	// s1.push_str(" truly");
	// println!("{}", s1);

	// let s = String::from("hello world this is a test");
	// // let hello = &s[..5];
	// // let world = &s[6..];
	// let hello = first_word(&s);
	// let world = second_word(&s);
	// println!("{}, {}!", hello, world);

	let a = [1,2,3,4,5];
	let slice = &a[1..3];
	assert_eq!(slice, &[2,3]);

}

fn takes_ownership(some_string: String) {
	println!("{}", some_string);
}

fn gives_ownership() -> String {
	let s1 = String::from("yours");
	s1
}

fn make_copy(r: &mut u32) {
	let mut x = *r;
	x += 1;
	println!("{}", x);
}

fn first_word(s: &str) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}

	&s[..]
}

fn second_word(s: &str) -> &str {
	let bytes = s.as_bytes();
	let mut last = 0;

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			if last > 0 {
				return &s[last+1..i]
			} else {
				last = i
			}
		}
	}

	&s[last+1..]
}
