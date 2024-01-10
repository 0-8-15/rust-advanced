#[macro_export]
macro_rules! user_input {
    ($prompt:expr) => {{
        println!("{}: ", $prompt);
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => input.trim().to_lowercase(),
            Err(_) => "".to_string(),
        }
    }};
}

fn main() {
    let msg = user_input!("soso");
    println!("{}", msg);
}
