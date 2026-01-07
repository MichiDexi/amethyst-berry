use crossterm::{
	execute,
	event::{
		KeyCode,
	},
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

pub struct InputField {
	// Size and position
	pub x : u16, pub y : u16,
	pub size : u16,
	pub text : String,

	// Extra options
	pub color : utils::Color,
	pub bgcolor : utils::Color,

	// Event polling
	pub hovered : bool,
	pub cursor : u16,
}


impl traits::UserInterface for InputField {
	fn draw(&self, out : &mut Stdout) {
		self.color.write_color(out, false);
		self.bgcolor.write_color(out, true);

		execute!(out, crossterm::cursor::MoveTo(self.x, self.y)).unwrap();
		utils::repeat(out, ' ', self.size);
		execute!(out, crossterm::cursor::MoveTo(self.x, self.y)).unwrap();
		write!(out, "{}", utils::shorten_text(&self.text, self.size)).unwrap();

		write!(out, "\x1b[0m").unwrap();

		stdout().flush().unwrap();
	}
	
	fn clear(&self, out : &mut Stdout) {
		execute!(out, crossterm::cursor::MoveTo(self.x, self.y)).unwrap();
		utils::repeat(out, ' ', self.size);

		stdout().flush().unwrap();
	}
	
	fn update(&mut self, input : &input::InputHandler) {
		self.hovered = utils::check_collision(
			self.x, self.y,
			self.size, 1,
			input.mouse.x, input.mouse.y
		);

		if self.hovered {
			// Cursor Movement
			if input.keyboard.just_pressed(KeyCode::Left) && self.cursor != 0 {
				self.cursor -= 1;
			}
			if input.keyboard.just_pressed(KeyCode::Right) && self.cursor != self.text.len() as u16 {
				self.cursor += 1;
			}

			// Special keys
			if input.keyboard.just_pressed(KeyCode::Backspace) && self.cursor != 0 {
				self.text.remove((self.cursor - 1) as usize);
				self.cursor -= 1;
			}
			if input.keyboard.just_pressed(KeyCode::Delete) && self.cursor != self.text.len() as u16 {
				self.text.remove((self.cursor) as usize);
			}

			for key in input.keyboard.just_pressed.iter() {
				if let KeyCode::Char(c) = key {
					self.text.insert(self.cursor as usize, *c);
					self.cursor += 1;
				}
			}
		}
	}

	fn is_hovered(&self) -> bool {
		self.hovered
	}

	fn set_position(&mut self, x : u16, y : u16) {
		self.x = x;
		self.y = y;
	}

	fn set_color(&mut self, color : bool) {
		self.color.color_enabled = color;
	}
}

impl InputField {
	fn new(nx : u16, ny : u16, ntext : u16) -> Self {
		InputField {
			x : nx,
			y : ny,
			size : ntext,
			text : "".to_string(),

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
			
			hovered : false,
			cursor : 0
		}
	}
}
