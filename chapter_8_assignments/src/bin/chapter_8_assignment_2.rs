use std::io;
use std::io::Write;




/*
 * chapter 8 assignment 2
 * pig latin
 */

fn main() {
	println!("chapter 8 assignment 2");
	println!("pig latin");
	println!();


	let mut word = String::new();

	loop {
		print!("Enter word (word length > 0): ");
		io::stdout().flush().unwrap();

		io::stdin()
			.read_line(&mut word)
			.expect("Failed to read line");

		word = String::from(word.trim());

		if word == String::from("") || word.contains(" ") {
			println!("invalid input, try again!");
		} else {
			break;
		}
	}

	if let Some(f) = word.get(0..1) {
		match "aeiou".contains(f) {
			true => {
				word.push_str("-hay");
			},
			false => {
				// word = String::from(format!("{}-{}ay", word[1..word.len()], word[0..1]));
				// word = String::from(word[1..word.len()])
				// word.push_str(format!("-{}ay", word[0..1]));
				let (head, tail) = word.split_at(1);
				word = format!("{}-{}ay", tail, head);
			}
		}
	}

	println!("{}!!", word);

}

