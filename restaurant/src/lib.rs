


mod front_of_house;


fn serve_order() {}

mod back_of_house {
	pub fn fix_incorrect_order() {
		cook_order();
		super::serve_order();
	}

	fn cook_order() {}

	#[derive(Debug)]
	pub struct Breakfast {
		pub toast: String,
		seasonal_fruit: String
	}

	impl Breakfast {
		pub fn summer(toast: &str) -> Breakfast {
			Breakfast {
				toast: String::from(toast),
				seasonal_fruit: String::from("peaches")
			}
		}
	}

	#[derive(Debug)]
	pub enum Appetizer {
		Soup,
		Salad,
	}
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
	// absolute path
	// crate::front_of_house::hosting::add_to_waitlist();
	// relative path
	// front_of_house::hosting::add_to_waitlist();


	let mut meal = back_of_house::Breakfast::summer("Rye");
	meal.toast = String::from("wheat");
	println!("I'd like {} toast please", meal.toast);
	println!("meal: {:?}", meal);

	let order1 = back_of_house::Appetizer::Soup;
	let order2 = back_of_house::Appetizer::Salad;
	println!("{:?} {:?}", order1, order2);

	hosting::add_to_waitlist();
	hosting::add_to_waitlist();
	hosting::add_to_waitlist();

	back_of_house::fix_incorrect_order();
}

