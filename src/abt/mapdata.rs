use std::path::Path;
use std::fs;

pub struct MapCluster {
	name : String,
	a_side : Map,
	b_side : Map,
	c_side : Map,
	d_side : Map,
}

pub struct Checkpoint {
	id : u8,
	name : String,
	notes : String,
	chokepoints : Vec<Chokepoint>,

	// Practice data
	min_deaths : u16,
}

pub struct Chokepoint {
	id : u8,
	name : String,
	notes : String,

	// Practice data
	highest_backtoback_amount : u16, // weird meassurement, but I'd use that
}

pub struct CheckpointRange {
	id_start : u8,
	id_end : u8,
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
	ranged_runs : Vec<CheckpointRange>,
}

impl MapCluster {
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
