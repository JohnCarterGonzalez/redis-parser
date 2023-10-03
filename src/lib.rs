pub mod connection;
pub mod error;
pub mod manager;

#[cfg(test)]
mod tests {
    #[test]
    fn is_this_thing_on() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
