pub use std::io::{
	Stdout,
	Write,
	stdout,
	self,
};

// Color structs

pub struct Color {
	pub color_enabled : bool,

	pub color : u8,
	pub bright : bool,

	pub truecolor : bool,
	pub red : u8,
	pub green : u8,
	pub blue : u8,
}

impl Color {
	pub fn write_color(&self, out : &mut Stdout, background : bool) {
		if !self.color_enabled {
			return;
		}
	
		let mut layer = match background {
			true => 4,
			false => 3,
		};
		
		if self.truecolor {
			write!(out, "\x1b[{}8;2;{};{};{}m", layer, self.red, self.green, self.blue).unwrap();
		}
		else {
			if self.bright {
				layer += 60; // Convert to bright
			}
			write!(out, "\x1b[{}{}m", layer, self.color).unwrap();
		}
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



// Check collision

pub fn check_collision(
	x : u16, y : u16,
	width : u16, height : u16,
	mx : u16, my : u16
) -> bool {

	mx >= x && // mx is to the right of the left side
	mx < x + width && // mx is to the left of the right side
	my >= y && // my is below the upper side
	my < y + height // mx is above the lower side (inside the box, is colliding)
}


// Hex <-> Decimal Convertions

pub fn hex_decimal(value : &str) -> u64 {
	let mut output : u64 = 0;
	let chars : Vec<char> = value.chars().collect();

	for i in 0..chars.len() {
		let character : char = chars[chars.len() - 1 - i];

		match character {
			'1' => output += 1,
			'2' => output += 2,
			'3' => output += 3,
			'4' => output += 4,
			'5' => output += 5,
			'6' => output += 6,
			'7' => output += 7,
			'8' => output += 8,
			'9' => output += 9,
			'A' => output += 10,
			'B' => output += 11,
			'C' => output += 12,
			'D' => output += 13,
			'E' => output += 14,
			'F' => output += 15,
			_ => {} // 0 -> +0; Anything else ignored
		}
		if i != chars.len()-1 {
			output <<= 4;
		}
	}
	output
}

pub fn decimal_hex(mut value: u64) -> String {
	if value == 0 {
		return "0".to_string();
	}

	let mut output = String::new();

	while value > 0 {
		let digit = (value & 0xF) as u8;

		let ch = match digit {
			0..=9  => (b'0' + digit) as char,
			10..=15 => (b'A' + (digit - 10)) as char,
			_ => {'0'},
		};

		output.push(ch);
		value >>= 4;
	}

	output.chars().rev().collect()
}

