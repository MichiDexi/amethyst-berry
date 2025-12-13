use crossterm::{
	execute,
	cursor::MoveTo,
};
use std::io::{stdout, Write};

use crate::utils;
use crate::input;

pub struct Box {
	// Size and position
	pub x : u16, pub y : u16,
	pub width : u16, pub height : u16,

	// Contents of the box
	pub text : String,

	// Extra options
	pub invert_on_hover : bool,
	pub color : u8,
	pub line_type : u8,

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
		write!(out, "{}", utils::get_char(self.line_type, 2)).unwrap();
		utils::repeat(&mut out, utils::get_char(self.line_type, 0), self.width-2);
		write!(out, "{}", utils::get_char(self.line_type, 3)).unwrap();

		// Middle
		for i in 0..self.height-2 {
			execute!(out, MoveTo(self.x, self.y + i + 1)).unwrap();
			write!(out, "{}", utils::get_char(self.line_type, 1)).unwrap();
			execute!(out, MoveTo(self.x + self.width - 1, self.y + i + 1)).unwrap();
			write!(out, "{}", utils::get_char(self.line_type, 1)).unwrap();
		}

		// Bottom
		execute!(out, MoveTo(self.x, self.y + self.height - 1)).unwrap();
		write!(out, "{}", utils::get_char(self.line_type, 4)).unwrap();
		utils::repeat(&mut out, utils::get_char(self.line_type, 0), self.width-2);
		write!(out, "{}", utils::get_char(self.line_type, 5)).unwrap();

		stdout().flush().unwrap();
	}

	pub fn update(&mut self, mouse : &input::Mouse) {

		let hovered = mouse.x >= self.x &&
			mouse.x < self.x + self.width &&
			mouse.y >= self.y &&
			mouse.y < self.y + self.height;

		self.hovered = hovered;

		if hovered {
			self.clicked = mouse.lclick;
			self.rclicked = mouse.rclick;
			return;
		}
		
		self.clicked = false;
		self.rclicked = false;
	}
}
