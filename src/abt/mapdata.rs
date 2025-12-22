pub use std::path::Path;
use std::fs;

use crate::helpers;

pub struct MapCluster {
	pub name : String,
	pub a_side : Map,
	pub b_side : Map,
	pub c_side : Map,
	pub d_side : Map,
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

	pub fn load(&mut self, path : &Path) {
		let mapcluster : Vec<String> = fs::read_to_string(path)
			.unwrap()
			.lines()
			.map(|line| line.to_string())
			.collect();

		for i in 0..mapcluster.len() {
			let map_obj : Map = parse_map(&mapcluster[i]);
			match i {
				0 => { self.a_side = map_obj; },
				1 => { self.b_side = map_obj; },
				2 => { self.c_side = map_obj; },
				3 => { self.d_side = map_obj; },
				_ => {}
			};
		}
	}

	pub fn save(path : &Path) {
		
	}
}

pub struct Checkpoint {
	pub id : u8,
	pub name : String,
	pub notes : String,
	pub chokepoints : Vec<Chokepoint>,

	// Practice data
	pub min_deaths : u16,
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
	pub id : u8,
	pub name : String,
	pub notes : String,

	// Practice data
	pub highest_backtoback_amount : u16, // weird meassurement, but I'd use that
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
	pub id_start : u8,
	pub id_end : u8,
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
	pub name : String,
	pub tag : String,
	pub difficulty : u8,
	pub clear_progress : u8,
	pub deaths : u32,

	// Strawberries
	pub strawberry_amount : u16,
	pub strawberry_collected : u16,
	
	// Golden berries refer to 'main-deathlessberries', the type can be changed
	pub goldberry_exists : bool,
	pub goldberry_collected : bool,
	pub goldberry_type : u8,
	
	// Silver berries refer to 'sub-deathlessberries', the type can be changed
	pub silverberry_exists : bool,
	pub silverberry_amount : u8,
	pub silverberry_collected : u8,
	pub silverberry_type : u8,

	// This is a place for the 'Moonberry' or 'Winged Goldberry'
	pub specialberry_exists : bool,
	pub specialberry_name : String,
	pub specialberry_collected : bool,

	// Challenge data
	pub challenge_activate : bool,
	pub min_dashes_pb : u16,
	pub min_dashes_bronze : u16,
	pub min_dashes_silver : u16,
	pub min_dashes_gold : u16,
	pub min_jumps_pb : u16,
	pub min_jumps_bronze : u16,
	pub min_jumps_silver : u16,
	pub min_jumps_gold : u16,

	// Speedrun data
	pub fastest_time : u64,
	pub speedrun_bronze : u64,
	pub speedrun_silver : u64,
	pub speedrun_gold : u64,
	
	// Extra data no one could care less about
	pub cassette_exists : bool,
	pub cassette_collected : bool,
	pub crystalheart_exists : bool,
	pub crystalheart_name : String,
	pub crystalheart_collected : bool,


	// Here is all the practice data
	// This is very complex data, that may or may not break when I'll try to parse it


	pub pb : u32, // Furthest the user has come
	pub min_deaths_pb : u32,
	pub checkpoints : Vec<Checkpoint>,
	pub range_runs : Vec<CheckpointRange>,
}

impl Map {
	pub fn new(nname : &str) -> Self {
		Self {
			name : nname.to_string(),
			tag : "".to_string(),
			difficulty : 0,
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

pub fn present_map(map : &Map) {
	println!("map {} - {} - {} - {} - {}", map.name, map.tag, map.difficulty, map.clear_progress, map.deaths);
	println!("str {} - {}", map.strawberry_amount, map.strawberry_collected);
	println!("gol {} - {} - {}", map.goldberry_exists, map.goldberry_collected, map.goldberry_type);
	println!("sil {} - {} - {} - {}", map.silverberry_exists, map.silverberry_amount, map.silverberry_collected, map.silverberry_type);
	println!("spe {} - {} - {}", map.specialberry_exists, map.specialberry_name, map.specialberry_collected);
	println!("cha {} - {} - {} - {} - {} - {} - {} - {} - {}", map.challenge_activate, map.min_dashes_pb, map.min_dashes_bronze, map.min_dashes_silver, map.min_dashes_gold, map.min_jumps_pb, map.min_jumps_bronze, map.min_jumps_silver, map.min_jumps_gold);
	println!("frn {} - {} - {} - {}", map.fastest_time, map.speedrun_bronze, map.speedrun_silver, map.speedrun_gold);
	println!("cry {} - {} - {} - {} - {}", map.cassette_exists, map.cassette_collected, map.crystalheart_exists, map.crystalheart_name, map.crystalheart_collected);
	println!("pra {} - {} - {} - {} - {}", map.pb, map.min_deaths_pb, map.difficulty, map.clear_progress, map.deaths);

	
	for checkpoint in &map.checkpoints {
		println!("  che {} - {} - {} - {}", checkpoint.name, checkpoint.id, checkpoint.notes, checkpoint.min_deaths);
		for chokepoint in &checkpoint.chokepoints {
			println!("    cho {} - {} - {} - {}", chokepoint.name, chokepoint.id, chokepoint.notes, chokepoint.highest_backtoback_amount);
		}
	}

	for range in &map.range_runs {
		println!("  ran {} - {}", range.id_start, range.id_end);
	}
}
