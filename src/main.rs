use crossterm::{
	execute,
	cursor,
};
use std::{
	// env, <- Will be important later
	io::{
		stdout,
		Write
	},
};
use std::{thread, time};

pub mod input;
pub mod interface;
pub mod utils;

fn main() {
	input::init().unwrap();
	let mut mouse : input::Mouse = input::Mouse { x : 0, y : 0, lclick : false, rclick : false };
	let mut window : input::Window = input::Window { focused : false, width : 0, height : 0 };

	let mut testobj : interface::splitbox::SplitBox =
	interface::splitbox::SplitBox {
		x : 10, y : 10,
		width : 41, height : 21,

		text : "".to_string(),
		
		color : 0,
		line_type : 0,
		
		horizontal : vec!(5, 10, 15),
		vertical : vec!(10, 20, 30),
	};

	for _ in 0..10 {
		testobj.draw();
		thread::sleep(time::Duration::from_millis(100));
	}
	
	let _ = input::uninit();
}
