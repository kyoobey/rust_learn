


pub fn internal_phrases_version () -> &'static str {
	"0.0.0"
}

pub fn version () -> &'static str {
	"0.0.0"
}



extern crate phrases;
pub use phrases::{
	english::{
		hello as en_hello,
		goodbye as en_goodbye,
	},
	japanese:: {
		hello as jp_hello,
		goodbye as jp_goodbye
	}
};


