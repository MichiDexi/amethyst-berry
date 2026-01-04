use crate::helpers;
use crate::abt::mapdata::*;

struct Writer {
	value : String
}

impl Writer {
	fn w_u8(&mut self, input : u8) {
		self.value.push_str(&helpers::utils::decimal_hex(input as u64, 2));
	}

	fn w_u16(&mut self, input : u16) {
		self.value.push_str(&helpers::utils::decimal_hex(input as u64, 4));
	}
	
	fn w_u32(&mut self, input : u32) {
		self.value.push_str(&helpers::utils::decimal_hex(input as u64, 8));
	}
	
	fn w_u64(&mut self, input : u64) {
		self.value.push_str(&helpers::utils::decimal_hex(input, 16));
	}


	fn w_op_u8(&mut self, input : Option<u8>) {
		if let Some(v) = input {
			self.value.push_str(&helpers::utils::decimal_hex(v as u64, 2));
		}
		else {
			self.value.push('N');
		}
	}
/*
	fn w_op_u16(&mut self, input : Option<u16>) {
		if let Some(v) = input {
			self.value.push_str(&helpers::utils::decimal_hex(v as u64, 4));
		}
		else {
			self.value.push('N');
		}
	}
	
	fn w_op_u32(&mut self, input : Option<u32>) {
		if let Some(v) = input {
			self.value.push_str(&helpers::utils::decimal_hex(v as u64, 8));
		}
		else {
			self.value.push('N');
		}
	}
	
	fn w_op_u64(&mut self, input : Option<u64>) {
		if let Some(v) = input {
			self.value.push_str(&helpers::utils::decimal_hex(v, 16));
		}
		else {
			self.value.push('N');
		}
	}
*/

	fn w_bool(&mut self, input : bool) {
		self.value.push_str(&bool_char(input));
	}

	fn w_op_bool(&mut self, input : Option<bool>) {
		if let Some(v) = input {
			self.value.push_str(&bool_char(v));
		}
		else {
			self.value.push('N');
		}
	}


	fn w_str(&mut self, input : &str) {
		self.value.push_str(&helpers::utils::decimal_hex(input.len() as u64, 2));
		self.value.push_str(input);
	}


	fn w_vec_open(&mut self) { self.value.push('<'); }
	fn w_vec_close(&mut self) { self.value.push('>'); }
	fn w_sep(&mut self) { self.value.push(';'); }

	fn new() -> Self {
		Self {
			value : "".to_string()
		}
	}
}


struct Reader {
	index : usize,
	string_map : String
}

impl Reader {
	fn r_u8(&mut self) -> u8 {
		let value : &str = &self.string_map[self.index..self.index+2];
		self.index += 2;
		helpers::utils::hex_decimal(value) as u8
	}
	fn r_u16(&mut self) -> u16 {
		let value : &str = &self.string_map[self.index..self.index+4];
		self.index += 4;
		helpers::utils::hex_decimal(value) as u16
	}
	fn r_u32(&mut self) -> u32 {
		let value : &str = &self.string_map[self.index..self.index+8];
		self.index += 8;
		helpers::utils::hex_decimal(value) as u32
	}
	fn r_u64(&mut self) -> u64 {
		let value : &str = &self.string_map[self.index..self.index+16];
		self.index += 16;
		helpers::utils::hex_decimal(value)
	}

