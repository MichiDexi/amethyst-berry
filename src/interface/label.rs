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

pub struct Label {
	// Size and position
	pub x : u16, pub y : u16,
	pub size : u16,
	pub text : String,

	// Extra options
	pub color : utils::Color,
	pub bgcolor : utils::Color,

	// Event polling
	pub hovered : bool,
}


impl Label {
	pub fn new(nx : u16, ny : u16, nsize : u16, ntext : &str) -> Self {
		Label {
			x : nx,
			y : ny,
			size : nsize,
			text : ntext.to_string(),

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
		}
	}
	
	pub fn draw(&self, out : &mut Stdout) {

		self.color.write_color(out, false);
		self.bgcolor.write_color(out, true);

		execute!(out, crossterm::cursor::MoveTo(self.x, self.y)).unwrap();
		utils::repeat(out, ' ', self.size);
		execute!(out, crossterm::cursor::MoveTo(self.x, self.y)).unwrap();
		write!(out, "{}", utils::shorten_text(&self.text, self.size)).unwrap();

		write!(out, "\x1b[0m").unwrap();

		stdout().flush().unwrap();
	}

	pub fn update(&mut self, input : &input::InputHandler) {
		self.hovered = utils::check_collision(
			self.x, self.y,
			self.size, 1,
			input.mouse.x, input.mouse.y
		);
	}
}
