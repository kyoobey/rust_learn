

fn main() {
	let number = 3;

	if {
		let number = number + 1;
		number < 5
	} {
		println!("number less than 4");
	} else {
		println!("number greater than or equal to 4");
	}

	let condition = true;
	let number_2 = if condition { 5 } else { 6 };
	println!("The value of number_2 is: {}", number_2);
}
