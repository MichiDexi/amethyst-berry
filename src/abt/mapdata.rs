use std::path::Path;
use std::fs;

pub struct MapCluster {
	name : String,
	a_side : Map,
	b_side : Map,
	c_side : Map,
	d_side : Map,
}

impl MapCluster {
	pub fn new(nname : &str) -> Self {
		Self {
			name : nname.to_string(),
			a_side : Map::new(&(nname.to_string() + " A-side")),
			b_side : Map::new(&(nname.to_string() + " B-side")),
			c_side : Map::new(&(nname.to_string() + " C-side")),
			d_side : Map::new(&(nname.to_string() + " D-side")),
		}
	}

	pub fn load(path : &Path) {
		let mapcluster : Vec<String> = fs::read_to_string(path)
			.unwrap()
			.lines()
			.map(|line| line.to_string())
			.collect();

		for i in 0..mapcluster.len() {
			let mut current_char : u16 = 0; // Index of the current character that's being parsed


			// String decoding
			let name : String = "a".to_string();
		}
	}

	pub fn save(path : &Path) {
		
	}
}


pub struct Checkpoint {
	id : u8,
	name : String,
	notes : String,
	chokepoints : Vec<Chokepoint>,

	// Practice data
	min_deaths : u16,
}

impl Checkpoint {
	pub fn new(nname : &str, nid : u8) -> Self {
		Self {
			id : nid,
			name : nname.to_string(),
			notes : "".to_string(),
			chokepoints : Vec::new(),
			min_deaths : 0
		}
	}
}


pub struct Chokepoint {
	id : u8,
	name : String,
	notes : String,

	// Practice data
	highest_backtoback_amount : u16, // weird meassurement, but I'd use that
}

impl Chokepoint {
	pub fn new(nname : &str, nid : u8) -> Self {
		Self {
			id : nid,
			name : nname.to_string(),
			notes : "".to_string(),
			highest_backtoback_amount : 0
		}
	}
}


pub struct CheckpointRange {
	id_start : u8,
	id_end : u8,
}

impl CheckpointRange {
	pub fn new(nid_s : u8, nid_e : u8) -> Self {
		Self {
			id_start : nid_s,
			id_end : nid_e
		}
	}
}


pub struct Map {
	// Main data
	name : String,
	tag : String,
	difficulty : u8,
	length : u8,
	clear_progress : u8,
	deaths : u32,

	// Strawberries
	strawberry_amount : u16,
	strawberry_collected : u16,
	
	// Golden berries refer to 'main-deathlessberries', the type can be changed
	goldberry_exists : bool,
	goldberry_collected : bool,
	goldberry_type : u8,
	
	// Silver berries refer to 'sub-deathlessberries', the type can be changed
	silverberry_exists : bool,
	silverberry_amount : u8,
	silverberry_collected : u8,
	silverberry_type : u8,

	// This is a place for the 'Moonberry' or 'Winged Goldberry'
	specialberry_exists : bool,
	specialberry_name : String,
	specialberry_collected : bool,

	// Challenge data
	challenge_activate : bool,
	min_dashes_pb : u16,
	min_dashes_bronze : u16,
	min_dashes_silver : u16,
	min_dashes_gold : u16,
	min_jumps_pb : u16,
	min_jumps_bronze : u16,
	min_jumps_silver : u16,
	min_jumps_gold : u16,

	// Speedrun data
	fastest_time : u64,
	speedrun_bronze : u64,
	speedrun_silver : u64,
	speedrun_gold : u64,
	
	// Extra data no one could care less about
	cassette_exists : bool,
	cassette_collected : bool,
	crystalheart_exists : bool,
	crystalheart_name : String,
	crystalheart_collected : bool,


	// Here is all the practice data
	// This is very complex data, that may or may not break when I'll try to parse it


	pb : u32, // Furthest the user has come
	min_deaths_pb : u32,
	checkpoints : Vec<Checkpoint>,
	range_runs : Vec<CheckpointRange>,
}

impl Map {
	pub fn new(nname : &str) -> Self {
		Self {
			name : nname.to_string(),
			tag : "".to_string(),
			difficulty : 0,
			length : 0,
			clear_progress : 0,
			deaths : 0,
			strawberry_amount : 0,
			strawberry_collected : 0,
			goldberry_exists : false,
			goldberry_collected : false,
			goldberry_type : 0,
			silverberry_exists : false,
			silverberry_amount : 0,
			silverberry_collected : 0,
			silverberry_type : 0,
			specialberry_exists : false,
			specialberry_name : "".to_string(),
			specialberry_collected : false,
			challenge_activate : false,
			min_dashes_pb : 0,
			min_dashes_bronze : 0,
			min_dashes_silver : 0,
			min_dashes_gold : 0,
			min_jumps_pb : 0,
			min_jumps_bronze : 0,
			min_jumps_silver : 0,
			min_jumps_gold : 0,
			fastest_time : 0,
			speedrun_bronze : 0,
			speedrun_silver : 0,
			speedrun_gold : 0,
			cassette_exists : false,
			cassette_collected : false,
			crystalheart_exists : false,
			crystalheart_name : "".to_string(),
			crystalheart_collected : false,
			pb : 0,
			min_deaths_pb : 0,
			checkpoints : Vec::new(),
			range_runs : Vec::new(),
		}
	}
}
