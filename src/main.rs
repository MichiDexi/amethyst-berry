mod mouse;

fn main() {
	mouse::init();
    let mut a : mouse::Mouse = mouse::Mouse { x : 0, y : 0, lclick : false };
    loop { let _ = a.update(); }
}
