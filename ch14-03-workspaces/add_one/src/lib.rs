
use std::ops::Add;
use num::One;
use rand;


pub fn add_one<T>(x: T) -> T
where
	T: Add<T, Output = T> + One
{
	x + One::one()
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		assert_eq!(3, add_one(2));
	}
}
