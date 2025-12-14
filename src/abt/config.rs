pub struct Program {
	pub menu : MainMenuLayout,
	pub list : ListLayout,
	pub terminal : TerminalData,
	pub other : Other,
}

pub struct MainMenuLayout {
	pub order : Vec<u8>
}

pub struct ListLayout {
	pub name : u8,
	pub index : u8,
	pub value : u8,
	pub date : u8,
	
	pub order : Vec<u8>
}

pub struct TerminalData {
	pub color : u8, // 0 -> no color; 1 -> 8 color; 2 -> 16 color; 3 -> 256 color; 4 -> truecolor
	pub cli : bool, // false -> TUI; true -> CLI
	pub mouse_enabled : bool,
}

pub struct Other {
	pub custom_load_directory : bool,
}

impl Program {
	pub fn initialize() -> Self {
		Program {
			MainMenuLayout {
				order : vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9),
			},
			ListLayout {
				name : 40,
				index : 10,
				value : 20,
				date : 30,

				order : vec!(0, 1, 2, 3),
			},
			TerminalData {
				color : 1,
				cli : false,
				mouse_enabled : false,
			},
			Other {
				custom_load_directory : false,
			}
		}
	}
}
