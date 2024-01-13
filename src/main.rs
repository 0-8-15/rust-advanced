
mod pets;
use pets::*;

mod db;
use db::*;

fn main() {
    {
        let conn = open_db().unwrap();
        let _ = conn.add_pet(pets::Pet{id: 1, name: "Alex".into(), photo: vec![]});
    }
    ;
    {
        let conn = open_db().unwrap();
        let _ = conn.show_all_pets();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let conn = open_db().unwrap();
        let _ = conn.add_pet(pets::Pet{id: 1, name: "Alex".into(), photo: vec![]});
	assert_eq!(conn.all_pets().expect("failed to load pets table").len(), 1)
    }

    #[test]
    fn test_remove() {
        let conn = open_db().unwrap();
        let _ = conn.del_pet(1);
	assert_eq!(conn.all_pets().expect("failed to load pets table").len(), 0)
    }

}
