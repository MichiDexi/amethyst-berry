use crossterm::{
	execute,
	cursor::MoveTo,
};
use std::io::{stdout, Write, Stdout};

use crate::helpers::utils;
use crate::helpers::input;

pub struct Slider {
	// Size and position
	pub x : u16, pub y : u16,
	pub size : u16,

	// Extra options
	pub colorset : [utils::Color; 4],
	pub color_bg : utils::Color,
	pub charset : [char; 4],

	// Event polling
	pub hovered : bool,
	pub selected : u8,
}


impl Slider {
	pub fn new(
		nx : u16, ny : u16,
		nsize : u16
	) -> Self {
		Slider {
			x : nx, y : ny,
			size : nsize,
			
			colorset : [
				utils::Color {
					color_enabled : true,
					color : 7,
					bright : false,

					truecolor : false,
					red : 0,
					green : 0,
					blue : 0,
				},
				utils::Color {
					color_enabled : true,
					color : 7,
					bright : false,

					truecolor : false,
					red : 0,
					green : 0,
					blue : 0,
				},
				utils::Color {
					color_enabled : true,
					color : 7,
					bright : false,

					truecolor : false,
					red : 0,
					green : 0,
					blue : 0,
				},
				utils::Color {
					color_enabled : true,
					color : 7,
					bright : false,

					truecolor : false,
					red : 0,
					green : 0,
					blue : 0,
				},
			],



			color_bg : utils::Color {
				color_enabled : false,
				color : 0,
				bright : false,

				truecolor : false,
				red : 0,
				green : 0,
				blue : 0,
			},

			charset : ['<', '|', '-', '>'],

			hovered : false,
			selected : 0
		}
	}
	
	pub fn draw(&self, out : &mut Stdout) {
	
		self.color_bg.write_color(out, true);

		// Left of the bar
		execute!(out, crossterm::cursor::MoveTo(self.x, self.y)).unwrap();
		self.colorset[0].write_color(out, false);
		
		write!(out, "{}", self.charset[0]).unwrap();

		// Background
		self.colorset[2].write_color(out, false);
		utils::repeat(out, self.charset[2], self.size);

		// Right of the bar
		self.colorset[3].write_color(out, false);
		write!(out, "{}", self.charset[3]).unwrap();

		// Selector
		execute!(out, crossterm::cursor::MoveTo(self.x +1 +self.selected as u16, self.y)).unwrap();
		self.colorset[1].write_color(out, false);
		write!(out, "{}", self.charset[1]).unwrap();

		write!(out, "\x1b[0m").unwrap();

		stdout().flush().unwrap();
	}

	pub fn update(&mut self, mouse : &input::Mouse) {
		self.hovered = utils::check_collision(
			self.x, self.y,
			self.size, 1,
			mouse.x, mouse.y
		);

		if self.hovered && mouse.lclickheld {
			self.selected = (mouse.x - self.x) as u8;
		}
	}
}
