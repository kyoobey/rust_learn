
use std::{fs, env};
use std::error::Error;


pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_sensitive: bool
}

impl Config {
	// pub fn new(args: &[String]) -> Result<Config, &'static str> {
	// 	if args.len() < 3 {
	// 		return Err("Not enough arguments!");
	// 	}

	// 	let query = args[1].clone();
	// 	let filename = args[2].clone();

	// 	let mut case_sensitive = env::var("MINIGREP_CASE_INSENSITIVE").is_err();

	// 	if args.len() > 3 {
	// 		if args.contains(&String::from("--case-insensitive")) {
	// 			case_sensitive = false;
	// 		} else if args.contains(&String::from("--case-sensitive")) {
	// 			case_sensitive = true;
	// 		}
	// 	}

	// 	Ok(Config { query, filename, case_sensitive })
	// }

	pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
		args.next();

		let query = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a query string")
		};

		let filename = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a file name")
		};

		let mut case_sensitive = env::var("MINIGREP_CASE_INSENSITIVE").is_err();
		let v = args.collect::<Vec<String>>();
		if v.contains(&String::from("--case-insensitive")) {
			case_sensitive = false;
		} else if v.contains(&String::from("--case-sensitive")) {
			case_sensitive = true;
		}

		Ok(Config { query, filename, case_sensitive })
	}
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	// let contents = fs::read_to_string(config.filename)
	// 					.expect("Something went wrong reading file");
	let contents = fs::read_to_string(config.filename)?;
	// println!("With text:\n{}", contents);

	// for line in search(&config.query, &contents) {
	// 	println!("{}", line);
	// }

	let results = if config.case_sensitive {
		search(&config.query, &contents)
	} else {
		search_case_insensitive(&config.query, &contents)
	};

	for line in results {
		println!("{}", &line);
	}

	Ok(())
}


pub fn search<'a>(query: & str, contents: &'a str) -> Vec<&'a str> {
	// let mut results: Vec<&str> = vec![];

	// for line in contents.lines() {
	// 	if line.contains(query) {
	// 		results.push(line);
	// 	}
	// }

	// results

	contents
		.lines()
		.filter(|line| line.contains(query))
		.collect()
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	// let mut results: Vec<&str> = vec![];

	// for line in contents.lines() {
	// 	if line.to_lowercase().contains(&query) {
	// 		results.push(line);
	// 	}
	// }

	// results

	contents
		.lines()
		.filter(|line| line.to_lowercase().contains(&query))
		.collect()
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn case_sensitive_1() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}

	#[test]
	fn case_sensitive_2() {
		let query = "Duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		let empty_str_vec: Vec<&str> = vec![];
		assert_eq!(empty_str_vec, search(query, contents));
	}

	#[test]
	fn case_insensitive() {
		let query = "rUst";
		let contents = "\
Rust:
safe, fast, productive.
Trust me.";

		assert_eq!(
			vec!["Rust:","Trust me."],
			search_case_insensitive(query, contents)
		);
	}
}
