pub use std::io::{
	Stdout,
	Write,
	stdout,
	self,
};

// Color structs

pub struct Color {
	pub color : u8,
	pub bright : bool
}

impl Color {
	pub fn set_color(&mut self, c : u8) {
		self.color = c;
	}

	pub fn write_color(&self, background : bool) {
		
	}
}

pub struct TrueColor {
	pub red : u8,
	pub green : u8,
	pub blue : u8,
}

impl TrueColor {
	pub fn set_color(&mut self, r : u8, g : u8, b : u8) {
		self.red = r;
		self.green = g;
		self.blue = b;
	}

	pub fn write_color(&self, background : bool) {
		
	}
}



// Unicode characters

pub const NORMAL : [char; 11] =    ['─', '│', '┌', '┐', '└', '┘', '├', '┤', '┬', '┴', '┼'];
pub const THICK : [char; 11] =     ['━', '┃', '┏', '┓', '┗', '┛', '┣', '┫', '┳', '┻', '╋'];
pub const DOT3 : [char; 2] =       ['┄', '┆'];
pub const DOT3_THICK : [char; 2] = ['┅', '┇'];
pub const DOT4 : [char; 2] =       ['┈', '┊'];
pub const DOT4_THICK : [char; 2] = ['┉', '┋'];
pub const MLINE : [char; 9] =      ['═', '║', '╔', '╗', '╚', '╝', '╠', '╣', '╬'];
pub const SHADING : [char; 6] =    ['░', '▒', '▓', '█', '▄', '▀'];

pub fn repeat(out : &mut Stdout, c : char, n : u16) {
	for _ in 0..n {
		write!(out, "{c}").unwrap();
	}
}

pub fn get_char(ctype : u8, index : u8) -> char {
	match ctype {
		0 => NORMAL[index as usize],
		1 => THICK[index as usize],
		2 => DOT3[index as usize],
		3 => DOT3_THICK[index as usize],
		4 => DOT4[index as usize],
		5 => DOT4_THICK[index as usize],
		6 => MLINE[index as usize],
		7 => SHADING[index as usize],
		_ => NORMAL[index as usize],
	}
}



// Other character shenanigans

pub fn shorten_text(text : &str, length : u16) -> String {
	if text.len() > length as usize {
		let mut output = text[0..length as usize - 3].to_string();
		output += "...";
		output
		// I used this earlier:
		// output = (*(&text[0_usize..length as usize-3_usize].to_string()).clone()).to_string();
		// It works, don't ask why
	}
	else {
		text.to_string()
		// This used to be '(&text).clone()' at one point
		// This function has the silliest history ever
	}
}
