use crossterm::{
	terminal::size,
	execute,
	cursor,
	event::KeyCode
};
use std::{
	// env, <- Will be important later
	io::{
		stdout,
		Write
	},
	thread::sleep
};

mod input;


fn main() {
	input::init();
	let mut a : input::Mouse = input::Mouse { x : 0, y : 0, lclick : false };
	let mut b : input::Window = input::Window { focused : false, width : 0, height : 0 };
	loop {
		let _ = input::update(&mut a, &mut b);
		let mut stdout = stdout();
		if a.lclick {
			execute!(stdout, cursor::MoveTo(a.x, a.y)).unwrap();
			let _ = write!(stdout, "a");
		}
		let _ = stdout.flush();
	}
}
