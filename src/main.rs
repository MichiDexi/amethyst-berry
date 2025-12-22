use std::{
	io,
	time::{
		Duration,
		Instant,
	},
	thread::{
		sleep
	},
	io::{
		stdout,
		Write,
		Stdout,
	},
};


pub mod interface;
pub mod helpers;
pub mod menus;
pub mod abt;

const TARGET_FPS : f64 = 30.0;

fn main() -> io::Result<()> {

	let map : abt::mapdata::Map =
	abt::mapdata::Map {
		name : "banger map name".to_string(),
		tag : "this is a nice tag :D".to_string(),
		difficulty : 255, // ooh very scary :(
		clear_progress : 0,
		deaths : 2147483647, // knowing the 32-bit signed integer limit is such a useless skill
		
		strawberry_amount : 20,
		strawberry_collected : 19, // dang thats close

		goldberry_exists : true, // even more suffering D:
		goldberry_collected : false, // didnt even survive the suffering :((
		goldberry_type : 3, // fun fact, this type doesnt exist

		silverberry_exists : true, // maxing out all fields to make the parser feel pain after it has inflicted pain to me while I was working on it
		silverberry_amount : 67,
		silverberry_collected : 1,
		silverberry_type : 2, // apparently, silver is now platinum

		specialberry_exists : true, // of course it does
		specialberry_name : "this berry is gonna make you sad, because its a waste of time".to_string(), // yes, it was
		specialberry_collected : false, // you didnt become sad and didnt waste your time, congratulations

		challenge_activate : true,
		min_dashes_pb : 256,
		min_dashes_bronze : 256,
		min_dashes_silver : 256,
		min_dashes_gold : 256,
		min_jumps_pb : 256,
		min_jumps_bronze : 256,
		min_jumps_silver : 256,
		min_jumps_gold : 256,

		fastest_time : 1985,
		speedrun_bronze : 1986,
		speedrun_silver : 1987,
		speedrun_gold : 1988,

		cassette_exists : true, // i still have the urge to use 'probably' instead of 'true'
		cassette_collected : true,
		crystalheart_exists : true,
		crystalheart_name : "you shattered the crystal heart".to_string(),
		crystalheart_collected : false, // wait, you didnt?

		pb : 0, // i wouldve made it negative but this is a u32, not a i32
		min_deaths_pb : 27,
		checkpoints : vec!(
			abt::mapdata::Checkpoint {
				id : 0,
				name : "first checkpoint".to_string(),
				notes : "easiest checkpoint".to_string(),
				chokepoints : vec!(
					abt::mapdata::Chokepoint {
						id : 0,
						name : "first chokepoint".to_string(),
						notes : "hardest part of the whole map".to_string(),
						highest_backtoback_amount : 99
					},
					abt::mapdata::Chokepoint {
						id : 1,
						name : "second chokepoint".to_string(),
						notes : "second hardest part of the whole map".to_string(),
						highest_backtoback_amount : /* hi */ 3
					},
				),
				min_deaths : 12049,
			},
			abt::mapdata::Checkpoint {
				id : 1,
				name : "second checkpoint".to_string(),
				notes : "hardest checkpoint".to_string(),
				chokepoints : Vec::new(),
				min_deaths : 12049,
			}
		),
		range_runs : vec!(
			abt::mapdata::CheckpointRange {
				id_start : 0,
				id_end : 0,
			},
			abt::mapdata::CheckpointRange {
				id_start : 0,
				id_end : 1,
			},
			abt::mapdata::CheckpointRange {
				id_start : 1,
				id_end : 1,
			}
		),
	};

	abt::mapdata::present_map(&map);

	
	
	abt::mapdata::present_map(
		&abt::mapparsing::parse_map(
			&abt::mapparsing::parse_map_as_string(&map)));

	println!("{}", &abt::mapparsing::parse_map_as_string(&map));

	return Ok(());
	

	// Initialize everything, including:
	// InputHandler, Main menu
	helpers::input::init()?;
	let framerate : Duration = Duration::from_secs_f64(1.0 / TARGET_FPS);
	let mut input : helpers::input::InputHandler = helpers::input::InputHandler::new();

	// Initialize variables that persist between menu switches
	let mut out = stdout();
	let mut menu = abt::menus::Menu::Main;
	let mut mainmenu = menus::mainmenu::MainMenu::init(&mut out);
	
	// Loop and use the program:
	// Switch between menus and interact with the filesystem
	loop {
		// Set frame up
		let now : Instant = Instant::now();
		input.update()?;

		// The actual frame
		match menu {
			abt::menus::Menu::Main => mainmenu.tick(&input, &mut out, &mut menu),
			abt::menus::Menu::Out => break,
			_ => {}
		}
		if menu == abt::menus::Menu::Out {
			break;
		}

		// Frame time management for consistent framerate
		stdout().flush().unwrap();
		let frame_duration = Instant::now().duration_since(now);
		if frame_duration < framerate {
			sleep(framerate - frame_duration);
		}
	}

	// The loop has been exited, undo initialization of
	// InputHandler and screen extras
	helpers::input::uninit()?;
	Ok(())
}
