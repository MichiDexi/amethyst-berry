use crossterm::{
	execute,
	cursor,
};
use std::{
	// env, <- Will be important later
	io::{
		stdout,
		Write,
	},
	thread::sleep,
	time::{
		Duration,
		Instant
	}
};

pub mod interface;
pub mod input;
pub mod utils;

const TARGET_FPS : f32 = 30.0;

fn main() {
	input::init().unwrap();
	let framerate : Duration = Duration::from_secs_f32(1.0 / TARGET_FPS);

	let mut mouse : input::Mouse = input::Mouse { x : 0, y : 0, lclick : false, rclick : false, lclickheld : false, rclickheld : false };
	let mut window : input::Window = input::Window { focused : false, width : 0, height : 0 };

	let mut testobj : interface::textbox::Box =
	interface::textbox::Box {
		x : 10, y : 10,
		width : 41, height : 21,

		text : "".to_string(),
		
		color : 0,
		line_type : 0,

		clicked : false,
		rclicked : false,
		hovered : false,
		invert_on_hover : false,
	};

	testobj.draw();

	loop {
		let now = Instant::now(); // Get frame time

		input::update(&mut mouse, &mut window).unwrap();
		testobj.update(&mouse);
		if mouse.lclick {
			let mut out = stdout();
			execute!(out, crossterm::cursor::MoveTo(0, 0)).unwrap();
			write!(out, "{}, {}", mouse.x, mouse.y).unwrap();
			stdout().flush().unwrap();
		}
		
		if testobj.clicked {
			break;
		}
		
		// Frame time management for consistent framerate
		let frame_duration = Instant::now().duration_since(now);
		if frame_duration < framerate {
			sleep(framerate - frame_duration);
		}
	}
	
	input::uninit().unwrap();
}
