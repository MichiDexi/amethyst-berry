use std::{
	io::Stdout,
	io::Write,
	rc::Rc,
	cell::RefCell,
};
use crate::{
	interface::traits,
	helpers::input,
	abt::menus,
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
			write!(out, "\x1b[3{}m", self.color).unwrap();
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
				layer += 6; // Convert to bright
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
	u64::from_str_radix(value, 16).unwrap_or(0)
}

pub fn decimal_hex(value: u64, width: usize) -> String {
	format!("{:0width$X}", value, width = width)
}



pub fn object(
	object : &mut impl traits::UserInterface, input : &input::InputHandler,
	menu_reference : &Rc<RefCell<menus::Menu>>, menu_clicked : menus::Menu,
	norm : (i16, i16), hover : (i16, i16), anchor : u8,
	out : &mut Stdout
) {
	let prev_state : bool = object.is_hovered();
	object.set_position(
		norm.0, norm.1,
		anchor, (input.window.width, input.window.height)
	);
	traits::UserInterface::update(object, input);
	let redraw_requested : bool =
		object.is_hovered() != prev_state;

	if redraw_requested {
		if prev_state {
			object.set_position(
				hover.0, hover.1,
				anchor, (input.window.width, input.window.height)
			);
		}
		else {
			object.set_position(
				norm.0, norm.1,
				anchor, (input.window.width, input.window.height)
			);
		}
		
		traits::UserInterface::clear(object, out);
	}
	
	if object.is_hovered() {
		object.set_position(
			hover.0, hover.1,
			anchor, (input.window.width, input.window.height)
		);
		object.set_color(true);
		if input.mouse.lclick {
			*menu_reference.borrow_mut() = menu_clicked;
		}
	}
	else {
		object.set_color(false);
		object.set_position(
			norm.0, norm.1,
			anchor, (input.window.width, input.window.height)
		);
	}
	
	if redraw_requested {
		traits::UserInterface::draw(object, out);
	}
}
