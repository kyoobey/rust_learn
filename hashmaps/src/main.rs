
use std::collections::HashMap;


fn main() {

	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);

	println!("{:?}", scores);


	let teams2 = vec![String::from("Blue"), String::from("Yellow")];
	let initial_scores2 = vec![10, 50];

	let mut scores2: HashMap<_, _> =
		teams2.into_iter().zip(initial_scores2.into_iter()).collect();


	let field_name_1 = String::from("Favorite color");
	let field_value_1 = String::from("Blue");

	let mut map_1 = HashMap::new();
	map_1.insert(field_name_1, field_value_1);

	// println!("{} {}", field_name_1, field_value_1);


	let mut scores3 = HashMap::new();
	scores3.insert(String::from("Blue"), 10);
	scores3.insert(String::from("Yellow"), 50);

	let team_name3 = String::from("Blue");
	let score3 = scores3.get(&team_name3);
	// println!("{:?}", score3);
	if let Some(score) = score3 {
		println!("{}", score);
	}

	for (key, value) in &scores3 {
		println!("{}: {}", &key, &value);
	}


	let mut scores4 = HashMap::new();
	scores4.insert(String::from("Blue"), 10);
	scores4.insert(String::from("Blue"), 25);
	println!("{:?}", scores4);

	scores4.entry(String::from("Yellow")).or_insert(50);
	scores4.entry(String::from("Blue")).or_insert(50);
	println!("{:?}", scores4);


	let text5 = "hello world wonderful world";
	let mut map5 = HashMap::new();
	for word in text5.split_whitespace() {
		let count = map5.entry(word).or_insert(0);
		*count += 1;
	}
	println!("{:?}", map5);


}

