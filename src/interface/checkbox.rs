use crossterm::{
	execute,
};
pub use std::io::{
	Stdout,
	Write,
	stdout,
	self,
};

use crate::helpers::utils;
use crate::helpers::input;
use crate::interface::traits;

pub struct CheckBox {
	// Size and position
	pub x : u16, pub y : u16,

	// Extra options
	pub color : utils::Color,
	pub bgcolor : utils::Color,
	pub charset : [char; 4],

	// Event polling
	pub hovered : bool,
	pub checked : bool
}


impl traits::UserInterface for CheckBox {
	fn draw(&self, out : &mut Stdout) {

		self.color.write_color(out, false);
		self.bgcolor.write_color(out, true);

		execute!(out, crossterm::cursor::MoveTo(self.x, self.y)).unwrap();
		write!(out, "{}", self.charset[0]).unwrap();

		if self.checked {
			write!(out, "{}", self.charset[1]).unwrap();
		}
		else {
			write!(out, "{}", self.charset[2]).unwrap();
		}
		write!(out, "{}", self.charset[3]).unwrap();

		write!(out, "\x1b[0m").unwrap();

		stdout().flush().unwrap();
	}

	fn clear(&self, out : &mut Stdout) {
		execute!(out, crossterm::cursor::MoveTo(self.x, self.y)).unwrap();
		utils::repeat(out, self.charset[0], 3);

		stdout().flush().unwrap();
	}

	fn update(&mut self, input : &input::InputHandler) {
		self.hovered = utils::check_collision(
			self.x, self.y,
			3, 1,
			input.mouse.x, input.mouse.y
		);

		if self.hovered && input.mouse.lclick {
			self.checked = !self.checked;
		}
	}
}

impl CheckBox {
	fn new(nx : u16, ny : u16) -> Self {
		CheckBox {
			x : nx,
			y : ny,

			color : utils::Color {
				color_enabled : true,
			
				color : 0,
				bright : false,
				
				truecolor : false,
				red : 0,
				green : 0,
				blue : 0,
			},

			bgcolor : utils::Color {
				color_enabled : true,
						
				color : 7,
				bright : false,
							
				truecolor : false,
				red : 0,
				green : 0,
				blue : 0,
			},

			charset : ['[', 'X', ' ', ']'],
			
			hovered : false,
			checked : false
		}
	}
}
