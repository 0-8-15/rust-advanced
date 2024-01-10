#![allow(warnings)]
 
use tokio::time::{sleep, Duration};
use futures;
use primal::is_prime;
 
#[tokio::main]
async fn main() {
    println!("Starte Programm");
    perform_tasks().await;
}
 
async fn c1(n: u64) -> bool {
    let p = is_prime(n);
    println!("{n:}: {}", p);
    p
}

macro_rules! select_primes {
    ($x:expr) => {{
	let y = $x.into_iter().map(c1);
	futures::future::join_all(y).await
    }}
}

async fn perform_tasks() {
    let x = vec![1999999927, 2000000872, 2000000087, 2000000084];
    let z = select_primes!(x);
    println!("Erstens: {}", z[0]);
}
