pub struct Map {
	/*
		A bunch of data comes here
	*/
}

impl Map {
	pub fn loadmap(&mut self, map : &Path) {
		
	}

	pub fn savemap(&self, map : &Path) {
		
	}
}

/*
	Everything that contains maps
*/

pub struct Lobby {
	maps : Vec<String>
}

pub struct Savefile {
	maps_lobbies : Vec<String>,
	lobby_oriented : bool
}

trait LoadMaps {
	
}

impl LoadMaps for Lobby {
	
}

impl LoadMaps for Savefile {
	
}
