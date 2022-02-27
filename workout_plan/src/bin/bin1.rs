use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;


// fn simulated_expensive_calculation(intensity: u32) -> u32 {
// 	println!("calculating slowly...");
// 	thread::sleep(Duration::from_secs(2));
// 	intensity
// }


fn generate_workout(intensity: u32, random_number: u32) {
	// if intensity < 25 {
	// 	println!(
	// 		"Today, do {} pushups!",
	// 		simulated_expensive_calculation(intensity)
	// 	);
	// 	println!(
	// 		"Today, do {} situps!",
	// 		simulated_expensive_calculation(intensity)
	// 	);
	// } else {
	// 	if random_number == 3 {
	// 		println!("Take a break today! Remember to stay hydrated!");
	// 	} else {
	// 		println!(
	// 			"Today, run for {} minutes!",
	// 			simulated_expensive_calculation(intensity)
	// 		);
	// 	}
	// }


	// let expensive_result = simulated_expensive_calculation(intensity);

	// if intensity < 25 {
	// 	println!("Today, do {} pushups!", expensive_result);
	// 	println!("Today, do {} situps!", expensive_result);
	// } else {
	// 	if random_number == 3 {
	// 		println!("Take a break today! Remember to stay hydrated!");
	// 	} else {
	// 		println!("Today, run for {} minutes!", expensive_result);
	// 	}
	// }


	// let expensive_closure = |num| {
	// let expensive_closure = |num: u32| -> u32 {
	// 	println!("calculating slowly...");
	// 	thread::sleep(Duration::from_secs(2));
	// 	num
	// };

	// if intensity < 25 {
	// 	println!("Today, do {} pushups!", expensive_closure(intensity));
	// 	println!("Today, do {} situps!", expensive_closure(intensity));
	// } else {
	// 	if random_number == 3 {
	// 		println!("Take a break today! Remember to stay hydrated!");
	// 	} else {
	// 		println!("Today, run for {} minutes!", expensive_closure(intensity));
	// 	}
	// }


	let mut expensive_result = Cacher::new(|num: &u32| -> u32 {
		println!("calculating slowly...");
		thread::sleep(Duration::from_secs(2));
		*num
	});

	if intensity < 25 {
		println!("Today, do {} pushups!", expensive_result.value(intensity));
		println!("Today, do {} situps!", expensive_result.value(intensity));
	} else {
		if random_number == 3 {
			println!("Take a break today! Remember to stay hydrated!");
		} else {
			println!("Today, run for {} minutes!", expensive_result.value(intensity));
		}
	}
}


struct Cacher<T, U, V>
where
	T: Fn(&U) -> V,            // i dont understand what should i do here... T: Fn(U) -> V ?
{
	calculation: T,
	// value: Option<u32>
	values: HashMap<U, V>
}

impl<T, U, V> Cacher<T, U, V>
where
	T: Fn(&U) -> V,
	U: Hash + Eq
{
	fn new(calculation: T) -> Cacher<T, U, V> {
		Cacher {
			calculation,
			// value: None
			values: HashMap::new()
		}
	}

	fn value(&mut self, arg: U) -> &V {
		// match self.value {
		// 	Some(v) => v,
		// 	None => {
		// 		let v = (self.calculation)(arg);
		// 		self.value = Some(v);
		// 		v
		// 	}
		// }

		if !self.values.contains_key(&arg) {
			let val = (self.calculation)(&arg);
			self.values.entry(arg).or_insert(val)
		} else {
			&self.values[&arg]
		}
	}
}


fn main() {
	let simulated_user_specified_number = 10;
	let simulated_random_number = 7;

	generate_workout(simulated_user_specified_number, simulated_random_number);
}

