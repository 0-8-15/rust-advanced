use debug_info;
use std::{thread::sleep, time::Duration};

#[debug_info::measure_time("huch")]
fn calc(calories_per_bite: i32, bites: i32) -> i32 {
    println!("calculating...");
    sleep(Duration::from_secs(1));
    calories_per_bite * bites
}
fn main() {
    println!("{}",calc(2,2));
}
