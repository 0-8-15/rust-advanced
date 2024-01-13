
mod pets;

mod db;

mod crud;

fn main() {
    crud::main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let conn = open_db().unwrap();
	let _ = conn.init_db();
	let mut pet: Pet = pets::Pet::new();
	pet.id="67e55044-10b1-426f-9247-bb680e5fe0c8".to_string();
	pet.name="Alex".into();
        let _ = conn.add_pet(pet);
	assert_eq!(conn.all_pets().expect("failed to load pets table").len(), 1)
    }

    #[test]
    fn test_remove() {
        let conn = open_db().unwrap();
	let l1 = conn.all_pets().expect("failed to load pets table").len();
        let _ = conn.del_pet("67e55044-10b1-426f-9247-bb680e5fe0c8".to_string());
	assert_eq!(conn.all_pets().expect("failed to load pets table").len(), l1-1)
    }

}
