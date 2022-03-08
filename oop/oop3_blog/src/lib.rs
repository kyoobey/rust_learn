

pub struct Post {
	state: Option<Box<dyn State>>,
	content: String,
	approve_count: u8
}

impl Post {
	pub fn new() -> Post {
		Post {
			state: Some(Box::new(Draft {})),
			content: String::new(),
			approve_count: 0
		}
	}

	// pub fn add_text(&mut self, text: &str) {
	// 	self.content.push_str(text);
	// }

	pub fn add_text(&mut self, text: &str) {
		self.content = self.state.as_ref().unwrap().add_text(&self.content, text);
	}

	pub fn content(&self) -> &str {
		self.state.as_ref().unwrap().content(self)
	}

	pub fn request_review(&mut self) {
		if let Some(s) = self.state.take() {
			self.state = Some(s.request_review())
		}
	}

	pub fn approve(&mut self) {
		if self.approve_count > 1 {
			if let Some(s) = self.state.take() {
				self.state = Some(s.approve())
			}
		} else {
			self.approve_count += 1
		}
	}
}


trait State {
	fn request_review(self: Box<Self>) -> Box<dyn State>;
	fn approve(self: Box<Self>) -> Box<dyn State>;
	fn reject(self: Box<Self>) -> Box<dyn State>;

	fn content<'a>(&self, post: &'a Post) -> &'a str {
		""
	}

	fn add_text(&self, original_text: &str, new_text: &str) -> String {
		original_text.to_string()
	}
}


struct Draft {}

impl State for Draft {
	fn request_review(self: Box<Self>) -> Box<dyn State> {
		Box::new(PendingReview {})
	}

	fn approve(self: Box<Self>) -> Box<dyn State> {
		self
	}

	fn reject(self: Box<Self>) -> Box<dyn State> {
		self
	}

	fn add_text(&self, original_text: &str, new_text: &str) -> String {
	    format!("{}{}", original_text, new_text)
	}
}


struct PendingReview {}

impl State for PendingReview {
	fn request_review(self: Box<Self>) -> Box<dyn State> {
		self
	}

	fn approve(self: Box<Self>) -> Box<dyn State> {
		Box::new(Published {})
	}

	fn reject(self: Box<Self>) -> Box<dyn State> {
		Box::new(Draft {})
	}
}


struct Published {}

impl State for Published {
	fn request_review(self: Box<Self>) -> Box<dyn State> {
		self
	}

	fn approve(self: Box<Self>) -> Box<dyn State> {
		self
	}

	fn content<'a>(&self, post: &'a Post) -> &'a str {
		&post.content
	}

	fn reject(self: Box<Self>) -> Box<dyn State> {
		self
	}
}

