const DATABASE_FILE_NAME: &str = "pets.db";

mod pets;

mod db;

mod crud;

fn main() {
    crud::main();
}
