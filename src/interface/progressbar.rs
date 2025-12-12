use crossterm::{
	execute,
	cursor::MoveTo,
};
use std::io::{stdout, Write};

use crate::utils;

pub struct ProgressBar {
	// Size and position
	pub x : u16, pub y : u16,
	pub size : u16,

	// Extra options
	pub color : utils::Color,
	pub line_type : u8,
	pub percentage_show : u8, // 0 -> Full numbers; 1 -> 1 decimal; 2 -> 2 decimal
	pub progress_full : u32, // How much of percentage_max is full
	pub progress_max : u32, // The max number the bar can show
}


impl ProgressBar {
	pub fn draw(&self) {
		let mut out = stdout();

		execute!(out, crossterm::cursor::MoveTo(self.x, self.y)).unwrap();
		write!(out, "|").unwrap();
		utils::repeat(&mut out, '-', self.size);
		write!(out, "|").unwrap();
		
		let percent : f64 = self.progress_full as f64 / self.progress_max as f64;
		let bars_full_amount : u16 = (percent * self.size as f64) as u16;

		execute!(out, crossterm::cursor::MoveTo(self.x +1, self.y)).unwrap();
		utils::repeat(&mut out, '=', bars_full_amount);

		stdout().flush().unwrap();
	}
}
