use std::io;
use std::io::Write;
use std::process;
use std::collections::HashMap;
use regex::Regex;



/*
 * chapter 8 assignment 2
 * user interface
 */

fn main() {
	println!("Chpater 8 assignment 3");
	println!("user interface");
	println!();


	println!("Syntax: Add <name> to <department>, s to show data, q to quit");

	let re = Regex::new(r"^Add (?P<name>\w+) to (?P<department>\w+)").unwrap();
	let mut map: HashMap<String, Vec<String>> = HashMap::new();

	loop {
		print!("Enter data: ");
		io::stdout().flush().unwrap();

		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");
		input = String::from(input.trim());

		let (head, _) = input.split_at(1);

		if head.contains("s") {
			for (department, vec) in &map {
				println!("Department: {}", department);
				print!("Names: ");
				for name in vec {
					print!("{}, ", name);
				}
				println!();
				// io::stdout().flush().unwrap();
			}
			continue;
		}

		if head.contains("q") {
			process::exit(0);
		}

		if re.is_match(&mut input) {

			let captures = re.captures(&input).unwrap();

			let v = map.entry(String::from(&captures["department"])).or_insert(vec![]);
			v.push(String::from(&captures["name"]));

		} else {
			println!("invalid input, try again!");
		}
	}
}

