use crossterm::{
	execute,
	cursor::MoveTo,
};
use std::io::{stdout, Write};

use crate::helpers::utils;

pub struct Label {
	// Size and position
	pub x : u16, pub y : u16,
	pub size : u16,
	pub text : String,

	// Extra options
	pub color : utils::Color,
	pub bgcolor : utils::Color,
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
		}
	}
	
	pub fn draw(&self) {
		let mut out = stdout();

		self.color.write_color(&mut out, false);
		self.bgcolor.write_color(&mut out, true);

		execute!(out, crossterm::cursor::MoveTo(self.x, self.y)).unwrap();
		write!(out, "{}", utils::shorten_text(&self.text, self.size)).unwrap();

		write!(out, "\x1b[0m").unwrap();

		stdout().flush().unwrap();
	}
}
