use std::f64::consts::E;

use actix_web::Error;
use serde;
#[derive(serde::Deserialize, Debug)]
struct Pet {
    id: i32,
    name: String
}

#[tokio::main]
async fn main() {
    let one = get_pet(10).await;
    println!("Status: {}", one.name);
}

async fn get_pet(id: u64) -> Pet {
    let x = match reqwest::get(format!("https://petstore3.swagger.io/api/v3/pet/{id:}")).await {
        Ok(response) => response,
        Err(x) => panic!("Nixda")
    };
    let y: String = match x.text().await {
        Ok(text) => text,
        Err(_) => panic!("Noch schlimmer")
    };
    println!("Status: {y:}");
    let yj = serde_json::from_str::<Pet>(&y).unwrap();
    yj
}
