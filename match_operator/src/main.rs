



#[derive(Debug)]
enum UsState {
	Alabama,
	Alaska,
}

enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState)
}

fn value_in_cents(coin: &mut Coin) -> u8 {
	match coin {
		Coin::Penny => {
			println!("Lucky Penny!");
			1
		},
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("State quarter from {:?}", state);
			25
		},
	}
}


fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i+1)
	}
}


fn main() {
	let mut c = Coin::Quarter(UsState::Alaska);
	println!("{:?}", value_in_cents(&mut c));

	let x = Some(10);
	println!("{:?}", plus_one(x));

	let config_max = Some(3u8);
	// match config_max {
	// 	Some(max) => println!("The maximum is configured to be {:?}", max),
	// 	_ => ()
	// }
	if let Some(max) = config_max {
		println!("The maximum is configured to be {:?}", max);
	}

	let mut count = 0;
	// match c {
	// 	Coin::Penny => println!("Lucky Penny!"),
	// 	_ => count += 1
	// }
	if let Coin::Penny = c {
		println!("Lucky Penny");
	} else {
		count += 1;
	}
}


