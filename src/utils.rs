pub struct Color {
	color : u8,
	bright : bool
}

impl Color {
	pub fn set_color(&mut self, c : u8) {
		self.color = c;
	}

	pub fn write_color(&self, background : bool) -> stdout {
		
	}
}


pub struct TrueColor {
	red : u8,
	green : u8,
	blue : u8,
}

impl TrueColor {
	pub fn set_color(&mut self, r : u8, g : u8, b : u8) {
		self.red = r;
		self.green = g;
		self.blue = b;
	}

	pub fn write_color(&self, background : bool) -> stdout {
		
	}
}
