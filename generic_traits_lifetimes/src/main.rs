use std::result;





fn main() {
	// let list = vec![34, 50, 25, 100, 65];
	// let mut largest = list[0];
	// for num in list {
	// 	if num > largest {
	// 		largest = num;
	// 	}
	// }
	// println!("The largest number is {:?}", &largest);

	// let list2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
	// let mut largest2 = list2[0];
	// for num in list2 {
	// 	if num > largest2 {
	// 		largest2 = num;
	// 	}
	// }
	// println!("The largest number is {:?}", &largest2);


	// let list = vec![34, 50, 25, 100, 65];
	// let result = largest_i32(&list);
	// println!("The result number is {}", result);

	// let list2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
	// let result2 = largest_i32(&list2);
	// println!("The result number is {}", result2);



	let list3 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
	let result3 = largest(&list3);
	println!("The largest int is {:?}", result3);

	let list4 = vec!['m', 'e', 'y', 'q'];
	let result4 = largest(&list4);
	println!("The largest char is {:?}", result4);


	let p1 = Point {x: 5, y: 10.0};
	println!("p1.x = {:?}", p1.x());
	let p2 = Point {x: "Hello", y: 'c'};
	let p3 = p1.mixup(p2);
	println!("p3.x = {:?} and p3.y = {:?}", p3.x, p3.y);
}


// fn largest_i32(list: &[i32]) -> i32 {
// 	let mut largest = list[0];
// 	for &num in list {
// 		if num > largest {
// 			largest = num;
// 		}
// 	}
// 	largest
// }

// fn largest_char(list: &[char]) -> char {
// 	let mut largest = list[0];
// 	for &num in list {
// 		if num > largest {
// 			largest = num;
// 		}
// 	}
// 	largest
// }

// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
// 	let mut largest = list[0];
// 	for &num in list {
// 		if num > largest {
// 			largest = num;
// 		}
// 	}
// 	largest
// }

fn largest<T: PartialOrd>(list: &[T]) -> &T {
	let mut largest_index = 0;
	for idx in 1..list.len() {
		if list[idx] > list[largest_index] {
			largest_index = idx;
		}
	}
	&list[largest_index]
}


struct Point<T, U> {
	x: T,
	y: U
}

impl<T, U> Point<T, U> {
	fn x(&self) -> &T {
		&self.x
	}
}

impl Point<f32, f32> {
	fn distance_from_origin(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}

impl<X1, Y1> Point<X1, Y1> {
	fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
		Point {
			x: self.x,
			y: other.y
		}
	}
}



