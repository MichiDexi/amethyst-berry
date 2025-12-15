use crossterm::{
	execute,
	cursor::MoveTo,
};
use std::io::{stdout, Write, Stdout};

use crate::helpers::utils;
use crate::helpers::input;
use crate::interface::traits;

pub struct Box {
	// Size and position
	pub x : u16, pub y : u16,
	pub width : u16, pub height : u16,

	// Extra options
	pub color : utils::Color,
	pub color_hovered : utils::Color,
	pub line_type : u8,

	// Event polling
	pub hovered : bool,
}


impl traits::UserInterface for Box {
	fn draw(&self, out : &mut Stdout) {

		if self.hovered {
			self.color_hovered.write_color(out, false);
		}
		else {
			self.color.write_color(out, false);
		}
		

		// Top
		execute!(out, crossterm::cursor::MoveTo(self.x, self.y)).unwrap();
		write!(out, "{}", utils::get_char(self.line_type, 2)).unwrap();
		utils::repeat(out, utils::get_char(self.line_type, 0), self.width-2);
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
		utils::repeat(out, utils::get_char(self.line_type, 0), self.width-2);
		write!(out, "{}", utils::get_char(self.line_type, 5)).unwrap();

		write!(out, "\x1b[0m").unwrap();

		stdout().flush().unwrap();
	}

	fn clear(&self, out : &mut Stdout) {
		// Top
		execute!(out, crossterm::cursor::MoveTo(self.x, self.y)).unwrap();
		utils::repeat(out, ' ', self.width);

		// Middle
		for i in 0..self.height-2 {
			execute!(out, MoveTo(self.x, self.y + i + 1)).unwrap();
			write!(out, " ").unwrap();
			execute!(out, MoveTo(self.x + self.width - 1, self.y + i + 1)).unwrap();
			write!(out, " ").unwrap();
		}

		// Bottom
		execute!(out, MoveTo(self.x, self.y + self.height - 1)).unwrap();
		utils::repeat(out, ' ', self.width);

		stdout().flush().unwrap();
	}
	
	fn update(&mut self, input : &input::InputHandler) {

		self.hovered = utils::check_collision(
			self.x, self.y,
			self.width, self.height,
			input.mouse.x, input.mouse.y
		);
	}
}

impl Box {
	pub fn new(nx : u16, ny : u16, nwidth : u16, nheight : u16) -> Self {
		Box {
			x : nx, y : ny,
			width : nwidth, height : nheight,
			
			color : utils::Color {
				color_enabled : true,
				color : 7,
				bright : false,

				truecolor : false,
				red : 0,
				green : 0,
				blue : 0,
			},

			color_hovered : utils::Color {
				color_enabled : true,
				color : 3,
				bright : false,

				truecolor : false,
				red : 0,
				green : 0,
				blue : 0,
			},

			line_type : 0,

			hovered : false,
		}
	}
}
