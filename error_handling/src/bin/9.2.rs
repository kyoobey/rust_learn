use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{ErrorKind, self, Read};


fn main() -> Result<(), Box<dyn Error>> {
	// panic!("crash and burn!");


	// let v = vec![1, 2, 3];
	// v[99];


	const FILE_NAME: &str = "hello.txt";
	// let f = File::open(FILE_NAME);

	// let f = match f {
	// 	Ok(file) => file,
	// 	Err(error) => panic!("[ERROR] Problem opening file: {:?}", error)
	// };

	// let f = match f {
	// 	Ok(file) => file,
	// 	Err(error) => match error.kind() {
	// 		ErrorKind::NotFound => match File::create(FILE_NAME) {
	// 			Ok(fc) => fc,
	// 			Err(e) => panic!("Problem creating the file: {:?}", e)
	// 		},
	// 		other_error => {
	// 			panic!("Problem opening the file: {:?}", other_error)
	// 		}
	// 	}
	// };

	// let f = File::open(FILE_NAME).unwrap_or_else(|error| {
	// 	if error.kind() == ErrorKind::NotFound {
	// 		File::create(FILE_NAME).unwrap_or_else(|e| {
	// 			panic!("Problem creating the file: {:?}", e);
	// 		})
	// 	} else {
	// 		panic!("Problem opening the file: {:?}", error);
	// 	}
	// });



	// let f2 = File::open(FILE_NAME)
	// 					.expect("Failed to open file");


	// read_username_from_file(FILE_NAME);



	let f = File::open(FILE_NAME)?;

	Ok(())

}



// fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
// 	let f = File::open(file_name);

// 	let mut f = match f {
// 		Ok(file) => file,
// 		Err(e) => return Err(e)
// 	};

// 	let mut s = String::new();

// 	match f.read_to_string(&mut s) {
// 		Ok(_) => Ok(s),
// 		Err(e) => Err(e)
// 	}
// }

// fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
// 	let mut f = File::open(file_name)?;
// 	let mut s = String::new();
// 	f.read_to_string(&mut s)?;
// 	Ok(s)
// }

// fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
// 	let mut s = String::new();
// 	File::open(file_name)?.read_to_string(&mut s)?;
// 	Ok(s)
// }

fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
	fs::read_to_string(file_name)
}

