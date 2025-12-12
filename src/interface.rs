use crossterm::{
	ExecutableCommand, execute,
	cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition}
};
use std::io;
use std::io::{stdout, Write, Stdout};

pub struct Box {
	// Size and position
	pub x : u16, pub y : u16,
	pub width : u16, pub height : u16,

	// Contents of the box
	pub text : String,

	// Extra options
	pub invert_on_hover : bool,
	pub color : u8,

	// Event polling
	pub hovered : bool,
	pub clicked : bool,
	pub rclicked : bool,
}

impl Box {
	pub fn draw(&self) {
		let mut out = stdout();

		// Top
		execute!(out, crossterm::cursor::MoveTo(self.x, self.y)).unwrap();
		write!(out, "┌").unwrap();
		repeat(&mut out, '─', self.width-2);
		write!(out, "┐").unwrap();

		// Middle
		for i in 0..self.height-2 {
			execute!(out, MoveTo(self.x, self.y + i + 1)).unwrap();
			write!(out, "│").unwrap();
			execute!(out, MoveTo(self.x + self.width - 1, self.y + i + 1)).unwrap();
			write!(out, "│").unwrap();
		}

		// Bottom
		execute!(out, MoveTo(self.x, self.y + self.height - 1)).unwrap();
		write!(out, "└").unwrap();
		repeat(&mut out, '─', self.width-2);
		write!(out, "┘").unwrap();

		stdout().flush().unwrap();
	}

	pub fn update(&mut self, mouse : (u16, u16, bool, bool)) {

		let hovered = mouse.0 >= self.x &&
			mouse.0 < self.x + self.width &&
			mouse.1 >= self.y &&
			mouse.1 >= self.y + self.height;

		self.hovered = hovered;

		if hovered {
			self.clicked = mouse.2;
			self.rclicked = mouse.3;
			return;
		}
		
		self.clicked = false;
		self.rclicked = false;
	}
}

fn repeat(out : &mut Stdout, c : char, n : u16) {
	for _ in 0..n {
		write!(out, "{c}").unwrap();
	}
}
