use std::cmp::{min, max};
use crossterm::{
	execute,
	cursor::MoveTo,
};
use std::io::{stdout, Write};

use crate::helpers::utils;

pub struct ProgressBar {
	// Size and position
	pub x : u16, pub y : u16,
	pub size : u16,

	// Extra options
	pub percentage_show : u8, // 0 -> Full numbers; 1 -> 1 decimal; 2 -> 2 decimal
	pub progress_full : u32, // How much of percentage_max is full
	pub progress_max : u32, // The max number the bar can show

	pub charset : [char; 4],
	pub colorset : [utils::Color; 4],
	pub bgcolorset : [utils::Color; 4],
	/*
		Charset:
		0 - Left of the progress bar
		1 - Filled part of the progress bar
		2 - Unfilled part of the progress bar
		3 - Right of the progress bar
	*/
}


impl ProgressBar {
	pub fn new(
		nx : u16, ny : u16, nsize : u16,
		nprogress_full : u32, nprogress_max : u32,
		ncharset : [char; 4] 
	) -> Self {

	ProgressBar {
			x : nx, y : ny,
			size : nsize,
	
			percentage_show : 0,
			progress_full : nprogress_full,
			progress_max : nprogress_max,
	
			charset : ncharset,
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
	
					color : 2,
					bright : false,
	
					truecolor : false,
					red : 0,
					green : 0,
					blue : 0,
				},
				utils::Color {
					color_enabled : true,
	
					color : 0,
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

			
			bgcolorset : [
				utils::Color {
					color_enabled : false,
					
					color : 4,
					bright : false,
					
					truecolor : false,
					red : 0,
					green : 0,
					blue : 0,
				},
				utils::Color {
					color_enabled : false,
			
					color : 5,
					bright : false,
			
					truecolor : false,
					red : 0,
					green : 0,
					blue : 0,
				},
				utils::Color {
					color_enabled : false,
			
					color : 6,
					bright : false,
			
					truecolor : false,
					red : 0,
					green : 0,
					blue : 0,
				},
				utils::Color {
					color_enabled : false,
														
					color : 7,
					bright : false,
														
					truecolor : false,
					red : 0,
					green : 0,
					blue : 0,
				},
			]
		}
	}

	pub fn draw(&self) {
		let mut out = stdout();

		// Left of the bar
		execute!(out, crossterm::cursor::MoveTo(self.x, self.y)).unwrap();
		self.colorset[0].write_color(&mut out, false);
		self.bgcolorset[0].write_color(&mut out, true);
		write!(out, "{}", self.charset[0]).unwrap();

		// Unfilled part
		self.colorset[2].write_color(&mut out, false);
		self.bgcolorset[2].write_color(&mut out, true);
		utils::repeat(&mut out, self.charset[2], self.size);

		// Right of the bar
		self.colorset[3].write_color(&mut out, false);
		self.bgcolorset[3].write_color(&mut out, true);
		write!(out, "{}", self.charset[3]).unwrap();

		// Calculation
		let percent : f64 = (self.progress_full as f64 / self.progress_max as f64).min(1.0);
		let bars_full_amount : u16 = (percent * self.size as f64) as u16;

		// Fill bar
		execute!(out, crossterm::cursor::MoveTo(self.x +1, self.y)).unwrap();
		self.colorset[1].write_color(&mut out, false);
		self.bgcolorset[1].write_color(&mut out, true);
		utils::repeat(&mut out, self.charset[1], bars_full_amount);

		write!(out, "\x1b[0m").unwrap();

		stdout().flush().unwrap();
	}
}
