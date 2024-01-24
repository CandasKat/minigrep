use std::fs;

pub fn search_in_file(filename: &str, query: &str) -> bool {
    if let Ok(contents) = fs::read_to_string(filename) {
    contents.contains(query)
    } else {
        false
    }

}

pub fn handle_args(args: &[String]) -> String {
    let filename = &args[1];
    let query = &args[2];

    let result = search_in_file(filename, query);
    if result {
        format!("'{}' found in {}", query, filename)
    } else {
        format!("'{}' not found in {}", query, filename)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn handle_args_with_valid_args_find_words() {
        let args = vec![String::from("minigrep"), String::from("test.txt"), String::from("rust")];
        assert_eq!(handle_args(&args), "'rust' found in test.txt");
    }

    #[test]
    fn handle_args_with_valid_args_not_find_words() {
        let args = vec![String::from("minigrep"), String::from("test.txt"), String::from("Python")];
        assert_eq!(handle_args(&args), "'Python' not found in test.txt");
    }

    #[test]
    fn find_word_in_file() {
        let filename = "test.txt";
        let query = "rust";
        assert!(search_in_file(filename, query));
    }

    #[test]
    fn not_find_word_in_file() {
        let filename = "test.txt";
        let query = "python";
        assert!(!search_in_file(filename, query));
    }
}

#[cfg(test)]
mod structural_test {
    use super::*;

    #[test]
    fn find_word_in_empty_file() {
        let filename = "empty.txt";
        let query = "rust";
        assert!(!search_in_file(filename, query));
    }

    #[test]
    fn find_word_in_nonexistent_file() {
        let filename = "nonexistent.txt";
        let query = "rust";
        assert!(!search_in_file(filename, query));
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn handle_args_with_insufficient_args() {
        let args = vec![String::from("minigrep")];
        handle_args(&args);
    }
}