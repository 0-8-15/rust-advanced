pub mod bijective;
pub mod sqlmdl;
pub mod sqltmdl;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 42;
        assert_eq!(result, 42);
    }
}
