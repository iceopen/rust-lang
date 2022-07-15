// References and Borrowing
fn string_name() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// Mutable References
fn change_main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

#[cfg(test)]
mod tests {
    use crate::{change_main, first_word_main, string_name};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_string_name() {
        string_name()
    }

    #[test]
    fn test_change_main() {
        change_main()
    }
}
