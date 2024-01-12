use serde_derive::{Deserialize, Serialize};
use serde_rusqlite::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Pet {
    pub id: usize,
    pub name: String,
    pub category: Category,
    pub photo: Vec<u8>,
    pub tags: Vec<Tag>,
    pub status: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

impl Pet {
    pub fn new(id: i64, name: String) -> Self {
        Self {
            id,
            name,
            category: Category {
                id: 0,
                name: String::new(),
            },
            tags: vec![],
            status: "available".to_string(),
        }
    }
}

trait PetShop {
    fn add_pet(&self, pet: Pet) -> Result<()>;
    fn remove_pet(&self, pet: String) -> Result<()>;
    fn show_all_pets(&self, pet: Pet) -> Result<()>;
    fn show_pads_with_tag(&self, tag: String) -> Result<()>;
}

/* ********************************************************************** */

use rusqlite::{Connection, Result};

impl PetShop for Connection {
    fn add_pet(&self, pet: Pet) -> Result<()> {
        match self.execute(
            "INSERT INTO pet (name, data) VALUES (?1, ?2)",
            (&pet.id, &pet.name),
        ) {
            Ok(_) => Ok(()),
            Err(x) => Err(x),
        }
    }
    fn remove_pet(&self, name: String) -> Result<()> {
        match self.execute("DELETE FROM pet WHERE name = ?1", (&name)) {
            Ok(_) => Ok(()),
            Err(x) => Err(x),
        }
    }
    fn show_all_pets(&self, pet: Pet) -> Result<()> {
        let mut stmt = self.prepare("SELECT * FROM pet");
        match stmt {
            Ok(stmt) => {
                let all = stmt.query_map([], |row| {
		    Ok(Pet { id: 1 })
		});
                for pet in all {
                    println!("Pet {pet:}");
                }
                Ok(())
            }
            Err(x) => Err(x),
        }
    }

    fn show_pads_with_tag(&self, tag: String) -> Result<()> {
        match self.execute(
            "INSERT INTO pet (name, data) VALUES (?1, ?2)",
            (&pet.id, &pet.name),
        ) {
            Ok(_) => Ok(()),
            Err(x) => Err(x),
        }
    }
}
