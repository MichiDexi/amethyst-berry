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
pub mod helpers;

const TARGET_FPS : f32 = 30.0;

fn main() {
	helpers::input::init().unwrap();
	let framerate : Duration = Duration::from_secs_f32(1.0 / TARGET_FPS);

	let mut mouse : helpers::input::Mouse = helpers::input::Mouse { x : 0, y : 0, lclick : false, rclick : false, lclickheld : false, rclickheld : false };
	let mut window : helpers::input::Window = helpers::input::Window { focused : false, width : 0, height : 0 };

	let mut testobj : interface::textbox::Box =
	interface::textbox::Box {
		x : 20, y : 10,
		width : 21, height : 11,

		text : "".to_string(),
		
		color : 0,
		line_type : 0,

		clicked : false,
		rclicked : false,
		hovered : false,
		invert_on_hover : false,
	};

	let mut label : interface::label::Label =
	interface::label::Label {
		x : 13, y : 9,
		size : 33,

		text : "This button will stop the program".to_string(),

		color : helpers::utils::Color {
			color : 0,
			bright : false,

			truecolor : false,
			red : 0,
			green : 0,
			blue : 0,
		},
		bg_color : helpers::utils::Color {
			color : 0,
			bright : false,
			
			truecolor : false,
			red : 0,
			green : 0,
			blue : 0,
		},
	};

	
	

	let mut c : u16 = 0;

	loop {
		let now = Instant::now(); // Get frame time

		let mut out = stdout();

		helpers::input::update(&mut mouse, &mut window).unwrap();
		testobj.update(&mouse);
		if mouse.lclick {
			
			execute!(out, crossterm::cursor::MoveTo(0, 0)).unwrap();
			write!(out, "{}, {}", mouse.x, mouse.y).unwrap();
			
		}
		
		if testobj.clicked {
			break;
		}

		c += 1;
		if c == 10 {
			label.y -= 1;
		}
		if c == 20 {
			label.y += 1;
			c = 0;
		}
		print!("\x1b[2J");
		label.draw();
		testobj.draw();
		stdout().flush().unwrap();
		
		// Frame time management for consistent framerate
		let frame_duration = Instant::now().duration_since(now);
		if frame_duration < framerate {
			sleep(framerate - frame_duration);
		}
	}
	
	helpers::input::uninit().unwrap();
}
