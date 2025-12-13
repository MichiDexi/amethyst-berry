use crossterm::{
	execute,
	cursor::MoveTo,
};
use std::io::{stdout, Write};

use crate::helpers::utils;

pub struct SplitBox {
	// Size and position
	pub x : u16, pub y : u16,
	pub width : u16, pub height : u16,

	// Contents of the box
	pub text : String,

	// Extra options
	pub color : u8,
	pub line_type : u8,

	// Splits
	pub horizontal : Vec<u16>,
	pub vertical : Vec<u16>,
}


impl SplitBox {
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


		// Vertical splits
		for line in self.vertical.clone() {
			execute!(out, MoveTo(self.x + line, self.y)).unwrap();
			write!(out, "{}", utils::get_char(self.line_type, 8)).unwrap();
			for i in 0..self.height-2 {
				execute!(out, MoveTo(self.x + line, self.y + i + 1)).unwrap();
				write!(out, "{}", utils::get_char(self.line_type, 1)).unwrap();
			}
			execute!(out, MoveTo(self.x + line, self.y + self.height - 1)).unwrap();
			write!(out, "{}", utils::get_char(self.line_type, 9)).unwrap();
		}

		// Horizontal splits
		for line in self.horizontal.clone() {
			execute!(out, MoveTo(self.x, self.y + line)).unwrap();
			write!(out, "{}", utils::get_char(self.line_type, 6)).unwrap();
			for i in 0..self.width-2 {
				execute!(out, MoveTo(self.x + i + 1, self.y + line)).unwrap();
				if self.vertical.contains(&(i+1)) {
					write!(out, "{}", utils::get_char(self.line_type, 10)).unwrap();
				}
				else {
					write!(out, "{}", utils::get_char(self.line_type, 0)).unwrap();
				}
			}
			execute!(out, MoveTo(self.x + self.width - 1, self.y + line)).unwrap();
			write!(out, "{}", utils::get_char(self.line_type, 7)).unwrap();
		}

		stdout().flush().unwrap();
	}
}