	fn r_op_u8(&mut self) -> Option<u8> {
		if self.string_map.as_bytes()[self.index] == b'N' {
			self.index += 1;
			return None;
		}
		let value : &str = &self.string_map[self.index..self.index+2];
		self.index += 2;
		Some(helpers::utils::hex_decimal(value) as u8)
	}
	/*
	fn r_op_u16(&mut self) -> Option<u16> {
		if self.string_map.as_bytes()[self.index] == b'N' {
			self.index += 1;
			return None;
		}
		let value : &str = &self.string_map[self.index..self.index+4];
		self.index += 4;
		Some(helpers::utils::hex_decimal(value) as u16)
	}
	fn r_op_u32(&mut self) -> Option<u32> {
		if self.string_map.as_bytes()[self.index] == b'N' {
			self.index += 1;
			return None;
		}
		let value : &str = &self.string_map[self.index..self.index+8];
		self.index += 8;
		Some(helpers::utils::hex_decimal(value) as u32)
	}
	fn r_op_u64(&mut self) -> Option<u64> {
		if self.string_map.as_bytes()[self.index] == b'N' {
			self.index += 1;
			return None;
		}
		let value : &str = &self.string_map[self.index..self.index+16];
		self.index += 16;
		Some(helpers::utils::hex_decimal(value))
	}
	*/

	fn r_bool(&mut self) -> bool {
		let v : u8 = self.string_map.as_bytes()[self.index];
		self.index += 1;
		v == b't'
	}
	fn r_op_bool(&mut self) -> Option<bool> {
		let v : u8 = self.string_map.as_bytes()[self.index];
		self.index += 1;
		match v {
			b't' => Some(true),
			b'f' => Some(false),
			_ => None // includes 'N' for None
		}
	}

	fn r_str(&mut self) -> String {
		let value : &str = &self.string_map[self.index..self.index+2];
		self.index += 2;
		let length : u8 = helpers::utils::hex_decimal(value) as u8;

		let output : String = self.string_map[self.index..self.index+length as usize].to_string();
		self.index += length as usize;
		output
	}

	fn r_vec_open(&mut self) -> bool {
		let out : bool = self.string_map.as_bytes()[self.index] == b'<';
		self.index += 1;
		out
	}
	fn r_vec_close(&mut self) -> bool {
		let out : bool = self.string_map.as_bytes()[self.index] == b'>';
		self.index += 1;
		out
	}
	fn r_sep(&mut self) -> bool {
		let out : bool = self.string_map.as_bytes()[self.index] == b';';
		self.index += 1;
		out
	}

	fn new(input : String) -> Self {
		assert!(input.is_ascii(), "non-ASCII input detected");
		Self {
			index : 0,
			string_map : input,
		}
	}
}


