pub struct Program {
	menu : MainMenuLayout,
	list : ListLayout,
	terminal : TerminalData
}

pub struct MainMenuLayout {
	order : Vec<String>
}

pub struct ListLayout {
	name : u8,
	index : u8,
	value : u8,
	date : u8,
	
	order : u8
}

pub struct TerminalData {
	color : u8, // 0 -> no color; 1 -> 8 color; 2 -> 16 color; 3 -> 256 color; 4 -> truecolor
	cli : bool, // false -> TUI; true -> CLI
	mouse_enabled : bool,
}

pub struct Other {
	
}
