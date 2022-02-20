use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::mem;



// can i get some tips ?

/*
 * chapter 8 assignment 1
 * mean, median and mode
 */

fn main() {
	println!("chapter 8 assignment 1");
	println!("mean, median and mode");


	// length of numbers vector
	let mut length: u32;
	let mut _n = String::new();
	loop {
		print!("Enter the number of numbers (natural number): ");
		io::stdout().flush().unwrap();	

		io::stdin()
			.read_line(&mut _n)
			.expect("Failed to read line");

		let mut _valid = false;
		length = match _n.trim().parse() {
			Ok(num) => {
				_valid = true;
				num
			},
			Err(_) => {
				println!("invalid input entered, try again!");
				continue;
			}
		};

		if length < 1 {
			println!("invalid input entered, try again!");
			continue;
		}

		if _valid {break}
	}


	// store numbers in vector
	let mut numbers = vec![];
	for idx in 0..length {
		let mut i: f32;
		let mut _i = String::new();
		loop {
			print!("Enter the {}th numebr (float): ", idx+1);
			io::stdout().flush().unwrap();

			io::stdin()
				.read_line(&mut _i)
				.expect("Failed to read line");

			let mut _valid = false;
			i = match _i.trim().parse() {
				Ok(num) => {
					_valid = true;
					num
				},
				Err(_) => {
					println!("invalid input entered, try again!");
					continue;
				}
			};

			if _valid {break}
		}

		numbers.push(i);
	}

	println!();


	// calculate and show mean/average
	let mut mean: f32 = 0.0;
	for number in &numbers {
		mean += number;
	}
	mean /= length as f32;
	println!("Mean (average): {}", mean);


	// calculate and show median/middle number
	numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

	let mut median: f32 = 0.0;
	match length % 2 {
		1 => {
			if let Some(num) = numbers.get((length/2) as usize) {
				median = *num;
			}
		}
		0 => {
			let mut a = 0.0;
			if let Some(num) = numbers.get((length/2-1) as usize) {
				a = *num;
			}
			let mut b = 0.0;
			if let Some(num) = numbers.get((length/2) as usize) {
				b = *num;
			}
			median = (a+b) as f32 / 2.0;
		},
		_ => {}
	}

	println!("Median (middle number): {}", median);


	// calculate and show mode
	let mut map: HashMap<Distance, u32> = HashMap::new();

	for number in numbers {
		let count = map.entry(Distance::new(number)).or_insert(0);
		*count += 1;
	}

	let mut mode = Distance::new(0.0);
	let mut last_highest_value = 0;
	for (key, value) in map {
		if value > last_highest_value {
			mode = key;
			last_highest_value = value;
		}
	}
	println!("Mode (most occuring number): {}", mode.to_float());

}




#[derive(Debug, Hash, Eq, PartialEq)]
struct Distance((u64, i16, i8));

impl Distance {
	fn new(val: f32) -> Distance {
		Distance(integer_decode(val))
	}

	fn to_float(self) -> f32 {
		let sign_f = self.0.2 as f32;
		let mantissa_f = self.0.0 as f32;
		let exponent_f = (2.0f32).powf(self.0.1 as f32);
		sign_f * mantissa_f * exponent_f
	}
}

// Returns the mantissa, exponent and sign as integers.
fn integer_decode(num: f32) -> (u64, i16, i8) {
	let bits: u32 = unsafe { mem::transmute(num) };
	let sign: i8 = if bits >> 31 == 0 { 1 } else { -1 };
	let mut exponent: i16 = ((bits >> 23) & 0xff) as i16;
	let mantissa = if exponent == 0 {
		(bits & 0x7fffff) << 1
	} else {
		(bits & 0x7fffff) | 0x800000
	};
	// Exponent bias + mantissa shift
	exponent -= 127 + 23;
	(mantissa as u64, exponent, sign)
}

