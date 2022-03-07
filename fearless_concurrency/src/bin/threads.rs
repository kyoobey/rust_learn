

use std::thread;
use std::time::Duration;


fn main() {

	// let handle = thread::spawn(|| {
	// 	for i in 1..10 {
	// 		println!("hi number {} from spawned thread!", i);
	// 		thread::sleep(Duration::from_millis(i));
	// 	}
	// });

	// for i in 1..5 {
	// 	println!("hi number {} from the main thread!", i);
	// 	thread::sleep(Duration::from_millis(i));
	// }

	// handle.join().unwrap();


	let v = vec![1, 2, 3];

	let handle = thread::spawn(move || {
		println!("Here's a vector: {:?}", vs);
	});

	handle.join().unwrap();

}

