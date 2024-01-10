pub trait MeinTrait {
    fn print(&self, str: String)   { print!("default"); }
}

struct MeinTyp;

impl MeinTrait for MeinTyp {
    fn print(&self, arg: String) {
	print!("{arg:}");
    }
}

fn main() {
    let m = MeinTyp;
    m.print("Hello, world!".to_string());
}
