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
		Write
	},
};
use std::rc::Rc;
use std::cell::RefCell;


pub mod interface;
pub mod helpers;
pub mod menus;
pub mod abt;

const TARGET_FPS : f64 = 60.0;

fn main() -> io::Result<()> {

	let abyss : abt::mapdata::Map = abt::mapdata::Map {
		name : "Abyss".to_string(),
		starred : false,
		rating : Some(100),
		tags : vec!("my favorite map".to_string()),
		difficulty : 7,
		clear_progress : Some(2),
		deaths : 1214,

		strawberry : abt::mapdata::Counted {
			collected : 11,
			total : 11,
		},
		goldberry : Some(abt::mapdata::TypedCollectable {
			collected : true,
			btype : 1,
		}),
		silverberries : None,

		specialberry : None,

		cassette : None,
		crystalheart : None,

		min_dashes : None,
		min_jumps : None,
		speedrun : None,

		pb : 0,
		min_deaths_pb : 314,
		checkpoints : vec!(
			abt::mapdata::Checkpoint {
				name : "Start".to_string(),
				notes : abt::mapdata::Notes {
					notes : "".to_string(),
					speedrun_notes : "".to_string(),
					min_dash_notes : "".to_string(),
					min_jump_notes : "".to_string(),
					goldenberry_notes : "".to_string(),
				},
				chokepoints : vec!(),
				silver_collected : None,

				min_deaths : 2,
				runs_died : 2,
				runs_passed : 0
			},
			abt::mapdata::Checkpoint {
				name : "Sunset".to_string(),
				notes : abt::mapdata::Notes {
					notes : "".to_string(),
					speedrun_notes : "".to_string(),
					min_dash_notes : "".to_string(),
					min_jump_notes : "".to_string(),
					goldenberry_notes : "".to_string(),
				},
				chokepoints : vec!(),
				silver_collected : None,

				min_deaths : 2,
				runs_died : 2,
				runs_passed : 0
			},
			abt::mapdata::Checkpoint {
				name : "Countryside".to_string(),
				notes : abt::mapdata::Notes {
					notes : "".to_string(),
					speedrun_notes : "".to_string(),
					min_dash_notes : "".to_string(),
					min_jump_notes : "".to_string(),
					goldenberry_notes : "".to_string(),
				},
				chokepoints : vec!(),
				silver_collected : None,

				min_deaths : 2,
				runs_died : 2,
				runs_passed : 0
			},
			abt::mapdata::Checkpoint {
				name : "City".to_string(),
				notes : abt::mapdata::Notes {
					notes : "".to_string(),
					speedrun_notes : "".to_string(),
					min_dash_notes : "".to_string(),
					min_jump_notes : "".to_string(),
					goldenberry_notes : "".to_string(),
				},
				chokepoints : vec!(),
				silver_collected : None,

				min_deaths : 2,
				runs_died : 2,
				runs_passed : 0
			},
			abt::mapdata::Checkpoint {
				name : "Abyss".to_string(),
				notes : abt::mapdata::Notes {
					notes : "".to_string(),
					speedrun_notes : "".to_string(),
					min_dash_notes : "".to_string(),
					min_jump_notes : "".to_string(),
					goldenberry_notes : "".to_string(),
				},
				chokepoints : vec!(),
				silver_collected : None,

				min_deaths : 2,
				runs_died : 2,
				runs_passed : 0
			}
		),
		range_runs : vec!(
			abt::mapdata::CheckpointRange {
				id_start : 0,
				id_end : 1,
			}
		)
	};
	println!("{}\n\n", abt::mapparsing::parse_map_as_string(abyss));


	let map = abt::mapparsing::parse_map_as_string(abt::mapdata::Map::new());
	abt::mapdata::present_map(&abt::mapdata::Map::new());
	println!("{}\n\n", map);

	if let Some(v) = abt::mapparsing::parse_string_as_map(abt::mapparsing::parse_map_as_string(abt::mapdata::Map::new())) {
		abt::mapdata::present_map(&v);
	}
	

	return Ok(());

	// Initialize everything, including:
	// InputHandler, Main menu
	helpers::input::init()?;
	let framerate : Duration = Duration::from_secs_f64(1.0 / TARGET_FPS);
	let mut input : helpers::input::InputHandler = helpers::input::InputHandler::new();

	// Initialize variables that persist between menu switches
	let mut out = stdout();
	let menu = Rc::new(RefCell::new(abt::menus::Menu::Main));
	let mut mainmenu = menus::mainmenu::MainMenu::init(Rc::clone(&menu));
	mainmenu.init_draw(&mut out);
	
	// Loop and use the program:
	// Switch between menus and interact with the filesystem
	loop {
		// Set frame up
		let now : Instant = Instant::now();
		input.update()?;

		// The actual frame
		let cmenu = *menu.borrow();
		match cmenu {
			abt::menus::Menu::Main => mainmenu.tick(&input, &mut out),
			abt::menus::Menu::Out => break,
			_ => {}
		}
		if *menu.borrow() == abt::menus::Menu::Out {
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
