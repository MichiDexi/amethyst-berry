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

	let mut testbox : interface::textbox::Box =
	interface::textbox::Box {
		x : 10, y : 10,
		width : 40, height :18,

		text : "".to_string(),
		invert_on_hover : false,
		hovered : false,
		clicked : false,
		rclicked : false,
		line_type : 0,
		color : 0
	};

	testbox.draw();
	thread::sleep(time::Duration::from_millis(60000));
	


	
	let _ = input::uninit();
}
