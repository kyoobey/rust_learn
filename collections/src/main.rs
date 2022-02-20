

fn main() {

	// let v: Vec<i32> = Vec::new();
	let v = vec![1,2,3,4,5];


	let third: &i32 = &v[2];
	println!("The third element is {:?}", third);

	let i = 5;
	// match v.get(i) {
	// 	Some(element) => println!("The {}th element is {}", i, element),
	// 	None => println!("There is no {}th element", i+1)
	// }
	if let Some(element) = v.get(i) {
		println!("The {}th element is {}", i+1, element);
	} else {
		println!("There is no {}th element", i+1);
	}


	// let mut v = vec![1,2,3,4,5];
	// let first = &v[0];
	// v.push(6);
	// println!("The first element is: {}", &first);


	let mut v2 = vec![100,32,57];
	for i in &mut v2 {
		*i += 50;
		println!("{}", i);
	}


	#[derive(Debug)]
	enum SpreadsheetCell {
		Int(i32),
		Float(f64),
		Text(String)
	}

	let row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Text(String::from("blue")),
		SpreadsheetCell::Float(10.12),
	];

}