pub fn parse_string_as_map(map : String) -> Option<Map> {
	let mut reader : Reader = Reader::new(map);
	let mut map_out : Map = Map::new();
	
	// Main data
	map_out.name = reader.r_str();
	map_out.starred = reader.r_bool();
	map_out.rating = reader.r_op_u8();
	let tag_len = reader.r_u8();

	if !reader.r_vec_open() { return None; }
	
	for _ in 0..tag_len {
		map_out.tags.push(reader.r_str());
	}

	println!("{}", reader.index);
	if !reader.r_vec_close() { return None; }
	
	map_out.difficulty = reader.r_u8();
	
	map_out.clear_progress = reader.r_op_u8();
	map_out.deaths = reader.r_u32();
	
	if !reader.r_sep() { return None; }
	
	// Strawberries
	map_out.strawberry.collected = reader.r_u16();
	map_out.strawberry.total = reader.r_u16();
	if !reader.r_sep() { return None; }
		
	// Golden berries
	let exists = reader.r_op_bool();
	if let Some(v) = exists {
		map_out.goldberry =
			Some(
				crate::abt::mapdata::TypedCollectable {
					collected : v,
					btype : reader.r_u8()
				}
			)
	}
	
	if !reader.r_sep() { return None; }

	// Silver berries
	let silverberries_exist = reader.r_op_u8();
	map_out.silverberries = silverberries_exist;
	if !reader.r_sep() { return None; }

	// Special berry
	let exists = reader.r_op_bool();
	if let Some(v) = exists {
		map_out.specialberry =
			Some(
				crate::abt::mapdata::NamedCollectable {
					collected : v,
					name : reader.r_str()
				}
			)
	}
	if !reader.r_sep() { return None; }

	// Extra data
	map_out.cassette = reader.r_op_bool();
	
	let exists = reader.r_op_bool();
	if let Some(v) = exists {
		map_out.crystalheart =
			Some(
				crate::abt::mapdata::NamedCollectable {
					collected : v,
					name : reader.r_str()
				}
			)
	}
	reader.r_sep();

	// Challenge data
	if reader.r_bool() {
		map_out.min_dashes =
			Some(
				Tiered {
					pb : reader.r_u16(),
					bronze : reader.r_u16(),
					silver : reader.r_u16(),
					gold : reader.r_u16(),
				}
			)
	}
	if reader.r_bool() {
		map_out.min_jumps =
			Some(
				Tiered {
					pb : reader.r_u16(),
					bronze : reader.r_u16(),
					silver : reader.r_u16(),
					gold : reader.r_u16(),
				}
			)
	}
	if reader.r_bool() {
		map_out.speedrun =
			Some(
				Tiered {
					pb : reader.r_u64(),
					bronze : reader.r_u64(),
					silver : reader.r_u64(),
					gold : reader.r_u64(),
				}
			)
	}
	reader.r_sep();

	// Practice data
	map_out.pb = reader.r_u32();
	map_out.min_deaths_pb = reader.r_u32();
	reader.r_sep();

	// Checkpoints
	let length = reader.r_u8();
	reader.r_vec_open();
	for _ in 0..length {
		let mut checkpoint_out : Checkpoint = Checkpoint::new(&reader.r_str());
		
		checkpoint_out.notes = Notes {
			notes : reader.r_str(),
			speedrun_notes : reader.r_str(),
			min_dash_notes : reader.r_str(),
			min_jump_notes : reader.r_str(),
			goldenberry_notes : reader.r_str(),
		};

		// Chokepoints
		let length_nested = reader.r_u8();
		reader.r_vec_open();
		for _ in 0..length_nested {
			let mut chokepoint_out : Chokepoint = Chokepoint::new(&reader.r_str());
			chokepoint_out.notes = Notes {
				notes : reader.r_str(),
				speedrun_notes : reader.r_str(),
				min_dash_notes : reader.r_str(),
				min_jump_notes : reader.r_str(),
				goldenberry_notes : reader.r_str(),
			};
			chokepoint_out.highest_backtoback_amount = reader.r_u16();
			chokepoint_out.runs_died = reader.r_u16();
			chokepoint_out.runs_passed = reader.r_u16();

			checkpoint_out.chokepoints.push(chokepoint_out);
		}
		reader.r_vec_close();
		
		if silverberries_exist.is_some() {
			checkpoint_out.silver_collected = Some(true);
		}
		checkpoint_out.runs_died = reader.r_u16();
		checkpoint_out.runs_passed = reader.r_u16();
		checkpoint_out.min_deaths = reader.r_u16();

		map_out.checkpoints.push(checkpoint_out);
	}
	reader.r_vec_close();
	reader.r_sep();

	// Ranges
	let length = reader.r_u8();
	reader.r_vec_open();
	for _ in 0..length {
		map_out.range_runs.push(CheckpointRange {
			id_start : reader.r_u8(),
			id_end : reader.r_u8(),
		});
	}
	reader.r_vec_close();

	Some(map_out)
}

