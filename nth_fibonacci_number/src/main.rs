
use std::io;

fn main() {

	loop {

		println!("Enter a positive integer (q to exit):");

		let mut n = String::new();

		io::stdin()
			.read_line(&mut n)
			.expect("Failed to read line");

		n = String::from(n.trim());
		if n == "q" {
			break;
		}

		let n: u32 = match n.parse() {
			Ok(num) => num,
			Err(_) => {
				println!("invalid number");
				continue
			}
		};

		println!("{}", fibonacci_number(n));

	}

}



fn fibonacci_number(n: u32) -> u32 {
	let mut last_number: u32 = 0;
	let mut current_number: u32 = 1;
	for _ in 1..n-1 {
		let new_number = last_number + current_number;
		last_number = current_number;
		current_number = new_number;
	}
	current_number
}

