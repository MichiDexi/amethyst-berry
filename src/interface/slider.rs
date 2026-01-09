use crossterm::{
	execute,
};
use std::io::{stdout, Write, Stdout};

use crate::helpers::utils;
use crate::helpers::input;
use crate::interface::traits;

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


impl traits::UserInterface for Slider {
	fn draw(&self, out : &mut Stdout) {
	
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

	fn clear(&self, out : &mut Stdout) {
		// Left of the bar
		execute!(out, crossterm::cursor::MoveTo(self.x, self.y)).unwrap();
		utils::repeat(out, ' ', self.size+2);

		stdout().flush().unwrap();
	}
	
	fn update(&mut self, input : &input::InputHandler) {
		self.hovered = utils::check_collision(
			self.x, self.y,
			self.size, 1,
			input.mouse.x, input.mouse.y
		);

		if self.hovered && input.mouse.lclickheld {
			self.selected = (input.mouse.x - self.x) as u8;
		}
	}

	fn redraw_requested(&mut self) -> bool {
		self.hovered
	}

	fn set_position(&mut self, x : i16, y : i16, anchor : u8, size : (u16, u16)) {
		self.x = (x + if anchor == 1 || anchor == 3 { size.0 } else {0} as i16) as u16;
		self.y = (y + if anchor == 2 || anchor == 3 { size.1 } else {0} as i16) as u16;
	}

	fn set_color(&mut self, _ : bool) { }
}

impl Slider {
	fn new(nx : u16, ny : u16, nsize : u16) -> Self {
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
}
