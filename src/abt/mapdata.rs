pub use std::path::Path;
use std::fs;

use crate::abt::mapparsing::*;

pub struct Checkpoint {
	pub name : String,
	pub notes : Notes,
	pub chokepoints : Vec<Chokepoint>,
	pub silver_collected : Option<bool>,

	// Practice data
	pub min_deaths : u16,
	pub runs_died : u16,
	pub runs_passed : u16,
}

impl Checkpoint {
	pub fn new(nname : &str, silvers_exist : bool) -> Self {
		Self {
			name : nname.to_string(),
			notes : Notes::new(),
			chokepoints : Vec::new(),
			silver_collected : match silvers_exist {
				true => Some(false),
				false => None
			},
			
			min_deaths : 0,
			runs_died : 0,
			runs_passed : 0,
		}
	}
}


pub struct Chokepoint {
	pub name : String,
	pub notes : Notes,

	// Practice data
	pub highest_backtoback_amount : u16, // weird meassurement, but I'd use that
	pub runs_died : u16,
	pub runs_passed : u16,
}

impl Chokepoint {
	pub fn new(nname : &str) -> Self {
		Self {
			name : nname.to_string(),
			notes : Notes::new(),
			
			highest_backtoback_amount : 0,
			runs_died : 0,
			runs_passed : 0,
		}
	}
}

pub struct Notes {
	pub notes : String,
	pub speedrun_notes : String,
	pub min_dash_notes : String,
	pub min_jump_notes : String,
	pub goldenberry_notes : String,
}

impl Notes {
	fn new() -> Self {
		Self {
			notes : "".to_string(),
			speedrun_notes : "".to_string(),
			min_dash_notes : "".to_string(),
			min_jump_notes : "".to_string(),
			goldenberry_notes : "".to_string(),
		}
	}
}


pub struct CheckpointRange {
	pub id_start : u8,
	pub id_end : u8,
}

impl CheckpointRange {
	pub fn new(nid_s : u8, nid_e : u8) -> Option<Self> {
		if nid_s >= nid_e {
			return None;
		}
	
		Some(Self {
			id_start : nid_s,
			id_end : nid_e
		})
	}
}


pub struct Map {
	// Main data
	pub name : String,
	pub starred : bool,
	pub rating : Option<u8>,
	pub tags : Vec<String>,
	pub difficulty : u8,
	pub clear_progress : Option<u8>,
	pub deaths : u32,

	// Berries
	pub strawberry : Counted,
	pub goldberry : Option<TypedCollectable>,
	pub silverberries : Option<u8>, // type

	// This is a place for the 'Moonberry' or 'Winged Goldberry'
	pub specialberry : Option<NamedCollectable>,

	// Extra
	pub cassette : Option<bool>,
	pub crystalheart : Option<NamedCollectable>,
	
	// Challenge data
	pub min_dashes : Option<Tiered<u16>>,
	pub min_jumps : Option<Tiered<u16>>,
	pub speedrun : Option<Tiered<u64>>,

	// Practice data
	pub pb : u32,
	pub min_deaths_pb : u32,
	pub checkpoints : Vec<Checkpoint>,
	pub range_runs : Vec<CheckpointRange>,
}

pub struct Counted {
	pub collected : u16,
	pub total : u16,
}

pub struct Tiered<T> {
	pub pb : T,
	pub bronze : T,
	pub silver : T,
	pub gold : T,
}

pub struct NamedCollectable {
	pub collected : bool,
	pub name : String
}

pub struct TypedCollectable {
	pub collected : bool,
	pub btype : u8
}


impl Map {
	pub fn new() -> Self {
		Self {
			name : "Unnamed".to_string(),
			starred : false,
			rating : None,
			tags : Vec::new(),
			difficulty : 0,
			clear_progress : None,
			deaths : 0,

			// Berries
			strawberry : Counted {
				collected : 0,
				total : 0,
			},
			goldberry : None,
			silverberries : None, // type

			// This is a place for the 'Moonberry' or 'Winged Goldberry'
			specialberry : None,

			// Extra
			cassette : None,
			crystalheart : None,
			
			// Challenge data
			min_dashes : None,
			min_jumps : None,
			speedrun : None,

			// Practice data
			pb : 0,
			min_deaths_pb : 0,
			checkpoints : Vec::new(),
			range_runs : Vec::new(),
		}
	}
}

pub fn present_map(map : &Map) {
	println!("name: {}", map.name);
	println!("starred: {}", map.starred);
	if let Some(v) = &map.rating {
		println!("rating: {}", v);
	}
	for tag in &map.tags {
		println!("tag: {}", tag);
	}
	println!("difficulty: {}", map.difficulty);
	if let Some(v) = &map.clear_progress {
		println!("clear_progress: {}", v);
	}
	println!("deaths: {}", map.deaths);

	println!("strawberry c: {}", map.strawberry.collected);
	println!("strawberry t: {}", map.strawberry.total);
	if let Some(v) = &map.goldberry {
		println!("goldberry c: {}", v.collected);
		println!("goldberry t: {}", v.btype);
	}
	if let Some(v) = &map.silverberries {
		println!("silver t: {}", v);
	}

	if let Some(v) = &map.specialberry {
		println!("special c: {}", v.collected);
		println!("special n: {}", v.name);
	}

	if let Some(v) = &map.cassette {
		println!("cassette c: {}", v);
	}

	if let Some(v) = &map.crystalheart {
		println!("crystalheart c: {}", v.collected);
		println!("crystalheart n: {}", v.name);
	}

	if let Some(v) = &map.min_dashes {
		println!("min_dashes: {} {} {} {}", v.pb, v.bronze, v.silver, v.gold);
	}
	if let Some(v) = &map.min_jumps {
		println!("min_jumps: {} {} {} {}", v.pb, v.bronze, v.silver, v.gold);
	}
	if let Some(v) = &map.speedrun {
		println!("speedrun: {} {} {} {}", v.pb, v.bronze, v.silver, v.gold);
	}
	
	println!("pb: {}", map.pb);
	println!("min_deaths_pb: {}", map.min_deaths_pb);
}
