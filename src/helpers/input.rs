use std::{
	time::{
		Duration,
		Instant,
	},
	io,
	collections::HashSet,
};
use crossterm::{
	event::{
		Event,
		KeyEventKind,
		KeyEvent,
		KeyCode,
		MouseEvent,
		MouseEventKind,
		MouseButton,
		poll,
		read,
	},
	terminal::{
		enable_raw_mode,
		disable_raw_mode,
		SetTitle,
	},
	execute,
};


// Keyboard config
const REPEAT_DELAY : Duration = Duration::from_millis(250); // Wait before first repeat
const REPEAT_RATE : Duration = Duration::from_millis(50);   // Interval between repeats

pub struct Mouse { // Has all the mouse attributes
	pub enabled : bool,
	pub x : u16,
	pub y : u16,
	pub lclick : bool,
	pub lclickheld : bool,
	pub rclick : bool,
	pub rclickheld : bool,
	pub scroll : i8, // 1 for up, -1 for down (I think)
}

pub struct Keyboard {
	pub pressed : HashSet<KeyCode>, // What is pressed currently
	pub just_pressed : HashSet<KeyCode>, // What was pressed this frame
	pub last_press_time : std::collections::HashMap<KeyCode, Instant>,
	// Length of press (used to calculate stuff)
}

pub struct Actions {
	pub up : bool,
	pub down : bool,
	pub left : bool,
	pub right : bool,
	pub confirm : bool,
	pub deny : bool,
}

pub struct Window { // Most of these don't even get updated, so I don't know why I made this at all
	pub focused : bool,
	pub width : u16,
	pub height : u16,
}


// This is the only struct that has actual functions!
// But those functions are more like abstractions...
// Skill issue on my end :(
// I'll remove this one soon maybe, or not, possibly
impl Keyboard {
	pub fn is_pressed(&self, key: KeyCode) -> bool {
		self.pressed.contains(&key)
	}

	pub fn just_pressed(&self, key: KeyCode) -> bool {
		self.just_pressed.contains(&key)
	}
}


// I bundled all the structs into one because I'm to lazy to initialize all of them separately
pub struct InputHandler {
	pub mouse : Mouse,
	pub keyboard : Keyboard,
	pub window : Window,
	pub actions : Actions,
}

impl InputHandler {
	pub fn new() -> Self { // Told you I was too lazy to initialize all of them :P
		let termsize = crossterm::terminal::size().unwrap();
		Self {
			mouse : Mouse {
				enabled : true,
				x : 0,
				y : 0,
				lclick : false,
				lclickheld : false,
				rclick : false,
				rclickheld : false,
				scroll : 0
			},
			keyboard : Keyboard {
				pressed : HashSet::new(),
				just_pressed : HashSet::new(),
				last_press_time : std::collections::HashMap::new(),
			},
			window : Window {
				focused : true, 
				width : termsize.0,
				height : termsize.1,
			},
			actions : Actions {
				up : false,
				down : false,
				left : false,
				right : false,
				confirm : false,
				deny : false,
			}
		}
	}

	pub fn update(&mut self) -> io::Result<()> { // This polls inputs

		self.mouse.lclick = false;
		self.mouse.rclick = false;
		let now = Instant::now();
		self.mouse.scroll = 0;
		self.keyboard.just_pressed.clear();
		
		while poll(Duration::from_millis(0))? {
			match read()? {
				Event::FocusGained => self.window.focused = true,
				Event::FocusLost => self.window.focused = false,
				Event::Mouse(event) => { Self::handle_mouse(event, &mut self.mouse); },
				Event::Resize(width, height) => {self.window.width = width; self.window.height = height},
				Event::Key(KeyEvent { code, kind, .. }) => {
					if matches!(kind, KeyEventKind::Press | KeyEventKind::Repeat) {
						if !self.keyboard.pressed.contains(&code) {
							self.keyboard.just_pressed.insert(code);
							self.keyboard.last_press_time.insert(code, Instant::now());
						}
						self.keyboard.pressed.insert(code);
					}
				}
				_ => {}
			}
		}

		// handle repeats
		for key in self.keyboard.pressed.iter() {
		    if let Some(&last_time) = self.keyboard.last_press_time.get(key) {
		        let elapsed = now.duration_since(last_time);
		        if elapsed >= REPEAT_DELAY {
		            // calculate how many repeats have passed
		            let repeats = ((elapsed - REPEAT_DELAY).as_millis() / REPEAT_RATE.as_millis()) as usize;
		            if repeats > 0 {
		                self.keyboard.just_pressed.insert(*key);       // trigger repeat
		                self.keyboard.last_press_time.insert(*key, now); // reset timer
		            }
		        }
		    }
		}
		
		// remove keys that have been released
		let key_timeout = Duration::from_millis(100); // adjust as needed
		self.keyboard.pressed.retain(|k| {
		    if let Some(&t) = self.keyboard.last_press_time.get(k) {
		        now.duration_since(t) < key_timeout
		    } else {
		        false
		    }
		});

		Self::handle_keybinds(&self.keyboard, &mut self.actions);
		
		Ok(())
	}

