mod mouse;

fn main() {
    let mut a : mouse::Mouse = mouse::Mouse { x : 0, y : 0, lclick : false };
    loop { let _ = a.update(); }
}
