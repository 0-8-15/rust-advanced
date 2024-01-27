
const DATABASE_FILE_NAME: &str = "pets.db";

mod pets;

mod db;

mod sqlmdl;

mod crud;

fn main() {
    crud::main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xlsx() {
	crate::resultlist::write();
    }

}
