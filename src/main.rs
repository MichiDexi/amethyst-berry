mod input;

fn main() {
	input::init();
    let mut a : input::Mouse = input::Mouse { x : 0, y : 0, lclick : false };
    let mut b : input::Window = input::Window { focused : false, width : 0, height : 0 };
    loop { let _ = input::update(&mut a, &mut b); }
}
