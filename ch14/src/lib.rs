
//! # ch14
//!
//! `ch14` is a collection of utilities to ake performing certain
//! calculations more convenient


use std::ops::Add;
use num::One;


/// Add one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5.25;
/// let answer = ch14::add_one(arg);
///
/// assert_eq!(6.25, answer);
/// ```

pub fn add_one<T>(x: T) -> T
where
	T: Add<T, Output = T> + One
{
	x + One::one()
}


pub mod Art {
	//! # Art
	//!
	//! A library for modeling artistic concepts

	pub use kinds::PrimaryColor;
	pub use kinds::SecondaryColor;
	pub use utils::mix;

	pub mod kinds {
		/// The primary colors according to RYB color model.
		pub enum PrimaryColor {
			Red,
			Yellow,
			Blue
		}

		/// The secondary colors according to the RYB color model.
		pub enum SecondaryColor {
			Orange,
			Green,
			Purple
		}
	}

	pub mod utils {
		use super::kinds::*;

		/// Combines two primary colors in equal amounts to create
		/// a secondary color.
		pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
			unimplemented!();
		}
	}
}

