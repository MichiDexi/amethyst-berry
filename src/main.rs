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

	let mut testobj : interface::progressbar::ProgressBar =
	interface::progressbar::ProgressBar {
		x : 10, y : 10,
		size : 40,

		progress_full : 0,
		percentage_show : 0,
		progress_max : 1000,
		
		line_type : 0,
		color : utils::Color {
			color : 0,
			bright : false
		},

		charset : ['<', '=', ' ', '>'],
	};

	for _ in 0..1000 {
		testobj.draw();
		thread::sleep(time::Duration::from_millis(5));
		testobj.progress_full += 1;
	}

	


	
	let _ = input::uninit();
}
