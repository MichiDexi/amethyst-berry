use crossterm::{
	execute,
	cursor::MoveTo,
};
use std::io::{
	Write,
	Stdout
};

use crate::helpers::utils;
use crate::helpers::input;
use crate::interface::traits;

pub struct SplitBox {
	// Size and position
	pub x : u16, pub y : u16,
	pub width : u16, pub height : u16,

	// Extra options
	pub color : utils::Color,
	pub line_type : u8,

	// Splits
	pub horizontal : Vec<u16>,
	pub vertical : Vec<u16>,

	// Event polling
	pub hovered : u8,
}


impl traits::UserInterface for SplitBox {
	fn draw(&self, out : &mut Stdout) {

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
	}

	fn clear(&self, out : &mut Stdout) {
		for i in 0..self.height {
			execute!(out, crossterm::cursor::MoveTo(self.x, self.y+i)).unwrap();
			utils::repeat(out, ' ', self.width);
		}
	}

	fn update(&mut self, input : &input::InputHandler) {

		let mut last_column : u16 = 0;
		let mut last_row : u16;
		let mut id : u8 = 0;

		let mut vertical = self.vertical.clone();
		vertical.push(self.height);
		let mut horizontal = self.vertical.clone();
		horizontal.push(self.width);

		for column in vertical.clone() {
			last_row = 0;
			for row in horizontal.clone() {
				if utils::check_collision(
					last_column+self.x,
					last_row+self.y,
					column - last_column,
					row - last_row,
					input.mouse.x, input.mouse.y
				) {
					self.hovered = id;
					return;
				}
				
				id += 1;
				last_row = row;
			}
			last_column = column;
		}
		self.hovered = 255;
	}

	fn redraw_requested(&mut self) -> bool {
		self.hovered != 255
	}

	fn set_position(&mut self, x : i16, y : i16, anchor : u8, size : (u16, u16)) {
		self.x = (x + if anchor == 1 || anchor == 3 { size.0 } else {0} as i16) as u16;
		self.y = (y + if anchor == 2 || anchor == 3 { size.1 } else {0} as i16) as u16;
	}

	fn set_color(&mut self, color : bool) {
		self.color.color_enabled = color;
	}
}

impl SplitBox {
	pub fn new(nx : u16, ny : u16, nwidth : u16, nheight : u16) -> Self {
		SplitBox {
			x : nx, y : ny,
			width : nwidth, height : nheight,

			color : utils::Color {
				color_enabled : false,
				color : 0,
				bright : false,

				truecolor : false,
				red : 0,
				green : 0,
				blue : 0,
			},

			line_type : 0,

			horizontal : vec!(nheight >> 1),
			vertical : vec!(nwidth >> 1),

			hovered : 255,
		}
	}
}
