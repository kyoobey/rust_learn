


struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64
}

fn build_user(email: &str, username: &str) -> User {
	let email = String::from(email);
	let username = String::from(username);
	User {
		// email: email,
		// username: username,
		email,
		username,
		active: true,
		sign_in_count: 1
	}
}


struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
    	self.width * self.height
    }

    fn width(&self) -> bool {
    	self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
    	self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
    	Rectangle {
    		width: size,
    		height: size
    	}
    }
}

// fn area(width: u32, height: u32) -> u32 {
// 	width * height
// }
// fn area(rect: (u32, u32)) -> u32 {
// 	rect.0 * rect.1
// }
fn area(rect: &Rectangle) -> u32 {
	rect.width * rect.height
}

// struct 


fn main() {

	// let user1 = User {
	// 	email: String::from("someone@example.com"),
	// 	username: String::from("someusername123"),
	// 	active: true,
	// 	sign_in_count: 1
	// };
	let user1 = build_user(
		"someone@example.com",
		"someusername123"
	);

	let user2 = User {
		email: String::from("another@example.com"),
		..user1
	};



	let black = Color(0, 0, 0);
	let origin = Point(0, 0, 0);


	// let width1 = 30;
	// let height1 = 50;

	// println!(
	// 	"The area of the rectangle is {} square pixels.",
	// 	area(width1, height1)
	// );

	// let rect1 = (30, 50);
	// println!(
	// 	"The area of the rectangle is {} square pixels.",
	// 	area(rect1)
	// );

	let scale = 1;
	let rect1 = Rectangle{
		width: dbg!(30 * scale),
		height: 50
	};
	let rect2 = Rectangle {
		width: 10,
		height: 40
	};
	let rect3 = Rectangle {
		width: 60,
		height: 45
	};
	let rect4 = Rectangle::square(50);
	println!(
		"The area of the rectangle is {} square pixels.",
		area(&rect1)
	);
	println!("rect1 is {:?}", rect1);
	dbg!(&rect1);

	println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
	println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}


