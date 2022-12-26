
#[cfg(test)]
mod tests {
    use crate::first_word_main;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_first_word_main() {
        first_word_main()
    }
}
