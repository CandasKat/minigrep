use std::fs;

pub fn search_in_file(filename: &str, query: &str) -> String {
    if let Ok(contents) = fs::read_to_string(filename) {
        if contents.contains(query) {
            return format!("'{}' found in {}", query, filename)
        }
        else {
            return format!("'{}' not found in {}", query, filename)
        }
        return format!("'{}' found in {}", query, filename)
    }
    else {
        return format!("")
    }
}

pub fn handle_args(args: &[String]) -> String {
    let filename = &args[1];
    let query = &args[2];

    let result = search_in_file(filename, query);
    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn handle_args_with_valid_args() {
        let args = vec![String::from("minigrep"), String::from("test.txt"), String::from("rust")];
        assert_eq!(handle_args(&args), "'rust' found in test.txt");
    }

    #[test]
    fn find_word_in_file() {
        let filename = "test.txt";
        let query = "rust";
        assert_eq!(search_in_file(filename, query), "'rust' found in test.txt");
    }

    #[test]
    fn not_find_word_in_file() {
        let filename = "test.txt";
        let query = "python";
        assert_eq!(search_in_file(filename, query), "'python' not found in test.txt");
    }
}