pub fn parse_map_as_string(map : Map) -> String {
	let mut writer : Writer = Writer::new();

	// Main data
	writer.w_str(&map.name);
	writer.w_bool(map.starred);
	writer.w_op_u8(map.rating);
	writer.w_u8(map.tags.len() as u8);

	writer.w_vec_open();
	for tag in &map.tags {
		writer.w_str(tag);
	}
	writer.w_vec_close();
	writer.w_u8(map.difficulty);
	
	writer.w_op_u8(map.clear_progress);
	writer.w_u32(map.deaths);

	writer.w_sep();


	// Strawberries
	writer.w_u16(map.strawberry.collected);
	writer.w_u16(map.strawberry.total);
	writer.w_sep();


	// Golden berries
	match &map.goldberry {
		Some(v) => {
			writer.w_bool(v.collected);
			writer.w_u8(v.btype);
		}
		None => {
			writer.w_op_bool(None);
		}
	}
	writer.w_sep();


	// Silver berries
	let silversexist : bool = map.silverberries.is_some();
	writer.w_op_u8(map.silverberries);
	writer.w_sep();


	// Special berry
	if let Some(v) = &map.specialberry {
		writer.w_bool(v.collected);
		writer.w_str(&v.name);
	}
	else {
		writer.w_op_bool(None);
	}
	writer.w_sep();

	// Extra data
	writer.w_op_bool(map.cassette);
	if let Some(v) = &map.crystalheart {
		writer.w_bool(v.collected);
		writer.w_str(&v.name);
	}
	else {
		writer.w_op_bool(None);
	}
	writer.w_sep();

	// Challenge data
	if let Some(v) = &map.min_dashes {
		writer.w_bool(true);
		writer.w_u16(v.pb);
		writer.w_u16(v.bronze);
		writer.w_u16(v.silver);
		writer.w_u16(v.gold);
	}
	else {
		writer.w_bool(false);
	}
	if let Some(v) = &map.min_jumps {
		writer.w_bool(true);
		writer.w_u16(v.pb);
		writer.w_u16(v.bronze);
		writer.w_u16(v.silver);
		writer.w_u16(v.gold);
	}
	else {
		writer.w_bool(false);
	}
	if let Some(v) = &map.speedrun {
		writer.w_bool(true);
		writer.w_u64(v.pb);
		writer.w_u64(v.bronze);
		writer.w_u64(v.silver);
		writer.w_u64(v.gold);
	}
	else {
		writer.w_bool(false);
	}
	writer.w_sep();

	// Practice data
	writer.w_u32(map.pb);
	writer.w_u32(map.min_deaths_pb);
	writer.w_sep();

	// Checkpoints
	writer.w_u8(map.checkpoints.len() as u8);
	writer.w_vec_open();
	for checkpoint in &map.checkpoints {
		writer.w_str(&checkpoint.name);

		// Checkpoint notes
		writer.w_str(&checkpoint.notes.notes);
		writer.w_str(&checkpoint.notes.speedrun_notes);
		writer.w_str(&checkpoint.notes.min_dash_notes);
		writer.w_str(&checkpoint.notes.min_jump_notes);
		writer.w_str(&checkpoint.notes.goldenberry_notes);

		// Chokepoints in checkpoints
		writer.w_u8(checkpoint.chokepoints.len() as u8);

		writer.w_vec_open();
		for chokepoint in &checkpoint.chokepoints {
			writer.w_str(&chokepoint.name);
			
			// Chokepoint notes
			writer.w_str(&checkpoint.notes.notes);
			writer.w_str(&checkpoint.notes.speedrun_notes);
			writer.w_str(&checkpoint.notes.min_dash_notes);
			writer.w_str(&checkpoint.notes.min_jump_notes);
			writer.w_str(&checkpoint.notes.goldenberry_notes);
			
			writer.w_u16(chokepoint.highest_backtoback_amount);
			writer.w_u16(chokepoint.runs_died);
			writer.w_u16(chokepoint.runs_passed);
		}
		writer.w_vec_close();

		if silversexist {
			writer.w_op_bool(checkpoint.silver_collected);
		}
		writer.w_u16(checkpoint.min_deaths);
		writer.w_u16(checkpoint.runs_died);
		writer.w_u16(checkpoint.runs_passed);
	}
	writer.w_vec_close();
	
	writer.w_sep();

	// Ranges
	writer.w_u8(map.range_runs.len() as u8);
	writer.w_vec_open();
	for run in &map.range_runs {
		writer.w_u8(run.id_start);
		writer.w_u8(run.id_end);
	}
	writer.w_vec_close();
	
	writer.value
}

fn bool_char(input : bool) -> String {
	match input {
		false => "f".to_string(),
		true => "t".to_string()
	}
}
