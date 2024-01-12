mod pet;
use pet::{Pet, Tag};
use rusqlite::{params, Connection, Result};
 
#[tokio::main]
async fn main() -> Result<()> {
    println!("create connection and default tables");
    let conn = create_tables()?;
 
    println!("get default pets");
    let pets = get_default_pets();
 
    println!("start adding pets");
    for pet in pets {
        println!("add {}", pet.name);
        add_pet(&pet, &conn).await?;
    }
 
    println!("show all pets");
    show_all_pets(&conn)?;
 
    println!("get pet with id = 0");
    let p = get_pet(0, &conn)?;
    println!("pet found: {p:?}");
 
    println!("get all pets with tag 'zutraulich'");
    show_pets_with_tag("zutraulich", &conn)?;
 
    println!("remove Pet '1' and show all pets");
    remove_pet(1, &conn)?;
    show_all_pets(&conn)?;
    Ok(())
}
 
fn create_tables() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
 
    conn.execute(
        "CREATE TABLE pets (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            category TEXT NOT NULL
        )",
        (),
    )?;
 
    conn.execute(
        "CREATE TABLE tags (
            pet_id INTEGER,
            tag TEXT NOT NULL,
            FOREIGN KEY (pet_id) REFERENCES pets (id)
        )",
        (), // empty list of parameters.
    )?;
 
    Ok(conn)
}
 
fn get_default_pets() -> Vec<Pet> {
    let pets = [
        ("Alex", "Hund", vec!["klein", "zutraulich"]),
        (
            "Sydney",
            "Katze",
            vec!["mopsig", "zutraulich", "Hinterhältig"],
        ),
        ("Stewie", "Meerschwein", vec!["ruhig", "kinderscheu"]),
    ];
 
    pets.into_iter()
        .enumerate()
        .map(|(id, pet)| {
            let name = pet.0.to_string();
            let category = pet.1.to_string();
            let tags: Vec<Tag> = pet
                .2
                .into_iter()
                .map(|tag_name| Tag {
                    id: id as i64,
                    name: tag_name.to_string(),
                })
                .collect();
 
            let mut p = Pet::new(id as i64, name);
            p.category.name = category;
            p.tags = tags;
            p
        })
        .collect()
}
 
/// wenn ein neues Haustier im Tierheim abgegeben wird, so wird dieses in der Datenbank, sowie im Petstore angelegt
async fn add_pet(pet: &Pet, conn: &Connection) -> Result<()> {
    // add to Database
    conn.execute(
        "INSERT INTO pets (id,name,category) VALUES (?1,?2,?3)",
        (pet.id, pet.name.clone(), pet.category.name.clone()),
    )?;
 
    for tag in pet.tags.clone() {
        conn.execute(
            "INSERT INTO tags (pet_id, tag) VALUES (?1,?2)",
            params![pet.id, tag.name],
        )?;
    }
 
    // add to Petstore
    let url = "https://petstore3.swagger.io/api/v3/pet";
    let client = reqwest::Client::new();
 
    let username = "test";
    let password = Some("abc123");
 
    // TODO: do something with the result
    let _result = client
        .post(url)
        .basic_auth(username, password)
        .json(pet)
        .send()
        .await;
 
    Ok(())
}
 
/// entfernt das Haustier aus der Datenbank, nicht jedoch vom Petstore
fn remove_pet(id: i64, conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM tags WHERE pet_id = $1", [id])?;
    conn.execute("DELETE FROM pets WHERE id = $1", [id])?;
    Ok(())
}
 
/// gibt alle Haustiere aus der Datenbank aus
fn show_all_pets(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, name, category FROM pets")?;
    let rows = stmt.query_map([], |row| {
        let mut p = Pet::new(row.get("id")?, row.get("name")?);
        p.category.name = row.get("category")?;
        Ok(p)
    })?;
 
    for pet in rows {
        match pet {
            Ok(p) => println!(
                "ID: {}, Name: {}, Category: {}",
                p.id, p.name, p.category.name
            ),
            Err(e) => eprintln!("Error: {e:?}"),
        }
    }
 
    Ok(())
}
 
fn get_pet(id: i64, conn: &Connection) -> Result<Pet> {
    let mut stmt = conn.prepare("SELECT id, name, category FROM pets WHERE id = $1")?;
    let mut rows = stmt.query_map([id], |row| {
        let mut p = Pet::new(row.get("id")?, row.get("name")?);
        p.category.name = row.get("category")?;
        Ok(p)
    })?;
 
    rows.next().unwrap()
}
 
/// gibt alle Haustiere aus, die einen angegebenen Tag haben
fn show_pets_with_tag(tag: &str, conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT pet_id FROM tags WHERE tag = $1")?;
    let rows = stmt.query_map([tag], |row| {
        let pet_id: i64 = row.get("pet_id")?;
        Ok(pet_id)
    })?;
 
    for result in rows {
        match result {
            Ok(id) => {
                let pet = get_pet(id, conn)?;
                println!("ID: {}, Name: {}, Tag: {}", id, pet.name, tag);
            }
            Err(e) => eprintln!("Error: {e:?}"),
        }
    }
 
    Ok(())
}
