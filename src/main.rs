fn kleener<T: PartialOrd>(a: T, b: T) -> T {
    if a < b { a } else { b }
}
fn main() {
    println!("kleener: {}", kleener(2,3));
}
