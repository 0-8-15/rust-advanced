
use std::f64::consts::E;

use actix_web::Error;
use serde;

mod pets;

#[tokio::main]
async fn main() {
    let one = get_pet(10).await;
    println!("Status: {}", one.name);
    do_main();
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


fn do_main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE pet (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )",
        (), // empty list of parameters.
    )?;
    let alex = Pet {
        id: 0,
        name: "Alex".to_string(),
        // data: None,
    };
    &conn.add_pet(alex);

    let mut stmt = conn.prepare("SELECT id, name, data FROM pet")?;
    let pets = stmt.query_map([], |row| {
        Ok(Pet {
            id: row.get("id")?,
            name: row.get(1)?,
            // data: row.get(2)?,
        })
    })?;

    for pet in pets {
        println!("Found pet {:?}", pet.unwrap());
    }
    Ok(())
}
