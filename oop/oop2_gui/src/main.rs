

use oop2_gui::Screen;
use oop2_gui::Draw;


struct SelectBox {
	width: u32,
	height: u32,
	options: Vec<String>
}

impl Draw for SelectBox {
	fn draw(&self) {
	    // 
	}
}


fn main() {
	let screen = Screen {
		components: vec![Box::new(String::from("Hi"))]
	};

	screen.run();
}

