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
	let _ = input::init();
	let mut a : input::Mouse = input::Mouse { x : 0, y : 0, lclick : false, rclick : false };
	let mut b : input::Window = input::Window { focused : false, width : 0, height : 0 };
	loop {
		let _ = input::update(&mut a, &mut b);
		let mut stdout = stdout();
		if a.lclick {
			execute!(stdout, cursor::MoveTo(a.x, a.y)).unwrap();
			let _ = write!(stdout, "O");
			if a.x == 0 && a.y == 0 {
				break;
			}
		}
		if a.rclick {
			execute!(stdout, cursor::MoveTo(a.x, a.y)).unwrap();
			let _ = write!(stdout, " ");
			if a.x == 0 && a.y == 0 {
				break;
			}
		}
		let _ = stdout.flush();
	}
	let _ = input::uninit();
}
