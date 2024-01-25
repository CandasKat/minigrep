use std::fs;

pub fn search_in_file(filename: &str, query: &str) -> Result<bool, &'static str> {
    match fs::read_to_string(filename) {
        Ok(contents) => Ok(contents.contains(query)),
        Err(_) => Err("Error reading file"),
    }
}

pub fn handle_args(args: &[String]) -> Result<String, &'static str> {
    if args.len() < 3 {
        return Err("Not enough arguments provided");
    }
    let filename = &args[1];
    let query = &args[2];

    match search_in_file(filename, query) {
        Ok(true) => Ok(format!("'{}' found in {}", query, filename)),
        Ok(false) => Ok(format!("'{}' not found in {}", query, filename)),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn handle_args_with_valid_args_find_words() {
        let args = vec![String::from("minigrep"), String::from("test.txt"), String::from("rust")];
        assert_eq!(handle_args(&args), Ok(String::from("'rust' found in test.txt")));
    }

    #[test]
    fn handle_args_with_valid_args_not_find_words() {
        let args = vec![String::from("minigrep"), String::from("test.txt"), String::from("Python")];
        assert_eq!(handle_args(&args), Ok(String::from("'Python' not found in test.txt")));
    }

    #[test]
    fn find_word_in_file() {
        let filename = "test.txt";
        let query = "rust";
        assert_eq!(search_in_file(filename, query), Ok(true));
    }

    #[test]
    fn not_find_word_in_file() {
        let filename = "test.txt";
        let query = "python";
        assert_eq!(search_in_file(filename, query), Ok(false));
    }

}

#[cfg(test)]
mod structural_test {
    use super::*;

    #[test]
    fn find_word_in_empty_file() {
        let filename = "empty.txt";
        let query = "rust";
        assert_eq!(search_in_file(filename, query), Ok(false));
    }

    #[test]
    fn find_word_in_nonexistent_file() {
        let filename = "nonexistent.txt";
        let query = "rust";
        assert!(matches!(search_in_file(filename, query), Err(_)));
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn handle_args_with_insufficient_args() {
        let args = vec![String::from("minigrep")];
        handle_args(&args);
    }
}