	fn handle_keybinds(keyboard : &Keyboard, actions : &mut Actions) {

		actions.up = handle_keybind(keyboard,
		vec!(&KeyCode::Char('w'), &KeyCode::Up, &KeyCode::Char('k')));

		actions.down = handle_keybind(keyboard,
		vec!(&KeyCode::Char('s'), &KeyCode::Down, &KeyCode::Char('j')));

		actions.left = handle_keybind(keyboard,
		vec!(&KeyCode::Char('a'), &KeyCode::Left, &KeyCode::Char('h')));

		actions.right = handle_keybind(keyboard,
		vec!(&KeyCode::Char('d'), &KeyCode::Right, &KeyCode::Char('l')));

		actions.confirm = handle_keybind(keyboard,
		vec!(&KeyCode::Char('y'), &KeyCode::Char(' '), &KeyCode::Char('z')));

		actions.deny = handle_keybind(keyboard,
		vec!(&KeyCode::Char('x'), &KeyCode::Backspace));

		fn handle_keybind(keyboard : &Keyboard, keys : Vec<&KeyCode>) -> bool {
			for key in keys {
				if keyboard.just_pressed.contains(key) {
					return true;
				}
			}
			false
		}
	}

	// I wonder what 'handle_mouse' does...
	// I think it might use the 'mouse' struct, I'm not sure...
	fn handle_mouse(event : MouseEvent, mouse : &mut Mouse) {
		if !mouse.enabled {
			return;
		}

		
		mouse.x = event.column;
		mouse.y = event.row;
	
		match event.kind {
			MouseEventKind::Down(MouseButton::Left)  => mouse.lclick     = true,
			MouseEventKind::Drag(MouseButton::Left)  => mouse.lclickheld = true,
			MouseEventKind::Up(MouseButton::Left)    => mouse.lclickheld = false,
			MouseEventKind::Down(MouseButton::Right) => mouse.rclick     = true,
			MouseEventKind::Drag(MouseButton::Right) => mouse.rclickheld = true,
			MouseEventKind::Up(MouseButton::Right)   => mouse.rclickheld = false,
			MouseEventKind::ScrollUp => mouse.scroll = 1,
			MouseEventKind::ScrollDown => mouse.scroll = -1,
			_ => {}
		}
	}
}


// Imagine doing stuff manually lmao
pub fn init() -> io::Result<()> {
	let mut stdout = io::stdout();
	execute!(stdout, crossterm::terminal::EnterAlternateScreen).unwrap();
	execute!(stdout, crossterm::cursor::Hide).unwrap();
	execute!(stdout, crossterm::event::EnableBracketedPaste).unwrap();
	execute!(stdout, crossterm::event::EnableFocusChange).unwrap();
	execute!(stdout, crossterm::event::EnableMouseCapture).unwrap();
	execute!(stdout, SetTitle("Amethyst berry tool"))?;
	enable_raw_mode()?;
	Ok(())
}

pub fn uninit() -> io::Result<()> { // De-initializes the end of all functions
	let mut stdout = io::stdout();
	execute!(stdout, crossterm::terminal::LeaveAlternateScreen).unwrap();
	execute!(stdout, crossterm::cursor::Show).unwrap();
	execute!(stdout, crossterm::event::DisableBracketedPaste).unwrap();
	execute!(stdout, crossterm::event::DisableFocusChange).unwrap();
	execute!(stdout, crossterm::event::DisableMouseCapture).unwrap();
	disable_raw_mode()?;
	Ok(())
}
