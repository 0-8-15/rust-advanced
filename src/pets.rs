use serde_derive::{Deserialize, Serialize};

use slintext::sqlmdl::*;

pub trait SqlKey<T> { fn sql_key(&self) -> T; }

use uuid::{Uuid};
//#[derive(Serialize, Deserialize, Debug, PartialEq)]
//pub struct PetId (String)
type PetId = String;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[derive(Clone)]
#[derive(Default)]
pub struct Pet {
    pub id: PetId,
    pub name: String,
    pub photo: Option<Vec<u8>>,
}

pub trait PetShop {
    fn init_db(&self) ->Result<()>;
    fn add_pet(&self, pet: Pet) -> Result<()>;
    fn del_pet(&self, pet: PetId) -> Result<()>;
    fn get_pet(&self, pet: String) -> Option<Pet>;
    fn all_pets(&self) -> Result<Vec<Pet>>;
    fn show_all_pets(&self) -> Result<()>;
    fn show_pads_with_tag(&self, _tag: String) -> Result<()> {todo!("tags")}
}

impl Pet {
    pub fn new() -> Self {
	Self {
	    id: Uuid::new_v4().to_string(),
	    name: "".to_string(),
	    photo: Some(vec![]),
	}
    }
}

impl SqlKey<PetId> for Pet { fn sql_key(&self) -> PetId { self.id.clone() } }

pub const PET_SHOP_TABLE: SqlIdTable<'_> = SqlIdTable {
    initially: &[
        "CREATE TABLE IF NOT EXISTS DataBaseVersion AS SELECT 1 AS Version",
        r#"
CREATE TABLE IF NOT EXISTS pet (
id      TEXT UNIQUE PRIMARY KEY,
name    TEXT UNIQUE NOT NULL,
photo   BLOB
)"#,
    ],
    create: "INSERT INTO pet (id, name, photo) VALUES (:id, :name, :photo)",
    read: "SELECT * from pet where id=?1",
    all: "SELECT * FROM pet ORDER BY name",
    update: "UPDATE pet SET id=:id, name=:name, photo=:photo WHERE id=:id",
    delete: "DELETE FROM pet WHERE id = :id", delkey: &["id"],
};

/* ********************************************************************** */

use serde_rusqlite::*;
use rusqlite::{Connection, Result};

impl PetShop for Connection {
    fn init_db(&self) ->Result<()> {
        match self.execute("CREATE TABLE IF NOT EXISTS pet (
        id    TEXT UNIQUE PRIMARY KEY,
        name  TEXT NOT NULL,
        photo  BLOB
        )",
    (), // empty list of parameters.
	) {
	    Ok(_) => Ok(()),
	    Err(err) => {println!("Err {err:?}"); panic!("No Shit Sherlock!");}
	}
    }
    fn add_pet(&self, pet: Pet) -> Result<()> {
	let mut stmt = match self.prepare("INSERT OR REPLACE INTO pet (id, name, photo) VALUES (:id, :name, :photo)") {
	    Ok(stmt) => stmt,
	    Err(err) => {println!("Err {err:?}"); return Err(err)}
	};
        match to_params_named(&pet) {
	    Ok(params) => {
		// let columns = columns_from_statement(&stmt);
		match stmt.execute(
		    params.to_slice().as_slice()) {
                    Ok(_) => Ok(()),
                    Err(x) => {println!("Err {x:?}");Err(x)}
		}
	    }
	    Err(err) => {println!("Err {err:?}"); Ok(()) /* FIXME This is lying */}
	}
    }
    fn del_pet(&self, name: PetId) -> Result<()> {
        match self.execute("DELETE FROM pet WHERE id = ?1", [&name]) {
            Ok(_) => Ok(()),
            Err(x) => Err(x),
        }
    }
    fn get_pet(&self, id: String) -> Option<Pet> {
	let mut stmt = match self.prepare("SELECT * from pet where id=?1") {
	    Ok(stmt) => stmt,
	    Err(_) => return None
	};
	let mut rows = stmt.query_and_then([id], from_row::<Pet>).unwrap(); // prove: hopefully never happens
	match rows.next() {
	    Some(row) => Some(row.expect("unexpected ERROR")),
	    None => None
	}
    }
    fn all_pets(&self) -> Result<Vec<Pet>> {
        let stmt = self.prepare("SELECT * FROM pet");
        match stmt {
            Ok(mut stmt) => {
                let rows = from_rows(stmt.query([]).unwrap()); // prove: hopefully never happens
                let mut all: Vec<Pet> = vec![];
                for p in rows {
                    all.push(p.unwrap()) // OOM
                }
                Ok(all)
            }
            Err(x) => Err(x),
        }
    }
    fn show_all_pets(&self) -> Result<()> {
        match self.all_pets() {
            Ok(all) => {
                println!("N: {}", all.len());
                for pet in all {
                    println!("Pet {:?}", pet);
                }
                Ok(())
            }
            Err(x) => Err(x),
        }
    }

    fn show_pads_with_tag(&self, _tag: String) -> Result<()> {todo!("tags")}
}

/*

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pet {
    id: usize,
    name: String,
    category: String,
    photo: Vec<u8>,
    tags: Vec<String>,
    status: String,
}

fn main() {
    let json_data = r#"
    [
        {
            "id": 1,
            "name": "Lucky",
            "category": "Hund",
            "photo": [255, 0, 255, 127],
            "tags": ["freundlich", "verspielt"],
            "status": "verf�gbar"
        },
        {
            "id": 2,
            "name": "Daisy",
            "category": "Hund",
            "photo": [255, 0, 255, 127],
            "tags": ["freundlich", "verspielt"],
            "status": "verf�gbar"
        }
    ]
    "#;

    let pet: Vec<Pet> = match serde_json::from_str(json_data) {
        Ok(pet) => pet,
        Err(err) => panic!("Error: {}", err)
    };
    println!("Pet: {:?}", pet.get(0));
}

 */
