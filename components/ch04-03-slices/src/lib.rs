fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("the first word is: {}", word);

    // word will get the value 5
    // this empties the String, making it equal to ""
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    s.clear();
}
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
